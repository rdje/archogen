//! SHA-256 (FIPS 180-4), the content hash every catalog record is identified by (leaf `M2.7.2`).
//!
//! `docs/decisions/decision_catalog-records.md` §3 hashes each facet of a record with SHA-256, and a review holds
//! only at the hash it names, so this function decides whether a review still holds. It is written here rather
//! than taken from a crate, because the engine depends on nothing outside `std`
//! (`decision_zero-dependency-engine-core.md`), and rather than run as the system's `shasum`, because the product
//! spawns no process (`NO-SUBPROCESS`).
//!
//! ⚠️ Three oracles check it, and none of them is this code:
//!
//! - the unit test below derives [`K`] and [`H0`] from their definitions, the cube and square roots of the first
//!   primes, in exact integer arithmetic;
//! - `tests/sha256.rs` reproduces the standard's published examples;
//! - the same test compares every message length across the padding's boundaries with digests the system's own
//!   tool wrote into `tests/fixtures/sha256_boundaries.txt` (`scripts/sha256_fixture.sh`).

use core::fmt;

/// The first 32 bits of the fractional parts of the cube roots of the first 64 primes (FIPS 180-4 §4.2.2).
const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// The first 32 bits of the fractional parts of the square roots of the first 8 primes (FIPS 180-4 §5.3.3).
const H0: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// A SHA-256 digest, written `sha256:` followed by 64 lowercase hexadecimal digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    /// The digest of `bytes`.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finish()
    }

    /// The 64 lowercase hexadecimal digits, without the `sha256:` prefix.
    #[must_use]
    pub fn hex(&self) -> String {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            out.push(char::from(DIGITS[usize::from(byte >> 4)]));
            out.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
        }
        out
    }

    /// Parse the written form: `sha256:`, then 64 lowercase hexadecimal digits (the record's §3). Anything else,
    /// uppercase digits included, is `None`, so one digest has one spelling and two spellings of it never compare
    /// unequal.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix("sha256:")?.as_bytes();
        if hex.len() != 64 {
            return None;
        }
        let mut out = [0u8; 32];
        for (byte, pair) in out.iter_mut().zip(hex.chunks_exact(2)) {
            *byte = (nibble(pair[0])? << 4) | nibble(pair[1])?;
        }
        Some(Self(out))
    }
}

/// The value of one lowercase hexadecimal digit.
fn nibble(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sha256:{}", self.hex())
    }
}

/// An incremental SHA-256 computation: feed the message in any number of pieces, then [`Sha256::finish`].
#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8],
    block: [u8; 64],
    filled: usize,
    /// The message length so far, in bytes. FIPS 180-4 appends it in bits, modulo 2^64.
    length: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// A computation over the empty message.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: H0,
            block: [0; 64],
            filled: 0,
            length: 0,
        }
    }

    /// Append `bytes` to the message.
    pub fn update(&mut self, mut bytes: &[u8]) {
        // A `usize` always fits in 64 bits on the targets this workspace builds for.
        self.length = self.length.wrapping_add(bytes.len() as u64);
        while !bytes.is_empty() {
            let take = (64 - self.filled).min(bytes.len());
            self.block[self.filled..self.filled + take].copy_from_slice(&bytes[..take]);
            self.filled += take;
            bytes = &bytes[take..];
            if self.filled == 64 {
                compress(&mut self.state, &self.block);
                self.filled = 0;
            }
        }
    }

    /// Pad the message (FIPS 180-4 §5.1.1) and return its digest.
    #[must_use]
    pub fn finish(mut self) -> Digest {
        let bits = self.length.wrapping_mul(8);
        self.block[self.filled] = 0x80;
        self.filled += 1;
        // The length takes the block's last eight bytes. When the `0x80` has left less room than that, this block
        // is finished with zeros and the length goes in one more.
        if self.filled > 56 {
            self.block[self.filled..].fill(0);
            compress(&mut self.state, &self.block);
            self.filled = 0;
        }
        self.block[self.filled..56].fill(0);
        self.block[56..].copy_from_slice(&bits.to_be_bytes());
        compress(&mut self.state, &self.block);
        let mut out = [0u8; 32];
        for (chunk, word) in out.chunks_exact_mut(4).zip(self.state) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        Digest(out)
    }
}

/// One application of the compression function to a 512-bit block (FIPS 180-4 §6.2.2).
fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for (word, bytes) in w.iter_mut().zip(block.chunks_exact(4)) {
        *word = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
    for t in 16..64 {
        let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
        let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16]
            .wrapping_add(s0)
            .wrapping_add(w[t - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (k, word) in K.iter().zip(w) {
        let big_s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h
            .wrapping_add(big_s1)
            .wrapping_add(ch)
            .wrapping_add(*k)
            .wrapping_add(word);
        let big_s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = big_s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (word, add) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *word = word.wrapping_add(add);
    }
}

#[cfg(test)]
mod tests {
    use super::{H0, K};

    /// The first `n` primes, by trial division.
    fn primes(n: usize) -> Vec<u128> {
        let mut found: Vec<u128> = Vec::new();
        let mut candidate = 2;
        while found.len() < n {
            if found.iter().all(|p| candidate % p != 0) {
                found.push(candidate);
            }
            candidate += 1;
        }
        found
    }

    /// The largest `x` with `x^power <= value`, by bisection: exact, where a floating-point root is not.
    fn integer_root(value: u128, power: u32) -> u128 {
        let (mut low, mut high) = (0u128, 1u128 << 64);
        while low < high {
            let mid = (low + high).div_ceil(2);
            if mid.checked_pow(power).is_some_and(|p| p <= value) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        low
    }

    /// The first 32 bits of the fractional part of `p^(1/power)`: `floor(p^(1/power) × 2^32) mod 2^32`, which is
    /// the integer root of `p × 2^(32 × power)`.
    fn fraction_bits(p: u128, power: u32) -> u32 {
        let root = integer_root(p << (32 * power), power);
        u32::try_from(root & 0xffff_ffff).expect("masked to 32 bits")
    }

    #[test]
    fn the_constants_are_the_fractional_parts_of_the_roots_of_the_first_primes() {
        let derived_k: Vec<u32> = primes(64)
            .into_iter()
            .map(|p| fraction_bits(p, 3))
            .collect();
        assert_eq!(
            derived_k, K,
            "K: cube roots of the first 64 primes (FIPS 180-4 §4.2.2)"
        );
        let derived_h0: Vec<u32> = primes(8).into_iter().map(|p| fraction_bits(p, 2)).collect();
        assert_eq!(
            derived_h0, H0,
            "H0: square roots of the first 8 primes (FIPS 180-4 §5.3.3)"
        );
    }
}
