//! The small grammars a record's text must meet (the record's §1, §2 and §4).
//!
//! Each is a plain predicate or parser over ASCII, so a test can name the exact rule it breaks, and nothing here
//! knows about forms or files.

/// A version, `MAJOR.MINOR.PATCH` (§2): three decimal numbers, each `0` or without a leading zero, no suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// The major number.
    pub major: u64,
    /// The minor number.
    pub minor: u64,
    /// The patch number.
    pub patch: u64,
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A requirement on a dependency's contract version, `MAJOR.MINOR`, with Cargo's caret meaning (§2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Requirement {
    /// The major number.
    pub major: u64,
    /// The minor number.
    pub minor: u64,
}

impl Requirement {
    /// Whether a version meets the requirement: at least `MAJOR.MINOR.0`, and below the next major, or below the
    /// next minor when the major is `0`.
    #[must_use]
    pub fn matches(self, v: Version) -> bool {
        if v.major != self.major {
            return false;
        }
        if self.major == 0 {
            v.minor == self.minor
        } else {
            v.minor >= self.minor
        }
    }
}

impl core::fmt::Display for Requirement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// One decimal number of a version: `0`, or digits without a leading zero, that fits a `u64`.
fn number(text: &str) -> Option<u64> {
    let bytes = text.as_bytes();
    if bytes.is_empty()
        || !bytes.iter().all(u8::is_ascii_digit)
        || (bytes.len() > 1 && bytes[0] == b'0')
    {
        return None;
    }
    text.parse().ok()
}

/// Parse `MAJOR.MINOR.PATCH`.
#[must_use]
pub fn version(text: &str) -> Option<Version> {
    let mut parts = text.split('.');
    let (major, minor, patch) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    Some(Version {
        major: number(major)?,
        minor: number(minor)?,
        patch: number(patch)?,
    })
}

/// Parse `MAJOR.MINOR`.
#[must_use]
pub fn requirement(text: &str) -> Option<Requirement> {
    let mut parts = text.split('.');
    let (major, minor) = (parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    Some(Requirement {
        major: number(major)?,
        minor: number(minor)?,
    })
}

/// One `-`-joined run of lowercase letters and digits, beginning with a letter when `letter_first`.
fn hyphenated(text: &str, letter_first: bool) -> bool {
    let bytes = text.as_bytes();
    let Some(&first) = bytes.first() else {
        return false;
    };
    if letter_first && !first.is_ascii_lowercase() {
        return false;
    }
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) || bytes[bytes.len() - 1] == b'-' {
        return false;
    }
    let mut previous = 0u8;
    for &b in bytes {
        let ok = b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-';
        if !ok || (b == b'-' && previous == b'-') {
            return false;
        }
        previous = b;
    }
    true
}

/// A record id (§1): dotted segments, each a lowercase letter followed by lowercase letters and digits with single
/// `-` between them.
#[must_use]
pub fn is_id(text: &str) -> bool {
    !text.is_empty() && text.split('.').all(|segment| hyphenated(segment, true))
}

/// A target stem (§2): lowercase letters and digits with single `-` between them.
#[must_use]
pub fn is_target_stem(text: &str) -> bool {
    hyphenated(text, false)
}

/// A maintainer, a task tree's id (§2): an uppercase letter, then uppercase letters and digits.
#[must_use]
pub fn is_maintainer(text: &str) -> bool {
    let bytes = text.as_bytes();
    matches!(bytes.first(), Some(b) if b.is_ascii_uppercase())
        && bytes
            .iter()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
}

/// A fact or cost name (§2): lowercase letters, digits, `-` and `.`.
#[must_use]
pub fn is_name(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
}

/// A path in normal form (§4): segments of ASCII letters, digits, `.`, `_`, `+` and `-`, joined by single `/`,
/// with no empty, `.` or `..` segment and no leading or trailing `/`.
#[must_use]
pub fn is_path(text: &str) -> bool {
    !text.is_empty()
        && text.split('/').all(|segment| {
            !segment.is_empty()
                && segment != "."
                && segment != ".."
                && segment
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'+' | b'-'))
        })
}

