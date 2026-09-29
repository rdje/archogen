//! `cost-accounting/1` cannot change without its identifier moving (leaf `PROGRAM.6.3`, `ROADMAP.md` §15).
//!
//! §7.4.1 asks every analysis to carry a *versioned* record of how time is charged, because "a changed accounting
//! rule invalidates every total previously stated under it". This freezes what the contract says — its version,
//! the three kinds of total, the ledger categories, the seven identifications with their statements, and the
//! table a ledger renders as — in a golden file named after the identifier. The golden for an identifier is
//! **never rewritten**: a change to any of it fails here until the identifier moves, and a new identifier gets its
//! golden only deliberately (`ARCHOGEN_BLESS_FORMATS=1 cargo test -p rt-analysis --test format_golden`).
//!
//! ⛔ The kinds and categories are listed through **exhaustive `match`es**: a variant added to `Accounting` or
//! `Category` stops this file compiling until it is written here — and then the golden differs, so the version
//! has to move.

use std::fs;
use std::path::{Path, PathBuf};

use rt_analysis::cost::CONTRACT_VERSION;
use rt_analysis::{Accounting, Category, Interval, Ledger, COST_ACCOUNTING_V1};

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

/// Compare `actual` with the golden for `identifier`; create one only when blessing, and never overwrite one.
fn check_golden(identifier: &str, actual: &str) {
    let path = goldens().join(format!("{}.golden", identifier.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(frozen) => assert!(
            frozen == actual,
            "what `{identifier}` says has changed. A golden is never rewritten: move the identifier (and the \
             register, docs/book/src/versions.md), then bless the new one.\n--- frozen\n{frozen}\n--- now\n{actual}"
        ),
        Err(_) if std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
            fs::create_dir_all(goldens()).expect("the goldens directory is creatable");
            fs::write(&path, actual).expect("the golden is writable");
        }
        Err(_) => panic!(
            "no golden for `{identifier}` at {}. A new identifier gets one deliberately: \
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p rt-analysis --test format_golden",
            path.display()
        ),
    }
}

fn every_accounting() -> Vec<Accounting> {
    let all = [
        Accounting::ExactTrace,
        Accounting::SafeEnvelope,
        Accounting::ObservedMaximum,
    ];
    for kind in all {
        match kind {
            Accounting::ExactTrace | Accounting::SafeEnvelope | Accounting::ObservedMaximum => {}
        }
    }
    all.to_vec()
}

fn every_category() -> Vec<Category> {
    let all = [
        Category::InitialDispatch,
        Category::TaskExecution("<task>".to_string()),
        Category::InterruptService,
        Category::TaskSwitch,
        Category::CriticalSection,
        Category::Instrumentation,
        Category::IdleWakeup,
    ];
    for category in &all {
        match category {
            Category::InitialDispatch
            | Category::TaskExecution(_)
            | Category::InterruptService
            | Category::TaskSwitch
            | Category::CriticalSection
            | Category::Instrumentation
            | Category::IdleWakeup => {}
        }
    }
    all.to_vec()
}

/// Everything `cost-accounting/1` says, as one canonical text.
fn contract_text() -> String {
    let mut out = format!("{CONTRACT_VERSION}\n");
    out.push_str(&format!(
        "contract record: {}\n",
        COST_ACCOUNTING_V1.version
    ));
    out.push_str("kinds of total:\n");
    for kind in every_accounting() {
        out.push_str(&format!("  {}\n", kind.slug()));
    }
    out.push_str("ledger categories:\n");
    for category in every_category() {
        out.push_str(&format!("  {category}\n"));
    }
    out.push_str("identifications:\n");
    for (term, statement) in COST_ACCOUNTING_V1.terms {
        out.push_str(&format!("  {term}: {statement}\n"));
    }
    out.push_str("a ledger renders as:\n");
    let ledger = Ledger::seal(
        vec![
            Interval {
                start: 0,
                end: 2,
                category: Category::InitialDispatch,
                activity: "dispatch a".to_string(),
            },
            Interval {
                start: 2,
                end: 5,
                category: Category::TaskExecution("a".to_string()),
                activity: "a runs".to_string(),
            },
        ],
        Accounting::ExactTrace,
    )
    .expect("two adjacent intervals seal");
    out.push_str(&ledger.render());
    out
}

#[test]
fn what_cost_accounting_1_says_is_frozen_under_its_identifier() {
    let identifier = CONTRACT_VERSION
        .split_whitespace()
        .next()
        .expect("a non-empty version");
    check_golden(identifier, &contract_text());
}
