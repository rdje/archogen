//! Just enough ELF and RISC-V to read what the pinned assembler made (leaf `M2.12.4.4`).
//!
//! Three of the port record's premises are about the object code, not the compiler's text: what `li`, `la`, `call`
//! and `tail` expand into and which registers that writes, that `j` assembles to the two-byte `c.j`, and that a
//! panic's path ends in the handler. The pinned toolchain ships no disassembler, so this reads the ELF's sections
//! and symbols and decodes the few instruction forms those premises meet. Anything else it is shown is refused,
//! not guessed: a premise that rests on an instruction this cannot read fails, and says so.

/// One section header: where its bytes are, in the file and in memory.
#[derive(Debug, Clone)]
pub struct Section {
    pub addr: u64,
    pub offset: u64,
}

/// One symbol.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    /// Its section's index; 0 for an undefined symbol.
    pub section: u16,
}

/// An ELF64 little-endian RISC-V file, relocatable or linked.
pub struct Elf<'a> {
    bytes: &'a [u8],
    linked: bool,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

fn u16_at(b: &[u8], at: usize) -> Result<u16, String> {
    b.get(at..at + 2)
        .map(|s| u16::from_le_bytes([s[0], s[1]]))
        .ok_or_else(|| format!("truncated at {at}"))
}

fn u32_at(b: &[u8], at: usize) -> Result<u32, String> {
    b.get(at..at + 4)
        .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
        .ok_or_else(|| format!("truncated at {at}"))
}

fn u64_at(b: &[u8], at: usize) -> Result<u64, String> {
    b.get(at..at + 8)
        .map(|s| u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
        .ok_or_else(|| format!("truncated at {at}"))
}

fn index(value: u64) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| format!("{value} does not fit an index"))
}

fn string(table: &[u8], at: usize) -> String {
    let rest = table.get(at..).unwrap_or_default();
    let end = rest.iter().position(|&b| b == 0).unwrap_or(rest.len());
    String::from_utf8_lossy(&rest[..end]).into_owned()
}

impl<'a> Elf<'a> {
    /// Read `bytes` as an ELF64 little-endian RISC-V file.
    ///
    /// # Errors
    ///
    /// Anything else, or a header that does not hold together.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        if bytes.get(..4) != Some(b"\x7fELF")
            || bytes.get(4) != Some(&2)
            || bytes.get(5) != Some(&1)
        {
            return Err("not an ELF64 little-endian file".to_owned());
        }
        if u16_at(bytes, 0x12)? != 243 {
            return Err("not a RISC-V file".to_owned());
        }
        let linked = match u16_at(bytes, 0x10)? {
            1 => false,
            2 => true,
            other => {
                return Err(format!(
                    "ELF type {other}, neither relocatable nor executable"
                ))
            }
        };
        let shoff = index(u64_at(bytes, 0x28)?)?;
        let shentsize = usize::from(u16_at(bytes, 0x3a)?);
        let shnum = usize::from(u16_at(bytes, 0x3c)?);
        let mut raw = Vec::new();
        for k in 0..shnum {
            let h = shoff + k * shentsize;
            raw.push((
                u32_at(bytes, h)?,
                u32_at(bytes, h + 4)?,
                u64_at(bytes, h + 16)?,
                u64_at(bytes, h + 24)?,
                u64_at(bytes, h + 32)?,
                u32_at(bytes, h + 40)?,
            ));
        }
        let table = |k: usize| -> Result<&[u8], String> {
            let (_, _, _, offset, size, _) = raw.get(k).ok_or("a section index out of range")?;
            let (start, size) = (index(*offset)?, index(*size)?);
            bytes
                .get(start..start + size)
                .ok_or_else(|| "a section past the end".to_owned())
        };
        let sections: Vec<Section> = raw
            .iter()
            .map(|(_, _, addr, offset, _, _)| Section {
                addr: *addr,
                offset: *offset,
            })
            .collect();
        let mut symbols = Vec::new();
        for (kind, link, k) in raw.iter().enumerate().map(|(k, r)| (r.1, r.5, k)) {
            if kind != 2 {
                continue;
            }
            let entries = table(k)?;
            let strings = table(link as usize)?;
            for e in entries.chunks_exact(24) {
                symbols.push(Symbol {
                    name: string(strings, u32_at(e, 0)? as usize),
                    section: u16_at(e, 6)?,
                    value: u64_at(e, 8)?,
                    size: u64_at(e, 16)?,
                });
            }
        }
        Ok(Self {
            bytes,
            linked,
            sections,
            symbols,
        })
    }

    /// The defined function symbol named `part`, or else the first whose name holds it, with a size.
    #[must_use]
    pub fn function(&self, part: &str) -> Option<&Symbol> {
        let defined = |s: &&Symbol| s.section != 0 && s.size > 0;
        self.symbols
            .iter()
            .filter(defined)
            .find(|s| s.name == part)
            .or_else(|| {
                self.symbols
                    .iter()
                    .filter(defined)
                    .find(|s| s.name.contains(part))
            })
    }

    /// Every undefined symbol's name.
    #[must_use]
    pub fn undefined(&self) -> Vec<&str> {
        self.symbols
            .iter()
            .filter(|s| s.section == 0 && !s.name.is_empty())
            .map(|s| s.name.as_str())
            .collect()
    }

    /// A function's bytes, and the address of its first.
    ///
    /// # Errors
    ///
    /// A symbol whose section or bytes are not in the file.
    pub fn code(&self, symbol: &Symbol) -> Result<(u64, &'a [u8]), String> {
        let section = self
            .sections
            .get(usize::from(symbol.section))
            .ok_or_else(|| format!("`{}`'s section is not in the file", symbol.name))?;
        let within = if self.linked {
            symbol
                .value
                .checked_sub(section.addr)
                .ok_or_else(|| format!("`{}` lies before its section", symbol.name))?
        } else {
            symbol.value
        };
        let start = index(section.offset + within)?;
        let end = start + index(symbol.size)?;
        let bytes = self
            .bytes
            .get(start..end)
            .ok_or_else(|| format!("`{}`'s bytes are past the end", symbol.name))?;
        Ok((symbol.value, bytes))
    }
}