/// A hash as §3 writes it: `sha256:` and 64 lowercase hexadecimal digits.
#[must_use]
pub fn is_hash(text: &str) -> bool {
    text.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

/// A real calendar date, `YYYY-MM-DD`, in the proleptic Gregorian calendar.
#[must_use]
pub fn date(text: &str) -> Option<(u32, u32, u32)> {
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let digits = |range: core::ops::Range<usize>| -> Option<u32> {
        let part = &text[range];
        part.bytes()
            .all(|b| b.is_ascii_digit())
            .then(|| part.parse().ok())
            .flatten()
    };
    let (year, month, day) = (digits(0..4)?, digits(5..7)?, digits(8..10)?);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    (1..=days).contains(&day).then_some((year, month, day))
}

/// Whether every byte is printable ASCII (`0x20`–`0x7E`) or, when `newline` is true, a line feed.
#[must_use]
pub fn is_printable(text: &[u8], newline: bool) -> bool {
    text.iter()
        .all(|&b| (0x20..=0x7e).contains(&b) || (newline && b == b'\n'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_are_three_numbers_without_leading_zeros() {
        assert_eq!(
            version("1.20.0"),
            Some(Version {
                major: 1,
                minor: 20,
                patch: 0
            })
        );
        for bad in [
            "1.2",
            "1.2.3.4",
            "01.2.3",
            "1.2.3-rc",
            "1..3",
            "",
            "1.2.x",
            "1.2.99999999999999999999",
        ] {
            assert_eq!(version(bad), None, "{bad}");
        }
    }

    #[test]
    fn requirements_use_the_caret_meaning() {
        let r = requirement("1.2").unwrap();
        assert!(r.matches(version("1.2.0").unwrap()) && r.matches(version("1.9.3").unwrap()));
        assert!(!r.matches(version("1.1.9").unwrap()) && !r.matches(version("2.0.0").unwrap()));
        let zero = requirement("0.3").unwrap();
        assert!(zero.matches(version("0.3.7").unwrap()));
        assert!(
            !zero.matches(version("0.4.0").unwrap()) && !zero.matches(version("0.2.0").unwrap())
        );
        assert_eq!(requirement("1.2.3"), None);
    }

    #[test]
    fn ids_are_dotted_hyphenated_segments() {
        assert!(is_id("rt.scheduler.fixed-priority") && is_id("a1.b-2"));
        for bad in [
            "", "Rt", "rt..x", ".rt", "rt.", "rt.1x", "rt.-x", "rt.x-", "rt.x--y", "rt_x",
        ] {
            assert!(!is_id(bad), "{bad}");
        }
    }

    #[test]
    fn stems_maintainers_names_paths_hashes() {
        assert!(
            is_target_stem("riscv-virt-up")
                && !is_target_stem("riscv--up")
                && !is_target_stem("Up")
        );
        assert!(
            is_maintainer("M2")
                && is_maintainer("PROGRAM")
                && !is_maintainer("m2")
                && !is_maintainer("2M")
        );
        assert!(is_name("api.mask") && is_name("service.uart0") && !is_name("Api") && !is_name(""));
        assert!(is_path("crates/rt-core") && is_path("a/b.c/d_e+f"));
        for bad in ["", "/a", "a/", "a//b", "a/./b", "a/../b", "a b", "a\\b"] {
            assert!(!is_path(bad), "{bad}");
        }
        assert!(is_hash(&format!("sha256:{}", "a".repeat(64))));
        assert!(!is_hash(&format!("sha256:{}", "A".repeat(64))) && !is_hash("sha256:abc"));
    }

    #[test]
    fn dates_are_real_calendar_dates() {
        assert_eq!(date("2024-02-29"), Some((2024, 2, 29)));
        for bad in [
            "2023-02-29",
            "1900-02-29",
            "2026-13-01",
            "2026-04-31",
            "2026-4-01",
            "2026-04-00",
        ] {
            assert_eq!(date(bad), None, "{bad}");
        }
        assert_eq!(date("2000-02-29"), Some((2000, 2, 29)));
    }
}
