//! The port's statement (the record's §14.4, `docs/specs/catalog/decision_catalog-records-port.md`,
//! `M2.12.4.3`): what the record supplying `switch` states of the fault contract's open points, and the
//! check-passing convention matched across records.
//!
//! > Under a selection where a record supplies `switch`, that record supplies every fact of the table that has no
//! > read condition, or whose read condition holds, `one-claim-per-trap`, and both costs, or the catalog is refused
//! > at load.
//!
//! The table's names, which of them are obligatory and which are read only under a condition, live here once; the
//! reader ([`crate::record`]) refuses what one record's text shows, and the selection ([`crate::selection`]) what
//! needs the catalog under a selection.

use crate::record::{Fact, FactValue};

/// The facts of §14.4's table, in its order. Each is a behavioral fact in the `switch` group (§12).
pub const FACTS: [&str; 23] = [
    "detects-non-job-calls",
    "services-preempt-completion-interval",
    "completion-decision-remade",
    "guard-check-contexts",
    "generated-guard-check-contexts",
    "guarded-stacks",
    "decision-placement",
    "api-entry-by-trap",
    "api-trap-preemptible-before-decode",
    "primitives-preemptible",
    "unmask-preemptible-before-check",
    "fault-window",
    "window-trap-preempts-outside",
    "window-trap-preempts-inside",
    "window-release-abandons",
    "window-primitive-consistent",
    "abandoned-primitive-completion",
    "fault-path-entries",
    "checks-trap",
    "traps-discriminated",
    "kept-record-readout",
    "panic-strategy-abort",
    "vector-direct",
];

/// The table's two facts that are no code fact, each taking a `file` locator and no other.
pub const NOT_CODE: [&str; 2] = ["generated-guard-check-contexts", "panic-strategy-abort"];

/// The facts the table marks obligatory: a known `no` of one is refused.
pub const OBLIGATORY: [&str; 12] = [
    "detects-non-job-calls",
    "guard-check-contexts",
    "generated-guard-check-contexts",
    "guarded-stacks",
    "decision-placement",
    "fault-window",
    "abandoned-primitive-completion",
    "fault-path-entries",
    "traps-discriminated",
    "kept-record-readout",
    "panic-strategy-abort",
    "vector-direct",
];

/// The fatal path's bound: two timing costs of the record supplying `switch`, in its group.
pub const COSTS: [&str; 2] = ["fatal-path.entry", "fatal-path.trap"];

/// The id prefix of a check-passing convention record.
pub const CONVENTION_PREFIX: &str = "convention.check-passing.";

/// The prefix of the fact a convention record states of itself.
pub const CONVENTION_STATED: &str = "convention-stated.";

/// The prefix of the fact another record states of its own guard checks.
pub const GUARD_CHECK_CONTEXTS: &str = "guard-check-contexts.";

/// How a read condition combines its facts.
#[derive(Debug, Clone, Copy)]
enum Condition {
    /// The one fact.
    Yes(&'static str),
    /// Both facts.
    And(&'static str, &'static str),
    /// At least one of the two.
    Or(&'static str, &'static str),
}

/// The facts read only under a condition, each with it, as the table's "Read only when" says.
const CONDITIONS: [(&str, Condition); 6] = [
    (
        "completion-decision-remade",
        Condition::Yes("services-preempt-completion-interval"),
    ),
    (
        "api-trap-preemptible-before-decode",
        Condition::Yes("api-entry-by-trap"),
    ),
    (
        "unmask-preemptible-before-check",
        Condition::Yes("primitives-preemptible"),
    ),
    (
        "window-trap-preempts-inside",
        Condition::Yes("primitives-preemptible"),
    ),
    (
        "window-release-abandons",
        Condition::Or(
            "window-trap-preempts-outside",
            "window-trap-preempts-inside",
        ),
    ),
    (
        "window-primitive-consistent",
        Condition::And("window-release-abandons", "window-trap-preempts-inside"),
    ),
];

/// Whether `facts` state `name` `yes`.
fn stated_yes(facts: &[Fact], name: &str) -> bool {
    facts
        .iter()
        .any(|f| f.name == name && matches!(f.value, FactValue::Known { holds: true, .. }))
}

/// Whether fact `name`'s read condition holds of `facts`: its facts, combined as it says, each for `and` and at
/// least one for `or`, stated `yes`, a fact absent or `unknown` counting as not `yes`. A fact with no read
/// condition is always read.
#[must_use]
pub fn condition_holds(name: &str, facts: &[Fact]) -> bool {
    match CONDITIONS.iter().find(|(f, _)| *f == name).map(|(_, c)| *c) {
        None => true,
        Some(Condition::Yes(a)) => stated_yes(facts, a),
        Some(Condition::And(a, b)) => stated_yes(facts, a) && stated_yes(facts, b),
        Some(Condition::Or(a, b)) => stated_yes(facts, a) || stated_yes(facts, b),
    }
}

/// The facts a record supplying `switch` must supply, given its own: every fact of the table with no read
/// condition or whose condition holds, and `one-claim-per-trap`.
#[must_use]
pub fn required(facts: &[Fact]) -> Vec<&'static str> {
    FACTS
        .iter()
        .copied()
        .filter(|name| condition_holds(name, facts))
        .chain(["one-claim-per-trap"])
        .collect()
}

/// Whether `name` is one of the self-named families, and the id it names: empty for a name that ends at the
/// family's prefix, which names no record and so is refused as another record's.
#[must_use]
pub fn self_named(name: &str) -> Option<&str> {
    [CONVENTION_STATED, GUARD_CHECK_CONTEXTS]
        .iter()
        .find_map(|prefix| name.strip_prefix(prefix))
}

/// Whether a fact named `name` takes a `file` locator and no other (§2, §14.4).
#[must_use]
pub fn file_located(name: &str) -> bool {
    NOT_CODE.contains(&name) || name.starts_with(CONVENTION_STATED)
}

/// Whether `id` is a check-passing convention record's. An id never ends in `.` (§1), so one with the prefix names
/// a convention.
#[must_use]
pub fn is_convention(id: &str) -> bool {
    id.starts_with(CONVENTION_PREFIX)
}
