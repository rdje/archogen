//! The status vocabulary only grows (leaf `M3.5.1.1`): each minor of [`VOCABULARY`]'s major has a golden that lists
//! every status it holds, with its code and family, and a golden is never rewritten.
//!
//! A status renamed, renumbered, moved between families or removed changes what an earlier minor holds, and fails here
//! against that minor's golden. A status added without the vocabulary's minor moving changes the current minor's
//! table, and fails against its golden until [`VOCABULARY`] moves — and the register with it,
//! `docs/book/src/versions.md` — and the new minor is blessed deliberately:
//! `ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-outcome --test vocabulary`.

use std::fs;
use std::path::{Path, PathBuf};

use archogen_outcome::{Status, Version, VOCABULARY};

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

fn golden(version: Version) -> PathBuf {
    goldens().join(format!("status-vocabulary-{version}.golden"))
}

/// The table of what `version` holds: every status that entered at or before it, in exit-code order.
fn table(version: Version) -> String {
    let mut out = format!("status-vocabulary {version}\n");
    for status in Status::ALL.iter().filter(|s| s.since() <= version) {
        let family = if status.is_diagnostic_result() {
            "result"
        } else {
            "process"
        };
        out.push_str(&format!("{} {} {family}\n", status.code(), status.slug()));
    }
    out
}

#[test]
fn every_minor_of_the_vocabulary_holds_what_its_golden_froze() {
    for minor in 0..=VOCABULARY.minor {
        let version = Version {
            major: VOCABULARY.major,
            minor,
        };
        let actual = table(version);
        match fs::read_to_string(golden(version)) {
            Ok(frozen) => assert!(
                frozen == actual,
                "vocabulary {version} no longer holds what it held. Within a major a status is never removed, renamed, \
                 renumbered or moved between families; a new status moves the minor (`VOCABULARY`, and the \
                 register).\n--- frozen\n{frozen}\n--- now\n{actual}"
            ),
            Err(_) if minor == VOCABULARY.minor && std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
                fs::write(golden(version), actual).expect("the golden is writable");
            }
            Err(_) => panic!(
                "no golden for vocabulary {version} at {}. The current minor gets one deliberately: \
                 ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-outcome --test vocabulary",
                golden(version).display()
            ),
        }
    }
}

#[test]
fn no_golden_names_a_vocabulary_the_code_does_not_have() {
    let current = format!("status-vocabulary-{VOCABULARY}.golden");
    for entry in fs::read_dir(goldens()).expect("the goldens directory") {
        let name = entry
            .expect("an entry")
            .file_name()
            .into_string()
            .expect("UTF-8");
        let Some(version) = name
            .strip_prefix("status-vocabulary-")
            .and_then(|rest| rest.strip_suffix(".golden"))
        else {
            continue;
        };
        let version = Version::parse(version).unwrap_or_else(|| panic!("{name}: not a version"));
        assert!(
            version <= VOCABULARY,
            "{name} names a vocabulary newer than the code's {VOCABULARY}; the current one is {current}"
        );
    }
}