/// What one decoded instruction is, as far as the premises need.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `auipc rd, imm`, the immediate already shifted.
    Auipc { rd: u8, imm: i64 },
    /// `jalr rd, imm(rs1)`, `c.jr` and `c.jalr` among them.
    Jalr { rd: u8, rs1: u8, imm: i64 },
    /// `jal rd, imm`.
    Jal { rd: u8, imm: i64 },
    /// The compressed `c.j imm`.
    CompressedJump { imm: i64 },
    /// `addi rd, rs1, imm`.
    Addi { imm: i64 },
    /// `mret`.
    Mret,
    /// Any other form this reads.
    Other,
}

/// One decoded instruction: its length in bytes, the register it writes (none for `x0`), and its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub len: usize,
    pub writes: Option<u8>,
    pub kind: Kind,
}

fn sign(value: u32, bits: u32) -> i64 {
    let shift = 64 - bits;
    (i64::from(value) << shift) >> shift
}

fn written(rd: u32) -> Option<u8> {
    u8::try_from(rd).ok().filter(|r| *r != 0)
}

/// Decode the instruction at the start of `bytes`.
///
/// # Errors
///
/// A form the premises never meet, which this refuses rather than guesses.
pub fn decode(bytes: &[u8]) -> Result<Decoded, String> {
    let half = u16_at(bytes, 0)?;
    if half & 3 != 3 {
        return compressed(half);
    }
    let w = u32_at(bytes, 0)?;
    let rd = (w >> 7) & 31;
    let rs1 = (w >> 15) & 31;
    let funct3 = (w >> 12) & 7;
    let imm_i = sign(w >> 20, 12);
    let at = |writes: Option<u8>, kind: Kind| {
        Ok(Decoded {
            len: 4,
            writes,
            kind,
        })
    };
    let rd8 = rd as u8;
    match w & 0x7f {
        0x37 => at(written(rd), Kind::Other),
        0x17 => at(
            written(rd),
            Kind::Auipc {
                rd: rd8,
                imm: sign(w & 0xffff_f000, 32),
            },
        ),
        0x6f => {
            let imm = ((w >> 31) & 1) << 20
                | ((w >> 21) & 0x3ff) << 1
                | ((w >> 20) & 1) << 11
                | ((w >> 12) & 0xff) << 12;
            at(
                written(rd),
                Kind::Jal {
                    rd: rd8,
                    imm: sign(imm, 21),
                },
            )
        }
        0x67 => at(
            written(rd),
            Kind::Jalr {
                rd: rd8,
                rs1: rs1 as u8,
                imm: imm_i,
            },
        ),
        0x13 if funct3 == 0 => at(written(rd), Kind::Addi { imm: imm_i }),
        0x13 | 0x1b | 0x33 | 0x3b | 0x03 => at(written(rd), Kind::Other),
        0x73 if w == 0x3020_0073 => at(None, Kind::Mret),
        0x73 if funct3 == 0 => at(None, Kind::Other),
        0x73 => at(written(rd), Kind::Other),
        0x23 | 0x63 | 0x0f => at(None, Kind::Other),
        op => Err(format!(
            "an instruction `{w:#010x}` of opcode {op:#04x}, which this does not read"
        )),
    }
}

fn compressed(h: u16) -> Result<Decoded, String> {
    let h = u32::from(h);
    let funct3 = h >> 13;
    let rd = (h >> 7) & 31;
    let rs2 = (h >> 2) & 31;
    let prime = |r: u32| written((r & 7) + 8);
    let at = |writes: Option<u8>, kind: Kind| {
        Ok(Decoded {
            len: 2,
            writes,
            kind,
        })
    };
    match (h & 3, funct3) {
        (0, 0 | 2 | 3) => at(prime(h >> 2), Kind::Other),
        (0, 6 | 7) => at(None, Kind::Other),
        (1, 0..=3) => at(written(rd), Kind::Other),
        (1, 4) => at(prime(h >> 7), Kind::Other),
        (1, 5) => {
            let bit = |from: u32, to: u32| ((h >> from) & 1) << to;
            let imm = bit(12, 11)
                | bit(11, 4)
                | ((h >> 9) & 3) << 8
                | bit(8, 10)
                | bit(7, 6)
                | bit(6, 7)
                | ((h >> 3) & 7) << 1
                | bit(2, 5);
            at(None, Kind::CompressedJump { imm: sign(imm, 12) })
        }
        (1, 6 | 7) => at(None, Kind::Other),
        (2, 0 | 2 | 3) => at(written(rd), Kind::Other),
        (2, 4) => {
            let high = (h >> 12) & 1;
            match (high, rd, rs2) {
                (0, _, 0) => at(
                    None,
                    Kind::Jalr {
                        rd: 0,
                        rs1: rd as u8,
                        imm: 0,
                    },
                ),
                (0, _, _) | (1, _, 1..) => at(written(rd), Kind::Other),
                (1, 0, 0) => at(None, Kind::Other),
                (_, _, _) => at(
                    Some(1),
                    Kind::Jalr {
                        rd: 1,
                        rs1: rd as u8,
                        imm: 0,
                    },
                ),
            }
        }
        (2, 6 | 7) => at(None, Kind::Other),
        _ => Err(format!(
            "a compressed instruction `{h:#06x}`, which this does not read"
        )),
    }
}

/// Every instruction of `bytes`, from address `at`, with its address.
///
/// # Errors
///
/// The first instruction [`decode`] refuses.
pub fn instructions(at: u64, bytes: &[u8]) -> Result<Vec<(u64, Decoded)>, String> {
    let mut out = Vec::new();
    let mut k = 0;
    while k < bytes.len() {
        let d = decode(&bytes[k..])?;
        out.push((at + k as u64, d));
        k += d.len;
    }
    Ok(out)
}

/// Every register a body writes.
#[must_use]
pub fn writes(body: &[(u64, Decoded)]) -> Vec<u8> {
    let mut out: Vec<u8> = body.iter().filter_map(|(_, d)| d.writes).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Every address a body calls or jumps to directly: `jal`, `c.j`, and an `auipc` with the `jalr` through its register
/// just after it.
#[must_use]
pub fn targets(body: &[(u64, Decoded)]) -> Vec<u64> {
    let mut out = Vec::new();
    for (k, (pc, d)) in body.iter().enumerate() {
        match d.kind {
            Kind::Jal { imm, .. } | Kind::CompressedJump { imm } => {
                out.push(pc.wrapping_add_signed(imm))
            }
            Kind::Auipc { rd, imm } => {
                if let Some((_, next)) = body.get(k + 1) {
                    if let Kind::Jalr { rs1, imm: low, .. } = next.kind {
                        if rs1 == rd {
                            out.push(pc.wrapping_add_signed(imm).wrapping_add_signed(low));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{decode, instructions, targets, writes, Kind};

    #[test]
    fn the_forms_the_premises_meet_decode() {
        // `j` to the next instruction, as the pin assembles it: `c.j` (§14.4's probe).
        let cj = decode(&0xa009u16.to_le_bytes()).unwrap();
        assert_eq!((cj.len, cj.kind), (2, Kind::CompressedJump { imm: 2 }));
        let mret = decode(&0x3020_0073u32.to_le_bytes()).unwrap();
        assert_eq!((mret.len, mret.kind, mret.writes), (4, Kind::Mret, None));
        // `addi a0, a0, -2048`.
        let addi = decode(&0x8005_0513u32.to_le_bytes()).unwrap();
        assert_eq!(
            (addi.kind, addi.writes),
            (Kind::Addi { imm: -2048 }, Some(10))
        );
        // `auipc ra, 0` then `jalr ra, 8(ra)`: a call to eight bytes past the `auipc`.
        let mut body = Vec::new();
        body.extend_from_slice(&0x0000_0097u32.to_le_bytes());
        body.extend_from_slice(&0x0080_80e7u32.to_le_bytes());
        let decoded = instructions(0x1000, &body).unwrap();
        assert_eq!(writes(&decoded), vec![1]);
        assert_eq!(targets(&decoded), vec![0x1008]);
        // `c.jr ra`, a return, writes nothing; `c.mv a0, a1` writes `a0`.
        assert_eq!(decode(&0x8082u16.to_le_bytes()).unwrap().writes, None);
        assert_eq!(decode(&0x852eu16.to_le_bytes()).unwrap().writes, Some(10));
    }

    #[test]
    fn a_form_the_premises_never_meet_is_refused() {
        // A floating-point load: opcode 0x07.
        assert!(decode(&0x0000_2007u32.to_le_bytes()).is_err());
        assert!(decode(&[0x13]).is_err(), "a truncated instruction");
    }
}
