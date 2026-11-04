//! The runtime plan's bounded exhaustive checker (leaf `M4.1.13`), which
//! `docs/decisions/decision_executable-design-reviews.md` item 1 asks of a design whose subject is a decision
//! procedure: it "enumerates a finite universe of inputs" and holds each outcome to the record. Universes of small
//! descriptions are generated in memory, every combination of a few independent choices, so that each is exhaustive
//! over its bounds: three services and their `uses` edges; two services with adapter instances; releases and routes of
//! every separation class; requesters of every kind; two tasks, periodic and sporadic, sharing services that take a
//! channel, with an obliged `delivery-bound` bound and bare and each task writing `(deadline-from release)` or not;
//! receivers of interrupt code, two tasks and the system's own level each programming an alarm itself, receiving an
//! expiry or neither, in both runtime orders; after round 12 (R12-R4), blocks and platforms with requirements of their
//! own, bound to by something or by nothing; every jitter, overrun policy and deadline reference a task line states;
//! durations past the exact arithmetic, each kind named in §1's order, and durations at its edge, written exactly
//! however they are spelled; and a second system, named by `uses` or `needs` or not, each lowered; after round 13
//! (R13-R1 to R13-R3), blocks and platforms whose `uses` or `needs` name a service, a policy, a system or another
//! block, alone, several at once or beside each other refusal of §1 that can co-apply, on the lowered system's hardware
//! or on another's; a block a service names, served though not the system's hardware, bound for requirements of its
//! own or not, model §8 rule 7's conflict since `M1.49`; and requesters naming several systems, held to the
//! naming-system refusal's order; and, after round 14 (R14-D1,
//! R14-R1, R14-R6), names one of which prefixes another — `m` and `m.s`, `y` and `y.b`, `d` and `d.x` — and names
//! continuing with `!`, a byte below `)`, so that each order §1 holds by text is held where a message's words would
//! give another: the naming-system refusal's namers and systems, tasks among them; the hardware-naming refusal's
//! blocks, platforms and operands, a platform's `needs` naming a service and its `uses` a policy, and a system named on
//! hardware the lowered system does not serve; the sources and the bounds past the arithmetic, and one requester's two
//! bounds on its instance's one obliged fact; receivers of interrupt code over two providers one of whose names
//! prefixes the other's, each requester programming an alarm, a deadline or both, which the search refuses
//! (`missing-fact`) unless one provider offers both, so that no system it resolves gives one receiver two things; and a
//! provider whose own requirement is bound to the node that takes its channel, whose own route then counts among the
//! node's users; and, after round 15 (R15-D2, R15-R2, R15-R5), tasks, the system's own level and a service bound to a
//! timer route's provider by a binding that takes no channel (model §8 rule 2) — a shared fact, the route naming them
//! through an instance or not at all, or a consumed fact met by `false` — beside one programming an alarm or receiving
//! an expiry there, in both runtime orders; periodic tasks bound to a source's block for another fact than
//! `interrupt-source`, beside one bound for it, in both runtime orders, and `interrupt-source` met by `false`, which is no
//! source; hardware the lowered system does not serve, another system's or none's, naming a service the lowered system
//! serves, a policy, a system or the lowered system itself, which neither naming refusal takes, each description
//! lowered for each of its systems; a platform the lowered system serves naming a system by `uses` or by `needs`; and a
//! node in a job naming by `needs` a node no job reaches; and, after round 16 (R16-D3, R16-R1, R16-R2, R16-B1,
//! R16-B2), sporadic tasks bound themselves for `interrupt-source`, beside the routes of another source and of their
//! own source's handler they reach, and beside a task binding none, released by those routes; a platform and the
//! blocks it composes, directly or through another block, each bound to the other, one to the other or neither, under
//! names that order each composite before its part or after it; and the system's own level, a service, a policy, a
//! platform and a block each served by an exclusive `deadline-over-delay` instance, writing its obliged bound bare,
//! within the arithmetic or past it, a platform's, a block's, a policy's and a service's bound within it each counted
//! on its instance's line; and, after round 17 (R17-D1, R17-R2, R17-R6), a block and a platform that a service and a
//! policy name, by `uses` or by `needs`, under names that order each namer before what it names or after it, the named
//! block or platform a provider, bound for requirements of its own, both or neither, composed by the platform or served
//! through its namer alone, the namers used by a task, by the system's own level alone, or the policy by nothing; and a
//! timer and a source a service or a policy names, bound to its namer for an alarm, for its interrupt, through the
//! instance it programs or not at all, whose own interrupt runs that namer — so that rule 12's named edge from hardware
//! to its namer is held, alone or beside a binding's reason, and each node in a job owes its call cost and its masked
//! runs; and, after round 18 (R18-D1, R18-D3, R18-R5), a block and a platform requiring nothing that a task, a service,
//! a policy, a source's handler and the system's own level each name or not, by `uses` or by `needs`, composed by the
//! platform or by nothing, the services used by the task or by the system's own level alone, bound to by nothing or by
//! an unrelated requester alone — so that hardware a served task, service or policy names is a node bound or not, with
//! its contexts, step, edges and owed facts, which a binding elsewhere leaves as the name gives them, the system's own
//! level's name and a platform's composition making none, and a node owes its bounds with no category of their own;
//! and, after round 19 (R19-D2), no universe of its own but every task of every universe owing the application its
//! `os-calls-category` beside its `os-calls`, each of the application's bounds and counts owed with its evidence
//! category a fact of its own, the reading's facts of a task, a source and a timer route held to §2.10's table and §3's
//! `FACT` list as the record writes them; and, after round 20 (R20-D3, R20-R7), a block or a platform the system serves
//! outside its hardware — one a served service or policy names that model §8 rule 7 makes no hardware — which round 20
//! had §1 refuse and which, since `M1.49` (`RTP-H22`), is that rule's conflict of each system serving it, the search's
//! `infeasible-configuration`, beside a task's, the system's own and a platform's `needs` making such a block hardware,
//! a namer the system does not serve, every refusal of §1 it leaves unreached, a system that declares no task, and one
//! service two systems serve, the name inside one's hardware and outside the other's;
//! a task's period, minimum separation, deadline and jitter that is not a time, in every unit the language accepts that
//! is no time, and a period or a minimum separation of zero, alone and two at once, which round 20 had §1's bullet over
//! tasks refuse and which, since `M1.42` (`RTP-H14`), the language rejects, `invalid-description`, whatever else the
//! system writes, before the search and every refusal of §1; beside them a zero deadline, which it accepts, a task
//! stating no release or no deadline left to the language, and what the bullet keeps, a periodic task bound to its
//! source, taken task by task in runtime order; ranks that start past 1 and skip, spelled
//! in every way the language reads an integer; and two timer routes, an expiry's and a wake's costs on one route, a
//! source and a timer route at one provider, and a component holding an instance, each a class of its own; and, after
//! round 23 (R23-D1, R23-R2, R23-R7), a system declaring two tasks of one name — one name carried twice or three times,
//! two names one of which prefixes the other, a name continuing with `!`, a later task of a name alone using a service
//! or programming an alarm — which since `M1.50` (`RTP-H23`) the language rejects, `invalid-description`, whatever else
//! it writes, before the search and every refusal of §1, beside its twin with every name distinct, lowered or refused
//! by what else it writes alone; a service and a policy naming themselves
//! by `uses` or by `needs`, taking a channel or not, the service handling a source or not, named by a task, by two, by
//! the system's own level alone or by nothing else, each plan held line by line to its twin's, which names itself not
//! at all; and an obliged fact's bound written `exactly` — alone, beside a looser and an equal `at-most`, one value in
//! two spellings, at the arithmetic's edge and past it — at every kind of requester, beside `at-most` and bare, with
//! the three writings the side reader refuses left to the search's answer; and, after round 24 (R24-D1, R24-D2,
//! R24-R1, R24-R2), two tasks, both sporadic or both periodic, one, the other, both or neither naming a plain block, a
//! source's block, the block handling that source or the system's platform, by `uses`, by `needs` or by both, before a
//! source of every separation class, each system beside its twin named from the other task, the two plans told apart
//! by their task lines alone, or by those and an owed trigger; a block, a platform, a service and a policy of one name,
//! requiring nothing, a channel or a source, named alike by a task, a service, a policy or the system's own level, one
//! plan but for that node's `(node …)` line; and one name carried three times beside another carried twice, rejected
//! like the rest of its universe; and,
//! after round 28 (R28-D2, R28-R4, R28-R7), two sporadic tasks, each binding a source itself, reaching it through its
//! handler or neither, the first both as well, before that source stating every separation class and a second source
//! the first task's other handler reaches, declared after it or before — so that each of §3's causes was reached, the
//! task's own source's three, `own-faster`, `own-unstated` and `own-absent`, written apart from the three the routes
//! leave — each system beside its twin reaching that source only through a handler, its cause the twin's written
//! `own-`; two tasks bound themselves to one source, one as far apart as it and one released less often; and a task
//! bound itself to a source while the handler it uses reaches another, beside its twin with that source alone; and,
//! after model §8 rule 16 landed (`M4.1.2.3`; `RTP-H24`, R29-R2, R30-R4), every universe read again and the releases,
//! own-sources and own-causes ones held anew, beside one universe of its own, a task's own source at the arithmetic's
//! edge, compared past 128 bits or stating a separation past the arithmetic. A sporadic task bound itself to a source
//! the analysis does not admit — shorter than the task's own separation, declared absent, past the exact arithmetic, or
//! stating none — is bound by the search to one it admits where the description offers one, and otherwise refused,
//! `unsupported-profile` or `missing-fact` as the rule and model §8 rule 6 give it, the reading deriving which from the
//! description and holding the search's verdict to it; and for each such system every record binding such a task to a
//! source the rule does not admit — the record the unamended rules gave, the search's over the description with every
//! source admitted and its `bind` lines over this description's program, with each such task bound in turn to each
//! source the description offers it — is judged by the configuration checker and lowered, each refusing it, the
//! checker under its check (5) naming rule 16, the lowering under §1's bullet of the generator's joint rules, both
//! naming each such task with its source and one code, the lowering its conflicts before its unknowns. So the task's
//! own source's three causes reach no plan, each stated unreachable with that reason (§2.2: "no record the generator
//! accepts binds a task's own source the analysis does not admit"), and the five the routes leave are still reached.
//!
//! Each system is run as the engine runs it, as `common::lowered` does: the language's check, the search, the `/2`
//! writer, and the lowering. **Every `archogen-assignment/2` record the search writes is judged by the configuration
//! checker**, `archogen_plan_check::check::plan_file`, as `crates/eadl-resolve/tests/plan.rs` judges one, and a record
//! it refuses is a finding naming that record. **Two tasks of one system carrying one name** are the language's to
//! reject, `invalid-description` (`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`; reference §7 rule 6; `M1.50`, `RTP-H23`):
//! such a description is no input (§1), so no search, record or lowering reads it, and another verdict, or the language
//! accepting one, would be a finding. **A task's duration that is not a time, and a release of zero**, are the
//! language's to reject alike (`LANGUAGE_REJECTS_A_TASK_S_DURATION`; model §4 rule 8; `M1.42`, `RTP-H14`). **A system's
//! side that contradicts itself, read whole**, is the
//! search's to refuse, `invalid-description` (`SEARCH_READS_THE_SIDE_WHOLE`; model §7 rule 4, §8 rule 1; `RTP-H20`): no
//! record of it reaches the checker, and a record the search wrote for one would be a finding. **A service or a policy
//! the system serves naming a block or a platform outside its hardware** is the search's to refuse too,
//! `infeasible-configuration`, whatever else the description writes (`SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE`;
//! model §8 rule 7; `M1.49`, `RTP-H22`): `Joint::map`'s refusal stops the search, no record of it reaches the checker
//! or the lowering, and a record written for one, or another verdict, would be a finding. **The records binding a
//! task's own source rule 16 does not admit**, which no search writes, are a class of their own (`RULE_16_RECORD`),
//! no finding: each is judged by the configuration checker and lowered, and each
//! refused by both, the codes agreeing. Each plan is held to an **independent reading** of
//! `docs/decisions/decision_runtime-plan.md`, written below as plain, unoptimised Rust from the record's text alone,
//! each rule citing the section it transcribes.
//!
//! ⭐ **The assignment is input.** The reading takes the description and the record's own `bind` lines — what the search
//! chose, never something predicted — and derives from them every line of the plan: §2.1's identity — the format, the
//! profile `/1` lowers, the system, the digest of the program the language's check accepted, the record's sha256 and
//! this engine's catalog — none copied from the record; §2.2's task lines, their index, rank, durations, deadline
//! reference, trigger — an unknown one with its cause, the source the task binds itself written apart from the one the
//! routes leave — and `(uses …)` item, every node the task names, hardware among
//! them; §2.3's `bind` lines, the record's copied, the one part of a plan copied, the assignment being the lowering's
//! input; §2.4's instance lines, an obliged fact with its requester's bound or bare; the release timer's line, each
//! route with its separation and rank, the enabled set and the interrupt model (§2.5); §2's `(node …)` lines, each node
//! with its kind; §2.6's `(context …)` lines; §2.7's `(depends …)` and `(step …)` lines, from model §8 rule 12's
//! graph, a component of several members one step; §2.9's obligations, `multiplex` among them; §2.10's owed facts, each
//! task's bounds and counts owed to the application each with a category of its own, every bound owed to the
//! realization with no category beside it; and the last line, `(complete no)`. **Every line of the plan is compared, in
//! its place**: the plan's whole text is held to the reading's, the forms in §3's order across kinds as well as within
//! one, and a line of a head §3 does not write is a disagreement of its own, never dropped. It derives which of §1's
//! refusals is due, where these universes reach one, and holds the lowering's refusal to its code, to the subjects it
//! names — each told by its exact text, a task's `(task SYSTEM NAME)` form or the system's own level's name as a word
//! of its own, never by a substring — in §1's order, each kind by its texts compared byte by byte, "never by its
//! message's words"; the pairs of the refusals naming pairs read from the names a message writes as code, in their
//! order, whatever words join them; to the clause it names, and no other, where §1 names one; to what each receiver
//! would receive, in §1's order — its alarms, then its expiries; and, of the generator's joint rules refusing a record
//! under model §8 rule 16, to each finding, a task, its source and a code, the conflicts before the unknowns. A task is
//! found by its text, unique in its system since the language rejects two tasks of one name, so no reading of a task by
//! its text ever meets such a system, and a text two tasks share fails the reading rather than merging them
//! (`side_of`). The reading is itself held to the
//! record's three worked texts (§3), whole, lowered from the `/2` records the record states for them; its owed facts to
//! §2.10's table and §3's `FACT` list, its forms, node kinds and causes to §3's own lists, and its causes of one source
//! to §2.2's pairing of the routes' with the task's own, the own three to §2.2's statement that no accepted record
//! reaches them; and its readers of a refusal's prose to messages written in words of their own, reworded and
//! misordered.
//!
//! ⛔ **Independence.** The reading was written by contexts that never read `crates/archogen-plan/src/lib.rs` nor the
//! corpus's outcomes. A disagreement is a finding, never something to make disappear: decide from the record's text
//! which side is right, and keep a reading the record supports failing on its system.
//!
//! Each universe's size is asserted — the systems generated and how each fared, lowered or not — with every system the
//! search resolves judged by the configuration checker, and each class of the record's rules it is built to reach held
//! to the number of systems its construction fixes, counted from each case's parameters or derived by hand, or, where
//! the search's choice or the record's rules over the description decide it, to some, so that a generator cannot shrink
//! unnoticed; a system the language or the search refuses is counted, never dropped. A
//! branch of the reading that no system can reach is stated so, its reason in its class's name, `unreachable: …`, and a
//! universe reaching one fails. Where the record says what a binding or a name elsewhere leaves unchanged, each
//! system's plan is also held line by line to its twin's.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;
use archogen_plan::{lower, Refusal};
use archogen_plan_check::check::plan_file;
use eadl_front::module::DirectoryModules;
use eadl_front::{Form, Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::Offered;
use eadl_resolve::plan;
use eadl_resolve::search::{search, Limits, Policy, Search};
use eadl_resolve::vocabulary::Usage;

// =====================================================================================================================
// The descriptions, generated in memory
// =====================================================================================================================

/// A duration as a description spells it: a whole number of milliseconds, or a spelling written as given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dur {
    Ms(u64),
    Spelled(&'static str),
}

/// A time of `1e-47 s`, whose denominator lies past signed 128-bit integers (§2.2), which the language still reads:
/// it reads a decimal of at most 38 fraction digits whose digits make at most `i64::MAX`.
const PAST: &str = "0.00000000000000000000000000000000000001 ns";
/// `9223372036854775807/10^38 s`: within the arithmetic, its digits the most the language reads, and compared with
/// [`WIDE_LONGER`] by cross-multiplication past 128 bits.
const WIDE: &str = "0.00000000000000000009223372036854775807 s";
/// [`WIDE`] in milliseconds: the same time, spelled otherwise.
const WIDE_IN_MS: &str = "0.00000000000000009223372036854775807 ms";
/// `922337203685477581/10^37 s`, a little longer than [`WIDE`].
const WIDE_LONGER: &str = "0.0000000000000000000922337203685477581 s";
/// `1/10^38 s`, spelled with a denominator of `10^47` that only its reduction brings within the arithmetic.
const REDUCED: &str = "0.00000000000000000000000000001000000000 ns";
/// `5^13/10^47 s = 1/(8192·10^34) s`: its reduced denominator, `8.192e37`, the nearest below `i128::MAX` a spelling
/// of the language reaches.
const TIGHT_WITHIN: &str = "0.00000000000000000000000000001220703125 ns";
/// `5^12/10^47 s = 1/(4096·10^35) s`: its reduced denominator, `4.096e38`, the nearest above `i128::MAX`.
const TIGHT_PAST: &str = "0.00000000000000000000000000000244140625 ns";

impl Dur {
    fn spelling(self) -> String {
        match self {
            Self::Ms(ms) => format!("{ms} ms"),
            Self::Spelled(text) => text.to_string(),
        }
    }

    /// What it is as a duration: a time, a time past the arithmetic, or a quantity of another dimension.
    fn time(self) -> Time {
        Time::of(&self.spelling())
    }

    /// Its seconds (§2.2), or `None` where they lie past the engine's exact arithmetic. A quantity that is not a time
    /// is rejected by the language before any plan (model §4 rule 8), so it is read nowhere this is.
    fn seconds(self) -> Option<Ratio> {
        match self.time() {
            Time::Seconds(time) => Some(time),
            Time::Past => None,
            Time::Other(unit) => {
                panic!(
                    "`{}` is in `{unit}`, not a time, which the language rejects before any plan",
                    self.spelling()
                )
            }
        }
    }
}

/// The language's units of time (model §1 rule 2: "Time is `s`, `ms`, `us` and `ns`"), each with the power of ten it
/// lies below a second.
const TIME_UNITS: [(&str, u32); 4] = [("s", 0), ("ms", 3), ("us", 6), ("ns", 9)];

/// Every other unit the language accepts, none of them a time: model §1 rule 2's frequencies, `Hz`, `kHz`, `MHz` and
/// `GHz`, and information, `bit` and `byte`, and the language's table's `KiB`, `MiB` and `tick` beside them
/// (`eadl_model::quantity::UNITS`, held to these by `the_reading_transcribes_the_shipped_vocabulary_and_catalog`).
/// Model §4 rule 8 has a task's `period`, `min-separation`, `deadline` and `jitter` be times (`M1.42`, `RTP-H14`), so
/// the reading derives the language's refusal of each of these and panics on none of them (round 20, R20-R7).
const OTHER_UNITS: [&str; 9] = [
    "Hz", "kHz", "MHz", "GHz", "bit", "byte", "KiB", "MiB", "tick",
];

/// The class of the language rejecting a task's duration in each of [`OTHER_UNITS`], in its order.
const NOT_A_TIME_IN: [&str; 9] = [
    "language: not a time, in `Hz`",
    "language: not a time, in `kHz`",
    "language: not a time, in `MHz`",
    "language: not a time, in `GHz`",
    "language: not a time, in `bit`",
    "language: not a time, in `byte`",
    "language: not a time, in `KiB`",
    "language: not a time, in `MiB`",
    "language: not a time, in `tick`",
];

/// A quantity read as a duration (§2.2; model §4 rule 8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Time {
    /// A time, its reduced seconds within the arithmetic.
    Seconds(Ratio),
    /// A time whose seconds lie past the arithmetic.
    Past,
    /// A quantity of another dimension, in this unit, which model §4 rule 8 refuses of a task's durations.
    Other(&'static str),
}

impl Time {
    /// What `NUMBER UNIT` spells, the unit one the language accepts.
    fn of(spelling: &str) -> Self {
        let (_, unit) = spelling.split_once(' ').expect("NUMBER UNIT");
        if TIME_UNITS.iter().any(|(time, _)| *time == unit) {
            return Ratio::of(spelling).map_or(Self::Past, Self::Seconds);
        }
        Self::Other(
            OTHER_UNITS
                .iter()
                .find(|other| **other == unit)
                .unwrap_or_else(|| panic!("the language accepts no unit `{unit}`")),
        )
    }
}

/// A reduced non-negative rational number of seconds, its numerator and denominator within signed 128-bit integers
/// (§2.2: "`N` and `D` lie within the engine's exact arithmetic, signed 128-bit integers").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ratio {
    n: u128,
    d: u128,
}

/// `a * b`, exactly, as its high and low 128 bits.
fn wide_product(a: u128, b: u128) -> (u128, u128) {
    const LOW: u128 = u64::MAX as u128;
    let (a1, a0) = (a >> 64, a & LOW);
    let (b1, b0) = (b >> 64, b & LOW);
    let (p00, p01, p10, p11) = (a0 * b0, a0 * b1, a1 * b0, a1 * b1);
    let middle = (p00 >> 64) + (p01 & LOW) + (p10 & LOW);
    let low = (p00 & LOW) | (middle << 64);
    let high = p11 + (p01 >> 64) + (p10 >> 64) + (middle >> 64);
    (high, low)
}

impl Ord for Ratio {
    /// Exact: `n1/d1` against `n2/d2` by `n1·d2` against `n2·d1`, each product in 256 bits.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        wide_product(self.n, other.d).cmp(&wide_product(other.n, self.d))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ratio {
    const ZERO: Self = Self { n: 0, d: 1 };

    /// The seconds `NUMBER UNIT` spells, reduced, or `None` where the reduced numerator or denominator lies past
    /// `i128::MAX`.
    fn of(spelling: &str) -> Option<Self> {
        let (number, unit) = spelling.split_once(' ').expect("NUMBER UNIT");
        let exponent: u32 = TIME_UNITS
            .iter()
            .find(|(time, _)| *time == unit)
            .map(|(_, exponent)| *exponent)
            .unwrap_or_else(|| panic!("`{unit}` is no unit of time; read it with `Time::of`"));
        let (whole, fraction) = number.split_once('.').unwrap_or((number, ""));
        let mut n: u128 = format!("{whole}{fraction}").parse().expect("digits");
        if n == 0 {
            return Some(Self::ZERO);
        }
        // The denominator is `10^k = 2^k 5^k`: reduce by every 2 and 5 the numerator shares.
        let k = u32::try_from(fraction.len()).expect("short") + exponent;
        let (mut twos, mut fives) = (k, k);
        while twos > 0 && n.is_multiple_of(2) {
            n /= 2;
            twos -= 1;
        }
        while fives > 0 && n.is_multiple_of(5) {
            n /= 5;
            fives -= 1;
        }
        let d = 2u128
            .checked_pow(twos)?
            .checked_mul(5u128.checked_pow(fives)?)?;
        let max = u128::try_from(i128::MAX).expect("positive");
        (n <= max && d <= max).then_some(Self { n, d })
    }

    /// §2.2: "Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1`".
    fn text(self) -> String {
        if self.d == 1 {
            format!("{} s", self.n)
        } else {
            format!("{}/{} s", self.n, self.d)
        }
    }
}

/// What a source's block states of its own `min-arrival-separation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Separation {
    /// Stated, in milliseconds.
    Stated(u64),
    /// Not named at all.
    Unstated,
    /// Declared absent.
    Absent,
    /// Stated once in each of these spellings.
    Spelled(&'static [&'static str]),
}

/// What the reading takes of a source's separation (§2.5, §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sep {
    Stated(Ratio),
    Unstated,
    Absent,
    /// A spelling lies past the exact arithmetic: one alone, or one of two spellings, whose comparison then overflows
    /// it (round 12, R12-D2: "a source separation whose spellings overflow the arithmetic" is "Refused past the
    /// arithmetic, as §1 says").
    Past,
}

impl Separation {
    fn read(self) -> Sep {
        match self {
            Self::Stated(ms) => Sep::Stated(Dur::Ms(ms).seconds().expect("a few milliseconds")),
            Self::Unstated => Sep::Unstated,
            Self::Absent => Sep::Absent,
            Self::Spelled(spellings) => {
                let read: Option<Vec<Ratio>> = spellings.iter().map(|s| Ratio::of(s)).collect();
                match read {
                    None => Sep::Past,
                    Some(times) => {
                        assert!(
                            times.windows(2).all(|w| w[0] == w[1]),
                            "the universes spell one separation, never two"
                        );
                        Sep::Stated(times[0])
                    }
                }
            }
        }
    }
}

/// A block, or a platform: its offers, as written, what it requires at its own level, and what the reading needs of
/// it. Its `uses` is composition, never a binding (model §8 rule 1).
#[derive(Debug, Clone, Default)]
struct Block {
    name: String,
    offers: Vec<String>,
    absent: Vec<String>,
    /// For a block offering `interrupt-source`: what it states of its separation.
    separation: Option<Separation>,
    /// Its `requires` clause: what it needs, and what it composes by `uses`.
    side: Side,
}

impl Block {
    /// Its form, `defblock` or `defplatform`.
    fn text(&self, head: &str) -> String {
        let mut out = format!("({head} {} (offers {})", self.name, self.offers.join(" "));
        if !self.absent.is_empty() {
            out.push_str(&format!(" (absent {})", self.absent.join(" ")));
        }
        let requires = self.side.clauses();
        if !requires.is_empty() {
            out.push_str(&format!(" (requires{requires})"));
        }
        out.push_str(")\n");
        out
    }
}

/// What one requester writes: the vocabulary facts its `needs` lists, the declarations its `uses` lists, each
/// `(FACT (at-most DURATION))` and `(FACT (exactly DURATION))` bound it writes, and any other requirement clause, as
/// written.
#[derive(Debug, Clone, Default)]
struct Side {
    needs: Vec<String>,
    uses: Vec<String>,
    /// Each `(FACT (at-most DURATION))` the requester writes.
    at_most: Vec<(String, Dur)>,
    /// Each `(FACT (exactly DURATION))` the requester writes (round 23, R23-R7).
    exactly: Vec<(String, Dur)>,
    other: Vec<String>,
}

impl Side {
    /// The side's clauses, each preceded by a space.
    fn clauses(&self) -> String {
        let mut out = String::new();
        for clause in &self.other {
            out.push(' ');
            out.push_str(clause);
        }
        for (fact, bound) in &self.at_most {
            out.push_str(&format!(" ({fact} (at-most {}))", bound.spelling()));
        }
        for (fact, bound) in &self.exactly {
            out.push_str(&format!(" ({fact} (exactly {}))", bound.spelling()));
        }
        if !self.needs.is_empty() {
            out.push_str(&format!(" (needs {})", self.needs.join(" ")));
        }
        if !self.uses.is_empty() {
            out.push_str(&format!(" (uses {})", self.uses.join(" ")));
        }
        out
    }
}

/// A service, or a policy (`defpolicy`), which §2.9 holds alike.
#[derive(Debug, Clone)]
struct Service {
    name: String,
    policy: bool,
    side: Side,
}

/// A task's release model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Release {
    Period(Dur),
    MinSeparation(Dur),
}

impl Release {
    fn duration(self) -> Dur {
        match self {
            Self::Period(d) | Self::MinSeparation(d) => d,
        }
    }

    /// Its clause's name, which §1 names a duration past the arithmetic by.
    fn clause(self) -> &'static str {
        match self {
            Self::Period(_) => "period",
            Self::MinSeparation(_) => "min-separation",
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    release: Release,
    /// Its deadline, where it is not its release's duration.
    deadline: Option<Dur>,
    /// Its `jitter` clause, where it writes one.
    jitter: Option<Dur>,
    /// Its `on-overrun` clause, where it writes one.
    overrun: Option<&'static str>,
    priority: u32,
    /// Its `priority` as spelled, where it is not [`Task::priority`] written in decimal: any `integer` or `hexadecimal`
    /// `docs/semantics/grammar.md` reads — a sign, leading zeros, `_` between digits, `0x` — beyond 32 bits too.
    spelled_priority: Option<&'static str>,
    /// The event its `deadline-from` clause names, where it writes one.
    deadline_from: Option<&'static str>,
    /// A clause it does not write, `period` (its release, of either kind) or `deadline`: §1's "none stated, which it
    /// refuses already", the language's answer.
    omitted: Option<&'static str>,
    side: Side,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            name: "t".to_string(),
            release: Release::Period(Dur::Ms(10)),
            deadline: None,
            jitter: None,
            overrun: None,
            priority: 1,
            spelled_priority: None,
            deadline_from: Some("release"),
            omitted: None,
            side: Side::default(),
        }
    }
}

/// The value of a number `docs/semantics/grammar.md` reads as an `integer` or a `hexadecimal`: `[ sign ]`, then
/// `digit { digit | "_" }`, or `("0x" | "0X") hex_digit { hex_digit | "_" }`.
fn integer_value(spelling: &str) -> u128 {
    let unsigned = spelling.strip_prefix('+').unwrap_or(spelling);
    assert!(!unsigned.starts_with('-'), "a rank is never negative");
    let digits: String = unsigned.chars().filter(|c| *c != '_').collect();
    match digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        Some(hex) => u128::from_str_radix(hex, 16).expect("hexadecimal digits"),
        None => digits.parse().expect("decimal digits"),
    }
}

impl Task {
    fn deadline(&self) -> Dur {
        self.deadline.unwrap_or(self.release.duration())
    }

    /// Its rank, the value of its `priority` (`decision_priority-comparison-direction.md`: "`N` is a rank"; model §4
    /// rule 5: "an integer from 1, and 1 is the highest", "the ranks of a system need not be contiguous").
    fn rank(&self) -> u128 {
        self.spelled_priority
            .map_or(u128::from(self.priority), integer_value)
    }

    fn text(&self) -> String {
        format!(
            "  (task {}{}{}{}{}{} (priority {}){})\n",
            self.name,
            if self.omitted == Some("period") {
                String::new()
            } else {
                format!(
                    " ({} {})",
                    self.release.clause(),
                    self.release.duration().spelling()
                )
            },
            if self.omitted == Some("deadline") {
                String::new()
            } else {
                format!(" (deadline {})", self.deadline().spelling())
            },
            self.jitter
                .map_or(String::new(), |j| format!(" (jitter {})", j.spelling())),
            self.deadline_from
                .map_or(String::new(), |e| format!(" (deadline-from {e})")),
            self.overrun
                .map_or(String::new(), |p| format!(" (on-overrun {p})")),
            self.spelled_priority
                .map_or_else(|| self.priority.to_string(), str::to_string),
            self.side.clauses()
        )
    }
}

/// A system of a description: its tasks, its own level and the platform it uses.
#[derive(Debug, Clone)]
struct System {
    name: String,
    tasks: Vec<Task>,
    own: Side,
    platform: String,
}

impl System {
    fn text(&self) -> String {
        let mut out = format!("(defsystem {}\n", self.name);
        for task in &self.tasks {
            out.push_str(&task.text());
        }
        let own = self.own.clauses();
        if !own.is_empty() {
            out.push_str(&format!("  (requires{own})\n"));
        }
        out.push_str(&format!("  (platform (uses {})))\n", self.platform));
        out
    }
}

/// One description and the system of it lowered: by default on one platform `soc` that uses every block.
#[derive(Debug, Clone)]
struct Description {
    system: String,
    blocks: Vec<Block>,
    /// Its platforms; none writes the one `soc` composing every block.
    platforms: Vec<Block>,
    services: Vec<Service>,
    tasks: Vec<Task>,
    /// The system's own level.
    own: Side,
    /// The platform the system's `platform` clause uses.
    platform: String,
    /// The description's other systems, written after the one lowered, or before it where `others_first`.
    others: Vec<System>,
    others_first: bool,
}

impl Default for Description {
    fn default() -> Self {
        Self {
            system: "sys".to_string(),
            blocks: Vec::new(),
            platforms: Vec::new(),
            services: Vec::new(),
            tasks: Vec::new(),
            own: Side::default(),
            platform: "soc".to_string(),
            others: Vec::new(),
            others_first: false,
        }
    }
}

impl Description {
    /// Its platforms: those written, or the one `soc` composing every block.
    fn platforms(&self) -> Vec<Block> {
        if !self.platforms.is_empty() {
            return self.platforms.clone();
        }
        vec![Block {
            name: "soc".to_string(),
            offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
            side: Side {
                uses: self.blocks.iter().map(|b| b.name.clone()).collect(),
                ..Side::default()
            },
            ..Block::default()
        }]
    }

    /// Every block and platform it declares.
    fn hardware(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .chain(self.platforms())
            .collect()
    }

    /// Every system it declares, the one lowered first.
    fn systems(&self) -> Vec<System> {
        std::iter::once(self.lowered())
            .chain(self.others.iter().cloned())
            .collect()
    }

    /// The system lowered, as a [`System`].
    fn lowered(&self) -> System {
        System {
            name: self.system.clone(),
            tasks: self.tasks.clone(),
            own: self.own.clone(),
            platform: self.platform.clone(),
        }
    }

    /// The description's text, as the language reads it.
    fn text(&self) -> String {
        let mut out = String::from("(eadl-version eadl/1)\n");
        for block in &self.blocks {
            out.push_str(&block.text("defblock"));
        }
        for platform in self.platforms() {
            out.push_str(&platform.text("defplatform"));
        }
        for service in &self.services {
            // A service writes its `requires` clause, empty where it requires nothing: the language asks for one.
            out.push_str(&format!(
                "({} {} (requires{}))\n",
                if service.policy {
                    "defpolicy"
                } else {
                    "defservice"
                },
                service.name,
                service.side.clauses()
            ));
        }
        if self.others_first {
            for other in &self.others {
                out.push_str(&other.text());
            }
        }
        out.push_str(&self.lowered().text());
        if !self.others_first {
            for other in &self.others {
                out.push_str(&other.text());
            }
        }
        out
    }
}

/// One generated system: what it varies, and its description.
#[derive(Debug, Clone)]
struct Case {
    label: String,
    description: Description,
}

fn names(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

/// A source's block: `interrupt-source`, its separation as `separation` says, and the `available-in-state` it offers.
fn source_block(name: &str, separation: Separation, states: &str) -> Block {
    let mut offers = names(&["(interrupt-source true)"]);
    match separation {
        Separation::Stated(ms) => offers.push(format!("(min-arrival-separation {ms} ms)")),
        Separation::Spelled(spellings) => {
            for spelling in spellings {
                offers.push(format!("(min-arrival-separation {spelling})"));
            }
        }
        Separation::Unstated | Separation::Absent => {}
    }
    if !states.is_empty() {
        offers.push(format!("(available-in-state {states})"));
    }
    offers.push("(reachable-at-privilege machine)".to_string());
    Block {
        name: name.to_string(),
        offers,
        absent: if separation == Separation::Absent {
            names(&["min-arrival-separation"])
        } else {
            Vec::new()
        },
        separation: Some(separation),
        side: Side::default(),
    }
}

/// One block raising both kinds of interrupt (§2: "one provider can raise a timer route and a source"): a timer with
/// what both adapter kinds need beneath, and a source stating 20 ms.
fn timer_and_source_block(name: &str, channels: u32) -> Block {
    let mut block = timer_block(name, channels, true);
    let at = block.offers.len() - 1;
    block
        .offers
        .insert(at, "(min-arrival-separation 20 ms)".to_string());
    block
        .offers
        .insert(at, "(interrupt-source true)".to_string());
    block.separation = Some(Separation::Stated(20));
    block
}

/// A timer: a block offering `relative-delay` with `channels` channels and, for `adapters`, what both adapter kinds
/// need beneath (`docs/semantics/adapters/adapters.eadl`) — but `counter-width`, so that an epoch extender's requester
/// is met by no block.
fn timer_block(name: &str, channels: u32, adapters: bool) -> Block {
    let mut offers = names(&["(relative-delay true)"]);
    if adapters {
        offers.extend(names(&[
            "(counter-modulus 65536)",
            "(tick-rate 10 MHz)",
            "(wrap-behavior modular)",
            "(observation-coherent true)",
        ]));
    }
    offers.push(format!("(channels {channels} tick)"));
    offers.push("(reachable-at-privilege machine)".to_string());
    Block {
        name: name.to_string(),
        offers,
        ..Block::default()
    }
}

/// Every declaration `from` reaches through `uses` and `needs` among `services`, at any depth.
fn uses_closure(services: &[Service], from: &[String]) -> BTreeSet<String> {
    let mut reached = BTreeSet::new();
    let mut todo: Vec<String> = from.to_vec();
    while let Some(at) = todo.pop() {
        if reached.insert(at.clone()) {
            if let Some(service) = services.iter().find(|s| s.name == at) {
                todo.extend(service.side.uses.iter().cloned());
                todo.extend(service.side.needs.iter().cloned());
            }
        }
    }
    reached
}

/// Whether the services, as a universe writes them, name one another round a cycle by `uses` or `needs`: a property of
/// the construction, which the counts of its components read.
fn names_a_cycle(services: &[Service]) -> bool {
    services.iter().any(|s| {
        let from: Vec<String> = s.side.uses.iter().chain(&s.side.needs).cloned().collect();
        uses_closure(services, &from).contains(&s.name)
    })
}

/// The system's own level reaches, by `uses` or by `needs`, each service no task reaches, so that every service is
/// served.
fn own_level(services: &[Service], tasks: &[Task], by_needs: bool) -> Side {
    let from: Vec<String> = tasks
        .iter()
        .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned())
        .collect();
    let reached = uses_closure(services, &from);
    let unreached = services
        .iter()
        .map(|s| s.name.clone())
        .filter(|s| !reached.contains(s))
        .collect();
    named(Side::default(), unreached, by_needs)
}

/// `side` naming `declarations` too, by `needs` or by `uses`.
fn named(mut side: Side, declarations: Vec<String>, by_needs: bool) -> Side {
    if by_needs {
        side.needs.extend(declarations);
    } else {
        side.uses.extend(declarations);
    }
    side
}

/// What a service of the services universes takes of a counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Counter {
    /// Nothing.
    None,
    /// A channel of a timer: `(needs relative-delay)`.
    Timer,
    /// An absolute deadline, which only a `deadline-over-delay` instance offers: `(needs absolute-deadline)`, its
    /// obliged `delivery-bound` left bare.
    Deadline,
    /// The same, writing `(delivery-bound (at-most 5 ms))`, the bound the instance then takes on (§2.4).
    DeadlineBound,
    /// A counter with a horizon no block offers, which only an `epoch-extender` instance meets.
    Extender,
}

/// The bound a [`Counter::DeadlineBound`] requester writes on `delivery-bound`, in milliseconds.
const DELIVERY_BOUND_MS: u64 = 5;

impl Counter {
    fn word(self) -> &'static str {
        match self {
            Self::None => "-",
            Self::Timer => "timer",
            Self::Deadline => "deadline",
            Self::DeadlineBound => "deadline<=5ms",
            Self::Extender => "extender",
        }
    }
}

fn service(name: &str, counter: Counter, source: bool, uses: Vec<String>) -> Service {
    let mut side = Side {
        uses,
        ..Side::default()
    };
    match counter {
        Counter::None => {}
        Counter::Timer => side.needs.push("relative-delay".to_string()),
        Counter::Deadline => side.needs.push("absolute-deadline".to_string()),
        Counter::DeadlineBound => {
            side.needs.push("absolute-deadline".to_string());
            side.at_most
                .push(("delivery-bound".to_string(), Dur::Ms(DELIVERY_BOUND_MS)));
        }
        Counter::Extender => {
            side.other
                .push("(unambiguous-horizon (at-least 60 s))".to_string());
            side.needs.extend(names(&[
                "counter-width",
                "tick-rate",
                "observation-coherent",
            ]));
        }
    }
    if source {
        side.needs.push("interrupt-source".to_string());
    }
    Service {
        name: name.to_string(),
        policy: false,
        side,
    }
}

/// Every ordered pair of distinct indices below `n`: the `uses` edges possible among `n` services.
fn pairs(n: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                out.push((i, j));
            }
        }
    }
    out
}

/// **Services and their callers** (§2.6, §2.9), three services: each takes a channel of the one timer, handles the
/// one source, both or neither; every set of `uses` edges among them, cycles included; a periodic task using every
/// subset of them; the system's own level using each service the task does not reach.
fn three_services() -> Vec<Case> {
    const SERVICES: [&str; 3] = ["a", "b", "c"];
    let edges = pairs(3);
    let mut cases = Vec::new();
    for capabilities in 0..4usize.pow(3) {
        for edge_set in 0..1usize << edges.len() {
            for task_set in 0..1usize << SERVICES.len() {
                let mut services = Vec::new();
                let mut label = String::from("caps=");
                for (i, name) in SERVICES.iter().enumerate() {
                    let cap = capabilities / 4usize.pow(u32::try_from(i).expect("small")) % 4;
                    let counter = if cap & 1 == 1 {
                        Counter::Timer
                    } else {
                        Counter::None
                    };
                    let source = cap & 2 == 2;
                    let uses = edges
                        .iter()
                        .enumerate()
                        .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                        .map(|(_, (_, to))| SERVICES[*to].to_string())
                        .collect();
                    label.push_str(&format!(
                        "{name}:{}{} ",
                        counter.word(),
                        if source { "+source" } else { "" }
                    ));
                    services.push(service(name, counter, source, uses));
                }
                let task_uses: Vec<String> = SERVICES
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| task_set >> i & 1 == 1)
                    .map(|(_, s)| (*s).to_string())
                    .collect();
                let tasks = vec![Task {
                    name: "t".to_string(),
                    release: Release::Period(Dur::Ms(10)),
                    priority: 1,
                    deadline_from: Some("release"),
                    side: Side {
                        uses: task_uses.clone(),
                        ..Side::default()
                    },
                    ..Task::default()
                }];
                let edge_words: Vec<String> = edges
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| edge_set >> k & 1 == 1)
                    .map(|(_, (from, to))| format!("{}>{}", SERVICES[*from], SERVICES[*to]))
                    .collect();
                label.push_str(&format!(
                    "uses=[{}] t=[{}]",
                    edge_words.join(","),
                    task_uses.join(",")
                ));
                let own = own_level(&services, &tasks, false);
                cases.push(Case {
                    label,
                    description: Description {
                        system: "sys".to_string(),
                        blocks: vec![
                            timer_block("timer.d", 3, false),
                            source_block("dev.s", Separation::Stated(20), ""),
                        ],
                        services,
                        tasks,
                        own,
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// The hardware of the adapters universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// One timer with three channels, and a source apart.
    OneTimer,
    /// Two timers with one channel each, and a source apart.
    TwoTimers,
    /// One block that is both the timer, with three channels, and the source.
    Both,
}

/// **Services with adapter instances** (§2.6, §2.9, §2.4), two declarations `a` and `b`, `b` a service or a policy:
/// each takes a channel of a timer, a `deadline-over-delay` instance's channel or an `epoch-extender` instance's
/// counter, or none, and handles the one source or not; every set of edges between them; a periodic task naming every
/// subset of them; every name written by `uses`, or every one by `needs`; one timer and a source, two timers and a
/// source, or one block raising both.
fn two_services_with_adapters() -> Vec<Case> {
    const SERVICES: [&str; 2] = ["a", "b"];
    const COUNTERS: [Counter; 4] = [
        Counter::None,
        Counter::Timer,
        Counter::Deadline,
        Counter::Extender,
    ];
    let edges = pairs(2);
    let subset = |set: usize| -> Vec<String> {
        SERVICES
            .iter()
            .enumerate()
            .filter(|(i, _)| set >> i & 1 == 1)
            .map(|(_, s)| (*s).to_string())
            .collect()
    };
    let mut cases = Vec::new();
    for shape in [Shape::OneTimer, Shape::TwoTimers, Shape::Both] {
        for b_policy in [false, true] {
            for by_needs in [false, true] {
                for capabilities in 0..8usize.pow(2) {
                    for edge_set in 0..1usize << edges.len() {
                        for t_set in 0..1usize << SERVICES.len() {
                            let mut services = Vec::new();
                            let mut label = format!(
                                "shape={shape:?} b={} naming={} caps=",
                                if b_policy { "policy" } else { "service" },
                                if by_needs { "needs" } else { "uses" }
                            );
                            for (i, name) in SERVICES.iter().enumerate() {
                                let cap =
                                    capabilities / 8usize.pow(u32::try_from(i).expect("small")) % 8;
                                let counter = COUNTERS[cap % 4];
                                let source = cap / 4 == 1;
                                let to: Vec<String> = edges
                                    .iter()
                                    .enumerate()
                                    .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                                    .map(|(_, (_, to))| SERVICES[*to].to_string())
                                    .collect();
                                label.push_str(&format!(
                                    "{name}:{}{} ",
                                    counter.word(),
                                    if source { "+source" } else { "" }
                                ));
                                let mut declared = service(name, counter, source, Vec::new());
                                declared.side = named(declared.side, to, by_needs);
                                declared.policy = b_policy && *name == "b";
                                services.push(declared);
                            }
                            let tasks = vec![Task {
                                name: "t".to_string(),
                                release: Release::Period(Dur::Ms(10)),
                                priority: 1,
                                deadline_from: Some("release"),
                                side: named(Side::default(), subset(t_set), by_needs),
                                ..Task::default()
                            }];
                            let edge_words: Vec<String> = edges
                                .iter()
                                .enumerate()
                                .filter(|(k, _)| edge_set >> k & 1 == 1)
                                .map(|(_, (from, to))| {
                                    format!("{}>{}", SERVICES[*from], SERVICES[*to])
                                })
                                .collect();
                            label.push_str(&format!(
                                "edges=[{}] t=[{}]",
                                edge_words.join(","),
                                subset(t_set).join(",")
                            ));
                            let own = own_level(&services, &tasks, by_needs);
                            let blocks = match shape {
                                Shape::OneTimer => vec![
                                    timer_block("timer.d", 3, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                Shape::TwoTimers => vec![
                                    timer_block("timer.d", 1, true),
                                    timer_block("timer.e", 1, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                Shape::Both => vec![timer_and_source_block("dev.t", 3)],
                            };
                            cases.push(Case {
                                label,
                                description: Description {
                                    system: "sys".to_string(),
                                    blocks,
                                    services,
                                    tasks,
                                    own,
                                    ..Description::default()
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    cases
}

/// Who a source of the releases universe is reached by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    /// Task `t` binds `interrupt-source` itself.
    DirectT,
    /// Task `u` binds it itself.
    DirectU,
    /// The system's own level binds it itself.
    DirectSys,
    /// A service `evt.X` binds it, used by `t`.
    ServiceT,
    /// `evt.X` used by a service `h.X` that `t` uses.
    ChainT,
    /// `evt.X` used by `u`.
    ServiceU,
    /// `evt.X` used by both tasks.
    ServiceTU,
    /// `evt.X` used by the system's own level alone, reaching no task.
    ServiceSys,
}

/// The separations a source may state, against task `t`'s 20 ms and `u`'s 40 ms: shorter, equal and longer than each,
/// unstated, and declared absent.
const SEPARATIONS: [Separation; 6] = [
    Separation::Stated(10),
    Separation::Stated(20),
    Separation::Stated(40),
    Separation::Stated(80),
    Separation::Unstated,
    Separation::Absent,
];

/// **Releases and routes** (§2.2, §2.5): task `t`, periodic or sporadic at 20 ms; task `u` absent, periodic or
/// sporadic at 40 ms; one or two sources, each of every separation class, reaching a task directly, through a service
/// or through two, or reaching none — a task bound to one itself released by it alone, whatever other source it
/// reaches (round 16, R16-B1); an alarm `t` uses, or none. The first source, `dev.z`, is declared first and the
/// second, `dev.a`, is met only by a service requiring `(available-in-state run sleep)`, which it alone offers, so the
/// two are distinct and their texts' order is not their declaration order. A task binding `interrupt-source` itself
/// may take either, each offering it: since model §8 rule 16 landed (`M4.1.2.3`), a sporadic one takes `dev.a` where
/// `dev.z` is not as far apart as it and `dev.a` is, and the search refuses the system where neither is; every record
/// binding it to one that is not is judged instead, the unamended rules', binding `dev.z`, among them
/// ([`RULE_16_RECORD`]). (Names written by `needs` are the
/// adapters universe's; here every declaration is named by `uses`.)
fn releases_and_routes() -> Vec<Case> {
    let mut cases = Vec::new();
    for alarm in [false, true] {
        for t_sporadic in [false, true] {
            for u_kind in [None, Some(false), Some(true)] {
                let mut first_paths = vec![Path::DirectT, Path::DirectSys];
                let mut second_paths = Vec::new();
                for path in [Path::ServiceT, Path::ChainT, Path::ServiceSys] {
                    first_paths.push(path);
                    second_paths.push(path);
                }
                if u_kind.is_some() {
                    first_paths.push(Path::DirectU);
                    for path in [Path::ServiceU, Path::ServiceTU] {
                        first_paths.push(path);
                        second_paths.push(path);
                    }
                }
                let mut sources: Vec<Vec<(Separation, Path)>> = Vec::new();
                for &s1 in &SEPARATIONS {
                    for &p1 in &first_paths {
                        sources.push(vec![(s1, p1)]);
                        for &s2 in &SEPARATIONS {
                            for &p2 in &second_paths {
                                sources.push(vec![(s1, p1), (s2, p2)]);
                            }
                        }
                    }
                }
                for chosen in sources {
                    cases.push(release_case(alarm, t_sporadic, u_kind, &chosen));
                }
            }
        }
    }
    cases
}

fn release_case(
    alarm: bool,
    t_sporadic: bool,
    u_kind: Option<bool>,
    sources: &[(Separation, Path)],
) -> Case {
    const BLOCKS: [&str; 2] = ["dev.z", "dev.a"];
    const STATES: [&str; 2] = ["run", "run sleep"];
    let mut t = Side::default();
    let mut u = Side::default();
    let mut own = Side::default();
    let mut blocks = Vec::new();
    let mut services = Vec::new();
    // A declaration named by `uses`.
    let name_in = |side: &mut Side, declaration: &str| side.uses.push(declaration.to_string());
    let mut label = format!(
        "t={} u={} alarm={} sources=",
        if t_sporadic { "sporadic" } else { "periodic" },
        match u_kind {
            None => "-",
            Some(false) => "periodic",
            Some(true) => "sporadic",
        },
        if alarm { "yes" } else { "no" }
    );
    for (i, (separation, path)) in sources.iter().enumerate() {
        let block = BLOCKS[i];
        let suffix = &block[4..];
        blocks.push(source_block(block, *separation, STATES[i]));
        label.push_str(&format!("{block}:{separation:?}:{path:?} "));
        let evt = format!("evt.{suffix}");
        let mut handler = service(&evt, Counter::None, true, Vec::new());
        if i == 1 {
            handler
                .side
                .other
                .push("(available-in-state run sleep)".to_string());
        }
        match path {
            Path::DirectT => t.needs.push("interrupt-source".to_string()),
            Path::DirectU => u.needs.push("interrupt-source".to_string()),
            Path::DirectSys => own.needs.push("interrupt-source".to_string()),
            Path::ServiceT => name_in(&mut t, &evt),
            Path::ChainT => {
                let chain = format!("h.{suffix}");
                name_in(&mut t, &chain);
                let mut through = service(&chain, Counter::None, false, Vec::new());
                name_in(&mut through.side, &evt);
                services.push(through);
            }
            Path::ServiceU => name_in(&mut u, &evt),
            Path::ServiceTU => {
                name_in(&mut t, &evt);
                name_in(&mut u, &evt);
            }
            Path::ServiceSys => name_in(&mut own, &evt),
        }
        if !matches!(path, Path::DirectT | Path::DirectU | Path::DirectSys) {
            services.push(handler);
        }
    }
    if alarm {
        blocks.push(timer_block("timer.d", 1, false));
        services.push(service("alarm", Counter::Timer, false, Vec::new()));
        name_in(&mut t, "alarm");
    }
    let mut tasks = vec![Task {
        name: "t".to_string(),
        release: if t_sporadic {
            Release::MinSeparation(Dur::Ms(20))
        } else {
            Release::Period(Dur::Ms(20))
        },
        priority: 1,
        deadline_from: Some("release"),
        side: t,
        ..Task::default()
    }];
    if let Some(sporadic) = u_kind {
        tasks.push(Task {
            name: "u".to_string(),
            release: if sporadic {
                Release::MinSeparation(Dur::Ms(40))
            } else {
                Release::Period(Dur::Ms(40))
            },
            priority: 2,
            deadline_from: Some("release"),
            side: u,
            ..Task::default()
        });
    }
    Case {
        label: label.trim_end().to_string(),
        description: Description {
            system: "sys".to_string(),
            blocks,
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// Who requires in the requesters universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Requester {
    /// The periodic task `t` itself.
    PeriodicTask,
    /// The sporadic task `t` itself.
    SporadicTask,
    /// The system's own level, beside a periodic task `t` naming nothing.
    OwnLevel,
    /// A service `s` the periodic task `t` names.
    Service,
}

/// **Requesters of every kind** (§1, §2.2, §2.6): a periodic task, a sporadic task, the system's own level or a
/// service, requiring a timer's channel, an absolute deadline, an extended counter or a source; with one timer that
/// both adapter kinds can program, and one source stating 20 ms.
fn requesters_of_every_kind() -> Vec<Case> {
    let mut cases = Vec::new();
    for requester in [
        Requester::PeriodicTask,
        Requester::SporadicTask,
        Requester::OwnLevel,
        Requester::Service,
    ] {
        for (counter, source) in [
            (Counter::Timer, false),
            (Counter::Deadline, false),
            (Counter::Extender, false),
            (Counter::None, true),
        ] {
            // What the requester writes: a service's requirements, the same for each kind of requester.
            let wanted = service("s", counter, source, Vec::new());
            let mut t = Side::default();
            let mut own = Side::default();
            let mut services = Vec::new();
            match requester {
                Requester::PeriodicTask | Requester::SporadicTask => {
                    // A task writes no constraint clause, the language's `task` having none: its extended counter
                    // is still met by the extender alone, no block offering `counter-width`.
                    t = wanted.side;
                    t.other.clear();
                }
                Requester::OwnLevel => own = wanted.side,
                Requester::Service => {
                    t.uses.push("s".to_string());
                    services.push(wanted);
                }
            }
            cases.push(Case {
                label: format!(
                    "requester={requester:?} requires={}{}",
                    counter.word(),
                    if source { "+source" } else { "" }
                ),
                description: Description {
                    system: "sys".to_string(),
                    blocks: vec![
                        timer_block("timer.d", 1, true),
                        source_block("dev.s", Separation::Stated(20), ""),
                    ],
                    services,
                    tasks: vec![Task {
                        name: "t".to_string(),
                        release: if requester == Requester::SporadicTask {
                            Release::MinSeparation(Dur::Ms(20))
                        } else {
                            Release::Period(Dur::Ms(20))
                        },
                        priority: 1,
                        deadline_from: Some("release"),
                        side: t,
                        ..Task::default()
                    }],
                    own,
                    ..Description::default()
                },
            });
        }
    }
    cases
}

/// **Two tasks sharing services that take a channel** (§2.2, §2.4, §2.6, §2.7, §2.9, §2.10): a periodic task `t` and
/// a sporadic task `u`, each writing `(deadline-from release)` or not; two services `a` and `b`, each taking a channel
/// of the one timer, an absolute deadline from a `deadline-over-delay` instance with its obliged `delivery-bound` bare
/// or bounded, or nothing, and handling the one source or not; every set of `uses` edges between them; each task using
/// every subset of them; the system's own level using each service no task reaches. Two jobs calling one channel, an
/// expiry running a node both jobs reach, a cycle making one initialisation step of two members.
fn two_tasks_sharing_services() -> Vec<Case> {
    const SERVICES: [&str; 2] = ["a", "b"];
    const COUNTERS: [Counter; 4] = [
        Counter::None,
        Counter::Timer,
        Counter::Deadline,
        Counter::DeadlineBound,
    ];
    let edges = pairs(2);
    let subset = |set: usize| -> Vec<String> {
        SERVICES
            .iter()
            .enumerate()
            .filter(|(i, _)| set >> i & 1 == 1)
            .map(|(_, s)| (*s).to_string())
            .collect()
    };
    let mut cases = Vec::new();
    for capabilities in 0..8usize.pow(2) {
        for edge_set in 0..1usize << edges.len() {
            for t_set in 0..1usize << SERVICES.len() {
                for u_set in 0..1usize << SERVICES.len() {
                    for from_set in 0..4usize {
                        let mut services = Vec::new();
                        let mut label = String::from("caps=");
                        for (i, name) in SERVICES.iter().enumerate() {
                            let cap =
                                capabilities / 8usize.pow(u32::try_from(i).expect("small")) % 8;
                            let counter = COUNTERS[cap % 4];
                            let source = cap / 4 == 1;
                            let uses = edges
                                .iter()
                                .enumerate()
                                .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                                .map(|(_, (_, to))| SERVICES[*to].to_string())
                                .collect();
                            label.push_str(&format!(
                                "{name}:{}{} ",
                                counter.word(),
                                if source { "+source" } else { "" }
                            ));
                            services.push(service(name, counter, source, uses));
                        }
                        let (t_from, u_from) = (from_set & 1 == 1, from_set & 2 == 2);
                        let tasks = vec![
                            Task {
                                name: "t".to_string(),
                                release: Release::Period(Dur::Ms(10)),
                                priority: 1,
                                deadline_from: t_from.then_some("release"),
                                side: named(Side::default(), subset(t_set), false),
                                ..Task::default()
                            },
                            Task {
                                name: "u".to_string(),
                                release: Release::MinSeparation(Dur::Ms(20)),
                                priority: 2,
                                deadline_from: u_from.then_some("release"),
                                side: named(Side::default(), subset(u_set), false),
                                ..Task::default()
                            },
                        ];
                        let edge_words: Vec<String> = edges
                            .iter()
                            .enumerate()
                            .filter(|(k, _)| edge_set >> k & 1 == 1)
                            .map(|(_, (from, to))| format!("{}>{}", SERVICES[*from], SERVICES[*to]))
                            .collect();
                        label.push_str(&format!(
                            "uses=[{}] t=[{}]{} u=[{}]{}",
                            edge_words.join(","),
                            subset(t_set).join(","),
                            if t_from { "+from" } else { "" },
                            subset(u_set).join(","),
                            if u_from { "+from" } else { "" },
                        ));
                        let own = own_level(&services, &tasks, false);
                        cases.push(Case {
                            label,
                            description: Description {
                                system: "sys".to_string(),
                                blocks: vec![
                                    timer_block("timer.d", 3, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                services,
                                tasks,
                                own,
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What one requester of the receivers universe programs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Programs {
    Nothing,
    /// An alarm: `(needs relative-delay)`, bound to the timer itself.
    Alarm,
    /// An absolute deadline: `(needs absolute-deadline)`, met by its own `deadline-over-delay` instance, whose
    /// `expiry` runs its requester's code — or by a block offering it, an alarm then.
    Deadline,
    /// Both: `(needs relative-delay absolute-deadline)`.
    Both,
}

impl Programs {
    /// Both an alarm and a deadline for one requester is no choice here: the two facts are of one function, `counter`,
    /// which the search binds to one provider whole, and neither the timer nor the instance offers both, so it
    /// concludes `missing-fact` on every such system — which the one-receiver universe holds.
    const ALL: [Self; 3] = [Self::Nothing, Self::Alarm, Self::Deadline];
    /// Every choice, both among them.
    const WITH_BOTH: [Self; 4] = [Self::Nothing, Self::Alarm, Self::Deadline, Self::Both];

    fn needs(self) -> Vec<String> {
        match self {
            Self::Nothing => Vec::new(),
            Self::Alarm => names(&["relative-delay"]),
            Self::Deadline => names(&["absolute-deadline"]),
            Self::Both => names(&["relative-delay", "absolute-deadline"]),
        }
    }
}

/// The counters of the one-receiver universe: `d` and `d.x`, each with four channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Counters {
    /// Two timers, each offering `relative-delay` and what a `deadline-over-delay` instance needs beneath.
    TwoTimers,
    /// The timer `d` offering `absolute-deadline` itself too, and the timer `d.x`: one requester may bind both facts
    /// to `d`, one alarm.
    OneOffersBoth,
    /// The timer `d`, and `d.x` offering `absolute-deadline` alone.
    DeadlineApart,
}

/// **One receiver's alarms and expiries** (§1; round 14, R14-R6: "Its alarms, then its expiries, each by its text"):
/// tasks `t` and `t!`, either first in runtime order, and the system's own level, each programming nothing, an alarm,
/// an absolute deadline or both; over two counters `d` and `d.x`, one name a prefix of the other, of each shape of
/// [`Counters`]. One receiver can receive two things only by two providers, an alarm at one and an alarm or an expiry
/// at another; the search binds a requester's `counter` facts to one provider whole, so it refuses every such system
/// (`missing-fact`), and the systems it refuses are counted. Where one provider offers both facts, a requester binding
/// both there programs one alarm, named once. So §1's order within one receiver is reachable by no system the search
/// resolves, and its order across receivers, each named with what it receives, is held over providers and instances
/// one of whose texts prefixes another's.
fn receivers_within_one() -> Vec<Case> {
    let mut cases = Vec::new();
    for counters in [
        Counters::TwoTimers,
        Counters::OneOffersBoth,
        Counters::DeadlineApart,
    ] {
        for t_programs in Programs::WITH_BOTH {
            for bang_programs in Programs::WITH_BOTH {
                for own_programs in Programs::WITH_BOTH {
                    for bang_first in [false, true] {
                        let task = |name: &str, priority: u32, programs: Programs| Task {
                            name: name.to_string(),
                            priority,
                            side: Side {
                                needs: programs.needs(),
                                ..Side::default()
                            },
                            ..Task::default()
                        };
                        let (t_priority, bang_priority) = if bang_first { (2, 1) } else { (1, 2) };
                        let mut d = timer_block("d", 4, true);
                        let d_x = match counters {
                            Counters::TwoTimers | Counters::OneOffersBoth => {
                                timer_block("d.x", 4, true)
                            }
                            Counters::DeadlineApart => Block {
                                name: "d.x".to_string(),
                                offers: names(&[
                                    "(absolute-deadline true)",
                                    "(channels 4 tick)",
                                    "(reachable-at-privilege machine)",
                                ]),
                                ..Block::default()
                            },
                        };
                        if counters == Counters::OneOffersBoth {
                            d.offers.insert(1, "(absolute-deadline true)".to_string());
                        }
                        cases.push(Case {
                            label: format!(
                                "{counters:?} t:{t_programs:?} t!:{bang_programs:?} own:{own_programs:?} first={}",
                                if bang_first { "t!" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![d, d_x],
                                tasks: vec![
                                    task("t", t_priority, t_programs),
                                    task("t!", bang_priority, bang_programs),
                                ],
                                own: Side {
                                    needs: own_programs.needs(),
                                    ..Side::default()
                                },
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// **Receivers of interrupt code** (§1): tasks `t` and `u`, each periodic or sporadic, each writing `(deadline-from
/// release)` or not, `t` first in runtime order or `u`; each of them and the system's own level programming nothing, an
/// alarm itself, or an absolute deadline whose expiry it receives; over one timer with channels for every requester.
/// So §1's refusal naming every receiver, each task in runtime order and then the system's own level, meets every set
/// of receivers in both runtime orders.
fn receivers_of_interrupt_code() -> Vec<Case> {
    let mut cases = Vec::new();
    for t_programs in Programs::ALL {
        for u_programs in Programs::ALL {
            for own_programs in Programs::ALL {
                for u_first in [false, true] {
                    for kinds in 0..4usize {
                        for from_set in 0..4usize {
                            let task = |name: &str,
                                        sporadic: bool,
                                        priority: u32,
                                        from: bool,
                                        programs: Programs| {
                                Task {
                                    name: name.to_string(),
                                    release: if sporadic {
                                        Release::MinSeparation(Dur::Ms(20))
                                    } else {
                                        Release::Period(Dur::Ms(20))
                                    },
                                    priority,
                                    deadline_from: from.then_some("release"),
                                    side: Side {
                                        needs: programs.needs(),
                                        ..Side::default()
                                    },
                                    ..Task::default()
                                }
                            };
                            let (t_sporadic, u_sporadic) = (kinds & 1 == 1, kinds & 2 == 2);
                            let (t_from, u_from) = (from_set & 1 == 1, from_set & 2 == 2);
                            let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
                            cases.push(Case {
                                label: format!(
                                    "t:{}{}:{t_programs:?} u:{}{}:{u_programs:?} own:{own_programs:?} first={}",
                                    if t_sporadic { "sporadic" } else { "periodic" },
                                    if t_from { "+from" } else { "" },
                                    if u_sporadic { "sporadic" } else { "periodic" },
                                    if u_from { "+from" } else { "" },
                                    if u_first { "u" } else { "t" },
                                ),
                                description: Description {
                                    system: "sys".to_string(),
                                    blocks: vec![timer_block("timer.d", 6, true)],
                                    services: Vec::new(),
                                    tasks: vec![
                                        task("t", t_sporadic, t_priority, t_from, t_programs),
                                        task("u", u_sporadic, u_priority, u_from, u_programs),
                                    ],
                                    own: Side {
                                        needs: own_programs.needs(),
                                        ..Side::default()
                                    },
                                    ..Description::default()
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    cases
}

/// What a block or a platform of the hardware universe requires at its own level (model §8 rule 1: "any other
/// declaration, for what it writes at its own level").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Requires {
    Nothing,
    /// A timer's channel: `(needs relative-delay)`, the alarm it programs itself.
    Alarm,
    /// An absolute deadline: `(needs absolute-deadline)`, met by its own `deadline-over-delay` instance, whose expiry
    /// runs its code.
    Deadline,
    /// An interrupt source: `(needs interrupt-source)`, its handler run in the source's interrupt.
    Source,
    /// Another block's output: `(needs observable-output)`.
    Output,
}

impl Requires {
    const ALL: [Self; 5] = [
        Self::Nothing,
        Self::Alarm,
        Self::Deadline,
        Self::Source,
        Self::Output,
    ];

    fn needs(self) -> Vec<String> {
        match self {
            Self::Nothing => Vec::new(),
            Self::Alarm => names(&["relative-delay"]),
            Self::Deadline => names(&["absolute-deadline"]),
            Self::Source => names(&["interrupt-source"]),
            Self::Output => names(&["observable-output"]),
        }
    }
}

/// Who binds to what a block or a platform of the hardware universe offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binder {
    Nothing,
    /// A service the system's own level uses.
    OwnService,
    /// A service task `t` uses.
    TService,
    /// A service task `u` uses.
    UService,
    /// A service both tasks use.
    BothService,
    /// Task `t` itself.
    TDirect,
    /// The system's own level itself.
    OwnDirect,
    /// A service task `u` uses that also handles the source, so that the source's interrupt reaches the block or the
    /// platform through the binding, beside `u`'s job.
    UHandler,
}

impl Binder {
    const ALL: [Self; 8] = [
        Self::Nothing,
        Self::OwnService,
        Self::TService,
        Self::UService,
        Self::BothService,
        Self::TDirect,
        Self::OwnDirect,
        Self::UHandler,
    ];
}

/// What names the service `alarm`, which task `t` uses, by a `uses` of its own: nothing, `dma.x` or `soc` — hardware
/// naming what is not hardware, refused before any plan since round 13 (§1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Composer {
    Nothing,
    Block,
    Platform,
}

/// **Hardware with requirements of its own** (§2, §2.6, §2.7, §2.9, §2.10; round 12, R12-R4): a block `dma.x`
/// offering `transfer-engine` and the platform `soc` offering `core-count`, each requiring nothing, a timer's channel,
/// an absolute deadline, the source or another block's output; each bound to by nothing, by a service the system's own
/// level, task `t`, task `u` or both use, by `t` or the system's own level itself, or by a service `u` uses that
/// handles the source; the service `alarm`, which `t`
/// uses, named by nothing, by `dma.x`'s `uses` or by `soc`'s; and task `u` periodic or sporadic. So hardware is a node
/// bound to by something or by nothing, runs in its own alarm's, its expiry's and its source's interrupts, is
/// multiplexed by two jobs, releases a sporadic task it handles a source for, and a task reaches the platform through a
/// binding — while what a block or a platform composes is never reached through it, composition giving no edge, and
/// the platform's binding to a block it composes keeps its edge (§2.7; round 16, R16-B2). Since round 13 (R13-D3), a
/// block's or a platform's `uses` naming `alarm` is refused before any plan (§1), naming it.
fn requiring_hardware() -> Vec<Case> {
    let mut cases = Vec::new();
    for dma in Requires::ALL {
        for soc in Requires::ALL {
            for dma_binder in Binder::ALL {
                for soc_binder in Binder::ALL {
                    for composer in [Composer::Nothing, Composer::Block, Composer::Platform] {
                        for u_sporadic in [false, true] {
                            cases.push(hardware_case(
                                (dma, dma_binder),
                                (soc, soc_binder),
                                composer,
                                u_sporadic,
                            ));
                        }
                    }
                }
            }
        }
    }
    cases
}

fn hardware_case(
    (dma, dma_binder): (Requires, Binder),
    (soc, soc_binder): (Requires, Binder),
    composer: Composer,
    u_sporadic: bool,
) -> Case {
    let mut t = Side {
        uses: names(&["alarm"]),
        ..Side::default()
    };
    let mut u = Side::default();
    let mut own = Side::default();
    let mut services = vec![service("alarm", Counter::Timer, false, Vec::new())];
    for (binder, via, fact) in [
        (dma_binder, "xfer", "transfer-engine"),
        (soc_binder, "cpu", "core-count"),
    ] {
        let mut through = Service {
            name: via.to_string(),
            policy: false,
            side: Side {
                needs: names(&[fact]),
                ..Side::default()
            },
        };
        match binder {
            Binder::Nothing => {}
            Binder::OwnService => own.uses.push(via.to_string()),
            Binder::UHandler => {
                through.side.needs.push(INTERRUPT_FACT.to_string());
                u.uses.push(via.to_string());
            }
            Binder::TService => t.uses.push(via.to_string()),
            Binder::UService => u.uses.push(via.to_string()),
            Binder::BothService => {
                t.uses.push(via.to_string());
                u.uses.push(via.to_string());
            }
            Binder::TDirect => t.needs.push(fact.to_string()),
            Binder::OwnDirect => own.needs.push(fact.to_string()),
        }
        if matches!(
            binder,
            Binder::OwnService
                | Binder::TService
                | Binder::UService
                | Binder::BothService
                | Binder::UHandler
        ) {
            services.push(through);
        }
    }
    let mut dma_block = Block {
        name: "dma.x".to_string(),
        offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
        side: Side {
            needs: dma.needs(),
            ..Side::default()
        },
        ..Block::default()
    };
    if composer == Composer::Block {
        dma_block.side.uses.push("alarm".to_string());
    }
    let blocks = vec![
        timer_block("timer.d", 4, true),
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        dma_block,
    ];
    let mut composed: Vec<String> = blocks.iter().map(|b| b.name.clone()).collect();
    if composer == Composer::Platform {
        composed.push("alarm".to_string());
    }
    let platform = Block {
        name: "soc".to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            needs: soc.needs(),
            uses: composed,
            ..Side::default()
        },
        ..Block::default()
    };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            release: Release::Period(Dur::Ms(10)),
            priority: 1,
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: if u_sporadic {
                Release::MinSeparation(Dur::Ms(20))
            } else {
                Release::Period(Dur::Ms(20))
            },
            priority: 2,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "dma.x:{dma:?} by {dma_binder:?}; soc:{soc:?} by {soc_binder:?}; alarm composed by {composer:?}; u {}",
            if u_sporadic { "sporadic" } else { "periodic" }
        ),
        description: Description {
            blocks,
            platforms: vec![platform],
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// The jitters a task of the task-clauses universe writes: none, zero, 1500 us, one longer than its period or
/// separation, and one past the arithmetic.
const JITTERS: [Option<Dur>; 5] = [
    None,
    Some(Dur::Ms(0)),
    Some(Dur::Spelled("1500 us")),
    Some(Dur::Ms(50)),
    Some(Dur::Spelled(PAST)),
];

/// The overrun policies a task writes: none, `fault` and `skip-late-job`, the two the profile performs.
const OVERRUNS: [Option<&str>; 3] = [None, Some("fault"), Some("skip-late-job")];

/// The deadline's reference: `release`, none written, and another event.
const REFERENCES: [Option<&str>; 3] = [Some("release"), None, Some("activation")];

/// **What a task line states** (§2.2, §1; round 12, R12-R4): a periodic task `t` and a sporadic task `u`, either
/// first in runtime order, each writing every jitter of [`JITTERS`], every overrun policy of [`OVERRUNS`] and every
/// deadline reference of [`REFERENCES`]. So "its release jitter, written as zero where none is stated" and "its overrun
/// policy, `fault` where none is stated" are read, and a deadline from another event is refused task by task in
/// runtime order, after a jitter past the arithmetic, the second list's first bullet.
fn task_clauses() -> Vec<Case> {
    let mut cases = Vec::new();
    for u_first in [false, true] {
        for t_jitter in JITTERS {
            for t_overrun in OVERRUNS {
                for t_from in REFERENCES {
                    for u_jitter in JITTERS {
                        for u_overrun in OVERRUNS {
                            for u_from in REFERENCES {
                                let (t_priority, u_priority) =
                                    if u_first { (2, 1) } else { (1, 2) };
                                let task =
                                    |name: &str,
                                     release: Release,
                                     priority: u32,
                                     jitter: Option<Dur>,
                                     overrun: Option<&'static str>,
                                     from: Option<&'static str>,
                                     uses: &str| Task {
                                        name: name.to_string(),
                                        release,
                                        jitter,
                                        overrun,
                                        priority,
                                        deadline_from: from,
                                        side: Side {
                                            uses: names(&[uses]),
                                            ..Side::default()
                                        },
                                        ..Task::default()
                                    };
                                cases.push(Case {
                                    label: format!(
                                        "t:{t_jitter:?},{t_overrun:?},{t_from:?} u:{u_jitter:?},{u_overrun:?},{u_from:?} \
                                         first={}",
                                        if u_first { "u" } else { "t" }
                                    ),
                                    description: Description {
                                        blocks: vec![
                                            Block {
                                                name: "uart.c".to_string(),
                                                offers: names(&[
                                                    "(observable-output true)",
                                                    "(reachable-at-privilege machine)",
                                                ]),
                                                ..Block::default()
                                            },
                                            source_block("dev.s", Separation::Stated(40), ""),
                                        ],
                                        services: vec![
                                            Service {
                                                name: "log".to_string(),
                                                policy: false,
                                                side: Side {
                                                    needs: names(&["observable-output"]),
                                                    ..Side::default()
                                                },
                                            },
                                            service("evt", Counter::None, true, Vec::new()),
                                        ],
                                        tasks: vec![
                                            task(
                                                "t",
                                                Release::Period(Dur::Ms(20)),
                                                t_priority,
                                                t_jitter,
                                                t_overrun,
                                                t_from,
                                                "log",
                                            ),
                                            task(
                                                "u",
                                                Release::MinSeparation(Dur::Ms(40)),
                                                u_priority,
                                                u_jitter,
                                                u_overrun,
                                                u_from,
                                                "evt",
                                            ),
                                        ],
                                        ..Description::default()
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}

/// What one source of the arithmetic universe states of its separation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceState {
    /// A separation within the arithmetic.
    Within,
    /// One spelling past it.
    Past,
    /// Two spellings, one within and one past, whose comparison overflows it.
    Overflowing,
    /// One spelling past it, the source bound by nothing, so that the plan writes no separation of it.
    PastUnbound,
}

/// What a `deadline-over-delay` requester of the arithmetic universe writes on `delivery-bound`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundState {
    Bare,
    Within,
    Past,
}

/// The separations of the arithmetic universe's sources, `dev` of 20 ms and `dev!` of 40 ms, and at the
/// arithmetic's edge `dev` of [`WIDE`] in two agreeing spellings and `dev!` of [`WIDE_LONGER`]; each also with a
/// spelling past the arithmetic beside it, [`PAST`] or, at the edge, [`TIGHT_PAST`].
const Z_WITHIN: [&[&str]; 2] = [&["20 ms"], &[WIDE, WIDE_IN_MS]];
const Z_OVERFLOWING: [&[&str]; 2] = [&["20 ms", PAST], &[WIDE, TIGHT_PAST]];
const A_WITHIN: [&[&str]; 2] = [&["40 ms"], &[WIDE_LONGER]];
const A_OVERFLOWING: [&[&str]; 2] = [&["40 ms", PAST], &[WIDE_LONGER, TIGHT_PAST]];
const ONLY_PAST: [&[&str]; 2] = [&[PAST], &[TIGHT_PAST]];

/// **Durations past the arithmetic, and at its edge** (§1, §2.2, §2.4, §2.5; round 12, R12-R4): a periodic task `t`
/// using a source `dev` and two `deadline-over-delay` requesters `dl` and `dl!`, and a sporadic task `u` using a
/// source `dev!`, either first in runtime order; each of `t`'s period, deadline and jitter past the arithmetic or not,
/// and `u`'s three past or none; each source within, past in one spelling, overflowing in two, or past and bound by
/// nothing; each requester's bound bare, within or past; and every value within the arithmetic a few milliseconds, or
/// at its edge — denominators of `10^37`, `10^38` and `8.192e37`, past it by the least a spelling reaches, spellings
/// only their reduction brings within, and comparisons whose cross-products pass 128 bits. The sources' texts,
/// `(source dev!)` before `(source dev)`, and the instances', `(adapter deadline-over-delay timer.d dl!)` before
/// `(adapter deadline-over-delay timer.d dl)`, are ordered unlike their names and their declarations, `!` lying below
/// `)` (round 14, R14-D1). So §1 names each duration past the arithmetic in its order, each kind by its texts, and a
/// duration within it is written exactly however it is spelled.
fn past_the_arithmetic() -> Vec<Case> {
    const SOURCES: [SourceState; 4] = [
        SourceState::Within,
        SourceState::Past,
        SourceState::Overflowing,
        SourceState::PastUnbound,
    ];
    const BOUNDS: [BoundState; 3] = [BoundState::Bare, BoundState::Within, BoundState::Past];
    let mut cases = Vec::new();
    for edge in [false, true] {
        for u_first in [false, true] {
            for t_past in 0..8usize {
                for u_past in [false, true] {
                    for z in SOURCES {
                        for a in SOURCES {
                            for dl_a in BOUNDS {
                                for dl_b in BOUNDS {
                                    cases.push(arithmetic_case(
                                        edge,
                                        u_first,
                                        (t_past, u_past),
                                        (z, a),
                                        (dl_a, dl_b),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}

fn arithmetic_case(
    edge: bool,
    u_first: bool,
    (t_past, u_past): (usize, bool),
    (z, a): (SourceState, SourceState),
    (dl_a, dl_b): (BoundState, BoundState),
) -> Case {
    let at = usize::from(edge);
    let within = |plain: Dur, wide: &'static str| if edge { Dur::Spelled(wide) } else { plain };
    let past = Dur::Spelled(if edge { TIGHT_PAST } else { PAST });
    let mut blocks = vec![timer_block("timer.d", 2, true)];
    let mut services = Vec::new();
    let mut t = Side {
        uses: names(&["dl", "dl!"]),
        ..Side::default()
    };
    let mut u = Side::default();
    for (block, state, within_spellings, overflowing, states, handler, side) in [
        ("dev", z, Z_WITHIN[at], Z_OVERFLOWING[at], "", "e", &mut t),
        (
            "dev!",
            a,
            A_WITHIN[at],
            A_OVERFLOWING[at],
            "run sleep",
            "e!",
            &mut u,
        ),
    ] {
        let spellings = match state {
            SourceState::Within => within_spellings,
            SourceState::Past | SourceState::PastUnbound => ONLY_PAST[at],
            SourceState::Overflowing => overflowing,
        };
        blocks.push(source_block(block, Separation::Spelled(spellings), states));
        if state != SourceState::PastUnbound {
            let mut evt = service(handler, Counter::None, true, Vec::new());
            if !states.is_empty() {
                evt.side
                    .other
                    .push(format!("(available-in-state {states})"));
            }
            services.push(evt);
            side.uses.push(handler.to_string());
        }
    }
    for (name, state, wide) in [("dl", dl_a, WIDE), ("dl!", dl_b, TIGHT_WITHIN)] {
        let mut deadline = service(name, Counter::Deadline, false, Vec::new());
        match state {
            BoundState::Bare => {}
            BoundState::Within => deadline.side.at_most.push((
                "delivery-bound".to_string(),
                within(Dur::Ms(DELIVERY_BOUND_MS), wide),
            )),
            BoundState::Past => deadline
                .side
                .at_most
                .push(("delivery-bound".to_string(), past)),
        }
        services.push(deadline);
    }
    let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
    let pick = |is_past: bool, within: Dur| if is_past { past } else { within };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            release: Release::Period(pick(t_past & 1 == 1, within(Dur::Ms(20), WIDE_LONGER))),
            deadline: Some(pick(t_past & 2 == 2, within(Dur::Ms(20), WIDE))),
            jitter: if t_past & 4 == 4 {
                Some(past)
            } else if edge {
                Some(Dur::Spelled(REDUCED))
            } else {
                None
            },
            priority: t_priority,
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: Release::MinSeparation(pick(u_past, within(Dur::Ms(40), WIDE))),
            deadline: Some(pick(u_past, within(Dur::Ms(40), WIDE_IN_MS))),
            jitter: u_past.then_some(past),
            priority: u_priority,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "{} first={} t-past={}{}{} u-past={u_past} dev:{z:?} dev!:{a:?} dl:{dl_a:?} dl!:{dl_b:?}",
            if edge { "edge" } else { "plain" },
            if u_first { "u" } else { "t" },
            if t_past & 1 == 1 { "period," } else { "" },
            if t_past & 2 == 2 { "deadline," } else { "" },
            if t_past & 4 == 4 { "jitter" } else { "" },
        ),
        description: Description {
            blocks,
            services,
            tasks,
            ..Description::default()
        },
    }
}

/// **A second system** (§1, §2; round 12, R12-R4 and R12-R1): systems `sys` and `other` in one description, each
/// lowered. A system is named by `uses`, or every one by `needs`, at each set of five sites — `sys`'s task, `sys`'s own
/// level and a service `sys`'s task uses naming `other`, and `other`'s task and a service only `other`'s task uses
/// naming `sys`; the service `alarm`, taking a channel, used by `sys`'s task alone or by both systems' tasks; `other`
/// on `sys`'s platform or on its own, which composes a block requiring a timer's channel; each system's own level using
/// nothing or a service. So §1's naming-system refusal reads the requesters the lowered system serves and no other —
/// a system reached served for its own level alone — the generator's closure is held to the configuration checker on
/// each, and a service two systems use, and hardware another system serves, are read system by system.
fn two_systems() -> Vec<Case> {
    let mut cases = Vec::new();
    for sites in 0..32usize {
        for by_needs in [false, true] {
            for shared_alarm in [false, true] {
                for own_platform in [false, true] {
                    for sys_own in 0..2usize {
                        for other_own in 0..3usize {
                            let [first, second] = two_systems_case(
                                sites,
                                by_needs,
                                shared_alarm,
                                own_platform,
                                (sys_own, other_own),
                            );
                            cases.push(first);
                            cases.push(second);
                        }
                    }
                }
            }
        }
    }
    cases
}

fn two_systems_case(
    sites: usize,
    by_needs: bool,
    shared_alarm: bool,
    own_platform: bool,
    (sys_own, other_own): (usize, usize),
) -> [Case; 2] {
    let site = |bit: usize| sites >> bit & 1 == 1;
    let name_in = |side: &mut Side, name: &str| {
        if by_needs {
            side.needs.push(name.to_string());
        } else {
            side.uses.push(name.to_string());
        }
    };
    let simple = |name: &str, needs: &str| Service {
        name: name.to_string(),
        policy: false,
        side: Side {
            needs: names(&[needs]),
            ..Side::default()
        },
    };
    let mut services = vec![
        simple("log", "observable-output"),
        simple("alarm", "relative-delay"),
    ];
    let mut t = Side {
        uses: names(&["log", "alarm"]),
        ..Side::default()
    };
    let mut o = Side {
        uses: names(&["log"]),
        ..Side::default()
    };
    if shared_alarm {
        o.uses.push("alarm".to_string());
    }
    let mut sys_level = Side::default();
    if sys_own == 1 {
        sys_level.uses.push("log".to_string());
    }
    let mut other_level = Side::default();
    match other_own {
        1 => other_level.uses.push("log".to_string()),
        2 => other_level.uses.push("alarm".to_string()),
        _ => {}
    }
    if site(0) {
        name_in(&mut t, "other");
    }
    if site(1) {
        name_in(&mut sys_level, "other");
    }
    if site(2) {
        let mut bridge = Service {
            name: "bridge.s".to_string(),
            policy: false,
            side: Side::default(),
        };
        name_in(&mut bridge.side, "other");
        services.push(bridge);
        t.uses.push("bridge.s".to_string());
    }
    if site(3) {
        name_in(&mut o, "sys");
    }
    if site(4) {
        let mut bridge = Service {
            name: "bridge.o".to_string(),
            policy: false,
            side: Side::default(),
        };
        name_in(&mut bridge.side, "sys");
        services.push(bridge);
        o.uses.push("bridge.o".to_string());
    }
    let uart = Block {
        name: "uart.c".to_string(),
        offers: names(&[
            "(observable-output true)",
            "(reachable-at-privilege machine)",
        ]),
        ..Block::default()
    };
    let mut blocks = vec![timer_block("timer.d", 4, false), uart];
    let platform = |name: &str, composes: &[&str]| Block {
        name: name.to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            uses: names(composes),
            ..Side::default()
        },
        ..Block::default()
    };
    let mut platforms = vec![platform("host.p", &["timer.d", "uart.c"])];
    if own_platform {
        blocks.push(timer_block("timer.q", 2, false));
        blocks.push(Block {
            name: "dma.q".to_string(),
            offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: names(&["relative-delay"]),
                ..Side::default()
            },
            ..Block::default()
        });
        platforms.push(platform("host.q", &["uart.c", "timer.q", "dma.q"]));
    }
    let sys = System {
        name: "sys".to_string(),
        tasks: vec![Task {
            name: "t".to_string(),
            side: t,
            ..Task::default()
        }],
        own: sys_level,
        platform: "host.p".to_string(),
    };
    let other = System {
        name: "other".to_string(),
        tasks: vec![Task {
            name: "o".to_string(),
            release: Release::Period(Dur::Ms(20)),
            side: o,
            ..Task::default()
        }],
        own: other_level,
        platform: if own_platform { "host.q" } else { "host.p" }.to_string(),
    };
    let label = format!(
        "naming={} by {} alarm={} other on {} sys-own={sys_own} other-own={other_own}",
        [
            "t>other",
            "sys>other",
            "bridge.s>other",
            "o>sys",
            "bridge.o>sys",
        ]
        .iter()
        .enumerate()
        .filter(|(bit, _)| site(*bit))
        .map(|(_, s)| *s)
        .collect::<Vec<_>>()
        .join(","),
        if by_needs { "needs" } else { "uses" },
        if shared_alarm { "both" } else { "sys" },
        if own_platform { "host.q" } else { "host.p" },
    );
    let lowering = |lowered: &System, other: &System, others_first: bool| Case {
        label: format!("{label}: lowering {}", lowered.name),
        description: Description {
            system: lowered.name.clone(),
            blocks: blocks.clone(),
            platforms: platforms.clone(),
            services: services.clone(),
            tasks: lowered.tasks.clone(),
            own: lowered.own.clone(),
            platform: lowered.platform.clone(),
            others: vec![other.clone()],
            others_first,
        },
    };
    [lowering(&sys, &other, false), lowering(&other, &sys, true)]
}

/// What a block's or a platform's `requires` names beside what it requires: each name written by `needs` (`true`) or
/// by `uses` (`false`).
type Naming = &'static [(bool, &'static str)];

/// What `timer.d`, the provider `alarm` is bound to, names: nothing, or `alarm` itself, by `uses` or by `needs`.
const TIMER_NAMING: [Naming; 3] = [&[], &[(false, "alarm")], &[(true, "alarm")]];
/// What `dma.x` names: nothing; the policy `pol.p` by each clause; the service `only.s`, which nothing else names, by
/// `needs`; the block `uart.c` it is bound to, by each clause, and the timer by `needs` — hardware, no refusal, its
/// `uses` composition and its `needs` not (model §8 rule 5's "`uses` alone is composition"); the block `blk.n` by
/// `needs`, a node no job reaches, `dma.x` running in `u`'s (round 15, R15-R5); the system `other`; and two operands at
/// once, written against their texts' order.
const DMA_NAMING: [Naming; 10] = [
    &[],
    &[(false, "pol.p")],
    &[(true, "pol.p")],
    &[(true, "only.s")],
    &[(false, "uart.c")],
    &[(true, "uart.c")],
    &[(true, "timer.d")],
    &[(true, "blk.n")],
    &[(false, "other")],
    &[(false, "pol.p"), (false, "only.s")],
];
/// What the platform `soc` names beside the blocks it composes: nothing; `only.s`; `pol.p` by `needs`; `alarm`, which
/// task `t` uses, and `only.s`; or the block `dev.s` by `needs`, hardware again.
const SOC_NAMING: [Naming; 5] = [
    &[],
    &[(false, "only.s")],
    &[(true, "pol.p")],
    &[(false, "alarm"), (false, "only.s")],
    &[(true, "dev.s")],
];
/// What `blk.q`, a block only the system `other`'s platform composes, names: nothing, or `only.s`.
const BLKQ_NAMING: [Naming; 2] = [&[], &[(false, "only.s")]];

/// Which other refusal of §1 the hardware-naming universe's task `t` would also meet: none; the first list's before
/// it — `t` periodic and bound to the source itself, or `t` naming the system `other` — or the second list's after it,
/// a jitter past the arithmetic, a deadline from another event, or an alarm `t` programs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Also {
    Nothing,
    PeriodicSource,
    NamesSystem,
    JitterPast,
    DeadlineFromOther,
    Receiver,
}

impl Also {
    const ALL: [Self; 6] = [
        Self::Nothing,
        Self::PeriodicSource,
        Self::NamesSystem,
        Self::JitterPast,
        Self::DeadlineFromOther,
        Self::Receiver,
    ];
}

/// `side` naming each of `naming` by its clause.
fn naming(mut side: Side, naming: Naming) -> Side {
    for (by_needs, name) in naming {
        if *by_needs {
            side.needs.push((*name).to_string());
        } else {
            side.uses.push((*name).to_string());
        }
    }
    side
}

/// **Hardware naming what is not hardware** (§1, §2, §2.7; round 13, R13-R2's "a service only composed, a block's
/// `needs` naming a declaration"): `timer.d`, which `alarm` is bound to, naming `alarm`; the block `dma.x`, bound for
/// an output of its own and bound to by the service `xfer` task `u` uses, naming a policy, a service only it names, a
/// system, two operands at once, or another block — no refusal; the platform `soc` naming a service only it names,
/// the service a task uses, a policy, or another block; `blk.q`, on the platform only the system `other` uses, naming
/// a service; each by `uses` or `needs`; and task `t` meeting each other refusal of §1 that can co-apply, of the first
/// list before this one or of the second after it. So the refusal is taken in its place, names each block or
/// platform and its operand by the namer's text and then the operand's, takes no hardware the lowered system does not
/// serve (round 15, R15-R2: "This refusal and the one above take only what the system serves"), and a block naming a
/// block is lowered: its `uses` composition and its `needs` each making no edge of their own, and a binding across
/// either keeping its edge (§2.7, rule 12; round 16, R16-B2) — `dma.x`, in `u`'s job, naming by `needs` `blk.n`,
/// which no job reaches (R15-R5).
fn hardware_naming() -> Vec<Case> {
    let mut cases = Vec::new();
    for timer in TIMER_NAMING {
        for dma in DMA_NAMING {
            for soc in SOC_NAMING {
                for blk_q in BLKQ_NAMING {
                    for also in Also::ALL {
                        cases.push(hardware_naming_case((timer, dma, soc, blk_q), also));
                    }
                }
            }
        }
    }
    cases
}

fn hardware_naming_case(
    (timer, dma, soc, blk_q): (Naming, Naming, Naming, Naming),
    also: Also,
) -> Case {
    let words = |naming: Naming| -> String {
        let named: Vec<String> = naming
            .iter()
            .map(|(by_needs, name)| format!("{}:{name}", if *by_needs { "needs" } else { "uses" }))
            .collect();
        format!("[{}]", named.join(","))
    };
    let mut timer_d = timer_block("timer.d", 4, false);
    timer_d.side = naming(Side::default(), timer);
    let dma_x = Block {
        name: "dma.x".to_string(),
        offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
        side: naming(
            Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
            dma,
        ),
        ..Block::default()
    };
    let blocks = vec![
        timer_d,
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        dma_x,
        Block {
            name: "blk.q".to_string(),
            offers: names(&["(debug-port true)", "(reachable-at-privilege machine)"]),
            side: naming(Side::default(), blk_q),
            ..Block::default()
        },
        // A node only as hardware bound for a requirement of its own, which nothing reaches.
        Block {
            name: "blk.n".to_string(),
            offers: names(&["(debug-port true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
            ..Block::default()
        },
    ];
    let platform = |name: &str, composes: &[&str], extra: Naming| Block {
        name: name.to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: naming(
            Side {
                uses: names(composes),
                ..Side::default()
            },
            extra,
        ),
        ..Block::default()
    };
    let platforms = vec![
        platform(
            "soc",
            &["timer.d", "dev.s", "uart.c", "dma.x", "blk.n"],
            soc,
        ),
        platform("host.q", &["blk.q"], &[]),
    ];
    let simple = |name: &str, policy: bool, needs: &[&str]| Service {
        name: name.to_string(),
        policy,
        side: Side {
            needs: names(needs),
            ..Side::default()
        },
    };
    let services = vec![
        simple("alarm", false, &["relative-delay"]),
        simple("xfer", false, &["transfer-engine"]),
        simple("only.s", false, &["relative-delay"]),
        simple("pol.p", true, &[]),
    ];
    let mut t = Task {
        name: "t".to_string(),
        side: Side {
            uses: names(&["alarm"]),
            ..Side::default()
        },
        ..Task::default()
    };
    match also {
        Also::Nothing => {}
        Also::PeriodicSource => t.side.needs.push(INTERRUPT_FACT.to_string()),
        Also::NamesSystem => t.side.uses.push("other".to_string()),
        Also::JitterPast => t.jitter = Some(Dur::Spelled(PAST)),
        Also::DeadlineFromOther => t.deadline_from = Some("activation"),
        Also::Receiver => t.side.needs.push("relative-delay".to_string()),
    }
    let u = Task {
        name: "u".to_string(),
        release: Release::Period(Dur::Ms(20)),
        priority: 2,
        side: Side {
            uses: names(&["xfer"]),
            ..Side::default()
        },
        ..Task::default()
    };
    let other = System {
        name: "other".to_string(),
        tasks: vec![Task {
            name: "o".to_string(),
            release: Release::Period(Dur::Ms(20)),
            ..Task::default()
        }],
        own: Side::default(),
        platform: "host.q".to_string(),
    };
    Case {
        label: format!(
            "timer.d names {} dma.x names {} soc names {} blk.q names {} t also {also:?}",
            words(timer),
            words(dma),
            words(soc),
            words(blk_q)
        ),
        description: Description {
            blocks,
            platforms,
            services,
            tasks: vec![t, u],
            others: vec![other],
            ..Description::default()
        },
    }
}

/// What a block of the prefixed hardware-naming universe names: nothing; the services `m.s` and `m` by `uses`, written
/// against their texts' order; the policy `m!` by `needs`; or the system `y` by `needs`.
const PREFIXED_BLOCK_NAMING: [Naming; 4] = [
    &[],
    &[(false, "m.s"), (false, "m")],
    &[(true, "m!")],
    &[(true, "y")],
];
/// What the platform `soc` names beside the blocks it composes: nothing; the service `m.s` by `needs`; the policy `m!`
/// by `uses`; or `m` by `needs` and `m!` by `uses`.
const PREFIXED_SOC_NAMING: [Naming; 4] = [
    &[],
    &[(true, "m.s")],
    &[(false, "m!")],
    &[(true, "m"), (false, "m!")],
];
/// What the block `q`, composed only by the platform of the system `y`, names: nothing; `y` by `needs`; the lowered
/// system `sys` by `uses`; or `m.s` by `uses` and `sys` by `needs`.
const Q_NAMING: [Naming; 4] = [
    &[],
    &[(true, "y")],
    &[(false, "sys")],
    &[(false, "m.s"), (true, "sys")],
];

/// **Hardware naming what is not hardware, over names one of which prefixes another** (§1; round 14, R14-D1 and
/// R14-R1): the timer `d`, to which the service `m` task `t` uses is bound, the blocks `d.x` and `d!`, and the platform
/// `soc` composing the three, of the lowered system `sys`; and the block `q`, composed only by `host.y`, the platform
/// of the system `y`. Each of `d`, `d.x` and `d!` names one choice of [`PREFIXED_BLOCK_NAMING`], `soc` one of
/// [`PREFIXED_SOC_NAMING`] and `q` one of [`Q_NAMING`]. So the namers' texts, `d`, `d!`, `d.x`, `q`, `soc`, and the
/// operands', `m`, `m!`, `m.s`, `sys`, `y`, are ordered unlike the words of their messages, one a prefix of another;
/// a platform's `needs` names a service and its `uses` a policy; a system is named — the lowered one among them — on
/// hardware the lowered system does not serve, which neither naming refusal takes since round 15 (R15-R2); and
/// served blocks naming a system meet the naming-system refusal first, their texts ordered alike — and with them `q`,
/// once `y` is reached, which serves what it writes at its own level, its `platform` clause among it (model §8 rules 1
/// and 7).
fn prefixed_hardware_naming() -> Vec<Case> {
    const NAMERS: usize = 5;
    let options = PREFIXED_BLOCK_NAMING.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(u32::try_from(NAMERS).expect("small")) {
        let pick =
            |namer: usize| choice / options.pow(u32::try_from(namer).expect("small")) % options;
        let (d, d_x, d_bang) = (
            PREFIXED_BLOCK_NAMING[pick(0)],
            PREFIXED_BLOCK_NAMING[pick(1)],
            PREFIXED_BLOCK_NAMING[pick(2)],
        );
        let (soc, q) = (PREFIXED_SOC_NAMING[pick(3)], Q_NAMING[pick(4)]);
        let words = |naming: Naming| -> String {
            let named: Vec<String> = naming
                .iter()
                .map(|(by_needs, name)| {
                    format!("{}:{name}", if *by_needs { "needs" } else { "uses" })
                })
                .collect();
            format!("[{}]", named.join(","))
        };
        let plain = |name: &str, offer: &str, named: Naming| Block {
            name: name.to_string(),
            offers: names(&[offer, "(reachable-at-privilege machine)"]),
            side: naming(Side::default(), named),
            ..Block::default()
        };
        let mut timer = timer_block("d", 2, false);
        timer.side = naming(Side::default(), d);
        let platform = |name: &str, composes: &[&str], extra: Naming| Block {
            name: name.to_string(),
            offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
            side: naming(
                Side {
                    uses: names(composes),
                    ..Side::default()
                },
                extra,
            ),
            ..Block::default()
        };
        let simple = |name: &str, policy: bool, needs: &[&str]| Service {
            name: name.to_string(),
            policy,
            side: Side {
                needs: names(needs),
                ..Side::default()
            },
        };
        cases.push(Case {
            label: format!(
                "d names {} d.x names {} d! names {} soc names {} q names {}",
                words(d),
                words(d_x),
                words(d_bang),
                words(soc),
                words(q)
            ),
            description: Description {
                blocks: vec![
                    timer,
                    plain("d.x", "(debug-port true)", d_x),
                    plain("d!", "(transfer-engine true)", d_bang),
                    plain("q", "(debug-port true)", q),
                ],
                platforms: vec![
                    platform("soc", &["d", "d.x", "d!"], soc),
                    platform("host.y", &["q"], &[]),
                ],
                services: vec![
                    simple("m", false, &["relative-delay"]),
                    simple("m.s", false, &[]),
                    simple("m!", true, &[]),
                ],
                tasks: vec![Task {
                    name: "t".to_string(),
                    side: Side {
                        uses: names(&["m"]),
                        ..Side::default()
                    },
                    ..Task::default()
                }],
                others: vec![System {
                    name: "y".to_string(),
                    tasks: vec![Task {
                        name: "z".to_string(),
                        release: Release::Period(Dur::Ms(20)),
                        ..Task::default()
                    }],
                    own: Side::default(),
                    platform: "host.y".to_string(),
                }],
                ..Description::default()
            },
        });
    }
    cases
}

/// How the block `blk` of the served-block universe is named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServedBy {
    /// A service `drv` names it by `uses`, and task `t` uses `drv`.
    TDrvUses,
    /// `drv` names it by `needs`, and `t` uses `drv`.
    TDrvNeeds,
    /// `drv`, used by task `u`.
    UDrv,
    /// `drv`, used by both tasks.
    BothDrv,
    /// `drv`, used by the system's own level.
    OwnDrv,
    /// `drv`, used by nothing: `blk` is served by no requester.
    NoDrv,
    /// Task `t` names `blk` itself: hardware (model §8 rule 7).
    TDirect,
    /// `drv`, used by `t`, which names `blk` itself too: hardware a served service names (round 20, R20-D3: "beside
    /// hardware a task names, which rule 7 makes hardware").
    TDrvAndT,
    /// `drv` and `blk`, each used by `u`.
    UDrvAndU,
    /// `drv` and `blk`, each used by the system's own level, `t` naming neither: hardware by the system's own name.
    OwnDrvAndOwn,
}

impl ServedBy {
    const ALL: [Self; 10] = [
        Self::TDrvUses,
        Self::TDrvNeeds,
        Self::UDrv,
        Self::BothDrv,
        Self::OwnDrv,
        Self::NoDrv,
        Self::TDirect,
        Self::TDrvAndT,
        Self::UDrvAndU,
        Self::OwnDrvAndOwn,
    ];
}

/// **A block a service names** (§2; model §8 rule 7; round 13, R13-R2; round 20, R20-D3; `M1.49`): a block `blk` no
/// platform composes, requiring nothing, a timer's channel, an absolute deadline, the source or an output; named by a
/// service `drv` that task `t`, task `u`, both, the system's own level or nothing uses, by `uses` or `needs`, or by
/// task `t` itself, or by `drv` beside `t`, `u` or the system's own level naming it too; its own offer bound to by a
/// service `t` uses or by nothing; and task `u` periodic or sporadic. "A declaration of another kind ends the path: a
/// block that only a service names is not hardware", so a served service or policy naming it is the search's conflict,
/// "judged once for the system whatever the assignment" (model §8 rule 7, `M1.49`), `infeasible-configuration`, bound
/// to or not; beside a task's or the system's own name, which makes it hardware, it is a node, bound or not, its
/// namer's named edge from it, running where what reaches it runs; and one only a service the system does not serve
/// names is neither refused nor a node, a requester bound to it alone being an assignment's conflict, which leaves
/// `missing-fact`.
fn served_blocks() -> Vec<Case> {
    let mut cases = Vec::new();
    for by in ServedBy::ALL {
        for requires in Requires::ALL {
            for bound_to in [false, true] {
                for u_sporadic in [false, true] {
                    cases.push(served_block_case(by, requires, bound_to, u_sporadic));
                }
            }
        }
    }
    cases
}

fn served_block_case(by: ServedBy, requires: Requires, bound_to: bool, u_sporadic: bool) -> Case {
    let mut t = Side::default();
    let mut u = Side::default();
    let mut own = Side::default();
    let mut services = Vec::new();
    let mut drv = Service {
        name: "drv".to_string(),
        policy: false,
        side: Side::default(),
    };
    if by == ServedBy::TDrvNeeds {
        drv.side.needs.push("blk".to_string());
    } else {
        drv.side.uses.push("blk".to_string());
    }
    match by {
        ServedBy::TDrvUses | ServedBy::TDrvNeeds => t.uses.push("drv".to_string()),
        ServedBy::UDrv => u.uses.push("drv".to_string()),
        ServedBy::BothDrv => {
            t.uses.push("drv".to_string());
            u.uses.push("drv".to_string());
        }
        ServedBy::OwnDrv => own.uses.push("drv".to_string()),
        ServedBy::NoDrv => {}
        ServedBy::TDirect => t.uses.push("blk".to_string()),
        ServedBy::TDrvAndT => t.uses.extend(names(&["drv", "blk"])),
        ServedBy::UDrvAndU => u.uses.extend(names(&["drv", "blk"])),
        ServedBy::OwnDrvAndOwn => own.uses.extend(names(&["drv", "blk"])),
    }
    if by != ServedBy::TDirect {
        services.push(drv);
    }
    if bound_to {
        services.push(Service {
            name: "xfer".to_string(),
            policy: false,
            side: Side {
                needs: names(&["transfer-engine"]),
                ..Side::default()
            },
        });
        t.uses.push("xfer".to_string());
    }
    let blocks = vec![
        timer_block("timer.d", 4, true),
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        Block {
            name: "blk".to_string(),
            offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: requires.needs(),
                ..Side::default()
            },
            ..Block::default()
        },
    ];
    let platform = Block {
        name: "soc".to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            uses: names(&["timer.d", "dev.s", "uart.c"]),
            ..Side::default()
        },
        ..Block::default()
    };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: if u_sporadic {
                Release::MinSeparation(Dur::Ms(20))
            } else {
                Release::Period(Dur::Ms(20))
            },
            priority: 2,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "blk:{requires:?} named {by:?}; its offer bound to {}; u {}",
            if bound_to { "by xfer" } else { "by nothing" },
            if u_sporadic { "sporadic" } else { "periodic" }
        ),
        description: Description {
            blocks,
            platforms: vec![platform],
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// **One instance's obliged bounds** (§1, §2.4; round 14, R14-R6: "one instance's obliged bounds by the fact's text"):
/// a `deadline-over-delay` requester `dl`, which task `t` uses, writing on `delivery-bound` a bound of 5 ms or one past
/// the arithmetic, and beside it none, a second of 3 ms or a second past the arithmetic. `adapters/1`'s kinds offer one
/// obliged fact at most (`the_reading_transcribes_the_shipped_vocabulary_and_catalog` holds the shipped catalog to
/// that), so an instance owes at most one bound and no order among its facts is reachable. A requirement writing one
/// fact's bound twice gives one obliged value, "the tightest bound its requester writes on that fact" (the adapters
/// record §1, item 5), named once where it lies past the arithmetic; the search refuses (`unsupported-profile`) the
/// two where one bound lies within the arithmetic and the other past it.
fn one_instance_s_bounds() -> Vec<Case> {
    let word = |bound: Dur| {
        if bound.seconds().is_some() {
            bound.spelling()
        } else {
            "past".to_string()
        }
    };
    let mut cases = Vec::new();
    for first in [Dur::Ms(DELIVERY_BOUND_MS), Dur::Spelled(PAST)] {
        for second in [None, Some(Dur::Ms(3)), Some(Dur::Spelled(TIGHT_PAST))] {
            let mut dl = service("dl", Counter::Deadline, false, Vec::new());
            dl.side.at_most.push(("delivery-bound".to_string(), first));
            if let Some(second) = second {
                dl.side.at_most.push(("delivery-bound".to_string(), second));
            }
            cases.push(Case {
                label: format!(
                    "dl writes {} and {}",
                    word(first),
                    second.map_or_else(|| "nothing".to_string(), word)
                ),
                description: Description {
                    blocks: vec![timer_block("timer.d", 2, true)],
                    services: vec![dl],
                    tasks: vec![Task {
                        name: "t".to_string(),
                        side: Side {
                            uses: names(&["dl"]),
                            ..Side::default()
                        },
                        ..Task::default()
                    }],
                    ..Description::default()
                },
            });
        }
    }
    cases
}

/// What the timer of the own-route universe requires: nothing, `dma.x`'s `transfer-engine`, `soc`'s `core-count`, or
/// both.
const TIMER_NEEDS: [&[&str]; 4] = [
    &[],
    &["transfer-engine"],
    &["core-count"],
    &["transfer-engine", "core-count"],
];

/// **A provider bound to the node that takes its channel** (§2.6, §2.9; round 14, R14-R1): the timer `timer.d`,
/// requiring one choice of [`TIMER_NEEDS`]; the block `dma.x`, offering `transfer-engine`, and the platform `soc`,
/// offering `core-count` and composing both, each taking a channel of the timer — an alarm it programs itself, or an
/// absolute deadline through a `deadline-over-delay` instance — or none; task `t` using `xfer`, which needs
/// `transfer-engine`, `cpu`, which needs `core-count`, both or neither; and task `u` using nothing, `xfer` or `cpu`.
/// So where the timer's own requirement is bound to the node taking its channel, that node's own timer route runs the
/// timer's code, which reaches it — "nothing else that route runs reaches it" fails, and the route is one of its users
/// (§2.9: "that route's own code delivering to it is no caller, but another node's code there calling it is") — and
/// where it is not, the route delivering to it is no caller; the timer and the block it is bound to make one
/// initialisation step of two members, and so do the timer and the platform composing it where each is bound to the
/// other, a binding keeping its edge across composition (§2.7; round 16, R16-B2).
fn own_route_through_its_provider() -> Vec<Case> {
    const TAKES: [Requires; 3] = [Requires::Nothing, Requires::Alarm, Requires::Deadline];
    const T_USES: [&[&str]; 4] = [&[], &["xfer"], &["cpu"], &["xfer", "cpu"]];
    const U_USES: [&[&str]; 3] = [&[], &["xfer"], &["cpu"]];
    let mut cases = Vec::new();
    for timer_needs in TIMER_NEEDS {
        for dma in TAKES {
            for soc in TAKES {
                for t_uses in T_USES {
                    for u_uses in U_USES {
                        let mut timer = timer_block("timer.d", 4, true);
                        timer.side.needs = names(timer_needs);
                        let dma_x = Block {
                            name: "dma.x".to_string(),
                            offers: names(&[
                                "(transfer-engine true)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: Side {
                                needs: dma.needs(),
                                ..Side::default()
                            },
                            ..Block::default()
                        };
                        let platform = Block {
                            name: "soc".to_string(),
                            offers: names(&[
                                "(core-count 1 tick)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: Side {
                                needs: soc.needs(),
                                uses: names(&["timer.d", "dma.x"]),
                                ..Side::default()
                            },
                            ..Block::default()
                        };
                        let services = [("xfer", "transfer-engine"), ("cpu", "core-count")]
                            .iter()
                            .filter(|(name, _)| t_uses.contains(name) || u_uses.contains(name))
                            .map(|(name, fact)| Service {
                                name: (*name).to_string(),
                                policy: false,
                                side: Side {
                                    needs: names(&[fact]),
                                    ..Side::default()
                                },
                            })
                            .collect();
                        let task = |name: &str, priority: u32, uses: &[&str]| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(10 * u64::from(priority))),
                            priority,
                            side: Side {
                                uses: names(uses),
                                ..Side::default()
                            },
                            ..Task::default()
                        };
                        cases.push(Case {
                            label: format!(
                                "timer.d needs {timer_needs:?}; dma.x takes {dma:?}; soc takes {soc:?}; t uses \
                                 {t_uses:?}; u uses {u_uses:?}"
                            ),
                            description: Description {
                                blocks: vec![timer, dma_x],
                                platforms: vec![platform],
                                services,
                                tasks: vec![task("t", 1, t_uses), task("u", 2, u_uses)],
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What each namer of the naming-order universe names: nothing; `y.b`; `y.b` and `y`; or `y!` and `y` — each pair
/// written against its texts' order, one text a prefix of the other.
const NAMED_SYSTEMS: [&[&str]; 4] = [&[], &["y.b"], &["y.b", "y"], &["y!", "y"]];

/// **The naming-system refusal's order, over names one of which prefixes another** (§1; round 13, R13-R1: "By the
/// namer's text, then the system's"; round 14, R14-D1: "a closing backquote sorts after `.`, so `y.b` is named before
/// `y`"): the lowered system `sys` with tasks `t`, first in runtime order, and `t!`; the services `m.s` and `m`,
/// declared in that order, both used by `t`; the systems `y.b`, `y!` and `y`, declared in that order; task `t`, task
/// `t!`, `m.s`, `m` and `sys`'s own level each naming one choice of [`NAMED_SYSTEMS`]; every name written by `uses` or
/// every one by `needs`; and `y.b`'s own level naming `y` or not. So the namers' texts — `(task sys t!)`, `(task sys
/// t)`, `m`, `m.s`, `sys`, and `y.b` once it is reached — and the systems' — `y`, `y!`, `y.b` — are each unlike
/// their runtime, declaration and name orders and the order their messages' words would give: `!` lies below the `)`
/// that closes `(task sys t)`, and a closing backquote above `!` and `.`. One namer names two systems, and a system
/// reached names another.
fn naming_order() -> Vec<Case> {
    const NAMERS: usize = 5;
    let mut cases = Vec::new();
    for choice in 0..NAMED_SYSTEMS
        .len()
        .pow(u32::try_from(NAMERS).expect("small"))
    {
        let pick = |namer: usize| {
            NAMED_SYSTEMS[choice
                / NAMED_SYSTEMS
                    .len()
                    .pow(u32::try_from(namer).expect("small"))
                % NAMED_SYSTEMS.len()]
        };
        let [t, t_bang, m_s, m, own] = [pick(0), pick(1), pick(2), pick(3), pick(4)];
        for by_needs in [false, true] {
            for yb_names_y in [false, true] {
                let side = |base: &[&str], named: &[&str]| {
                    named_in(
                        Side {
                            uses: names(base),
                            ..Side::default()
                        },
                        named,
                        by_needs,
                    )
                };
                let system = |name: &str, own: Side| System {
                    name: name.to_string(),
                    tasks: vec![Task {
                        name: "z".to_string(),
                        release: Release::Period(Dur::Ms(20)),
                        ..Task::default()
                    }],
                    own,
                    platform: "soc".to_string(),
                };
                let service = |name: &str, named: &[&str]| Service {
                    name: name.to_string(),
                    policy: false,
                    side: side(&[], named),
                };
                cases.push(Case {
                    label: format!(
                        "t>{t:?} t!>{t_bang:?} m.s>{m_s:?} m>{m:?} sys>{own:?} by {} y.b>y={yb_names_y}",
                        if by_needs { "needs" } else { "uses" }
                    ),
                    description: Description {
                        blocks: vec![Block {
                            name: "uart.c".to_string(),
                            offers: names(&[
                                "(observable-output true)",
                                "(reachable-at-privilege machine)",
                            ]),
                            ..Block::default()
                        }],
                        services: vec![service("m.s", m_s), service("m", m)],
                        tasks: vec![
                            Task {
                                name: "t".to_string(),
                                priority: 1,
                                side: side(&["m.s", "m"], t),
                                ..Task::default()
                            },
                            Task {
                                name: "t!".to_string(),
                                release: Release::Period(Dur::Ms(20)),
                                priority: 2,
                                side: side(&[], t_bang),
                                ..Task::default()
                            },
                        ],
                        own: side(&[], own),
                        others: vec![
                            system(
                                "y.b",
                                if yb_names_y {
                                    side(&[], &["y"])
                                } else {
                                    Side::default()
                                },
                            ),
                            system("y!", Side::default()),
                            system("y", Side::default()),
                        ],
                        others_first: true,
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// `side` naming `declarations` too, by `needs` or by `uses`.
fn named_in(side: Side, declarations: &[&str], by_needs: bool) -> Side {
    named(side, names(declarations), by_needs)
}

/// What one requester of the bound-otherwise universe requires, of the timer `d`, which offers `observable-output`
/// too, or of `nd`, which offers `relative-delay` as `false` (round 15, R15-D2: the receivers' "bound to its provider
/// itself" named no fact, "where the code, §2.6 and model §8 rule 2 read one").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OnRoute {
    Nothing,
    /// `(needs observable-output)`: bound to `d` for a shared fact, which never takes a channel (model §8 rule 2).
    Shared,
    /// `(needs counter-width tick-rate observation-coherent)`: an extended counter only the shared `epoch-extender`
    /// instance over `d` offers, whose own binding beneath makes `d`'s timer route, "which carries the requesters bound
    /// to it" (rule 15); its `wake` runs the instance's code alone (§2.6).
    Wake,
    /// Both: named by `d`'s timer route, and bound to `d` itself for a fact that takes no channel.
    WakeShared,
    /// `(needs relative-delay)`: an alarm it programs at `d` itself.
    Alarm,
    /// `(needs relative-delay observable-output)`: an alarm it programs at `d` itself, and `d`'s output beside.
    AlarmShared,
    /// `(needs absolute-deadline observable-output)`: an expiry its own `deadline-over-delay` instance over `d`
    /// delivers, and `d`'s output beside.
    ExpiryShared,
    /// `(relative-delay false)`, met by `nd`: a consumed fact at `false`, which "takes none" (rule 2) and makes no route
    /// (rule 8: "bound through a consumed fact at `true`"). A task writes no constraint clause, so the language refuses
    /// one writing it.
    False,
}

impl OnRoute {
    fn side(self) -> Side {
        let needs: &[&str] = match self {
            Self::Nothing | Self::False => &[],
            Self::Shared => &["observable-output"],
            Self::Wake => &["counter-width", "tick-rate", "observation-coherent"],
            Self::WakeShared => &[
                "counter-width",
                "tick-rate",
                "observation-coherent",
                "observable-output",
            ],
            Self::Alarm => &["relative-delay"],
            Self::AlarmShared => &["relative-delay", "observable-output"],
            Self::ExpiryShared => &["absolute-deadline", "observable-output"],
        };
        Side {
            needs: names(needs),
            other: if self == Self::False {
                names(&["(relative-delay false)"])
            } else {
                Vec::new()
            },
            ..Side::default()
        }
    }
}

/// **Bound to a timer route's provider otherwise** (§1, §2.6, §2.9; round 15, R15-D2): tasks `t` and `u`, either first
/// in runtime order, and the system's own level, over the timer `d`, which offers `observable-output` too, and `nd`,
/// which offers `relative-delay` as `false`; each programming an alarm at `d` beside `d`'s output, an expiry beside it,
/// counting an extended counter whose instance's channel is `d`'s, with `d`'s output beside or not, or requiring `d`'s
/// output alone, or nothing; `t` and the system's own level also `(relative-delay false)`; and a service `s` both tasks
/// use, declared or not, counting the extended counter beside `d`'s output, programming an alarm at `d`, or requiring
/// `(relative-delay false)`. So §1's receivers are each "bound to its provider itself by a binding that takes a channel
/// (model §8 rule 2)", never by a binding that takes none — one a timer route names through an instance, or one the route
/// does not name, beside one programming an alarm there — §2.6's code a timer route runs is a node bound for "a consumed
/// fact", and §2.9's multiplexed node is one "that takes a channel", a consumed fact at `false` taking none. The
/// system's own level at `false` beside a task programming an alarm is no such system: the system's side, read whole,
/// contradicts itself, and the search refuses it ([`SEARCH_READS_THE_SIDE_WHOLE`]). Beside `s` programming one, or a
/// task receiving an expiry, it stands, as `s` at `false` does beside a task programming one, a service being a side of
/// its own.
fn bound_otherwise_on_a_route() -> Vec<Case> {
    const T: [OnRoute; 7] = [
        OnRoute::Nothing,
        OnRoute::Shared,
        OnRoute::Wake,
        OnRoute::WakeShared,
        OnRoute::AlarmShared,
        OnRoute::ExpiryShared,
        OnRoute::False,
    ];
    const U: [OnRoute; 3] = [OnRoute::Nothing, OnRoute::WakeShared, OnRoute::AlarmShared];
    const S: [Option<OnRoute>; 4] = [
        None,
        Some(OnRoute::WakeShared),
        Some(OnRoute::Alarm),
        Some(OnRoute::False),
    ];
    let mut cases = Vec::new();
    for t_requires in T {
        for u_requires in U {
            for own_requires in T {
                for s_requires in S {
                    for u_first in [false, true] {
                        let mut d = timer_block("d", 6, true);
                        let at = d.offers.len() - 2;
                        d.offers.insert(at, "(observable-output true)".to_string());
                        let nd = Block {
                            name: "nd".to_string(),
                            offers: names(&[
                                "(relative-delay false)",
                                "(reachable-at-privilege machine)",
                            ]),
                            ..Block::default()
                        };
                        let uses: &[&str] = if s_requires.is_some() { &["s"] } else { &[] };
                        let task = |name: &str, priority: u32, requires: OnRoute| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(20)),
                            priority,
                            side: Side {
                                uses: names(uses),
                                ..requires.side()
                            },
                            ..Task::default()
                        };
                        let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
                        cases.push(Case {
                            label: format!(
                                "t:{t_requires:?} u:{u_requires:?} own:{own_requires:?} s:{s_requires:?} first={}",
                                if u_first { "u" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![d, nd],
                                services: s_requires
                                    .map(|requires| Service {
                                        name: "s".to_string(),
                                        policy: false,
                                        side: requires.side(),
                                    })
                                    .into_iter()
                                    .collect(),
                                tasks: vec![
                                    task("t", t_priority, t_requires),
                                    task("u", u_priority, u_requires),
                                ],
                                own: own_requires.side(),
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What one requester of the periodic universe requires, of the source `dev`, which offers `debug-port` too, or of
/// `ns`, which offers `interrupt-source` as `false` (round 15, R15-D2: the periodic task's "bound to an interrupt source
/// itself" named no fact).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OfSource {
    Nothing,
    /// `(needs debug-port)`: bound to `dev`, a block offering `interrupt-source`, for another fact.
    Other,
    /// `(needs interrupt-source)`: bound to `dev` for its interrupt.
    Source,
    /// `(needs interrupt-source debug-port)`: both.
    Both,
    /// `(interrupt-source false)`, met by `ns`: bound for `interrupt-source`, its requirement met by `false`, which is no
    /// source (model §8 rule 8: "bound through `interrupt-source` at `true`"). A task writes no constraint clause, so
    /// the language refuses one writing it.
    False,
}

impl OfSource {
    fn side(self) -> Side {
        let needs: &[&str] = match self {
            Self::Nothing | Self::False => &[],
            Self::Other => &["debug-port"],
            Self::Source => &["interrupt-source"],
            Self::Both => &["interrupt-source", "debug-port"],
        };
        Side {
            needs: names(needs),
            other: if self == Self::False {
                names(&["(interrupt-source false)"])
            } else {
                Vec::new()
            },
            ..Side::default()
        }
    }
}

/// **A periodic task bound to a source's block otherwise** (§1, §2.2, §2.5; round 15, R15-D2): periodic tasks `t` and
/// `p`, either first in runtime order, a sporadic task `u`, and the system's own level, over the source `dev`, which
/// offers `debug-port` too, and `ns`, which offers `interrupt-source` as `false`; `t` and `p` each requiring `dev`'s
/// interrupt, its `debug-port`, both or nothing, `t` also `(interrupt-source false)`; the system's own level each of
/// those but both; and a service `e` that `u` uses, declared or not, requiring `dev`'s interrupt or `(interrupt-source
/// false)`. So §1's periodic refusal takes a task "bound itself for `interrupt-source`, its requirement met by `true`",
/// in runtime order past one bound to the same block for another fact, and a binding at `false` is no route and
/// releases no task. The system's own level at `false` beside `t` or `p` requiring `dev`'s interrupt is no such system:
/// the system's side, read whole, contradicts itself, and the search refuses it ([`SEARCH_READS_THE_SIDE_WHOLE`]). `e`
/// at `false` beside them stands, a service being a side of its own.
fn periodic_bound_otherwise() -> Vec<Case> {
    const T: [OfSource; 5] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::Both,
        OfSource::False,
    ];
    const P: [OfSource; 4] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::Both,
    ];
    const OWN: [OfSource; 4] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::False,
    ];
    const E: [Option<OfSource>; 3] = [None, Some(OfSource::Source), Some(OfSource::False)];
    let mut cases = Vec::new();
    for t_requires in T {
        for p_requires in P {
            for own_requires in OWN {
                for e_requires in E {
                    for p_first in [false, true] {
                        let mut dev = source_block("dev", Separation::Stated(20), "");
                        let at = dev.offers.len() - 1;
                        dev.offers.insert(at, "(debug-port true)".to_string());
                        let ns = Block {
                            name: "ns".to_string(),
                            offers: names(&[
                                "(interrupt-source false)",
                                "(reachable-at-privilege machine)",
                            ]),
                            ..Block::default()
                        };
                        let periodic = |name: &str, priority: u32, requires: OfSource| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(20)),
                            priority,
                            side: requires.side(),
                            ..Task::default()
                        };
                        let (t_priority, p_priority) = if p_first { (2, 1) } else { (1, 2) };
                        cases.push(Case {
                            label: format!(
                                "t:{t_requires:?} p:{p_requires:?} own:{own_requires:?} e:{e_requires:?} first={}",
                                if p_first { "p" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![dev, ns],
                                services: e_requires
                                    .map(|requires| Service {
                                        name: "e".to_string(),
                                        policy: false,
                                        side: requires.side(),
                                    })
                                    .into_iter()
                                    .collect(),
                                tasks: vec![
                                    periodic("t", t_priority, t_requires),
                                    periodic("p", p_priority, p_requires),
                                    Task {
                                        name: "u".to_string(),
                                        release: Release::MinSeparation(Dur::Ms(20)),
                                        priority: 3,
                                        side: Side {
                                            uses: if e_requires.is_some() {
                                                names(&["e"])
                                            } else {
                                                Vec::new()
                                            },
                                            ..Side::default()
                                        },
                                        ..Task::default()
                                    },
                                ],
                                own: own_requires.side(),
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What a block or a platform the lowered system does not serve names (round 15, R15-R2): nothing; the service
/// `alarm`, which the lowered system's task uses, by `uses`; the policy `pol.p` by `needs`; the system `sys` by `uses`;
/// or the system `other` by `needs`.
const UNSERVED_NAMING: [Naming; 5] = [
    &[],
    &[(false, "alarm")],
    &[(true, "pol.p")],
    &[(false, "sys")],
    &[(true, "other")],
];
/// What `soc`, the platform of `sys`, names beside the blocks it composes (round 15, R15-R5): nothing; the policy
/// `pol.p` by `uses`; the system `other`, by `uses` or by `needs`; or the service `via.s`, which names `lone` by `uses`,
/// so that a system serving `soc` serves `via.s` through what hardware names (§1: "serving what it names"), and
/// `via.s` leaves it naming a block outside its hardware (model §8 rule 7, `M1.49`).
const SOC_NAMES: [Naming; 5] = [
    &[],
    &[(false, "pol.p")],
    &[(false, "other")],
    &[(true, "other")],
    &[(false, "via.s")],
];

/// **Hardware the lowered system does not serve** (§1; round 15, R15-R2 and R15-R5): the system `sys`, its task `t`
/// using `alarm` bound to `timer.d` and its task `u` using `xfer` bound to `dma.x`, on the platform `soc` composing both;
/// the system `other` on `host.q`, which composes `blk.q`; and the block `lone`, which no platform composes and only the
/// service `via.s` names. Each of `blk.q`, `host.q` and `lone` names one choice of [`UNSERVED_NAMING`], and `soc` one of
/// [`SOC_NAMES`]; each description is lowered for `sys` and for `other`. So hardware a system does not serve
/// names what is not hardware — a service it serves, a policy, a system, the lowered system itself — and is refused by
/// neither naming bullet, "nothing else reach[ing] its plan"; the same hardware, served when its own system is lowered,
/// is refused, a platform naming a system by `uses` or `needs` meeting the naming-system refusal first; a system
/// reached by a platform serves its own `platform` clause, whose platform names the lowered one; and a system serving
/// `soc` — its own platform, or another system's it reaches — with `soc` naming `via.s` is model §8 rule 7's conflict
/// (`M1.49`), whatever else its hardware names, `via.s` naming `lone`, which neither system's hardware holds.
fn unserved_hardware_naming() -> Vec<Case> {
    let options = UNSERVED_NAMING.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(3) {
        let pick = |namer: u32| UNSERVED_NAMING[choice / options.pow(namer) % options];
        let (blk_q, host_q, lone) = (pick(0), pick(1), pick(2));
        for soc in SOC_NAMES {
            let words = |naming: Naming| -> String {
                let named: Vec<String> = naming
                    .iter()
                    .map(|(by_needs, name)| {
                        format!("{}:{name}", if *by_needs { "needs" } else { "uses" })
                    })
                    .collect();
                format!("[{}]", named.join(","))
            };
            let plain = |name: &str, offer: &str, named: Naming| Block {
                name: name.to_string(),
                offers: names(&[offer, "(reachable-at-privilege machine)"]),
                side: naming(Side::default(), named),
                ..Block::default()
            };
            let platform = |name: &str, composes: &[&str], extra: Naming| Block {
                name: name.to_string(),
                offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
                side: naming(
                    Side {
                        uses: names(composes),
                        ..Side::default()
                    },
                    extra,
                ),
                ..Block::default()
            };
            let simple = |name: &str, policy: bool, needs: &[&str]| Service {
                name: name.to_string(),
                policy,
                side: Side {
                    needs: names(needs),
                    ..Side::default()
                },
            };
            let task = |name: &str, period: u64, priority: u32, uses: &[&str]| Task {
                name: name.to_string(),
                release: Release::Period(Dur::Ms(period)),
                priority,
                side: Side {
                    uses: names(uses),
                    ..Side::default()
                },
                ..Task::default()
            };
            let sys = System {
                name: "sys".to_string(),
                tasks: vec![task("t", 10, 1, &["alarm"]), task("u", 20, 2, &["xfer"])],
                own: Side::default(),
                platform: "soc".to_string(),
            };
            let other = System {
                name: "other".to_string(),
                tasks: vec![task("o", 20, 1, &[])],
                own: Side::default(),
                platform: "host.q".to_string(),
            };
            let label = format!(
                "blk.q names {} host.q names {} lone names {} soc names {}",
                words(blk_q),
                words(host_q),
                words(lone),
                words(soc)
            );
            let lowering = |lowered: &System, beside: &System, others_first: bool| Case {
                label: format!("{label}: lowering {}", lowered.name),
                description: Description {
                    system: lowered.name.clone(),
                    blocks: vec![
                        timer_block("timer.d", 2, false),
                        plain("dma.x", "(transfer-engine true)", &[]),
                        plain("blk.q", "(debug-port true)", blk_q),
                        plain("lone", "(debug-port true)", lone),
                    ],
                    platforms: vec![
                        platform("soc", &["timer.d", "dma.x"], soc),
                        platform("host.q", &["blk.q"], host_q),
                    ],
                    services: vec![
                        simple("alarm", false, &["relative-delay"]),
                        simple("xfer", false, &["transfer-engine"]),
                        simple("pol.p", true, &[]),
                        Service {
                            name: "via.s".to_string(),
                            policy: false,
                            side: Side {
                                uses: names(&["lone"]),
                                ..Side::default()
                            },
                        },
                    ],
                    tasks: lowered.tasks.clone(),
                    own: lowered.own.clone(),
                    platform: lowered.platform.clone(),
                    others: vec![beside.clone()],
                    others_first,
                },
            };
            cases.push(lowering(&sys, &other, false));
            cases.push(lowering(&other, &sys, true));
        }
    }
    cases
}

/// What a task of the own-sources universe uses: nothing, the handler `evt.a` of the source `dev.a`, the handler
/// `evt.z` of the source `dev.z`, or both.
const HANDLERS_USED: [&[&str]; 4] = [&[], &["evt.a"], &["evt.z"], &["evt.a", "evt.z"]];

/// **Sporadic tasks and their own sources** (§2.2; round 16, R16-B1: "a sporadic task bound itself for
/// `interrupt-source` is released by that source; the routes it reaches decide only where it binds none"): the
/// sporadic task `t`, at 20 ms, bound itself for `interrupt-source` or not, and the sporadic task `u`, at 40 ms, binding
/// none; `t` using one choice of [`HANDLERS_USED`] and `u` one of the first three; the sources `dev.z` and `dev.a`, each
/// stating every class of [`SEPARATIONS`], declared in either order. Each handler is held to its own source by the
/// power states it requires, `(available-in-state …)`, which only that source offers — a qualifier judged at every
/// provider bound for the handler (model §8 rule 13); a task writes no constraint clause, so which source `t`'s own
/// binding takes is the search's, read from the record. So a task's own source decides alone: beside another source's
/// route it reaches, whether that one alone would release it every or leave it unknown; beside its own source's
/// handler, that route naming a requester it reaches too; and a task binding none, beside it, is released by the routes
/// it reaches. Since model §8 rule 16 landed (`M4.1.2.3`), `t` bound itself takes a source stating at least its 20 ms,
/// and the search refuses the system where neither does, every record binding it to one that does not judged instead
/// ([`RULE_16_RECORD`]).
fn own_sources() -> Vec<Case> {
    let mut cases = Vec::new();
    for a_first in [false, true] {
        for t_bound in [false, true] {
            for t_uses in HANDLERS_USED {
                for u_uses in &HANDLERS_USED[..3] {
                    for z in SEPARATIONS {
                        for a in SEPARATIONS {
                            cases.push(own_sources_case(
                                a_first,
                                (t_bound, t_uses),
                                u_uses,
                                (z, a),
                            ));
                        }
                    }
                }
            }
        }
    }
    cases
}

fn own_sources_case(
    a_first: bool,
    (t_bound, t_uses): (bool, &[&str]),
    u_uses: &[&str],
    (z, a): (Separation, Separation),
) -> Case {
    let mut blocks = vec![
        source_block("dev.z", z, "run idle"),
        source_block("dev.a", a, "run sleep"),
    ];
    if a_first {
        blocks.reverse();
    }
    let handler = |name: &str, states: &str| {
        let mut evt = service(name, Counter::None, true, Vec::new());
        evt.side
            .other
            .push(format!("(available-in-state {states})"));
        evt
    };
    let services = [("evt.a", "run sleep"), ("evt.z", "run idle")]
        .iter()
        .filter(|(name, _)| t_uses.contains(name) || u_uses.contains(name))
        .map(|(name, states)| handler(name, states))
        .collect();
    let sporadic = |name: &str, ms: u64, priority: u32, bound: bool, uses: &[&str]| Task {
        name: name.to_string(),
        release: Release::MinSeparation(Dur::Ms(ms)),
        priority,
        side: Side {
            needs: if bound {
                names(&[INTERRUPT_FACT])
            } else {
                Vec::new()
            },
            uses: names(uses),
            ..Side::default()
        },
        ..Task::default()
    };
    Case {
        label: format!(
            "{} first; t {}uses {t_uses:?}; u uses {u_uses:?}; dev.z:{z:?} dev.a:{a:?}",
            if a_first { "dev.a" } else { "dev.z" },
            if t_bound {
                "bound itself for interrupt-source, "
            } else {
                ""
            },
        ),
        description: Description {
            blocks,
            services,
            tasks: vec![
                sporadic("t", 20, 1, t_bound, t_uses),
                sporadic("u", 40, 2, false, u_uses),
            ],
            ..Description::default()
        },
    }
}

/// The names of the composition universe's platform and its two blocks, `[platform, block, block]`: each composite's
/// text below its part's, or above it.
const COMPOSITION_NAMES: [[&str; 3]; 2] = [["n.a", "n.m", "n.z"], ["n.z", "n.m", "n.a"]];

/// What one declaration of the composition universe requires of the other two: nothing, the first's offer, the
/// second's, or both.
const OF_THE_OTHERS: [[bool; 2]; 4] = [[false, false], [true, false], [false, true], [true, true]];

/// **A binding across composition** (§2.7; model §8 rule 12; round 16, R16-B2: "a binding keeps its edge, composition
/// alone giving none"): the platform `P`, the system's, offering `core-count`; the block `B`, offering
/// `transfer-engine`; and the block `C`, offering `debug-port`. `P` composes `B` and `C`, or `B` and, through `B`, `C`;
/// each of the three requires each choice of [`OF_THE_OTHERS`] of the other two's offers, its only provider; and the
/// three are named by one choice of [`COMPOSITION_NAMES`], each composite's text below its part's or above it. So a
/// composite is bound to its part, its part to it, or both, directly and at depth, a platform and a block each the
/// composite, under names whose order differs from the binding's: the edge a binding gives orders two steps where
/// their texts would order them the other way, and a composite and its part bound each to the other are one step.
fn bound_across_composition() -> Vec<Case> {
    const OFFERS: [(&str, &str); 3] = [
        ("core-count", "(core-count 1 tick)"),
        ("transfer-engine", "(transfer-engine true)"),
        ("debug-port", "(debug-port true)"),
    ];
    let mut cases = Vec::new();
    for names_of in COMPOSITION_NAMES {
        for deep in [false, true] {
            for choice in 0..OF_THE_OTHERS.len().pow(3) {
                let requires = |who: usize| -> Vec<String> {
                    let wanted = OF_THE_OTHERS
                        [choice / OF_THE_OTHERS.len().pow(u32::try_from(who).expect("small")) % 4];
                    let others: Vec<usize> = (0..3).filter(|i| *i != who).collect();
                    others
                        .iter()
                        .zip(wanted)
                        .filter(|(_, wanted)| *wanted)
                        .map(|(other, _)| OFFERS[*other].0.to_string())
                        .collect()
                };
                let declared = |who: usize, uses: &[&str]| Block {
                    name: names_of[who].to_string(),
                    offers: names(&[OFFERS[who].1, "(reachable-at-privilege machine)"]),
                    side: Side {
                        needs: requires(who),
                        uses: names(uses),
                        ..Side::default()
                    },
                    ..Block::default()
                };
                let [p, b, c] = names_of;
                let (p_uses, b_uses): (Vec<&str>, Vec<&str>) = if deep {
                    (vec![b], vec![c])
                } else {
                    (vec![b, c], vec![])
                };
                cases.push(Case {
                    label: format!(
                        "{p} composes {p_uses:?}, {b} composes {b_uses:?}; {p} needs {:?}, {b} needs {:?}, {c} needs \
                         {:?}",
                        requires(0),
                        requires(1),
                        requires(2)
                    ),
                    description: Description {
                        blocks: vec![declared(1, &b_uses), declared(2, &[])],
                        platforms: vec![declared(0, &p_uses)],
                        tasks: vec![Task::default()],
                        platform: p.to_string(),
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// What one requester of the obliged-bounds universe writes: no requirement, `(needs absolute-deadline)` with its
/// instance's obliged `delivery-bound` bare, with a bound of 5 ms, or with one past the arithmetic.
const DEADLINE_STATES: [Option<BoundState>; 4] = [
    None,
    Some(BoundState::Bare),
    Some(BoundState::Within),
    Some(BoundState::Past),
];

/// **The obliged bounds of every requester an instance serves** (§1, §2.4; round 16, R16-R1: the reading took "an
/// obliged bound's requester among services, tasks and hardware, never the own level"): the system's own level, the
/// service `dl` and the policy `pol` its task `t` uses, its platform `soc` and the block `blk` it composes, each
/// writing one choice of [`DEADLINE_STATES`], each met by an exclusive `deadline-over-delay` instance over the one timer.
/// So §1 names each bound past the arithmetic by its instance's text, the system's own level's among them, ahead of
/// that level's expiry, which a bound within the arithmetic leaves the refusal of its receiver (§1); and a plan writes
/// a platform's, a block's, a policy's and a service's bound on its instance's line.
fn obliged_bounds_of_every_requester() -> Vec<Case> {
    let options = DEADLINE_STATES.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(5) {
        let pick = |who: u32| DEADLINE_STATES[choice / options.pow(who) % options];
        let side = |state: Option<BoundState>| -> Side {
            let mut side = Side::default();
            if let Some(state) = state {
                side.needs.push("absolute-deadline".to_string());
                match state {
                    BoundState::Bare => {}
                    BoundState::Within => side
                        .at_most
                        .push(("delivery-bound".to_string(), Dur::Ms(DELIVERY_BOUND_MS))),
                    BoundState::Past => side
                        .at_most
                        .push(("delivery-bound".to_string(), Dur::Spelled(PAST))),
                }
            }
            side
        };
        let (own, dl, pol, soc, blk) = (pick(0), pick(1), pick(2), pick(3), pick(4));
        let mut platform = Block {
            name: "soc".to_string(),
            offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
            side: side(soc),
            ..Block::default()
        };
        platform.side.uses = names(&["timer.d", "blk"]);
        cases.push(Case {
            label: format!("own:{own:?} dl:{dl:?} pol:{pol:?} soc:{soc:?} blk:{blk:?}"),
            description: Description {
                blocks: vec![
                    timer_block("timer.d", 5, true),
                    Block {
                        name: "blk".to_string(),
                        offers: names(&[
                            "(transfer-engine true)",
                            "(reachable-at-privilege machine)",
                        ]),
                        side: side(blk),
                        ..Block::default()
                    },
                ],
                platforms: vec![platform],
                services: vec![
                    Service {
                        name: "dl".to_string(),
                        policy: false,
                        side: side(dl),
                    },
                    Service {
                        name: "pol".to_string(),
                        policy: true,
                        side: side(pol),
                    },
                ],
                tasks: vec![Task {
                    side: Side {
                        uses: names(&["dl", "pol"]),
                        ..Side::default()
                    },
                    ..Task::default()
                }],
                own: side(own),
                ..Description::default()
            },
        });
    }
    cases
}

/// What a namer of the named-hardware universe names, and by which clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HardwareNamed {
    Nothing,
    BlockByUses,
    BlockByNeeds,
    PlatformByUses,
    PlatformByNeeds,
}

impl HardwareNamed {
    const ALL: [Self; 5] = [
        Self::Nothing,
        Self::BlockByUses,
        Self::BlockByNeeds,
        Self::PlatformByUses,
        Self::PlatformByNeeds,
    ];

    /// `side` naming what this names, of `block` and `platform`.
    fn name(self, side: &mut Side, block: &str, platform: &str) {
        match self {
            Self::Nothing => {}
            Self::BlockByUses => side.uses.push(block.to_string()),
            Self::BlockByNeeds => side.needs.push(block.to_string()),
            Self::PlatformByUses => side.uses.push(platform.to_string()),
            Self::PlatformByNeeds => side.needs.push(platform.to_string()),
        }
    }
}

/// The names of the named-hardware universe's service and policy, `[service, policy]`: each text below every
/// hardware text, `m.…`, so that a named edge orders its two nodes against their texts, or above them.
const NAMER_NAMES: [[&str; 2]; 2] = [["a.s", "a.p"], ["z.s", "z.p"]];

/// Who uses the named-hardware universe's service and policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamersUsed {
    /// Task `t` uses both.
    ByTask,
    /// The system's own level uses both: neither is in a job.
    ByOwnLevel,
    /// Task `t` uses the service, and nothing the policy: the system does not serve it, so it is no node, and what it
    /// names it serves none of (model §8 rule 7).
    PolicyByNothing,
}

const NAMERS_USED: [NamersUsed; 3] = [
    NamersUsed::ByTask,
    NamersUsed::ByOwnLevel,
    NamersUsed::PolicyByNothing,
];

/// **Hardware a service or a policy names** (§2.7; model §8 rule 12; round 17, R17-D1: "a service or a policy naming a
/// block or a platform gets no initialisation edge, though §2 makes that name a call"; R17-R2): the block `m.blk`,
/// offering `transfer-engine`, composed by the platform `m.soc` or by nothing; `m.soc`, the system's, offering
/// `core-count` and composing `m.out`, which offers `observable-output`; each of `m.blk` and `m.soc` requiring
/// `observable-output` or nothing; a service and a policy named by one choice of [`NAMER_NAMES`], each naming one choice
/// of [`HardwareNamed`], the service bound to `m.blk` for `transfer-engine` or not and the policy to `m.soc` for
/// `core-count` or not; and one choice of [`NAMERS_USED`]. So a block or a platform a service or a policy names is a
/// node as a provider, as bound for requirements of its own, as both, or, since round 18 (R18-D1), as named alone, its
/// edge `named` alone, and a policy the system does not serve is no namer; the edge's line holds a binding's reason
/// beside `named`, or `named` alone,
/// ordering two steps unlike their texts or like them; a block no platform composes is served through its namer alone,
/// a served namer naming it being the search's conflict and a requester bound to it alone an assignment's (model §8
/// rule 7); and each node is in a job or in none, owing its call cost and its masked runs where it is in one.
fn named_hardware() -> Vec<Case> {
    const BLOCK: &str = "m.blk";
    const PLATFORM: &str = "m.soc";
    let mut cases = Vec::new();
    for [s, p] in NAMER_NAMES {
        for s_names in HardwareNamed::ALL {
            for p_names in HardwareNamed::ALL {
                for (choice, users) in (0..1u32 << 5).flat_map(|c| NAMERS_USED.map(|u| (c, u))) {
                    let bit = |i: u32| choice >> i & 1 == 1;
                    let (blk_requires, s_binds, composed, soc_requires, p_binds) =
                        (bit(0), bit(1), bit(2), bit(3), bit(4));
                    let output = |requires: bool| {
                        if requires {
                            names(&["observable-output"])
                        } else {
                            Vec::new()
                        }
                    };
                    let block = Block {
                        name: BLOCK.to_string(),
                        offers: names(&[
                            "(transfer-engine true)",
                            "(reachable-at-privilege machine)",
                        ]),
                        side: Side {
                            needs: output(blk_requires),
                            ..Side::default()
                        },
                        ..Block::default()
                    };
                    let mut composes = names(&["m.out"]);
                    if composed {
                        composes.push(BLOCK.to_string());
                    }
                    let platform = Block {
                        name: PLATFORM.to_string(),
                        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
                        side: Side {
                            needs: output(soc_requires),
                            uses: composes,
                            ..Side::default()
                        },
                        ..Block::default()
                    };
                    let mut service = Service {
                        name: s.to_string(),
                        policy: false,
                        side: Side::default(),
                    };
                    if s_binds {
                        service.side.needs.push("transfer-engine".to_string());
                    }
                    s_names.name(&mut service.side, BLOCK, PLATFORM);
                    let mut policy = Service {
                        name: p.to_string(),
                        policy: true,
                        side: Side::default(),
                    };
                    if p_binds {
                        policy.side.needs.push("core-count".to_string());
                    }
                    p_names.name(&mut policy.side, BLOCK, PLATFORM);
                    let using = |used: &[&str]| Side {
                        uses: names(used),
                        ..Side::default()
                    };
                    let (task_side, own) = match users {
                        NamersUsed::ByTask => (using(&[s, p]), Side::default()),
                        NamersUsed::ByOwnLevel => (Side::default(), using(&[s, p])),
                        NamersUsed::PolicyByNothing => (using(&[s]), Side::default()),
                    };
                    cases.push(Case {
                        label: format!(
                            "{s} names {s_names:?}{}; {p} names {p_names:?}{}; {BLOCK} requires {}, composed {}; \
                             {PLATFORM} requires {}; used {users:?}",
                            if s_binds { ", bound to m.blk" } else { "" },
                            if p_binds { ", bound to m.soc" } else { "" },
                            if blk_requires { "output" } else { "nothing" },
                            if composed { "by m.soc" } else { "by nothing" },
                            if soc_requires { "output" } else { "nothing" },
                        ),
                        description: Description {
                            blocks: vec![
                                Block {
                                    name: "m.out".to_string(),
                                    offers: names(&[
                                        "(observable-output true)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    ..Block::default()
                                },
                                block,
                            ],
                            platforms: vec![platform],
                            services: vec![service, policy],
                            tasks: vec![Task {
                                side: task_side,
                                ..Task::default()
                            }],
                            own,
                            platform: PLATFORM.to_string(),
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// What the namer of the interrupts' named-hardware universe requires: nothing, an alarm it programs itself, an
/// absolute deadline through an exclusive `deadline-over-delay` instance, the source, or the source beside either.
const NAMER_REQUIRES: [&[&str]; 6] = [
    &[],
    &["relative-delay"],
    &["absolute-deadline"],
    &["interrupt-source"],
    &["relative-delay", "interrupt-source"],
    &["absolute-deadline", "interrupt-source"],
];

/// **Hardware a service or a policy names, in an interrupt** (§2.6, §2.7; round 17, R17-D1): the timer `m.tmr` and the
/// source `m.src`, both composed by the platform `m.soc`; one namer, `a.x` or `z.x`, its text below the hardware's or
/// above it, a service or a policy, requiring one choice of [`NAMER_REQUIRES`] and naming one choice of
/// [`HardwareNamed`] — `m.tmr` for the block, `m.src` for the platform's place, so that it names the timer or the source
/// by `uses` or by `needs`; and used by the task `t` or by the system's own level alone. So the named block is a
/// provider bound to its namer directly, for a channel or for its interrupt, or through the instance it programs, or a
/// provider of nothing, a node as named alone since round 18 (R18-D1); and the edge from it, beside a binding's reason
/// or not, joins a namer running in the named block's own interrupt.
fn named_hardware_in_interrupts() -> Vec<Case> {
    let mut cases = Vec::new();
    for namer in ["a.x", "z.x"] {
        for requires in NAMER_REQUIRES {
            for named in HardwareNamed::ALL {
                for policy in [false, true] {
                    for by_task in [false, true] {
                        let mut side = Side {
                            needs: names(requires),
                            ..Side::default()
                        };
                        named.name(&mut side, "m.tmr", "m.src");
                        let users = Side {
                            uses: names(&[namer]),
                            ..Side::default()
                        };
                        let (task_side, own) = if by_task {
                            (users, Side::default())
                        } else {
                            (Side::default(), users)
                        };
                        cases.push(Case {
                            label: format!(
                                "{} {namer} needs {requires:?}, names {}; used by {}",
                                if policy { "policy" } else { "service" },
                                match named {
                                    HardwareNamed::Nothing => "nothing",
                                    HardwareNamed::BlockByUses => "m.tmr by uses",
                                    HardwareNamed::BlockByNeeds => "m.tmr by needs",
                                    HardwareNamed::PlatformByUses => "m.src by uses",
                                    HardwareNamed::PlatformByNeeds => "m.src by needs",
                                },
                                if by_task {
                                    "t"
                                } else {
                                    "the system's own level"
                                }
                            ),
                            description: Description {
                                blocks: vec![
                                    timer_block("m.tmr", 4, true),
                                    source_block("m.src", Separation::Stated(20), ""),
                                ],
                                platforms: vec![Block {
                                    name: "m.soc".to_string(),
                                    offers: names(&[
                                        "(core-count 1 tick)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    side: Side {
                                        uses: names(&["m.tmr", "m.src"]),
                                        ..Side::default()
                                    },
                                    ..Block::default()
                                }],
                                services: vec![Service {
                                    name: namer.to_string(),
                                    policy,
                                    side,
                                }],
                                tasks: vec![Task {
                                    side: task_side,
                                    ..Task::default()
                                }],
                                own,
                                platform: "m.soc".to_string(),
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What the unbound-hardware universe names: the block `m.blk` composed by the platform, `m.blk` composed by nothing,
/// or the platform `m.soc` itself, composing `m.blk`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unbound {
    Composed,
    Alone,
    Platform,
}

impl Unbound {
    const ALL: [Self; 3] = [Self::Composed, Self::Alone, Self::Platform];

    /// The block or platform named.
    fn name(self) -> &'static str {
        match self {
            Self::Composed | Self::Alone => "m.blk",
            Self::Platform => "m.soc",
        }
    }

    /// The fact it offers that an unrelated requester can be bound to it for.
    fn offer(self) -> &'static str {
        match self {
            Self::Composed | Self::Alone => "transfer-engine",
            Self::Platform => "core-count",
        }
    }
}

/// The namers of the unbound-hardware universe, each naming the hardware or not: the task `t`, the service `a.s`, the
/// policy `a.p`, the source's handler `a.h` and the system's own level.
const UNBOUND_NAMERS: [&str; 5] = ["t", "a.s", "a.p", "a.h", "sys"];

/// **Named hardware bound for nothing** (§2, §2.6, §2.7, §2.10; model §8 rule 12; round 18, R18-D1: "hardware a
/// service, a policy or a task names, a call, and nothing binds places no code: no context, no step, nothing owed — and
/// placed when an unrelated binding makes it a node"; R18-D3): the block `m.blk`, offering `transfer-engine` and
/// requiring nothing, composed by the platform `m.soc` or by nothing, or `m.soc` itself, offering `core-count`,
/// composing the source `m.src` always and `m.blk` then; named, by `uses` or by `needs`, by every subset of
/// [`UNBOUND_NAMERS`] — the task `t`, the service `a.s`, the policy `a.p`, the source's handler `a.h`, bound to `m.src`
/// for `interrupt-source`, and the system's own level; the three used by `t` or by the system's own level alone; and
/// beside it nothing, or the service `w`, used by the system's own level alone and naming nothing, bound to it for its
/// offer. So hardware a served task, service or policy names is a node bound for nothing and bound to by nothing, in a
/// job, in its handler's interrupt or in neither, its edge to each service or policy naming it `named` alone, ordering
/// the hardware's step before a namer whose text is less; the system's own level naming it makes none, nor does a
/// platform composing a block nothing else names (composition is no call, §2); and a binding elsewhere, `w`'s, changes
/// nothing the name gives — the pair held line by line in its test. A block no platform composes and no task nor the
/// system's own level names is no hardware (model §8 rule 7): a served service or policy naming it is the search's
/// conflict, `infeasible-configuration`, and `w` bound to it alone an assignment's, which leaves `missing-fact`.
fn unbound_named_hardware() -> Vec<Case> {
    let mut cases = Vec::new();
    for target in Unbound::ALL {
        for naming in 0..1u32 << UNBOUND_NAMERS.len() {
            for by_needs in [false, true] {
                for used_by_task in [false, true] {
                    for bound_elsewhere in [false, true] {
                        let names_it = |namer: &str| {
                            let at = UNBOUND_NAMERS
                                .iter()
                                .position(|n| *n == namer)
                                .expect("a namer");
                            naming >> at & 1 == 1
                        };
                        let side_of_namer = |namer: &str, mut side: Side| {
                            if names_it(namer) {
                                side = named(side, vec![target.name().to_string()], by_needs);
                            }
                            side
                        };
                        let namer = |name: &str, policy: bool, needs: &[&str]| Service {
                            name: name.to_string(),
                            policy,
                            side: side_of_namer(
                                name,
                                Side {
                                    needs: names(needs),
                                    ..Side::default()
                                },
                            ),
                        };
                        let mut services = vec![
                            namer("a.s", false, &[]),
                            namer("a.p", true, &[]),
                            namer("a.h", false, &["interrupt-source"]),
                        ];
                        let namers = names(&["a.s", "a.p", "a.h"]);
                        let (task_uses, mut own_uses) = if used_by_task {
                            (namers, Vec::new())
                        } else {
                            (Vec::new(), namers)
                        };
                        if bound_elsewhere {
                            services.push(Service {
                                name: "w".to_string(),
                                policy: false,
                                side: Side {
                                    needs: names(&[target.offer()]),
                                    ..Side::default()
                                },
                            });
                            own_uses.push("w".to_string());
                        }
                        let task_side = side_of_namer(
                            "t",
                            Side {
                                uses: task_uses,
                                ..Side::default()
                            },
                        );
                        let own = side_of_namer(
                            "sys",
                            Side {
                                uses: own_uses,
                                ..Side::default()
                            },
                        );
                        let mut composes = names(&["m.src"]);
                        if target != Unbound::Alone {
                            composes.push("m.blk".to_string());
                        }
                        let namers: Vec<&str> = UNBOUND_NAMERS
                            .iter()
                            .copied()
                            .filter(|n| names_it(n))
                            .collect();
                        cases.push(Case {
                            label: format!(
                                "{target:?} {} named by {namers:?} by {}; a.s, a.p and a.h used by {}; {}",
                                target.name(),
                                if by_needs { "needs" } else { "uses" },
                                if used_by_task {
                                    "t"
                                } else {
                                    "the system's own level"
                                },
                                if bound_elsewhere {
                                    "w bound to it"
                                } else {
                                    "nothing bound to it"
                                }
                            ),
                            description: Description {
                                blocks: vec![
                                    source_block("m.src", Separation::Stated(20), ""),
                                    Block {
                                        name: "m.blk".to_string(),
                                        offers: names(&[
                                            "(transfer-engine true)",
                                            "(reachable-at-privilege machine)",
                                        ]),
                                        ..Block::default()
                                    },
                                ],
                                platforms: vec![Block {
                                    name: "m.soc".to_string(),
                                    offers: names(&[
                                        "(core-count 1 tick)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    side: Side {
                                        uses: composes,
                                        ..Side::default()
                                    },
                                    ..Block::default()
                                }],
                                services,
                                tasks: vec![Task {
                                    side: task_side,
                                    ..Task::default()
                                }],
                                own,
                                platform: "m.soc".to_string(),
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// One task's clauses in the task-bullet universe: its release, deadline and jitter as written, whether it binds
/// `interrupt-source` itself, and a clause it does not write.
#[derive(Debug, Clone, Copy)]
struct Clauses {
    word: &'static str,
    release: Release,
    deadline: Dur,
    jitter: Option<Dur>,
    source: bool,
    omitted: Option<&'static str>,
}

/// The clauses a task of the task-bullet universe writes, what §1's bullet over tasks held until `M1.42`: well-formed;
/// each clause model §4 rule 8 refuses alone — a period, a minimum separation, a deadline and a jitter that is not a
/// time, a period and a minimum separation of zero; a periodic task bound itself to a source, which the bullet still
/// holds (`M1.43`); two at once, a clause rule 8 refuses beside another or beside that binding; a zero deadline, which
/// rule 8 does not refuse, being a time; and neither release nor deadline stated, which the language refuses ("none
/// stated, which it refuses already"). Every unit the language accepts that is no time is written in some clause: `Hz`,
/// `kHz`, `MHz`, `GHz`, `bit`, `byte`, `KiB`, `MiB` and `tick`.
const BULLET_CLAUSES: [Clauses; 16] = {
    const WELL: Clauses = Clauses {
        word: "well-formed",
        release: Release::Period(Dur::Ms(20)),
        deadline: Dur::Ms(20),
        jitter: None,
        source: false,
        omitted: None,
    };
    [
        WELL,
        Clauses {
            word: "period in Hz",
            release: Release::Period(Dur::Spelled("50 Hz")),
            ..WELL
        },
        Clauses {
            word: "min-separation in KiB",
            release: Release::MinSeparation(Dur::Spelled("1 KiB")),
            ..WELL
        },
        Clauses {
            word: "deadline in bit",
            deadline: Dur::Spelled("8 bit"),
            ..WELL
        },
        Clauses {
            word: "jitter in tick",
            jitter: Some(Dur::Spelled("3 tick")),
            ..WELL
        },
        Clauses {
            word: "zero period",
            release: Release::Period(Dur::Spelled("0 ms")),
            deadline: Dur::Spelled("0 ms"),
            ..WELL
        },
        Clauses {
            word: "zero min-separation",
            release: Release::MinSeparation(Dur::Spelled("0 s")),
            deadline: Dur::Spelled("0 s"),
            ..WELL
        },
        Clauses {
            word: "periodic bound to its source",
            source: true,
            ..WELL
        },
        Clauses {
            word: "zero period, deadline in GHz",
            release: Release::Period(Dur::Spelled("0 ms")),
            deadline: Dur::Spelled("1 GHz"),
            ..WELL
        },
        Clauses {
            word: "periodic bound to its source, jitter in MiB",
            jitter: Some(Dur::Spelled("5 MiB")),
            source: true,
            ..WELL
        },
        Clauses {
            word: "period in kHz, bound to its source",
            release: Release::Period(Dur::Spelled("2 kHz")),
            source: true,
            ..WELL
        },
        Clauses {
            word: "zero min-separation, jitter in MHz",
            release: Release::MinSeparation(Dur::Spelled("0 ms")),
            deadline: Dur::Spelled("0 ms"),
            jitter: Some(Dur::Spelled("1 MHz")),
            ..WELL
        },
        Clauses {
            word: "zero deadline",
            deadline: Dur::Spelled("0 ms"),
            ..WELL
        },
        Clauses {
            word: "deadline in Hz, jitter in byte",
            deadline: Dur::Spelled("4 Hz"),
            jitter: Some(Dur::Spelled("2 byte")),
            ..WELL
        },
        Clauses {
            word: "no release",
            omitted: Some("period"),
            ..WELL
        },
        Clauses {
            word: "no deadline",
            omitted: Some("deadline"),
            ..WELL
        },
    ]
};

/// What else the task-bullet universe's system meets, of §1's refusals after the bullet over tasks: nothing; the
/// naming-system refusal, its own level naming the system `other`; the hardware-naming refusal, its platform `soc`
/// naming the service `s`; or a duration past the arithmetic, the obliged bound of the service `dl` its own level uses.
const BULLET_ALSO: [&str; 4] = [
    "nothing",
    "names a system",
    "hardware names a service",
    "a bound past",
];

/// **A task's durations, and what §1's bullet over tasks keeps** (model §4 rule 8; `M1.42`, `RTP-H14`; round 20,
/// R20-R7: "§1's bullet over tasks is read only for its periodic-source clause, and a unit past `s`, `ms`, `us` and
/// `ns` panics"): tasks `t` and `u`, either first in runtime order, each writing one choice of [`BULLET_CLAUSES`], over
/// the source `dev.s`, which a task bound to a source binds; and one choice of [`BULLET_ALSO`]. So the language rejects
/// every duration rule 8 refuses, in either task and either order, whatever else the system writes, before the search
/// and every refusal of §1; a unit the language accepts is read as no time and never panics; a task stating no release
/// or no deadline is the language's to refuse; and what the bullet keeps, a periodic task bound to its source, is taken
/// "task by task in runtime order", named "the task and its `period`", before the first list's later bullets and the
/// second list.
fn task_bullet() -> Vec<Case> {
    let mut cases = Vec::new();
    for t_clauses in BULLET_CLAUSES {
        for u_clauses in BULLET_CLAUSES {
            for u_first in [false, true] {
                for also in BULLET_ALSO {
                    let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
                    let task = |name: &str, priority: u32, clauses: Clauses| Task {
                        name: name.to_string(),
                        release: clauses.release,
                        deadline: Some(clauses.deadline),
                        jitter: clauses.jitter,
                        priority,
                        omitted: clauses.omitted,
                        side: Side {
                            needs: if clauses.source {
                                names(&[INTERRUPT_FACT])
                            } else {
                                Vec::new()
                            },
                            ..Side::default()
                        },
                        ..Task::default()
                    };
                    let mut composes = names(&["dev.s", "timer.d"]);
                    let mut own = Side::default();
                    match also {
                        "names a system" => own.uses.push("other".to_string()),
                        "hardware names a service" => composes.push("s".to_string()),
                        "a bound past" => own.uses.push("dl".to_string()),
                        _ => {}
                    }
                    let mut dl = service("dl", Counter::Deadline, false, Vec::new());
                    dl.side
                        .at_most
                        .push(("delivery-bound".to_string(), Dur::Spelled(PAST)));
                    cases.push(Case {
                        label: format!(
                            "t:{} u:{} first={} also:{also}",
                            t_clauses.word,
                            u_clauses.word,
                            if u_first { "u" } else { "t" }
                        ),
                        description: Description {
                            blocks: vec![
                                source_block("dev.s", Separation::Stated(20), ""),
                                timer_block("timer.d", 2, true),
                            ],
                            platforms: vec![Block {
                                name: "soc".to_string(),
                                offers: names(&[
                                    "(core-count 1 tick)",
                                    "(reachable-at-privilege machine)",
                                ]),
                                side: Side {
                                    uses: composes,
                                    ..Side::default()
                                },
                                ..Block::default()
                            }],
                            services: vec![
                                Service {
                                    name: "s".to_string(),
                                    policy: false,
                                    side: Side::default(),
                                },
                                dl,
                            ],
                            tasks: vec![
                                task("t", t_priority, t_clauses),
                                task("u", u_priority, u_clauses),
                            ],
                            own,
                            others: vec![System {
                                name: "other".to_string(),
                                tasks: vec![Task {
                                    name: "o".to_string(),
                                    ..Task::default()
                                }],
                                own: Side::default(),
                                platform: "soc".to_string(),
                            }],
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// What a served namer of the outside-hardware universe names: nothing; `x` by `uses`; `x.y` by `needs`; `x!` by `uses`
/// and `x` by `needs`, written against their texts' order; or `p`, the platform of the system `other`, by `uses`.
const N_NAMING: [Naming; 5] = [
    &[],
    &[(false, "x")],
    &[(true, "x.y")],
    &[(false, "x!"), (true, "x")],
    &[(false, "p")],
];
/// What `n.s` names: nothing; `x.y` by `uses`; or `q`, the block `p` composes, by `needs`.
const NS_NAMING: [Naming; 3] = [&[], &[(false, "x.y")], &[(true, "q")]];
/// What the policy `n!` names: nothing; `x!` by `uses`; or `p2`, a platform no system uses, by `needs`.
const NBANG_NAMING: [Naming; 3] = [&[], &[(false, "x!")], &[(true, "p2")]];

/// Who serves the namers of the outside-hardware universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutsideServed {
    /// Task `t` uses `n` and `n.s`; the system's own level uses `n!`.
    TaskAndOwn,
    /// The system's own level uses `n`; `t` uses the service `m`, which uses `n.s`; nothing uses `n!`.
    OwnAndThrough,
    /// `t` uses `n!`; nothing uses `n` nor `n.s`.
    PolicyAlone,
}

/// **A block or a platform the system serves outside its hardware** (model §8 rule 7; round 20, R20-D3; `M1.49`): the
/// system `sys`, on the platform `soc` composing the timer `d`; the blocks `x`, `x.y`, `x!` and `q`, which no platform
/// of `sys` composes, `q` composed by `p`, the platform of the system `other`, and the platform `p2`, which no system
/// uses; the services `n` and `n.s` and the policy `n!`, naming one choice of [`N_NAMING`], [`NS_NAMING`] and
/// [`NBANG_NAMING`], served as one choice of [`OutsideServed`]; `x`, `x.y` or `x!` made hardware by another name —
/// none, `t`'s, the system's own level's `needs`, or `soc`'s `needs` — "beside hardware a task names, which rule 7
/// makes hardware"; and `t` meeting another refusal or not: a zero period, which the language rejects (model §4 rule 8;
/// `M1.42`), or, of §1, none, `t` naming the system `other`, an alarm it programs itself, a deadline from another
/// event, a jitter past the arithmetic, or `x` naming the service `m`. Beside them, a system declaring no task whose
/// own level uses `n` naming `x` or nothing, programming an alarm or not. So the language's refusal comes before all of
/// it, whatever is served and named; a served name outside the hardware is the search's conflict whatever else of §1
/// `t` meets, no record reaching the checker or §1; a block a task names, the system's own names or its hardware's
/// `needs` bring in is lowered, a node, as `M4.1.18` makes it; and where no served name lies outside, each other
/// refusal of §1 is given as it is alone.
fn outside_hardware_universe() -> Vec<Case> {
    const HARDWARE_BY: [&str; 4] = ["nothing", "t names x", "sys needs x.y", "soc needs x!"];
    const ALSO: [&str; 7] = [
        "nothing",
        "t's zero period",
        "t names other",
        "t programs an alarm",
        "t's deadline from another event",
        "t's jitter past the arithmetic",
        "x names m",
    ];
    let words = |naming: Naming| -> String {
        let named: Vec<String> = naming
            .iter()
            .map(|(by_needs, name)| format!("{}:{name}", if *by_needs { "needs" } else { "uses" }))
            .collect();
        format!("[{}]", named.join(","))
    };
    let plain = |name: &str, offer: &str| Block {
        name: name.to_string(),
        offers: names(&[offer, "(reachable-at-privilege machine)"]),
        ..Block::default()
    };
    let platform = |name: &str, composes: &[&str]| Block {
        name: name.to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            uses: names(composes),
            ..Side::default()
        },
        ..Block::default()
    };
    let namer = |name: &str, policy: bool, named: Naming| Service {
        name: name.to_string(),
        policy,
        side: naming(Side::default(), named),
    };
    let description = |n: Naming,
                       n_s: Naming,
                       n_bang: Naming,
                       tasks: Vec<Task>,
                       own: Side,
                       (soc_needs, x_names_m): (bool, bool)| {
        let mut x = plain("x", "(debug-port true)");
        if x_names_m {
            x.side.uses.push("m".to_string());
        }
        let mut soc = platform("soc", &["d"]);
        if soc_needs {
            soc.side.needs.push("x!".to_string());
        }
        Description {
            blocks: vec![
                timer_block("d", 4, false),
                x,
                plain("x.y", "(debug-port true)"),
                plain("x!", "(transfer-engine true)"),
                plain("q", "(debug-port true)"),
            ],
            platforms: vec![soc, platform("p", &["q"]), platform("p2", &[])],
            services: vec![
                namer("n", false, n),
                namer("n.s", false, n_s),
                namer("n!", true, n_bang),
                Service {
                    name: "m".to_string(),
                    policy: false,
                    side: Side::default(),
                },
            ],
            tasks,
            own,
            others: vec![System {
                name: "other".to_string(),
                tasks: vec![Task {
                    name: "o".to_string(),
                    ..Task::default()
                }],
                own: Side::default(),
                platform: "p".to_string(),
            }],
            ..Description::default()
        }
    };
    let mut cases = Vec::new();
    for n in N_NAMING {
        for n_s in NS_NAMING {
            for n_bang in NBANG_NAMING {
                for served in [
                    OutsideServed::TaskAndOwn,
                    OutsideServed::OwnAndThrough,
                    OutsideServed::PolicyAlone,
                ] {
                    for hardware_by in HARDWARE_BY {
                        for also in ALSO {
                            let mut t = Task::default();
                            let mut own = Side::default();
                            let mut services_m = Vec::new();
                            match served {
                                OutsideServed::TaskAndOwn => {
                                    t.side.uses.extend(names(&["n", "n.s"]));
                                    own.uses.push("n!".to_string());
                                }
                                OutsideServed::OwnAndThrough => {
                                    own.uses.push("n".to_string());
                                    t.side.uses.push("m".to_string());
                                    services_m.push("n.s".to_string());
                                }
                                OutsideServed::PolicyAlone => t.side.uses.push("n!".to_string()),
                            }
                            match hardware_by {
                                "t names x" => t.side.uses.push("x".to_string()),
                                "sys needs x.y" => own.needs.push("x.y".to_string()),
                                _ => {}
                            }
                            match also {
                                "t's zero period" => {
                                    t.release = Release::Period(Dur::Spelled("0 ms"));
                                    t.deadline = Some(Dur::Spelled("0 ms"));
                                }
                                "t names other" => t.side.uses.push("other".to_string()),
                                "t programs an alarm" => {
                                    t.side.needs.push("relative-delay".to_string())
                                }
                                "t's deadline from another event" => {
                                    t.deadline_from = Some("activation")
                                }
                                "t's jitter past the arithmetic" => {
                                    t.jitter = Some(Dur::Spelled(PAST))
                                }
                                _ => {}
                            }
                            let mut d = description(
                                n,
                                n_s,
                                n_bang,
                                vec![t],
                                own,
                                (hardware_by == "soc needs x!", also == "x names m"),
                            );
                            d.services[3].side.uses = services_m;
                            cases.push(Case {
                                label: format!(
                                    "n names {} n.s names {} n! names {} served {served:?}; hardware by {hardware_by}; \
                                     also {also}",
                                    words(n),
                                    words(n_s),
                                    words(n_bang)
                                ),
                                description: d,
                            });
                        }
                    }
                }
            }
        }
    }
    for n in [N_NAMING[0], N_NAMING[1]] {
        for alarm in [false, true] {
            let mut own = Side {
                uses: names(&["n"]),
                ..Side::default()
            };
            if alarm {
                own.needs.push("relative-delay".to_string());
            }
            cases.push(Case {
                label: format!(
                    "no task; the system's own level uses n, naming {}{}",
                    words(n),
                    if alarm { ", and programs an alarm" } else { "" }
                ),
                description: description(n, &[], &[], Vec::new(), own, (false, false)),
            });
        }
    }
    cases
}

/// The spellings of a rank in the ranks universe, each a value the language reads as an `integer` or a `hexadecimal`
/// (`docs/semantics/grammar.md`): `1`, `2`, `+4`, `007`, `0x0A`, `1_000` and `9223372036854775807`, the largest it reads.
const RANK_SPELLINGS: [&str; 7] = [
    "1",
    "2",
    "+4",
    "007",
    "0x0A",
    "1_000",
    "9223372036854775807",
];

/// **Ranks that do not start at 1 or are not consecutive** (§2.2, §2.5, §1; model §4 rule 5: "the ranks of a system need
/// not be contiguous, because only their order is used"; round 20, R20-R7: "ranks are only ever 1 to 3, so a lowering
/// writing index = rank − 1 passes every universe"): tasks `a`, `b` and `c`, of periods 10, 20 and 30 ms, ranked by
/// every three distinct spellings of [`RANK_SPELLINGS`] in every order; all periodic, `a` sporadic and bound to the
/// source `dev.s` itself, each writing `(deadline-from activation)`, or each programming an alarm at `d`. So each
/// task's runtime index is "the number of tasks of smaller rank", its rank written as a decimal `INT` "with no sign and
/// no leading zero", the release timer's tasks by index, and the bullets over tasks taken in runtime order — the
/// receivers named in it — whatever the ranks' values, their spellings and their texts' order.
fn ranks() -> Vec<Case> {
    const KINDS: [&str; 4] = [
        "periodic",
        "a sporadic",
        "deadline from activation",
        "alarms",
    ];
    let mut cases = Vec::new();
    for (i, a) in RANK_SPELLINGS.iter().enumerate() {
        for (j, b) in RANK_SPELLINGS.iter().enumerate() {
            for (k, c) in RANK_SPELLINGS.iter().enumerate() {
                if i == j || j == k || i == k {
                    continue;
                }
                for kind in KINDS {
                    let task = |name: &str, ms: u64, rank: &'static str| {
                        let mut task = Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(ms)),
                            spelled_priority: Some(rank),
                            ..Task::default()
                        };
                        match kind {
                            "deadline from activation" => task.deadline_from = Some("activation"),
                            "alarms" => task.side.needs.push("relative-delay".to_string()),
                            _ => {}
                        }
                        task
                    };
                    let mut tasks = vec![task("a", 10, a), task("b", 20, b), task("c", 30, c)];
                    if kind == "a sporadic" {
                        tasks[0].release = Release::MinSeparation(Dur::Ms(10));
                        tasks[0].side.needs.push(INTERRUPT_FACT.to_string());
                    }
                    cases.push(Case {
                        label: format!("a:{a} b:{b} c:{c} {kind}"),
                        description: Description {
                            blocks: vec![
                                timer_block("d", 3, false),
                                source_block("dev.s", Separation::Stated(20), ""),
                            ],
                            tasks,
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// What the service `log` of the one-service universe names: nothing; `uart.a`, of `sa`'s hardware, by `uses` or by
/// `needs`; `pa`, `sa`'s platform; `uart.b`, of `sb`'s hardware; or `pb`, `sb`'s platform, by `needs`.
const LOG_NAMING: [Naming; 6] = [
    &[],
    &[(false, "uart.a")],
    &[(true, "uart.a")],
    &[(false, "pa")],
    &[(false, "uart.b")],
    &[(true, "pb")],
];

/// Who uses `log`: `sa`'s task, `sb`'s, both, `sa`'s own level or `sb`'s.
const LOG_USERS: [&str; 5] = ["ta", "tb", "both", "sa", "sb"];

/// **One service two systems serve** (model §8 rule 7, `M1.49`: "It is each serving system's, so a system the name
/// leaves inside its hardware is not refused for it"): the systems `sa` on `pa`, composing `uart.a`, and `sb` on `pb`,
/// composing `uart.b`, with one service `log` naming one choice of [`LOG_NAMING`], used as one choice of [`LOG_USERS`];
/// each description lowered for each system. So one name is inside one system's hardware and outside the other's: the
/// second is the search's to refuse where it serves `log`, and the first lowers, the name a node of its plan; a system
/// that does not serve `log` is refused for nothing it names.
fn one_service_two_systems() -> Vec<Case> {
    let mut cases = Vec::new();
    for log in LOG_NAMING {
        for users in LOG_USERS {
            let using = |used: bool| Side {
                uses: if used { names(&["log"]) } else { Vec::new() },
                ..Side::default()
            };
            let system = |name: &str, task: &str, platform: &str| System {
                name: name.to_string(),
                tasks: vec![Task {
                    name: task.to_string(),
                    side: using(users == task || users == "both"),
                    ..Task::default()
                }],
                own: using(users == name),
                platform: platform.to_string(),
            };
            let (sa, sb) = (system("sa", "ta", "pa"), system("sb", "tb", "pb"));
            let uart = |name: &str| Block {
                name: name.to_string(),
                offers: names(&[
                    "(observable-output true)",
                    "(reachable-at-privilege machine)",
                ]),
                ..Block::default()
            };
            let platform = |name: &str, composes: &str| Block {
                name: name.to_string(),
                offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
                side: Side {
                    uses: names(&[composes]),
                    ..Side::default()
                },
                ..Block::default()
            };
            let named: Vec<String> = log
                .iter()
                .map(|(by_needs, name)| {
                    format!("{}:{name}", if *by_needs { "needs" } else { "uses" })
                })
                .collect();
            let lowering = |lowered: &System, beside: &System, others_first: bool| Case {
                label: format!(
                    "log names [{}] used by {users}: lowering {}",
                    named.join(","),
                    lowered.name
                ),
                description: Description {
                    system: lowered.name.clone(),
                    blocks: vec![uart("uart.a"), uart("uart.b")],
                    platforms: vec![platform("pa", "uart.a"), platform("pb", "uart.b")],
                    services: vec![Service {
                        name: "log".to_string(),
                        policy: false,
                        side: naming(Side::default(), log),
                    }],
                    tasks: lowered.tasks.clone(),
                    own: lowered.own.clone(),
                    platform: lowered.platform.clone(),
                    others: vec![beside.clone()],
                    others_first,
                },
            };
            cases.push(lowering(&sa, &sb, false));
            cases.push(lowering(&sb, &sa, true));
        }
    }
    cases
}

/// The tasks' names of the one-name universe, in declared order, each task ranked by its place: two tasks of one name;
/// three; `t` and `t!` each carried twice; `t.x` and `t` each carried twice; and, after round 24 (R24-R2), `t` carried
/// three times and `u` twice, interleaved — each beside its twin, the same tasks with every name distinct. These are
/// the shapes §1's refusal was held over while the lowering gave it, for its message's order; the language's refusal
/// has no such order to hold, and they stay as what it must take alike, whatever the number of tasks or of names, with
/// each twin, `t` beside `t!` or `t.x` beside `t` among them, accepted: one name prefixing another is no name declared
/// twice.
const ONE_NAME: [[&[&str]; 2]; 5] = [
    [&["t", "t"], &["t", "u"]],
    [&["t", "t", "t"], &["t", "u", "v"]],
    [&["t", "t!", "t!", "t"], &["t", "t!", "u!", "u"]],
    [&["t.x", "t", "t", "t.x"], &["t.x", "t", "u", "u.x"]],
    [&["t", "u", "t", "u", "t"], &["t", "u", "v", "w", "x"]],
];

/// Which tasks of the one-name universe use `s`, a service taking a channel of the timer: none; the first; the next
/// declared after it carrying its name — its twin's task at that place — alone, round 23's "with only the second
/// binding"; or both.
const ONE_NAME_BINDERS: [&str; 4] = ["none", "first", "second", "both"];

/// What else a system of the one-name universe writes: nothing; its last task's period zero, which since `M1.42` the
/// language rejects as it does two tasks of one name (model §4 rule 8); and, each a refusal that comes after the
/// language's check accepts a description, in §1's order: its last task naming the
/// system itself (the naming-system bullet); its platform naming `s` (hardware naming what is not hardware); its last
/// task's jitter past the arithmetic; its last task's deadline from activation; its first task, the next carrying its
/// name, or both, programming an alarm itself (a receiver of interrupt code). And, no refusal of §1's since `M1.49` but
/// the search's, before the lowering reads any: its last task using `w`, which names `out.b`, a block no platform
/// composes (model §8 rule 7).
const ONE_NAME_LATER: [&str; 10] = [
    "none",
    "zero",
    "system",
    "operand",
    "past",
    "activation",
    "alarm",
    "alarm2",
    "alarms",
    "outside",
];

/// **A system declaring two tasks of one name** (reference §7 rule 6, §2.2; round 23, R23-D1, R23-R7: "two tasks of one
/// name merged into one plan, against reference §7 rule 6's "never merged and never resolved by order" and §2.2's
/// "stable logical identity""): each name pattern of [`ONE_NAME`] and its twin, the tasks named by each of
/// [`ONE_NAME_BINDERS`] using a service, beside each of [`ONE_NAME_LATER`]. So the language's refusal
/// ([`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]; `M1.50`, `RTP-H23`) is held on every such system, whatever else it
/// writes, each twin the language accepts lowered or refused by the later refusal alone, a twin writing a period of
/// zero the language's to reject too (model §4 rule 8; `M1.42`), and a twin a served service leaves naming a block
/// outside its hardware the search's to refuse, no record of it reaching §1.
fn one_name_two_tasks() -> Vec<Case> {
    let mut cases = Vec::new();
    for [carried, twin] in ONE_NAME {
        let second = (1..carried.len())
            .find(|i| carried[*i] == carried[0])
            .expect("the first name carried twice");
        let last = carried.len() - 1;
        for (duplicated, pattern) in [(true, carried), (false, twin)] {
            for binder in ONE_NAME_BINDERS {
                for later in ONE_NAME_LATER {
                    let mut tasks: Vec<Task> = pattern
                        .iter()
                        .enumerate()
                        .map(|(i, name)| Task {
                            name: (*name).to_string(),
                            release: Release::Period(Dur::Ms(10 * (i as u64 + 1))),
                            priority: u32::try_from(i + 1).expect("small"),
                            ..Task::default()
                        })
                        .collect();
                    let using: &[usize] = match binder {
                        "first" => &[0],
                        "second" => &[second],
                        "both" => &[0, second],
                        _ => &[],
                    };
                    for at in using {
                        tasks[*at].side.uses.push("s".to_string());
                    }
                    let mut blocks = vec![timer_block("timer.d", 4, false)];
                    let mut services = vec![service("s", Counter::Timer, false, Vec::new())];
                    let mut composed = names(&["timer.d"]);
                    match later {
                        "zero" => tasks[last].release = Release::Period(Dur::Ms(0)),
                        "system" => tasks[last].side.uses.push("sys".to_string()),
                        "operand" => composed.push("s".to_string()),
                        "past" => tasks[last].jitter = Some(Dur::Spelled(PAST)),
                        "activation" => tasks[last].deadline_from = Some("activation"),
                        "alarm" => tasks[0].side.needs.push("relative-delay".to_string()),
                        "alarm2" => tasks[second].side.needs.push("relative-delay".to_string()),
                        "alarms" => {
                            for at in [0, second] {
                                tasks[at].side.needs.push("relative-delay".to_string());
                            }
                        }
                        "outside" => {
                            blocks.push(Block {
                                name: "out.b".to_string(),
                                offers: names(&[
                                    "(transfer-engine true)",
                                    "(reachable-at-privilege machine)",
                                ]),
                                ..Block::default()
                            });
                            services.push(Service {
                                name: "w".to_string(),
                                policy: false,
                                side: Side {
                                    uses: names(&["out.b"]),
                                    ..Side::default()
                                },
                            });
                            tasks[last].side.uses.push("w".to_string());
                        }
                        _ => {}
                    }
                    cases.push(Case {
                        label: format!(
                            "{} {} binder:{binder} later:{later}",
                            if duplicated { "one-name" } else { "distinct" },
                            pattern.join(",")
                        ),
                        description: Description {
                            blocks,
                            platforms: vec![Block {
                                name: "soc".to_string(),
                                offers: names(&[
                                    "(core-count 1 tick)",
                                    "(reachable-at-privilege machine)",
                                ]),
                                side: Side {
                                    uses: composed,
                                    ..Side::default()
                                },
                                ..Block::default()
                            }],
                            services,
                            tasks,
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// How a service or a policy of the self-naming universe names itself: not at all, by `uses` or by `needs`.
const SELF_NAMES: [&str; 3] = ["none", "uses", "needs"];

/// What names a service or a policy of the self-naming universe beside itself, by `uses`: nothing, the system's own
/// level, the task `t`, or the tasks `t` and `u`.
const SELF_NAMERS: [&str; 4] = ["nothing", "own", "t", "t u"];

/// One system of the self-naming universe, its label naming each choice: the service `s`'s name of itself, whether it
/// takes a channel of the timer, whether it handles the source and what names it; the policy `p`'s name of itself,
/// whether it takes a channel, what names it, and whether it names `s` too.
#[derive(Debug, Clone, Copy)]
struct SelfNaming {
    s_self: &'static str,
    s_channel: bool,
    s_source: bool,
    s_namers: &'static str,
    p_self: &'static str,
    p_channel: bool,
    p_namers: &'static str,
    p_names_s: bool,
}

impl SelfNaming {
    fn label(self) -> String {
        let flag = |on: bool, word: &'static str| if on { word } else { "-" };
        format!(
            "s:{} {} {} by:{} p:{} {} by:{} {}",
            self.s_self,
            flag(self.s_channel, "channel"),
            flag(self.s_source, "source"),
            self.s_namers,
            self.p_self,
            flag(self.p_channel, "channel"),
            self.p_namers,
            flag(self.p_names_s, "p-uses-s")
        )
    }
}

/// **A service and a policy naming themselves** (§2.7, §2.9; model §8 rule 12's "but to itself"; §4's `.2`, "nor the
/// node itself by a name of itself"; round 23, R23-R2, R23-R7): the service `s` and the policy `p`, each naming itself
/// by each of [`SELF_NAMES`], taking a channel of the timer or not, `s` handling the source or not, each named by each
/// of [`SELF_NAMERS`], and `p` naming `s` or not — so that on the timer's route, `s`'s own, `p`'s code may call `s` —
/// each beside its twin, which names itself not at all. So a name of itself is held to give no edge, no user on the
/// node's own timer route nor anywhere, and nothing else, the plan its twin's but for the digests, while another node's
/// call on that route stays a user.
fn self_naming() -> Vec<Case> {
    let mut choices = Vec::new();
    for s_self in SELF_NAMES {
        for s_channel in [false, true] {
            for s_source in [false, true] {
                for s_namers in SELF_NAMERS {
                    for p_self in SELF_NAMES {
                        for p_channel in [false, true] {
                            for p_namers in SELF_NAMERS {
                                for p_names_s in [false, true] {
                                    choices.push(SelfNaming {
                                        s_self,
                                        s_channel,
                                        s_source,
                                        s_namers,
                                        p_self,
                                        p_channel,
                                        p_namers,
                                        p_names_s,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    choices.into_iter().map(self_naming_case).collect()
}

fn self_naming_case(choice: SelfNaming) -> Case {
    let mut tasks = vec![
        Task::default(),
        Task {
            name: "u".to_string(),
            release: Release::Period(Dur::Ms(20)),
            priority: 2,
            ..Task::default()
        },
    ];
    let mut own = Side::default();
    let mut services = Vec::new();
    for (name, policy, by, channel, source, namers) in [
        (
            "s",
            false,
            choice.s_self,
            choice.s_channel,
            choice.s_source,
            choice.s_namers,
        ),
        (
            "p",
            true,
            choice.p_self,
            choice.p_channel,
            false,
            choice.p_namers,
        ),
    ] {
        let mut side = Side::default();
        if channel {
            side.needs.push("relative-delay".to_string());
        }
        if source {
            side.needs.push(INTERRUPT_FACT.to_string());
        }
        match by {
            "uses" => side.uses.push(name.to_string()),
            "needs" => side.needs.push(name.to_string()),
            _ => {}
        }
        if policy && choice.p_names_s {
            side.uses.push("s".to_string());
        }
        match namers {
            "own" => own.uses.push(name.to_string()),
            "t" => tasks[0].side.uses.push(name.to_string()),
            "t u" => {
                tasks[0].side.uses.push(name.to_string());
                tasks[1].side.uses.push(name.to_string());
            }
            _ => {}
        }
        services.push(Service {
            name: name.to_string(),
            policy,
            side,
        });
    }
    Case {
        label: choice.label(),
        description: Description {
            blocks: vec![
                timer_block("timer.d", 4, false),
                source_block("src.e", Separation::Stated(20), ""),
            ],
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// What a requester of the `exactly` universe writes on `delivery-bound` beside `(needs absolute-deadline)`: a label,
/// its `at-most` bounds and its `exactly` ones. Bare; `at-most` alone; `exactly` alone; `exactly` beside a looser
/// `at-most` and an equal one; one `exactly` value in two spellings; `exactly` at the arithmetic's edge and past it;
/// and three the side reader refuses — `exactly` beside an `at-most` it contradicts, two `exactly` values, and
/// `exactly` beside an `at-most` past the arithmetic (the adapters record §1, item 5: "the value of an `exactly` bound,
/// which the side reader already refuses beside a bound it contradicts").
const EXACT_WRITINGS: [(&str, &[Dur], &[Dur]); 11] = [
    ("bare", &[], &[]),
    ("at-most 5 ms", &[Dur::Ms(5)], &[]),
    ("exactly 5 ms", &[], &[Dur::Ms(5)]),
    ("exactly 5 ms, at-most 10 ms", &[Dur::Ms(10)], &[Dur::Ms(5)]),
    ("exactly 5 ms, at-most 5 ms", &[Dur::Ms(5)], &[Dur::Ms(5)]),
    (
        "exactly 5 ms and 5000 us",
        &[],
        &[Dur::Ms(5), Dur::Spelled("5000 us")],
    ),
    ("exactly at the edge", &[], &[Dur::Spelled(TIGHT_WITHIN)]),
    ("exactly past", &[], &[Dur::Spelled(PAST)]),
    ("exactly 5 ms, at-most 3 ms", &[Dur::Ms(3)], &[Dur::Ms(5)]),
    ("exactly 5 ms and 3 ms", &[], &[Dur::Ms(5), Dur::Ms(3)]),
    (
        "exactly 5 ms, at-most past",
        &[Dur::Spelled(PAST)],
        &[Dur::Ms(5)],
    ),
];

/// The requesters of the `exactly` universe, each served by an exclusive `deadline-over-delay` instance: the system's
/// own level, the service `dl` and the policy `pol` its task uses, its platform `soc` and the block `blk` it composes.
const EXACT_REQUESTERS: [&str; 5] = ["sys", "dl", "pol", "soc", "blk"];

/// **An obliged fact's bound written `exactly`** (§2.4, §1; round 23, R23-R7): one requester of [`EXACT_REQUESTERS`]
/// writing each of [`EXACT_WRITINGS`], the others nothing. So §2.4's value — "the requirement's value at that exclusive
/// instance, the "ordinary value" of the adapters record's §1, item 5", that item's "the value of an `exactly` bound
/// ...; else the least of its `at-most` bounds" — is held beside `at-most` and bare, and §1 names an `exactly` bound
/// past the arithmetic as it names an `at-most` one.
fn exactly_bounds() -> Vec<Case> {
    let mut cases = Vec::new();
    for requester in EXACT_REQUESTERS {
        for (writing, at_most, exact) in EXACT_WRITINGS {
            let side = |who: &str| -> Side {
                let mut side = Side::default();
                if who == requester {
                    side.needs.push("absolute-deadline".to_string());
                    side.at_most.extend(
                        at_most
                            .iter()
                            .map(|bound| ("delivery-bound".to_string(), *bound)),
                    );
                    side.exactly.extend(
                        exact
                            .iter()
                            .map(|bound| ("delivery-bound".to_string(), *bound)),
                    );
                }
                side
            };
            let mut platform = Block {
                name: "soc".to_string(),
                offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
                side: side("soc"),
                ..Block::default()
            };
            platform.side.uses = names(&["timer.d", "blk"]);
            cases.push(Case {
                label: format!("{requester} writes {writing}"),
                description: Description {
                    blocks: vec![
                        timer_block("timer.d", 2, true),
                        Block {
                            name: "blk".to_string(),
                            offers: names(&[
                                "(transfer-engine true)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: side("blk"),
                            ..Block::default()
                        },
                    ],
                    platforms: vec![platform],
                    services: vec![
                        Service {
                            name: "dl".to_string(),
                            policy: false,
                            side: side("dl"),
                        },
                        Service {
                            name: "pol".to_string(),
                            policy: true,
                            side: side("pol"),
                        },
                    ],
                    tasks: vec![Task {
                        side: Side {
                            uses: names(&["dl", "pol"]),
                            ..Side::default()
                        },
                        ..Task::default()
                    }],
                    own: side("sys"),
                    ..Description::default()
                },
            });
        }
    }
    cases
}

/// What a task of the which-task universe names: the plain block `blk`, the source's block `dev`, the block `hnd` that
/// handles `dev`'s source, or the system's platform `soc`.
const WHICH_NAMED: [&str; 4] = ["blk", "dev", "hnd", "soc"];

/// How a task of the which-task universe names it: by `uses`, by `needs`, or by both, one node named twice.
const WHICH_CLAUSES: [&str; 3] = ["uses", "needs", "both"];

/// Which task of the which-task universe names it, in this order so that each system's twin, the other task naming it,
/// follows it: `t`, `u`, both, or neither.
const WHICH_NAMERS: [&str; 4] = ["t", "u", "both", "neither"];

/// The separations `dev` states in the which-task universe, against `t`'s 20 ms and `u`'s 40 ms when they are sporadic:
/// shorter than both, shorter than `u`'s alone, equal to `u`'s, none, and declared absent.
const WHICH_SEPARATIONS: [Separation; 5] = [
    Separation::Stated(10),
    Separation::Stated(30),
    Separation::Stated(40),
    Separation::Unstated,
    Separation::Absent,
];

/// **Which task names a block** (§2.2; round 24, R24-D1, R24-R1): two tasks, `t` and `u`, both sporadic at 20 ms and
/// 40 ms or both periodic; a source's block `dev` stating each of [`WHICH_SEPARATIONS`], a block `hnd` requiring
/// `interrupt-source`, bound to `dev` — the source's handler, the requester its route names — and a plain block `blk`,
/// each composed by the platform `soc`; `t` bound to `blk` for `transfer-engine` or bound to nothing; one of
/// [`WHICH_NAMED`] named by the tasks of [`WHICH_NAMERS`], by each of [`WHICH_CLAUSES`]. Each system naming it from one
/// task stands beside its twin naming it from the other: before round 24 a task's `(uses …)` dropped the block or
/// platform it names and `unknown` carried no cause, so that two such twins naming `hnd` before a source faster than
/// both tasks wrote one plan ("two systems alike but for which task names a block; two alike but for which task reaches
/// a faster source's block", §3). Now each task's line says which node its job calls — `blk` among them whether or not
/// the task naming it is bound to it — and what leaves its trigger unknown.
fn which_task_names() -> Vec<Case> {
    let mut cases = Vec::new();
    for binds in [false, true] {
        for sporadic in [true, false] {
            for named in WHICH_NAMED {
                for separation in WHICH_SEPARATIONS {
                    for clause in WHICH_CLAUSES {
                        for namer in WHICH_NAMERS {
                            cases.push(which_case(
                                binds, sporadic, named, separation, clause, namer,
                            ));
                        }
                    }
                }
            }
        }
    }
    cases
}

/// One system of the which-task universe, its label naming each choice, which task names the hardware last.
fn which_case(
    binds: bool,
    sporadic: bool,
    named: &str,
    separation: Separation,
    clause: &str,
    namer: &str,
) -> Case {
    let name_it = |side: &mut Side| {
        if clause != "needs" {
            side.uses.push(named.to_string());
        }
        if clause != "uses" {
            side.needs.push(named.to_string());
        }
    };
    let mut t = Side::default();
    let mut u = Side::default();
    if binds {
        t.needs.push("transfer-engine".to_string());
    }
    if matches!(namer, "t" | "both") {
        name_it(&mut t);
    }
    if matches!(namer, "u" | "both") {
        name_it(&mut u);
    }
    let release = |ms: u64| {
        if sporadic {
            Release::MinSeparation(Dur::Ms(ms))
        } else {
            Release::Period(Dur::Ms(ms))
        }
    };
    Case {
        label: format!(
            "{} {named} dev:{separation:?} by:{clause} t-binds-blk:{binds} named-by:{namer}",
            if sporadic { "sporadic" } else { "periodic" }
        ),
        description: Description {
            blocks: vec![
                source_block("dev", separation, ""),
                Block {
                    name: "hnd".to_string(),
                    offers: names(&[
                        "(observable-output true)",
                        "(reachable-at-privilege machine)",
                    ]),
                    side: Side {
                        needs: names(&[INTERRUPT_FACT]),
                        ..Side::default()
                    },
                    ..Block::default()
                },
                Block {
                    name: "blk".to_string(),
                    offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
                    ..Block::default()
                },
            ],
            tasks: vec![
                Task {
                    name: "t".to_string(),
                    release: release(20),
                    priority: 1,
                    side: t,
                    ..Task::default()
                },
                Task {
                    name: "u".to_string(),
                    release: release(40),
                    priority: 2,
                    side: u,
                    ..Task::default()
                },
            ],
            ..Description::default()
        },
    }
}

/// The kinds the declaration `k` of the node-kind universe is declared as.
const K_KINDS: [&str; 4] = ["block", "platform", "service", "policy"];

/// What `k` requires at its own level: nothing, a channel of the timer `timer.d`, or the source `dev`'s interrupt.
const K_REQUIRES: [&str; 3] = ["nothing", "alarm", "source"];

/// What names `k`: the task `t` itself, the service `s` that `t` uses, the policy `p` that `t` uses, or the system's
/// own level.
const K_NAMERS: [&str; 4] = ["t", "s", "p", "sys"];

/// **Each node kind** (§2, §3; round 24, R24-D2): one declaration `k`, a block or a platform `soc` composes, a service
/// or a policy, requiring each of [`K_REQUIRES`], named by each of [`K_NAMERS`] by `uses` or by `needs`; beside it a
/// timer `timer.d` of two channels and a source's block `dev` stating 20 ms, which `soc` composes, and the periodic
/// task `t`. Every kind but `instance`, which the adapters universes reach, is a node here, and the four declarations
/// of one name and one requirement, named alike, write one plan but for `k`'s `(node …)` line — before round 24 they
/// wrote one plan byte for byte ("a block and a service of one name, each a node of its kind", §3) — save where the
/// system's own level alone names `k` requiring nothing: a service or a policy it names is served, a node, and a block
/// or a platform it names is no call and no node (§2).
fn node_kinds() -> Vec<Case> {
    let mut cases = Vec::new();
    for requires in K_REQUIRES {
        for namer in K_NAMERS {
            for by_needs in [false, true] {
                for kind in K_KINDS {
                    let needs = match requires {
                        "alarm" => names(&["relative-delay"]),
                        "source" => names(&[INTERRUPT_FACT]),
                        _ => Vec::new(),
                    };
                    let k_side = Side {
                        needs,
                        ..Side::default()
                    };
                    let name_k = |side: Side| named(side, names(&["k"]), by_needs);
                    let mut t = Side::default();
                    let mut own = Side::default();
                    let mut services = Vec::new();
                    match namer {
                        "t" => t = name_k(t),
                        "sys" => own = name_k(own),
                        _ => {
                            t.uses.push(namer.to_string());
                            services.push(Service {
                                name: namer.to_string(),
                                policy: namer == "p",
                                side: name_k(Side::default()),
                            });
                        }
                    }
                    let k_block = Block {
                        name: "k".to_string(),
                        offers: names(&[
                            "(transfer-engine true)",
                            "(reachable-at-privilege machine)",
                        ]),
                        side: k_side.clone(),
                        ..Block::default()
                    };
                    let mut blocks = vec![
                        timer_block("timer.d", 2, false),
                        source_block("dev", Separation::Stated(20), ""),
                    ];
                    let mut platforms = Vec::new();
                    match kind {
                        "block" => blocks.push(k_block),
                        "platform" => platforms.push(k_block),
                        _ => services.push(Service {
                            name: "k".to_string(),
                            policy: kind == "policy",
                            side: k_side,
                        }),
                    }
                    let mut composed = names(&["timer.d", "dev"]);
                    if matches!(kind, "block" | "platform") {
                        composed.push("k".to_string());
                    }
                    platforms.insert(
                        0,
                        Block {
                            name: "soc".to_string(),
                            offers: names(&[
                                "(core-count 1 tick)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: Side {
                                uses: composed,
                                ..Side::default()
                            },
                            ..Block::default()
                        },
                    );
                    cases.push(Case {
                        label: format!(
                            "requires:{requires} named-by:{namer} by:{} kind:{kind}",
                            if by_needs { "needs" } else { "uses" }
                        ),
                        description: Description {
                            blocks,
                            platforms,
                            services,
                            tasks: vec![Task {
                                release: Release::Period(Dur::Ms(20)),
                                side: t,
                                ..Task::default()
                            }],
                            own,
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// How the sporadic task `t` of the own-causes universe comes to the source `dev.x`: it binds `interrupt-source`
/// itself, it uses `dev.x`'s handler `evt.x`, both, or neither.
const T_REACH: [&str; 4] = ["own", "handler", "both", "none"];

/// How the sporadic task `u` of the own-causes universe comes to `dev.x`: it binds `interrupt-source` itself, it uses
/// `evt.x`, or neither.
const U_REACH: [&str; 3] = ["own", "handler", "none"];

/// The second source of the own-causes universe, `dev.y`, whose handler `evt.y` the task `t` uses: none, or one stating
/// each class of [`SEPARATIONS`], declared after `dev.x` or before it.
const Y_SOURCES: [Option<(Separation, bool)>; 13] = [
    None,
    Some((SEPARATIONS[0], false)),
    Some((SEPARATIONS[1], false)),
    Some((SEPARATIONS[2], false)),
    Some((SEPARATIONS[3], false)),
    Some((SEPARATIONS[4], false)),
    Some((SEPARATIONS[5], false)),
    Some((SEPARATIONS[0], true)),
    Some((SEPARATIONS[1], true)),
    Some((SEPARATIONS[2], true)),
    Some((SEPARATIONS[3], true)),
    Some((SEPARATIONS[4], true)),
    Some((SEPARATIONS[5], true)),
];

/// **The task's own source written apart** (§2.2, §3; round 28, R28-D2, R28-R4, R28-R7): the sporadic tasks `t`, at
/// 20 ms, and `u`, at 40 ms, each coming to the source `dev.x` as [`T_REACH`] and [`U_REACH`] say, `dev.x` stating each
/// class of [`SEPARATIONS`]; and `t` also using `evt.y`, the handler of a second source `dev.y`, as [`Y_SOURCES`] says.
/// Each handler is held to its own source by the power states it requires, `(available-in-state …)`, which only that
/// source offers (model §8 rule 13). A task can write no such qualifier — it says what it requires by `needs` and
/// `uses` alone (`docs/semantics/kinds/os-rt.eadl`), and the language refuses one — so where `dev.y` is declared,
/// which source a task's own `interrupt-source` takes is the search's, read from the record; with `dev.x` alone the
/// construction fixes every binding. So: a task bound itself to a source, released by it alone, beside its twin
/// reaching that source only through `evt.x`, whose cause is `faster`, `unstated` or `absent` where the source leaves
/// the twin unknown; two tasks bound themselves to `dev.x`, one as far apart as it and one released less often; and a
/// task bound itself to a source while the handler it uses reaches another. Since model §8 rule 16 landed (`M4.1.2.3`,
/// R29-R2), a task bound itself to a source the analysis does not admit is bound to another that it admits, or the
/// search refuses the system, so the own causes, `own-faster`, `own-unstated` and `own-absent`, reach no plan: every
/// record binding a task to a source the rule does not admit — the unamended rules' among them, the one that reached an
/// own cause before — is judged instead, refused by the checker and the lowering ([`RULE_16_RECORD`]). Every cause the
/// routes leave is reached here, and every own cause's record refused.
fn own_causes() -> Vec<Case> {
    let mut cases = Vec::new();
    for y in Y_SOURCES {
        for x in SEPARATIONS {
            for u in U_REACH {
                for t in T_REACH {
                    cases.push(own_causes_case(x, t, u, y));
                }
            }
        }
    }
    cases
}

/// One system of the own-causes universe, its label `dev.x:SEPARATION t:REACH u:REACH dev.y:SEPARATION|none
/// first:dev.x|dev.y`.
fn own_causes_case(x: Separation, t: &str, u: &str, y: Option<(Separation, bool)>) -> Case {
    let mut blocks = vec![source_block("dev.x", x, "run idle")];
    if let Some((y, y_first)) = y {
        blocks.insert(usize::from(!y_first), source_block("dev.y", y, "run sleep"));
    }
    let handler = |name: &str, states: &str| {
        let mut evt = service(name, Counter::None, true, Vec::new());
        evt.side
            .other
            .push(format!("(available-in-state {states})"));
        evt
    };
    let mut services = Vec::new();
    if matches!(t, "handler" | "both") || u == "handler" {
        services.push(handler("evt.x", "run idle"));
    }
    if y.is_some() {
        services.push(handler("evt.y", "run sleep"));
    }
    let sporadic = |name: &str, ms: u64, priority: u32, reach: &str, also: Option<&str>| {
        let mut side = Side::default();
        if matches!(reach, "own" | "both") {
            side.needs.push(INTERRUPT_FACT.to_string());
        }
        if matches!(reach, "handler" | "both") {
            side.uses.push("evt.x".to_string());
        }
        side.uses.extend(also.map(str::to_string));
        Task {
            name: name.to_string(),
            release: Release::MinSeparation(Dur::Ms(ms)),
            priority,
            side,
            ..Task::default()
        }
    };
    Case {
        label: format!(
            "dev.x:{x:?} t:{t} u:{u} dev.y:{} first:{}",
            y.map_or_else(|| "none".to_string(), |(y, _)| format!("{y:?}")),
            if y.is_some_and(|(_, y_first)| y_first) {
                "dev.y"
            } else {
                "dev.x"
            }
        ),
        description: Description {
            blocks,
            services,
            tasks: vec![
                sporadic("t", 20, 1, t, y.map(|_| "evt.y")),
                sporadic("u", 40, 2, u, None),
            ],
            ..Description::default()
        },
    }
}

/// The source a plan's `bind` lines — the record's, copied as written (§2.3) — say `task` is bound to itself for
/// `interrupt-source`, where it is.
fn own_source_bound(plan: &str, task: &str) -> Option<String> {
    let opening = format!("(bind (task sys {task}) {INTERRUPT_FACT} ");
    plan.lines().find_map(|line| {
        line.strip_prefix(&opening)
            .and_then(|rest| rest.strip_suffix(')'))
            .map(str::to_string)
    })
}

/// The classes of the own-causes universe a system's construction fixes, from its label and, where `dev.y` is declared,
/// the source each task binding `interrupt-source` itself is bound to, `t_source` and `u_source`, the search's choice
/// read from the record (§2.2, §2.5): each task's trigger by the source it binds itself, else by the routes it
/// reaches — `t` reaching `dev.x` through `evt.x` and `dev.y` through `evt.y`, `u` reaching `dev.x` through `evt.x` —
/// against its own separation; its own source beside another it reaches, and its route naming a requester it reaches;
/// a task binding none beside one bound itself, reaching that one's source or not; two tasks bound themselves to one
/// source; several sources, by their texts unlike their declaration order; and two routes stating one separation tied.
fn own_causes_classes(
    label: &str,
    t_source: Option<&str>,
    u_source: Option<&str>,
) -> BTreeSet<&'static str> {
    let word = |key: &str| {
        label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a parameter")
    };
    let (x, t, u, y) = (word("dev.x:"), word("t:"), word("u:"), word("dev.y:"));
    let y = (y != "none").then_some(y);
    let separation = |source: &str| match source {
        "dev.x" => x,
        "dev.y" => y.expect("`dev.y` declared"),
        other => panic!("no source `{other}` in the own-causes universe"),
    };
    let (t_own, u_own) = (matches!(t, "own" | "both"), u == "own");
    // A plan whose `bind` lines lose a task's own binding, or invent one, is a finding the reading already reports,
    // its `bind` lines against the record's: no class is counted for it, so the counts move with it.
    if t_own != t_source.is_some() || u_own != u_source.is_some() {
        return BTreeSet::new();
    }
    // What each task reaches through the handlers it uses.
    let mut t_reaches: Vec<&str> = Vec::new();
    if matches!(t, "handler" | "both") {
        t_reaches.push("dev.x");
    }
    if y.is_some() {
        t_reaches.push("dev.y");
    }
    let u_reaches: Vec<&str> = if u == "handler" {
        vec!["dev.x"]
    } else {
        Vec::new()
    };
    let mut out = BTreeSet::new();
    for (ms, own, reaches, other_own) in [
        (20, t_source, &t_reaches, u_source),
        (40, u_source, &u_reaches, t_source),
    ] {
        if let Some(own) = own {
            let decided = separation_decides(separation(own), ms, true);
            out.extend(["trigger: its own source", decided.1]);
            let others: Vec<&&str> = reaches.iter().filter(|s| **s != own).collect();
            out.insert(if others.is_empty() {
                "trigger: its own source, no other reaching it"
            } else {
                "trigger: its own source, beside another reaching it"
            });
            if let [other] = others.as_slice() {
                out.insert(match (decided.0, separation_decides(separation(other), ms, false).0) {
                    ("unknown", "every") => OWN_UNKNOWN_BESIDE_EVERY,
                    ("every", "unknown") => {
                        "trigger: its own source releasing it every, beside one that alone would leave it unknown"
                    }
                    ("every", _) => {
                        "trigger: its own source releasing it every, beside another that alone would release it every \
                         from itself"
                    }
                    _ => OWN_UNKNOWN_BESIDE_UNKNOWN,
                });
            }
            // Its route names the handler the task uses: `evt.x` for `dev.x`, `evt.y` for `dev.y`.
            if reaches.contains(&own) {
                out.insert(
                    "trigger: its own source, reached through a requester its route names too",
                );
            }
        } else {
            if let Some(other_own) = other_own {
                out.insert("trigger: binding none, beside a task bound to its own source");
                if reaches.contains(&other_own) {
                    out.insert(
                        "trigger: binding none, reaching the handler of another task's own source",
                    );
                }
            }
            out.insert(match reaches.as_slice() {
                [] => "trigger: `(unknown unreached)`, no source reaching it",
                [one] => separation_decides(separation(one), ms, false).1,
                _ => "trigger: `(unknown (several ITEM …))`, several sources",
            });
            // `(source dev.x)` before `(source dev.y)` by their texts, where `dev.y` is declared first.
            if reaches.len() > 1 && word("first:") == "dev.y" {
                out.insert(
                    "trigger: several sources, by their texts unlike their declaration order",
                );
            }
        }
    }
    if let (Some(t_source), Some(u_source)) = (t_source, u_source) {
        if t_source == u_source {
            out.insert("trigger: two tasks bound themselves to one source");
            // `t` as far apart as the source at 20 ms, `u` released less often.
            if separation(t_source) == "Stated(20)" {
                out.insert(OWN_ONE_RELEASED_LESS_OFTEN);
            }
        }
    }
    // A source's route, where a requester is bound to it: `evt.x`, `evt.y`, or a task bound itself.
    let routed = |source: &str| {
        t_source == Some(source)
            || u_source == Some(source)
            || t_reaches.contains(&source)
            || u_reaches.contains(&source)
    };
    if let Some(y) = y {
        if routed("dev.x") && routed("dev.y") && x.starts_with("Stated(") && x == y {
            out.insert("rank: two stated separations tied");
        }
    }
    out
}

/// The minimum separations of the sporadic task of the arithmetic's-edge universe: 20 ms, [`WIDE`] and
/// [`WIDE_LONGER`], each within the arithmetic, the last two compared with each other past 128 bits.
const EDGE_TASKS: [Dur; 3] = [Dur::Ms(20), Dur::Spelled(WIDE), Dur::Spelled(WIDE_LONGER)];

/// The separations its source states: 20 ms, [`WIDE`], [`WIDE_LONGER`], and [`PAST`], past the arithmetic.
const EDGE_SOURCES: [Separation; 4] = [
    Separation::Stated(20),
    Separation::Spelled(&[WIDE]),
    Separation::Spelled(&[WIDE_LONGER]),
    Separation::Spelled(&[PAST]),
];

/// **A task's own source at the arithmetic's edge** (model §8 rule 16, `M4.1.2.3`; `RTP-H24`: "where the exact
/// arithmetic cannot decide the comparison, as model §7 rule 2 holds"): the sporadic task `t`, bound itself for
/// `interrupt-source`, its minimum separation each of [`EDGE_TASKS`], the source `dev.x` stating each of
/// [`EDGE_SOURCES`]. So rule 16 admits a source as far apart as the task or farther, compared exactly where the two
/// times' cross-multiplication lies past 128 bits, refuses one shorter, so compared too, and refuses one past the
/// arithmetic, which no comparison decides — each refused system's record, the unamended rules' and its one record
/// binding `t`, judged by the checker and lowered. (A task's own separation past the arithmetic is no input here: no source is admitted for it, so no
/// search writes the record the unamended rules gave it.)
fn own_source_at_the_edge() -> Vec<Case> {
    let mut cases = Vec::new();
    for task in EDGE_TASKS {
        for source in EDGE_SOURCES {
            cases.push(Case {
                label: format!("t:{} dev.x:{source:?}", task.spelling()),
                description: Description {
                    blocks: vec![source_block("dev.x", source, "")],
                    tasks: vec![Task {
                        release: Release::MinSeparation(task),
                        side: Side {
                            needs: names(&[INTERRUPT_FACT]),
                            ..Side::default()
                        },
                        ..Task::default()
                    }],
                    ..Description::default()
                },
            });
        }
    }
    cases
}

// =====================================================================================================================
// The reading: docs/decisions/decision_runtime-plan.md, §2 and §3, transcribed rule by rule
// =====================================================================================================================

/// The facts a requester bound to takes one of the provider's channels: those
/// `docs/semantics/vocabulary/vocabulary.eadl` marks `(use consumed)` — a timer route's facts (§2.9: "A channel taken
/// on a consumed fact makes a timer route").
const CONSUMED: [&str; 2] = ["absolute-deadline", "relative-delay"];

/// The interrupt fact a source's requester is bound for (§2.6: "for the interrupt fact where the route is a source").
const INTERRUPT_FACT: &str = "interrupt-source";

/// What the reading takes of an adapter kind, transcribed from `docs/semantics/adapters/adapters.eadl` and held to the
/// shipped catalog by `the_reading_transcribes_the_shipped_vocabulary_and_catalog`.
struct AdapterKind {
    name: &'static str,
    /// The facts it requires `beneath` of the provider it programs, in written order.
    beneath: &'static [&'static str],
    /// The facts it offers `(obliged)`, in offer order (§2.4).
    obliged: &'static [&'static str],
    /// Its obligations, in written order (§2.4).
    obligations: &'static [&'static str],
    /// Its costs, in written order (§2.4).
    costs: &'static [&'static str],
    /// Whether its cost interrupt runs its requester's code (§2.6: "the plan reads `expiry`, a delivery to its
    /// requester, as running it, and `wake`, an epoch advance no requester observes but by reading, as running the
    /// instance's alone").
    runs_requester: bool,
}

const KINDS: [AdapterKind; 2] = [
    AdapterKind {
        name: "deadline-over-delay",
        beneath: &[
            "relative-delay",
            "counter-modulus",
            "tick-rate",
            "observation-coherent",
        ],
        obliged: &["delivery-bound"],
        obligations: &["delivery", "chain"],
        costs: &["expiry"],
        runs_requester: true,
    },
    AdapterKind {
        name: "epoch-extender",
        beneath: &[
            "relative-delay",
            "counter-modulus",
            "tick-rate",
            "observation-coherent",
            "wrap-behavior",
        ],
        obliged: &[],
        obligations: &["update", "coherent"],
        costs: &["wake"],
        runs_requester: false,
    },
];

/// The facts owed for each task to the application (§2.10's first row). §2.2: "Each bound and count among them is owed
/// with `ROADMAP.md` §7.3's record of a bound ... its evidence category a fact of its own" — the execution bound's
/// `execution-category`, the masked runs' `masked-category`, the stack bound's `stack-category`, and, after round 19
/// (R19-D2), "each OS call it may make, with the most one job makes, `n_{i,p}`, and those counts' evidence category",
/// `os-calls-category`.
const TASK_FACTS: [&str; 12] = [
    "entry-point",
    "execution-bound",
    "execution-category",
    "figures-under-preemption",
    "masked-category",
    "masked-runs",
    "masks-only-through-api",
    "os-calls",
    "os-calls-category",
    "stack-bound",
    "stack-category",
    "task-facts",
];

/// The facts owed for each source to the realization (§2.10).
const SOURCE_FACTS: [&str; 5] = [
    "service-cost",
    "acknowledge-point",
    "deferred-work",
    "no-application-code",
    "one-request-per-arrival",
];

/// The facts owed for each timer route to the realization (§2.10).
const TIMER_ROUTE_FACTS: [&str; 2] = ["service-cost", "arrival-bound"];

/// An instance's text: `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)` (§2.4).
fn is_instance(name: &str) -> bool {
    name.starts_with("(adapter ")
}

fn kind_of(instance: &str) -> &'static AdapterKind {
    let kind = instance["(adapter ".len()..]
        .split(' ')
        .next()
        .expect("a kind");
    KINDS
        .iter()
        .find(|k| k.name == kind)
        .unwrap_or_else(|| panic!("the reading knows no adapter kind `{kind}`"))
}

/// What `requester` writes at its own level: a task's `(task SYSTEM NAME)` side, the system's own level's by the system's
/// name, a service's, a policy's, a block's or a platform's.
///
/// ⛔ A task is found by its text, `(task SYSTEM NAME)`, which two tasks of one name would share: read so, they would be
/// one requester, the first's side standing for both — the merge round 23 found in the lowering (R23-D1). The language
/// rejects such a system ([`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]) and [`outcome_of_the_search`] goes no further
/// with one, so nothing reads a task of it by its text; and so that no reading can merge two tasks unnoticed, a text
/// two tasks share fails here.
fn side_of(description: &Description, requester: &str) -> Option<Side> {
    if requester == description.system {
        return Some(description.own.clone());
    }
    let mut tasks = description
        .tasks
        .iter()
        .filter(|t| format!("(task {} {})", description.system, t.name) == requester);
    let task = tasks.next();
    assert!(
        tasks.next().is_none(),
        "two tasks read as one by their text `{requester}`: the language rejects two tasks of one name before anything \
         reads a task by its text"
    );
    task.map(|t| t.side.clone())
        .or_else(|| {
            description
                .services
                .iter()
                .find(|s| s.name == requester)
                .map(|s| s.side.clone())
        })
        .or_else(|| {
            description
                .hardware()
                .into_iter()
                .find(|b| b.name == requester)
                .map(|b| b.side)
        })
}

/// Whether the requirement `bind` meets is met by the value `true` (model §8 rule 2: "`(needs f)`, `(f true)` or the
/// group whose head it is. `(f false)` takes none"). An instance requires what its kind writes `beneath`, each a
/// `(needs …)` (`docs/semantics/adapters/adapters.eadl`).
fn met_by_true(description: &Description, bind: &Bind) -> bool {
    if is_instance(&bind.requester) {
        return true;
    }
    let side = side_of(description, &bind.requester)
        .unwrap_or_else(|| panic!("the reading knows no requester `{}`", bind.requester));
    if side.needs.contains(&bind.fact) {
        return true;
    }
    for clause in &side.other {
        let clause = form(clause);
        let clause = items(&clause);
        if clause.first().map(text_of).as_deref() == Some(bind.fact.as_str()) {
            return !matches!(clause.get(1), Some(Sx::Atom(value)) if value == "false");
        }
    }
    panic!(
        "the reading finds no requirement of `{}` on `{}`",
        bind.requester, bind.fact
    )
}

/// Model §8 rule 2: "A binding takes a channel when its requirement is on a consumed fact and is met by the value
/// `true` ... `(f false)` takes none, and a shared fact never does" — and so makes its provider's timer route (rule 8:
/// "a *timer*, bound through a consumed fact at `true`"; §2.9: "A channel taken on a consumed fact makes a timer
/// route").
fn takes_channel(description: &Description, bind: &Bind) -> bool {
    CONSUMED.contains(&bind.fact.as_str()) && met_by_true(description, bind)
}

/// Model §8 rule 8: "a *source*, bound through `interrupt-source` at `true`" — and §1's "bound itself for
/// `interrupt-source`, its requirement met by `true` as model §8 rule 2 reads one".
fn relies_on_source(description: &Description, bind: &Bind) -> bool {
    bind.fact == INTERRUPT_FACT && met_by_true(description, bind)
}

/// §3's node kinds (round 24, R24-D2): "`NODEKIND` one of `block`, `platform`, `instance`, `service` or `policy`", held
/// to the record's sentence by `the_reading_gives_the_record_s_worked_texts`.
const NODE_KINDS: [&str; 5] = ["block", "platform", "instance", "service", "policy"];

/// §3's causes of an unknown trigger (round 24, R24-R1; round 28, R28-D2): "`CAUSE` one of `unreached`, `(several ITEM
/// …)`, `(unstated ITEM)`, `(absent ITEM)`, `(faster ITEM)`, `(own-unstated ITEM)`, `(own-absent ITEM)` or `(own-faster
/// ITEM)`", each by its head, held to the record's sentence by `the_reading_gives_the_record_s_worked_texts`: the last
/// three the task's own source's, the three before them the one source the routes leave (§2.2).
const CAUSES: [&str; 8] = [
    "unreached",
    "several",
    "unstated",
    "absent",
    "faster",
    "own-unstated",
    "own-absent",
    "own-faster",
];

/// The class of a plan whose task's own source states a separation shorter than the task's, `(own-faster ITEM)` —
/// a branch of §2.2 no plan reaches: "since `RTP-H24` landed (`M4.1.2.3`, model §8 rule 16) no record the generator
/// accepts binds a task's own source the analysis does not admit: `(own-faster ITEM)`, `(own-absent ITEM)` and
/// `(own-unstated ITEM)` stay in the grammar, the forms the unamended rules let through" (§2.2, R29-R2). The record that
/// would reach it is refused before anything is lowered, and held as such ([`RULE_16_RECORD`], [`RULE_16_SHORTER`]).
const OWN_FASTER: &str =
    "unreachable: `(unknown (own-faster ITEM))`, its own source's separation shorter, model §8 rule 16 refusing \
     such a record (§2.2)";
/// [`OWN_FASTER`]'s twin for a task's own source stating no separation, `(own-unstated ITEM)` ([`RULE_16_UNSTATED`]).
const OWN_UNSTATED: &str =
    "unreachable: `(unknown (own-unstated ITEM))`, its own source stating no separation, model §8 rule 16 refusing \
     such a record (§2.2)";
/// [`OWN_FASTER`]'s twin for a task's own source declaring its separation absent, `(own-absent ITEM)`
/// ([`RULE_16_ABSENT`]).
const OWN_ABSENT: &str =
    "unreachable: `(unknown (own-absent ITEM))`, its own source's separation absent, model §8 rule 16 refusing such \
     a record (§2.2)";
/// The own causes' classes, in §3's order of their causes: `own-unstated`, `own-absent`, `own-faster`.
const OWN_CAUSES_UNREACHABLE: [&str; 3] = [OWN_UNSTATED, OWN_ABSENT, OWN_FASTER];
/// A task's own source leaving it unknown — one of the own causes — beside another source whose route alone would
/// release it every: unreachable as they are.
const OWN_UNKNOWN_BESIDE_EVERY: &str =
    "unreachable: its own source leaving it unknown, beside one that alone would release it every, model §8 rule 16 \
     refusing such a record";
/// A task's own source leaving it unknown beside another that alone would leave it unknown too: unreachable as the
/// own causes are.
const OWN_UNKNOWN_BESIDE_UNKNOWN: &str =
    "unreachable: its own source leaving it unknown, beside one that alone would leave it unknown too, model §8 rule \
     16 refusing such a record";
/// Two tasks bound themselves to one source, one as far apart as it and one released less often, whose own source is
/// shorter than its separation, `(own-faster ITEM)`: unreachable as the own causes are.
const OWN_ONE_RELEASED_LESS_OFTEN: &str =
    "unreachable: two tasks bound themselves to one source, one as far apart as it, one released less often, model \
     §8 rule 16 refusing such a record";

/// The cause §2.2 writes for the one source that decides a task's trigger and leaves it unknown, by its head:
/// `unstated` "where it states no separation", `absent` "where it declares it absent", `faster` "where it states one
/// shorter than the task's", "each written `(own-unstated ITEM)`, `(own-absent ITEM)` or `(own-faster ITEM)` where that
/// source is the one the task binds itself" (round 28, R28-D2).
fn one_source_cause(cause: &str, own: bool) -> &'static str {
    let written = match (cause, own) {
        ("unstated", false) => "unstated",
        ("absent", false) => "absent",
        ("faster", false) => "faster",
        ("unstated", true) => "own-unstated",
        ("absent", true) => "own-absent",
        ("faster", true) => "own-faster",
        _ => panic!("`{cause}` is no cause of one source in §2.2"),
    };
    assert!(CAUSES.contains(&written), "a cause §3 lists, `{written}`");
    written
}

/// Whether a task's trigger, as the reading writes it, is §3's `(unknown CAUSE)`, whatever its cause.
fn unknown_trigger(trigger: &str) -> bool {
    let unknown = trigger.starts_with("(unknown ");
    if unknown {
        let cause = trigger["(unknown ".len()..]
            .trim_start_matches('(')
            .split([' ', ')'])
            .next()
            .expect("a cause");
        assert!(
            CAUSES.contains(&cause),
            "no cause `{cause}` in §3: {trigger}"
        );
    }
    unknown
}

/// A node's kind (§2, §3; round 24, R24-D2): `instance` for an `(adapter …)` text, else the kind of the declaration of
/// that name — a platform, a block, a policy or a service.
fn node_kind(description: &Description, node: &str) -> &'static str {
    let kind = if is_instance(node) {
        "instance"
    } else if description.platforms().iter().any(|p| p.name == node) {
        "platform"
    } else if description.blocks.iter().any(|b| b.name == node) {
        "block"
    } else {
        match description.services.iter().find(|s| s.name == node) {
            Some(service) if service.policy => "policy",
            Some(_) => "service",
            None => panic!("the reading knows no declaration `{node}` to be a node"),
        }
    };
    assert!(
        NODE_KINDS.contains(&kind),
        "a kind §3 lists, `{kind}` for `{node}`"
    );
    kind
}

/// §3: "A list with no members is written with its head alone".
fn list<I, S>(head: &str, items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut out = format!("({head}");
    for item in items {
        out.push(' ');
        out.push_str(item.as_ref());
    }
    out.push(')');
    out
}

/// One `(bind REQ FACT PROVIDER)` line of the plan, its three tokens as written (§2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Bind {
    requester: String,
    fact: String,
    provider: String,
}

/// One route as `Joint::check` reports it, "one for each provider and kind" (§2.5).
#[derive(Debug, Clone)]
struct Route {
    /// Its interrupt: `(source NAME)` or `(timer-route NAME)` (§2).
    item: String,
    provider: String,
    source: bool,
    /// "the requesters the route names" (§2.5).
    requesters: BTreeSet<String>,
    /// `(INSTANCE NAME)`, each cost of an instance that programs this provider (§2.5).
    costs: BTreeSet<String>,
}

/// What a requester reaches (§2): "A requester **reaches** what it names by `uses` or `needs`, and what it is bound
/// to as its provider, at any depth".
fn reach(edges: &BTreeMap<String, BTreeSet<String>>, from: &str) -> BTreeSet<String> {
    let mut reached = BTreeSet::new();
    let mut todo: Vec<&String> = edges.get(from).into_iter().flatten().collect();
    while let Some(at) = todo.pop() {
        if reached.insert(at.clone()) {
            todo.extend(edges.get(at).into_iter().flatten());
        }
    }
    reached
}

/// Whether comparing `a` with `b` by cross-multiplication in signed 128 bits would overflow: the reading compares
/// exactly, and counts the systems that need it.
fn past_128_bits(a: Ratio, b: Ratio) -> bool {
    let max = u128::try_from(i128::MAX).expect("positive");
    [(a.n, b.d), (b.n, a.d)]
        .iter()
        .any(|(x, y)| x.checked_mul(*y).is_none_or(|p| p > max))
}

/// Model §8 rule 7: "A system's **hardware** is the blocks and platforms its `platform` clause, its own `uses` and
/// `needs` or its tasks' name, and every block or platform those blocks and platforms name in turn through `uses` or
/// `needs` ... at any depth. A declaration of another kind ends the path: a block that only a service names is not
/// hardware."
fn system_hardware(description: &Description) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut todo: Vec<String> = std::iter::once(description.platform.clone())
        .chain(description.own.uses.iter().cloned())
        .chain(description.own.needs.iter().cloned())
        .chain(
            description
                .tasks
                .iter()
                .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned()),
        )
        .collect();
    let mut out = BTreeSet::new();
    while let Some(at) = todo.pop() {
        if let Some(block) = hardware.get(&at) {
            if out.insert(at.clone()) {
                todo.extend(
                    block
                        .side
                        .uses
                        .iter()
                        .chain(&block.side.needs)
                        .filter(|n| hardware.contains_key(n.as_str()))
                        .cloned(),
                );
            }
        }
    }
    out
}

/// The blocks and platforms the system **serves** (§2: "a block or a platform the system serves that is bound for
/// requirements of its own"; §1: "This refusal and the one above take only what the system serves"): each block or
/// platform among "a declaration its closure reaches through `uses` and `needs`" from "its own level, one of its tasks"
/// (model §8 rule 7) — its own level writing its `platform` clause too (model §8 rule 1: "what it writes at its own level
/// outside its tasks"), so its hardware (rule 7) is among them — the closure passing through every service, policy,
/// block and platform it reaches, as the joint rules do, "serving what it names" of hardware too (§1). So a block only a
/// service names is served, though it is no hardware (round 13, R13-R2: the reading had taken rule 7's "hardware" where
/// §2 says "serves"). A system named is refused before any plan (§1), so the closure goes no further into one.
fn served_hardware(description: &Description) -> BTreeSet<String> {
    served_hardware_by(description, true)
}

/// [`served_hardware`], the closure passing through what a block or a platform names that is not hardware where
/// `through_operands`, and stopping there otherwise.
fn served_hardware_by(description: &Description, through_operands: bool) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut out = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut todo: Vec<String> = std::iter::once(description.platform.clone())
        .chain(description.own.uses.iter().cloned())
        .chain(description.own.needs.iter().cloned())
        .chain(
            description
                .tasks
                .iter()
                .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned()),
        )
        .collect();
    while let Some(named) = todo.pop() {
        if !seen.insert(named.clone()) {
            continue;
        }
        if let Some(service) = description.services.iter().find(|s| s.name == named) {
            todo.extend(service.side.uses.iter().chain(&service.side.needs).cloned());
        } else if let Some(block) = hardware.get(&named) {
            out.insert(named.clone());
            todo.extend(
                block
                    .side
                    .uses
                    .iter()
                    .chain(&block.side.needs)
                    .filter(|n| through_operands || hardware.contains_key(n.as_str()))
                    .cloned(),
            );
        }
    }
    assert!(
        system_hardware(description).is_subset(&out),
        "the system's hardware is among what it serves"
    );
    out
}

/// Each block or platform `description` declares and the lowered system does not serve, with each declaration it names
/// by `uses` or `needs` that is neither a block nor a platform: hardware §1's refusal does not take (round 15, R15-R2:
/// "This refusal and the one above take only what the system serves ...: nothing else reaches its plan").
fn unserved_naming_other_kinds(description: &Description) -> Vec<(String, String)> {
    let served = served_hardware(description);
    let hardware: BTreeSet<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let others: BTreeSet<String> = description
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(description.systems().into_iter().map(|s| s.name))
        .filter(|name| !hardware.contains(name))
        .collect();
    let mut out = BTreeSet::new();
    for block in description.hardware() {
        if served.contains(&block.name) {
            continue;
        }
        for named in block.side.uses.iter().chain(&block.side.needs) {
            if others.contains(named) {
                out.insert((block.name.clone(), named.clone()));
            }
        }
    }
    out.into_iter().collect()
}

/// The record's reading of one system: every line of the kinds it derives.
struct Reading {
    lines: Vec<String>,
    /// The classes of the record's rules this system exercises, so that a universe's reach is counted.
    classes: BTreeSet<&'static str>,
    /// Where the record's bindings contradict what the reading derives from the description itself.
    findings: Vec<String>,
}

/// The plan `docs/decisions/decision_runtime-plan.md` gives `description` with the assignment `binds`, for the lines
/// the reading derives.
fn read_the_record(description: &Description, binds: &[Bind]) -> Reading {
    let system = description.system.as_str();
    // §2.2's identity is "`(task SYSTEM TASK)`, `TASK` unique in its system", which the language holds (reference §7
    // rule 6; `M1.50`). Every rule below keys a task by that text, so a reading of a system breaking it would merge its
    // tasks as round 23 found the lowering did (R23-D1).
    assert!(
        names_distinct(description),
        "two tasks of one name: the language rejects the system before any plan"
    );
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    // The blocks and platforms in the order the description declares them.
    let hardware_order: Vec<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let platforms: BTreeSet<String> = description
        .platforms()
        .into_iter()
        .map(|p| p.name)
        .collect();
    let declared_services: BTreeSet<&str> = description
        .services
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    let systems: BTreeSet<String> = description.systems().into_iter().map(|s| s.name).collect();
    let declared = |name: &str| {
        declared_services.contains(name) || hardware.contains_key(name) || systems.contains(name)
    };
    let mut classes = BTreeSet::new();
    let mut findings = Vec::new();

    // §2: a requester reaches what it names by `uses` or `needs` — a declaration, never a vocabulary fact — and what
    // it is bound to as its provider, at any depth, "but for a block's or a platform's `uses` or `needs`, which names
    // only hardware (§1) and is never a call — its `uses` composition (rule 1; rule 5's "`uses` alone is composition"),
    // its `needs` a block it brings into the system's hardware (rule 7): hardware reaches only what its own requirements
    // are bound to". So a block's or a platform's names make no edge here, whatever they name.
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut name = |who: String, side: &Side| {
        for named in side.uses.iter().chain(&side.needs) {
            if declared(named) {
                edges.entry(who.clone()).or_default().insert(named.clone());
            }
        }
    };
    for service in &description.services {
        name(service.name.clone(), &service.side);
    }
    for task in &description.tasks {
        name(task_text(task), &task.side);
    }
    name(system.to_string(), &description.own);
    let served_hardware = served_hardware(description);
    for block in hardware.values() {
        // §1: "a block or a platform the system serves naming, by `uses` or `needs`, a declaration that is neither a
        // block nor a platform" is refused before any plan; one the system does not serve may, "nothing else" reaching
        // its plan (round 15, R15-R2).
        assert!(
            !served_hardware.contains(&block.name)
                || block
                    .side
                    .uses
                    .iter()
                    .chain(&block.side.needs)
                    .all(|n| !declared(n) || hardware.contains_key(n.as_str())),
            "`{}` names what is not hardware, which §1 refuses before any plan",
            block.name
        );
        if block
            .side
            .needs
            .iter()
            .any(|n| hardware.contains_key(n.as_str()))
        {
            classes.insert("hardware: a block's or a platform's `needs` naming a block");
        }
    }
    for bind in binds {
        edges
            .entry(bind.requester.clone())
            .or_default()
            .insert(bind.provider.clone());
    }

    // Model §8 rule 7's hardware; §2's hardware the system serves, its hardware and what its closure reaches, is above.
    let system_hardware = system_hardware(description);
    // Round 15 (R15-R2): hardware the system does not serve, naming what is not hardware, and the system lowered.
    let served_services: BTreeSet<String> = {
        let mut reached = reach(&edges, system);
        for task in &description.tasks {
            reached.extend(reach(&edges, &task_text(task)));
        }
        reached
    };
    for (namer, operand) in unserved_naming_other_kinds(description) {
        let block = &hardware[&namer];
        let platform = platforms.contains(&namer);
        let by_needs = block.side.needs.contains(&operand);
        classes.insert(if systems.contains(&operand) {
            match (platform, by_needs) {
                (true, false) => "unserved: a platform's `uses` naming a system",
                (true, true) => "unserved: a platform's `needs` naming a system",
                (false, false) => "unserved: a block's `uses` naming a system",
                (false, true) => "unserved: a block's `needs` naming a system",
            }
        } else if description
            .services
            .iter()
            .any(|s| s.name == operand && s.policy)
        {
            if platform {
                "unserved: a platform naming a policy"
            } else {
                "unserved: a block naming a policy"
            }
        } else if platform {
            "unserved: a platform naming a service"
        } else {
            "unserved: a block naming a service"
        });
        if operand == system {
            classes.insert("unserved: hardware naming the lowered system");
        }
        if served_services.contains(&operand) {
            classes.insert("unserved: hardware naming a service the lowered system serves");
        }
        if !description.systems().iter().any(|s| {
            s.platform == namer
                || hardware
                    .get(&s.platform)
                    .is_some_and(|p| p.side.uses.contains(&namer))
        }) {
            classes.insert("unserved: hardware no system's platform composes");
        }
    }

    // §2: "A **node** below is a node of the joint rules' initialization graph (§2.7): a provider an active binding
    // names (a block, a platform or an adapter's instance), a block or a platform the system serves that is bound for
    // requirements of its own — a requester like any other ... — a block or a platform of the system's hardware that a
    // served task, service or policy names by `uses` or `needs`, whose code that name calls (`M4.1.18`) ..., a service,
    // or a policy", one the system serves outside its hardware being model §8 rule 7's conflict, of which no record
    // reaches here (`M1.49`). A task and the system's own level are requesters but never nodes. A service is the
    // system's where the system or a task of it reaches it.
    let mut served = reach(&edges, system);
    for task in &description.tasks {
        served.extend(reach(&edges, &task_text(task)));
    }
    let mut nodes: BTreeSet<String> = binds.iter().map(|b| b.provider.clone()).collect();
    for bind in binds {
        if hardware.contains_key(&bind.requester) {
            if served_hardware.contains(&bind.requester) {
                nodes.insert(bind.requester.clone());
                if !system_hardware.contains(&bind.requester) {
                    // Served outside the hardware only through a service or a policy naming it, or a block so served
                    // composing it: model §8 rule 7's conflict, the search writing no record (`M1.49`).
                    classes.insert(
                        "unreachable: a block outside the hardware bound for requirements of its own in a plan, model \
                         §8 rule 7's conflict refusing the service or policy that serves it",
                    );
                }
            } else {
                findings.push(format!(
                    "`{}` is bound for requirements of its own, and is no block or platform `{system}` serves (§2; \
                     model §8 rule 7)",
                    bind.requester
                ));
            }
        }
    }
    for service in &description.services {
        if served.contains(&service.name) {
            nodes.insert(service.name.clone());
        }
    }
    // Round 18 (R18-D1; `M4.1.18`): hardware a served task, service or policy names by `uses` or `needs` is a node,
    // bound or not — model §8 rule 12's "each block or platform a served task, service or policy names by `uses` or
    // `needs`, a call" — of the system's hardware since round 20 (R20-D3), a block or a platform outside it being rule
    // 7's conflict, no record of it reaching here (`M1.49`). Every task of the system is served (rule 7: "its own
    // level, one of its tasks"). The system's own level is no such namer: rule 12 lists it among what is "none" — it
    // "initialises nothing" — and its `uses` "says what the system includes" (rule 4); nor is a block or a platform,
    // whose names are "never a call" (§2). Each called block or platform, with who names it: the namer's text, its kind
    // and whether by `needs`.
    let mut called: BTreeMap<String, BTreeSet<(String, &'static str, bool)>> = BTreeMap::new();
    {
        let mut call = |namer: String, kind: &'static str, side: &Side| {
            for (named, by_needs) in side
                .uses
                .iter()
                .map(|n| (n, false))
                .chain(side.needs.iter().map(|n| (n, true)))
            {
                if hardware.contains_key(named) {
                    called.entry(named.clone()).or_default().insert((
                        namer.clone(),
                        kind,
                        by_needs,
                    ));
                }
            }
        };
        for task in &description.tasks {
            call(task_text(task), "task", &task.side);
        }
        for service in description
            .services
            .iter()
            .filter(|s| served.contains(&s.name))
        {
            // A source's handler: a service or a policy bound for `interrupt-source` at `true` (§2.6; model §8 rule 8).
            let handler = binds
                .iter()
                .any(|b| b.requester == service.name && relies_on_source(description, b));
            let kind = match (service.policy, handler) {
                (_, true) => "handler",
                (true, false) => "policy",
                (false, false) => "service",
            };
            call(service.name.clone(), kind, &service.side);
        }
    }
    for (block, namers) in &called {
        assert!(
            served_hardware.contains(block),
            "what a served requester names, the system serves (model §8 rule 7)"
        );
        nodes.insert(block.clone());
        let provider = binds.iter().any(|b| &b.provider == block);
        let requiring = binds.iter().any(|b| &b.requester == block);
        if requiring {
            continue; // a node as bound for requirements of its own too, round 12's (R12-D1)
        }
        if provider {
            // "placed when an unrelated binding makes it a node" (R18-D1): a provider only of requesters that do not
            // name it, the name giving what it gives beside that binding.
            if binds
                .iter()
                .filter(|b| &b.provider == block)
                .all(|b| !namers.iter().any(|(namer, _, _)| *namer == b.requester))
            {
                classes.insert(
                    "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                );
            }
            continue;
        }
        for (_, kind, by_needs) in namers {
            classes.insert(match (*kind, platforms.contains(block)) {
                ("task", false) => "named: a block a task names, bound for nothing, a node",
                ("task", true) => "named: a platform a task names, bound for nothing, a node",
                ("service", false) => "named: a block a service names, bound for nothing, a node",
                ("service", true) => "named: a platform a service names, bound for nothing, a node",
                ("policy", false) => "named: a block a policy names, bound for nothing, a node",
                ("policy", true) => "named: a platform a policy names, bound for nothing, a node",
                (_, false) => "named: a block a source's handler names, bound for nothing, a node",
                (_, true) => {
                    "named: a platform a source's handler names, bound for nothing, a node"
                }
            });
            classes.insert(if *by_needs {
                "named: hardware named by `needs`, bound for nothing, a node"
            } else {
                "named: hardware named by `uses`, bound for nothing, a node"
            });
        }
        if !system_hardware.contains(block) {
            // §2's node is "a block or a platform of the system's hardware that a served task, service or policy names"
            // (round 20, R20-D3): one outside it is model §8 rule 7's conflict, the search writing no record (`M1.49`).
            classes.insert(
                "unreachable: a block outside the hardware a served service or policy names in a plan, model §8 rule \
                 7's conflict refusing it",
            );
        }
    }
    // Hardware a served service or policy names, which model §8 rule 7 makes hardware by another path than the
    // platform's composition: a task's name, the system's own level's, or a block's or a platform's `needs`.
    for (block, namers) in &called {
        if !namers.iter().any(|(_, kind, _)| *kind != "task") {
            continue;
        }
        let names_it = |side: &Side| side.uses.iter().chain(&side.needs).any(|n| n == block);
        if description.tasks.iter().any(|t| names_it(&t.side)) {
            classes.insert("hardware: a block a served service or policy names, which a task's name makes hardware");
        }
        if names_it(&description.own) {
            classes.insert(
                "hardware: a block a served service or policy names, which the system's own level's name makes \
                 hardware",
            );
        }
        if system_hardware
            .iter()
            .any(|h| hardware[h].side.needs.contains(block))
        {
            classes.insert(
                "hardware: a block a served service or policy names, which a block's or a platform's `needs` brings \
                 in",
            );
        }
    }
    // A block outside the hardware a service or a policy the system does not serve names: neither served nor refused,
    // the conflict being "each serving system's" (model §8 rule 7).
    if description.services.iter().any(|s| {
        !served.contains(&s.name)
            && s.side
                .uses
                .iter()
                .chain(&s.side.needs)
                .any(|n| hardware.contains_key(n.as_str()) && !system_hardware.contains(n))
    }) {
        classes.insert(
            "outside: a block outside the hardware named by a service or a policy the system does not serve, no \
             refusal",
        );
    }
    // What names hardware and makes it no node: the system's own level, a service or a policy the system does not
    // serve, or a block or a platform composing it, whose names are "never a call" (§2) — where nothing else makes it
    // one.
    for (name, _) in hardware.iter().filter(|(name, _)| !nodes.contains(*name)) {
        let names_it = |side: &Side| side.uses.iter().chain(&side.needs).any(|n| n == name);
        if called.keys().any(|c| hardware[c].side.uses.contains(name)) {
            classes.insert(
                "named: a block hardware a served requester names composes, nothing calling nor binding it, no node",
            );
        }
        if names_it(&description.own) {
            classes.insert("named: hardware the system's own level names, no node");
        }
        if description
            .services
            .iter()
            .any(|s| !served.contains(&s.name) && names_it(&s.side))
        {
            classes.insert(
                "named: hardware a service or a policy the system does not serve names, no node",
            );
        }
    }
    // Another system's requesters are none of this one's (model §8 rule 7; §1's refusal scoped to "the requesters the
    // system serves", round 12's R12-R1).
    let names_a_system = |side: &Side| {
        side.uses
            .iter()
            .chain(&side.needs)
            .any(|n| systems.contains(n))
    };
    if description
        .others
        .iter()
        .any(|o| names_a_system(&o.own) || o.tasks.iter().any(|t| names_a_system(&t.side)))
        || description
            .services
            .iter()
            .any(|s| !served.contains(&s.name) && names_a_system(&s.side))
    {
        classes.insert("naming: a system another system's requester names, outside this one's");
    }
    if description.others.iter().any(|o| {
        o.tasks.iter().any(|t| {
            t.side
                .uses
                .iter()
                .chain(&t.side.needs)
                .any(|n| declared_services.contains(n.as_str()) && nodes.contains(n))
        })
    }) {
        classes.insert("second system: a service another system's task also uses");
    }
    if hardware
        .iter()
        .any(|(name, block)| !block.side.needs.is_empty() && !served_hardware.contains(name))
    {
        classes.insert("second system: requiring hardware the lowered system does not serve");
    }
    for node in nodes.iter().filter(|n| hardware.contains_key(n.as_str())) {
        if binds.iter().any(|b| &b.requester == node) {
            classes.insert(if platforms.contains(node) {
                "hardware: a platform bound for requirements of its own"
            } else {
                "hardware: a block bound for requirements of its own"
            });
            classes.insert(if binds.iter().any(|b| &b.provider == node) {
                "hardware: requiring, and bound to by a requester"
            } else {
                "hardware: requiring, and bound to by nothing"
            });
        }
    }

    // §2.5: "each route `Joint::check` reports, one for each provider and kind (model §8 rule 8; `Obligation::Route`),
    // its interrupt a timer route or a source, with the requesters the route names". A requester bound to a provider
    // for `interrupt-source` at `true` relies on its source; one bound for a consumed fact at `true`, taking a channel,
    // on its alarm (model §8 rules 2 and 8) — a fact at `false`, or a shared fact, makes no route. An instance is no
    // route of its own: the route is its provider's, and names the requesters bound to the instance (the worked epoch
    // extender: "the route names the requester bound to the instance, `time.monotonic`"; rule 15: "its own binding
    // beneath making its device's timer route, which carries the requesters bound to it").
    let relying = |requester: &str| -> BTreeSet<String> {
        if is_instance(requester) {
            binds
                .iter()
                .filter(|b| b.provider == requester)
                .map(|b| b.requester.clone())
                .collect()
        } else {
            BTreeSet::from([requester.to_string()])
        }
    };
    let mut routes: BTreeMap<String, Route> = BTreeMap::new();
    for bind in binds.iter().filter(|b| !is_instance(&b.provider)) {
        let source = relies_on_source(description, bind);
        let timer = takes_channel(description, bind);
        if bind.fact == INTERRUPT_FACT && !source {
            classes.insert(
                "bound otherwise: a requester bound for `interrupt-source` at `false`, no route",
            );
        }
        if CONSUMED.contains(&bind.fact.as_str()) && !timer {
            classes.insert(
                "bound otherwise: a requester bound for a consumed fact at `false`, no channel",
            );
        }
        if !source && !timer {
            continue;
        }
        let item = if source {
            format!("(source {})", bind.provider)
        } else {
            format!("(timer-route {})", bind.provider)
        };
        let route = routes.entry(item.clone()).or_insert_with(|| Route {
            item,
            provider: bind.provider.clone(),
            source,
            requesters: BTreeSet::new(),
            costs: BTreeSet::new(),
        });
        route.requesters.extend(relying(&bind.requester));
        // §2.5: "Each cost is written on that route's line, with the instance that takes it. A shared instance's cost
        // counts once".
        if timer && is_instance(&bind.requester) {
            for cost in kind_of(&bind.requester).costs {
                route.costs.insert(format!("({} {cost})", bind.requester));
            }
        }
    }
    let separation_of = |provider: &str| -> Sep {
        hardware
            .get(provider)
            .and_then(|b| b.separation)
            .unwrap_or_else(|| panic!("`{provider}` is a source block of the description"))
            .read()
    };
    let stated_of = |route: &Route| -> Option<Ratio> {
        match separation_of(&route.provider) {
            Sep::Stated(time) => Some(time),
            Sep::Unstated | Sep::Absent => None,
            Sep::Past => panic!("a separation past the arithmetic is refused before any plan (§1)"),
        }
    };

    // §2.5, the order among routes: "Each route gets a distinct **rank**, rank `1` taken first": the sources with a
    // stated separation, ascending by it, ties broken on their interrupt's text; then the sources whose separation is
    // `unknown` or `absent`, by their interrupt's text; then the timer routes, by their interrupt's text.
    let rank_key = |route: &Route| -> (u8, Ratio, String) {
        if route.source {
            match stated_of(route) {
                Some(time) => (0, time, route.item.clone()),
                None => (1, Ratio::ZERO, route.item.clone()),
            }
        } else {
            (2, Ratio::ZERO, route.item.clone())
        }
    };
    let mut ranked: Vec<&Route> = routes.values().collect();
    ranked.sort_by_key(|route| rank_key(route));
    let stated: Vec<Ratio> = routes
        .values()
        .filter(|r| r.source)
        .filter_map(stated_of)
        .collect();
    for (i, a) in stated.iter().enumerate() {
        for b in &stated[i + 1..] {
            if a == b {
                classes.insert("rank: two stated separations tied");
            }
            if past_128_bits(*a, *b) {
                classes.insert("exact: two separations compared past 128 bits");
            }
        }
    }
    let unstated = routes.values().any(|r| r.source && stated_of(r).is_none());
    if !stated.is_empty() && unstated && routes.values().any(|r| !r.source) {
        classes.insert("rank: every class in one system");
    }
    // Round 20 (R20-R7): shapes the universes reached and no class held.
    if routes.values().filter(|r| !r.source).count() >= 2 {
        classes.insert("route: two timer routes, by their interrupts' texts");
    }
    if routes
        .values()
        .any(|r| r.source && routes.contains_key(&format!("(timer-route {})", r.provider)))
    {
        classes.insert("route: a source and a timer route at one provider, two routes");
    }
    if routes.values().any(|r| {
        r.costs.iter().any(|c| c.ends_with(" expiry)"))
            && r.costs.iter().any(|c| c.ends_with(" wake)"))
    }) {
        classes.insert("route: an expiry's and a wake's costs on one route, by their texts");
    }

    let mut lines = Vec::new();

    // §2.4 and §3: `(instance INSTANCE OBLIGED … (obligations NAME …) (costs NAME …))`, by text; each fact the kind
    // offers `obliged` written with "its value, the requirement's value at that exclusive instance, the "ordinary
    // value" of the adapters record's §1, item 5", `(obliged FACT DUR)`, or, "where the requirement writes no bound,
    // the fact alone", `(obliged FACT)`. That item: "An instance offers it at the tightest bound its requester writes
    // on that fact, over every requirement it writes there ...: the value of an `exactly` bound, which the side reader
    // already refuses beside a bound it contradicts; else the least of its `at-most` bounds" — here of the instance's
    // one requester, the last item of its `(adapter KIND OVER REQUESTER)` text (round 23, R23-R7: no universe had
    // written `exactly`).
    let instances: BTreeSet<String> = binds
        .iter()
        .flat_map(|b| [&b.requester, &b.provider])
        .filter(|n| is_instance(n))
        .cloned()
        .collect();
    for instance in &instances {
        let kind = kind_of(instance);
        let mut line = format!("(instance {instance}");
        let instance_form = form(instance);
        let requester = items(&instance_form).get(3).map(text_of);
        for fact in kind.obliged {
            let within = |bound: &Dur| {
                bound
                    .seconds()
                    .expect("a bound past the arithmetic is refused before any plan (§1)")
            };
            let side = requester.as_deref().and_then(|r| side_of(description, r));
            let at_most: Vec<Ratio> = side
                .iter()
                .flat_map(|side| side.at_most.iter())
                .filter(|(f, _)| f == fact)
                .map(|(_, bound)| within(bound))
                .collect();
            let exact: Vec<Ratio> = side
                .iter()
                .flat_map(|side| side.exactly.iter())
                .filter(|(f, _)| f == fact)
                .map(|(_, bound)| within(bound))
                .collect();
            let bound = match exact.first() {
                Some(value) => {
                    // "which the side reader already refuses beside a bound it contradicts": a record holds none.
                    assert!(
                        exact.iter().all(|other| other == value)
                            && at_most.iter().all(|most| value <= most),
                        "`{instance}`'s requester writes an `exactly` bound beside one it contradicts, which the side \
                         reader refuses"
                    );
                    classes.insert("instance: an obliged bound written `exactly`, its value taken");
                    if at_most.iter().any(|most| value < most) {
                        classes.insert(
                            "instance: an `exactly` bound beside a looser `at-most`, the `exactly`'s value taken",
                        );
                    }
                    if at_most.iter().any(|most| value == most) {
                        classes.insert("instance: an `exactly` bound beside an equal `at-most`");
                    }
                    let spellings: BTreeSet<String> = side
                        .iter()
                        .flat_map(|side| side.exactly.iter())
                        .filter(|(f, _)| f == fact)
                        .map(|(_, bound)| bound.spelling())
                        .collect();
                    if spellings.len() >= 2 {
                        classes.insert("instance: one `exactly` value in two spellings");
                    }
                    Some(*value)
                }
                None => at_most.into_iter().min(),
            };
            match bound {
                Some(time) => {
                    classes.insert("instance: an obliged fact with its bound");
                    let requester = requester.as_deref().expect("a bound's requester");
                    // The bound's requester, of every kind (round 17, R17-R6: "a service's bound within the arithmetic
                    // generated and counted by no class").
                    classes.insert(if platforms.contains(requester) {
                        "instance: a platform's obliged bound"
                    } else if hardware.contains_key(requester) {
                        "instance: a block's obliged bound"
                    } else if let Some(service) =
                        description.services.iter().find(|s| s.name == requester)
                    {
                        if service.policy {
                            "instance: a policy's obliged bound"
                        } else {
                            "instance: a service's obliged bound"
                        }
                    } else {
                        // A task or the system's own level: the instance's expiry runs its code, which §1 refuses
                        // before any plan ("an adapter instance with a cost running its requester's code (`expiry`)
                        // whose requester it is").
                        "unreachable: a task's or the system's own level's obliged bound in a plan, §1 refusing the \
                         expiry it would receive"
                    });
                    if time.d > u128::from(u64::MAX) {
                        classes.insert("exact: an obliged bound at the arithmetic's edge");
                    }
                    line.push_str(&format!(" (obliged {fact} {})", time.text()));
                }
                None => {
                    classes.insert("instance: an obliged fact bare");
                    line.push_str(&format!(" (obliged {fact})"));
                }
            }
        }
        line.push_str(&format!(
            " {} {})",
            list("obligations", kind.obligations),
            list("costs", kind.costs)
        ));
        lines.push(line);
    }

    // §2.6, execution contexts. For a route `I` at provider `X`, the code its interrupt runs: `X` itself; a node the
    // route names that is bound to `X` itself, for the interrupt fact where the route is a source, or for a consumed
    // fact where it is a timer route; for a timer route at `X`, an instance bound to `X` for a consumed fact; a node
    // the route names through such an instance, when the instance's kind runs its requester's code in that interrupt;
    // and every node one of these reaches. "A task or the system's own level that a route names adds no code".
    let mut runs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for route in routes.values() {
        let mut base = BTreeSet::from([route.provider.clone()]);
        for bind in binds.iter().filter(|b| b.provider == route.provider) {
            // "for the interrupt fact where the route is a source, or for a consumed fact where it is a timer route":
            // the binding the route names it for (model §8 rules 2 and 8).
            let for_this_kind = if route.source {
                relies_on_source(description, bind)
            } else {
                takes_channel(description, bind)
            };
            if !for_this_kind {
                continue;
            }
            if is_instance(&bind.requester) {
                base.insert(bind.requester.clone());
                if kind_of(&bind.requester).runs_requester {
                    classes.insert("context: an expiry running its requester");
                    for through in binds.iter().filter(|b| b.provider == bind.requester) {
                        if nodes.contains(&through.requester)
                            && route.requesters.contains(&through.requester)
                        {
                            base.insert(through.requester.clone());
                            if hardware.contains_key(&through.requester) {
                                classes.insert("hardware: an expiry running its code");
                            }
                        }
                    }
                } else {
                    classes.insert("context: a wake running the instance alone");
                }
            } else if nodes.contains(&bind.requester) && route.requesters.contains(&bind.requester)
            {
                base.insert(bind.requester.clone());
                if hardware.contains_key(&bind.requester) {
                    classes.insert(if route.source {
                        "hardware: a source's handler"
                    } else {
                        "hardware: an alarm it programs itself"
                    });
                }
            }
        }
        let mut all = base.clone();
        for node in &base {
            all.extend(
                reach(&edges, node)
                    .into_iter()
                    .filter(|n| nodes.contains(n)),
            );
        }
        if all.iter().any(|n| {
            hardware.contains_key(n)
                && !base.contains(n)
                && binds.iter().any(|b| &b.requester == n)
                && binds
                    .iter()
                    .any(|b| &b.provider == n && base.contains(&b.requester))
        }) {
            classes.insert("hardware: in an interrupt through a binding to it");
        }
        runs.insert(route.item.clone(), all);
    }
    // §2.6: "`job` for a task, and for every node a task reaches. ... a task's line is `job` alone".
    let mut in_a_job = BTreeSet::new();
    for task in &description.tasks {
        in_a_job.extend(
            reach(&edges, &task_text(task))
                .into_iter()
                .filter(|n| nodes.contains(n)),
        );
    }
    // Hardware's names are no call (§2): what a block or a platform composes by `uses`, or names by `needs`, is run by
    // none of the contexts it runs in, but where something else puts it there.
    for (name, block) in &hardware {
        if !nodes.contains(name) {
            continue;
        }
        let composed = || block.side.uses.iter().filter(|n| nodes.contains(*n));
        if in_a_job.contains(name) {
            if platforms.contains(name) {
                classes.insert("composition: a platform a task reaches through a binding");
            }
            if composed().any(|n| !in_a_job.contains(n)) {
                classes.insert("composition: what a node in a job composes is in none");
            }
            if block
                .side
                .needs
                .iter()
                .any(|n| nodes.contains(n) && !in_a_job.contains(n))
            {
                classes.insert("hardware: what a node in a job names by `needs` is in none");
            }
        }
        for run in runs.values().filter(|run| run.contains(name)) {
            if composed().any(|n| !run.contains(n)) {
                classes
                    .insert("composition: what a node in an interrupt composes is not run there");
            }
        }
    }
    // Round 18 (R18-D1): hardware a served requester names, bound for nothing and bound to by nothing, runs where its
    // namers run — "`job` ... for every node a task reaches", and "every node one of these reaches" in an interrupt
    // (§2.6) — or, its namers used by the system's own level alone, in no job.
    for block in called.keys().filter(|b| {
        !binds
            .iter()
            .any(|bind| &bind.provider == *b || &bind.requester == *b)
    }) {
        classes.insert(if in_a_job.contains(block) {
            "named: hardware named and bound for nothing, in a job"
        } else {
            "named: hardware named and bound for nothing, in no job"
        });
        if runs.values().any(|run| run.contains(block)) {
            classes.insert(
                "named: hardware named and bound for nothing, in an interrupt that runs its namer",
            );
        }
    }

    // Round 23 (R23-R2): a service or a policy naming itself by `uses` or `needs`. Model §8 rule 12 gives "an edge ...
    // from each node a service or a policy names by `uses` or `needs` ... to that namer but to itself", and §4's `.2` a
    // node no user "by a name of itself": a name of itself serves nothing — the closure from the system and its tasks
    // reaches the namer by another name or not at all — and adds no context, the node running where it runs.
    let names_itself = |service: &Service| -> Option<&'static str> {
        if service.side.uses.contains(&service.name) {
            Some("uses")
        } else if service.side.needs.contains(&service.name) {
            Some("needs")
        } else {
            None
        }
    };
    for service in &description.services {
        let Some(by) = names_itself(service) else {
            continue;
        };
        if !nodes.contains(&service.name) {
            classes.insert(
                "self: a service or a policy naming itself, named by nothing else, no node",
            );
            continue;
        }
        classes.insert(match (service.policy, by) {
            (false, "uses") => "self: a service naming itself by `uses`, a node",
            (false, _) => "self: a service naming itself by `needs`, a node",
            (true, "uses") => "self: a policy naming itself by `uses`, a node",
            (true, _) => "self: a policy naming itself by `needs`, a node",
        });
        if !in_a_job.contains(&service.name) {
            classes.insert("self: a node naming itself, in no job");
        }
        if routes
            .values()
            .any(|r| r.source && runs[&r.item].contains(&service.name))
        {
            classes.insert("self: a node naming itself, run in a source's interrupt");
        }
    }

    // §2.2, what releases each task: `timer` for a periodic task; for a task with a minimum separation, "the source it
    // is bound to itself for `interrupt-source` met by `true`, where it is bound to one ... and otherwise the external
    // source whose route names the task or a requester it reaches" (round 16, R16-B1), `(source NAME every)` "when the
    // source's stated separation is at least the task's"; and, after round 24 (R24-R1), "`(unknown CAUSE)` otherwise,
    // the cause written so that `RTP-H1` maps each to its code and no consumer re-derives this rule: `unreached` where
    // the task binds no source itself and no source's route reaches it; `(several ITEM …)`, each source by its text,
    // where several do; and for its one source `(unstated ITEM)` where it states no separation, `(absent ITEM)` where
    // it declares it absent, `(faster ITEM)` where it states one shorter than the task's, each written `(own-unstated
    // ITEM)`, `(own-absent ITEM)` or `(own-faster ITEM)` where that source is the one the task binds itself, since the
    // task's own `(needs interrupt-source)` repairs a source the routes leave and not one it binds itself" (round 28,
    // R28-D2) — `ITEM` the source's interrupt, `(source NAME)` (§2, §3: "each `ITEM` a source"). "A timer route
    // releases no task." So the routes a task reaches decide only where it binds no source itself: its own source
    // decides alone, whatever else reaches it, and is the one source a cause names, its three causes apart from the
    // routes' three. Since model §8 rule 16 landed (`M4.1.2.3`, R29-R2), "no record the generator accepts binds a
    // task's own source the analysis does not admit", so the own three are branches no plan reaches, each stated
    // unreachable with that reason ([`OWN_CAUSES_UNREACHABLE`]), the record that would reach one refused before it is
    // lowered ([`RULE_16_RECORD`]).
    let mut released_by = BTreeMap::new();
    for task in &description.tasks {
        let trigger = match task.release {
            Release::Period(_) => "timer".to_string(),
            Release::MinSeparation(own) => {
                let own = own
                    .seconds()
                    .expect("a separation past the arithmetic is refused before any plan (§1)");
                // What the source `route` gives the task — the one the task binds itself where `bound_itself`, else the
                // one the routes leave: its trigger, and the class of §2.2's rule it meets.
                let decide = |route: &Route, bound_itself: bool| -> (String, &'static str) {
                    let unknown = |cause: &'static str| {
                        format!(
                            "(unknown ({} {}))",
                            one_source_cause(cause, bound_itself),
                            route.item
                        )
                    };
                    match (separation_of(&route.provider), bound_itself) {
                        (Sep::Stated(time), _) if time >= own => (
                            format!("(source {} every)", route.provider),
                            if time == own {
                                "trigger: every, equal separations"
                            } else {
                                "trigger: every, a longer separation"
                            },
                        ),
                        (Sep::Stated(_), false) => (
                            unknown("faster"),
                            "trigger: `(unknown (faster ITEM))`, a shorter separation",
                        ),
                        (Sep::Unstated, false) => (
                            unknown("unstated"),
                            "trigger: `(unknown (unstated ITEM))`, no separation stated",
                        ),
                        (Sep::Absent, false) => (
                            unknown("absent"),
                            "trigger: `(unknown (absent ITEM))`, the separation absent",
                        ),
                        (Sep::Stated(_), true) => (unknown("faster"), OWN_FASTER),
                        (Sep::Unstated, true) => (unknown("unstated"), OWN_UNSTATED),
                        (Sep::Absent, true) => (unknown("absent"), OWN_ABSENT),
                        (Sep::Past, _) => panic!("refused before any plan (§1)"),
                    }
                };
                let mut reached = reach(&edges, &task_text(task));
                reached.insert(task_text(task));
                let sources: Vec<&Route> = routes
                    .values()
                    .filter(|r| r.source && r.requesters.iter().any(|q| reached.contains(q)))
                    .collect();
                // The source the task is bound to itself, for `interrupt-source` met by `true` (model §8 rule 2): one
                // requirement, so one binding, at most.
                let bound_itself: Vec<&Bind> = binds
                    .iter()
                    .filter(|b| {
                        b.requester == task_text(task)
                            && !is_instance(&b.provider)
                            && relies_on_source(description, b)
                    })
                    .collect();
                assert!(
                    bound_itself.len() <= 1,
                    "one requirement on `interrupt-source`, one binding"
                );
                let mut deciding: Option<&Route> = None;
                let (trigger, class) = if let Some(own_source) = bound_itself.first() {
                    let route = &routes[&format!("(source {})", own_source.provider)];
                    classes.insert("trigger: its own source");
                    let others: Vec<&&Route> = sources
                        .iter()
                        .filter(|r| r.provider != route.provider)
                        .collect();
                    let decided = decide(route, true);
                    if others.is_empty() {
                        classes.insert("trigger: its own source, no other reaching it");
                    } else {
                        classes.insert("trigger: its own source, beside another reaching it");
                    }
                    // Another alone would decide otherwise: the record's own source, never the routes. (Round 20's
                    // counts: the class had taken another source releasing it every too, its trigger's text naming
                    // that source, for one that would leave it unknown.)
                    if let [other] = others.as_slice() {
                        let alone = decide(other, false).0;
                        classes.insert(match (unknown_trigger(&decided.0), unknown_trigger(&alone)) {
                            (true, false) => OWN_UNKNOWN_BESIDE_EVERY,
                            (false, true) => {
                                "trigger: its own source releasing it every, beside one that alone would leave it \
                                 unknown"
                            }
                            (false, false) => {
                                "trigger: its own source releasing it every, beside another that alone would release \
                                 it every from itself"
                            }
                            (true, true) => OWN_UNKNOWN_BESIDE_UNKNOWN,
                        });
                    }
                    if route
                        .requesters
                        .iter()
                        .any(|q| *q != task_text(task) && reached.contains(q))
                    {
                        classes.insert(
                            "trigger: its own source, reached through a requester its route names too",
                        );
                    }
                    deciding = Some(route);
                    decided
                } else {
                    // Beside another task bound to a source itself: this one, binding none, is released by the routes
                    // it reaches, one of them that task's own source where it reaches its handler.
                    let others_own: BTreeSet<&String> = binds
                        .iter()
                        .filter(|b| {
                            b.requester.starts_with("(task ")
                                && b.requester != task_text(task)
                                && !is_instance(&b.provider)
                                && relies_on_source(description, b)
                        })
                        .map(|b| &b.provider)
                        .collect();
                    if !others_own.is_empty() {
                        classes
                            .insert("trigger: binding none, beside a task bound to its own source");
                        if sources.iter().any(|r| others_own.contains(&r.provider)) {
                            classes.insert(
                                "trigger: binding none, reaching the handler of another task's own source",
                            );
                        }
                    }
                    match sources.as_slice() {
                        [one] => {
                            if one.requesters.iter().any(|q| hardware.contains_key(q)) {
                                classes.insert("trigger: a source a block or a platform handles");
                            }
                            deciding = Some(*one);
                            decide(one, false)
                        }
                        [] => (
                            "(unknown unreached)".to_string(),
                            "trigger: `(unknown unreached)`, no source reaching it",
                        ),
                        several => {
                            // "each source by its text": the routes iterate by their interrupts' texts, each compared
                            // byte by byte.
                            let items: BTreeSet<&str> =
                                several.iter().map(|r| r.item.as_str()).collect();
                            let declared: Vec<&str> = hardware_order
                                .iter()
                                .map(String::as_str)
                                .filter(|name| several.iter().any(|r| r.provider == *name))
                                .collect();
                            if items
                                .iter()
                                .map(|item| &item["(source ".len()..item.len() - 1])
                                .ne(declared.iter().copied())
                            {
                                classes.insert(
                                    "trigger: several sources, by their texts unlike their declaration order",
                                );
                            }
                            (
                                format!("(unknown {})", list("several", items)),
                                "trigger: `(unknown (several ITEM …))`, several sources",
                            )
                        }
                    }
                };
                if let Some(Sep::Stated(time)) = deciding.map(|r| separation_of(&r.provider)) {
                    if past_128_bits(time, own) {
                        classes.insert(
                            "exact: a source's and a task's separations compared past 128 bits",
                        );
                    }
                }
                classes.insert(class);
                trigger
            }
        };
        released_by.insert(task_text(task), trigger);
    }
    // Two tasks bound themselves to one source (round 28, R28-R4): each released by it as its own separation decides
    // against the source's, so one as far apart as the source is released on every arrival where one released less
    // often is not, `(own-faster ITEM)` — which model §8 rule 16 refuses, so no plan reaches it (R29-R2).
    let mut own_binders: BTreeMap<&str, Vec<Ratio>> = BTreeMap::new();
    for task in &description.tasks {
        let Release::MinSeparation(apart) = task.release else {
            continue;
        };
        if let Some(bind) = binds.iter().find(|b| {
            b.requester == task_text(task)
                && !is_instance(&b.provider)
                && relies_on_source(description, b)
        }) {
            own_binders
                .entry(bind.provider.as_str())
                .or_default()
                .push(apart.seconds().expect("within the arithmetic (§1)"));
        }
    }
    for (provider, apart) in &own_binders {
        if apart.len() < 2 {
            continue;
        }
        classes.insert("trigger: two tasks bound themselves to one source");
        if let Sep::Stated(time) = separation_of(provider) {
            if apart.contains(&time) && apart.iter().any(|t| *t > time) {
                classes.insert(OWN_ONE_RELEASED_LESS_OFTEN);
            }
        }
    }

    // §2.2 and §3: `(task INDEX (task SYSTEM NAME) (rank INT) RELEASE (deadline DUR) REFERENCE (jitter DUR) (overrun
    // POLICY) (released-by TRIGGER) (uses NAME …))`, by index. The index is "the number of tasks of smaller rank", the
    // rank the task's `priority` (`decision_priority-comparison-direction.md`: "`N` is a rank"); the reference
    // `(deadline-from release)` where the description writes it, else `(deadline-from)`; "its release jitter, written
    // as zero where none is stated", and "its overrun policy, `fault` where none is stated", a `POLICY` "written as the
    // description writes it" (§3); and, after round 24 (R24-D1), "every node it names by `uses` or `needs`, each once —
    // a service, a policy, or a block or a platform of the system's hardware, whose code the name calls (`M4.1.18`) — a
    // fact it names being no member, its `bind` line saying what it is bound to, so that the plan says which job calls
    // which node", by name (§3: "`uses` items by name"). Every such name is a node (§2: "a block or a platform of the
    // system's hardware that a served task ... names", a task's name making it hardware by model §8 rule 7), and a task
    // naming a system is refused before any plan (§1).
    let mut task_lines: Vec<(usize, String)> = Vec::new();
    for task in &description.tasks {
        let index = description
            .tasks
            .iter()
            .filter(|other| other.rank() < task.rank())
            .count();
        let within = |d: Dur| {
            d.seconds()
                .expect("a duration past the arithmetic is refused before any plan (§1)")
        };
        let release = within(task.release.duration());
        let deadline = within(task.deadline());
        let jitter = task.jitter.map_or(Ratio::ZERO, within);
        let named: Vec<&str> = task
            .side
            .uses
            .iter()
            .chain(&task.side.needs)
            .map(String::as_str)
            .filter(|n| declared(n))
            .collect();
        assert!(
            named.iter().all(|n| nodes.contains(*n)),
            "every declaration a task names is a node (§2), a system refused before any plan (§1)"
        );
        let members: BTreeSet<&str> = named.iter().copied().collect();
        for member in &members {
            if !hardware.contains_key(*member) {
                continue;
            }
            classes.insert(if platforms.contains(*member) {
                "uses: a platform a task names, a member"
            } else {
                "uses: a block a task names, a member"
            });
            classes.insert(if task.side.needs.iter().any(|n| n == member) {
                "uses: hardware a task names by `needs`, a member"
            } else {
                "uses: hardware a task names by `uses`, a member"
            });
            classes.insert(
                if binds
                    .iter()
                    .any(|b| b.requester == task_text(task) && b.provider == *member)
                {
                    "uses: hardware a task names and is bound to, a member"
                } else {
                    "uses: hardware a task names and is not bound to, a member"
                },
            );
            if members.iter().any(|m| declared_services.contains(m)) {
                classes.insert("uses: hardware and a service or a policy a task names, by name");
            }
        }
        if named.len() > members.len() {
            classes.insert("uses: a node a task names twice, written once");
        }
        let reference = match task.deadline_from {
            Some("release") => {
                classes.insert("task: deadline-from written");
                "(deadline-from release)"
            }
            None => {
                classes.insert("task: deadline-from owed");
                "(deadline-from)"
            }
            Some(other) => panic!("a deadline from `{other}` is refused before any plan (§1)"),
        };
        classes.insert(match task.jitter {
            None => "task: jitter zero where none is stated",
            Some(_) if jitter == Ratio::ZERO => "task: jitter written zero",
            Some(_) if jitter > release => "task: jitter longer than its release's duration",
            Some(_) => "task: jitter written",
        });
        classes.insert(match task.overrun {
            None => "task: overrun fault where none is stated",
            Some("fault") => "task: overrun fault as written",
            Some(_) => "task: overrun another policy as written",
        });
        if [release, deadline, jitter]
            .iter()
            .any(|t| t.d > u128::from(u64::MAX))
        {
            classes.insert("exact: a task's duration at the arithmetic's edge");
        }
        if deadline == Ratio::ZERO {
            // Model §4 rule 8 refuses a `period` or `min-separation` of zero, no deadline of zero.
            classes.insert("task: a zero deadline");
        }
        // §3: "`INT` is a decimal integer with no sign and no leading zero", whatever the description's spelling.
        if task
            .spelled_priority
            .is_some_and(|spelled| spelled != task.rank().to_string())
        {
            classes.insert("rank: a rank spelled otherwise than its decimal `INT`");
        }
        if task.rank() > u128::from(u32::MAX) {
            classes.insert("rank: a rank past 32 bits");
        }
        task_lines.push((
            index,
            format!(
                "(task {index} {} (rank {}) ({} {}) (deadline {}) {reference} (jitter {}) (overrun {}) \
                 (released-by {}) {})",
                task_text(task),
                task.rank(),
                task.release.clause(),
                release.text(),
                deadline.text(),
                jitter.text(),
                task.overrun.unwrap_or("fault"),
                released_by[&task_text(task)],
                list("uses", members)
            ),
        ));
    }
    task_lines.sort();
    lines.extend(task_lines.into_iter().map(|(_, line)| line));
    // Round 20 (R20-R7): ranks whose index is not the rank less one, "the ranks of a system need not be contiguous"
    // (model §4 rule 5), and an order by value that no order by spelling gives.
    let mut ranks: Vec<u128> = description.tasks.iter().map(Task::rank).collect();
    ranks.sort_unstable();
    if ranks.first().is_some_and(|first| *first > 1) {
        classes.insert("rank: a rank past 1 first");
    }
    if ranks.windows(2).any(|w| w[1] > w[0] + 1) {
        classes.insert("rank: ranks not consecutive");
    }
    let spelled = |t: &Task| {
        t.spelled_priority
            .map_or_else(|| t.priority.to_string(), str::to_string)
    };
    let mut by_rank: Vec<&Task> = description.tasks.iter().collect();
    by_rank.sort_by_key(|t| t.rank());
    let mut by_spelling = by_rank.clone();
    by_spelling.sort_by_key(|t| spelled(t));
    if by_rank
        .iter()
        .zip(&by_spelling)
        .any(|(a, b)| !std::ptr::eq(*a, *b))
    {
        classes.insert("rank: runtime order unlike the ranks' spellings' order");
    }

    // §2.5, the release timer "releases every task whose trigger is `timer`", `releases` items by index (§3); a task's
    // index is "the number of tasks of smaller rank" (§2.2), its rank its priority's place.
    let mut by_index: Vec<&Task> = description.tasks.iter().collect();
    by_index.sort_by_key(|t| t.rank());
    let released: Vec<String> = by_index
        .iter()
        .map(|t| task_text(t))
        .filter(|t| released_by[t] == "timer")
        .collect();
    lines.push(format!("(timer simulator {})", list("releases", &released)));

    // §2.5 and §3: `(route INT ITEM (separation DUR|unknown|absent|programmed) (level LEVEL) (deliverable yes)
    // (requesters REQ …) (costs (INSTANCE NAME) …))`, by rank. A source's separation is "the duration the provider
    // states", `absent` "where it declares the fact absent", `unknown` "where it states none"; a timer route's
    // `programmed`. The level is the profile's caller level, `machine` for `rt-static-up-v1` (§1).
    for (index, route) in ranked.iter().enumerate() {
        let separation = if route.source {
            match separation_of(&route.provider) {
                Sep::Stated(time) => {
                    if time.d > u128::from(u64::MAX) {
                        classes.insert("exact: a separation at the arithmetic's edge");
                    }
                    time.text()
                }
                Sep::Unstated => "unknown".to_string(),
                Sep::Absent => "absent".to_string(),
                Sep::Past => panic!("refused before any plan (§1)"),
            }
        } else {
            "programmed".to_string()
        };
        lines.push(format!(
            "(route {} {} (separation {separation}) (level machine) (deliverable yes) {} {})",
            index + 1,
            route.item,
            list("requesters", &route.requesters),
            list("costs", &route.costs)
        ));
    }
    // §2.5, the enabled set: `timer`; each route — "`timer`, then the routes by rank" (§3).
    lines.push(list(
        "enabled",
        std::iter::once("timer").chain(ranked.iter().map(|r| r.item.as_str())),
    ));
    // §2.5, the interrupt model: "interrupts do not nest, and every masked section masks every interrupt"; and §2.10,
    // "The plan's last line says whether it is complete, and `/1`'s never is".
    lines.push("(interrupt-model (nesting no) (masking all))".to_string());
    lines.push("(complete no)".to_string());

    // §2 and §3, after round 24 (R24-D2): "The plan writes each node with its kind — `block`, `platform`, `instance`,
    // `service` or `policy` — so that the simulator and the emitter read from it which nodes are devices ..., never
    // from the description": `(node NODE NODEKIND)`, "one for each node", "`node` lines by their text". An instance is
    // known by its `(adapter …)` text; a block, a platform, a service and a policy by the declaration of that name.
    for node in &nodes {
        let kind = node_kind(description, node);
        classes.insert(match kind {
            "block" => "node: a block",
            "platform" => "node: a platform",
            "instance" => "node: an instance",
            "service" => "node: a service",
            _ => "node: a policy",
        });
        lines.push(format!("(node {node} {kind})"));
    }

    // §2.6 and §3: one `(context SUBJECT CONTEXT …)` line for each task and each node, by subject's text; within a line
    // `job`, then each `(interrupt …)` by its text, then `initialisation` — "for every node".
    let mut contexts: Vec<(String, String)> = Vec::new();
    for task in &description.tasks {
        contexts.push((
            task_text(task),
            format!("(context {} job)", task_text(task)),
        ));
    }
    for node in &nodes {
        let mut parts = Vec::new();
        if in_a_job.contains(node) {
            parts.push("job".to_string());
        }
        let mut interrupts: Vec<String> = runs
            .iter()
            .filter(|(_, run)| run.contains(node))
            .map(|(item, _)| format!("(interrupt {item})"))
            .collect();
        interrupts.sort();
        if interrupts.len() >= 2 {
            classes.insert("context: a node in two interrupts");
        }
        parts.extend(interrupts);
        parts.push("initialisation".to_string());
        contexts.push((
            node.clone(),
            format!("(context {node} {})", parts.join(" ")),
        ));
    }
    contexts.sort();
    lines.extend(contexts.into_iter().map(|(_, line)| line));

    // §2.7: the graph is `Joint::initialization`, model §8 rule 12: "An edge runs from a provider to each requester
    // bound to it, whatever composes what, but to itself — composition alone giving none (`M4.1.16`) — and from each
    // node a service or a policy names by `uses` or `needs`, a block or a platform included (`M4.1.17`), to that namer
    // — a block's or a platform's own names giving none; each carries its reason", between nodes — "the system's own
    // level ... and its tasks ... are none". §2.7: "naming each reason `Joint::initialization` gives, a binding's fact or
    // `named` for a `uses` or `needs` — a block's or a platform's `uses` or `needs` giving none, hardware's names never a
    // call (§2), while a service's or a policy's naming a block or a platform, a call, gives one (`M4.1.17`);
    // composition, a `uses`, gives no edge either, while a binding keeps its edge whatever composes what, a composite
    // and its part each bound to the other being one component" (round 16, R16-B2; round 17, R17-D1). Composition is a
    // block's or a platform's `uses` alone (rule 1; rule 5: "names it through `uses`, at any depth through blocks and
    // platforms, whose `uses` alone is composition"): it gives no edge of its own, and takes none a binding gives. An
    // instance composes nothing, and "the initialization graph keeps its edges, its device before it and it before its
    // requesters" (rule 15). A block's or a platform's names make no named edge; one naming anything but hardware is
    // refused (§1). A block or a platform a served service or policy names is a node whatever binds it (§2; round 18,
    // R18-D1, `M4.1.18`), so its named edge stands bound or not; a task naming one gives none, a task being no node.
    let composes = |from: &str| -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut todo = vec![from.to_string()];
        while let Some(at) = todo.pop() {
            if let Some(block) = hardware.get(&at) {
                for named in block
                    .side
                    .uses
                    .iter()
                    .filter(|n| hardware.contains_key(n.as_str()))
                {
                    if reached.insert(named.clone()) {
                        todo.push(named.clone());
                    }
                }
            }
        }
        reached
    };
    #[derive(Default)]
    struct Reasons {
        facts: BTreeSet<String>,
        named: bool,
    }
    let mut graph: BTreeMap<(String, String), Reasons> = BTreeMap::new();
    // The edges a binding gives across composition, `(earlier, later)`: the provider before its requester.
    let mut across: BTreeSet<(String, String)> = BTreeSet::new();
    for bind in binds {
        if nodes.contains(&bind.requester)
            && nodes.contains(&bind.provider)
            && bind.requester != bind.provider
        {
            let composite_bound = composes(&bind.requester).contains(&bind.provider);
            let part_bound = composes(&bind.provider).contains(&bind.requester);
            if composite_bound || part_bound {
                across.insert((bind.provider.clone(), bind.requester.clone()));
                let (composite, part) = if composite_bound {
                    classes.insert("initialisation: a composite bound to its part, its edge kept");
                    (&bind.requester, &bind.provider)
                } else {
                    classes.insert("initialisation: a part bound to its composite, its edge kept");
                    (&bind.provider, &bind.requester)
                };
                if !hardware[composite].side.uses.contains(part) {
                    classes.insert(
                        "initialisation: an edge across composition at depth, through another part, kept",
                    );
                }
                if !platforms.contains(composite) {
                    classes.insert(
                        "initialisation: a block composing a block, an edge across it kept",
                    );
                }
            }
            let by_needs = |a: &str, b: &str| {
                hardware
                    .get(a)
                    .is_some_and(|block| block.side.needs.iter().any(|n| n == b))
            };
            if by_needs(&bind.provider, &bind.requester)
                || by_needs(&bind.requester, &bind.provider)
            {
                classes.insert(
                    "initialisation: an edge across a block's `needs` naming a block stands",
                );
            }
            if hardware.contains_key(&bind.requester) {
                classes.insert("initialisation: an edge to a block or a platform bound for its own requirements");
            }
            graph
                .entry((bind.provider.clone(), bind.requester.clone()))
                .or_default()
                .facts
                .insert(bind.fact.clone());
        }
    }
    // The named edges from hardware, `(earlier, later)`: the block or the platform before the service or the policy
    // naming it.
    let mut named_hardware: BTreeSet<(String, String)> = BTreeSet::new();
    for namer in description
        .services
        .iter()
        .filter(|s| nodes.contains(&s.name))
    {
        for (named, by_needs) in namer
            .side
            .uses
            .iter()
            .map(|n| (n, false))
            .chain(namer.side.needs.iter().map(|n| (n, true)))
        {
            if *named == namer.name {
                // Rule 12: "to that namer but to itself" (round 23, R23-R2).
                classes.insert("self: a node naming itself, no edge from it to itself");
                continue;
            }
            if !declared(named) {
                continue;
            }
            if !hardware.contains_key(named) {
                if nodes.contains(named) {
                    graph
                        .entry((named.clone(), namer.name.clone()))
                        .or_default()
                        .named = true;
                    classes.insert("initialisation: a named edge");
                }
                continue;
            }
            // Round 18 (R18-D1) inverted round 17's class, "hardware a service or a policy names, no node, no edge": §2
            // makes every block or platform a served service or policy names a node, bound or not, as `called` reads it
            // above — the reading's own construction, so asserted rather than counted.
            assert!(
                nodes.contains(named),
                "hardware a served service or policy names is a node (§2, `M4.1.18`)"
            );
            graph
                .entry((named.clone(), namer.name.clone()))
                .or_default()
                .named = true;
            named_hardware.insert((named.clone(), namer.name.clone()));
            classes.insert(match (namer.policy, platforms.contains(named), by_needs) {
                (false, false, false) => {
                    "initialisation: a named edge from a block a service names by `uses`"
                }
                (false, false, true) => {
                    "initialisation: a named edge from a block a service names by `needs`"
                }
                (false, true, false) => {
                    "initialisation: a named edge from a platform a service names by `uses`"
                }
                (false, true, true) => {
                    "initialisation: a named edge from a platform a service names by `needs`"
                }
                (true, false, false) => {
                    "initialisation: a named edge from a block a policy names by `uses`"
                }
                (true, false, true) => {
                    "initialisation: a named edge from a block a policy names by `needs`"
                }
                (true, true, false) => {
                    "initialisation: a named edge from a platform a policy names by `uses`"
                }
                (true, true, true) => {
                    "initialisation: a named edge from a platform a policy names by `needs`"
                }
            });
            let provider = binds.iter().any(|b| &b.provider == named);
            let requiring = binds.iter().any(|b| &b.requester == named);
            classes.insert(match (provider, requiring) {
                (true, false) => "initialisation: a named edge from hardware a provider alone",
                (false, true) => {
                    "initialisation: a named edge from hardware bound for its own requirements alone"
                }
                (true, true) => {
                    "initialisation: a named edge from hardware a provider and bound for its own requirements"
                }
                (false, false) => {
                    "initialisation: a named edge from hardware bound for nothing and bound to by nothing"
                }
            });
            if !system_hardware.contains(named) {
                classes.insert(
                    "unreachable: a named edge from a block outside the hardware, model §8 rule 7's conflict refusing \
                     the name",
                );
            }
            // Beside the path a binding gives through an instance: the device, then the instance, then the namer.
            if binds.iter().any(|beneath| {
                &beneath.provider == named
                    && is_instance(&beneath.requester)
                    && binds
                        .iter()
                        .any(|b| b.requester == namer.name && b.provider == beneath.requester)
            }) {
                classes.insert(
                    "initialisation: a named edge from hardware an instance programs, beside the instance's path",
                );
            }
            if routes
                .values()
                .any(|r| &r.provider == named && runs[&r.item].contains(&namer.name))
            {
                classes.insert(
                    "initialisation: a named edge from hardware whose own interrupt runs its namer",
                );
            }
        }
    }
    // A service or a policy the system does not serve is no node, so no edge reaches it, whatever it names.
    if description.services.iter().any(|s| {
        !nodes.contains(&s.name)
            && s.side
                .uses
                .iter()
                .chain(&s.side.needs)
                .any(|n| hardware.contains_key(n.as_str()) && nodes.contains(n))
    }) {
        classes.insert(
            "initialisation: hardware a node, named by a service or a policy the system does not serve, no edge",
        );
    }
    // Rule 12's two clauses each end "but to itself": no edge joins a node to itself.
    assert!(
        graph.keys().all(|(earlier, later)| earlier != later),
        "rule 12 gives no edge from a node to itself"
    );
    // "one line for each ordered pair of nodes an edge joins, the node the edge orders earlier first ... naming each
    // reason", by "the earlier node's text, then the later's; within a line, `(binding …)` reasons by fact, then
    // `named`" (§3).
    for ((earlier, later), reasons) in &graph {
        let mut because: Vec<String> = reasons
            .facts
            .iter()
            .map(|f| format!("(binding {f})"))
            .collect();
        if reasons.named {
            because.push("named".to_string());
        }
        lines.push(format!(
            "(depends {earlier} {later} {})",
            list("because", because)
        ));
    }
    // "Its steps are its strongly connected components, in a topological order taken by repeatedly choosing, among the
    // components whose predecessors have all been taken, the one whose least member's text is least. A step names its
    // members by their text, in order."
    let after = |node: &str| -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut todo = vec![node.to_string()];
        while let Some(at) = todo.pop() {
            for (earlier, later) in graph.keys() {
                if *earlier == at && reached.insert(later.clone()) {
                    todo.push(later.clone());
                }
            }
        }
        reached
    };
    let reached: BTreeMap<&String, BTreeSet<String>> =
        nodes.iter().map(|n| (n, after(n))).collect();
    let components: BTreeSet<Vec<String>> = nodes
        .iter()
        .map(|n| {
            nodes
                .iter()
                .filter(|m| *m == n || (reached[n].contains(*m) && reached[*m].contains(n)))
                .cloned()
                .collect()
        })
        .collect();
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut step = 0;
    while taken.len() < nodes.len() {
        let ready = components
            .iter()
            .filter(|c| !taken.contains(&c[0]))
            .filter(|c| {
                graph.keys().all(|(earlier, later)| {
                    !c.contains(later) || c.contains(earlier) || taken.contains(earlier)
                })
            })
            .min_by(|a, b| a[0].cmp(&b[0]))
            .expect("a component whose predecessors are all taken");
        if ready.len() >= 2 {
            classes.insert("initialisation: a component of several members");
            if ready.iter().any(|member| is_instance(member)) {
                classes.insert("initialisation: a component holding an instance");
            }
        }
        step += 1;
        lines.push(format!("(step {step} {})", ready.join(" ")));
        taken.extend(ready.iter().cloned());
    }
    // Round 16 (R16-B2): "a composite and its part each bound to the other being one component"; and an edge across
    // composition between two components, its earlier node's text the greater, so that the texts alone would order the
    // two the other way.
    for (earlier, later) in &across {
        if reached[earlier].contains(later) && reached[later].contains(earlier) {
            if across.contains(&(later.clone(), earlier.clone())) {
                classes.insert(
                    "initialisation: a composite and its part bound each to the other, one component",
                );
            }
        } else if earlier > later {
            classes.insert(
                "initialisation: an edge across composition ordering two steps unlike their texts",
            );
        }
    }
    // Round 17 (R17-D1): a named edge from hardware on the line a binding's reason holds too, `(binding …)` reasons
    // first, then `named` (§3); and one, `named` its one reason and the graph's one path between its two nodes, that
    // orders the earlier, whose text is the greater, before the later.
    for (earlier, later) in &named_hardware {
        if !graph[&(earlier.clone(), later.clone())].facts.is_empty() {
            classes
                .insert("initialisation: a named edge from hardware beside a binding's, one line");
            continue;
        }
        let mut reached = BTreeSet::new();
        let mut todo = vec![earlier.clone()];
        while let Some(at) = todo.pop() {
            for (from, to) in graph.keys() {
                if *from == at && !(from == earlier && to == later) && reached.insert(to.clone()) {
                    todo.push(to.clone());
                }
            }
        }
        if !reached.contains(later) && earlier > later {
            classes.insert(
                "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike \
                 their texts",
            );
        }
    }

    // §2.9: "A node that takes a channel — a service, a policy, or a block or platform with requirements of its own;
    // an instance aside, whose kind's obligations say how it shares — is **multiplexed** where it has two or more
    // **users**: each of the system's tasks that reaches it (§2), directly or through services, policies and bindings;
    // and each interrupt whose context line (§2.6) holds it, but for its own timer route — at the provider whose
    // channel it takes, or that the instance whose channel it takes programs — where nothing else that route runs
    // reaches it". "the system's own level is none".
    let mut multiplexed = BTreeSet::new();
    let self_named = |node: &str| {
        description
            .services
            .iter()
            .any(|s| s.name == node && names_itself(s).is_some())
    };
    for node in nodes.iter().filter(|n| !is_instance(n)) {
        let channels: Vec<&Bind> = binds
            .iter()
            .filter(|b| &b.requester == node && takes_channel(description, b))
            .collect();
        if channels.is_empty() {
            continue; // it takes no channel (model §8 rule 2)
        }
        let mut own_routes = BTreeSet::new();
        for channel in channels {
            if is_instance(&channel.provider) {
                for programmed in binds
                    .iter()
                    .filter(|b| b.requester == channel.provider && takes_channel(description, b))
                {
                    own_routes.insert(format!("(timer-route {})", programmed.provider));
                }
            } else {
                own_routes.insert(format!("(timer-route {})", channel.provider));
            }
        }
        let mut users = BTreeSet::new();
        for task in &description.tasks {
            if reach(&edges, &task_text(task)).contains(node) {
                users.insert(task_text(task));
            }
        }
        for (item, run) in &runs {
            if !run.contains(node) {
                continue;
            }
            if own_routes.contains(item) {
                // "that route's own code delivering to it is no caller, but another node's code there calling it is"
                if run
                    .iter()
                    .any(|n| n != node && reach(&edges, n).contains(node))
                {
                    users.insert(item.clone());
                    classes
                        .insert("multiplex: its own route a user, another node there reaching it");
                    if self_named(node) {
                        // Its name of itself takes nothing from another node's call there (round 23, R23-R2).
                        classes.insert(
                            "self: a node naming itself on its own timer route, another node there calling it a user",
                        );
                    }
                    // The route's provider, `(timer-route X)`'s `X`, bound for a requirement of its own to the node.
                    let provider = &item["(timer-route ".len()..item.len() - 1];
                    if binds
                        .iter()
                        .any(|b| b.requester == provider && &b.provider == node)
                    {
                        classes.insert(
                            "multiplex: its own route a user, the route's provider bound to it",
                        );
                    }
                } else {
                    classes.insert("multiplex: its own route no user");
                    // "nor the node itself by a name of itself" (§4, `.2`): its own name of itself, `n != node` above,
                    // is no call there (round 23, R23-R2).
                    if self_named(node) {
                        classes.insert(
                            "self: a node naming itself on its own timer route, no user of itself there",
                        );
                    }
                }
            } else {
                users.insert(item.clone());
                classes.insert("multiplex: another route a user");
            }
        }
        if self_named(node) {
            classes.insert(match users.len() {
                0 => "self: a node naming itself, taking a channel, with no user",
                1 => "self: a node naming itself with one user, not multiplexed",
                _ => "self: a node naming itself, multiplexed by its other users",
            });
        }
        if users.len() >= 2 {
            multiplexed.insert(node.clone());
            classes.insert("multiplex: multiplexed");
            if hardware.contains_key(node) {
                classes.insert("multiplex: a block or a platform multiplexed");
            }
            let jobs = description
                .tasks
                .iter()
                .filter(|t| users.contains(&task_text(t)))
                .count();
            if jobs == 0 {
                classes.insert("multiplex: multiplexed by interrupts alone");
            } else if jobs >= 2 {
                classes.insert("multiplex: multiplexed by two jobs");
            } else {
                classes.insert("multiplex: multiplexed by a job and an interrupt");
            }
            if description
                .services
                .iter()
                .any(|s| &s.name == node && s.policy)
            {
                classes.insert("multiplex: a policy multiplexed");
            }
        } else if users.len() == 1 {
            classes.insert("multiplex: a channel with one user");
            if hardware.contains_key(node) {
                classes.insert("multiplex: a block or a platform's channel with one user");
            }
        }
    }

    // §2.9's table and §3: `(obligation KIND SUBJECT (discharged-by LEAF …))`, by text.
    let mut obligations = Vec::new();
    for instance in &instances {
        for obligation in kind_of(instance).obligations {
            obligations.push(format!(
                "(obligation {obligation} {instance} (discharged-by M4.2 M4.8))"
            ));
        }
    }
    for node in &multiplexed {
        obligations.push(format!(
            "(obligation multiplex {node} (discharged-by M4.2 M2.21.2))"
        ));
    }
    for route in routes.values() {
        obligations.push(format!(
            "(obligation route {} (discharged-by M4.2 M4.3))",
            route.item
        ));
    }
    obligations.sort();
    lines.extend(obligations);

    // §2.10's table and §3: `(owed SUBJECT FACT (by OWNER))`, by text.
    let mut owed = Vec::new();
    let mut owe = |subject: &str, fact: &str, owner: &str| {
        owed.push(format!("(owed {subject} {fact} (by {owner}))"));
    };
    for task in &description.tasks {
        // "each task | `entry-point`, ... `os-calls`, `os-calls-category`, ... | `application`" — every task, whatever
        // it is released by or calls (§2.2, §2.10; round 19, R19-D2).
        for fact in TASK_FACTS {
            owe(&task_text(task), fact, "application");
        }
        if task.deadline_from.is_none() {
            owe(&task_text(task), "deadline-from", "description");
        }
        // "each task whose trigger is `(unknown CAUSE)`", of whatever cause (§2.10).
        if unknown_trigger(&released_by[&task_text(task)]) {
            owe(&task_text(task), "trigger", "description");
        }
    }
    owe(system, "leaves-interrupt-hardware-alone", "application");
    // "each node | `contexts`, `no-wait`, `runtime-discipline` | `realization`" — the last the composite-inputs record's
    // §2 fact for the code the realization generates for it (round 16, R16-D3); and "each node whose context line holds
    // `job` | `call-cost`, the time one call of its code takes in a job; `masked-runs`, its code's masked runs there |
    // `realization`" — "A node in a job owes its masked runs as a task does" (round 17, R17-R2). No category is owed
    // beside them: "Every bound owed to the realization — a call cost, masked runs, a cost's terms, an overhead, a
    // service cost, an arrival bound, an obliged fact — is owed with `ROADMAP.md` §7.3's record of a bound ... The
    // realization states the record with the bound", while "The application's categories are facts of their own"
    // (§2.10; round 18, R18-D3) — so a task's `masked-category` stays, beside its `os-calls-category` (round 19,
    // R19-D2), and a node owes none.
    for node in &nodes {
        owe(node, "contexts", "realization");
        owe(node, "no-wait", "realization");
        owe(node, "runtime-discipline", "realization");
        if in_a_job.contains(node) {
            owe(node, "call-cost", "realization");
            owe(node, "masked-runs", "realization");
            classes.insert(if is_instance(node) {
                "owed: an instance in a job, its call cost and masked runs"
            } else if hardware.contains_key(node) {
                "owed: a block or a platform in a job, its call cost and masked runs"
            } else {
                "owed: a service or a policy in a job, its call cost and masked runs"
            });
        } else {
            classes.insert("owed: a node in no job, neither its call cost nor masked runs");
        }
    }
    for instance in &instances {
        let kind = kind_of(instance);
        if !kind.costs.is_empty() {
            owe(instance, "cost-terms", "realization");
        }
        for fact in kind.obliged {
            owe(instance, fact, "realization");
        }
    }
    for node in &multiplexed {
        owe(node, "overhead", "realization");
    }
    for route in routes.values() {
        if route.source {
            if separation_of(&route.provider) == Sep::Unstated {
                owe(&route.item, "separation", "description");
            }
            for fact in SOURCE_FACTS {
                owe(&route.item, fact, "realization");
            }
        } else {
            for fact in TIMER_ROUTE_FACTS {
                owe(&route.item, fact, "realization");
            }
        }
    }
    owed.sort();
    lines.extend(owed);

    Reading {
        lines,
        classes,
        findings,
    }
}

// =====================================================================================================================
// Model §8 rule 16: a task released by its own source
// =====================================================================================================================

/// What model §8 rule 16 finds of one sporadic task bound itself to a source the analysis does not admit (`M4.1.2.3`;
/// `RTP-H24`): "A sporadic task bound itself to a source for `interrupt-source` at `true` is released on every arrival
/// of that source (the runtime plan's §2.2), so the analysis admits it only where the source's stated
/// `min-arrival-separation` is at least the task's own `min-separation` ... Judged beside rule 9's judgement of callers,
/// of each served task bound so: a separation shorter than the task's, one declared absent — an unbounded source, which
/// the analysis refuses under its condition 3 — or a comparison the exact arithmetic cannot decide (§7 rule 2) is a
/// conflict, `unsupported-profile`; a source stating no value for it — none, bare, or a bound, which the relation reads
/// as no value — an unknown, `missing-fact`."
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Unadmitted {
    /// The task, `(task SYSTEM NAME)`.
    task: String,
    /// The source's block or platform.
    source: String,
    /// `unsupported-profile`, a conflict, or `missing-fact`, an unknown.
    code: &'static str,
    /// What the source states against the task: `shorter`, `absent` or `undecidable`, each a conflict, or `unstated`,
    /// an unknown.
    cause: &'static str,
}

/// What rule 16 says of the sporadic `task` released on every arrival of the source `block`: `None` where the analysis
/// admits it, "the source's stated `min-arrival-separation` ... at least the task's own `min-separation`", and
/// otherwise the code and the cause. Two times are compared exactly, as §2.2 compares them for the trigger; a
/// separation past the arithmetic, the source's or the task's, is "a comparison the exact arithmetic cannot decide".
fn rule_16_judges(task: &Task, block: &Block) -> Option<(&'static str, &'static str)> {
    let Release::MinSeparation(apart) = task.release else {
        return None;
    };
    let separation = block
        .separation
        .unwrap_or_else(|| {
            panic!(
                "`{}` offers `interrupt-source`, stating nothing the reading reads of its separation",
                block.name
            )
        })
        .read();
    let own = match apart.time() {
        Time::Seconds(own) => Some(own),
        Time::Past => None,
        Time::Other(unit) => panic!(
            "no universe binds a task's own source with its minimum separation in `{unit}`, which the language rejects"
        ),
    };
    match (separation, own) {
        (Sep::Unstated, _) => Some(("missing-fact", "unstated")),
        (Sep::Absent, _) => Some(("unsupported-profile", "absent")),
        (Sep::Past, _) | (_, None) => Some(("unsupported-profile", "undecidable")),
        (Sep::Stated(time), Some(own)) if time >= own => None,
        (Sep::Stated(_), Some(_)) => Some(("unsupported-profile", "shorter")),
    }
}

/// Whether no two tasks of any one system the description declares carry one name, as the language holds of what it
/// accepts (reference §7 rule 6, under which "two systems' tasks may share a name";
/// [`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]): every rule reads a task by its text, rule 16 among them, so each is
/// read only of a description where none do.
fn names_distinct(description: &Description) -> bool {
    std::iter::once(&description.tasks)
        .chain(description.others.iter().map(|system| &system.tasks))
        .all(|tasks| {
            let names: BTreeSet<&str> = tasks.iter().map(|t| t.name.as_str()).collect();
            names.len() == tasks.len()
        })
}

/// The sporadic tasks of the lowered system requiring `interrupt-source` themselves — a task writes it by `needs` alone
/// (`docs/semantics/kinds/os-rt.eadl`) — each with every source it may be bound to: each block or platform of the
/// system's hardware (model §8 rule 7) offering `(interrupt-source true)`, reachable at the profile's level (rule 9).
fn own_source_candidates(description: &Description) -> Vec<(&Task, Vec<Block>)> {
    let hardware = system_hardware(description);
    let sources: Vec<Block> = description
        .hardware()
        .into_iter()
        .filter(|b| {
            hardware.contains(&b.name)
                && b.offers.iter().any(|o| o == "(interrupt-source true)")
                && b.offers
                    .iter()
                    .any(|o| o == "(reachable-at-privilege machine)")
        })
        .collect();
    description
        .tasks
        .iter()
        .filter(|t| {
            matches!(t.release, Release::MinSeparation(_))
                && t.side.needs.iter().any(|f| f == INTERRUPT_FACT)
        })
        .map(|t| (t, sources.clone()))
        .collect()
}

/// Rule 16's verdict on the search where it leaves no assignment: "Since the search tries every assignment (rule 6), a
/// task whose description offers a source at least as far apart is bound to it, and one whose description offers none
/// is refused, naming the task and the source". Rule 6: "When no assignment holds, the description takes the highest
/// code, by precedence, of one assignment's findings: the first blocked by unknowns alone, `missing-fact`, or, where
/// every assignment has a conflict, the first's conflicts ... — each finding its own code where it names one, as ...
/// rule 16 name[s] `unsupported-profile`". The tasks' own sources are chosen apart, no rule joining them, so some
/// assignment is blocked by unknowns alone where each such task may take a source admitted or stating no separation —
/// `missing-fact` — and every assignment has a conflict where some task's every source is one — `unsupported-profile`,
/// rule 16's own code.
fn rule_16_search(description: &Description) -> Option<&'static str> {
    let mut refused = None;
    for (task, sources) in own_source_candidates(description) {
        let judged: Vec<Option<(&str, &str)>> =
            sources.iter().map(|s| rule_16_judges(task, s)).collect();
        if judged.is_empty() || judged.iter().any(Option::is_none) {
            continue;
        }
        if judged
            .iter()
            .flatten()
            .all(|(code, _)| *code == "unsupported-profile")
        {
            return Some("unsupported-profile");
        }
        refused = Some("missing-fact");
    }
    refused
}

/// The search's verdict for a code, as [`flow`] writes it.
fn verdict_written(code: &str) -> &'static str {
    match code {
        "unsupported-profile" => "UnsupportedProfile",
        "missing-fact" => "MissingFact",
        other => panic!("rule 16 gives no `{other}`"),
    }
}

/// Whether some sporadic task binding its own source may be bound to one the analysis does not admit: the systems
/// whose record the unamended rules gave can differ from the search's.
fn rule_16_may_refuse(description: &Description) -> bool {
    own_source_candidates(description)
        .iter()
        .any(|(task, sources)| sources.iter().any(|s| rule_16_judges(task, s).is_some()))
}

/// Rule 16's findings on a record's bindings, "of each served task bound so": each sporadic task bound itself, for
/// `interrupt-source` met by `true` (model §8 rule 2), to a source the analysis does not admit — the conflicts first,
/// then the unknowns, as §1 names the generator's findings, each group by the task's text.
fn unadmitted(description: &Description, binds: &[Bind]) -> Vec<Unadmitted> {
    let system = description.system.as_str();
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut out = Vec::new();
    for task in &description.tasks {
        let text = format!("(task {system} {})", task.name);
        for bind in binds.iter().filter(|b| {
            b.requester == text && !is_instance(&b.provider) && relies_on_source(description, b)
        }) {
            if let Some((code, cause)) = rule_16_judges(task, &hardware[&bind.provider]) {
                out.push(Unadmitted {
                    task: text.clone(),
                    source: bind.provider.clone(),
                    code,
                    cause,
                });
            }
        }
    }
    out.sort_by_key(|u| (u.code != "unsupported-profile", u.task.clone()));
    out
}

/// A separation no task of these universes is released more often than, a thousand seconds: the one
/// [`admitting_twin`] gives every source.
const ADMITTING_MS: u64 = 1_000_000;

/// `description` with every source stating [`ADMITTING_MS`] — a separation rule 16 admits for every task bound to it —
/// and nothing else changed: the description whose search gives the record the unamended rules gave this one. Model §8
/// reads a source's `min-arrival-separation` in rule 16 alone — rule 8 reports it on the route, "an obligation, never a
/// finding" — and no requester of these universes constrains it (asserted), so the search over the twin, which rule 16
/// never refuses, is the search without rule 16 over the description, and its `bind` lines are the unamended rules'.
fn admitting_twin(description: &Description) -> Description {
    let mut sides: Vec<Side> = vec![description.own.clone()];
    sides.extend(description.tasks.iter().map(|t| t.side.clone()));
    sides.extend(description.services.iter().map(|s| s.side.clone()));
    sides.extend(description.hardware().into_iter().map(|b| b.side));
    assert!(
        sides.iter().all(|side| {
            !side.needs.iter().any(|f| f == "min-arrival-separation")
                && !side
                    .at_most
                    .iter()
                    .chain(&side.exactly)
                    .any(|(f, _)| f == "min-arrival-separation")
                && !side
                    .other
                    .iter()
                    .any(|clause| clause.contains("min-arrival-separation"))
        }),
        "a requester constrains `min-arrival-separation`: the twin's search is not the unamended one"
    );
    let admitting = Dur::Ms(ADMITTING_MS)
        .seconds()
        .expect("within the arithmetic");
    for task in &description.tasks {
        if let Release::MinSeparation(apart) = task.release {
            if let Time::Seconds(own) = apart.time() {
                assert!(
                    own <= admitting,
                    "`{}` is released less often than {ADMITTING_MS} ms",
                    task.name
                );
            }
        }
    }
    fn admit(block: &mut Block) {
        if block.separation.is_none() {
            return;
        }
        block.separation = Some(Separation::Stated(ADMITTING_MS));
        block
            .offers
            .retain(|offer| !offer.starts_with("(min-arrival-separation "));
        let at = block
            .offers
            .iter()
            .position(|offer| offer == "(interrupt-source true)")
            .map_or(block.offers.len(), |at| at + 1);
        block
            .offers
            .insert(at, format!("(min-arrival-separation {ADMITTING_MS} ms)"));
        block.absent.retain(|fact| fact != "min-arrival-separation");
    }
    let mut twin = description.clone();
    twin.blocks.iter_mut().for_each(admit);
    twin.platforms.iter_mut().for_each(admit);
    twin
}

/// The class of the systems for which a record binding a sporadic task to a source the analysis does not admit is
/// judged — every assignment of the tasks' own sources, each taking one its description offers, over the record the
/// unamended rules gave ([`rule_16_records`]), but the search's: "the forms the unamended rules let through" (§2.2,
/// R29-R2), which every input reaching one of [`OWN_CAUSES_UNREACHABLE`] had before rule 16 landed. Rule 16 refuses
/// each "in the model, the generator's joint rules and the checker's" (`RTP-H24`): the configuration checker, under its
/// check (5), `plan-joint` — "a sporadic task bound itself to a source faster than it, unbounded, or stating no
/// separation (rule 16)" — a record whose bindings do not hold together, which "is `tool-failure`"
/// (`docs/book/src/plan-check.md`); and the lowering, under §1's "the generator's own joint rules, run over the record
/// the checker accepted, find a conflict, an unknown or a map finding: `tool-failure`, naming both verdicts, the
/// checker's acceptance and each of the generator's findings — its conflicts, then its unknowns". Each names each such
/// task with its source and the code rule 16 gives, the two agreeing. No finding: no search writes these records, and
/// both roles refuse each, as they must.
///
/// ⚠️ The lowering does not read the checker's result record yet (R16-R6), so it is held to the bullet of the
/// generator's joint rules; once it does, §1's earlier "its status is not `ok`: `tool-failure`, naming both verdicts"
/// refuses each record first, under the same code.
const RULE_16_RECORD: &str =
    "rule 16: a record binding a task's own source the analysis does not admit, refused by the configuration checker \
     and by the lowering's joint rules, the codes agreeing";
/// Of [`RULE_16_RECORD`]: the record the unamended rules gave — the search's over [`admitting_twin`], its `bind` lines
/// over this description's program — binding one, the record that reached an own cause before rule 16 landed.
const RULE_16_UNAMENDED: &str =
    "rule 16: among them the record the unamended rules gave, the search's with every source admitted";
/// Of [`RULE_16_RECORD`]: a task's own source stating a separation shorter than the task's, a conflict — the input of
/// [`OWN_FASTER`].
const RULE_16_SHORTER: &str =
    "rule 16: such a record, a task's own source stating a separation shorter than the task's, `unsupported-profile`";
/// Of [`RULE_16_RECORD`]: a task's own source declaring its separation absent, a conflict — the input of
/// [`OWN_ABSENT`].
const RULE_16_ABSENT: &str =
    "rule 16: such a record, a task's own source declaring its separation absent, `unsupported-profile`";
/// Of [`RULE_16_RECORD`]: a task's own source stating no separation, an unknown — the input of [`OWN_UNSTATED`].
const RULE_16_UNSTATED: &str =
    "rule 16: such a record, a task's own source stating no separation, `missing-fact`";
/// Of [`RULE_16_RECORD`]: a task's own source's separation past the exact arithmetic, "a comparison the exact
/// arithmetic cannot decide", a conflict.
const RULE_16_UNDECIDABLE: &str =
    "rule 16: such a record, a task's own source's separation past the exact arithmetic, `unsupported-profile`";
/// Of [`RULE_16_RECORD`]: two tasks refused, one by a conflict and one by an unknown, §1 naming the conflict first.
const RULE_16_CONFLICT_BEFORE_UNKNOWN: &str =
    "rule 16: such a record, two tasks refused, the conflict named before the unknown";
/// Of [`RULE_16_RECORD`]: two tasks bound themselves to one source, the one it admits beside the one released less
/// often, refused alone — among its inputs, [`OWN_ONE_RELEASED_LESS_OFTEN`]'s.
const RULE_16_ONE_OF_TWO: &str =
    "rule 16: such a record, two tasks bound themselves to one source, one admitted, the one released less often \
     alone refused";
/// A system of [`RULE_16_UNAMENDED`] the search resolves, binding the task to another source the analysis admits.
const RULE_16_REBOUND: &str =
    "rule 16: the search binds the task to a source the analysis admits, the unamended rules to one it does not";
/// The search refusing a system rule 16 leaves no assignment, `unsupported-profile`: a task's every source a conflict.
const RULE_16_SEARCH_CONFLICT: &str =
    "rule 16: the search refuses, `unsupported-profile`, a task's every source a conflict";
/// The search refusing a system rule 16 leaves no assignment, `missing-fact`: some assignment blocked by unknowns alone.
const RULE_16_SEARCH_UNKNOWN: &str =
    "rule 16: the search refuses, `missing-fact`, a task's every source a conflict or stating no separation";

/// The codes of the status vocabulary a finding of the joint rules carries (model §8 rule 6).
const JOINT_CODES: [&str; 4] = [
    "infeasible-configuration",
    "invalid-description",
    "missing-fact",
    "unsupported-profile",
];

/// What one finding of rule 16 in a message names, read by its names alone, never its words (round 20, R20-R7): each
/// `(task SYSTEM NAME)` form ([`subjects_named`]), each block or platform the description declares written as code
/// ([`names_mentioned`]), and each code of [`JOINT_CODES`] written as a word of its own — one of each, the task, the
/// source and the code, or `Err` saying what else the clause names. A clause naming none of them is `Ok(None)`.
fn rule_16_named(
    clause: &str,
    description: &Description,
) -> Result<Option<(String, String, String)>, String> {
    let tasks: Vec<String> = subjects_named(clause, &description.system)
        .into_iter()
        .filter(|subject| subject.starts_with("(task "))
        .collect();
    let hardware: BTreeSet<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let mut sources: Vec<String> = Vec::new();
    for name in names_mentioned(clause, &hardware) {
        if !sources.contains(&name) {
            sources.push(name);
        }
    }
    let codes: Vec<&str> = words_outside_instances(clause)
        .iter()
        .map(|word| word.trim_end_matches([':', '.', ',']))
        .filter_map(|word| JOINT_CODES.iter().find(|code| **code == word).copied())
        .collect();
    match (tasks.as_slice(), sources.as_slice(), codes.as_slice()) {
        ([], [], []) => Ok(None),
        ([task], [source], [code]) => Ok(Some((task.clone(), source.clone(), (*code).to_string()))),
        _ => Err(format!(
            "a finding names the tasks {tasks:?}, the sources {sources:?} and the codes {codes:?}, not one of each: \
             {clause:?}"
        )),
    }
}

/// Where a message does not name the generator's findings of rule 16 as §1 orders them: clause by clause, each naming a
/// task, its source and a code ([`rule_16_named`]) — "its conflicts, then its unknowns ..., each group in the order the
/// joint rules give it", an order the record leaves to the joint rules within a group, so each group is held whole,
/// and the groups in that order.
fn joint_disagrees(
    message: &str,
    description: &Description,
    findings: &[Unadmitted],
) -> Option<String> {
    let mut wrong = Vec::new();
    let mut named = Vec::new();
    for clause in message.split(';') {
        match rule_16_named(clause, description) {
            Ok(Some(finding)) => named.push(finding),
            Ok(None) => {}
            Err(why) => wrong.push(why),
        }
    }
    let group = |findings: &[(String, String, String)], code: &str| {
        let mut group: Vec<(String, String, String)> =
            findings.iter().filter(|f| f.2 == code).cloned().collect();
        group.sort();
        group
    };
    let wanted: Vec<(String, String, String)> = findings
        .iter()
        .map(|u| (u.task.clone(), u.source.clone(), u.code.to_string()))
        .collect();
    let conflicts_first = named
        .windows(2)
        .all(|w| !(w[0].2 == "missing-fact" && w[1].2 == "unsupported-profile"));
    if !conflicts_first
        || named.len() != wanted.len()
        || JOINT_CODES
            .iter()
            .any(|code| group(&named, code) != group(&wanted, code))
    {
        wrong.push(format!(
            "it names {named:?}, each a task, its source and a code, where the generator's joint rules give \
             {wanted:?}, its conflicts, then its unknowns"
        ));
    }
    (!wrong.is_empty()).then(|| wrong.join("; "))
}

/// Where the configuration checker's refusal of a record is not rule 16's of `expected`: its status `tool-failure`,
/// "A corrupted plan is `tool-failure`" (`docs/book/src/plan-check.md`); each finding of its check (5), `plan-joint`,
/// saying "(rule 16)", and naming one task, its source and one code; and the findings those of `expected`, each once,
/// whatever their order — the checker's order no record fixes.
fn rule_16_checker_disagrees(
    refused: &Checked,
    description: &Description,
    expected: &[Unadmitted],
) -> Option<String> {
    let mut wrong = Vec::new();
    if refused.status != "tool-failure" {
        wrong.push(format!(
            "its status is `{}`, not `tool-failure`",
            refused.status
        ));
    }
    let mut named = Vec::new();
    for (code, message) in &refused.findings {
        if code != "plan-joint" || !message.contains("(rule 16)") {
            wrong.push(format!(
                "a finding `{code}` not of its check (5) under rule 16: {message}"
            ));
            continue;
        }
        match rule_16_named(message, description) {
            Ok(Some(finding)) => named.push(finding),
            Ok(None) => wrong.push(format!("a finding naming no task: {message}")),
            Err(why) => wrong.push(why),
        }
    }
    named.sort();
    let mut wanted: Vec<(String, String, String)> = expected
        .iter()
        .map(|u| (u.task.clone(), u.source.clone(), u.code.to_string()))
        .collect();
    wanted.sort();
    if named != wanted {
        wrong.push(format!(
            "it names {named:?}, each a task, its source and a code, where rule 16 gives {wanted:?}"
        ));
    }
    (!wrong.is_empty()).then(|| wrong.join("; "))
}

/// The search over the twin `text` ([`admitting_twin`]): its record and its `(program …)` line, or the search's
/// verdict. A universe's systems that differ only in their sources' separations share one twin, so each twin is
/// searched once, whichever worker meets it first.
fn twin_searched(text: &str, system: &str) -> Result<(String, String), String> {
    type Searched = BTreeMap<(String, String), Result<(String, String), String>>;
    static SEARCHED: std::sync::OnceLock<std::sync::Mutex<Searched>> = std::sync::OnceLock::new();
    let cache = SEARCHED.get_or_init(Default::default);
    let key = (text.to_string(), system.to_string());
    if let Some(searched) = cache.lock().expect("the twins").get(&key) {
        return searched.clone();
    }
    let program = common::checked(text).expect("the language accepts the twin");
    let searched = searched_record(&program, system).map(|record| {
        (
            record,
            format!("(program \"{}\")", plan::program_digest(&program)),
        )
    });
    cache
        .lock()
        .expect("the twins")
        .insert(key, searched.clone());
    searched
}

/// What the reading finds of the records binding a task's own source the analysis does not admit, for one system
/// ([`RULE_16_RECORD`]).
#[derive(Default)]
struct Rule16Records {
    /// The record the unamended rules gave, where it binds one, over the description's program.
    unamended: Option<String>,
    /// The records judged by the configuration checker and lowered.
    judged: usize,
    /// Of them, those both refuse as rule 16 says.
    refused: usize,
    classes: BTreeSet<&'static str>,
    findings: Vec<String>,
}

/// Every record binding a task's own source the analysis does not admit that `case`'s system can be given, but
/// `searched`, the search's record (none where the search refused): the record the unamended rules gave — the search's
/// over [`admitting_twin`], its `bind` lines over this description's program — with each sporadic task binding
/// `interrupt-source` itself bound in turn to each source of [`own_source_candidates`], every assignment of them, the
/// `/2` record's other lines unchanged. Each binding one rule 16 refuses is judged by the configuration checker and
/// lowered, each held to rule 16's findings on it. `None` where no task's own source may be one the analysis does not
/// admit.
fn rule_16_records(universe: &str, case: &Case, searched: Option<&str>) -> Option<Rule16Records> {
    let description = &case.description;
    if language_rejects(description) || !rule_16_may_refuse(description) {
        return None;
    }
    let text = description.text();
    let mut out = Rule16Records::default();
    let reproducer = |record: &str, what: &str| {
        format!(
            "== {universe}: {}\n-- the description\n{text}-- a record binding a task's own source, the unamended \
             rules' with each such task bound in turn to each source\n{record}{what}",
            case.label
        )
    };
    let (twin_record, twin_line) = match twin_searched(
        &admitting_twin(description).text(),
        &description.system,
    ) {
        Ok(searched) => searched,
        Err(verdict) => {
            out.findings.push(reproducer(
                    "",
                    &format!(
                        "-- the search over the description with every source stating {ADMITTING_MS} ms resolves \
                         no record: {verdict}\n"
                    ),
                ));
            return Some(out);
        }
    };
    let program = common::checked(&text).expect("the language accepts the description");
    assert_eq!(
        twin_record.matches(&twin_line).count(),
        1,
        "one program line"
    );
    let unamended = twin_record.replace(
        &twin_line,
        &format!("(program \"{}\")", plan::program_digest(&program)),
    );
    // Every assignment of the tasks' own sources, each the record's line `(bind TASK interrupt-source SOURCE)` with
    // another source, the `/2` record's order unchanged: its lines are its bindings', one for each requirement.
    let system = description.system.as_str();
    let mut records = vec![unamended.clone()];
    for (task, sources) in own_source_candidates(description) {
        let opening_text = format!("(bind (task {system} {}) {INTERRUPT_FACT} ", task.name);
        let opening = opening_text.as_str();
        records = records
            .iter()
            .flat_map(|record| {
                assert_eq!(
                    record.lines().filter(|l| l.starts_with(opening)).count(),
                    1,
                    "the unamended rules bind `{}`'s own source",
                    task.name
                );
                sources.iter().map(move |source| {
                    record
                        .lines()
                        .map(|line| {
                            if line.starts_with(opening) {
                                format!("{opening}{})\n", source.name)
                            } else {
                                format!("{line}\n")
                            }
                        })
                        .collect::<String>()
                })
            })
            .collect();
    }
    assert!(
        records.contains(&unamended),
        "the unamended rules' record among every assignment"
    );
    let searched_binds = searched.map(binds_of);
    for record in records {
        let binds = binds_of(&record);
        let refused = unadmitted(description, &binds);
        if refused.is_empty() || searched_binds.as_ref() == Some(&binds) {
            continue;
        }
        out.judged += 1;
        let mut agreeing = true;
        match judged_by_the_checker(&text, &record) {
            None => {
                agreeing = false;
                out.findings.push(reproducer(
                    &record,
                    &format!(
                        "-- the configuration checker accepts it, where model §8 rule 16 refuses {refused:?} (its \
                         check (5))\n"
                    ),
                ));
            }
            Some(checked) => {
                if let Some(wrong) = rule_16_checker_disagrees(&checked, description, &refused) {
                    agreeing = false;
                    out.findings.push(reproducer(
                        &record,
                        &format!(
                            "-- the configuration checker refuses it: {checked}\n-- {wrong}\n"
                        ),
                    ));
                }
            }
        }
        let due = refusal_the_record_gives(description, &binds)
            .expect("§1's bullet of the generator's joint rules, rule 16 refusing the record");
        match lower(&program, &record, &common::vocabulary(), &common::catalog()) {
            Ok(plan) => {
                agreeing = false;
                out.findings.push(reproducer(
                    &record,
                    &format!(
                        "-- the code writes\n{}-- the record refuses it: `{}`, naming {} (§1)\n",
                        plan.text(),
                        due.code,
                        due.summary()
                    ),
                ));
            }
            Err(refusal) => {
                if let Some(wrong) = refusal_disagrees(&due, &refusal, description) {
                    agreeing = false;
                    out.findings.push(reproducer(
                        &record,
                        &format!(
                            "-- the code writes\nrefused {refusal}\n-- {wrong} (§1: `{}`, naming {})\n",
                            due.code,
                            due.summary()
                        ),
                    ));
                }
            }
        }
        if !agreeing {
            continue;
        }
        out.refused += 1;
        out.classes.insert(RULE_16_RECORD);
        for u in &refused {
            out.classes.insert(match u.cause {
                "shorter" => RULE_16_SHORTER,
                "absent" => RULE_16_ABSENT,
                "unstated" => RULE_16_UNSTATED,
                _ => RULE_16_UNDECIDABLE,
            });
        }
        if refused.iter().any(|u| u.code == "unsupported-profile")
            && refused.iter().any(|u| u.code == "missing-fact")
        {
            out.classes.insert(RULE_16_CONFLICT_BEFORE_UNKNOWN);
        }
        // Two tasks bound themselves to one source, one of them admitted.
        if refused.iter().any(|u| {
            binds.iter().any(|b| {
                b.provider == u.source
                    && b.requester != u.task
                    && b.requester.starts_with(&format!("(task {system} "))
                    && relies_on_source(description, b)
                    && !refused.iter().any(|r| r.task == b.requester)
            })
        }) {
            out.classes.insert(RULE_16_ONE_OF_TWO);
        }
        if record == unamended {
            out.classes.insert(RULE_16_UNAMENDED);
            out.unamended = Some(record);
            if searched.is_some() {
                out.classes.insert(RULE_16_REBOUND);
            }
        }
    }
    Some(out)
}

// =====================================================================================================================
// Reading a plan's lines
// =====================================================================================================================

/// A form of a plan's line.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Sx {
    Atom(String),
    List(Vec<Sx>),
}

/// `line`, one form of canonical text (§3).
fn form(line: &str) -> Sx {
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    let mut atom = String::new();
    for ch in line.chars() {
        if matches!(ch, '(' | ')' | ' ') {
            if !atom.is_empty() {
                stack
                    .last_mut()
                    .expect("open")
                    .push(Sx::Atom(std::mem::take(&mut atom)));
            }
            if ch == '(' {
                stack.push(Vec::new());
            } else if ch == ')' {
                let done = stack.pop().expect("balanced");
                stack.last_mut().expect("balanced").push(Sx::List(done));
            }
        } else {
            atom.push(ch);
        }
    }
    assert!(atom.is_empty() && stack.len() == 1, "one form: {line}");
    stack
        .pop()
        .expect("one")
        .into_iter()
        .next()
        .expect("a form")
}

/// A form's canonical text (§3).
fn text_of(sx: &Sx) -> String {
    match sx {
        Sx::Atom(atom) => atom.clone(),
        Sx::List(items) => list(&text_of(&items[0]), items[1..].iter().map(text_of)),
    }
}

fn items(sx: &Sx) -> &[Sx] {
    match sx {
        Sx::List(items) => items,
        Sx::Atom(atom) => panic!("a list, not `{atom}`"),
    }
}

/// The plan's `bind` lines.
fn binds_of(plan: &str) -> Vec<Bind> {
    plan.lines()
        .filter(|line| line.starts_with("(bind "))
        .map(|line| {
            let form = form(line);
            let items = items(&form);
            assert_eq!(items.len(), 4, "{line}");
            Bind {
                requester: text_of(&items[1]),
                fact: text_of(&items[2]),
                provider: text_of(&items[3]),
            }
        })
        .collect()
}

/// §3's forms, by head, in "The forms come in the order below": the order of a plan's lines across kinds, held to §3's
/// own list by `the_reading_gives_the_record_s_worked_texts` — `node` among them since round 24 (R24-D2).
const FORMS: [&str; 20] = [
    "format",
    "profile",
    "system",
    "program",
    "assignment",
    "adapters",
    "task",
    "bind",
    "instance",
    "timer",
    "route",
    "enabled",
    "interrupt-model",
    "node",
    "context",
    "depends",
    "step",
    "obligation",
    "owed",
    "complete",
];

/// A line's form, by its place in [`FORMS`], or `None` where §3 writes no form of its head.
fn form_rank(line: &str) -> Option<usize> {
    let head = line
        .strip_prefix('(')
        .and_then(|rest| rest.split([' ', ')']).next())?;
    FORMS.iter().position(|form| *form == head)
}

/// The kinds of line the reading derives, as the checker reports them: every line of a plan, and `unknown`, the lines
/// of a head §3 does not write, of which the reading expects none (round 13, R13-R3: such a line is a disagreement,
/// never dropped).
const KINDS_OF_LINE: [&str; 17] = [
    "identity",
    "task",
    "bind",
    "node",
    "context",
    "depends",
    "step",
    "multiplex",
    "route",
    "enabled",
    "timer",
    "instance",
    "obligation",
    "owed",
    "interrupt-model",
    "complete",
    "unknown",
];

/// Which of the kinds `line` is: `multiplex` is the `(obligation multiplex …)` lines and the `overhead` they owe
/// (§2.9); `unknown`, a line of a head §3 does not write.
fn kind_of_line(line: &str) -> &'static str {
    let Some(rank) = form_rank(line).filter(|_| one_form(line)) else {
        return "unknown";
    };
    match FORMS[rank] {
        "format" | "profile" | "system" | "program" | "assignment" | "adapters" => "identity",
        "obligation" if line.starts_with("(obligation multiplex ") => "multiplex",
        // `(owed SUBJECT FACT (by OWNER))`: the fact is the word before its last form.
        "owed" => {
            let fact = line
                .rsplit_once(" (by ")
                .and_then(|(before, _)| before.rsplit(' ').next());
            if fact == Some("overhead") {
                "multiplex"
            } else {
                "owed"
            }
        }
        other => KINDS_OF_LINE
            .iter()
            .copied()
            .find(|kind| *kind == other)
            .expect("each other form a kind of its own"),
    }
}

/// Whether `line` is one balanced form, `(` first and the `)` closing it last (§3: "each line is one form").
fn one_form(line: &str) -> bool {
    let mut depth = 0usize;
    for (at, ch) in line.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
                if depth == 0 && at + 1 != line.len() {
                    return false;
                }
            }
            _ if depth == 0 => return false,
            _ => {}
        }
    }
    depth == 0 && line.starts_with('(')
}

fn by_kind<'a>(lines: impl IntoIterator<Item = &'a str>) -> BTreeMap<&'static str, Vec<String>> {
    let mut out: BTreeMap<&'static str, Vec<String>> =
        KINDS_OF_LINE.iter().map(|k| (*k, Vec::new())).collect();
    for line in lines {
        out.get_mut(kind_of_line(line))
            .expect("a kind")
            .push(line.to_string());
    }
    out
}

// =====================================================================================================================
// The checker
// =====================================================================================================================

/// What the reading and the plan say of one kind of line, where they differ.
struct Disagreement {
    kind: &'static str,
    expected: Vec<String>,
    written: Vec<String>,
}

/// What judging one plan gave.
struct Judged {
    /// The kinds of line the plan agrees with the reading on.
    agreed: Vec<&'static str>,
    disagreements: Vec<Disagreement>,
    classes: BTreeSet<&'static str>,
}

/// The plan's identity (§2.1, §3): "the format, the profile, the system, the program's digest, the assignment's sha256
/// and the adapter catalog's version", each derived, none copied from the record lowered (round 20, R20-R7): the format
/// `/1`; the profile `rt-static-up-v1`, "the one whose rules `/1` lowers" (§1), its identifier as `/2` writes it (§3);
/// the system lowered; the digest of the program the language's check accepted, which the lowering "computes
/// (`eadl_resolve::plan::program_digest`), never taking either from its caller" (§1) — `program`, computed by that
/// function, the one value taken from outside the record's text, whose algorithm is the assignment format's (§1's "its
/// program digest is not the program's"); the sha256 of the record's text, "the assignment's sha256"; and this engine's
/// catalog's version (§1: "its `(adapters …)` names a catalog other than this engine's: `stale-input`"),
/// `eadl_resolve::adapter::VERSION`. Where `elided`, the two digests are written `…`, as the record's worked texts write
/// them.
fn identity(system: &str, program: &str, record: &str, elided: bool) -> Vec<String> {
    let (program, assignment) = if elided {
        (
            "(program \"…\")".to_string(),
            "(assignment \"…\")".to_string(),
        )
    } else {
        (
            format!("(program \"{program}\")"),
            format!("(assignment \"{}\")", Digest::of(record.as_bytes()).hex()),
        )
    };
    vec![
        "(format \"archogen-runtime-plan/1\")".to_string(),
        "(profile rt-static-up-v1)".to_string(),
        format!("(system {system})"),
        program,
        assignment,
        format!("(adapters {})", eadl_resolve::adapter::VERSION),
    ]
}

/// One system's plan judged against the reading, which takes the assignment from `record`, the `/2` record lowered:
/// §2.3's `bind` lines are that record's, "copied as written" — the one part of a plan the reading copies, the
/// assignment being the lowering's input (§1) — and `program` is the digest of the program the language accepted.
fn judge(description: &Description, program: &str, record: &str, plan: &str) -> Judged {
    judge_elided(description, program, record, plan, false)
}

/// [`judge`], the identity's two digests written `…` where `elided`.
fn judge_elided(
    description: &Description,
    program: &str,
    record: &str,
    plan: &str,
    elided: bool,
) -> Judged {
    let reading = read_the_record(description, &binds_of(record));
    let mut expected = by_kind(reading.lines.iter().map(String::as_str));
    expected.insert(
        "identity",
        identity(&description.system, program, record, elided),
    );
    expected.insert(
        "bind",
        record
            .lines()
            .filter(|line| line.starts_with("(bind "))
            .map(str::to_string)
            .collect(),
    );
    let written = by_kind(plan.lines());
    let mut agreed = Vec::new();
    let mut disagreements = Vec::new();
    if !reading.findings.is_empty() {
        disagreements.push(Disagreement {
            kind: "nodes",
            expected: reading.findings.clone(),
            written: Vec::new(),
        });
    }
    for kind in KINDS_OF_LINE {
        if expected[kind] == written[kind] {
            agreed.push(kind);
        } else {
            disagreements.push(Disagreement {
                kind,
                expected: expected[kind].clone(),
                written: written[kind].clone(),
            });
        }
    }
    // The whole text, every line in its place (round 13, R13-R3): §3's "The forms come in the order below", each form's
    // lines in the order §3 gives within it. Each form's lines but `obligation`'s and `owed`'s are of one kind, already
    // in their order, which a stable sort by form keeps; those two forms' lines, `multiplex`'s among them, are "by their
    // text". A line of a head §3 does not write has no place: the reading expects none, so a plan holding one differs
    // here too.
    let by_text = |line: &str| {
        matches!(
            form_rank(line).map(|r| FORMS[r]),
            Some("obligation" | "owed")
        )
    };
    let mut whole: Vec<String> = expected
        .iter()
        .filter(|(kind, _)| **kind != "unknown")
        .flat_map(|(_, lines)| lines.iter().cloned())
        .collect();
    // `sort_by_cached_key` is stable: it sorts each key with its index.
    whole.sort_by_cached_key(|line| (form_rank(line), by_text(line).then(|| line.clone())));
    // §3: "each line is one form and ends with a single newline, the last line included", "and no blank line": the
    // plan's bytes, not its lines alone.
    let canonical: String = whole.iter().map(|line| format!("{line}\n")).collect();
    let lines: Vec<String> = plan.split('\n').map(str::to_string).collect();
    if plan == canonical {
        agreed.push("whole text");
    } else {
        // The text split at each newline: its last piece empty where it ends with one.
        let wanted: Vec<String> = canonical.split('\n').map(str::to_string).collect();
        let at = lines
            .iter()
            .zip(&wanted)
            .position(|(l, w)| l != w)
            .unwrap_or_else(|| lines.len().min(wanted.len()));
        disagreements.push(Disagreement {
            kind: "whole text",
            expected: std::iter::once(format!("from line {}:", at + 1))
                .chain(wanted.iter().skip(at).take(3).cloned())
                .collect(),
            written: std::iter::once(format!("from line {}:", at + 1))
                .chain(lines.iter().skip(at).take(3).cloned())
                .collect(),
        });
    }
    Judged {
        agreed,
        disagreements,
        classes: reading.classes,
    }
}

/// What the engine did with one system, run as `common::lowered` runs it — the language's check, the search, the
/// `/2` writer and the lowering — with the `archogen-assignment/2` record kept and judged by the configuration checker,
/// `archogen_plan_check::check::plan_file`, as `crates/eadl-resolve/tests/plan.rs`'s `resolved_and_judged` judges one.
enum Flow {
    /// The language's check refused the description: its verdict. Such a description is no input (§1).
    Rejected(String),
    /// The search resolved no plan: its verdict.
    Unresolved(String),
    /// The search resolved one.
    Resolved {
        /// The record the search's writer wrote.
        record: String,
        /// The digest of the program the language's check accepted (§1: the lowering "computes its digest
        /// (`eadl_resolve::plan::program_digest`), never taking either from its caller").
        program: String,
        /// The configuration checker's verdict on it, where it refuses.
        checker: Option<Checked>,
        /// The plan's text, or the lowering's refusal.
        lowered: Result<String, Refusal>,
    },
}

/// The configuration checker's refusal of a record: its status and each finding, its code and its message.
#[derive(Debug)]
struct Checked {
    status: String,
    findings: Vec<(String, String)>,
}

impl std::fmt::Display for Checked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status)?;
        for (code, message) in &self.findings {
            write!(f, "; {code}: {message}")?;
        }
        Ok(())
    }
}

fn flow(text: &str, system: &str) -> Flow {
    let program = match common::checked(text) {
        Ok(program) => program,
        Err(verdict) => return Flow::Rejected(verdict.to_string()),
    };
    let record = match searched_record(&program, system) {
        Ok(record) => record,
        Err(verdict) => return Flow::Unresolved(verdict),
    };
    let checker = judged_by_the_checker(text, &record);
    let lowered = lower(&program, &record, &common::vocabulary(), &common::catalog())
        .map(|plan| plan.text().to_owned());
    Flow::Resolved {
        record,
        program: plan::program_digest(&program),
        checker,
        lowered,
    }
}

/// `program`'s `system` searched as the engine searches it, at the profile's caller level with the shipped catalog, and
/// its `/2` record written: the record, or the search's verdict.
fn searched_record(program: &Program, system: &str) -> Result<String, String> {
    let vocabulary = common::vocabulary();
    let catalog = common::catalog();
    let declarations = &program.root().declarations;
    let decl = declarations
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && f.items().get(1).and_then(Form::as_symbol) == Some(system)
        })
        .expect("the system")
        .clone();
    let policy = Policy {
        adapters: catalog.clone(),
        ..Policy::default()
    };
    let resolution = match search(
        declarations,
        &decl,
        RT_STATIC_UP_V1.caller_level,
        &vocabulary,
        &policy,
        Limits::DEFAULT,
    ) {
        Search::Resolved(resolution) => resolution,
        Search::Refused { verdict, .. } => return Err(format!("{verdict:?}")),
        Search::Inconclusive { .. } => return Err("inconclusive".to_string()),
    };
    Ok(plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(program),
    ))
}

/// The configuration checker's verdict on `record` over the description `text`, as
/// `crates/eadl-resolve/tests/plan.rs`'s `resolved_and_judged` judges one: its refusal, or `None` where it accepts.
fn judged_by_the_checker(text: &str, record: &str) -> Option<Checked> {
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &DirectoryModules::new(common::root()),
        &RT_STATIC_UP_V1,
        "plan.json",
        record,
    );
    verdict.err().map(|refused| Checked {
        status: refused.status.slug().to_string(),
        findings: refused
            .findings
            .iter()
            .map(|f| (f.code.to_string(), f.message.to_string()))
            .collect(),
    })
}

/// What a refusal §1 gives names.
enum Names {
    /// One task, `(task SYSTEM NAME)`: "a bullet over tasks is taken task by task in runtime order", and the first
    /// task it applies to is the one given; with the clause, where §1 names one — "naming the task and its `period`"
    /// (round 14, R14-R5).
    Task {
        task: String,
        clause: Option<&'static str>,
    },
    /// The naming-system refusal: each requester the system serves that names a system, with the system it names —
    /// "naming each such name, by the namer's text and then the system's" (round 13, R13-R1), texts compared byte by
    /// byte (§2).
    Systems(Vec<(String, String)>),
    /// The refusal of hardware naming what is not hardware: each block or platform the system serves whose `uses` or
    /// `needs` names a declaration that is neither, with that operand — "naming each, by the block's or platform's text
    /// and then the operand's".
    Operands(Vec<(String, String)>),
    /// The durations past the arithmetic, "naming each — the tasks' in runtime order, then the sources' by their
    /// interrupt's text, then the bounds by their instance's text and then the fact's", each task's clauses "in the
    /// order the bullet writes them": a task's as `(task SYSTEM NAME) CLAUSE`, a source's as `(source NAME)`, a
    /// bound's as `INSTANCE FACT`.
    Durations(Vec<String>),
    /// The interrupt-code refusal: every receiver, in §1's order — "each task in runtime order, then the system's own
    /// level" — each with what it would receive.
    Receivers(Vec<Receiver>),
    /// "the system declares no task".
    NoTask,
    /// The generator's joint rules refusing the record: "naming both verdicts, the checker's acceptance and each of the
    /// generator's findings — its conflicts, then its unknowns, then the map's refusals and the map's unknowns, each
    /// group in the order the joint rules give it" — of model §8 rule 16, each task bound itself to a source the
    /// analysis does not admit, with that source and the code rule 16 gives ([`unadmitted`]'s order).
    Joint(Vec<Unadmitted>),
}

/// One receiver of interrupt code (§1): a task's `(task SYSTEM NAME)`, or the system's name for its own level, with
/// what it would receive — "first the alarms it programs, each naming the provider, then its expiries, each naming
/// the instance that delivers it, each kind by those texts" (round 14, R14-R6). A set iterates by its texts, each
/// compared byte by byte; an alarm is one for each provider, "an interrupt ... named by its kind and its provider"
/// (§2), however many consumed facts the receiver binds there.
struct Receiver {
    subject: String,
    alarms: BTreeSet<String>,
    expiries: BTreeSet<String>,
}

impl Receiver {
    /// What it would receive, in §1's order: its alarms' providers, then its expiries' instances.
    fn received(&self) -> impl Iterator<Item = &String> {
        self.alarms.iter().chain(&self.expiries)
    }
}

/// A refusal §1 gives one system.
struct Due {
    code: &'static str,
    names: Names,
    /// The classes of §1's rules it exercises.
    classes: BTreeSet<&'static str>,
}

impl Due {
    /// What it names, for a reproducer.
    fn summary(&self) -> String {
        match &self.names {
            Names::Task { task, clause } => {
                clause.map_or_else(|| task.clone(), |clause| format!("{task} {clause}"))
            }
            Names::Systems(pairs) | Names::Operands(pairs) => pairs
                .iter()
                .map(|(namer, named)| format!("{namer} naming {named}"))
                .collect::<Vec<_>>()
                .join(", "),
            Names::NoTask => "the system's lack of a task".to_string(),
            Names::Joint(findings) => findings
                .iter()
                .map(|u| format!("{} bound to {}, `{}`", u.task, u.source, u.code))
                .collect::<Vec<_>>()
                .join(", then "),
            Names::Durations(items) => items.join(", "),
            Names::Receivers(receivers) => receivers
                .iter()
                .map(|r| {
                    format!(
                        "{} receiving {}",
                        r.subject,
                        r.received().cloned().collect::<Vec<_>>().join(" then ")
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

/// Whether one of `texts` is a proper prefix of another (round 14, R14-R1: "names one of which prefixes another").
fn prefixed<'a>(texts: impl IntoIterator<Item = &'a String> + Clone) -> bool {
    texts.clone().into_iter().any(|a| {
        texts
            .clone()
            .into_iter()
            .any(|b| b.len() > a.len() && b.starts_with(a.as_str()))
    })
}

/// Whether `text` holds a byte a symbol may hold below `)` — `!`, `#`, `$`, `%`, `&` or `'`
/// (`docs/semantics/grammar.md`'s `symbol_char`) — so that a name continuing with it sorts before the `)` that closes a
/// shorter one's form.
fn below_paren(text: &str) -> bool {
    text.bytes()
        .any(|b| matches!(b, b'!' | b'#' | b'$' | b'%' | b'&' | b'\''))
}

/// Whether `items`, held in the order §1 states — each by its text, compared byte by byte — would be ordered otherwise
/// by `worded`, the words a message names each with (round 14, R14-D1: "a closing backquote sorts after `.`, so `y.b`
/// is named before `y`").
fn worded_otherwise<T: Clone>(items: &[T], worded: impl Fn(&T) -> String) -> bool {
    let mut by_words = items.to_vec();
    by_words.sort_by_key(|item| worded(item));
    by_words.iter().map(&worded).collect::<Vec<_>>()
        != items.iter().map(&worded).collect::<Vec<_>>()
}

/// The classes of round 14 (R14-D1, R14-R1) a refusal naming pairs reaches, `pairs` held in §1's order — "by the
/// namer's text and then the system's" for `naming`, "by the block's or platform's text and then the operand's" for
/// `operand`: one namer's text a prefix of another's; one named text a prefix of another's; a name continuing with a
/// byte below `)`; tasks' texts, `(task SYSTEM NAME)`, ordered unlike their names; and an order the message's words
/// would give otherwise, each pair worded as `` `NAMER` names `OPERAND` ``.
fn name_order_classes(
    pairs: &[(String, String)],
    refusal: &str,
    classes: &mut BTreeSet<&'static str>,
) {
    let pick = |[naming, operand]: [&'static str; 2]| match refusal {
        "naming" => naming,
        _ => operand,
    };
    let namers: BTreeSet<&String> = pairs.iter().map(|(namer, _)| namer).collect();
    let named: BTreeSet<&String> = pairs.iter().map(|(_, named)| named).collect();
    if prefixed(namers.iter().copied()) {
        classes.insert(pick([
            "naming: one namer's text a prefix of another's",
            "operand: one namer's text a prefix of another's",
        ]));
    }
    if prefixed(named.iter().copied()) {
        classes.insert(pick([
            "naming: one system's text a prefix of another's",
            "operand: one operand's text a prefix of another's",
        ]));
    }
    if namers.iter().chain(&named).any(|text| below_paren(text)) {
        classes.insert(pick([
            "naming: a name continuing with a byte below `)`",
            "operand: a name continuing with a byte below `)`",
        ]));
    }
    let tasks: Vec<&String> = namers
        .iter()
        .copied()
        .filter(|namer| namer.starts_with("(task "))
        .collect();
    if worded_otherwise(&tasks, |task| {
        task.trim_end_matches(')')
            .rsplit(' ')
            .next()
            .expect("a name")
            .to_string()
    }) {
        classes.insert(pick([
            "naming: tasks' texts ordered unlike their names",
            // §1's hardware-naming bullet names "a block or a platform the system serves", never a task.
            "unreachable: a task among the namers of hardware naming what is not hardware, blocks and platforms alone",
        ]));
    }
    if worded_otherwise(pairs, |(namer, named)| format!("`{namer}` names `{named}`")) {
        classes.insert(pick([
            "naming: an order the message's words would give otherwise",
            "operand: an order the message's words would give otherwise",
        ]));
    }
}

/// §1: "a declaration or a task the system serves naming a system by `uses` or `needs`", each with the system it
/// names, "by the namer's text and then the system's". The requesters a system serves are "its own level, one of its
/// tasks, or a declaration its closure reaches through `uses` and `needs`" (model §8 rule 7) — round 12's R12-R1
/// scoped the refusal to them — and its hardware, §2's "a block or a platform the system serves". A system so reached
/// is a requester "for what it writes at its own level outside its tasks" (model §8 rule 1), its tasks being that
/// system's: its `requires`, and its `platform` clause, `(platform (uses P))`, whose `uses` the closure follows as it
/// follows the lowered system's own, through `P` to the blocks `P` composes (round 14's widening; the second widening
/// had stopped at its `requires`, which the joint rules do not).
fn systems_named(description: &Description) -> Vec<(String, String)> {
    let systems: BTreeMap<String, System> = description
        .systems()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect();
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut todo: Vec<(String, Side)> = description
        .tasks
        .iter()
        .map(|t| {
            (
                format!("(task {} {})", description.system, t.name),
                t.side.clone(),
            )
        })
        .chain(std::iter::once((
            description.system.clone(),
            description.own.clone(),
        )))
        .chain(
            system_hardware(description)
                .into_iter()
                .map(|name| (name.clone(), hardware[&name].side.clone())),
        )
        .collect();
    let mut seen = BTreeSet::new();
    let mut out = BTreeSet::new();
    while let Some((namer, side)) = todo.pop() {
        if !seen.insert(namer.clone()) {
            continue;
        }
        for named in side.uses.iter().chain(&side.needs) {
            if let Some(other) = systems.get(named) {
                out.insert((namer.clone(), named.clone()));
                let mut own = other.own.clone();
                own.uses.push(other.platform.clone());
                todo.push((named.clone(), own));
            } else if let Some(service) = description.services.iter().find(|s| &s.name == named) {
                todo.push((named.clone(), service.side.clone()));
            } else if let Some(block) = hardware.get(named) {
                todo.push((named.clone(), block.side.clone()));
            }
        }
    }
    // A set of pairs iterates by its first text, then its second, each compared byte by byte.
    out.into_iter().collect()
}

/// §1: "a block or a platform the system serves naming, by `uses` or `needs`, a declaration that is neither a block nor
/// a platform", "naming each, by the block's or platform's text and then the operand's". Since round 15 (R15-R2) it
/// takes "only what the system serves, as the bullet over tasks takes its tasks: nothing else reaches its plan", as the
/// naming-system bullet before it does: a block or a platform of another system, or of none, is read by neither.
fn hardware_naming_other_kinds(description: &Description) -> Vec<(String, String)> {
    let served = served_hardware(description);
    let hardware: BTreeSet<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let others: BTreeSet<String> = description
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(description.systems().into_iter().map(|s| s.name))
        .filter(|name| !hardware.contains(name))
        .collect();
    let mut out = BTreeSet::new();
    for block in description.hardware() {
        if !served.contains(&block.name) {
            continue;
        }
        for named in block.side.uses.iter().chain(&block.side.needs) {
            if others.contains(named) {
                out.insert((block.name.clone(), named.clone()));
            }
        }
    }
    out.into_iter().collect()
}

/// What the system's own level names: its `platform` clause, its `uses` and its `needs` (model §8 rule 1: "what it
/// writes at its own level outside its tasks").
fn own_level_names(description: &Description) -> Vec<String> {
    std::iter::once(description.platform.clone())
        .chain(description.own.uses.iter().cloned())
        .chain(description.own.needs.iter().cloned())
        .collect()
}

/// What the system's tasks name by `uses` or `needs`.
fn tasks_names(description: &Description) -> Vec<String> {
    description
        .tasks
        .iter()
        .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned())
        .collect()
}

/// The services and policies the closure reaches from `from` through `uses` and `needs` (model §8 rule 7), passing
/// through each service and policy it reaches; through each block and platform where `through_hardware` — hardware
/// "serving what it names" (§1); and through each system named where `through_systems`, "what a system it names writes
/// at its own level — its `platform` clause included — being served too" (§1), that system's tasks being its own.
fn services_reached(
    description: &Description,
    from: Vec<String>,
    through_hardware: bool,
    through_systems: bool,
) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let systems: BTreeMap<String, System> = description
        .systems()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect();
    let mut todo = from;
    let mut seen = BTreeSet::new();
    let mut out = BTreeSet::new();
    while let Some(named) = todo.pop() {
        if !seen.insert(named.clone()) {
            continue;
        }
        if let Some(service) = description.services.iter().find(|s| s.name == named) {
            todo.extend(service.side.uses.iter().chain(&service.side.needs).cloned());
            out.insert(named);
        } else if let Some(block) = hardware.get(&named).filter(|_| through_hardware) {
            todo.extend(block.side.uses.iter().chain(&block.side.needs).cloned());
        } else if let Some(system) = systems.get(&named).filter(|_| through_systems) {
            todo.extend(system.own.uses.iter().chain(&system.own.needs).cloned());
            todo.push(system.platform.clone());
        }
    }
    out
}

/// The services and policies the system serves: those "its closure reaches through `uses` and `needs`" from "its own
/// level, one of its tasks" (model §8 rule 7), the closure taking every declaration it reaches. A system named and
/// hardware naming a service are §1's refusals, given after the search, so the closure the search judges rule 7's
/// conflict over passes through both.
fn served_services(description: &Description) -> BTreeSet<String> {
    let mut from = own_level_names(description);
    from.extend(tasks_names(description));
    services_reached(description, from, true, true)
}

/// Model §8 rule 7 (`M1.49`, `RTP-H22`): "A service or a policy the system serves that names, by `uses` or `needs`, a
/// block or platform outside that hardware is no path into it, and the same conflict, judged once for the system
/// whatever the assignment: `infeasible-configuration` ... It is each serving system's, so a system the name leaves
/// inside its hardware is not refused for it." So each service or policy the system serves ([`served_services`]), with
/// each block or platform it names by `uses` or `needs` that is not of the system's hardware (rule 7,
/// [`system_hardware`]). Non-empty, the system is the search's to refuse before any assignment, whatever else its
/// description writes; a block such a block composes is served too, and named by no service or policy, so it adds no
/// pair; and a system a service names is no block or platform, so it adds none either (`M1.46`'s, and §1's until then).
fn outside_hardware(description: &Description) -> Vec<(String, String)> {
    let declared: BTreeSet<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let hardware = system_hardware(description);
    let served = served_services(description);
    let mut out = BTreeSet::new();
    for service in description
        .services
        .iter()
        .filter(|s| served.contains(&s.name))
    {
        for named in service.side.uses.iter().chain(&service.side.needs) {
            if declared.contains(named) && !hardware.contains(named) {
                out.insert((service.name.clone(), named.clone()));
            }
        }
    }
    out.into_iter().collect()
}

/// The classes of model §8 rule 7's conflict a system reaches, each read from what its description writes, the search
/// writing no record of it: `outside` is [`outside_hardware`]'s, non-empty. What the search's finding names is not read
/// here: the harness takes its verdict alone (`searched_record`).
fn outside_classes(
    description: &Description,
    outside: &[(String, String)],
) -> BTreeSet<&'static str> {
    let mut classes = BTreeSet::from([SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE]);
    let declared: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let platforms: BTreeSet<String> = description
        .platforms()
        .into_iter()
        .map(|p| p.name)
        .collect();
    let hardware = system_hardware(description);
    let from_tasks = services_reached(description, tasks_names(description), true, true);
    let mut roots = own_level_names(description);
    roots.extend(tasks_names(description));
    let directly: BTreeSet<&String> = roots.iter().collect();
    let through_services = services_reached(description, roots.clone(), false, false);
    let through_hardware = services_reached(description, roots.clone(), true, false);
    let is_declared = |name: &String| {
        declared.contains_key(name)
            || description.services.iter().any(|s| &s.name == name)
            || description.systems().iter().any(|s| &s.name == name)
    };
    for (namer, named) in outside {
        let service = description
            .services
            .iter()
            .find(|s| &s.name == namer)
            .expect("a service or a policy");
        classes.insert(if service.policy {
            "outside: named by a policy"
        } else {
            "outside: named by a service"
        });
        classes.insert(if service.side.needs.contains(named) {
            "outside: named by `needs`"
        } else {
            "outside: named by `uses`"
        });
        classes.insert(if from_tasks.contains(namer) {
            "outside: its namer served by a task"
        } else {
            "outside: its namer served by the system's own level alone"
        });
        if !directly.contains(namer) {
            classes.insert(if through_services.contains(namer) {
                "outside: its namer served through another service or policy"
            } else if through_hardware.contains(namer) {
                "outside: its namer served only through what hardware the system serves names"
            } else {
                "outside: its namer served only through a system it names"
            });
        }
        let composed = description
            .platforms()
            .iter()
            .chain(&description.blocks)
            .any(|p| p.side.uses.contains(named));
        classes.insert(if platforms.contains(named) {
            if description.systems().iter().any(|s| &s.platform == named) {
                "outside: a platform another system uses"
            } else {
                "outside: a platform no system uses"
            }
        } else if composed {
            "outside: a block a platform outside the hardware composes"
        } else {
            "outside: a block no platform composes"
        });
        // Read from its declaration, no record binding it: a fact it needs, which names no declaration.
        if declared[named].side.needs.iter().any(|n| !is_declared(n)) {
            classes.insert("outside: requiring something of its own");
        }
    }
    let namers: BTreeSet<&String> = outside.iter().map(|(n, _)| n).collect();
    let named: BTreeSet<&String> = outside.iter().map(|(_, n)| n).collect();
    if outside.len() >= 2 {
        classes.insert("outside: several");
    }
    if namers.len() < outside.len() {
        classes.insert("outside: one namer naming two");
    }
    if named.len() < outside.len() {
        classes.insert("outside: one block or platform named by two");
    }
    // Beside hardware a served service or policy names, which leaves the conflict as it is.
    let served = served_services(description);
    if description.services.iter().any(|s| {
        served.contains(&s.name)
            && s.side
                .uses
                .iter()
                .chain(&s.side.needs)
                .any(|n| hardware.contains(n))
    }) {
        classes.insert("outside: beside hardware a served service or policy names");
    }
    classes
}

/// §1, the refusals these universes can reach, the first that applies, from the description and the record's own
/// `bind` lines: "They are taken in the order written, this list and then the next, ... and the first that applies is
/// the one given; a bullet over tasks is taken task by task in runtime order".
/// - Of the first list, in its order: what its bullet over tasks keeps, "a periodic task bound itself for
///   `interrupt-source`, its requirement met by `true` as model §8 rule 2 reads one, which states a second release
///   model": `tool-failure`, "naming the task and its `period`"; then "a declaration or a task the system serves naming
///   a system by `uses` or `needs`":
///   `tool-failure`, "naming each such name, by the namer's text and then the system's"; then "a block or a platform the
///   system serves naming, by `uses` or `needs`, a declaration that is neither a block nor a platform": `tool-failure`,
///   "naming each, by the block's or platform's text and then the operand's" — "This refusal and the one above take
///   only what the system serves". (`M1.43`, `M1.46` and `M1.47` have the language refuse them, and "a description the
///   language's check refuses is no input".)
/// - Of the second, `unsupported-profile`, in its order: "a duration the plan would write past the engine's exact
///   arithmetic (§2.2): a task's period, minimum separation, deadline or jitter, a source's separation, or an obliged
///   bound, naming each — the tasks' in runtime order, then the sources' by their interrupt's text, then the bounds by
///   their instance's text"; "a task whose deadline is measured from an event other than its release"; and "a task, or
///   the system's own level, that would receive an interrupt's code: a timer route that names it, bound to its
///   provider itself by a binding that takes a channel (model §8 rule 2) — an alarm it programs, as §2.6 reads that
///   binding — or an adapter instance with a cost running its requester's code (`expiry`) whose requester it is, the one
///   an exclusive instance serves or one bound to a shared instance. ... It names every such receiver — each task in
///   runtime order, then the system's own level — with what it would receive"; and, between the durations and the
///   deadline, "the system declares no task".
///
/// No other bullet applies here: no system has placement memory, and the two refusals after the last, of a record's
/// supported contexts and its initialisation's calls, wait for a record that can be a provider (`M3.7`, `RTP-H10`). A
/// system a served service or policy leaves naming hardware outside, which round 20 had §1 refuse, is model §8 rule 7's
/// conflict since `M1.49`: the search writes no record of it ([`outside_hardware`]), so nothing here reads one. Nor of
/// two tasks of one name, which round 23 had §1 refuse: since `M1.50` the language rejects them
/// ([`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]). Nor of a task's duration that is not a time or a release of zero,
/// which round 20 had the bullet over tasks refuse: since `M1.42` the language rejects them
/// ([`LANGUAGE_REJECTS_A_TASK_S_DURATION`]).
fn refusal_the_record_gives(description: &Description, binds: &[Bind]) -> Option<Due> {
    // First, after the rebuild's, which every record of these universes passes: "the generator's own joint rules, run
    // over the record the checker accepted, find a conflict, an unknown or a map finding: `tool-failure`, naming both
    // verdicts" — of model §8 rule 16 (`M4.1.2.3`), "in the model, the generator's joint rules and the checker's", a
    // task bound itself to a source the analysis does not admit. No record the search writes reaches it; the records
    // [`rule_16_records`] builds do ([`RULE_16_RECORD`]).
    let joint = unadmitted(description, binds);
    if !joint.is_empty() {
        return Some(Due {
            code: "tool-failure",
            names: Names::Joint(joint),
            classes: BTreeSet::from(["refusal: the generator's joint rules, model §8 rule 16"]),
        });
    }
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let mut by_index: Vec<&Task> = description.tasks.iter().collect();
    by_index.sort_by_key(|t| t.rank());
    // A periodic task bound to a source's block, `interrupt-source` true among its offers, for another fact: "bound
    // itself for `interrupt-source`" fails, and it is no second release model (round 15, R15-D2).
    let bound_otherwise = |task: &Task| {
        binds.iter().any(|b| {
            b.requester == task_text(task)
                && !relies_on_source(description, b)
                && description
                    .hardware()
                    .iter()
                    .any(|h| h.name == b.provider && h.separation.is_some())
        })
    };
    // §1's bullet over tasks, the first list's, what the language leaves of it: "a periodic task bound itself for
    // `interrupt-source`, its requirement met by `true` as model §8 rule 2 reads one, which states a second release
    // model ...: `tool-failure`, naming the task and its `period`", "taken task by task in runtime order" (`M1.43` is
    // to move it to the language, `RTP-H15`). Its clauses that are not a time and its
    // releases of zero, which round 20 read here (R20-R7), are the language's since `M1.42` (`RTP-H14`): no description
    // writing one reaches this ([`LANGUAGE_REJECTS_A_TASK_S_DURATION`]), so a task has one defect at most, and no order
    // among its clauses is left to read. Its first clause, "none stated, which it refuses already (`missing-fact` for
    // no release model, a deadline's cardinality one)", is the language's too: a task stating neither is rejected, as
    // the task-bullet universe holds.
    let second_release_model = |task: &Task| {
        matches!(task.release, Release::Period(_))
            && binds
                .iter()
                .any(|b| b.requester == task_text(task) && relies_on_source(description, b))
    };
    for (index, task) in by_index.iter().enumerate() {
        if !second_release_model(task) {
            continue;
        }
        let mut classes = BTreeSet::from(["refusal: a periodic task bound to a source itself"]);
        if by_index[index + 1..]
            .iter()
            .any(|t| second_release_model(t))
        {
            classes.insert("task bullet: the first ill-formed task in runtime order named, a later one ill-formed too");
        }
        let declared_at = |t: &Task| description.tasks.iter().position(|d| std::ptr::eq(d, t));
        if by_index[index + 1..]
            .iter()
            .any(|t| declared_at(t) < declared_at(task) && second_release_model(t))
        {
            classes.insert(
                "task bullet: a task declared earlier ill-formed too, after the one named in runtime order",
            );
        }
        if !systems_named(description).is_empty() {
            classes.insert("task bullet: before a requester naming a system");
        }
        if !hardware_naming_other_kinds(description).is_empty() {
            classes.insert("periodic: before hardware naming what is not hardware");
        }
        if by_index[..index]
            .iter()
            .any(|t| matches!(t.release, Release::Period(_)) && bound_otherwise(t))
        {
            classes.insert(
                "periodic: one bound to the source's block otherwise first in runtime order, the one bound for \
                 `interrupt-source` named",
            );
        }
        if bound_otherwise(task) {
            classes
                .insert("periodic: the one named bound to the source's block for another fact too");
        }
        return Some(Due {
            code: "tool-failure",
            names: Names::Task {
                task: task_text(task),
                clause: Some("period"),
            },
            classes,
        });
    }

    let named = systems_named(description);
    if !named.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a requester naming a system"]);
        let hardware_names: BTreeSet<String> =
            description.hardware().into_iter().map(|b| b.name).collect();
        let platform_names: BTreeSet<String> = description
            .platforms()
            .into_iter()
            .map(|p| p.name)
            .collect();
        for (namer, named_system) in &named {
            classes.insert(if namer.starts_with("(task ") {
                "naming: a task naming a system"
            } else if namer == system {
                "naming: the system's own level naming a system"
            } else if description.services.iter().any(|s| &s.name == namer) {
                "naming: a service naming a system"
            } else if hardware_names.contains(namer) {
                "naming: a block or a platform naming a system"
            } else {
                "naming: a named system's own level naming a system"
            });
            // Round 15 (R15-R5): which clause of which kind of hardware names the system.
            if hardware_names.contains(namer) {
                let by_needs = side_of(description, namer)
                    .is_some_and(|side| side.needs.contains(named_system));
                classes.insert(match (platform_names.contains(namer), by_needs) {
                    (true, false) => "naming: a platform's `uses` naming a system",
                    (true, true) => "naming: a platform's `needs` naming a system",
                    (false, false) => "naming: a block's `uses` naming a system",
                    (false, true) => "naming: a block's `needs` naming a system",
                });
            }
            if named_system == system {
                classes.insert("naming: the lowered system itself named");
            }
        }
        if named.len() >= 2 {
            classes.insert("naming: several names");
        }
        if named.windows(2).any(|w| w[0].0 == w[1].0) {
            classes.insert("naming: one namer naming two systems");
        }
        if !hardware_naming_other_kinds(description).is_empty() {
            classes.insert("naming: before hardware naming what is not hardware");
        }
        let served = served_hardware(description);
        if named
            .iter()
            .any(|(namer, _)| hardware_names.contains(namer) && !served.contains(namer))
        {
            classes.insert(
                "naming: hardware served through a reached system's platform naming a system",
            );
        }
        name_order_classes(&named, "naming", &mut classes);
        return Some(Due {
            code: "tool-failure",
            names: Names::Systems(named),
            classes,
        });
    }

    // The first list's last bullet, after the naming-system one: hardware naming what is not hardware.
    let operands = hardware_naming_other_kinds(description);
    if !operands.is_empty() {
        let mut classes = BTreeSet::from(["refusal: hardware naming what is not hardware"]);
        let platforms: BTreeSet<String> = description
            .platforms()
            .into_iter()
            .map(|p| p.name)
            .collect();
        let named_by = |namer: &str, operand: &str, needs: bool| {
            description.hardware().iter().any(|b| {
                b.name == namer
                    && if needs {
                        b.side.needs.iter().any(|n| n == operand)
                    } else {
                        b.side.uses.iter().any(|n| n == operand)
                    }
            })
        };
        for (namer, operand) in &operands {
            let kind = if description
                .services
                .iter()
                .any(|s| &s.name == operand && s.policy)
            {
                "policy"
            } else if description.services.iter().any(|s| &s.name == operand) {
                "service"
            } else {
                "system"
            };
            let by = if named_by(namer, operand, true) {
                "needs"
            } else {
                "uses"
            };
            let who = if platforms.contains(namer) {
                "a platform"
            } else {
                "a block"
            };
            classes.insert(match (who, by, kind) {
                ("a block", "uses", "service") => "operand: a block's `uses` naming a service",
                ("a block", "uses", "policy") => "operand: a block's `uses` naming a policy",
                ("a block", _, "service") => "operand: a block's `needs` naming a service",
                ("a block", _, "policy") => "operand: a block's `needs` naming a policy",
                (_, "uses", "service") => "operand: a platform's `uses` naming a service",
                (_, "uses", "policy") => "operand: a platform's `uses` naming a policy",
                (_, _, "service") => "operand: a platform's `needs` naming a service",
                (_, _, "policy") => "operand: a platform's `needs` naming a policy",
                // Both bullets take what the system serves, the naming-system one first: hardware the system serves
                // naming a system is that bullet's, and hardware it does not serve is neither's (R15-R2).
                _ => "unreachable: a system the operand of hardware the system serves, the naming-system refusal first",
            });
            if binds
                .iter()
                .any(|b| &b.requester == operand && &b.provider == namer)
            {
                classes.insert("operand: a block naming the service bound to it");
            }
            let named_elsewhere = description
                .tasks
                .iter()
                .map(|t| &t.side)
                .chain(std::iter::once(&description.own))
                .chain(description.services.iter().map(|s| &s.side))
                .any(|side| side.uses.iter().chain(&side.needs).any(|n| n == operand));
            if kind == "service" && !named_elsewhere {
                classes.insert("operand: a service only hardware names");
            }
        }
        if !unserved_naming_other_kinds(description).is_empty() {
            classes.insert(
                "operand: hardware the system does not serve naming what is not hardware too, not named",
            );
        }
        // A namer served only through a service hardware names lies outside the hardware, and that service names it or
        // what composes it: model §8 rule 7's conflict (`M1.49`), no record reaching this refusal.
        let served_otherwise = served_hardware_by(description, false);
        if operands
            .iter()
            .any(|(namer, _)| !served_otherwise.contains(namer))
        {
            classes.insert(
                "unreachable: hardware naming what is not hardware, served only through what served hardware names, \
                 model §8 rule 7's conflict refusing it",
            );
        }
        if operands.len() >= 2 {
            classes.insert("operand: several, named in order");
        }
        if operands.windows(2).any(|w| w[0].0 == w[1].0) {
            classes.insert("operand: one namer naming two");
        }
        if operands.windows(2).any(|w| w[0].1 > w[1].1) {
            classes.insert("operand: the namers' order unlike the operands'");
        }
        name_order_classes(&operands, "operand", &mut classes);
        // The second list's bullets it comes before, where they would apply too.
        if description.tasks.iter().any(|t| {
            t.release.duration().seconds().is_none()
                || t.deadline().seconds().is_none()
                || t.jitter.is_some_and(|j| j.seconds().is_none())
        }) {
            classes.insert("operand: before a duration past the arithmetic");
        }
        if description
            .tasks
            .iter()
            .any(|t| t.deadline_from.is_some_and(|e| e != "release"))
        {
            classes.insert("operand: before a deadline from another event");
        }
        if binds.iter().any(|b| {
            (b.requester == system || by_index.iter().any(|t| task_text(t) == b.requester))
                && if is_instance(&b.provider) {
                    kind_of(&b.provider).runs_requester
                } else {
                    takes_channel(description, b)
                }
        }) {
            classes.insert("operand: before a receiver of interrupt code");
        }
        return Some(Due {
            code: "tool-failure",
            names: Names::Operands(operands),
            classes,
        });
    }

    // Durations past the arithmetic: each task's, in runtime order; each source's whose route the plan would write;
    // each obliged bound an instance would take on.
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let platforms: BTreeSet<String> = description
        .platforms()
        .into_iter()
        .map(|p| p.name)
        .collect();
    let mut past = Vec::new();
    for task in &by_index {
        if task.release.duration().seconds().is_none() {
            past.push(format!("{} {}", task_text(task), task.release.clause()));
        }
        if task.deadline().seconds().is_none() {
            past.push(format!("{} deadline", task_text(task)));
        }
        if task.jitter.is_some_and(|j| j.seconds().is_none()) {
            past.push(format!("{} jitter", task_text(task)));
        }
    }
    let tasks_past = past.len();
    // A set of texts iterates by each, compared byte by byte: "the sources' by their interrupt's text".
    let sources: BTreeSet<String> = binds
        .iter()
        .filter(|b| relies_on_source(description, b) && !is_instance(&b.provider))
        .filter(|b| {
            hardware
                .get(&b.provider)
                .and_then(|block| block.separation)
                .is_some_and(|s| s.read() == Sep::Past)
        })
        .map(|b| format!("(source {})", b.provider))
        .collect();
    let sources_past = sources.len();
    let source_texts: Vec<String> = sources.iter().cloned().collect();
    let sources_unlike_names = worded_otherwise(&source_texts, |text| {
        text["(source ".len()..text.len() - 1].to_string()
    });
    past.extend(sources);
    // "then the bounds by their instance's text and then the fact's": a set of pairs iterates by the first, then the
    // second. An exclusive instance's requester is any requester it can serve (§2.4: "the requirement's value at that
    // exclusive instance"; model §8 rule 1): a task, a service or a policy, a block or a platform, or the system's own
    // level, "for what it writes at its own level outside its tasks" (round 16, R16-R1: the reading had looked among
    // services, tasks and hardware alone).
    let mut bounds: BTreeSet<(String, String)> = BTreeSet::new();
    let mut bounded_requesters: BTreeSet<&'static str> = BTreeSet::new();
    for instance in binds.iter().map(|b| &b.provider).filter(|p| is_instance(p)) {
        let instance_form = form(instance);
        let Some(requester) = items(&instance_form).get(3).map(text_of) else {
            continue;
        };
        let side = side_of(description, &requester);
        for fact in kind_of(instance).obliged {
            // Its `at-most` bounds and its `exactly` ones alike (round 23, R23-R7): the value §2.4 writes is either's.
            let past_of = |bounds: &[(String, Dur)]| {
                bounds
                    .iter()
                    .any(|(f, bound)| f == fact && bound.seconds().is_none())
            };
            if side.as_ref().is_some_and(|side| past_of(&side.exactly)) {
                bounded_requesters.insert("past: an obliged bound written `exactly`");
            }
            if side
                .as_ref()
                .is_some_and(|side| past_of(&side.at_most) || past_of(&side.exactly))
            {
                bounds.insert((instance.clone(), (*fact).to_string()));
                bounded_requesters.insert(if requester == system {
                    "past: an obliged bound of the system's own level"
                } else if platforms.contains(&requester) {
                    "past: an obliged bound of a platform"
                } else if hardware.contains_key(&requester) {
                    "past: an obliged bound of a block"
                } else if description
                    .services
                    .iter()
                    .any(|s| s.name == requester && s.policy)
                {
                    "past: an obliged bound of a policy"
                } else {
                    "past: an obliged bound of a service"
                });
            }
        }
    }
    let bounds_past = bounds.len();
    let instance_texts: Vec<String> = bounds.iter().map(|(i, _)| i.clone()).collect();
    // An exclusive instance's text ends with its requester's: the bounds ordered by their requesters' names otherwise.
    let bounds_unlike_names = worded_otherwise(&instance_texts, |text| {
        text.trim_end_matches(')')
            .rsplit(' ')
            .next()
            .expect("a requester")
            .to_string()
    });
    past.extend(
        bounds
            .into_iter()
            .map(|(instance, fact)| format!("{instance} {fact}")),
    );
    if !past.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a duration past the arithmetic"]);
        if sources_unlike_names {
            classes.insert("past: the sources' texts ordered unlike their providers' names");
        }
        if bounds_unlike_names {
            classes.insert("past: the bounds' instances ordered unlike their requesters' names");
        }
        if tasks_past > 0 {
            classes.insert("past: a task's clause");
        }
        if sources_past > 0 {
            classes.insert("past: a source's separation");
        }
        if bounds_past > 0 {
            classes.insert("past: an obliged bound");
        }
        classes.extend(bounded_requesters);
        if [tasks_past, sources_past, bounds_past]
            .iter()
            .filter(|n| **n > 0)
            .count()
            == 3
        {
            classes.insert("past: a task's, a source's and a bound at once");
        }
        if past.len() >= 2 {
            classes.insert("past: several, named in order");
        }
        if description
            .tasks
            .iter()
            .any(|t| t.deadline_from.is_some_and(|e| e != "release"))
        {
            classes.insert("past: before a deadline from another event");
        }
        return Some(Due {
            code: "unsupported-profile",
            names: Names::Durations(past),
            classes,
        });
    }

    // "the system declares no task: the scheduler a build runs (`RTP-H1`) has no empty form".
    if description.tasks.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a system with no task"]);
        if binds.iter().any(|b| {
            b.requester == system
                && if is_instance(&b.provider) {
                    kind_of(&b.provider).runs_requester
                } else {
                    takes_channel(description, b)
                }
        }) {
            classes.insert("no task: before the system's own level receiving interrupt code");
        }
        return Some(Due {
            code: "unsupported-profile",
            names: Names::NoTask,
            classes,
        });
    }

    for task in &by_index {
        if task.deadline_from.is_some_and(|e| e != "release") {
            let mut classes = BTreeSet::from(["refusal: a deadline measured from another event"]);
            if !std::ptr::eq(*task, &description.tasks[0])
                && description.tasks[0]
                    .deadline_from
                    .is_some_and(|e| e != "release")
            {
                classes.insert(
                    "deadline: the first in runtime order named, an earlier declared one measured from another event too",
                );
            }
            return Some(Due {
                code: "unsupported-profile",
                names: Names::Task {
                    task: task_text(task),
                    clause: None,
                },
                classes,
            });
        }
    }

    // "It names every such receiver — each task in runtime order, then the system's own level — with what it would
    // receive: first the alarms it programs, each naming the provider, then its expiries, each naming the instance that
    // delivers it, each kind by those texts."
    let mut receivers = Vec::new();
    let mut classes = BTreeSet::new();
    let requesters = by_index
        .iter()
        .map(|t| task_text(t))
        .chain(std::iter::once(system.to_string()));
    for requester in requesters {
        let mut receiver = Receiver {
            subject: requester.clone(),
            alarms: BTreeSet::new(),
            expiries: BTreeSet::new(),
        };
        let mut consumed_at: BTreeMap<String, usize> = BTreeMap::new();
        for bind in binds.iter().filter(|b| b.requester == requester) {
            if is_instance(&bind.provider) {
                if kind_of(&bind.provider).runs_requester {
                    receiver.expiries.insert(bind.provider.clone());
                    classes.insert("receivers: an expiry");
                }
            } else if takes_channel(description, bind) {
                // "a timer route that names it, bound to its provider itself by a binding that takes a channel (model
                // §8 rule 2)": the route at that provider names it (rule 8), and the binding is the alarm it programs.
                receiver.alarms.insert(bind.provider.clone());
                *consumed_at.entry(bind.provider.clone()).or_default() += 1;
                classes.insert("receivers: an alarm programmed itself");
            }
        }
        // What else it is bound to at a provider it receives from, for a fact that takes no channel there.
        for bind in binds.iter().filter(|b| b.requester == requester) {
            if is_instance(&bind.provider) || takes_channel(description, bind) {
                continue;
            }
            if receiver.alarms.contains(&bind.provider) {
                classes.insert("receivers: an alarm, its receiver bound to its provider for a fact taking no channel too");
            }
            if receiver
                .expiries
                .iter()
                .any(|instance| text_of(&items(&form(instance))[2]) == bind.provider)
            {
                classes.insert(
                    "receivers: an expiry, its receiver bound to the instance's provider for a fact taking no channel too",
                );
            }
        }
        if consumed_at.values().any(|facts| *facts >= 2) {
            classes.insert("receivers: one alarm for two consumed facts at one provider");
        }
        if receiver.alarms.len() + receiver.expiries.len() >= 2 {
            classes.insert("receivers: one receiver receiving two");
        }
        if receiver.received().next().is_some() {
            receivers.push(receiver);
        }
    }
    if !receivers.is_empty() {
        classes.insert(match receivers.len() {
            1 => "receivers: one",
            2 => "receivers: two",
            _ => "receivers: three",
        });
        let tasks: Vec<&String> = receivers
            .iter()
            .map(|r| &r.subject)
            .filter(|s| *s != system)
            .collect();
        if tasks.len() < receivers.len() {
            classes.insert(if tasks.is_empty() {
                "receivers: the system's own level alone"
            } else {
                "receivers: tasks and the system's own level"
            });
        }
        if tasks.windows(2).any(|w| w[0] > w[1]) {
            classes.insert("receivers: runtime order unlike the tasks' text order");
        }
        // The providers programmed, an alarm's or an instance's `OVER`, one's name a prefix of another's.
        let programmed: BTreeSet<String> = receivers
            .iter()
            .flat_map(|r| r.received())
            .map(|item| {
                if is_instance(item) {
                    text_of(&items(&form(item))[2])
                } else {
                    item.clone()
                }
            })
            .collect();
        if prefixed(&programmed) {
            classes.insert("receivers: providers one of whose names prefixes another's");
        }
        return Some(Due {
            code: "unsupported-profile",
            names: Names::Receivers(receivers),
            classes,
        });
    }

    None
}

/// Round 15's classes (R15-D2), which §1's two refusals keyed by their bindings meet whatever the system's outcome: a
/// task or the system's own level that a timer route names — bound for a consumed fact at `true` to the route's
/// provider, or to an instance whose own binding beneath is there (model §8 rules 2, 8 and 15) — and is bound to that
/// provider itself only for a fact that takes no channel, so receives no alarm there ("bound to its provider itself by a
/// binding that takes a channel"); one bound to a timer route's provider for such a fact, which the route does not name;
/// either beside one programming an alarm at that provider; and a periodic task bound to a source's block for another
/// fact than `interrupt-source`, so no second release model ("bound itself for `interrupt-source`").
fn keyed_classes(description: &Description, binds: &[Bind]) -> BTreeSet<&'static str> {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let requesters: Vec<(String, bool)> = description
        .tasks
        .iter()
        .map(|t| (task_text(t), true))
        .chain(std::iter::once((system.to_string(), false)))
        .collect();
    // Each timer route's provider, with the requesters it names.
    let mut routes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for bind in binds
        .iter()
        .filter(|b| !is_instance(&b.provider) && takes_channel(description, b))
    {
        let named = routes.entry(bind.provider.clone()).or_default();
        if is_instance(&bind.requester) {
            named.extend(
                binds
                    .iter()
                    .filter(|b| b.provider == bind.requester)
                    .map(|b| b.requester.clone()),
            );
        } else {
            named.insert(bind.requester.clone());
        }
    }
    let programs_at = |requester: &str, provider: &str| {
        binds.iter().any(|b| {
            b.requester == requester && b.provider == provider && takes_channel(description, b)
        })
    };
    let mut classes = BTreeSet::new();
    for (requester, is_task) in &requesters {
        for bind in binds.iter().filter(|b| {
            &b.requester == requester && !is_instance(&b.provider) && !takes_channel(description, b)
        }) {
            let Some(named) = routes.get(&bind.provider) else {
                continue;
            };
            if programs_at(requester, &bind.provider) {
                continue; // a receiver of that alarm, the receivers' class
            }
            classes.insert(match (is_task, named.contains(requester)) {
                (true, true) => {
                    "bound otherwise: a task a timer route names, bound to its provider for a fact taking no channel"
                }
                (true, false) => {
                    "bound otherwise: a task bound to a timer route's provider for a fact taking no channel, the \
                     route not naming it"
                }
                (false, true) => {
                    "bound otherwise: the system's own level a timer route names, bound to its provider for a fact \
                     taking no channel"
                }
                (false, false) => {
                    "bound otherwise: the system's own level bound to a timer route's provider for a fact taking no \
                     channel, the route not naming it"
                }
            });
            if requesters
                .iter()
                .any(|(other, _)| other != requester && programs_at(other, &bind.provider))
            {
                classes.insert(
                    "bound otherwise: beside a task or the system's own level programming an alarm at that provider",
                );
            }
        }
    }
    let sources: BTreeSet<String> = description
        .hardware()
        .into_iter()
        .filter(|b| b.separation.is_some())
        .map(|b| b.name)
        .collect();
    for task in description
        .tasks
        .iter()
        .filter(|t| matches!(t.release, Release::Period(_)))
    {
        if binds.iter().any(|b| {
            b.requester == task_text(task)
                && sources.contains(&b.provider)
                && !relies_on_source(description, b)
        }) {
            classes.insert("periodic: a periodic task bound to a source's block for another fact");
        }
    }
    classes
}

/// A refusal's message read as forms: its parenthesised forms as trees, every other word an atom; backquotes, quotes,
/// commas and semicolons separate words. Prose is no canonical text, so an unbalanced parenthesis is closed rather
/// than refused.
fn prose(message: &str) -> Vec<Sx> {
    fn flush(atom: &mut String, stack: &mut [Vec<Sx>]) {
        if !atom.is_empty() {
            stack
                .last_mut()
                .expect("open")
                .push(Sx::Atom(std::mem::take(atom)));
        }
    }
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    let mut atom = String::new();
    for ch in message.chars() {
        match ch {
            '(' => {
                flush(&mut atom, &mut stack);
                stack.push(Vec::new());
            }
            ')' => {
                flush(&mut atom, &mut stack);
                if stack.len() > 1 {
                    let done = stack.pop().expect("open");
                    stack.last_mut().expect("open").push(Sx::List(done));
                }
            }
            c if c.is_whitespace() || matches!(c, '`' | '"' | ',' | ';') => {
                flush(&mut atom, &mut stack);
            }
            c => atom.push(c),
        }
    }
    flush(&mut atom, &mut stack);
    while stack.len() > 1 {
        let done = stack.pop().expect("open");
        stack.last_mut().expect("open").push(Sx::List(done));
    }
    stack.pop().expect("one")
}

/// A form's canonical text, `()` included.
fn prose_text(sx: &Sx) -> String {
    match sx {
        Sx::List(items) if items.is_empty() => "()".to_string(),
        other => text_of(other),
    }
}

/// Every parenthesised form of a refusal's message, at any depth, by its canonical text.
fn forms_in(message: &str) -> BTreeSet<String> {
    fn walk(sx: &Sx, out: &mut BTreeSet<String>) {
        if let Sx::List(items) = sx {
            out.insert(prose_text(sx));
            for item in items {
                walk(item, out);
            }
        }
    }
    let mut out = BTreeSet::new();
    for sx in prose(message) {
        walk(&sx, &mut out);
    }
    out
}

/// Every word of a refusal's message outside an instance's `(adapter …)` text, inside other forms included.
fn words_outside_instances(message: &str) -> BTreeSet<String> {
    fn walk(sx: &Sx, out: &mut BTreeSet<String>) {
        match sx {
            Sx::Atom(atom) => {
                out.insert(atom.clone());
            }
            Sx::List(items) => {
                if items.first() != Some(&Sx::Atom("adapter".to_string())) {
                    for item in items {
                        walk(item, out);
                    }
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    for sx in prose(message) {
        walk(&sx, &mut out);
    }
    out
}

/// The subjects a refusal's message names, each once, in the order it first names them: each `(task SYSTEM NAME)` form,
/// and the system's own level by the system's name as a word of its own — compared whole, never as a substring of
/// another text (round 11, R11-R3), so the `sys` inside `(task sys t)` names no system. An instance's `(adapter …)`
/// text is a name of its own: the requester written inside it is the instance's, no subject of the message.
fn subjects_named(message: &str, system: &str) -> Vec<String> {
    fn word(atom: &str) -> &str {
        let atom = atom
            .strip_suffix("'s")
            .or_else(|| atom.strip_suffix("’s"))
            .unwrap_or(atom);
        atom.trim_end_matches(['.', ':', '!', '?'])
            .trim_matches(['\'', '‘', '’'])
    }
    fn walk(sx: &Sx, system: &str, out: &mut Vec<String>) {
        let mut name = |subject: String| {
            if !out.contains(&subject) {
                out.push(subject);
            }
        };
        match sx {
            Sx::Atom(atom) => {
                if word(atom) == system {
                    name(system.to_string());
                }
            }
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "adapter" {
                    return;
                }
                if head == "task"
                    && items.len() == 3
                    && items.iter().all(|i| matches!(i, Sx::Atom(_)))
                {
                    name(prose_text(sx));
                    return;
                }
                for item in items {
                    walk(item, system, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    for sx in prose(message) {
        walk(&sx, system, &mut out);
    }
    out
}

/// The durations a refusal's message names past the arithmetic, in the order it names them: a task's clause as
/// `(task SYSTEM NAME) CLAUSE`, the task the last `(task …)` form named and the clause a form headed by `period`,
/// `min-separation`, `deadline` or `jitter`; a source as its `(source NAME)` form; and a bound as its instance's
/// `(adapter …)` text, with the obliged fact the message names after it.
fn durations_named(message: &str) -> Vec<String> {
    const CLAUSES: [&str; 4] = ["period", "min-separation", "deadline", "jitter"];
    let obliged: BTreeSet<&str> = KINDS
        .iter()
        .flat_map(|k| k.obliged.iter().copied())
        .collect();
    let mut out: Vec<String> = Vec::new();
    let mut task: Option<String> = None;
    let mut instance: Option<usize> = None;
    for sx in prose(message) {
        match &sx {
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "task"
                    && items.len() == 3
                    && items.iter().all(|i| matches!(i, Sx::Atom(_)))
                {
                    task = Some(prose_text(&sx));
                    instance = None;
                } else if CLAUSES.contains(&head) {
                    if let Some(task) = &task {
                        out.push(format!("{task} {head}"));
                    }
                } else if head == "source" {
                    task = None;
                    instance = None;
                    out.push(prose_text(&sx));
                } else if head == "adapter" {
                    task = None;
                    out.push(prose_text(&sx));
                    instance = Some(out.len() - 1);
                }
            }
            Sx::Atom(atom) => {
                if let Some(at) = instance {
                    if obliged.contains(atom.as_str()) {
                        out[at].push_str(&format!(" {atom}"));
                        instance = None;
                    }
                }
            }
        }
    }
    out
}

/// What a refusal's message names, in the order it names them, each time it names one: each name it writes as code,
/// between backquotes, as the record writes a name, that is one of `declared` — a declaration's name, or a task's
/// `(task SYSTEM NAME)` form — compared whole. §1 says what a refusal names and in which order, never in what words,
/// so no other word is read, its verbs among them (round 20, R20-R7: the reader had keyed on the lowering's "names" and
/// "names the system").
fn names_mentioned(message: &str, declared: &BTreeSet<String>) -> Vec<String> {
    message
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::trim)
        .filter(|span| declared.contains(*span))
        .map(str::to_string)
        .collect()
}

/// Whether `mentioned`, a message's names in its order ([`names_mentioned`]), names `pairs` in their order and nothing
/// else: each pair "by the namer's text and then" the other's (§1), the namer written before what it names — before
/// each, or once before the several a namer names in a row.
fn names_pairs(mentioned: &[String], pairs: &[(String, String)]) -> bool {
    fn from(mentioned: &[String], pairs: &[(String, String)], previous: Option<&String>) -> bool {
        let Some(((namer, named), rest)) = pairs.split_first() else {
            return mentioned.is_empty();
        };
        let after_namer = mentioned.first() == Some(namer)
            && mentioned.get(1) == Some(named)
            && from(&mentioned[2..], rest, Some(namer));
        let namer_once = previous == Some(namer)
            && mentioned.first() == Some(named)
            && from(&mentioned[1..], rest, Some(namer));
        after_namer || namer_once
    }
    from(mentioned, pairs, None)
}

/// Whether a refusal's message names `clause`, outside an instance's text: a form it heads, such as `(period 20 ms)`, or
/// the word itself.
fn clause_named(message: &str, clause: &str) -> bool {
    fn walk(sx: &Sx, clause: &str) -> bool {
        match sx {
            Sx::Atom(atom) => atom == clause,
            Sx::List(items) => {
                items.first() != Some(&Sx::Atom("adapter".to_string()))
                    && items.iter().any(|item| walk(item, clause))
            }
        }
    }
    prose(message).iter().any(|sx| walk(sx, clause))
}

/// What a refusal's message names each receiver receiving, in the order it names them, as `(SUBJECT, ITEM)`: read
/// clause by clause, a clause ending at a `;`, each naming one subject — a `(task …)` form or the system's own level's
/// name, as [`subjects_named`] reads them — with each instance's `(adapter …)` text it holds and each of `hardware` it
/// names outside such a text, an alarm's provider. A clause naming something received beside no subject or several
/// says not who receives it, and is returned as such. §1 fixes who receives what and in which order, not the words.
fn received_named(
    message: &str,
    system: &str,
    hardware: &BTreeSet<String>,
) -> Result<Vec<(String, String)>, String> {
    fn walk(sx: &Sx, hardware: &BTreeSet<String>, out: &mut Vec<String>) {
        match sx {
            Sx::Atom(atom) => {
                if hardware.contains(atom) && !out.contains(atom) {
                    out.push(atom.clone());
                }
            }
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "adapter" {
                    let text = prose_text(sx);
                    if !out.contains(&text) {
                        out.push(text);
                    }
                } else if head != "task" {
                    // A `(task …)` form names a subject, never what is received.
                    for item in items {
                        walk(item, hardware, out);
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for clause in message.split(';') {
        let mut received = Vec::new();
        for sx in prose(clause) {
            walk(&sx, hardware, &mut received);
        }
        if received.is_empty() {
            continue;
        }
        match subjects_named(clause, system).as_slice() {
            [subject] => out.extend(received.into_iter().map(|item| (subject.clone(), item))),
            subjects => {
                return Err(format!(
                    "a clause names {received:?} received beside {subjects:?}, saying not who receives it: {clause:?}"
                ))
            }
        }
    }
    Ok(out)
}

/// Where the lowering's `refusal` is not the one `due`: its code, and what it names. Of a refusal naming a task, the
/// task, and the clause where §1 names one. Of the interrupt-code refusal, the subjects it names in their order, and
/// what it says each would receive, in §1's order — its alarms, then its expiries, each kind by its texts: §1 fixes the
/// receivers, what each receives and their order, not the message's wording; so an expiry is held to its instance's
/// text, written whole, and an alarm a receiver programs itself to its provider's name, written outside any instance's
/// text — an interrupt being "named by its kind and its provider" (§2). Of the durations past the arithmetic, each, in
/// §1's order. Of the naming-system refusal, each requester and the system it names, by the namer's text and then the
/// system's; of the refusal of hardware naming what is not hardware, each block or platform and its operand, by the
/// namer's text and then the operand's — each text compared byte by byte, "never by its message's words".
fn refusal_disagrees(due: &Due, refusal: &Refusal, description: &Description) -> Option<String> {
    let system = description.system.as_str();
    let declared: BTreeSet<String> = description
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(description.hardware().into_iter().map(|b| b.name))
        .chain(description.systems().into_iter().map(|s| s.name))
        .chain(
            description
                .tasks
                .iter()
                .map(|t| format!("(task {system} {})", t.name)),
        )
        .collect();
    let mut wrong = Vec::new();
    if refusal.code != due.code {
        wrong.push(format!(
            "its code is `{}`, the record's `{}`",
            refusal.code, due.code
        ));
    }
    match &due.names {
        Names::Task { task, clause } => {
            let named = subjects_named(&refusal.message, system);
            if named != [task.clone()] {
                wrong.push(format!("it names {named:?}, the record [{task:?}]"));
            }
            // "naming the task and its `period`": that clause, and no other of the task's.
            if let Some(clause) = clause {
                let clauses: Vec<&str> = ["period", "min-separation", "deadline", "jitter"]
                    .into_iter()
                    .filter(|c| clause_named(&refusal.message, c))
                    .collect();
                if clauses != [*clause] {
                    wrong.push(format!(
                        "it names the clauses {clauses:?}, the record `{clause}` alone"
                    ));
                }
            }
        }
        Names::Systems(pairs) | Names::Operands(pairs) => {
            let named = names_mentioned(&refusal.message, &declared);
            if !names_pairs(&named, pairs) {
                wrong.push(format!(
                    "it names {named:?}, in that order, where the record names {pairs:?}, each namer and then what \
                     it names, in that order"
                ));
            }
        }
        Names::NoTask => {
            // "the system declares no task": what is missing named, and no task, there being none.
            let named = subjects_named(&refusal.message, system);
            if named.iter().any(|subject| subject != system) {
                wrong.push(format!("it names {named:?}, of a system with no task"));
            }
        }
        Names::Joint(findings) => {
            if let Some(unlike) = joint_disagrees(&refusal.message, description, findings) {
                wrong.push(unlike);
            }
        }
        Names::Durations(items) => {
            let named = durations_named(&refusal.message);
            if named != *items {
                wrong.push(format!(
                    "it names {named:?} past the arithmetic, the record {items:?}, in that order"
                ));
            }
        }
        Names::Receivers(receivers) => {
            let subjects: Vec<String> = receivers.iter().map(|r| r.subject.clone()).collect();
            let named = subjects_named(&refusal.message, system);
            if named != subjects {
                wrong.push(format!(
                    "it names {named:?}, the record {subjects:?}, in that order"
                ));
            }
            let forms = forms_in(&refusal.message);
            let words = words_outside_instances(&refusal.message);
            let unsaid: Vec<&String> = receivers
                .iter()
                .flat_map(|r| r.expiries.iter())
                .filter(|instance| !forms.contains(*instance))
                .chain(
                    receivers
                        .iter()
                        .flat_map(|r| r.alarms.iter())
                        .filter(|provider| !words.contains(*provider)),
                )
                .collect();
            if !unsaid.is_empty() {
                wrong.push(format!(
                    "it does not name what would be received: {unsaid:?}"
                ));
            }
            // Each receiver with what it would receive, in §1's order (round 14, R14-R6).
            let hardware: BTreeSet<String> =
                description.hardware().into_iter().map(|b| b.name).collect();
            let expected: Vec<(String, String)> = receivers
                .iter()
                .flat_map(|r| r.received().map(|item| (r.subject.clone(), item.clone())))
                .collect();
            match received_named(&refusal.message, system, &hardware) {
                Ok(received) if received == expected => {}
                Ok(received) => wrong.push(format!(
                    "it names each receiver receiving {received:?}, the record {expected:?}, in that order"
                )),
                Err(unpaired) => wrong.push(unpaired),
            }
        }
    }
    (!wrong.is_empty()).then(|| wrong.join("; "))
}

/// The systems whose side, read whole, contradicts itself, which the search refuses `invalid-description`: no record
/// written, so nothing for the configuration checker to judge and nothing to lower. Model §8 rule 1: "Two tasks of one
/// system are two requesters, though §7 reads them as one side"; §7 rule 4: "Every `requires`, `needs` and `uses` a
/// declaration writes, at the positions §2's presence reads, is read as one side, so a contradiction split across two
/// clauses is one: constraints on one fact that no value satisfies together are `invalid-description`". The reading
/// derives the fact from the description alone ([`fact_the_side_contradicts`]), and a system it names is this class
/// only where the search gives that verdict: a record written for one, or another verdict, is a finding. `RTP-H20` is
/// what this holds, "so that the configuration checker refuses no record the search writes for that reason".
const SEARCH_READS_THE_SIDE_WHOLE: &str =
    "the search refuses, `invalid-description`, a system's side that contradicts itself, read whole";

/// The systems a service or a policy they serve leaves naming a block or a platform outside their hardware, which the
/// search refuses `infeasible-configuration`: model §8 rule 7's conflict, "judged once for the system whatever the
/// assignment" (`M1.49`, `RTP-H22`), `Joint::map`'s refusal stopping the search, so no record is written, for the
/// configuration checker to judge or the lowering to refuse. The reading derives the conflict from the description
/// alone ([`outside_hardware`]), and a system it names is this class only where the search gives that verdict: a record
/// written for one, or another verdict, is a finding. Round 20 had §1 refuse such a system; the bullet went with
/// `M1.49`.
const SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE: &str =
    "the search refuses, `infeasible-configuration`, a service or a policy the system serves naming a block or a \
     platform outside its hardware";

/// The systems declaring two tasks of one name, which the language rejects, `invalid-description` (reference §7 rule 6:
/// "Two tasks of one system are refused alike, since every rule keying a requester by its name would read them as one";
/// `M1.50`, `RTP-H23`). A description the language's check refuses is no input (§1), so no search, record or lowering
/// reads one. The reading derives the class from the description alone ([`names_distinct`]), and a system it names is
/// of it only where the language gives that verdict: another verdict is a finding, and so is the language accepting
/// one, which nothing goes on to read. Round 23 had §1 refuse such a system; the bullet went with `M1.50`, and with it
/// the class of records the configuration checker refused for reading a binding of the text two such tasks carry as
/// the first's.
const LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME: &str =
    "the language rejects, `invalid-description`, two tasks of one system carrying one name";

/// The systems a task of whose description writes a `period`, `min-separation`, `deadline` or `jitter` that is not a
/// time, or a `period` or `min-separation` of zero, which the language rejects, `invalid-description` (model §4 rule 8:
/// "Both are the language's rule, and both `invalid-description`, naming the clause"; `M1.42`, `RTP-H14`). A
/// description the language's check refuses is no input (§1), so no search, record or lowering reads one, whatever else
/// it writes: `invalid-description` is the first thing to fix of anything else the language finds, a `missing-fact`
/// beside it among them (`eadl_model::check`: "The **verdict** is the highest-precedence one"; `Verdict::precedence`).
/// The reading derives the class from the description alone ([`durations_the_language_rejects`]), and a system it names
/// is of it only where the language gives that verdict: another verdict is a finding, and so is the language accepting
/// one. Round 20 had §1's bullet over tasks refuse such a system; those clauses of the bullet went with `M1.42`, and
/// with them the classes of its order and its message.
const LANGUAGE_REJECTS_A_TASK_S_DURATION: &str =
    "the language rejects, `invalid-description`, a task's duration that is not a time or a release of zero";

/// What model §4 rule 8 refuses of what the tasks of every system the description declares write, each as its class:
/// "A task's `period`, `min-separation`, `deadline` and `jitter` are times, and its `period` or `min-separation` is
/// above zero". A deadline or a jitter of zero is a time, which the rule leaves alone. Empty where it refuses nothing.
fn durations_the_language_rejects(description: &Description) -> BTreeSet<&'static str> {
    let mut classes = BTreeSet::new();
    let others = description.others.iter().flat_map(|system| &system.tasks);
    for task in description.tasks.iter().chain(others) {
        let release = (task.omitted != Some("period")).then(|| task.release.duration());
        let deadline = (task.omitted != Some("deadline")).then(|| task.deadline());
        let (not_a_time, zero) = match task.release {
            Release::Period(_) => (
                "language: a period that is not a time",
                "language: a period of zero",
            ),
            Release::MinSeparation(_) => (
                "language: a minimum separation that is not a time",
                "language: a minimum separation of zero",
            ),
        };
        for (written, class) in [
            (release, not_a_time),
            (deadline, "language: a deadline that is not a time"),
            (task.jitter, "language: a jitter that is not a time"),
        ] {
            if let Some(Time::Other(unit)) = written.map(Dur::time) {
                classes.insert(class);
                classes.insert(
                    NOT_A_TIME_IN[OTHER_UNITS.iter().position(|u| *u == unit).expect("a unit")],
                );
            }
        }
        if release.map(Dur::time) == Some(Time::Seconds(Ratio::ZERO)) {
            classes.insert(zero);
        }
    }
    classes
}

/// Whether the language rejects the description by a rule these universes hold it to, whatever else it writes: two
/// tasks of one name ([`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]) or a task's duration
/// ([`LANGUAGE_REJECTS_A_TASK_S_DURATION`]). Such a description is no input, so no rule after the language's check is
/// read of it.
fn language_rejects(description: &Description) -> bool {
    !names_distinct(description) || !durations_the_language_rejects(description).is_empty()
}

/// The fact on which the lowered system's side, read whole — its own level's clauses and its tasks' (model §8 rule 1;
/// §7 rule 4) — requires both `true` and `false`, a requirement `(needs f)` or `(f true)` beside `(f false)`.
fn fact_the_side_contradicts(description: &Description) -> Option<String> {
    let mut values: BTreeMap<String, BTreeSet<bool>> = BTreeMap::new();
    for side in std::iter::once(&description.own).chain(description.tasks.iter().map(|t| &t.side)) {
        for fact in &side.needs {
            values.entry(fact.clone()).or_default().insert(true);
        }
        for clause in &side.other {
            let clause = form(clause);
            let clause = items(&clause);
            if let (Some(Sx::Atom(fact)), Some(Sx::Atom(value))) = (clause.first(), clause.get(1)) {
                if value == "true" || value == "false" {
                    values
                        .entry(fact.clone())
                        .or_default()
                        .insert(value == "true");
                }
            }
        }
    }
    values
        .into_iter()
        .find(|(_, values)| values.len() == 2)
        .map(|(fact, _)| fact)
}

/// How one system fared.
struct Outcome {
    /// `lowered`, `rejected` by the language, `unresolved` by the search, or `refused` by the lowering.
    class: &'static str,
    /// The verdict of a system not lowered.
    verdict: String,
    /// Whether the configuration checker judged its record: every system the search resolves.
    judged: bool,
    /// The kinds of line a plan agrees with the reading on.
    agreed: Vec<&'static str>,
    /// The classes of the record's rules the system exercises.
    classes: BTreeSet<&'static str>,
    /// Where the code and the record's reading disagree, or the checker refuses the record: each a reproducer.
    findings: Vec<String>,
    /// The plan the code writes, where it writes one and the universe keeps them.
    plan: Option<String>,
    /// The records binding a task's own source the analysis does not admit judged by the configuration checker and
    /// lowered ([`RULE_16_RECORD`]), and those both refuse as rule 16 says.
    rule_16: (usize, usize),
    /// The record the unamended rules gave, where it binds one and the universe keeps it.
    unamended: Option<String>,
}

/// How one system fared: the search's record judged and lowered, and, where a task's own source may be one model §8
/// rule 16 does not admit, every record binding one judged and lowered too ([`rule_16_records`]).
fn outcome(universe: &str, case: &Case, keep: bool) -> Outcome {
    let (mut out, searched) = outcome_of_the_search(universe, case, keep);
    if let Some(records) = rule_16_records(universe, case, searched.as_deref()) {
        out.rule_16 = (records.judged, records.refused);
        out.classes.extend(records.classes);
        out.findings.extend(records.findings);
        if keep {
            out.unamended = records.unamended;
        }
    }
    out
}

/// How one system fared as the engine runs it, with the record the search wrote, where it wrote one.
fn outcome_of_the_search(universe: &str, case: &Case, keep: bool) -> (Outcome, Option<String>) {
    let text = case.description.text();
    let mut out = Outcome {
        class: "lowered",
        verdict: String::new(),
        judged: false,
        agreed: Vec::new(),
        classes: BTreeSet::new(),
        findings: Vec::new(),
        plan: None,
        rule_16: (0, 0),
        unamended: None,
    };
    // Model §8 rule 16 and rule 6: where no source a sporadic task bound itself may take is admitted, the search refuses
    // the system with the code they give, "naming the task and the source"; otherwise it binds one admitted.
    let rule_16 = (!language_rejects(&case.description))
        .then(|| rule_16_search(&case.description))
        .flatten();
    // Model §7 rule 4 and §8 rule 1: a system's side that contradicts itself, read whole, is `invalid-description`
    // whatever could be bound for it, so it has no record for rule 16 to judge ([`SEARCH_READS_THE_SIDE_WHOLE`]).
    let contradicted = fact_the_side_contradicts(&case.description);
    let reads_whole = |fact: &str| {
        format!(
            "`{}`'s side, read whole, requires `{fact}` both `true` and `false`: `invalid-description` (model §7 rule \
             4; §8 rule 1)",
            case.description.system
        )
    };
    // Model §8 rule 7: a served service or policy naming hardware outside the system's is "judged once for the system
    // whatever the assignment" ([`SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE`]).
    let outside = outside_hardware(&case.description);
    // Reference §7 rule 6: two tasks of one name are the language's to reject, whatever else the description writes
    // ([`LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME`]).
    let one_name = !names_distinct(&case.description);
    let two_tasks = || {
        format!(
            "reference §7 rule 6 rejects it `invalid-description`: two tasks of `{}` carry one name",
            case.description.system
        )
    };
    // Model §4 rule 8: a task's durations are the language's to reject, whatever else the description writes
    // ([`LANGUAGE_REJECTS_A_TASK_S_DURATION`]).
    let durations = durations_the_language_rejects(&case.description);
    let refused_durations =
        || format!("model §4 rule 8 rejects it `invalid-description`: {durations:?}");
    let (record, program, checker, lowered) = match flow(&text, &case.description.system) {
        Flow::Rejected(verdict) => {
            out.class = "rejected";
            let rejects_it = |why: String| {
                format!(
                    "== {universe}: {}\n-- the description\n{text}-- the language rejects it `{verdict}`, where \
                     {why}\n",
                    case.label
                )
            };
            if one_name {
                if verdict == "invalid-description" {
                    out.classes.insert(LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME);
                } else {
                    out.findings.push(rejects_it(two_tasks()));
                }
            }
            if !durations.is_empty() {
                if verdict == "invalid-description" {
                    out.classes.insert(LANGUAGE_REJECTS_A_TASK_S_DURATION);
                    out.classes.extend(&durations);
                } else {
                    out.findings.push(rejects_it(refused_durations()));
                }
            }
            out.verdict = verdict;
            return (out, None);
        }
        // Nothing below reads such a system: every rule finds a task by its text ([`side_of`]), and reads a duration as
        // a time ([`Dur::seconds`]).
        Flow::Unresolved(_) | Flow::Resolved { .. } if language_rejects(&case.description) => {
            let accepts_it = |why: String| {
                format!(
                    "== {universe}: {}\n-- the description\n{text}-- the language accepts it, where {why}\n",
                    case.label
                )
            };
            if one_name {
                out.findings.push(accepts_it(two_tasks()));
            }
            if !durations.is_empty() {
                out.findings.push(accepts_it(refused_durations()));
            }
            return (out, None);
        }
        Flow::Unresolved(verdict) => {
            out.class = "unresolved";
            if !outside.is_empty() {
                if verdict == "InfeasibleConfiguration" {
                    out.classes = outside_classes(&case.description, &outside);
                } else {
                    out.findings.push(format!(
                        "== {universe}: {}\n-- the description\n{text}-- the search refuses it `{verdict}`, where \
                         model §8 rule 7 gives `infeasible-configuration`, `{}` serving {outside:?}\n",
                        case.label, case.description.system
                    ));
                }
                // Neither rule states which verdict a system writing both takes, and no universe writes both.
                if contradicted.is_some() || rule_16.is_some() {
                    out.classes.insert(
                        "unreachable: a name outside the hardware beside a side that contradicts itself or a source \
                         rule 16 refuses, no universe writing both",
                    );
                }
            } else if let Some(fact) = &contradicted {
                if verdict == "InvalidDescription" {
                    out.classes.insert(SEARCH_READS_THE_SIDE_WHOLE);
                } else {
                    out.findings.push(format!(
                        "== {universe}: {}\n-- the description\n{text}-- the search refuses it `{verdict}`, where {}\n",
                        case.label,
                        reads_whole(fact)
                    ));
                }
            } else if let Some(code) = rule_16 {
                if verdict == verdict_written(code) {
                    out.classes.insert(if code == "unsupported-profile" {
                        RULE_16_SEARCH_CONFLICT
                    } else {
                        RULE_16_SEARCH_UNKNOWN
                    });
                } else {
                    out.findings.push(format!(
                        "== {universe}: {}\n-- the description\n{text}-- the search refuses it `{verdict}`, where model \
                         §8 rules 16 and 6 give `{code}`: no source a sporadic task bound itself may take is admitted\n",
                        case.label
                    ));
                }
            }
            out.verdict = verdict;
            return (out, None);
        }
        Flow::Resolved {
            record,
            program,
            checker,
            lowered,
        } => (record, program, checker, lowered),
    };
    out.judged = true;
    let reproducer = |written: &str, what: &str| {
        format!(
            "== {universe}: {}\n-- the description\n{text}-- the record\n{record}{written}{what}",
            case.label
        )
    };
    if let Some(code) = rule_16 {
        out.findings.push(reproducer(
            "",
            &format!(
                "-- the search resolves it, where model §8 rules 16 and 6 refuse it `{code}`: no source a sporadic \
                 task bound itself may take is admitted\n"
            ),
        ));
    }
    if let Some(fact) = &contradicted {
        out.findings.push(reproducer(
            "",
            &format!("-- the search resolves it, where {}\n", reads_whole(fact)),
        ));
    }
    if !outside.is_empty() {
        out.findings.push(reproducer(
            "",
            &format!(
                "-- the search resolves it, where model §8 rule 7 refuses it `infeasible-configuration`: `{}` serving \
                 {outside:?}\n",
                case.description.system
            ),
        ));
    }
    // The checker's verdict: every record the search writes accepted, a refusal a finding naming it.
    if let Some(refused) = &checker {
        out.findings.push(reproducer(
            "",
            &format!(
                "-- the configuration checker refuses the record the search wrote: {refused}\n"
            ),
        ));
    }
    let due = refusal_the_record_gives(&case.description, &binds_of(&record));
    match lowered {
        Ok(plan) => {
            if keep {
                out.plan = Some(plan.clone());
            }
            if let Some(due) = &due {
                out.findings.push(reproducer(
                    &format!("-- the code writes\n{plan}"),
                    &format!(
                        "-- the record refuses it: `{}`, naming {} (§1)\n",
                        due.code,
                        due.summary()
                    ),
                ));
                return (out, Some(record));
            }
            let judged = judge(&case.description, &program, &record, &plan);
            out.agreed = judged.agreed;
            out.classes = judged.classes;
            out.classes
                .extend(keyed_classes(&case.description, &binds_of(&record)));
            if !judged.disagreements.is_empty() {
                let mut what = String::new();
                for d in &judged.disagreements {
                    what.push_str(&format!(
                        "-- {}: the record's reading expects\n{}\n-- {}: the plan writes\n{}\n",
                        d.kind,
                        d.expected.join("\n"),
                        d.kind,
                        d.written.join("\n")
                    ));
                }
                out.findings
                    .push(reproducer(&format!("-- the code writes\n{plan}"), &what));
            }
        }
        Err(refusal) => {
            out.class = "refused";
            out.verdict = refusal.to_string();
            let wrong = match &due {
                Some(due) => refusal_disagrees(due, &refusal, &case.description),
                None => Some("the record refuses it for none of §1's reasons".to_string()),
            };
            match (wrong, due) {
                (None, Some(due)) => {
                    out.classes = due.classes;
                    out.classes
                        .extend(keyed_classes(&case.description, &binds_of(&record)));
                }
                (Some(wrong), due) => out.findings.push(reproducer(
                    &format!("-- the code writes\nrefused {refusal}\n"),
                    &format!(
                        "-- {wrong}{}\n",
                        due.map_or(String::new(), |due| format!(
                            " (§1: `{}`, naming {})",
                            due.code,
                            due.summary()
                        ))
                    ),
                )),
                (None, None) => unreachable!("a refusal with none due disagrees"),
            }
        }
    }
    (out, Some(record))
}

/// What a universe gave.
#[derive(Debug, Default)]
struct Tally {
    generated: usize,
    lowered: usize,
    rejected: usize,
    unresolved: usize,
    refused: usize,
    /// The systems whose record the configuration checker judged.
    judged: usize,
    /// For each kind of line, the lowered systems whose plan agrees with the reading on it.
    agreed: BTreeMap<&'static str, usize>,
    /// For each class of the record's rules, the systems exercising it.
    classes: BTreeMap<&'static str, usize>,
    /// Each verdict of a system not lowered, with how many gave it and the first that did.
    verdicts: BTreeMap<String, (usize, String)>,
    /// Each system the search resolved no plan for: its label, with the search's verdict.
    unresolved_systems: BTreeMap<String, String>,
    /// The records binding a task's own source the analysis does not admit judged by the configuration checker and
    /// lowered ([`RULE_16_RECORD`]), and those both refuse as rule 16 says.
    rule_16: (usize, usize),
    findings: Vec<String>,
    elapsed: std::time::Duration,
}

/// Every case lowered and judged, on every core: the cases are independent.
fn run(universe: &str, cases: &[Case]) -> Tally {
    run_keeping(universe, cases, false).0
}

/// [`run`], with the plan the code writes for each case, where it writes one and `keep`: kept only where a test holds
/// plans against each other, so that the large universes hold none in memory.
fn run_keeping(universe: &str, cases: &[Case], keep: bool) -> (Tally, Vec<Option<String>>) {
    let (tally, kept) = run_kept(universe, cases, keep);
    (tally, kept.into_iter().map(|k| k.plan).collect())
}

/// What a universe keeps of one system, where a test holds systems against each other.
#[derive(Debug, Default)]
struct Kept {
    /// The plan the code writes, where it writes one.
    plan: Option<String>,
    /// The record the unamended rules gave, where it binds a task's own source the analysis does not admit
    /// ([`RULE_16_UNAMENDED`]).
    unamended: Option<String>,
}

/// [`run_keeping`], with the record the unamended rules gave each case, where it binds a task's own source the analysis
/// does not admit.
fn run_kept(universe: &str, cases: &[Case], keep: bool) -> (Tally, Vec<Kept>) {
    let start = std::time::Instant::now();
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let mut outcomes: Vec<(usize, Outcome)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|first| {
                scope.spawn(move || {
                    cases
                        .iter()
                        .enumerate()
                        .skip(first)
                        .step_by(threads)
                        .map(|(i, case)| (i, outcome(universe, case, keep)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().expect("a worker"))
            .collect()
    });
    outcomes.sort_by_key(|(i, _)| *i);
    let mut tally = Tally {
        generated: cases.len(),
        elapsed: start.elapsed(),
        ..Tally::default()
    };
    let mut kept = Vec::with_capacity(cases.len());
    for (i, outcome) in outcomes {
        kept.push(Kept {
            plan: outcome.plan,
            unamended: outcome.unamended,
        });
        tally.rule_16.0 += outcome.rule_16.0;
        tally.rule_16.1 += outcome.rule_16.1;
        match outcome.class {
            "lowered" => tally.lowered += 1,
            "rejected" => tally.rejected += 1,
            "unresolved" => tally.unresolved += 1,
            _ => tally.refused += 1,
        }
        if outcome.judged {
            tally.judged += 1;
        }
        if outcome.class == "unresolved" {
            tally
                .unresolved_systems
                .insert(cases[i].label.clone(), outcome.verdict.clone());
        }
        if outcome.class != "lowered" {
            tally
                .verdicts
                .entry(format!("{} {}", outcome.class, outcome.verdict))
                .or_insert((0, cases[i].label.clone()))
                .0 += 1;
        }
        for kind in outcome.agreed {
            *tally.agreed.entry(kind).or_default() += 1;
        }
        for class in outcome.classes {
            *tally.classes.entry(class).or_default() += 1;
        }
        tally.findings.extend(outcome.findings);
    }
    (tally, kept)
}

/// How many systems of a universe a class of the record's rules is held to reach (round 20, R20-R7: "classes are held by
/// presence, not by count").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reach {
    /// As many as the universe's construction fixes: counted from its cases' parameters beside the call, or derived
    /// there by hand.
    Count(usize),
    /// At least one, where the count is the search's choice or follows from the record's rules over the description
    /// rather than from the construction alone.
    Some,
}

/// A class held to be reached by some system.
fn some(class: &'static str) -> (&'static str, Reach) {
    (class, Reach::Some)
}

/// A class held to be reached by exactly `count` systems, as the universe's construction fixes.
fn exactly(class: &'static str, count: usize) -> (&'static str, Reach) {
    (class, Reach::Count(count))
}

/// A class held to be reached by exactly the systems of `cases` that `reaches`, a predicate over each case's
/// construction — its parameters, as its description writes them — says reach it.
fn counted(
    class: &'static str,
    cases: &[Case],
    reaches: impl Fn(&Case) -> bool,
) -> (&'static str, Reach) {
    exactly(class, cases.iter().filter(|case| reaches(case)).count())
}

/// The universe's sizes held to `sizes` — generated, lowered, rejected by the language, unresolved by the search,
/// refused by the lowering — every system the search resolved judged by the configuration checker, each class of
/// `exercised` reached by as many systems as it is held to, and every finding, in one failure naming each.
fn hold(
    universe: &str,
    tally: &Tally,
    sizes: (usize, usize, usize, usize, usize),
    exercised: &[(&str, Reach)],
) {
    let got = (
        tally.generated,
        tally.lowered,
        tally.rejected,
        tally.unresolved,
        tally.refused,
    );
    eprintln!(
        "{universe}, in {:.1?}: {} generated, {} lowered, {} rejected, {} unresolved, {} refused; {} records judged by \
         the configuration checker, and {} binding a task's own source rule 16 refuses, judged and lowered\n  agreed: \
         {:?}\n  classes: {:?}\n  not lowered: {:?}",
        tally.elapsed,
        got.0,
        got.1,
        got.2,
        got.3,
        got.4,
        tally.judged,
        tally.rule_16.0,
        tally.agreed,
        tally.classes,
        tally.verdicts
    );
    let mut problems = String::new();
    if got != sizes {
        problems.push_str(&format!(
            "{universe}'s sizes (generated, lowered, rejected, unresolved, refused) moved: {got:?}, held {sizes:?}; \
             not lowered: {:?}\n",
            tally.verdicts
        ));
    }
    if tally.judged != tally.lowered + tally.refused {
        problems.push_str(&format!(
            "{universe}: {} records judged by the configuration checker, of {} the search resolved\n",
            tally.judged,
            tally.lowered + tally.refused
        ));
    }
    // Every record binding a task's own source the analysis does not admit, judged and lowered, refused by both as
    // rule 16 says ([`RULE_16_RECORD`]).
    if tally.rule_16.0 != tally.rule_16.1 {
        problems.push_str(&format!(
            "{universe}: {} records binding a task's own source rule 16 refuses judged, {} of them refused by the \
             checker and the lowering as the rule says\n",
            tally.rule_16.0, tally.rule_16.1
        ));
    }
    let missed: Vec<String> = exercised
        .iter()
        .filter_map(|(class, reach)| {
            let reached = tally.classes.get(class).copied().unwrap_or(0);
            match reach {
                Reach::Some if reached == 0 => Some(format!("{class:?}")),
                // A class held to be reached by no system is a branch to state unreachable, its reason in its name.
                Reach::Count(0) => Some(format!("{class:?}, held to be reached by no system")),
                Reach::Count(count) if *count != reached => {
                    Some(format!("{class:?}, reached {reached} times, held {count}"))
                }
                _ => None,
            }
        })
        .collect();
    // A branch the reading states unreachable, its reason in its name, reached by no system of any universe.
    let unreachable: Vec<(&&str, &usize)> = tally
        .classes
        .iter()
        .filter(|(class, _)| class.starts_with("unreachable: "))
        .collect();
    if !unreachable.is_empty() {
        problems.push_str(&format!(
            "{universe} reaches a branch the reading states unreachable: {unreachable:?}\n"
        ));
    }
    if !missed.is_empty() {
        problems.push_str(&format!(
            "{universe} no longer exercises as held: {}\n",
            missed.join("; ")
        ));
    }
    if !tally.findings.is_empty() {
        problems.push_str(&format!(
            "{universe}: {} findings, a record the configuration checker refuses or a plan the record's reading does \
             not give:\n{}",
            tally.findings.len(),
            tally.findings.join("\n")
        ));
    }
    assert!(problems.is_empty(), "{problems}");
}

/// Each system the search resolves no plan for, held by its exact label with its verdict, the reading deriving both:
/// `InfeasibleConfiguration` for each system model §8 rule 7's conflict takes ([`outside_hardware`]), "judged once for
/// the system whatever the assignment" and so before any assignment's finding, `conflicts` of them; and `MissingFact`
/// for each other of `cases` that `missing_fact`, a predicate over its construction, says no assignment can satisfy,
/// `missing` of them: a requester bound to hardware outside the system is an assignment's conflict, and nothing else
/// offering the fact, the search concludes `missing-fact` (rules 6 and 7). A system the language rejects for two tasks
/// of one name or for a task's duration is no search's, whatever else it writes ([`language_rejects`]).
fn hold_unresolved(
    universe: &str,
    tally: &Tally,
    cases: &[Case],
    (conflicts, missing): (usize, usize),
    missing_fact: impl Fn(&Case) -> bool,
) {
    let (mut infeasible, mut unknown) = (0, 0);
    let read: BTreeMap<String, String> = cases
        .iter()
        .filter(|c| !language_rejects(&c.description))
        .filter_map(|c| {
            let verdict = if !outside_hardware(&c.description).is_empty() {
                infeasible += 1;
                "InfeasibleConfiguration"
            } else if missing_fact(c) {
                unknown += 1;
                "MissingFact"
            } else {
                return None;
            };
            Some((c.label.clone(), verdict.to_string()))
        })
        .collect();
    assert_eq!(
        (infeasible, unknown),
        (conflicts, missing),
        "{universe}: the systems the reading derives, `infeasible-configuration` and `missing-fact`"
    );
    assert_eq!(
        read.len(),
        conflicts + missing,
        "{universe}: each label one system's"
    );
    assert_eq!(
        tally.unresolved_systems, read,
        "{universe}: the systems the search resolves no plan for, each with its verdict"
    );
}

#[test]
fn three_services_their_channels_sources_and_callers() {
    let cases = three_services();
    // 4 capabilities for each of 3 services, each of the 6 `uses` edges present or not, each of 3 services used by the
    // task or not.
    assert_eq!(cases.len(), 4usize.pow(3) * (1 << 6) * (1 << 3));
    let tally = run("three services", &cases);
    // Every system lowered, its one task writing `(deadline-from release)` and no jitter or overrun policy. Every
    // service served, by the task or the system's own level: a named edge for each `uses` edge written, and a component
    // of several members for each cycle among them — 39 of the 64 sets of edges, all but the 25 acyclic ones. The task
    // in a job wherever it uses a service, every service being a node.
    hold(
        "three services",
        &tally,
        (32_768, 32_768, 0, 0, 0),
        &[
            exactly("task: deadline-from written", 32_768),
            exactly("task: jitter zero where none is stated", 32_768),
            exactly("task: overrun fault where none is stated", 32_768),
            counted("initialisation: a named edge", &cases, |c| {
                c.description
                    .services
                    .iter()
                    .any(|s| !s.side.uses.is_empty())
            }),
            counted(
                "initialisation: a component of several members",
                &cases,
                |c| names_a_cycle(&c.description.services),
            ),
            exactly(
                "initialisation: a component of several members",
                64 * 39 * 8,
            ),
            counted(
                "owed: a service or a policy in a job, its call cost and masked runs",
                &cases,
                |c| !c.description.tasks[0].side.uses.is_empty(),
            ),
            some("context: a node in two interrupts"),
            some("multiplex: multiplexed"),
            some("multiplex: multiplexed by interrupts alone"),
            some("multiplex: a channel with one user"),
            some("multiplex: its own route no user"),
            some("multiplex: its own route a user, another node there reaching it"),
            some("multiplex: another route a user"),
        ],
    );
}

#[test]
fn two_services_with_adapter_instances() {
    let cases = two_services_with_adapters();
    // 3 shapes; `b` a service or a policy; names by `uses` or `needs`; 8 capabilities for each of 2 services; each of
    // 2 edges; each of 2 services named by the task or not.
    assert_eq!(cases.len(), 3 * 2 * 2 * 8usize.pow(2) * (1 << 2) * (1 << 2));
    let tally = run("two services with adapters", &cases);
    // Unresolved: the 256 over two timers of one channel each where one service takes a deadline and the other an
    // extended counter, the first naming the second (2 orders, 4 choices of their sources, 2 sets of edges holding
    // that one, 4 of the task's, 2 kinds of `b`, 2 clauses): the deadline's instance and the extender's take a channel
    // each, so at two devices, against model §8 rule 14's one time base. Every other lowered.
    let counter = |s: &Service, fact: &str| s.side.needs.iter().any(|n| n == fact);
    let deadline = |s: &Service| counter(s, "absolute-deadline");
    let extender = |s: &Service| counter(s, "counter-width");
    let channel = |s: &Service| counter(s, "relative-delay") || deadline(s) || extender(s);
    let source = |s: &Service| counter(s, INTERRUPT_FACT);
    let two_timers = |c: &Case| c.description.blocks.iter().any(|b| b.name == "timer.e");
    let one_provider_both = |c: &Case| c.description.blocks.len() == 1;
    let lowered = |c: &Case| {
        let [a, b] = [&c.description.services[0], &c.description.services[1]];
        let names = |x: &Service, y: &Service| {
            x.side
                .uses
                .iter()
                .chain(&x.side.needs)
                .any(|n| *n == y.name)
        };
        !(two_timers(c)
            && ((deadline(a) && extender(b) && names(a, b))
                || (extender(a) && deadline(b) && names(b, a))))
    };
    let any = |c: &Case, f: &dyn Fn(&Service) -> bool| c.description.services.iter().any(f);
    hold(
        "two services with adapters",
        &tally,
        (12_288, 12_032, 0, 256, 0),
        &[
            exactly("task: deadline-from written", 12_032),
            exactly("task: jitter zero where none is stated", 12_032),
            exactly("task: overrun fault where none is stated", 12_032),
            counted("instance: an obliged fact bare", &cases, |c| {
                lowered(c) && any(c, &deadline)
            }),
            counted("context: an expiry running its requester", &cases, |c| {
                lowered(c) && any(c, &deadline)
            }),
            counted("context: a wake running the instance alone", &cases, |c| {
                lowered(c) && any(c, &extender)
            }),
            counted("initialisation: a named edge", &cases, |c| {
                lowered(c)
                    && c.description.services.iter().any(|s| {
                        s.side
                            .uses
                            .iter()
                            .chain(&s.side.needs)
                            .any(|n| n == "a" || n == "b")
                    })
            }),
            counted(
                "initialisation: a component of several members",
                &cases,
                |c| lowered(c) && names_a_cycle(&c.description.services),
            ),
            // Round 20 (R20-R7): over two timers of one channel each, two services each taking a channel take one
            // at each, two timer routes — but two extended counters, which one shared instance over one timer serves;
            // one block raising both kinds, a service taking its channel beside one handling its source; one timer, a
            // deadline's instance and an extender's both programming it.
            counted(
                "route: two timer routes, by their interrupts' texts",
                &cases,
                |c| {
                    let [a, b] = [&c.description.services[0], &c.description.services[1]];
                    lowered(c)
                        && two_timers(c)
                        && channel(a)
                        && channel(b)
                        && !(extender(a) && extender(b))
                },
            ),
            counted(
                "route: a source and a timer route at one provider, two routes",
                &cases,
                |c| one_provider_both(c) && any(c, &channel) && any(c, &source),
            ),
            counted(
                "route: an expiry's and a wake's costs on one route, by their texts",
                &cases,
                |c| !two_timers(c) && any(c, &deadline) && any(c, &extender),
            ),
            // Round 24 (R24-D2): an instance a node of its kind, wherever a deadline or an extended counter makes one.
            counted("node: an instance", &cases, |c| {
                lowered(c) && (any(c, &deadline) || any(c, &extender))
            }),
            some("owed: an instance in a job, its call cost and masked runs"),
            some("owed: a service or a policy in a job, its call cost and masked runs"),
            some("owed: a block or a platform in a job, its call cost and masked runs"),
            some("owed: a node in no job, neither its call cost nor masked runs"),
            some("context: a node in two interrupts"),
            some("multiplex: multiplexed"),
            some("multiplex: multiplexed by interrupts alone"),
            some("multiplex: a policy multiplexed"),
            some("multiplex: a channel with one user"),
            some("multiplex: its own route no user"),
            some("multiplex: its own route a user, another node there reaching it"),
            some("multiplex: another route a user"),
        ],
    );
}

#[test]
fn releases_and_routes_of_every_separation_class() {
    let cases = releases_and_routes();
    // An alarm or none; `t` of 2 kinds; `u` absent or of 2 kinds. For each source, 6 separations; the first source has
    // 5 paths, 8 with task `u`, the second 3, or 5 with `u`.
    let per = |first: usize, second: usize| 6 * first + 6 * first * 6 * second;
    assert_eq!(cases.len(), 2 * 2 * (per(5, 3) + 2 * per(8, 5)));
    let tally = run("releases and routes", &cases);
    // The classes each system's construction fixes, read from its label: each source's separation and path, each
    // task's kind ([`release_classes`]), model §8 rule 16 deciding which source a task bound itself takes, or that the
    // search refuses the system.
    let fixed: Vec<BTreeSet<&str>> = cases.iter().map(|c| release_classes(&c.label)).collect();
    let count = |class: &'static str| fixed.iter().filter(|f| f.contains(class)).count();
    let fixed_count = |class: &'static str| exactly(class, count(class));
    // Refused by the search where rule 16 leaves a task bound itself no source (588 systems: 308 `unsupported-profile`,
    // 280 `missing-fact`), by the lowering where a periodic task is bound to a source itself (1,716), and every other
    // system lowered.
    let unresolved = count(RULE_16_SEARCH_CONFLICT) + count(RULE_16_SEARCH_UNKNOWN);
    let refused = count("refusal: a periodic task bound to a source itself");
    assert_eq!(
        (unresolved, refused),
        (588, 1_716),
        "the systems the construction refuses"
    );
    hold(
        "releases and routes",
        &tally,
        (14_184, 14_184 - 588 - 1_716, 0, 588, 1_716),
        &[
            fixed_count("refusal: a periodic task bound to a source itself"),
            exactly("refusal: a periodic task bound to a source itself", 1_716),
            exactly("task: deadline-from written", 11_880),
            exactly("task: jitter zero where none is stated", 11_880),
            exactly("task: overrun fault where none is stated", 11_880),
            fixed_count(RULE_16_SEARCH_CONFLICT),
            fixed_count(RULE_16_SEARCH_UNKNOWN),
            fixed_count(RULE_16_RECORD),
            fixed_count(RULE_16_UNAMENDED),
            fixed_count(RULE_16_SHORTER),
            fixed_count(RULE_16_ABSENT),
            fixed_count(RULE_16_UNSTATED),
            fixed_count(RULE_16_REBOUND),
            fixed_count("initialisation: a named edge"),
            fixed_count("trigger: every, equal separations"),
            fixed_count("trigger: every, a longer separation"),
            fixed_count("trigger: `(unknown (faster ITEM))`, a shorter separation"),
            fixed_count("trigger: `(unknown (unstated ITEM))`, no separation stated"),
            fixed_count("trigger: `(unknown (absent ITEM))`, the separation absent"),
            fixed_count("trigger: `(unknown unreached)`, no source reaching it"),
            fixed_count("trigger: `(unknown (several ITEM …))`, several sources"),
            fixed_count("trigger: its own source"),
            fixed_count("trigger: its own source, no other reaching it"),
            fixed_count("trigger: its own source, beside another reaching it"),
            fixed_count(
                "trigger: its own source releasing it every, beside one that alone would leave it unknown",
            ),
            fixed_count(
                "trigger: its own source releasing it every, beside another that alone would release it every from \
                 itself",
            ),
            fixed_count("trigger: its own source, reached through a requester its route names too"),
            fixed_count("trigger: binding none, beside a task bound to its own source"),
            fixed_count("trigger: binding none, reaching the handler of another task's own source"),
            fixed_count("rank: two stated separations tied"),
            fixed_count("rank: every class in one system"),
            // Round 24 (R24-R1): `(several ITEM …)`, "each source by its text" — `(source dev.a)` before `(source
            // dev.z)`, the order unlike the blocks' declaration, `dev.z` first, wherever several reach a task.
            exactly(
                "trigger: several sources, by their texts unlike their declaration order",
                count("trigger: `(unknown (several ITEM …))`, several sources"),
            ),
        ],
    );
}

/// A sporadic task's trigger as a universe's construction fixes it, from a source's separation as a label writes it
/// (`Stated(MS)`, `Unstated` or `Absent`) against the task's `own_ms`: whether every arrival releases it, and the class
/// of §2.2's rule it meets — the source the task binds itself where `bound_itself`, whose causes are `own-faster`,
/// `own-unstated` and `own-absent` (round 28, R28-D2), each unreachable since model §8 rule 16 landed (R29-R2), else
/// the one the routes leave.
fn separation_decides(
    separation: &str,
    own_ms: u64,
    bound_itself: bool,
) -> (&'static str, &'static str) {
    let stated = separation
        .strip_prefix("Stated(")
        .map(|ms| ms.trim_end_matches(')').parse::<u64>().expect("ms"));
    match (stated, bound_itself) {
        (Some(ms), _) if ms == own_ms => ("every", "trigger: every, equal separations"),
        (Some(ms), _) if ms > own_ms => ("every", "trigger: every, a longer separation"),
        (Some(_), false) => (
            "unknown",
            "trigger: `(unknown (faster ITEM))`, a shorter separation",
        ),
        (None, false) if separation == "Unstated" => (
            "unknown",
            "trigger: `(unknown (unstated ITEM))`, no separation stated",
        ),
        (None, false) => (
            "unknown",
            "trigger: `(unknown (absent ITEM))`, the separation absent",
        ),
        (Some(_), true) => ("unknown", OWN_FASTER),
        (None, true) if separation == "Unstated" => ("unknown", OWN_UNSTATED),
        (None, true) => ("unknown", OWN_ABSENT),
    }
}

/// Whether model §8 rule 16 admits a task of `own_ms` released on every arrival of a source of `separation`, as a label
/// writes it: a stated separation at least the task's.
fn admits(separation: &str, own_ms: u64) -> bool {
    separation_decides(separation, own_ms, false).0 == "every"
}

/// The class of [`RULE_16_RECORD`] a source of `separation`, as a label writes it, gives a task it does not admit.
fn rule_16_cause(separation: &str) -> &'static str {
    match separation {
        "Unstated" => RULE_16_UNSTATED,
        "Absent" => RULE_16_ABSENT,
        _ => RULE_16_SHORTER,
    }
}

/// The classes of the releases universe a system's construction fixes, from its label — `t=KIND u=KIND alarm=yes|no
/// sources=dev.X:SEPARATION:PATH …` — as the universe builds it: a periodic task bound to a source itself refused
/// (§1); a sporadic task binding `interrupt-source` itself — its path `DirectT` or `DirectU`, on the first source — bound
/// by the search to the first source declared that model §8 rule 16 admits for it, each source offering
/// `interrupt-source`: the first, `dev.z`, where it admits it, as the universe held before rule 16 landed, and else the
/// second, `dev.a`, the rule's choice; and where it admits neither, the search refusing the system, `missing-fact` where
/// one states no separation, else `unsupported-profile` (rules 16 and 6). Each record binding it to a source rule 16
/// does not admit is refused by the checker and the lowering ([`RULE_16_RECORD`]), the unamended rules' among them
/// wherever it does not admit `dev.z`, which those rules bound. Otherwise
/// each sporadic task's trigger by the source it binds itself, else by the sources whose routes reach it — through a
/// service it uses, or a service a service it uses names — each separation against its own, `t`'s 20 ms and `u`'s 40
/// ms (§2.2); a source's route where something is bound to it, a task bound itself, the system's own level or its
/// handler; two routed sources stating one separation tied, and every rank class with the alarm (§2.5); and a named
/// edge for each service naming another, `h.X` naming `evt.X`.
fn release_classes(label: &str) -> BTreeSet<&'static str> {
    let word = |key: &str| {
        label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a parameter")
    };
    let (t, u, alarm) = (word("t="), word("u="), word("alarm=") == "yes");
    // Each source, in declaration order: its block, its separation and its path.
    let sources: Vec<(&str, &str, &str)> = label
        .split(' ')
        .filter(|w| w.starts_with("dev.") || w.starts_with("sources=dev."))
        .map(|w| {
            let mut parts = w.trim_start_matches("sources=").split(':');
            (
                parts.next().expect("a block"),
                parts.next().expect("a separation"),
                parts.next().expect("a path"),
            )
        })
        .collect();
    let mut out = BTreeSet::new();
    let own = |task: &str| if task == "t" { "DirectT" } else { "DirectU" };
    if (t == "periodic" && sources.iter().any(|(_, _, p)| *p == own("t")))
        || (u == "periodic" && sources.iter().any(|(_, _, p)| *p == own("u")))
    {
        out.insert("refusal: a periodic task bound to a source itself");
        return out;
    }
    // Model §8 rule 16: the source each sporadic task binding `interrupt-source` itself is bound to.
    let mut own_source: BTreeMap<&str, &str> = BTreeMap::new();
    for (task, kind, ms) in [("t", t, 20), ("u", u, 40)] {
        let Some(&(first, separation, _)) = sources.iter().find(|(_, _, p)| *p == own(task)) else {
            continue;
        };
        if kind != "sporadic" {
            continue;
        }
        // Each record binding it to a source rule 16 does not admit, the unamended rules' binding `dev.z`.
        for (_, other, _) in &sources {
            if !admits(other, ms) {
                out.extend([RULE_16_RECORD, rule_16_cause(other)]);
            }
        }
        if !admits(separation, ms) {
            out.insert(RULE_16_UNAMENDED);
        }
        match sources.iter().find(|(_, s, _)| admits(s, ms)) {
            Some(&(name, _, _)) => {
                own_source.insert(task, name);
                if name != first {
                    out.insert(RULE_16_REBOUND);
                }
            }
            None => {
                out.insert(if sources.iter().any(|(_, s, _)| *s == "Unstated") {
                    RULE_16_SEARCH_UNKNOWN
                } else {
                    RULE_16_SEARCH_CONFLICT
                });
                return out;
            }
        }
    }
    // A source reaches a task through its handler `evt.X`, which the task uses, or uses through `h.X`.
    let through_handler = |task: &str, path: &str| match task {
        "t" => matches!(path, "ServiceT" | "ChainT" | "ServiceTU"),
        _ => matches!(path, "ServiceU" | "ServiceTU"),
    };
    for (task, kind, ms) in [("t", t, 20), ("u", u, 40)] {
        if kind != "sporadic" {
            continue;
        }
        let reaching: Vec<&(&str, &str, &str)> = sources
            .iter()
            .filter(|(name, _, path)| {
                own_source.get(task) == Some(name) || through_handler(task, path)
            })
            .collect();
        if let Some(&mine) = own_source.get(task) {
            let (_, separation, path) = sources
                .iter()
                .find(|(n, _, _)| *n == mine)
                .expect("a source");
            let decided = separation_decides(separation, ms, true);
            out.extend(["trigger: its own source", decided.1]);
            let others: Vec<&&(&str, &str, &str)> =
                reaching.iter().filter(|(n, _, _)| *n != mine).collect();
            out.insert(if others.is_empty() {
                "trigger: its own source, no other reaching it"
            } else {
                "trigger: its own source, beside another reaching it"
            });
            if let [(_, other, _)] = others.as_slice() {
                out.insert(match (decided.0, separation_decides(other, ms, false).0) {
                    ("unknown", "every") => OWN_UNKNOWN_BESIDE_EVERY,
                    ("every", "unknown") => {
                        "trigger: its own source releasing it every, beside one that alone would leave it unknown"
                    }
                    ("every", _) => {
                        "trigger: its own source releasing it every, beside another that alone would release it every \
                         from itself"
                    }
                    _ => OWN_UNKNOWN_BESIDE_UNKNOWN,
                });
            }
            // Its own source's route names the handler the task uses: `dev.a`, taken in place of `dev.z`.
            if through_handler(task, path) {
                out.insert(
                    "trigger: its own source, reached through a requester its route names too",
                );
            }
        } else {
            let other = if task == "t" { "u" } else { "t" };
            if let Some(other_own) = own_source.get(other) {
                out.insert("trigger: binding none, beside a task bound to its own source");
                if reaching.iter().any(|(n, _, _)| n == other_own) {
                    out.insert(
                        "trigger: binding none, reaching the handler of another task's own source",
                    );
                }
            }
            out.insert(match reaching.as_slice() {
                [] => "trigger: `(unknown unreached)`, no source reaching it",
                [(_, separation, _)] => separation_decides(separation, ms, false).1,
                _ => "trigger: `(unknown (several ITEM …))`, several sources",
            });
        }
    }
    // A source's route, where something is bound to it for `interrupt-source`: a task bound itself, or, on every other
    // path, the system's own level or its handler.
    let routed: Vec<&str> = sources
        .iter()
        .filter(|(name, _, path)| {
            own_source.values().any(|s| s == name) || !matches!(*path, "DirectT" | "DirectU")
        })
        .map(|(_, separation, _)| *separation)
        .collect();
    let stated: Vec<&str> = routed
        .iter()
        .copied()
        .filter(|separation| separation.starts_with("Stated("))
        .collect();
    if stated.len() == 2 && stated[0] == stated[1] {
        out.insert("rank: two stated separations tied");
    }
    if alarm && !stated.is_empty() && stated.len() < routed.len() {
        out.insert("rank: every class in one system");
    }
    if sources.iter().any(|(_, _, p)| *p == "ChainT") {
        out.insert("initialisation: a named edge");
    }
    out
}

#[test]
fn requesters_of_every_kind_refused_or_lowered() {
    let cases = requesters_of_every_kind();
    // 4 requesters, each requiring one of 4 things.
    assert_eq!(cases.len(), 4 * 4);
    let tally = run("requesters of every kind", &cases);
    // Of each requester, periodic task, sporadic task, the system's own level and a service, requiring an alarm, a
    // deadline, an extended counter or the source: the three that program an alarm or take an expiry themselves refused,
    // naming one receiver each (§1), and the periodic task bound to the source; the 9 others lowered — every extended
    // counter, its wake running the instance alone; the sporadic task's own source, 20 ms against its 20 ms; the system's
    // own level's source; and the service's four.
    hold(
        "requesters of every kind",
        &tally,
        (16, 9, 0, 0, 7),
        &[
            exactly("refusal: a periodic task bound to a source itself", 1),
            exactly("receivers: an alarm programmed itself", 3),
            exactly("receivers: an expiry", 3),
            exactly("receivers: one", 6),
            exactly("receivers: the system's own level alone", 2),
            exactly("instance: an obliged fact bare", 1),
            exactly("context: a wake running the instance alone", 4),
            exactly("context: an expiry running its requester", 1),
            exactly("trigger: every, equal separations", 1),
            exactly("trigger: its own source", 1),
            exactly("trigger: its own source, no other reaching it", 1),
            exactly("trigger: `(unknown unreached)`, no source reaching it", 1),
            exactly("task: deadline-from written", 9),
            exactly("task: jitter zero where none is stated", 9),
            exactly("task: overrun fault where none is stated", 9),
        ],
    );
}

#[test]
fn two_tasks_sharing_services_that_take_a_channel() {
    let cases = two_tasks_sharing_services();
    // 8 capabilities for each of 2 services, each of 2 `uses` edges, each of 2 services used by `t` or not and by `u`
    // or not, and each task writing `(deadline-from release)` or not.
    assert_eq!(
        cases.len(),
        8usize.pow(2) * (1 << 2) * (1 << 2) * (1 << 2) * 4
    );
    let tally = run("two tasks sharing services", &cases);
    // Every system lowered. A task writing `(deadline-from release)` in 3 of 4 choices, one owing it in 3 of 4; a
    // deadline's instance, bounded or bare, in 28 of the 64 capabilities for each kind, and one or the other in 48; a
    // named edge in 3 of the 4 sets of edges, a cycle in 1. The sporadic `u` released by `dev.s`'s 20 ms, its own 20 ms,
    // wherever the services it uses reach one handling the source, and otherwise by none.
    let needs = |s: &Service, fact: &str| s.side.needs.iter().any(|n| n == fact);
    let bound = |s: &Service| !s.side.at_most.is_empty();
    let reaches_source = |c: &Case| {
        uses_closure(&c.description.services, &c.description.tasks[1].side.uses)
            .iter()
            .any(|n| {
                c.description
                    .services
                    .iter()
                    .any(|s| &s.name == n && needs(s, INTERRUPT_FACT))
            })
    };
    hold(
        "two tasks sharing services",
        &tally,
        (16_384, 16_384, 0, 0, 0),
        &[
            exactly("task: deadline-from written", 12_288),
            exactly("task: deadline-from owed", 12_288),
            exactly("task: jitter zero where none is stated", 16_384),
            exactly("task: overrun fault where none is stated", 16_384),
            counted("trigger: every, equal separations", &cases, reaches_source),
            counted(
                "trigger: `(unknown unreached)`, no source reaching it",
                &cases,
                |c| !reaches_source(c),
            ),
            counted("instance: an obliged fact with its bound", &cases, |c| {
                c.description
                    .services
                    .iter()
                    .any(|s| needs(s, "absolute-deadline") && bound(s))
            }),
            counted("instance: an obliged fact bare", &cases, |c| {
                c.description
                    .services
                    .iter()
                    .any(|s| needs(s, "absolute-deadline") && !bound(s))
            }),
            exactly("context: an expiry running its requester", 48 * 16_384 / 64),
            exactly("initialisation: a named edge", 12_288),
            exactly("initialisation: a component of several members", 4_096),
            some("context: a node in two interrupts"),
            some("multiplex: multiplexed"),
            some("multiplex: multiplexed by two jobs"),
            some("multiplex: a channel with one user"),
            some("multiplex: its own route no user"),
            some("multiplex: its own route a user, another node there reaching it"),
            some("multiplex: another route a user"),
        ],
    );
}

#[test]
fn receivers_of_interrupt_code_each_named_in_order() {
    let cases = receivers_of_interrupt_code();
    // What `t`, `u` and the system's own level each program, of 3; which task is first; each task of 2 kinds; each
    // writing `(deadline-from release)` or not.
    assert_eq!(cases.len(), 3usize.pow(3) * 2 * 4 * 4);
    let tally = run("receivers of interrupt code", &cases);
    // Of the 27 choices of what `t`, `u` and the system's own level program, each over 32 of the other choices: the one
    // programming nothing lowered; 6 with one receiver, 12 with two, 8 with three; 19 with an alarm and 19 with an
    // expiry; 2 with the system's own level alone receiving, 16 with it and a task; and 4 with both tasks receiving,
    // over 3 of the system's own level's, `u` first in half of their 32. Of the 32 lowered, a task writes
    // `(deadline-from release)` in 24, one owes it in 24, and a task is sporadic, bound to no source, in 24.
    hold(
        "receivers of interrupt code",
        &tally,
        (864, 32, 0, 0, 832),
        &[
            exactly("task: deadline-from written", 24),
            exactly("task: deadline-from owed", 24),
            exactly("task: jitter zero where none is stated", 32),
            exactly("task: overrun fault where none is stated", 32),
            exactly("trigger: `(unknown unreached)`, no source reaching it", 24),
            exactly("receivers: one", 6 * 32),
            exactly("receivers: two", 12 * 32),
            exactly("receivers: three", 8 * 32),
            exactly("receivers: an alarm programmed itself", 19 * 32),
            exactly("receivers: an expiry", 19 * 32),
            exactly("receivers: the system's own level alone", 2 * 32),
            exactly("receivers: tasks and the system's own level", 16 * 32),
            exactly(
                "receivers: runtime order unlike the tasks' text order",
                4 * 3 * 16,
            ),
        ],
    );
}

#[test]
fn requiring_hardware_its_nodes_contexts_and_multiplexing() {
    let cases = requiring_hardware();
    // What `dma.x` and `soc` each require, of 5; who binds to each, of 8; what composes `alarm`, of 3; `u` of 2 kinds.
    assert_eq!(cases.len(), 5 * 5 * 8 * 8 * 3 * 2);
    let tally = run("requiring hardware", &cases);
    // A third of it lowered: `alarm` named by `dma.x` or `soc` is refused (§1, round 13). Of the lowered, each count
    // read from what `dma.x` and `soc` require and who binds to them, as the label writes it: requiring hardware bound to
    // by its binder or by nothing (neither binds the other's offer); an alarm, an expiry or a handler of the source
    // where either requires one; an edge to whichever requires anything; `soc` bound to a block it composes for an
    // alarm, the source or an output, `timer.d` and `uart.c` texts above `soc`'s, `dev.s`'s below.
    let part = |c: &Case, who: &str| -> (String, String) {
        let at = c.label.find(&format!("{who}:")).expect("a part") + who.len() + 1;
        let rest = &c.label[at..];
        let requires = rest.split(' ').next().expect("requires").to_string();
        let binder = rest
            .split(" by ")
            .nth(1)
            .and_then(|b| b.split(';').next())
            .expect("a binder")
            .to_string();
        (requires, binder)
    };
    let lowered = |c: &Case| c.label.contains("alarm composed by Nothing");
    let requires = |c: &Case, what: &str| ["dma.x", "soc"].iter().any(|w| part(c, w).0 == what);
    let requiring = |c: &Case, bound: bool| {
        ["dma.x", "soc"].iter().any(|w| {
            let (requires, binder) = part(c, w);
            requires != "Nothing" && (binder != "Nothing") == bound
        })
    };
    hold(
        "requiring hardware",
        &tally,
        (9_600, 3_200, 0, 0, 6_400),
        &[
            exactly("refusal: hardware naming what is not hardware", 6_400),
            exactly("operand: a block's `uses` naming a service", 3_200),
            exactly("operand: a platform's `uses` naming a service", 3_200),
            counted(
                "hardware: a block bound for requirements of its own",
                &cases,
                |c| lowered(c) && part(c, "dma.x").0 != "Nothing",
            ),
            counted(
                "hardware: a platform bound for requirements of its own",
                &cases,
                |c| lowered(c) && part(c, "soc").0 != "Nothing",
            ),
            counted(
                "hardware: requiring, and bound to by nothing",
                &cases,
                |c| lowered(c) && requiring(c, false),
            ),
            counted(
                "hardware: requiring, and bound to by a requester",
                &cases,
                |c| lowered(c) && requiring(c, true),
            ),
            counted("hardware: an alarm it programs itself", &cases, |c| {
                lowered(c) && requires(c, "Alarm")
            }),
            counted("hardware: an expiry running its code", &cases, |c| {
                lowered(c) && requires(c, "Deadline")
            }),
            counted("context: an expiry running its requester", &cases, |c| {
                lowered(c) && requires(c, "Deadline")
            }),
            counted("instance: an obliged fact bare", &cases, |c| {
                lowered(c) && requires(c, "Deadline")
            }),
            counted("hardware: a source's handler", &cases, |c| {
                lowered(c) && requires(c, "Source")
            }),
            counted(
                "initialisation: an edge to a block or a platform bound for its own requirements",
                &cases,
                |c| {
                    lowered(c) && (part(c, "dma.x").0 != "Nothing" || part(c, "soc").0 != "Nothing")
                },
            ),
            counted(
                "initialisation: a composite bound to its part, its edge kept",
                &cases,
                |c| {
                    lowered(c) && ["Alarm", "Source", "Output"].contains(&part(c, "soc").0.as_str())
                },
            ),
            counted(
                "initialisation: an edge across composition ordering two steps unlike their texts",
                &cases,
                |c| lowered(c) && ["Alarm", "Output"].contains(&part(c, "soc").0.as_str()),
            ),
            exactly("multiplex: a channel with one user", 3_200),
            exactly("multiplex: its own route no user", 3_200),
            exactly(
                "owed: a service or a policy in a job, its call cost and masked runs",
                3_200,
            ),
            exactly(
                "owed: a block or a platform in a job, its call cost and masked runs",
                3_200,
            ),
            exactly("task: deadline-from written", 3_200),
            some("hardware: in an interrupt through a binding to it"),
            some("multiplex: a block or a platform multiplexed"),
            some("multiplex: multiplexed by two jobs"),
            some("multiplex: multiplexed by a job and an interrupt"),
            some("multiplex: a block or a platform's channel with one user"),
            some("multiplex: another route a user"),
            some("composition: a platform a task reaches through a binding"),
            some("composition: what a node in a job composes is in none"),
            some("composition: what a node in an interrupt composes is not run there"),
            some("trigger: a source a block or a platform handles"),
            some("trigger: every, equal separations"),
            some("trigger: `(unknown unreached)`, no source reaching it"),
            // Round 24 (R24-D2): the platform a node of its kind, bound for requirements of its own or a provider.
            counted("node: a platform", &cases, |c| {
                let (requires, binder) = part(c, "soc");
                lowered(c) && (requires != "Nothing" || binder != "Nothing")
            }),
        ],
    );
}

#[test]
fn task_clauses_jitter_overrun_and_reference() {
    let cases = task_clauses();
    // Which task is first; for each of 2 tasks, 5 jitters, 3 overrun policies and 3 references.
    assert_eq!(cases.len(), 2 * (5 * 3 * 3) * (5 * 3 * 3));
    let tally = run("task clauses", &cases);
    // Each count read from the tasks' clauses as the universe writes them: refused, a jitter past the arithmetic first,
    // then a deadline from another event, task by task in runtime order (§1); every other lowered, its task lines
    // reading each clause.
    let tasks = |c: &Case| c.description.tasks.clone();
    let past = |t: &Task| t.jitter.is_some_and(|j| j.seconds().is_none());
    let other_event = |t: &Task| t.deadline_from == Some("activation");
    let lowered = |c: &Case| tasks(c).iter().all(|t| !past(t) && !other_event(t));
    let any = |c: &Case, f: &dyn Fn(&Task) -> bool| lowered(c) && tasks(c).iter().any(f);
    let first_runtime_named = |c: &Case| {
        let mut by_rank = tasks(c);
        by_rank.sort_by_key(Task::rank);
        !tasks(c).iter().any(past)
            && by_rank
                .iter()
                .find(|t| other_event(t))
                .map(|t| t.name.clone())
                != Some(tasks(c)[0].name.clone())
            && other_event(&tasks(c)[0])
    };
    hold(
        "task clauses",
        &tally,
        (4_050, 1_152, 0, 0, 2_898),
        &[
            counted("task: jitter zero where none is stated", &cases, |c| any(c, &|t| t.jitter.is_none())),
            counted("task: jitter written zero", &cases, |c| any(c, &|t| t.jitter == Some(Dur::Ms(0)))),
            counted("task: jitter written", &cases, |c| {
                any(c, &|t| t.jitter == Some(Dur::Spelled("1500 us")))
            }),
            counted("task: jitter longer than its release's duration", &cases, |c| {
                any(c, &|t| t.jitter == Some(Dur::Ms(50)))
            }),
            counted("task: overrun fault where none is stated", &cases, |c| {
                any(c, &|t| t.overrun.is_none())
            }),
            counted("task: overrun fault as written", &cases, |c| {
                any(c, &|t| t.overrun == Some("fault"))
            }),
            counted("task: overrun another policy as written", &cases, |c| {
                any(c, &|t| t.overrun == Some("skip-late-job"))
            }),
            counted("task: deadline-from written", &cases, |c| {
                any(c, &|t| t.deadline_from == Some("release"))
            }),
            counted("task: deadline-from owed", &cases, |c| any(c, &|t| t.deadline_from.is_none())),
            counted("refusal: a deadline measured from another event", &cases, |c| {
                !tasks(c).iter().any(past) && tasks(c).iter().any(other_event)
            }),
            counted(
                "deadline: the first in runtime order named, an earlier declared one measured from another event \
                 too",
                &cases,
                first_runtime_named,
            ),
            counted("refusal: a duration past the arithmetic", &cases, |c| tasks(c).iter().any(past)),
            counted("past: a task's clause", &cases, |c| tasks(c).iter().any(past)),
            counted("past: several, named in order", &cases, |c| tasks(c).iter().all(past)),
            counted("past: before a deadline from another event", &cases, |c| {
                tasks(c).iter().any(past) && tasks(c).iter().any(other_event)
            }),
            exactly("trigger: every, equal separations", 1_152),
            exactly("owed: a block or a platform in a job, its call cost and masked runs", 1_152),
            exactly("owed: a service or a policy in a job, its call cost and masked runs", 1_152),
        ],
    );
}

#[test]
fn durations_past_the_arithmetic_and_at_its_edge() {
    let cases = past_the_arithmetic();
    // Values plain or at the edge; which task is first; `t`'s three clauses each past or not, `u`'s all or none; each
    // of 2 sources of 4 states; each of 2 bounds of 3.
    assert_eq!(cases.len(), 2 * 2 * 8 * 2 * 4 * 4 * 3 * 3);
    let tally = run("past the arithmetic", &cases);
    // Each count read from the label's parameters: a duration past the arithmetic is named where the plan would write
    // it — every task clause, each source a handler binds, `PastUnbound`'s none, each bound — and the 64 with none
    // lowered (2 values, 2 orders, each source within or past and bound by nothing, each bound bare or within); at the
    // edge, each value its denominator past 64 bits, and `u`'s trigger decided by `dev!`'s separation against its own,
    // longer at the edge, equal plain, unknown where `dev!` is bound by nothing.
    let has = |c: &Case, word: &str| c.label.contains(word);
    let edge = |c: &Case| c.label.starts_with("edge ");
    let task_items = |c: &Case| {
        ["period,", "deadline,", "jitter"]
            .iter()
            .filter(|w| {
                c.label
                    .split(" u-past=")
                    .next()
                    .is_some_and(|t| t.contains(**w))
            })
            .count()
            + if has(c, "u-past=true") { 3 } else { 0 }
    };
    let source_past = |c: &Case, which: &str| {
        has(c, &format!("{which}:Past ")) || has(c, &format!("{which}:Overflowing "))
    };
    let sources =
        |c: &Case| usize::from(source_past(c, "dev")) + usize::from(source_past(c, "dev!"));
    let bounds =
        |c: &Case| usize::from(has(c, "dl:Past ")) + usize::from(c.label.ends_with("dl!:Past"));
    let lowered = |c: &Case| task_items(c) + sources(c) + bounds(c) == 0;
    let within = |c: &Case, which: &str| has(c, &format!("{which}:Within "));
    hold(
        "past the arithmetic",
        &tally,
        (9_216, 64, 0, 0, 9_152),
        &[
            counted("refusal: a duration past the arithmetic", &cases, |c| {
                !lowered(c)
            }),
            counted("past: a task's clause", &cases, |c| task_items(c) > 0),
            counted("past: a source's separation", &cases, |c| sources(c) > 0),
            counted("past: an obliged bound", &cases, |c| bounds(c) > 0),
            counted("past: an obliged bound of a service", &cases, |c| {
                bounds(c) > 0
            }),
            counted(
                "past: a task's, a source's and a bound at once",
                &cases,
                |c| task_items(c) > 0 && sources(c) > 0 && bounds(c) > 0,
            ),
            counted("past: several, named in order", &cases, |c| {
                task_items(c) + sources(c) + bounds(c) >= 2
            }),
            counted(
                "past: the sources' texts ordered unlike their providers' names",
                &cases,
                |c| sources(c) == 2,
            ),
            counted(
                "past: the bounds' instances ordered unlike their requesters' names",
                &cases,
                |c| bounds(c) == 2,
            ),
            counted(
                "exact: a task's duration at the arithmetic's edge",
                &cases,
                |c| lowered(c) && edge(c),
            ),
            counted(
                "exact: a separation at the arithmetic's edge",
                &cases,
                |c| lowered(c) && edge(c) && (within(c, "dev") || within(c, "dev!")),
            ),
            counted(
                "exact: an obliged bound at the arithmetic's edge",
                &cases,
                |c| {
                    lowered(c)
                        && edge(c)
                        && (has(c, "dl:Within ") || c.label.ends_with("dl!:Within"))
                },
            ),
            counted(
                "exact: two separations compared past 128 bits",
                &cases,
                |c| lowered(c) && edge(c) && within(c, "dev") && within(c, "dev!"),
            ),
            counted(
                "exact: a source's and a task's separations compared past 128 bits",
                &cases,
                |c| lowered(c) && edge(c) && within(c, "dev!"),
            ),
            counted("trigger: every, a longer separation", &cases, |c| {
                lowered(c) && edge(c) && within(c, "dev!")
            }),
            counted("trigger: every, equal separations", &cases, |c| {
                lowered(c) && !edge(c) && within(c, "dev!")
            }),
            counted(
                "trigger: `(unknown unreached)`, no source reaching it",
                &cases,
                |c| lowered(c) && !within(c, "dev!"),
            ),
            counted("instance: an obliged fact with its bound", &cases, |c| {
                lowered(c) && (has(c, "dl:Within ") || c.label.ends_with("dl!:Within"))
            }),
            counted("instance: a service's obliged bound", &cases, |c| {
                lowered(c) && (has(c, "dl:Within ") || c.label.ends_with("dl!:Within"))
            }),
            counted("instance: an obliged fact bare", &cases, |c| {
                lowered(c) && (has(c, "dl:Bare ") || c.label.ends_with("dl!:Bare"))
            }),
            counted("task: jitter written", &cases, |c| lowered(c) && edge(c)),
            exactly("task: deadline-from written", 64),
            exactly("context: an expiry running its requester", 64),
        ],
    );
}

#[test]
fn a_second_system_named_or_not() {
    let cases = two_systems();
    // 5 naming sites, each present or not; by `uses` or `needs`; `alarm` used by one system or both; `other` on `sys`'s
    // platform or its own; `sys`'s own level of 2 and `other`'s of 3; each description lowered for each system.
    assert_eq!(cases.len(), (1 << 5) * 2 * 2 * 2 * 2 * 3 * 2);
    for pair in cases.chunks(2) {
        assert_eq!(
            pair[0].description.text(),
            pair[1].description.text(),
            "one description, lowered for each of its systems"
        );
    }
    let tally = run("a second system", &cases);
    // Each count read from the label: the sites a lowered system serves — `sys` its task's, its own level's and
    // `bridge.s`'s, `other` its task's and `bridge.o`'s, and, once it names `sys`, `sys`'s own level's — each a pair of
    // the naming-system refusal (§1); a system with none lowered, the other system's sites then naming outside it. Over
    // 48 choices of the rest for each set of sites.
    let site = |c: &Case, name: &str| {
        c.label
            .split(' ')
            .next()
            .and_then(|w| w.strip_prefix("naming="))
            .is_some_and(|sites| sites.split(',').any(|s| s == name))
    };
    let lowering_sys = |c: &Case| c.label.ends_with("lowering sys");
    let pairs = |c: &Case| -> usize {
        if lowering_sys(c) {
            ["t>other", "sys>other", "bridge.s>other"]
                .iter()
                .filter(|s| site(c, s))
                .count()
        } else {
            let own = ["o>sys", "bridge.o>sys"]
                .iter()
                .filter(|s| site(c, s))
                .count();
            own + usize::from(own > 0 && site(c, "sys>other"))
        }
    };
    let lowered = |c: &Case| pairs(c) == 0;
    let other_names = |c: &Case| {
        if lowering_sys(c) {
            site(c, "o>sys") || site(c, "bridge.o>sys")
        } else {
            site(c, "t>other") || site(c, "sys>other") || site(c, "bridge.s>other")
        }
    };
    let own_platform = |c: &Case| c.label.contains("other on host.q");
    let shared = |c: &Case| c.label.contains("alarm=both");
    hold(
        "a second system",
        &tally,
        (3_072, 576, 0, 0, 2_496),
        &[
            exactly("refusal: a requester naming a system", 2_496),
            counted("naming: a task naming a system", &cases, |c| {
                if lowering_sys(c) {
                    site(c, "t>other")
                } else {
                    site(c, "o>sys")
                }
            }),
            counted(
                "naming: the system's own level naming a system",
                &cases,
                |c| lowering_sys(c) && site(c, "sys>other"),
            ),
            counted("naming: a service naming a system", &cases, |c| {
                if lowering_sys(c) {
                    site(c, "bridge.s>other")
                } else {
                    site(c, "bridge.o>sys")
                }
            }),
            counted(
                "naming: a named system's own level naming a system",
                &cases,
                |c| !lowering_sys(c) && pairs(c) > 1 && site(c, "sys>other"),
            ),
            counted("naming: the lowered system itself named", &cases, |c| {
                !lowering_sys(c) && pairs(c) > 1 && site(c, "sys>other")
            }),
            counted("naming: several names", &cases, |c| pairs(c) >= 2),
            counted(
                "naming: a system another system's requester names, outside this one's",
                &cases,
                |c| lowered(c) && other_names(c),
            ),
            exactly(
                "second system: a service another system's task also uses",
                576,
            ),
            counted(
                "second system: requiring hardware the lowered system does not serve",
                &cases,
                |c| lowered(c) && lowering_sys(c) && own_platform(c),
            ),
            counted(
                "hardware: a block bound for requirements of its own",
                &cases,
                |c| lowered(c) && !lowering_sys(c) && own_platform(c),
            ),
            counted("hardware: an alarm it programs itself", &cases, |c| {
                lowered(c) && !lowering_sys(c) && own_platform(c)
            }),
            counted("multiplex: a channel with one user", &cases, |c| {
                lowered(c) && (lowering_sys(c) || shared(c))
            }),
            counted("multiplex: its own route no user", &cases, |c| {
                lowered(c)
                    && (lowering_sys(c)
                        || shared(c)
                        || c.label.ends_with("other-own=2: lowering other")
                        || own_platform(c))
            }),
            exactly("task: deadline-from written", 576),
        ],
    );
}

#[test]
fn hardware_naming_what_is_not_hardware_refused_in_its_place() {
    let cases = hardware_naming();
    // What `timer.d` names, of 3; `dma.x`, of 10; `soc`, of 5; `blk.q`, of 2; the other refusal `t` meets, of 6.
    assert_eq!(cases.len(), 3 * 10 * 5 * 2 * 6);
    let tally = run("hardware naming", &cases);
    // Lowered: the 20 whose hardware `sys` serves names hardware alone — `timer.d` nothing, `dma.x` nothing or a block
    // (5), `soc` nothing or a block (2) — whatever `blk.q`, which `sys` does not serve, names (2; round 15, R15-R2), and
    // whose task `t` meets no other refusal; every other refused. Each count read from the label's namings and `t`'s
    // other refusal, §1's bullets taken in their order: the periodic task first, then a system named — by `t` or by
    // `dma.x` — then the hardware's operands, then the second list.
    let named = |c: &Case, namer: &str| -> Vec<String> {
        let at = c.label.find(&format!("{namer} names [")).expect("a namer") + namer.len() + 8;
        let inside = &c.label[at..at + c.label[at..].find(']').expect("closed")];
        inside
            .split(',')
            .filter(|w| !w.is_empty())
            .map(str::to_string)
            .collect()
    };
    let also = |c: &Case| c.label.rsplit(' ').next().expect("also").to_string();
    let other_kind = |item: &String| {
        let name = item.split(':').nth(1).expect("a name");
        ["alarm", "only.s", "pol.p", "other"].contains(&name)
    };
    let served = ["timer.d", "dma.x", "soc"];
    let operands = |c: &Case| -> Vec<(String, String)> {
        served
            .iter()
            .flat_map(|namer| {
                named(c, namer)
                    .into_iter()
                    .filter(other_kind)
                    .map(|item| ((*namer).to_string(), item))
            })
            .collect()
    };
    let names_system = |c: &Case| {
        also(c) == "NamesSystem" || named(c, "dma.x").contains(&"uses:other".to_string())
    };
    let given = |c: &Case| -> &'static str {
        if also(c) == "PeriodicSource" {
            "periodic"
        } else if names_system(c) {
            "naming"
        } else if !operands(c).is_empty() {
            "operand"
        } else {
            match also(c).as_str() {
                "JitterPast" => "past",
                "DeadlineFromOther" => "deadline",
                "Receiver" => "receivers",
                _ => "lowered",
            }
        }
    };
    let operand = |c: &Case, who: &str, item: &str| {
        given(c) == "operand" && named(c, who).contains(&item.to_string())
    };
    let lowered = |c: &Case| given(c) == "lowered";
    let dma = |c: &Case, item: &str| named(c, "dma.x").contains(&item.to_string());
    hold(
        "hardware naming",
        &tally,
        (1_800, 20, 0, 0, 1_780),
        &[
            counted("refusal: a periodic task bound to a source itself", &cases, |c| given(c) == "periodic"),
            counted("refusal: a requester naming a system", &cases, |c| given(c) == "naming"),
            counted("refusal: hardware naming what is not hardware", &cases, |c| given(c) == "operand"),
            counted("refusal: a duration past the arithmetic", &cases, |c| given(c) == "past"),
            counted("refusal: a deadline measured from another event", &cases, |c| given(c) == "deadline"),
            counted("receivers: one", &cases, |c| given(c) == "receivers"),
            counted("periodic: before hardware naming what is not hardware", &cases, |c| {
                given(c) == "periodic" && !operands(c).is_empty()
            }),
            counted("task bullet: before a requester naming a system", &cases, |c| {
                given(c) == "periodic" && dma(c, "uses:other")
            }),
            counted("naming: a task naming a system", &cases, |c| {
                given(c) == "naming" && also(c) == "NamesSystem"
            }),
            counted("naming: a block or a platform naming a system", &cases, |c| {
                given(c) == "naming" && dma(c, "uses:other")
            }),
            counted("naming: several names", &cases, |c| {
                given(c) == "naming" && also(c) == "NamesSystem" && dma(c, "uses:other")
            }),
            counted("naming: before hardware naming what is not hardware", &cases, |c| {
                given(c) == "naming" && !operands(c).is_empty()
            }),
            counted("operand: a block's `uses` naming a service", &cases, |c| {
                operand(c, "timer.d", "uses:alarm") || operand(c, "dma.x", "uses:only.s")
            }),
            counted("operand: a block's `needs` naming a service", &cases, |c| {
                operand(c, "timer.d", "needs:alarm") || operand(c, "dma.x", "needs:only.s")
            }),
            counted("operand: a block's `uses` naming a policy", &cases, |c| operand(c, "dma.x", "uses:pol.p")),
            counted("operand: a block's `needs` naming a policy", &cases, |c| {
                operand(c, "dma.x", "needs:pol.p")
            }),
            counted("operand: a platform's `uses` naming a service", &cases, |c| {
                operand(c, "soc", "uses:only.s") || operand(c, "soc", "uses:alarm")
            }),
            counted("operand: a platform's `needs` naming a policy", &cases, |c| operand(c, "soc", "needs:pol.p")),
            counted("operand: a block naming the service bound to it", &cases, |c| {
                given(c) == "operand" && !named(c, "timer.d").is_empty()
            }),
            counted("operand: a service only hardware names", &cases, |c| {
                operand(c, "dma.x", "uses:only.s")
                    || operand(c, "dma.x", "needs:only.s")
                    || operand(c, "soc", "uses:only.s")
            }),
            counted(
                "operand: hardware the system does not serve naming what is not hardware too, not named",
                &cases,
                |c| given(c) == "operand" && !named(c, "blk.q").is_empty(),
            ),
            counted("operand: several, named in order", &cases, |c| {
                given(c) == "operand" && operands(c).len() >= 2
            }),
            counted("operand: one namer naming two", &cases, |c| {
                given(c) == "operand" && served.iter().any(|n| named(c, n).iter().filter(|i| other_kind(i)).count() >= 2)
            }),
            counted("operand: before a duration past the arithmetic", &cases, |c| {
                given(c) == "operand" && also(c) == "JitterPast"
            }),
            counted("operand: before a deadline from another event", &cases, |c| {
                given(c) == "operand" && also(c) == "DeadlineFromOther"
            }),
            counted("operand: before a receiver of interrupt code", &cases, |c| {
                given(c) == "operand" && also(c) == "Receiver"
            }),
            counted("unserved: a block naming a service", &cases, |c| lowered(c) && !named(c, "blk.q").is_empty()),
            exactly("hardware: a block bound for requirements of its own", 20),
            counted("hardware: a block's or a platform's `needs` naming a block", &cases, |c| {
                lowered(c)
                    && (["needs:uart.c", "needs:timer.d", "needs:blk.n"].iter().any(|i| dma(c, i))
                        || named(c, "soc").contains(&"needs:dev.s".to_string()))
            }),
            counted("hardware: what a node in a job names by `needs` is in none", &cases, |c| {
                lowered(c) && dma(c, "needs:blk.n")
            }),
            counted("initialisation: a composite bound to its part, its edge kept", &cases, |c| {
                lowered(c) && dma(c, "uses:uart.c")
            }),
            counted("initialisation: a block composing a block, an edge across it kept", &cases, |c| {
                lowered(c) && dma(c, "uses:uart.c")
            }),
            counted(
                "initialisation: an edge across composition ordering two steps unlike their texts",
                &cases,
                |c| lowered(c) && dma(c, "uses:uart.c"),
            ),
            counted("initialisation: an edge across a block's `needs` naming a block stands", &cases, |c| {
                lowered(c) && dma(c, "needs:uart.c")
            }),
            some("operand: the namers' order unlike the operands'"),
        ],
    );
}

#[test]
fn a_block_a_service_names_outside_the_hardware_the_search_s_conflict() {
    let cases = served_blocks();
    // How `blk` is named, of 10; what it requires, of 5; its offer bound to or not; `u` of 2 kinds.
    assert_eq!(cases.len(), 10 * 5 * 2 * 2);
    let tally = run("served blocks", &cases);
    // Unresolved: the 100 where a served `drv` names `blk`, no hardware (5 namings, 5 requirements, 2 of its offer
    // bound to or not, 2 kinds of `u`), model §8 rule 7's conflict, `infeasible-configuration`, "whatever the
    // assignment", so `xfer`'s binding changes nothing; and the 10 where `xfer` needs `blk`'s offer, `drv` served by
    // nothing (1 naming, 5 requirements, 2 kinds of `u`): a requester bound outside the hardware is an assignment's
    // conflict, and nothing else offers `transfer-engine`, so the search concludes `missing-fact`. Lowered: the 80
    // where a task or the system's own level names `blk` (4 namings, 5, 2, 2), and the 10 where `drv` is served by
    // nothing and nothing binds `blk` (1, 5, 2). 100 + 10 + 80 + 10 = 200. Each count read from the label: `blk`
    // hardware where a task or the system's own level names it; its namers; what it requires; whether `xfer` binds it.
    let by = |c: &Case| {
        c.label
            .split(" named ")
            .nth(1)
            .and_then(|r| r.split(';').next())
            .expect("a naming")
            .to_string()
    };
    let requires = |c: &Case| {
        c.label["blk:".len()..]
            .split(' ')
            .next()
            .expect("requires")
            .to_string()
    };
    let bound = |c: &Case| c.label.contains("bound to by xfer");
    let hardware =
        |c: &Case| ["TDirect", "TDrvAndT", "UDrvAndU", "OwnDrvAndOwn"].contains(&by(c).as_str());
    let named_also = |c: &Case| ["TDrvAndT", "UDrvAndU", "OwnDrvAndOwn"].contains(&by(c).as_str());
    let conflict = |c: &Case| !hardware(c) && by(c) != "NoDrv";
    let lowered = |c: &Case| hardware(c) || (by(c) == "NoDrv" && !bound(c));
    let bare = |c: &Case| requires(c) == "Nothing" && !bound(c);
    hold_unresolved("served blocks", &tally, &cases, (100, 10), |c| {
        by(c) == "NoDrv" && bound(c)
    });
    hold(
        "served blocks",
        &tally,
        (200, 90, 0, 110, 0),
        &[
            counted(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &cases, conflict),
            counted("outside: named by a service", &cases, conflict),
            counted("outside: a block no platform composes", &cases, conflict),
            counted("outside: named by `uses`", &cases, |c| conflict(c) && by(c) != "TDrvNeeds"),
            counted("outside: named by `needs`", &cases, |c| conflict(c) && by(c) == "TDrvNeeds"),
            counted("outside: its namer served by a task", &cases, |c| conflict(c) && by(c) != "OwnDrv"),
            counted("outside: its namer served by the system's own level alone", &cases, |c| {
                conflict(c) && by(c) == "OwnDrv"
            }),
            counted("outside: requiring something of its own", &cases, |c| {
                conflict(c) && requires(c) != "Nothing"
            }),
            counted(
                "outside: a block outside the hardware named by a service or a policy the system does not serve, no \
                 refusal",
                &cases,
                |c| lowered(c) && by(c) == "NoDrv",
            ),
            counted("named: hardware a service or a policy the system does not serve names, no node", &cases, |c| {
                lowered(c) && by(c) == "NoDrv"
            }),
            counted("initialisation: a named edge from a block a service names by `uses`", &cases, named_also),
            counted(
                "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
                &cases,
                |c| named_also(c) && bare(c),
            ),
            counted("initialisation: a named edge from hardware a provider alone", &cases, |c| {
                named_also(c) && requires(c) == "Nothing" && bound(c)
            }),
            counted(
                "initialisation: a named edge from hardware bound for its own requirements alone",
                &cases,
                |c| named_also(c) && requires(c) != "Nothing" && !bound(c),
            ),
            counted(
                "initialisation: a named edge from hardware a provider and bound for its own requirements",
                &cases,
                |c| named_also(c) && requires(c) != "Nothing" && bound(c),
            ),
            counted("named: a block a service names, bound for nothing, a node", &cases, |c| {
                named_also(c) && bare(c)
            }),
            counted("named: a block a task names, bound for nothing, a node", &cases, |c| {
                ["TDirect", "TDrvAndT", "UDrvAndU"].contains(&by(c).as_str()) && bare(c)
            }),
            counted("named: hardware named by `uses`, bound for nothing, a node", &cases, |c| {
                hardware(c) && bare(c)
            }),
            counted("named: hardware named and bound for nothing, in a job", &cases, |c| {
                hardware(c) && bare(c) && by(c) != "OwnDrvAndOwn"
            }),
            counted("named: hardware named and bound for nothing, in no job", &cases, |c| {
                by(c) == "OwnDrvAndOwn" && bare(c)
            }),
            counted(
                "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                &cases,
                |c| hardware(c) && requires(c) == "Nothing" && bound(c),
            ),
            counted(
                "hardware: a block a served service or policy names, which a task's name makes hardware",
                &cases,
                |c| ["TDrvAndT", "UDrvAndU"].contains(&by(c).as_str()),
            ),
            counted(
                "hardware: a block a served service or policy names, which the system's own level's name makes \
                 hardware",
                &cases,
                |c| by(c) == "OwnDrvAndOwn",
            ),
            counted("hardware: a block bound for requirements of its own", &cases, |c| {
                hardware(c) && requires(c) != "Nothing"
            }),
            counted("hardware: an alarm it programs itself", &cases, |c| hardware(c) && requires(c) == "Alarm"),
            counted("hardware: an expiry running its code", &cases, |c| hardware(c) && requires(c) == "Deadline"),
            counted("hardware: a source's handler", &cases, |c| hardware(c) && requires(c) == "Source"),
            counted("trigger: a source a block or a platform handles", &cases, |c| {
                by(c) == "UDrvAndU" && requires(c) == "Source" && c.label.ends_with("u sporadic")
            }),
            counted(
                "second system: requiring hardware the lowered system does not serve",
                &cases,
                |c| lowered(c) && by(c) == "NoDrv" && requires(c) != "Nothing",
            ),
            exactly("task: deadline-from written", 90),
        ],
    );
}

#[test]
fn several_systems_named_in_the_namers_order() {
    let cases = naming_order();
    // What each of 5 namers names, of 4; by `uses` or `needs`; `y.b`'s own level naming `y` or not.
    assert_eq!(cases.len(), 4usize.pow(5) * 2 * 2);
    let tally = run("naming order", &cases);
    // Lowered: the 4 where no namer names a system (by either clause, `y.b` naming `y` or not, `y.b` never reached).
    // Every other refused, its pairs read from what each namer writes: `t`, `t!`, `m.s`, `m` and the system's own level,
    // and `y.b`'s own level once a namer names `y.b`.
    let systems = |side: &Side| -> Vec<String> {
        side.uses
            .iter()
            .chain(&side.needs)
            .filter(|n| ["y", "y.b", "y!"].contains(&n.as_str()))
            .cloned()
            .collect()
    };
    let namers = |c: &Case| -> Vec<(&'static str, Vec<String>)> {
        let d = &c.description;
        let mut out = vec![
            ("(task sys t)", systems(&d.tasks[0].side)),
            ("(task sys t!)", systems(&d.tasks[1].side)),
            ("m.s", systems(&d.services[0].side)),
            ("m", systems(&d.services[1].side)),
            ("sys", systems(&d.own)),
        ];
        if out
            .iter()
            .any(|(_, named)| named.contains(&"y.b".to_string()))
        {
            out.push(("y.b", systems(&d.others[0].own)));
        }
        out.retain(|(_, named)| !named.is_empty());
        out
    };
    let named = |c: &Case| -> Vec<String> { namers(c).into_iter().flat_map(|(_, n)| n).collect() };
    let by = |c: &Case, namer: &str| namers(c).iter().any(|(n, _)| *n == namer);
    hold(
        "naming order",
        &tally,
        (4_096, 4, 0, 0, 4_092),
        &[
            exactly("refusal: a requester naming a system", 4_092),
            counted("naming: a task naming a system", &cases, |c| {
                by(c, "(task sys t)") || by(c, "(task sys t!)")
            }),
            counted(
                "naming: the system's own level naming a system",
                &cases,
                |c| by(c, "sys"),
            ),
            counted("naming: a service naming a system", &cases, |c| {
                by(c, "m.s") || by(c, "m")
            }),
            counted(
                "naming: a named system's own level naming a system",
                &cases,
                |c| by(c, "y.b"),
            ),
            counted("naming: several names", &cases, |c| named(c).len() >= 2),
            counted("naming: one namer naming two systems", &cases, |c| {
                namers(c).iter().any(|(_, n)| n.len() >= 2)
            }),
            counted(
                "naming: one namer's text a prefix of another's",
                &cases,
                |c| by(c, "m") && by(c, "m.s"),
            ),
            counted(
                "naming: one system's text a prefix of another's",
                &cases,
                |c| {
                    let n = named(c);
                    n.contains(&"y".to_string())
                        && (n.contains(&"y.b".to_string()) || n.contains(&"y!".to_string()))
                },
            ),
            counted(
                "naming: a name continuing with a byte below `)`",
                &cases,
                |c| by(c, "(task sys t!)") || named(c).contains(&"y!".to_string()),
            ),
            counted(
                "naming: tasks' texts ordered unlike their names",
                &cases,
                |c| by(c, "(task sys t)") && by(c, "(task sys t!)"),
            ),
            counted(
                "naming: a system another system's requester names, outside this one's",
                &cases,
                |c| {
                    let yb = &c.description.others[0].own;
                    named(c).is_empty() && !(yb.uses.is_empty() && yb.needs.is_empty())
                },
            ),
            some("naming: an order the message's words would give otherwise"),
            exactly("task: deadline-from written", 4),
        ],
    );
}

#[test]
fn several_blocks_and_platforms_naming_prefixed_names_in_their_texts_order() {
    let cases = prefixed_hardware_naming();
    // What each of `d`, `d.x`, `d!`, `soc` and `q` names, of 4.
    assert_eq!(cases.len(), 4usize.pow(5));
    let tally = run("prefixed hardware naming", &cases);
    // Lowered: the 4 where no block or platform `sys` serves names anything but hardware, whatever `q`, which it serves
    // only once `y` is reached, names (round 15, R15-R2); every other refused, by the naming-system refusal where a
    // block `sys` serves names `y`, else by hardware naming what is not hardware. Each count read from what each block
    // and `soc` names, `q` served where a block names `y`, as §1 orders the pairs.
    let named = |side: &Side, platform: bool| -> Vec<(String, bool)> {
        side.uses
            .iter()
            .filter(|n| !platform || !["d", "d.x", "d!"].contains(&n.as_str()))
            .map(|n| (n.clone(), false))
            .chain(side.needs.iter().map(|n| (n.clone(), true)))
            .collect()
    };
    let namings = |c: &Case| -> Vec<(&'static str, Vec<(String, bool)>)> {
        let d = &c.description;
        vec![
            ("d", named(&d.blocks[0].side, false)),
            ("d.x", named(&d.blocks[1].side, false)),
            ("d!", named(&d.blocks[2].side, false)),
            ("q", named(&d.blocks[3].side, false)),
            ("soc", named(&d.platforms[0].side, true)),
        ]
    };
    let is_system = |n: &str| ["y", "sys"].contains(&n);
    let y_reached = |c: &Case| {
        namings(c)
            .iter()
            .any(|(who, n)| *who != "q" && n.iter().any(|(m, _)| m == "y"))
    };
    let served = |c: &Case, who: &str| who != "q" || y_reached(c);
    let pairs = |c: &Case, systems: bool| -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = namings(c)
            .into_iter()
            .filter(|(who, _)| served(c, who))
            .flat_map(|(who, n)| {
                n.into_iter()
                    .filter(|(m, _)| is_system(m) == systems)
                    .map(move |(m, _)| (who.to_string(), m))
            })
            .collect();
        out.sort();
        out
    };
    let naming = |c: &Case| !pairs(c, true).is_empty();
    let operand = |c: &Case| !naming(c) && !pairs(c, false).is_empty();
    let lowered = |c: &Case| !naming(c) && !operand(c);
    let has = |c: &Case, who: &str, what: &str, by_needs: bool| {
        namings(c)
            .iter()
            .any(|(w, n)| *w == who && n.iter().any(|(m, b)| m == what && *b == by_needs))
    };
    let blocks_name = |c: &Case, what: &str, by_needs: bool| {
        ["d", "d.x", "d!"].iter().any(|w| has(c, w, what, by_needs))
    };
    let q_unserved = |c: &Case| !y_reached(c);
    let operands_of =
        |c: &Case| -> Vec<String> { pairs(c, false).into_iter().map(|(_, o)| o).collect() };
    hold(
        "prefixed hardware naming",
        &tally,
        (1_024, 4, 0, 0, 1_020),
        &[
            counted("refusal: a requester naming a system", &cases, naming),
            counted("refusal: hardware naming what is not hardware", &cases, operand),
            counted("naming: a block or a platform naming a system", &cases, naming),
            counted("naming: a block's `needs` naming a system", &cases, |c| {
                naming(c) && pairs(c, true).iter().any(|(w, m)| has(c, w, m, true))
            }),
            counted("naming: a block's `uses` naming a system", &cases, |c| naming(c) && has(c, "q", "sys", false)),
            counted("naming: hardware served through a reached system's platform naming a system", &cases, |c| {
                naming(c) && pairs(c, true).iter().any(|(w, _)| w == "q")
            }),
            counted("naming: the lowered system itself named", &cases, |c| {
                naming(c) && pairs(c, true).iter().any(|(_, m)| m == "sys")
            }),
            counted("naming: several names", &cases, |c| pairs(c, true).len() >= 2),
            counted("naming: one namer's text a prefix of another's", &cases, |c| {
                let namers: BTreeSet<String> = pairs(c, true).into_iter().map(|(w, _)| w).collect();
                namers.contains("d") && (namers.contains("d.x") || namers.contains("d!"))
            }),
            counted("naming: a name continuing with a byte below `)`", &cases, |c| {
                pairs(c, true).iter().any(|(w, _)| w == "d!")
            }),
            exactly("naming: before hardware naming what is not hardware", 592),
            counted("operand: a block's `uses` naming a service", &cases, |c| {
                operand(c) && blocks_name(c, "m.s", false)
            }),
            counted("operand: a block's `needs` naming a policy", &cases, |c| operand(c) && blocks_name(c, "m!", true)),
            counted("operand: a platform's `needs` naming a service", &cases, |c| {
                operand(c) && (has(c, "soc", "m.s", true) || has(c, "soc", "m", true))
            }),
            counted("operand: a platform's `uses` naming a policy", &cases, |c| {
                operand(c) && has(c, "soc", "m!", false)
            }),
            counted("operand: a block naming the service bound to it", &cases, |c| {
                operand(c) && has(c, "d", "m", false)
            }),
            counted("operand: a service only hardware names", &cases, |c| {
                operand(c) && operands_of(c).contains(&"m.s".to_string())
            }),
            counted(
                "operand: hardware the system does not serve naming what is not hardware too, not named",
                &cases,
                |c| {
                    let q = &c.description.blocks[3].side;
                    operand(c) && q_unserved(c) && !(q.uses.is_empty() && q.needs.is_empty())
                },
            ),
            counted("operand: several, named in order", &cases, |c| operand(c) && operands_of(c).len() >= 2),
            counted("operand: one namer naming two", &cases, |c| {
                let p = pairs(c, false);
                operand(c) && p.windows(2).any(|w| w[0].0 == w[1].0)
            }),
            counted("operand: one namer's text a prefix of another's", &cases, |c| {
                let namers: BTreeSet<String> = pairs(c, false).into_iter().map(|(w, _)| w).collect();
                operand(c) && namers.contains("d") && (namers.contains("d.x") || namers.contains("d!"))
            }),
            counted("operand: one operand's text a prefix of another's", &cases, |c| {
                let o = operands_of(c);
                operand(c) && o.contains(&"m".to_string()) && (o.contains(&"m.s".to_string()) || o.contains(&"m!".to_string()))
            }),
            counted("operand: a name continuing with a byte below `)`", &cases, |c| {
                operand(c) && pairs(c, false).iter().any(|(w, o)| w == "d!" || o == "m!")
            }),
            counted("operand: the namers' order unlike the operands'", &cases, |c| {
                operand(c) && pairs(c, false).windows(2).any(|w| w[0].1 > w[1].1)
            }),
            counted("unserved: a block naming a service", &cases, |c| lowered(c) && has(c, "q", "m.s", false)),
            counted("unserved: a block's `needs` naming a system", &cases, |c| {
                lowered(c) && (has(c, "q", "y", true) || has(c, "q", "sys", true))
            }),
            counted("unserved: a block's `uses` naming a system", &cases, |c| lowered(c) && has(c, "q", "sys", false)),
            counted("unserved: hardware naming the lowered system", &cases, |c| {
                lowered(c) && (has(c, "q", "sys", false) || has(c, "q", "sys", true))
            }),
            counted("second system: requiring hardware the lowered system does not serve", &cases, |c| {
                lowered(c) && !c.description.blocks[3].side.needs.is_empty()
            }),
            some("naming: an order the message's words would give otherwise"),
            some("operand: an order the message's words would give otherwise"),
            exactly("multiplex: a channel with one user", 4),
            exactly("task: deadline-from written", 4),
        ],
    );
}

#[test]
fn one_receiver_s_alarms_and_expiries() {
    let cases = receivers_within_one();
    // 3 shapes of counters; what `t`, `t!` and the system's own level each program, of 4; which task is first.
    assert_eq!(cases.len(), 3 * 4usize.pow(3) * 2);
    let tally = run("one receiver", &cases);
    let programs = |c: &Case, who: &str| !c.label.contains(&format!(" {who}:Nothing"));
    let receivers = |c: &Case| ["t", "t!", "own"].iter().filter(|w| programs(c, w)).count();
    let resolved = |c: &Case| c.label.starts_with("OneOffersBoth ") || !c.label.contains(":Both");
    // Unresolved: the 148 where some requester programs both and no one provider offers both (37 choices of the 64
    // with a `Both`, 2 shapes, 2 orders), each `missing-fact`: one receiver may receive one thing, never two. Lowered:
    // the 6 where nothing is programmed. Every other refused, naming its receivers.
    let both_apart: BTreeSet<String> = cases
        .iter()
        .filter(|c| c.label.contains("Both") && !c.label.starts_with("OneOffersBoth"))
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(both_apart.len(), 148);
    assert_eq!(
        tally.unresolved_systems.keys().cloned().collect::<BTreeSet<_>>(),
        both_apart,
        "the systems the search resolves no plan for are those where one requester programs both apart"
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "MissingFact"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "one receiver",
        &tally,
        (384, 6, 0, 148, 230),
        &[
            // Fixed by what each requester programs: the receivers, one for each requester programming anything, in
            // §1's order — `t!` first in runtime order or `t`, `(task sys t!)` the lesser text — and, where `d` offers
            // both facts, one requester programming both at `d` alone (model §8 rule 13: one provider for its
            // function). Whether a deadline is taken at a block offering it or through an instance, and over which
            // timer, is the search's choice: so are the alarms', the expiries' and the providers' classes.
            counted("receivers: one", &cases, |c| {
                resolved(c) && receivers(c) == 1
            }),
            counted("receivers: two", &cases, |c| {
                resolved(c) && receivers(c) == 2
            }),
            counted("receivers: three", &cases, |c| {
                resolved(c) && receivers(c) == 3
            }),
            counted("receivers: tasks and the system's own level", &cases, |c| {
                resolved(c) && programs(c, "own") && (programs(c, "t") || programs(c, "t!"))
            }),
            counted("receivers: the system's own level alone", &cases, |c| {
                resolved(c) && programs(c, "own") && !programs(c, "t") && !programs(c, "t!")
            }),
            counted(
                "receivers: runtime order unlike the tasks' text order",
                &cases,
                |c| {
                    resolved(c)
                        && programs(c, "t")
                        && programs(c, "t!")
                        && c.label.ends_with("first=t")
                },
            ),
            counted(
                "receivers: one alarm for two consumed facts at one provider",
                &cases,
                |c| c.label.starts_with("OneOffersBoth ") && c.label.contains(":Both"),
            ),
            exactly("task: deadline-from written", 6),
            some("receivers: an alarm programmed itself"),
            some("receivers: an expiry"),
            some("receivers: providers one of whose names prefixes another's"),
        ],
    );
    assert!(
        !tally
            .classes
            .contains_key("receivers: one receiver receiving two"),
        "a receiver receiving two things is a system the search resolves no plan for"
    );
}

#[test]
fn one_instance_owes_one_bound_at_most() {
    let cases = one_instance_s_bounds();
    // A first bound of 2 values, a second of 3 (none among them).
    assert_eq!(cases.len(), 2 * 3);
    let tally = run("one instance's bounds", &cases);
    // One bound within, or two: lowered, the tightest obliged. One past, or two: refused, naming the bound once. One
    // within and one past: the search's answer, `unsupported-profile`.
    let mixed = BTreeSet::from([
        "dl writes 5 ms and past".to_string(),
        "dl writes past and 3 ms".to_string(),
    ]);
    assert_eq!(
        tally
            .unresolved_systems
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        mixed,
        "the search resolves no plan where one fact's bounds lie one within the arithmetic, one past it: {:?}",
        tally.unresolved_systems
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "UnsupportedProfile"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "one instance's bounds",
        &tally,
        (6, 2, 0, 2, 2),
        &[
            exactly("instance: an obliged fact with its bound", 2),
            exactly("instance: a service's obliged bound", 2),
            exactly("context: an expiry running its requester", 2),
            exactly("refusal: a duration past the arithmetic", 2),
            exactly("past: an obliged bound", 2),
            exactly("past: an obliged bound of a service", 2),
            exactly("task: deadline-from written", 2),
        ],
    );
}

#[test]
fn a_provider_bound_to_the_node_taking_its_channel() {
    let cases = own_route_through_its_provider();
    // What the timer requires, of 4; what `dma.x` and `soc` each take, of 3; what `t` uses, of 4, and `u`, of 3.
    assert_eq!(cases.len(), 4 * 3 * 3 * 4 * 3);
    let tally = run("own route", &cases);
    // Every system lowered. Each count read from what the timer, `dma.x` and `soc` require and what the tasks use, as
    // the universe writes them: requiring hardware bound to by a requester — the timer by whichever takes its channel,
    // itself or through an instance; `dma.x` by `xfer` or the timer; `soc` by `cpu` or the timer — or by nothing; a
    // component where the timer is bound to what takes its channel, an instance among it where that one takes a
    // deadline; `soc` bound to the timer it composes for an alarm, the timer bound to `soc` for `core-count`, one
    // component where both are, else two steps the binding orders unlike their texts, `timer.d` above `soc`.
    let timer = |c: &Case, fact: &str| c.description.blocks[0].side.needs.iter().any(|n| n == fact);
    fn takes(c: &Case, who: usize) -> Option<&str> {
        let side = if who == 0 {
            &c.description.blocks[1].side
        } else {
            &c.description.platforms[0].side
        };
        side.needs.first().map(String::as_str)
    }
    fn dma(c: &Case) -> Option<&str> {
        takes(c, 0)
    }
    fn soc(c: &Case) -> Option<&str> {
        takes(c, 1)
    }
    let used = |c: &Case, service: &str| {
        c.description
            .tasks
            .iter()
            .any(|t| t.side.uses.iter().any(|u| u == service))
    };
    type Bound<'a> = dyn Fn(&Case) -> Option<bool> + 'a;
    let requiring_bound: [&Bound<'_>; 3] = [
        &|c| {
            (!c.description.blocks[0].side.needs.is_empty())
                .then(|| dma(c).is_some() || soc(c).is_some())
        },
        &|c| dma(c).map(|_| used(c, "xfer") || timer(c, "transfer-engine")),
        &|c| soc(c).map(|_| used(c, "cpu") || timer(c, "core-count")),
    ];
    let cycle = |c: &Case| {
        (timer(c, "transfer-engine") && dma(c).is_some())
            || (timer(c, "core-count") && soc(c).is_some())
    };
    let deadline = Some("absolute-deadline");
    hold(
        "own route",
        &tally,
        (432, 432, 0, 0, 0),
        &[
            counted(
                "hardware: a block bound for requirements of its own",
                &cases,
                |c| !c.description.blocks[0].side.needs.is_empty() || dma(c).is_some(),
            ),
            counted(
                "hardware: a platform bound for requirements of its own",
                &cases,
                |c| soc(c).is_some(),
            ),
            counted(
                "hardware: requiring, and bound to by a requester",
                &cases,
                |c| requiring_bound.iter().any(|f| f(c) == Some(true)),
            ),
            counted(
                "hardware: requiring, and bound to by nothing",
                &cases,
                |c| requiring_bound.iter().any(|f| f(c) == Some(false)),
            ),
            counted("hardware: an alarm it programs itself", &cases, |c| {
                dma(c) == Some("relative-delay") || soc(c) == Some("relative-delay")
            }),
            counted("hardware: an expiry running its code", &cases, |c| {
                dma(c) == deadline || soc(c) == deadline
            }),
            counted("context: an expiry running its requester", &cases, |c| {
                dma(c) == deadline || soc(c) == deadline
            }),
            counted("instance: an obliged fact bare", &cases, |c| {
                dma(c) == deadline || soc(c) == deadline
            }),
            counted(
                "initialisation: a component of several members",
                &cases,
                cycle,
            ),
            counted(
                "initialisation: a component holding an instance",
                &cases,
                |c| {
                    (timer(c, "transfer-engine") && dma(c) == deadline)
                        || (timer(c, "core-count") && soc(c) == deadline)
                },
            ),
            counted(
                "initialisation: a composite bound to its part, its edge kept",
                &cases,
                |c| soc(c) == Some("relative-delay"),
            ),
            counted(
                "initialisation: a part bound to its composite, its edge kept",
                &cases,
                |c| timer(c, "core-count"),
            ),
            counted(
                "initialisation: a composite and its part bound each to the other, one component",
                &cases,
                |c| soc(c) == Some("relative-delay") && timer(c, "core-count"),
            ),
            counted(
                "initialisation: an edge across composition ordering two steps unlike their texts",
                &cases,
                |c| soc(c) == Some("relative-delay") && !timer(c, "core-count"),
            ),
            counted(
                "initialisation: an edge to a block or a platform bound for its own requirements",
                &cases,
                |c| {
                    !c.description.blocks[0].side.needs.is_empty()
                        || dma(c).is_some()
                        || soc(c).is_some()
                },
            ),
            counted(
                "owed: a service or a policy in a job, its call cost and masked runs",
                &cases,
                |c| !c.description.services.is_empty(),
            ),
            exactly("task: deadline-from written", 432),
            some("multiplex: its own route a user, the route's provider bound to it"),
            some("multiplex: its own route a user, another node there reaching it"),
            some("multiplex: its own route no user"),
            some("multiplex: a block or a platform multiplexed"),
            some("multiplex: multiplexed by a job and an interrupt"),
            some("multiplex: a block or a platform's channel with one user"),
            some("composition: a platform a task reaches through a binding"),
        ],
    );
}

#[test]
fn bound_to_a_timer_route_s_provider_by_a_binding_taking_no_channel() {
    let cases = bound_otherwise_on_a_route();
    // What `t` and the system's own level each require, of 7; `u`, of 3; `s`, of 4; which task is first.
    assert_eq!(cases.len(), 7 * 3 * 7 * 4 * 2);
    let tally = run("bound otherwise on a route", &cases);
    // Rejected: the 168 where `t` writes `(relative-delay false)` (3 choices of `u`, 7 of the system's own level, 4 of
    // `s`, 2 orders), `invalid-description`: a task writes no constraint clause.
    let writes_false: BTreeSet<String> = cases
        .iter()
        .filter(|c| c.label.starts_with("t:False "))
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(writes_false.len(), 168);
    assert_eq!(
        tally
            .verdicts
            .get("rejected invalid-description")
            .map(|v| v.0),
        Some(168),
        "the language refuses a task's constraint clause: {:?}",
        tally.verdicts
    );
    // Each count read from what each requester requires, as the label writes it: a timer route at `d` wherever a task,
    // the system's own level or `s` takes a channel there, itself or through an instance over `d`, naming those bound
    // for an alarm and the requesters of each instance; a task or the system's own level bound to `d` for its output
    // alone, named by the route or not, beside one programming an alarm there or not ([`keyed_classes`]); its receivers,
    // an alarm or an expiry beside `d`'s output each (§1).
    let of = |c: &Case, who: &str| -> String {
        let word = c
            .label
            .split(' ')
            .find_map(|w| w.strip_prefix(&format!("{who}:")))
            .expect("a requirement");
        word.trim_start_matches("Some(")
            .trim_end_matches(')')
            .to_string()
    };
    let channel =
        |v: &str| ["Wake", "WakeShared", "AlarmShared", "ExpiryShared", "Alarm"].contains(&v);
    let alarm = |v: &str| ["AlarmShared", "Alarm"].contains(&v);
    let receives = |v: &str| ["AlarmShared", "ExpiryShared"].contains(&v);
    let output = |v: &str| ["Shared", "WakeShared", "AlarmShared", "ExpiryShared"].contains(&v);
    let rejected = |c: &Case| of(c, "t") == "False";
    // Unresolved: the 64 whose own level writes `(relative-delay false)` beside a task needing `relative-delay` (8 of
    // the 18 choices of `t` and `u` the language accepts: `t` programming an alarm with each of 3 `u`, `u` with each of
    // 6 `t`, the one where both do counted once; 4 of `s`, 2 orders), each `invalid-description`, no value satisfying
    // the side read whole ([`SEARCH_READS_THE_SIDE_WHOLE`]; model §7 rule 4, §8 rule 1; `RTP-H20`). So the
    // configuration checker judges none of them, and accepts every record it does judge.
    let contradicted = |c: &Case| {
        !rejected(c)
            && of(c, "own") == "False"
            && (of(c, "t") == "AlarmShared" || of(c, "u") == "AlarmShared")
    };
    let refused_by_search: BTreeMap<String, String> = cases
        .iter()
        .filter(|c| contradicted(c))
        .map(|c| (c.label.clone(), "InvalidDescription".to_string()))
        .collect();
    assert_eq!(refused_by_search.len(), 64);
    assert_eq!(
        tally.unresolved_systems, refused_by_search,
        "the search resolves no plan where the system's side, read whole, contradicts itself"
    );
    let resolved = |c: &Case| !rejected(c) && !contradicted(c);
    let receivers = |c: &Case| {
        ["t", "u", "own"]
            .iter()
            .filter(|w| receives(&of(c, w)))
            .count()
    };
    // Lowered: the 320 with no receiver, 4 of `t`, 2 of `u`, 5 of the system's own level, 4 of `s`, 2 orders; a task
    // programming an alarm is a receiver, so none of them is among the 64. Refused: the 1_008 the language accepts,
    // less those 320 and the 64: 624, each naming its receivers (§1).
    let lowered = |c: &Case| resolved(c) && receivers(c) == 0;
    let refused = |c: &Case| resolved(c) && receivers(c) > 0;
    let route = |c: &Case| ["t", "u", "own", "s"].iter().any(|w| channel(&of(c, w)));
    let keyed = |c: &Case, task: bool, named: bool| {
        resolved(c)
            && route(c)
            && ["t", "u", "own"].iter().any(|w| {
                let v = of(c, w);
                (*w != "own") == task && output(&v) && !alarm(&v) && channel(&v) == named
            })
    };
    let beside = |c: &Case| {
        resolved(c)
            && route(c)
            && ["t", "u", "own"].iter().any(|w| {
                let v = of(c, w);
                output(&v)
                    && !alarm(&v)
                    && ["t", "u", "own"].iter().any(|x| x != w && alarm(&of(c, x)))
            })
    };
    let any = |c: &Case, values: &[&str]| {
        ["t", "u", "own", "s"]
            .iter()
            .any(|w| values.contains(&of(c, w).as_str()))
    };
    hold(
        "bound otherwise on a route",
        &tally,
        (1_176, 320, 168, 64, 624),
        &[
            exactly(SEARCH_READS_THE_SIDE_WHOLE, 64),
            counted(SEARCH_READS_THE_SIDE_WHOLE, &cases, contradicted),
            counted(
                "bound otherwise: a task a timer route names, bound to its provider for a fact taking no channel",
                &cases,
                |c| keyed(c, true, true),
            ),
            counted(
                "bound otherwise: the system's own level a timer route names, bound to its provider for a fact taking \
                 no channel",
                &cases,
                |c| keyed(c, false, true),
            ),
            counted(
                "bound otherwise: a task bound to a timer route's provider for a fact taking no channel, the route not \
                 naming it",
                &cases,
                |c| keyed(c, true, false),
            ),
            counted(
                "bound otherwise: the system's own level bound to a timer route's provider for a fact taking no \
                 channel, the route not naming it",
                &cases,
                |c| keyed(c, false, false),
            ),
            counted(
                "bound otherwise: beside a task or the system's own level programming an alarm at that provider",
                &cases,
                beside,
            ),
            counted(
                "bound otherwise: a requester bound for a consumed fact at `false`, no channel",
                &cases,
                |c| lowered(c) && (of(c, "own") == "False" || of(c, "s") == "False"),
            ),
            counted(
                "receivers: an alarm, its receiver bound to its provider for a fact taking no channel too",
                &cases,
                |c| refused(c) && ["t", "u", "own"].iter().any(|w| of(c, w) == "AlarmShared"),
            ),
            counted(
                "receivers: an expiry, its receiver bound to the instance's provider for a fact taking no channel too",
                &cases,
                |c| refused(c) && ["t", "own"].iter().any(|w| of(c, w) == "ExpiryShared"),
            ),
            counted("receivers: one", &cases, |c| resolved(c) && receivers(c) == 1),
            counted("receivers: two", &cases, |c| resolved(c) && receivers(c) == 2),
            counted("receivers: three", &cases, |c| resolved(c) && receivers(c) == 3),
            counted("receivers: tasks and the system's own level", &cases, |c| {
                refused(c) && receives(&of(c, "own")) && (receives(&of(c, "t")) || receives(&of(c, "u")))
            }),
            counted("receivers: the system's own level alone", &cases, |c| {
                refused(c) && receives(&of(c, "own")) && !receives(&of(c, "t")) && !receives(&of(c, "u"))
            }),
            counted("receivers: runtime order unlike the tasks' text order", &cases, |c| {
                refused(c) && receives(&of(c, "t")) && receives(&of(c, "u")) && c.label.ends_with("first=u")
            }),
            counted("context: a wake running the instance alone", &cases, |c| {
                lowered(c) && any(c, &["Wake", "WakeShared"])
            }),
            counted("multiplex: multiplexed by two jobs", &cases, |c| lowered(c) && of(c, "s") == "Alarm"),
            exactly("task: deadline-from written", 320),
        ],
    );
}

#[test]
fn a_periodic_task_bound_to_a_source_s_block_for_another_fact() {
    let cases = periodic_bound_otherwise();
    // What `t` requires, of 5; `p`, of 4; the system's own level, of 4; `e`, of 3; which periodic task is first.
    assert_eq!(cases.len(), 5 * 4 * 4 * 3 * 2);
    let tally = run("periodic bound otherwise", &cases);
    // Rejected: the 96 where `t` writes `(interrupt-source false)` (4 choices of `p`, 4 of the system's own level, 3 of
    // `e`, 2 orders), `invalid-description`: a task writes no constraint clause.
    assert_eq!(
        tally
            .verdicts
            .get("rejected invalid-description")
            .map(|v| v.0),
        Some(96),
        "the language refuses a task's constraint clause: {:?}",
        tally.verdicts
    );
    // Each count read from what each requester requires, as the label writes it: §1's bullet over tasks named on the
    // first periodic task in runtime order bound for `interrupt-source` itself, `p` first or `t`, beside the other bound
    // to `dev` for `debug-port` or for both; and, lowered, `u` released by `dev`'s 20 ms where `e` handles it.
    let of = |c: &Case, who: &str| -> String {
        let word = c
            .label
            .split(' ')
            .find_map(|w| w.strip_prefix(&format!("{who}:")))
            .expect("a requirement");
        word.trim_start_matches("Some(")
            .trim_end_matches(')')
            .to_string()
    };
    let source = |v: &str| ["Source", "Both"].contains(&v);
    let other = |v: &str| ["Other", "Both"].contains(&v);
    let rejected = |c: &Case| of(c, "t") == "False";
    let order = |c: &Case| {
        if c.label.ends_with("first=p") {
            ["p", "t"]
        } else {
            ["t", "p"]
        }
    };
    let named = |c: &Case| order(c).into_iter().find(|w| source(&of(c, w)));
    // Unresolved: the 72 whose own level writes `(interrupt-source false)` beside `t` or `p` needing `interrupt-source`
    // (12 of the 16 choices of `t` and `p` the language accepts, all but the 4 where neither is `Source` or `Both`; 3
    // of `e`, 2 orders), each `invalid-description`, no value satisfying the side read whole
    // ([`SEARCH_READS_THE_SIDE_WHOLE`]; model §7 rule 4, §8 rule 1; `RTP-H20`). So the configuration checker judges
    // none of them, and accepts every record it does judge.
    let contradicted = |c: &Case| !rejected(c) && of(c, "own") == "False" && named(c).is_some();
    let refused_by_search: BTreeMap<String, String> = cases
        .iter()
        .filter(|c| contradicted(c))
        .map(|c| (c.label.clone(), "InvalidDescription".to_string()))
        .collect();
    assert_eq!(refused_by_search.len(), 72);
    assert_eq!(
        tally.unresolved_systems, refused_by_search,
        "the search resolves no plan where the system's side, read whole, contradicts itself"
    );
    let resolved = |c: &Case| !rejected(c) && !contradicted(c);
    // Lowered: the 96 where neither `t` nor `p` requires the interrupt, 4 choices of them, 4 of the system's own level,
    // 3 of `e`, 2 orders. Refused: the 12 choices of them where one does, 3 of the system's own level, all but `false`,
    // 3 of `e`, 2 orders: 216.
    let refused = |c: &Case| resolved(c) && named(c).is_some();
    let lowered = |c: &Case| resolved(c) && named(c).is_none();
    hold(
        "periodic bound otherwise",
        &tally,
        (480, 96, 96, 72, 216),
        &[
            counted(SEARCH_READS_THE_SIDE_WHOLE, &cases, contradicted),
            exactly(SEARCH_READS_THE_SIDE_WHOLE, 72),
            counted("refusal: a periodic task bound to a source itself", &cases, refused),
            counted("periodic: a periodic task bound to a source's block for another fact", &cases, |c| {
                resolved(c) && (other(&of(c, "t")) || other(&of(c, "p")))
            }),
            counted(
                "periodic: one bound to the source's block otherwise first in runtime order, the one bound for \
                 `interrupt-source` named",
                &cases,
                |c| refused(c) && named(c) == Some(order(c)[1]) && other(&of(c, order(c)[0])),
            ),
            counted("periodic: the one named bound to the source's block for another fact too", &cases, |c| {
                named(c).is_some_and(|w| of(c, w) == "Both") && resolved(c)
            }),
            counted(
                "task bullet: the first ill-formed task in runtime order named, a later one ill-formed too",
                &cases,
                |c| resolved(c) && source(&of(c, "t")) && source(&of(c, "p")),
            ),
            counted(
                "task bullet: a task declared earlier ill-formed too, after the one named in runtime order",
                &cases,
                |c| resolved(c) && source(&of(c, "t")) && source(&of(c, "p")) && order(c)[0] == "p",
            ),
            counted("bound otherwise: a requester bound for `interrupt-source` at `false`, no route", &cases, |c| {
                lowered(c) && (of(c, "own") == "False" || of(c, "e") == "False")
            }),
            counted("trigger: every, equal separations", &cases, |c| lowered(c) && of(c, "e") == "Source"),
            counted(
                "trigger: `(unknown unreached)`, no source reaching it",
                &cases,
                |c| lowered(c) && of(c, "e") != "Source",
            ),
            exactly("task: deadline-from written", 96),
        ],
    );
}

#[test]
fn hardware_the_lowered_system_does_not_serve_naming_what_is_not_hardware() {
    let cases = unserved_hardware_naming();
    // What each of `blk.q`, `host.q` and `lone` names, of 5; `soc`, of 5; each description lowered for each system.
    assert_eq!(cases.len(), 5usize.pow(3) * 5 * 2);
    for pair in cases.chunks(2) {
        assert_eq!(
            pair[0].description.text(),
            pair[1].description.text(),
            "one description, lowered for each of its systems"
        );
    }
    let tally = run("unserved hardware naming", &cases);
    // Unresolved `infeasible-configuration`, model §8 rule 7's conflict, the 170 serving `via.s`, which names `lone`,
    // and `lone` is of neither system's hardware: only `soc` names `via.s`, and the system serves `soc` — `sys`, the
    // platform it uses, whatever `blk.q`, `host.q` and `lone` name (125 choices of the three); or `other`, where
    // `blk.q` or `host.q`, which it serves, names `sys` by `uses` (9 of the 25 choices of the two, 5 of `lone`, 45), so
    // that `sys`'s `platform` clause is served, "what a system it names writes at its own level ... being served too"
    // (§1). Unresolved `missing-fact`, the 215 lowering `other` that serve `alarm` and not `via.s`: `blk.q` or `host.q`
    // names it (9 of the 25 choices of the two, 5 of `lone`, 5 of `soc`: 225), less the 10 of those the conflict takes,
    // one naming `alarm`, the other `sys`, and `soc` `via.s` (2, 5 of `lone`); `alarm`'s `relative-delay` has no
    // provider among `other`'s hardware, so the search concludes `missing-fact` (model §8 rule 7). `lone` names `alarm`
    // to no effect of its own: only `via.s` serves `lone`, and then the conflict takes the system.
    hold_unresolved(
        "unserved hardware naming",
        &tally,
        &cases,
        (170, 215),
        |c| {
            let names =
                |namer: &str, naming: &str| c.label.contains(&format!("{namer} names [{naming}]"));
            c.label.ends_with("lowering other")
                && (names("blk.q", "uses:alarm") || names("host.q", "uses:alarm"))
        },
    );
    // Lowered: lowering `sys`, the 125 where `soc` names nothing, whatever `blk.q`, `host.q` and `lone`, which `sys`
    // does not serve, name; lowering `other`, the 25 where `blk.q` and `host.q` name nothing, whatever `soc` and `lone`
    // name: 150. Refused, the 715 left of the 1,250: lowering `sys`, the 375 where `soc` names `pol.p` or `other` (3 of
    // its choices, 125 of the others); lowering `other`, the 340 left of its 625, past its 25 lowered and its 260
    // unresolved. Each count read from what the four namers write ([`unserved_model`]).
    let model: Vec<UnservedModel> = cases.iter().map(unserved_model).collect();
    let count = |class: &'static str, f: &dyn Fn(&UnservedModel) -> bool| {
        exactly(class, model.iter().filter(|m| f(m)).count())
    };
    let pair = |m: &UnservedModel, platform: bool, by_needs: bool| {
        m.naming.iter().any(|(namer, _, needs)| {
            ["soc", "host.q"].contains(&namer.as_str()) == platform && *needs == by_needs
        })
    };
    let operand = |m: &UnservedModel, namer: &str, item: &str| {
        m.outcome == "operand" && m.operands.iter().any(|(n, o)| n == namer && o == item)
    };
    let unserved = |m: &UnservedModel, platform: bool, items: &[&str], by_needs: Option<bool>| {
        m.outcome == "lowered"
            && m.unserved.iter().any(|(namer, item, needs)| {
                ["soc", "host.q"].contains(&namer.as_str()) == platform
                    && items.contains(&item.as_str())
                    && by_needs.is_none_or(|b| b == *needs)
            })
    };
    hold(
        "unserved hardware naming",
        &tally,
        (1_250, 150, 0, 385, 715),
        &[
            count(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &|m| m.outcome == "outside"),
            count("outside: named by a service", &|m| m.outcome == "outside"),
            count("outside: named by `uses`", &|m| m.outcome == "outside"),
            count("outside: a block no platform composes", &|m| m.outcome == "outside"),
            count("outside: its namer served by the system's own level alone", &|m| m.outcome == "outside"),
            count("outside: its namer served only through what hardware the system serves names", &|m| {
                m.outcome == "outside" && m.lowering == "sys"
            }),
            count("outside: its namer served only through a system it names", &|m| {
                m.outcome == "outside" && m.lowering == "other"
            }),
            count("unserved: a block naming a service", &|m| unserved(m, false, &["alarm", "via.s"], None)),
            count("unserved: a block naming a policy", &|m| unserved(m, false, &["pol.p"], None)),
            count("unserved: a block's `uses` naming a system", &|m| {
                unserved(m, false, &["sys", "other"], Some(false))
            }),
            count("unserved: a block's `needs` naming a system", &|m| {
                unserved(m, false, &["sys", "other"], Some(true))
            }),
            count("unserved: a platform naming a service", &|m| unserved(m, true, &["alarm", "via.s"], None)),
            count("unserved: a platform naming a policy", &|m| unserved(m, true, &["pol.p"], None)),
            count("unserved: a platform's `uses` naming a system", &|m| {
                unserved(m, true, &["sys", "other"], Some(false))
            }),
            count("unserved: a platform's `needs` naming a system", &|m| {
                unserved(m, true, &["sys", "other"], Some(true))
            }),
            count("unserved: hardware naming the lowered system", &|m| {
                m.outcome == "lowered" && m.unserved.iter().any(|(_, item, _)| *item == m.lowering)
            }),
            count("unserved: hardware naming a service the lowered system serves", &|m| {
                m.outcome == "lowered" && m.lowering == "sys" && m.unserved.iter().any(|(_, item, _)| item == "alarm")
            }),
            count("unserved: hardware no system's platform composes", &|m| {
                m.outcome == "lowered" && m.unserved.iter().any(|(namer, _, _)| namer == "lone")
            }),
            count("second system: requiring hardware the lowered system does not serve", &|m| {
                m.outcome == "lowered" && m.unserved.iter().any(|(_, _, needs)| *needs)
            }),
            count("named: hardware a service or a policy the system does not serve names, no node", &|m| {
                m.outcome == "lowered"
            }),
            count(
                "outside: a block outside the hardware named by a service or a policy the system does not serve, no \
                 refusal",
                &|m| m.outcome == "lowered",
            ),
            count("multiplex: a channel with one user", &|m| m.outcome == "lowered" && m.lowering == "sys"),
            count("refusal: hardware naming what is not hardware", &|m| m.outcome == "operand"),
            count("operand: a platform's `uses` naming a policy", &|m| operand(m, "soc", "pol.p")),
            count("operand: a platform's `needs` naming a policy", &|m| operand(m, "host.q", "pol.p")),
            count("operand: a block's `needs` naming a policy", &|m| operand(m, "blk.q", "pol.p")),
            count("operand: several, named in order", &|m| m.outcome == "operand" && m.operands.len() >= 2),
            count(
                "operand: hardware the system does not serve naming what is not hardware too, not named",
                &|m| m.outcome == "operand" && !m.unserved.is_empty(),
            ),
            count("refusal: a requester naming a system", &|m| m.outcome == "naming"),
            count("naming: a block or a platform naming a system", &|m| m.outcome == "naming"),
            count("naming: before hardware naming what is not hardware", &|m| m.outcome == "naming"),
            count("naming: a platform's `uses` naming a system", &|m| m.outcome == "naming" && pair(m, true, false)),
            count("naming: a platform's `needs` naming a system", &|m| m.outcome == "naming" && pair(m, true, true)),
            count("naming: a block's `uses` naming a system", &|m| m.outcome == "naming" && pair(m, false, false)),
            count("naming: a block's `needs` naming a system", &|m| m.outcome == "naming" && pair(m, false, true)),
            count("naming: the lowered system itself named", &|m| {
                m.outcome == "naming" && m.naming.iter().any(|(_, system, _)| *system == m.lowering)
            }),
            count("naming: several names", &|m| m.outcome == "naming" && m.naming.len() >= 2),
            count("naming: hardware served through a reached system's platform naming a system", &|m| {
                m.outcome == "naming" && m.naming.iter().any(|(namer, _, _)| !m.served_directly.contains(namer))
            }),
        ],
    );
}

/// What the unserved-hardware universe's construction gives one system: the namers `soc`, `host.q`, `blk.q` and `lone`
/// each name what [`UNSERVED_NAMING`] or [`SOC_NAMES`] chose; the lowered system serves its own platform and what it
/// composes, and, by what a served namer names, another system's — "what a system it names writes at its own level —
/// its `platform` clause included — being served too" (§1) — and the service `via.s`, which names `lone`. So a system
/// `soc` leaves serving `via.s` is model §8 rule 7's conflict, `lone` being of neither system's hardware, whatever else
/// is named; the lowering of `other` serving `alarm`, whose channel no block of its hardware offers, is otherwise
/// unresolved; otherwise the served namers' pairs naming a system are the naming-system refusal's, else their pairs
/// naming a service or a policy the hardware-naming refusal's, else the system lowers, every unserved namer's names
/// counted apart.
struct UnservedModel {
    lowering: String,
    outcome: &'static str,
    /// The naming-system refusal's pairs, with whether by `needs`.
    naming: Vec<(String, String, bool)>,
    operands: Vec<(String, String)>,
    /// What each namer the system does not serve names that is not hardware, with whether by `needs`.
    unserved: Vec<(String, String, bool)>,
    /// The namers the system serves by its own names and those of the services and hardware it serves, entering no
    /// system it names.
    served_directly: BTreeSet<String>,
}

fn unserved_model(case: &Case) -> UnservedModel {
    let d = &case.description;
    let names_of = |side: &Side, composes: &[&str]| -> Vec<(String, bool)> {
        side.uses
            .iter()
            .filter(|n| !composes.contains(&n.as_str()))
            .map(|n| (n.clone(), false))
            .chain(side.needs.iter().map(|n| (n.clone(), true)))
            .collect()
    };
    let namers: BTreeMap<&str, Vec<(String, bool)>> = BTreeMap::from([
        ("soc", names_of(&d.platforms[0].side, &["timer.d", "dma.x"])),
        ("host.q", names_of(&d.platforms[1].side, &["blk.q"])),
        ("blk.q", names_of(&d.blocks[2].side, &[])),
        ("lone", names_of(&d.blocks[3].side, &[])),
    ]);
    let lowering = d.system.clone();
    let platform_of = |system: &str| -> Vec<&str> {
        if system == "sys" {
            vec!["soc"]
        } else {
            vec!["host.q", "blk.q"]
        }
    };
    let serve = |through_systems: bool| -> (BTreeSet<String>, bool) {
        let mut served: BTreeSet<String> = platform_of(&lowering)
            .into_iter()
            .map(str::to_string)
            .collect();
        let mut alarm = false;
        loop {
            let before = served.len();
            for namer in served.clone() {
                for (item, _) in &namers[namer.as_str()] {
                    match item.as_str() {
                        "sys" | "other" if through_systems && *item != lowering => {
                            served.extend(platform_of(item).into_iter().map(str::to_string));
                        }
                        "via.s" => {
                            served.insert("lone".to_string());
                        }
                        "alarm" => alarm = true,
                        _ => {}
                    }
                }
            }
            if served.len() == before {
                break;
            }
        }
        (served, alarm)
    };
    let (served, alarm) = serve(true);
    let (served_directly, _) = serve(false);
    let mut out = UnservedModel {
        lowering: lowering.clone(),
        outcome: "lowered",
        naming: Vec::new(),
        operands: Vec::new(),
        unserved: Vec::new(),
        served_directly,
    };
    for (namer, items) in &namers {
        for (item, needs) in items {
            if !served.contains(*namer) {
                out.unserved
                    .push(((*namer).to_string(), item.clone(), *needs));
            } else if ["sys", "other"].contains(&item.as_str()) {
                out.naming
                    .push(((*namer).to_string(), item.clone(), *needs));
            } else {
                out.operands.push(((*namer).to_string(), item.clone()));
            }
        }
    }
    out.outcome = if served.contains("lone") {
        "outside"
    } else if lowering == "other" && alarm {
        "unresolved"
    } else if !out.naming.is_empty() {
        "naming"
    } else if !out.operands.is_empty() {
        "operand"
    } else {
        "lowered"
    };
    out
}

#[test]
fn sporadic_tasks_released_by_their_own_sources() {
    let cases = own_sources();
    // Which source is declared first; `t` bound itself or not; what `t` uses, of 4, and `u`, of 3; each of 2 sources'
    // separations, of 6.
    assert_eq!(cases.len(), 2 * 2 * 4 * 3 * 6 * 6);
    let tally = run("own sources", &cases);
    // Fixed by the construction: `t` bound itself in half, model §8 rule 16 binding it to a source that states at least
    // its 20 ms, either one, and the search refusing it where neither does (rules 16 and 6) — `missing-fact` where one
    // states none, else `unsupported-profile`: of the 36 pairs of separations, the 9 of `10 ms`, none and absent, 5 with
    // one stating none, each pair in 24 systems; a record binding `t` to each source rule 16 does not admit refused by
    // the checker and the lowering, by that source's separation; `u`, binding none, beside `t` there, released by the
    // one handler it uses or by none; `t` binding none released by none, by one or by two sources as it uses handlers.
    // Which source `t`'s own binding takes, where both are admitted, is the search's, read from the record, as is the
    // one the unamended rules took: so are the classes of `t`'s own source, of the record those rules gave and of every
    // separation class `t` may decide.
    let bound = |c: &Case| c.label.contains("bound itself");
    let uses = |c: &Case, task: usize| c.description.tasks[task].side.uses.len();
    let separations = |c: &Case| {
        let of = |key: &str| {
            c.label
                .split(' ')
                .find_map(|w| w.strip_prefix(key))
                .expect("a separation")
                .to_string()
        };
        [of("dev.z:"), of("dev.a:")]
    };
    let refused = |c: &Case| bound(c) && !separations(c).iter().any(|s| admits(s, 20));
    let lowered = |c: &Case| !refused(c);
    let unknown = |c: &Case| refused(c) && separations(c).iter().any(|s| s == "Unstated");
    assert_eq!(
        (
            cases.iter().filter(|c| refused(c)).count(),
            cases.iter().filter(|c| unknown(c)).count()
        ),
        (9 * 24, 5 * 24)
    );
    hold(
        "own sources",
        &tally,
        (1_728, 1_728 - 216, 0, 216, 0),
        &[
            counted(RULE_16_SEARCH_CONFLICT, &cases, |c| refused(c) && !unknown(c)),
            counted(RULE_16_SEARCH_UNKNOWN, &cases, unknown),
            counted("trigger: its own source", &cases, |c| bound(c) && lowered(c)),
            counted(
                "trigger: binding none, beside a task bound to its own source",
                &cases,
                |c| bound(c) && lowered(c),
            ),
            counted("trigger: `(unknown unreached)`, no source reaching it", &cases, |c| {
                lowered(c) && (uses(c, 1) == 0 || (!bound(c) && uses(c, 0) == 0))
            }),
            counted("trigger: `(unknown (several ITEM …))`, several sources", &cases, |c| !bound(c) && uses(c, 0) == 2),
            counted("owed: a service or a policy in a job, its call cost and masked runs", &cases, |c| {
                lowered(c) && uses(c, 0) + uses(c, 1) > 0
            }),
            counted("owed: a block or a platform in a job, its call cost and masked runs", &cases, |c| {
                lowered(c) && (bound(c) || uses(c, 0) + uses(c, 1) > 0)
            }),
            exactly("task: deadline-from written", 1_728 - 216),
            counted(RULE_16_RECORD, &cases, |c| {
                bound(c) && separations(c).iter().any(|s| !admits(s, 20))
            }),
            counted(RULE_16_SHORTER, &cases, |c| {
                bound(c) && separations(c).iter().any(|s| s == "Stated(10)")
            }),
            counted(RULE_16_ABSENT, &cases, |c| {
                bound(c) && separations(c).iter().any(|s| s == "Absent")
            }),
            counted(RULE_16_UNSTATED, &cases, |c| {
                bound(c) && separations(c).iter().any(|s| s == "Unstated")
            }),
            some(RULE_16_UNAMENDED),
            some(RULE_16_REBOUND),
            some("trigger: its own source, no other reaching it"),
            some("trigger: its own source, beside another reaching it"),
            some("trigger: its own source releasing it every, beside one that alone would leave it unknown"),
            some("trigger: its own source, reached through a requester its route names too"),
            some("trigger: binding none, reaching the handler of another task's own source"),
            some("trigger: every, equal separations"),
            some("trigger: every, a longer separation"),
            some("trigger: `(unknown (faster ITEM))`, a shorter separation"),
            some("trigger: `(unknown (unstated ITEM))`, no separation stated"),
            some("trigger: `(unknown (absent ITEM))`, the separation absent"),
            some("rank: two stated separations tied"),
        ],
    );
}

#[test]
fn a_binding_keeps_its_edge_across_composition() {
    let cases = bound_across_composition();
    // 2 namings; `P` composing `C` itself or through `B`; what each of 3 declarations requires of the other two, of 4.
    assert_eq!(cases.len(), 2 * 2 * 4usize.pow(3));
    let tally = run("across composition", &cases);
    // Every system lowered. Each count read from who requires whose offer — each of the three the only provider of its
    // own — and who composes whom: a binding from each requirement, an edge from its provider to it; the composite
    // `P` of `B` and `C`, `B` of `C` where it is deep; the texts ordering each composite below its part or above it.
    // [`Composition`] holds them.
    let model: Vec<Composition> = cases.iter().map(Composition::of).collect();
    let count = |class: &'static str, f: &dyn Fn(&Composition) -> bool| {
        exactly(class, model.iter().filter(|m| f(m)).count())
    };
    hold(
        "across composition",
        &tally,
        (256, 256, 0, 0, 0),
        &[
            count(
                "hardware: a block bound for requirements of its own",
                &|m| m.requires(1) || m.requires(2),
            ),
            count(
                "hardware: a platform bound for requirements of its own",
                &|m| m.requires(0),
            ),
            count("hardware: requiring, and bound to by a requester", &|m| {
                (0..3).any(|x| m.requires(x) && m.required(x))
            }),
            count("hardware: requiring, and bound to by nothing", &|m| {
                (0..3).any(|x| m.requires(x) && !m.required(x))
            }),
            count(
                "initialisation: a composite bound to its part, its edge kept",
                &|m| {
                    m.composing()
                        .iter()
                        .any(|(composite, part)| m.edge(*part, *composite))
                },
            ),
            count(
                "initialisation: a part bound to its composite, its edge kept",
                &|m| {
                    m.composing()
                        .iter()
                        .any(|(composite, part)| m.edge(*composite, *part))
                },
            ),
            count(
                "initialisation: a block composing a block, an edge across it kept",
                &|m| m.deep && (m.edge(1, 2) || m.edge(2, 1)),
            ),
            count(
                "initialisation: an edge across composition at depth, through another part, kept",
                &|m| m.deep && (m.edge(0, 2) || m.edge(2, 0)),
            ),
            count(
                "initialisation: a composite and its part bound each to the other, one component",
                &|m| {
                    m.composing().iter().any(|(composite, part)| {
                        m.edge(*part, *composite) && m.edge(*composite, *part)
                    })
                },
            ),
            count("initialisation: a component of several members", &|m| {
                (0..3).any(|x| (0..3).any(|y| x != y && m.reaches(x, y) && m.reaches(y, x)))
            }),
            count(
                "initialisation: an edge across composition ordering two steps unlike their texts",
                &|m| {
                    m.composing().iter().any(|(composite, part)| {
                        [(*composite, *part), (*part, *composite)]
                            .iter()
                            .any(|(earlier, later)| {
                                m.edge(*earlier, *later)
                                    && !m.reaches(*later, *earlier)
                                    && m.names[*earlier] > m.names[*later]
                            })
                    })
                },
            ),
            count(
                "initialisation: an edge to a block or a platform bound for its own requirements",
                &|m| (0..3).any(|x| m.requires(x)),
            ),
            count(
                "owed: a node in no job, neither its call cost nor masked runs",
                &|m| (0..3).any(|x| m.requires(x)),
            ),
            exactly("task: deadline-from written", 256),
        ],
    );
}

/// What the composition universe's construction gives one system: the names of `P`, `B` and `C`, in that order; who
/// requires whose offer, `edges[provider][requester]`, the provider's binding's edge before its requester (model §8
/// rule 12); and whether `P` composes `C` through `B`.
struct Composition {
    names: [String; 3],
    edges: [[bool; 3]; 3],
    deep: bool,
}

impl Composition {
    fn of(case: &Case) -> Self {
        let d = &case.description;
        let declarations = [&d.platforms[0], &d.blocks[0], &d.blocks[1]];
        let offers = ["core-count", "transfer-engine", "debug-port"];
        let mut edges = [[false; 3]; 3];
        for (requester, declaration) in declarations.iter().enumerate() {
            for (provider, offer) in offers.iter().enumerate() {
                if declaration.side.needs.iter().any(|n| n == offer) {
                    edges[provider][requester] = true;
                }
            }
        }
        Self {
            names: declarations.map(|b| b.name.clone()),
            edges,
            deep: !d.blocks[0].side.uses.is_empty(),
        }
    }

    fn edge(&self, earlier: usize, later: usize) -> bool {
        self.edges[earlier][later]
    }

    fn requires(&self, who: usize) -> bool {
        (0..3).any(|provider| self.edges[provider][who])
    }

    fn required(&self, who: usize) -> bool {
        self.edges[who].iter().any(|e| *e)
    }

    fn reaches(&self, from: usize, to: usize) -> bool {
        let mut seen = [false; 3];
        let mut todo = vec![from];
        while let Some(at) = todo.pop() {
            for (next, edge) in self.edges[at].iter().enumerate() {
                if *edge && !seen[next] {
                    seen[next] = true;
                    todo.push(next);
                }
            }
        }
        seen[to]
    }

    /// Each composite with each part it composes, at any depth.
    fn composing(&self) -> Vec<(usize, usize)> {
        if self.deep {
            vec![(0, 1), (0, 2), (1, 2)]
        } else {
            vec![(0, 1), (0, 2)]
        }
    }
}

#[test]
fn every_requester_s_obliged_bound_named_past_the_arithmetic() {
    let cases = obliged_bounds_of_every_requester();
    // What each of 5 requesters writes, of 4.
    assert_eq!(cases.len(), 4usize.pow(5));
    let tally = run("obliged bounds", &cases);
    // Lowered: the 81 where the system's own level requires no deadline and no bound lies past the arithmetic (3 choices
    // of each of the 4 others). Every other refused: naming each bound past the arithmetic, where one is; else the
    // system's own level, receiving its instance's expiry (§1). Each count read from what each requester writes.
    let state = |c: &Case, who: usize| -> Option<&'static str> {
        let d = &c.description;
        let side = [
            &d.own,
            &d.services[0].side,
            &d.services[1].side,
            &d.platforms[0].side,
            &d.blocks[1].side,
        ][who];
        side.needs
            .iter()
            .any(|n| n == "absolute-deadline")
            .then(|| match side.at_most.first() {
                None => "bare",
                Some((_, bound)) if bound.seconds().is_some() => "within",
                Some(_) => "past",
            })
    };
    let past = |c: &Case| (0..5).filter(|w| state(c, *w) == Some("past")).count();
    let lowered = |c: &Case| past(c) == 0 && state(c, 0).is_none();
    let any = |c: &Case, who: &[usize], s: &str| who.iter().any(|w| state(c, *w) == Some(s));
    let deadline = |c: &Case, who: &[usize]| who.iter().any(|w| state(c, *w).is_some());
    hold(
        "obliged bounds",
        &tally,
        (1_024, 81, 0, 0, 943),
        &[
            counted("refusal: a duration past the arithmetic", &cases, |c| {
                past(c) > 0
            }),
            counted("past: an obliged bound", &cases, |c| past(c) > 0),
            counted(
                "past: an obliged bound of the system's own level",
                &cases,
                |c| state(c, 0) == Some("past"),
            ),
            counted("past: an obliged bound of a service", &cases, |c| {
                state(c, 1) == Some("past")
            }),
            counted("past: an obliged bound of a policy", &cases, |c| {
                state(c, 2) == Some("past")
            }),
            counted("past: an obliged bound of a platform", &cases, |c| {
                state(c, 3) == Some("past")
            }),
            counted("past: an obliged bound of a block", &cases, |c| {
                state(c, 4) == Some("past")
            }),
            counted("past: several, named in order", &cases, |c| past(c) >= 2),
            counted("receivers: an expiry", &cases, |c| {
                past(c) == 0 && state(c, 0).is_some()
            }),
            counted("receivers: one", &cases, |c| {
                past(c) == 0 && state(c, 0).is_some()
            }),
            counted("receivers: the system's own level alone", &cases, |c| {
                past(c) == 0 && state(c, 0).is_some()
            }),
            counted("instance: an obliged fact with its bound", &cases, |c| {
                lowered(c) && any(c, &[1, 2, 3, 4], "within")
            }),
            counted("instance: an obliged fact bare", &cases, |c| {
                lowered(c) && any(c, &[1, 2, 3, 4], "bare")
            }),
            counted("instance: a service's obliged bound", &cases, |c| {
                lowered(c) && any(c, &[1], "within")
            }),
            counted("instance: a policy's obliged bound", &cases, |c| {
                lowered(c) && any(c, &[2], "within")
            }),
            counted("instance: a platform's obliged bound", &cases, |c| {
                lowered(c) && any(c, &[3], "within")
            }),
            counted("instance: a block's obliged bound", &cases, |c| {
                lowered(c) && any(c, &[4], "within")
            }),
            counted(
                "hardware: a block bound for requirements of its own",
                &cases,
                |c| lowered(c) && deadline(c, &[4]),
            ),
            counted(
                "hardware: a platform bound for requirements of its own",
                &cases,
                |c| lowered(c) && deadline(c, &[3]),
            ),
            counted("hardware: an expiry running its code", &cases, |c| {
                lowered(c) && deadline(c, &[3, 4])
            }),
            counted(
                "hardware: requiring, and bound to by nothing",
                &cases,
                |c| lowered(c) && deadline(c, &[3, 4]),
            ),
            counted("context: an expiry running its requester", &cases, |c| {
                lowered(c) && deadline(c, &[1, 2, 3, 4])
            }),
            counted("multiplex: a channel with one user", &cases, |c| {
                lowered(c) && deadline(c, &[1, 2])
            }),
            counted("multiplex: its own route no user", &cases, |c| {
                lowered(c) && deadline(c, &[1, 2, 3, 4])
            }),
            counted(
                "owed: an instance in a job, its call cost and masked runs",
                &cases,
                |c| lowered(c) && deadline(c, &[1, 2]),
            ),
            counted(
                "owed: a node in no job, neither its call cost nor masked runs",
                &cases,
                |c| lowered(c) && deadline(c, &[3, 4]),
            ),
            exactly(
                "owed: a service or a policy in a job, its call cost and masked runs",
                81,
            ),
            exactly("task: deadline-from written", 81),
        ],
    );
}

#[test]
fn a_named_edge_from_hardware_a_service_or_a_policy_names() {
    let cases = named_hardware();
    // 2 namings; what the service and the policy each name, of 5; `m.blk` and `m.soc` each requiring an output or not;
    // the service bound to `m.blk` or not, the policy to `m.soc` or not; `m.blk` composed or not; who uses the two, of
    // 3.
    assert_eq!(cases.len(), 2 * 5 * 5 * (1 << 5) * 3);
    let tally = run("named hardware", &cases);
    // Unresolved `infeasible-configuration`, model §8 rule 7's conflict, the 1,344 where a served service or policy
    // names `m.blk` and no platform composes it (2 namings, 16 choices of the other four, and 16 choices of what the
    // two name where both are used, twice, or 10 where only the service is: 2 × 16 × 42), bound to or not, "whatever
    // the assignment". Unresolved `missing-fact`, the 528 where the service needs `m.blk`'s offer, no platform
    // composing it, and no served namer names it (2 namings, 8 choices of the other three, and 9 choices of what the
    // two name where both are used, twice, or 15 where only the service is: 2 × 8 × 33): a requester bound to a block
    // outside the hardware is an assignment's conflict (rule 7), and nothing else offers `transfer-engine`. Lowered:
    // the 2,928 left of the 4,800. 1,344 + 528 + 2,928 = 4,800.
    hold_unresolved("named hardware", &tally, &cases, (1_344, 528), |c| {
        c.label.contains(", bound to m.blk;") && c.label.contains("composed by nothing")
    });
    // Each count read from the label's choices ([`NamedModel`]).
    let model: Vec<NamedModel> = cases.iter().map(NamedModel::of).collect();
    let count = |class: &'static str, f: &dyn Fn(&NamedModel) -> bool| {
        exactly(class, model.iter().filter(|m| f(m)).count())
    };
    let edge_class = |m: &NamedModel, policy: bool, platform: bool, by_needs: bool| {
        m.lowered()
            && m.named_edges().iter().any(|e| {
                e.policy == policy && (e.hardware == "m.soc") == platform && e.by_needs == by_needs
            })
    };
    let bare = |m: &NamedModel, policy: bool, platform: bool| {
        m.lowered()
            && m.named_edges().iter().any(|e| {
                e.policy == policy
                    && (e.hardware == "m.soc") == platform
                    && !m.requiring(e.hardware)
                    && !m.provider(e.hardware)
            })
    };
    hold(
        "named hardware",
        &tally,
        (4_800, 2_928, 0, 1_872, 0),
        &[
            count("initialisation: a named edge from a block a service names by `uses`", &|m| edge_class(m, false, false, false)),
            count("initialisation: a named edge from a block a service names by `needs`", &|m| edge_class(m, false, false, true)),
            count("initialisation: a named edge from a platform a service names by `uses`", &|m| edge_class(m, false, true, false)),
            count("initialisation: a named edge from a platform a service names by `needs`", &|m| edge_class(m, false, true, true)),
            count("initialisation: a named edge from a block a policy names by `uses`", &|m| edge_class(m, true, false, false)),
            count("initialisation: a named edge from a block a policy names by `needs`", &|m| edge_class(m, true, false, true)),
            count("initialisation: a named edge from a platform a policy names by `uses`", &|m| edge_class(m, true, true, false)),
            count("initialisation: a named edge from a platform a policy names by `needs`", &|m| edge_class(m, true, true, true)),
            count("initialisation: a named edge from hardware a provider alone", &|m| {
                m.lowered() && m.named_edges().iter().any(|e| m.provider(e.hardware) && !m.requiring(e.hardware))
            }),
            count("initialisation: a named edge from hardware bound for its own requirements alone", &|m| {
                m.lowered() && m.named_edges().iter().any(|e| !m.provider(e.hardware) && m.requiring(e.hardware))
            }),
            count("initialisation: a named edge from hardware a provider and bound for its own requirements", &|m| {
                m.lowered() && m.named_edges().iter().any(|e| m.provider(e.hardware) && m.requiring(e.hardware))
            }),
            count("initialisation: a named edge from hardware bound for nothing and bound to by nothing", &|m| {
                m.lowered() && m.named_edges().iter().any(|e| !m.provider(e.hardware) && !m.requiring(e.hardware))
            }),
            count("initialisation: a named edge from hardware beside a binding's, one line", &|m| {
                m.lowered() && m.named_edges().iter().any(|e| m.binds(e.namer, e.hardware))
            }),
            count(
                "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike their \
                 texts",
                &|m| m.lowered() && m.below && m.named_edges().iter().any(|e| !m.binds(e.namer, e.hardware)),
            ),
            count(
                "initialisation: hardware a node, named by a service or a policy the system does not serve, no edge",
                &|m| m.lowered() && !m.policy_served() && m.policy_names.is_some_and(|(h, _)| m.node(h)),
            ),
            count("named: a block a service names, bound for nothing, a node", &|m| bare(m, false, false)),
            count("named: a platform a service names, bound for nothing, a node", &|m| bare(m, false, true)),
            count("named: a block a policy names, bound for nothing, a node", &|m| bare(m, true, false)),
            count("named: a platform a policy names, bound for nothing, a node", &|m| bare(m, true, true)),
            count("named: hardware a service or a policy the system does not serve names, no node", &|m| {
                m.lowered() && !m.policy_served() && m.policy_names.is_some_and(|(h, _)| !m.node(h))
            }),
            count("hardware: a block bound for requirements of its own", &|m| m.lowered() && m.requiring("m.blk")),
            count("hardware: a platform bound for requirements of its own", &|m| m.lowered() && m.requiring("m.soc")),
            count("hardware: requiring, and bound to by a requester", &|m| {
                m.lowered() && ["m.blk", "m.soc"].iter().any(|h| m.requiring(h) && m.provider(h))
            }),
            count("hardware: requiring, and bound to by nothing", &|m| {
                m.lowered() && ["m.blk", "m.soc"].iter().any(|h| m.requiring(h) && !m.provider(h))
            }),
            count("initialisation: a composite bound to its part, its edge kept", &|m| m.lowered() && m.soc_requires),
            count("initialisation: an edge to a block or a platform bound for its own requirements", &|m| {
                m.lowered() && (m.requiring("m.blk") || m.requiring("m.soc"))
            }),
            count("second system: requiring hardware the lowered system does not serve", &|m| {
                m.lowered() && m.blk_requires && !m.composed
            }),
            count(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &|m| m.outcome() == "outside"),
            count("outside: a block no platform composes", &|m| m.outcome() == "outside"),
            count("outside: named by a service", &|m| m.outcome() == "outside" && m.service_names.is_some_and(|(h, _)| h == "m.blk")),
            count("outside: named by a policy", &|m| {
                m.outcome() == "outside" && m.policy_served() && m.policy_names.is_some_and(|(h, _)| h == "m.blk")
            }),
            count("outside: named by `uses`", &|m| m.outcome() == "outside" && m.names_block_by(false)),
            count("outside: named by `needs`", &|m| m.outcome() == "outside" && m.names_block_by(true)),
            count("outside: its namer served by a task", &|m| m.outcome() == "outside" && m.users != NamersUsed::ByOwnLevel),
            count("outside: its namer served by the system's own level alone", &|m| {
                m.outcome() == "outside" && m.users == NamersUsed::ByOwnLevel
            }),
            count("outside: requiring something of its own", &|m| m.outcome() == "outside" && m.blk_requires),
            count("outside: several", &|m| m.outcome() == "outside" && m.outside_pairs() == 2),
            count("outside: one block or platform named by two", &|m| m.outcome() == "outside" && m.outside_pairs() == 2),
            count(
                "outside: beside hardware a served service or policy names",
                &|m| {
                    m.outcome() == "outside"
                        && (m.service_names.is_some_and(|(h, _)| h == "m.soc")
                            || (m.policy_served() && m.policy_names.is_some_and(|(h, _)| h == "m.soc")))
                },
            ),
            exactly("task: deadline-from written", 2_928),
            some("named: a block hardware a served requester names composes, nothing calling nor binding it, no node"),
            some("composition: a platform a task reaches through a binding"),
            some("owed: a service or a policy in a job, its call cost and masked runs"),
            some("owed: a block or a platform in a job, its call cost and masked runs"),
            some("owed: a node in no job, neither its call cost nor masked runs"),
        ],
    );
}

/// What the named-hardware universe's construction gives one system ([`named_hardware`]): what the service and the
/// policy name, each `(hardware, by needs)`; the five choices; who uses the two; and whether the namers' texts lie below
/// the hardware's.
struct NamedModel {
    service_names: Option<(&'static str, bool)>,
    policy_names: Option<(&'static str, bool)>,
    blk_requires: bool,
    s_binds: bool,
    composed: bool,
    soc_requires: bool,
    p_binds: bool,
    users: NamersUsed,
    below: bool,
}

/// A named edge, from the hardware a service or a policy the system serves names to that namer.
struct NamedEdge {
    hardware: &'static str,
    namer: &'static str,
    policy: bool,
    by_needs: bool,
}

impl NamedModel {
    fn of(case: &Case) -> Self {
        let d = &case.description;
        let names = |side: &Side| -> Option<(&'static str, bool)> {
            ["m.blk", "m.soc"].into_iter().find_map(|h| {
                if side.uses.iter().any(|n| n == h) {
                    Some((h, false))
                } else if side.needs.iter().any(|n| n == h) {
                    Some((h, true))
                } else {
                    None
                }
            })
        };
        let needs = |side: &Side, fact: &str| side.needs.iter().any(|n| n == fact);
        let users = if !d.own.uses.is_empty() {
            NamersUsed::ByOwnLevel
        } else if d.tasks[0].side.uses.len() == 2 {
            NamersUsed::ByTask
        } else {
            NamersUsed::PolicyByNothing
        };
        Self {
            service_names: names(&d.services[0].side),
            policy_names: names(&d.services[1].side),
            blk_requires: needs(&d.blocks[1].side, "observable-output"),
            s_binds: needs(&d.services[0].side, "transfer-engine"),
            composed: d.platforms[0].side.uses.iter().any(|n| n == "m.blk"),
            soc_requires: needs(&d.platforms[0].side, "observable-output"),
            p_binds: needs(&d.services[1].side, "core-count"),
            users,
            below: d.services[0].name.starts_with("a."),
        }
    }

    fn policy_served(&self) -> bool {
        self.users != NamersUsed::PolicyByNothing
    }

    /// The service's and the served policy's names of `m.blk`, a block no platform composes, `m.soc` being the
    /// system's: model §8 rule 7's conflict's pairs ([`outside_hardware`]).
    fn outside_pairs(&self) -> usize {
        let outside = |names: Option<(&str, bool)>| {
            usize::from(!self.composed && names.is_some_and(|(h, _)| h == "m.blk"))
        };
        outside(self.service_names)
            + if self.policy_served() {
                outside(self.policy_names)
            } else {
                0
            }
    }

    fn names_block_by(&self, by_needs: bool) -> bool {
        let names = |n: Option<(&str, bool)>| n == Some(("m.blk", by_needs));
        names(self.service_names) || (self.policy_served() && names(self.policy_names))
    }

    /// Model §8 rule 7's conflict where a served namer names `m.blk` and no platform composes it, whatever is bound;
    /// otherwise unresolved where the service is bound to it so, an assignment's conflict with nothing else offering
    /// the fact; lowered otherwise.
    fn outcome(&self) -> &'static str {
        if self.outside_pairs() > 0 {
            "outside"
        } else if self.s_binds && !self.composed {
            "unresolved"
        } else {
            "lowered"
        }
    }

    fn lowered(&self) -> bool {
        self.outcome() == "lowered"
    }

    /// Whether `namer` is bound to `hardware`: the service to `m.blk` for `transfer-engine`, the served policy to
    /// `m.soc` for `core-count`.
    fn binds(&self, namer: &str, hardware: &str) -> bool {
        match (namer, hardware) {
            ("service", "m.blk") => self.s_binds,
            ("policy", "m.soc") => self.p_binds && self.policy_served(),
            _ => false,
        }
    }

    fn provider(&self, hardware: &str) -> bool {
        self.binds("service", hardware) || self.binds("policy", hardware)
    }

    fn requiring(&self, hardware: &str) -> bool {
        match hardware {
            "m.blk" => self.blk_requires && self.composed,
            _ => self.soc_requires,
        }
    }

    fn named_edges(&self) -> Vec<NamedEdge> {
        let mut out = Vec::new();
        for (namer, names, policy) in [
            ("service", self.service_names, false),
            (
                "policy",
                self.policy_names.filter(|_| self.policy_served()),
                true,
            ),
        ] {
            if let Some((hardware, by_needs)) = names {
                out.push(NamedEdge {
                    hardware,
                    namer,
                    policy,
                    by_needs,
                });
            }
        }
        out
    }

    fn node(&self, hardware: &str) -> bool {
        self.requiring(hardware)
            || self.provider(hardware)
            || self.named_edges().iter().any(|e| e.hardware == hardware)
    }
}

#[test]
fn a_named_edge_from_hardware_whose_interrupt_runs_its_namer() {
    let cases = named_hardware_in_interrupts();
    // 2 namings; what the namer requires, of 6, and names, of 5; a service or a policy; used by `t` or the system's own
    // level.
    assert_eq!(cases.len(), 2 * 6 * 5 * 2 * 2);
    let tally = run("named hardware in interrupts", &cases);
    // Every system lowered. Each count read from what the one namer requires and names: an alarm at `m.tmr` itself, a
    // deadline through an instance over it, `m.src`'s interrupt, as the construction writes them — so the named block a
    // provider of the namer, directly or through the instance, or of nothing; `m.tmr`'s timer route running the namer
    // where it programs it, `m.src`'s where it handles it; the namer taking a channel where it programs an alarm or a
    // deadline, its own route no user of it, the source's route one beside the task where both reach it.
    let namer = |c: &Case| c.description.services[0].clone();
    let requires = |c: &Case, fact: &str| namer(c).side.needs.iter().any(|n| n == fact);
    let named = |c: &Case| -> Option<(&'static str, bool)> {
        let side = namer(c).side;
        ["m.tmr", "m.src"].into_iter().find_map(|h| {
            if side.uses.iter().any(|n| n == h) {
                Some((h, false))
            } else if side.needs.iter().any(|n| n == h) {
                Some((h, true))
            } else {
                None
            }
        })
    };
    let alarm = |c: &Case| requires(c, "relative-delay");
    let deadline = |c: &Case| requires(c, "absolute-deadline");
    let source = |c: &Case| requires(c, INTERRUPT_FACT);
    let by_task = |c: &Case| !c.description.tasks[0].side.uses.is_empty();
    let policy = |c: &Case| namer(c).policy;
    // The named hardware bound to by the namer itself, or through its instance; and bound for nothing at all.
    let direct = |c: &Case| match named(c) {
        Some(("m.tmr", _)) => alarm(c),
        Some(_) => source(c),
        None => false,
    };
    let provider =
        |c: &Case| direct(c) || (named(c).is_some_and(|(h, _)| h == "m.tmr") && deadline(c));
    let unbound = |c: &Case| named(c).is_some() && !provider(c);
    // An unbound named block in the interrupt that runs its namer: the other device's route.
    let in_interrupt = |c: &Case| {
        unbound(c)
            && match named(c) {
                Some(("m.tmr", _)) => source(c),
                _ => alarm(c) || deadline(c),
            }
    };
    let edge = |c: &Case, is_policy: bool, by_needs: bool| {
        policy(c) == is_policy && named(c).is_some_and(|(_, needs)| needs == by_needs)
    };
    let channel = |c: &Case| alarm(c) || deadline(c);
    hold(
        "named hardware in interrupts",
        &tally,
        (240, 240, 0, 0, 0),
        &[
            counted("initialisation: a named edge from a block a service names by `uses`", &cases, |c| edge(c, false, false)),
            counted("initialisation: a named edge from a block a service names by `needs`", &cases, |c| edge(c, false, true)),
            counted("initialisation: a named edge from a block a policy names by `uses`", &cases, |c| edge(c, true, false)),
            counted("initialisation: a named edge from a block a policy names by `needs`", &cases, |c| edge(c, true, true)),
            counted("initialisation: a named edge from hardware a provider alone", &cases, provider),
            counted("initialisation: a named edge from hardware beside a binding's, one line", &cases, direct),
            counted(
                "initialisation: a named edge from hardware an instance programs, beside the instance's path",
                &cases,
                |c| named(c).is_some_and(|(h, _)| h == "m.tmr") && deadline(c),
            ),
            counted("initialisation: a named edge from hardware whose own interrupt runs its namer", &cases, provider),
            counted(
                "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
                &cases,
                unbound,
            ),
            counted(
                "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike their \
                 texts",
                &cases,
                |c| namer(c).name == "a.x" && named(c).is_some() && !provider(c),
            ),
            counted("named: hardware named and bound for nothing, in a job", &cases, |c| unbound(c) && by_task(c)),
            counted("named: hardware named and bound for nothing, in no job", &cases, |c| unbound(c) && !by_task(c)),
            counted("named: hardware named and bound for nothing, in an interrupt that runs its namer", &cases, in_interrupt),
            counted("named: a block a service names, bound for nothing, a node", &cases, |c| {
                unbound(c) && !policy(c) && !source(c)
            }),
            counted("named: a block a policy names, bound for nothing, a node", &cases, |c| {
                unbound(c) && policy(c) && !source(c)
            }),
            counted("named: a block a source's handler names, bound for nothing, a node", &cases, |c| {
                unbound(c) && source(c)
            }),
            counted("named: hardware named by `needs`, bound for nothing, a node", &cases, |c| {
                unbound(c) && named(c).is_some_and(|(_, needs)| needs)
            }),
            counted("named: hardware named by `uses`, bound for nothing, a node", &cases, |c| {
                unbound(c) && named(c).is_some_and(|(_, needs)| !needs)
            }),
            counted(
                "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                &cases,
                |c| named(c).is_some_and(|(h, _)| h == "m.tmr") && deadline(c),
            ),
            counted("context: a node in two interrupts", &cases, |c| channel(c) && source(c)),
            counted("context: an expiry running its requester", &cases, deadline),
            counted("instance: an obliged fact bare", &cases, deadline),
            counted("multiplex: its own route no user", &cases, channel),
            counted("multiplex: another route a user", &cases, |c| channel(c) && source(c)),
            counted("multiplex: multiplexed", &cases, |c| channel(c) && source(c) && by_task(c)),
            counted("multiplex: multiplexed by a job and an interrupt", &cases, |c| {
                channel(c) && source(c) && by_task(c)
            }),
            counted("multiplex: a policy multiplexed", &cases, |c| channel(c) && source(c) && by_task(c) && policy(c)),
            counted("multiplex: a channel with one user", &cases, |c| channel(c) && source(c) != by_task(c)),
            counted("owed: a service or a policy in a job, its call cost and masked runs", &cases, by_task),
            counted("owed: an instance in a job, its call cost and masked runs", &cases, |c| deadline(c) && by_task(c)),
            counted("owed: a block or a platform in a job, its call cost and masked runs", &cases, |c| {
                by_task(c) && (named(c).is_some() || !namer(c).side.needs.is_empty())
            }),
            counted("owed: a node in no job, neither its call cost nor masked runs", &cases, |c| !by_task(c)),
            exactly("task: deadline-from written", 240),
        ],
    );
}

/// Whether `line` names `name` as a word of its own: a plan's tokens are separated by spaces and parentheses (§3).
fn mentions(line: &str, name: &str) -> bool {
    line.split([' ', '(', ')']).any(|token| token == name)
}

#[test]
fn hardware_a_served_requester_names_a_node_bound_or_not() {
    let cases = unbound_named_hardware();
    // What is named, of 3; which of 5 namers name it; by `uses` or `needs`; the three services used by `t` or by the
    // system's own level; `w` bound to it or not.
    assert_eq!(cases.len(), 3 * (1 << 5) * 2 * 2 * 2);
    let (tally, plans) = run_keeping("unbound named hardware", &cases, true);
    // Unresolved `infeasible-configuration`, model §8 rule 7's conflict, the 56 where some of the three services, each
    // served, names `m.blk`, which no platform composes and neither `t` nor the system's own level names (7 subsets of
    // the three, 2 clauses, 2 users, `w` bound to it or not, "whatever the assignment"). Unresolved `missing-fact`, the
    // 4 where `w` is bound to it and nothing names it (2 clauses, 2 users): a requester bound to a block outside the
    // hardware is an assignment's conflict (rule 7), and nothing else offers `transfer-engine`. Lowered: the 708 left
    // of the 768.
    hold_unresolved("unbound named hardware", &tally, &cases, (56, 4), |c| {
        c.label.starts_with("Alone ")
            && c.label.ends_with("w bound to it")
            && !c.label.contains("\"t\"")
            && !c.label.contains("\"sys\"")
    });
    // Each count read from the label: what is named, by whom and by which clause, who uses the three services, and
    // whether `w` binds it. A served namer — `t`, `a.s`, `a.p` or `a.h`, never the system's own level — makes the named
    // hardware a node, bound or not; `m.blk` no platform composes is hardware only where `t` or the system's own level
    // names it, and otherwise the search's conflict where a service names it, or unresolved where `w` alone binds it.
    let target = |c: &Case| {
        if c.label.starts_with("Platform ") {
            "m.soc"
        } else {
            "m.blk"
        }
    };
    let alone = |c: &Case| c.label.starts_with("Alone ");
    let by = |c: &Case, namer: &str| {
        c.label
            .split(" named by [")
            .nth(1)
            .and_then(|r| r.split(']').next())
            .is_some_and(|namers| namers.split(", ").any(|n| n == format!("{namer:?}")))
    };
    let by_needs = |c: &Case| c.label.contains("] by needs;");
    let by_task = |c: &Case| c.label.contains("used by t;");
    let bound = |c: &Case| c.label.ends_with("w bound to it");
    let services = ["a.s", "a.p", "a.h"];
    let by_service = |c: &Case| services.iter().any(|n| by(c, n));
    let hardware = |c: &Case| !alone(c) || by(c, "t") || by(c, "sys");
    let outcome = |c: &Case| {
        if hardware(c) {
            "lowered"
        } else if by_service(c) {
            "outside"
        } else if bound(c) {
            "unresolved"
        } else {
            "lowered"
        }
    };
    let lowered = |c: &Case| outcome(c) == "lowered";
    let called = |c: &Case| lowered(c) && (by(c, "t") || by_service(c));
    let bare = |c: &Case| called(c) && !bound(c);
    let named_as = |c: &Case, namer: &str| bare(c) && by(c, namer);
    let in_job = |c: &Case| by(c, "t") || (by_task(c) && by_service(c));
    let edge = |c: &Case, namers: &[&str], platform: bool, needs: bool| {
        lowered(c)
            && (target(c) == "m.soc") == platform
            && by_needs(c) == needs
            && namers.iter().any(|n| by(c, n))
    };
    let outside = |c: &Case| outcome(c) == "outside";
    hold(
        "unbound named hardware",
        &tally,
        (768, 708, 0, 60, 0),
        &[
            counted("named: a block a task names, bound for nothing, a node", &cases, |c| {
                named_as(c, "t") && target(c) == "m.blk"
            }),
            counted("named: a platform a task names, bound for nothing, a node", &cases, |c| {
                named_as(c, "t") && target(c) == "m.soc"
            }),
            // Round 24 (R24-D1): what `t` names a member of its `(uses …)`, bound or not.
            counted("uses: a block a task names, a member", &cases, |c| {
                lowered(c) && by(c, "t") && target(c) == "m.blk"
            }),
            counted("uses: a platform a task names, a member", &cases, |c| {
                lowered(c) && by(c, "t") && target(c) == "m.soc"
            }),
            counted(
                "uses: hardware and a service or a policy a task names, by name",
                &cases,
                |c| lowered(c) && by(c, "t") && by_task(c),
            ),
            counted("named: a block a service names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.s") && target(c) == "m.blk"
            }),
            counted("named: a platform a service names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.s") && target(c) == "m.soc"
            }),
            counted("named: a block a policy names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.p") && target(c) == "m.blk"
            }),
            counted("named: a platform a policy names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.p") && target(c) == "m.soc"
            }),
            counted("named: a block a source's handler names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.h") && target(c) == "m.blk"
            }),
            counted("named: a platform a source's handler names, bound for nothing, a node", &cases, |c| {
                named_as(c, "a.h") && target(c) == "m.soc"
            }),
            counted("named: hardware named by `uses`, bound for nothing, a node", &cases, |c| bare(c) && !by_needs(c)),
            counted("named: hardware named by `needs`, bound for nothing, a node", &cases, |c| bare(c) && by_needs(c)),
            counted("named: hardware named and bound for nothing, in a job", &cases, |c| bare(c) && in_job(c)),
            counted("named: hardware named and bound for nothing, in no job", &cases, |c| bare(c) && !in_job(c)),
            counted(
                "named: hardware named and bound for nothing, in an interrupt that runs its namer",
                &cases,
                |c| bare(c) && by(c, "a.h"),
            ),
            counted(
                "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                &cases,
                |c| called(c) && bound(c),
            ),
            counted("named: hardware the system's own level names, no node", &cases, |c| {
                lowered(c) && by(c, "sys") && !called(c) && !bound(c)
            }),
            counted(
                "named: a block hardware a served requester names composes, nothing calling nor binding it, no node",
                &cases,
                |c| called(c) && target(c) == "m.soc",
            ),
            counted("composition: a platform a task reaches through a binding", &cases, |c| {
                called(c) && target(c) == "m.soc" && in_job(c)
            }),
            counted("composition: what a node in a job composes is in none", &cases, |c| {
                called(c) && target(c) == "m.soc" && by(c, "t") && !by_task(c)
            }),
            counted("initialisation: a named edge from a block a service names by `uses`", &cases, |c| {
                edge(c, &["a.s", "a.h"], false, false)
            }),
            counted("initialisation: a named edge from a block a service names by `needs`", &cases, |c| {
                edge(c, &["a.s", "a.h"], false, true)
            }),
            counted("initialisation: a named edge from a platform a service names by `uses`", &cases, |c| {
                edge(c, &["a.s", "a.h"], true, false)
            }),
            counted("initialisation: a named edge from a platform a service names by `needs`", &cases, |c| {
                edge(c, &["a.s", "a.h"], true, true)
            }),
            counted("initialisation: a named edge from a block a policy names by `uses`", &cases, |c| {
                edge(c, &["a.p"], false, false)
            }),
            counted("initialisation: a named edge from a block a policy names by `needs`", &cases, |c| {
                edge(c, &["a.p"], false, true)
            }),
            counted("initialisation: a named edge from a platform a policy names by `uses`", &cases, |c| {
                edge(c, &["a.p"], true, false)
            }),
            counted("initialisation: a named edge from a platform a policy names by `needs`", &cases, |c| {
                edge(c, &["a.p"], true, true)
            }),
            counted(
                "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
                &cases,
                |c| lowered(c) && by_service(c) && !bound(c),
            ),
            counted("initialisation: a named edge from hardware a provider alone", &cases, |c| {
                lowered(c) && by_service(c) && bound(c)
            }),
            counted(
                "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike their \
                 texts",
                &cases,
                |c| lowered(c) && by_service(c),
            ),
            counted(
                "hardware: a block a served service or policy names, which a task's name makes hardware",
                &cases,
                |c| by_service(c) && by(c, "t"),
            ),
            counted(
                "hardware: a block a served service or policy names, which the system's own level's name makes hardware",
                &cases,
                |c| by_service(c) && by(c, "sys"),
            ),
            counted(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &cases, outside),
            counted("outside: a block no platform composes", &cases, outside),
            counted("outside: named by a service", &cases, |c| outside(c) && (by(c, "a.s") || by(c, "a.h"))),
            counted("outside: named by a policy", &cases, |c| outside(c) && by(c, "a.p")),
            counted("outside: named by `uses`", &cases, |c| outside(c) && !by_needs(c)),
            counted("outside: named by `needs`", &cases, |c| outside(c) && by_needs(c)),
            counted("outside: its namer served by a task", &cases, |c| outside(c) && by_task(c)),
            counted("outside: its namer served by the system's own level alone", &cases, |c| outside(c) && !by_task(c)),
            counted("outside: several", &cases, |c| {
                outside(c) && services.iter().filter(|n| by(c, n)).count() >= 2
            }),
            counted("outside: one block or platform named by two", &cases, |c| {
                outside(c) && services.iter().filter(|n| by(c, n)).count() >= 2
            }),
            counted("owed: a service or a policy in a job, its call cost and masked runs", &cases, |c| {
                lowered(c) && by_task(c)
            }),
            exactly("task: deadline-from written", 708),
            some("owed: a block or a platform in a job, its call cost and masked runs"),
            some("owed: a node in no job, neither its call cost nor masked runs"),
        ],
    );
    // "placed when an unrelated binding makes it a node" (R18-D1): each system beside its twin, the one with `w` bound
    // to the named hardware, where a served task, service or policy names it — `w`, used by the system's own level
    // alone, runs in no job and no interrupt, so a provider of it takes no context from it (§2.6). Every line naming
    // the hardware, its context, its edges to its namers and what it owes, is the same in both, but for `w`'s own lines
    // and the steps, which `w`'s node renumbers. Compared: 3 targets × 30 subsets with a served namer × 2 clauses × 2
    // users, less the 28 the search refuses with their twins, `m.blk` outside the hardware (7 subsets of the three
    // services alone × 2 × 2).
    let mut compared = 0;
    for (pair, plans) in cases.chunks(2).zip(plans.chunks(2)) {
        let [alone, beside] = pair else {
            unreachable!("pairs")
        };
        assert_eq!(
            alone.label.strip_suffix("nothing bound to it"),
            beside.label.strip_suffix("w bound to it"),
            "a system and its twin"
        );
        let target = if alone.label.starts_with("Platform ") {
            "m.soc"
        } else {
            "m.blk"
        };
        let served_namer = ["\"t\"", "\"a.s\"", "\"a.p\"", "\"a.h\""]
            .iter()
            .any(|n| alone.label.contains(n));
        let (Some(without), Some(with)) = (&plans[0], &plans[1]) else {
            continue;
        };
        if !served_namer {
            continue;
        }
        let naming = |plan: &str| -> Vec<String> {
            plan.lines()
                .filter(|l| mentions(l, target) && !mentions(l, "w") && !l.starts_with("(step "))
                .map(str::to_string)
                .collect()
        };
        assert!(
            naming(without)
                .iter()
                .any(|l| l.starts_with(&format!("(context {target} "))),
            "{}: `{target}`, named by a served requester, a node\n{without}",
            alone.label
        );
        assert_eq!(
            naming(without),
            naming(with),
            "{}: `w`'s binding changes what the name gives",
            alone.label
        );
        compared += 1;
    }
    assert_eq!(compared, 3 * 30 * 2 * 2 - 28);
}

/// Re-purposed by `M1.42` (`RTP-H14`), and renamed from `the_bullet_over_tasks_taken_whole_in_runtime_order`. This held
/// §1's bullet over tasks whole, taken task by task in runtime order and each task's clauses in the order the bullet
/// writes them, naming the task and the clause (round 20, R20-R7). A task's duration that is not a time and a release
/// of zero have left the bullet for the language (model §4 rule 8), whose check comes before the search and §1 alike:
/// no order among them is left to hold and no message of the lowering's to read, so the classes of both went with them.
/// What stays is the language's verdict on every such system, whatever else it writes, each clause and each unit held
/// to the systems writing it, over the same 2,048 combinations; beside them a zero deadline, which the rule leaves
/// alone, a task stating no release or no deadline, the language's by other rules, and what the bullet keeps, a
/// periodic task bound to its source (`M1.43`, not yet moved), taken in runtime order before §1's later refusals, which
/// what the language accepts reaches by what else it writes alone.
#[test]
fn a_task_s_duration_rejected_by_the_language_whatever_else_it_writes() {
    let cases = task_bullet();
    // What `t` and `u` each write, of 16; which is first; what else the system meets, of 4.
    assert_eq!(cases.len(), 16 * 16 * 2 * 4);
    let tally = run("task bullet", &cases);
    // Of the 16 choices of [`BULLET_CLAUSES`], the 11 of `TABLE` below write what model §4 rule 8 refuses. Of the other
    // 5, the language accepts 3: `well-formed`, `zero deadline`, a time the rule leaves alone, and `periodic bound to
    // its source`, the bullet's until `M1.43`. It rejects 2 by other rules: `no release`, `missing-fact`, and `no
    // deadline`, `invalid-description`, a deadline's cardinality. Each of the 256 pairs is 2 orders × 4 others = 8
    // systems:
    // - Rejected by rule 8: a pair writing one of the 11 at least, 256 - 5 × 5 = 231 of them, 1,848 systems, whatever
    //   else the pair writes and whatever else the system meets. `invalid-description` above any `missing-fact` beside
    //   it (`eadl_model::check`: "The **verdict** is the highest-precedence one"; `Verdict::precedence`).
    // - Rejected by the others: of the 5 × 5 = 25 pairs left, all but the 3 × 3 = 9 the language accepts, 16 of them,
    //   128 systems. `invalid-description` for the 5 × 5 - 4 × 4 = 9 with `no deadline`, 72 systems; `missing-fact` for
    //   the other 7, each with `no release`, 56 systems. So 1,848 + 128 = 1,976 rejected: 1,920 and 56.
    // - Refused by what the bullet keeps: of the 9 pairs accepted, the 9 - 2 × 2 = 5 with `periodic bound to its
    //   source`, whatever else the system meets, 40 systems.
    // - Of the 2 × 2 = 4 pairs left, each of `well-formed` or `zero deadline`, 2 orders: 8 systems for each other,
    //   lowered where it meets nothing, otherwise refused by the one refusal of §1 it meets alone, 24 systems. So 8
    //   lowered, 1,976 rejected, none unresolved, 40 + 24 = 64 refused: 2,048.
    let verdict = |verdict: &str| tally.verdicts.get(verdict).map_or(0, |v| v.0);
    assert_eq!(
        (
            verdict("rejected invalid-description"),
            verdict("rejected missing-fact")
        ),
        ((231 + 9) * 2 * 4, 7 * 2 * 4),
        "the language's answer: {:?}",
        tally.verdicts
    );
    // What each of the 11 writes that rule 8 refuses, derived by hand from the clauses `BULLET_CLAUSES` states it with:
    // its clauses, and the units that are no time it writes them in. A class `n` of these rows write is reached by the
    // systems of the pairs writing one of them at least, (256 - (16 - n) × (16 - n)) × 8: of `n` = 1 for a minimum
    // separation not a time and for each unit but `Hz`, 248; of `n` = 2 for a period not a time, a period of zero, a
    // minimum separation of zero and `Hz`, 480; of `n` = 3 for a deadline not a time, 696; of `n` = 4 for a jitter not
    // a time, 896. Rejected whichever task writes it, whichever comes first and whatever else the pair writes, so no
    // order among them is read.
    type Row = (
        &'static str,
        &'static [&'static str],
        &'static [&'static str],
    );
    const TABLE: [Row; 11] = [
        (
            "period in Hz",
            &["language: a period that is not a time"],
            &["Hz"],
        ),
        (
            "min-separation in KiB",
            &["language: a minimum separation that is not a time"],
            &["KiB"],
        ),
        (
            "deadline in bit",
            &["language: a deadline that is not a time"],
            &["bit"],
        ),
        (
            "jitter in tick",
            &["language: a jitter that is not a time"],
            &["tick"],
        ),
        ("zero period", &["language: a period of zero"], &[]),
        (
            "zero min-separation",
            &["language: a minimum separation of zero"],
            &[],
        ),
        (
            "zero period, deadline in GHz",
            &[
                "language: a deadline that is not a time",
                "language: a period of zero",
            ],
            &["GHz"],
        ),
        (
            "periodic bound to its source, jitter in MiB",
            &["language: a jitter that is not a time"],
            &["MiB"],
        ),
        (
            "period in kHz, bound to its source",
            &["language: a period that is not a time"],
            &["kHz"],
        ),
        (
            "zero min-separation, jitter in MHz",
            &[
                "language: a jitter that is not a time",
                "language: a minimum separation of zero",
            ],
            &["MHz"],
        ),
        (
            "deadline in Hz, jitter in byte",
            &[
                "language: a deadline that is not a time",
                "language: a jitter that is not a time",
            ],
            &["Hz", "byte"],
        ),
    ];
    let row = |c: &Case, task: &str| {
        let word = c
            .label
            .split(&format!("{task}:"))
            .nth(1)
            .expect("a task")
            .split(" u:")
            .next()
            .expect("a word");
        let word = word.split(" first=").next().expect("a word");
        TABLE.iter().find(|r| r.0 == word)
    };
    let writes = |c: &Case, read: &dyn Fn(&Row) -> bool| {
        ["t", "u"].iter().any(|t| row(c, t).is_some_and(read))
    };
    let clause = |class: &'static str| {
        counted(class, &cases, move |c| {
            writes(c, &|r: &Row| r.1.contains(&class))
        })
    };
    let unit = |class: &'static str, unit: &'static str| {
        counted(class, &cases, move |c| {
            writes(c, &|r: &Row| r.2.contains(&unit))
        })
    };
    hold(
        "task bullet",
        &tally,
        (2_048, 8, 1_976, 0, 64),
        &[
            exactly(LANGUAGE_REJECTS_A_TASK_S_DURATION, 231 * 2 * 4),
            clause("language: a period that is not a time"),
            clause("language: a minimum separation that is not a time"),
            clause("language: a deadline that is not a time"),
            clause("language: a jitter that is not a time"),
            clause("language: a period of zero"),
            clause("language: a minimum separation of zero"),
            unit("language: not a time, in `Hz`", "Hz"),
            unit("language: not a time, in `kHz`", "kHz"),
            unit("language: not a time, in `MHz`", "MHz"),
            unit("language: not a time, in `GHz`", "GHz"),
            unit("language: not a time, in `bit`", "bit"),
            unit("language: not a time, in `byte`", "byte"),
            unit("language: not a time, in `KiB`", "KiB"),
            unit("language: not a time, in `MiB`", "MiB"),
            unit("language: not a time, in `tick`", "tick"),
            // What the bullet keeps: 5 pairs, 2 orders, 4 others. Both tasks bound to the source is one pair, so the
            // first in runtime order named, and, where `u` is first, the task declared earlier after it. Beside a
            // system named or hardware naming a service, one other each.
            exactly("refusal: a periodic task bound to a source itself", 5 * 2 * 4),
            exactly(
                "task bullet: the first ill-formed task in runtime order named, a later one ill-formed too",
                2 * 4,
            ),
            exactly(
                "task bullet: a task declared earlier ill-formed too, after the one named in runtime order",
                4,
            ),
            exactly("task bullet: before a requester naming a system", 5 * 2),
            exactly("periodic: before hardware naming what is not hardware", 5 * 2),
            // What else the system meets, alone: the 4 pairs of `well-formed` and `zero deadline`, 2 orders, one other
            // each.
            exactly("refusal: a requester naming a system", 4 * 2),
            exactly("naming: the system's own level naming a system", 4 * 2),
            exactly("refusal: hardware naming what is not hardware", 4 * 2),
            exactly("operand: a platform's `uses` naming a service", 4 * 2),
            exactly("refusal: a duration past the arithmetic", 4 * 2),
            exactly("past: an obliged bound of a service", 4 * 2),
            // Lowered: the 3 of those pairs writing a `zero deadline`, 2 orders, meeting nothing else.
            exactly("task: a zero deadline", 3 * 2),
            exactly("task: deadline-from written", 4 * 2),
        ],
    );
}

#[test]
fn a_block_or_a_platform_served_outside_the_hardware_a_conflict_whatever_else_is_written() {
    let cases = outside_hardware_universe();
    // What `n`, `n.s` and `n!` each name, of 5, 3 and 3; who serves them, of 3; what makes a block hardware, of 4; what
    // else `t` meets, of 7; and 4 systems with no task.
    assert_eq!(cases.len(), 5 * 3 * 3 * 3 * 4 * 7 + 4);
    let tally = run("outside hardware", &cases);
    // Rejected `invalid-description`: the 45 namings × 3 × 4 = 540 choices beside `t`'s zero period (model §4 rule 8;
    // `M1.42`), whatever is served and named, the language's check before the search and §1 alike. Lowered: where every
    // served namer names hardware alone and `t` meets no other refusal. With `t` meeting none, of
    // the namings each served namer's choices that name hardware alone — `n`: nothing, and `x` or `x.y` where made
    // hardware; `n.s`: nothing, and `x.y` where made hardware; `n!`: nothing, and `x!` where made hardware — times the
    // choices of the namers not served: served by `t` and the system's own level, 1 + 2 + 4 + 2 = 9 over the four ways
    // of making hardware; served by the system's own level and through `m`, `n!` free, 3 × (1 + 2 + 4 + 1) = 24; `n!`
    // alone, `n` and `n.s` free, 15 × (1 + 1 + 1 + 2) = 75. With `x` naming `m`, the same where `x` is served by no
    // name, or else the hardware-naming refusal: 7, 18 and 60. 108 + 85 = 193. Those 108 choices with no served name
    // outside are the same whatever else `t` meets. Refused: with each of the 4 it meets by a bullet of §1 (`other`,
    // the jitter, the deadline, the alarm), 4 × 108 = 432; with `x` naming `m`, the 108 - 85 = 23 where `x` is
    // hardware; and the 2 systems with no task whose `n` names nothing. 432 + 23 + 2 = 457. Unresolved
    // `infeasible-configuration`, model §8 rule 7's conflict, each other: beside each of the 6 things `t` meets the
    // language accepts lie 540 choices, 432 of them past those 108, whatever `t` meets, no bullet of §1 reached (6 ×
    // 432 = 2,592); and the 2 systems with no task whose `n` names `x`. 193 + 540 + 2,594 + 457 = 3,784. None is
    // `missing-fact`: `d` offers the one fact anything here requires.
    hold_unresolved("outside hardware", &tally, &cases, (2_594, 0), |_| false);
    let model: Vec<OutsideModel> = cases.iter().map(OutsideModel::of).collect();
    let count = |class: &'static str, f: &dyn Fn(&OutsideModel) -> bool| {
        exactly(class, model.iter().filter(|m| f(m)).count())
    };
    let conflict = |m: &OutsideModel| m.outcome == "outside";
    let blocks = ["x", "x.y", "x!"];
    hold(
        "outside hardware",
        &tally,
        (3_784, 193, 540, 2_594, 457),
        &[
            count(LANGUAGE_REJECTS_A_TASK_S_DURATION, &|m| m.outcome == "rejected"),
            count("language: a period of zero", &|m| m.outcome == "rejected"),
            count(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &conflict),
            count("refusal: a system with no task", &|m| m.outcome == "no task"),
            count("refusal: a requester naming a system", &|m| m.outcome == "naming"),
            count("naming: a task naming a system", &|m| m.outcome == "naming"),
            count("no task: before the system's own level receiving interrupt code", &|m| {
                m.outcome == "no task" && m.also == "the system's own level programs an alarm"
            }),
            count("receivers: one", &|m| m.outcome == "receivers"),
            count("refusal: a deadline measured from another event", &|m| m.outcome == "deadline"),
            count("refusal: a duration past the arithmetic", &|m| m.outcome == "past"),
            count("refusal: hardware naming what is not hardware", &|m| m.outcome == "operand"),
            count("operand: a service only hardware names", &|m| m.outcome == "operand" && m.served != "OwnAndThrough"),
            count("outside: named by a service", &|m| conflict(m) && m.pairs.iter().any(|(n, _, _)| n != "n!")),
            count("outside: named by a policy", &|m| conflict(m) && m.pairs.iter().any(|(n, _, _)| n == "n!")),
            count("outside: named by `uses`", &|m| conflict(m) && m.pairs.iter().any(|(_, _, needs)| !needs)),
            count("outside: named by `needs`", &|m| conflict(m) && m.pairs.iter().any(|(_, _, needs)| *needs)),
            count("outside: its namer served by a task", &|m| {
                conflict(m) && m.pairs.iter().any(|(n, _, _)| m.from_tasks.contains(n))
            }),
            count("outside: its namer served by the system's own level alone", &|m| {
                conflict(m) && m.pairs.iter().any(|(n, _, _)| !m.from_tasks.contains(n))
            }),
            count("outside: its namer served through another service or policy", &|m| {
                conflict(m) && m.served == "OwnAndThrough" && m.pairs.iter().any(|(n, _, _)| n == "n.s")
            }),
            count("outside: a block no platform composes", &|m| {
                conflict(m) && m.pairs.iter().any(|(_, b, _)| blocks.contains(&b.as_str()))
            }),
            count("outside: a block a platform outside the hardware composes", &|m| {
                conflict(m) && m.pairs.iter().any(|(_, b, _)| b == "q")
            }),
            count("outside: a platform another system uses", &|m| conflict(m) && m.pairs.iter().any(|(_, b, _)| b == "p")),
            count("outside: a platform no system uses", &|m| conflict(m) && m.pairs.iter().any(|(_, b, _)| b == "p2")),
            count("outside: several", &|m| conflict(m) && m.pairs.len() >= 2),
            count("outside: one namer naming two", &|m| conflict(m) && m.pairs.windows(2).any(|w| w[0].0 == w[1].0)),
            count("outside: one block or platform named by two", &|m| {
                conflict(m) && m.pairs.iter().any(|(n, b, _)| m.pairs.iter().any(|(o, c, _)| o != n && c == b))
            }),
            count("outside: beside hardware a served service or policy names", &|m| {
                conflict(m) && m.served_names_hardware
            }),
            count(
                "outside: a block outside the hardware named by a service or a policy the system does not serve, no \
                 refusal",
                &|m| m.outcome == "lowered" && m.unserved_name_outside,
            ),
            count(
                "hardware: a block a served service or policy names, which a task's name makes hardware",
                &|m| m.outcome == "lowered" && m.made == "t names x" && m.served_names("x"),
            ),
            count(
                "hardware: a block a served service or policy names, which the system's own level's name makes hardware",
                &|m| m.outcome == "lowered" && m.made == "sys needs x.y" && m.served_names("x.y"),
            ),
            count(
                "hardware: a block a served service or policy names, which a block's or a platform's `needs` brings in",
                &|m| m.outcome == "lowered" && m.made == "soc needs x!" && m.served_names("x!"),
            ),
            count("named: a block a service names, bound for nothing, a node", &|m| {
                m.outcome == "lowered" && m.served_namings.iter().any(|(n, b)| n != "n!" && m.hardware.contains(b))
            }),
            count("named: a block a policy names, bound for nothing, a node", &|m| {
                m.outcome == "lowered" && m.served_namings.iter().any(|(n, b)| n == "n!" && m.hardware.contains(b))
            }),
        ],
    );
}

/// What the outside-hardware universe's construction gives one system ([`outside_hardware_universe`]): who serves the
/// namers, what makes a block hardware, what else `t` meets, what each namer names — and so what it comes to: the
/// language's rejection of a zero period, whatever else it writes, its check before the search (model §4 rule 8;
/// `M1.42`); model §8 rule 7's conflict where a served namer names a block or a platform outside the hardware, `d` and
/// whatever `t`'s, the system's own or `soc`'s `needs` names being the hardware, the search's before any refusal of §1;
/// otherwise §1's refusal given, in its order: the naming-system refusal where `t` names `other`; the hardware-naming
/// refusal where `x`, served, names `m`; then a jitter past the arithmetic; the system with no task; a deadline from
/// another event; and an alarm `t` programs.
struct OutsideModel {
    served: &'static str,
    made: &'static str,
    also: &'static str,
    outcome: &'static str,
    /// Each served namer with each block or platform outside the hardware it names, and whether by `needs`.
    pairs: Vec<(String, String, bool)>,
    /// What each served namer names.
    served_namings: Vec<(String, String)>,
    from_tasks: BTreeSet<String>,
    hardware: BTreeSet<String>,
    served_names_hardware: bool,
    unserved_name_outside: bool,
}

impl OutsideModel {
    fn of(case: &Case) -> Self {
        let d = &case.description;
        let label = &case.label;
        let pick =
            |options: &[&'static str]| *options.iter().find(|o| label.contains(*o)).unwrap_or(&"");
        let no_task = d.tasks.is_empty();
        let served = if no_task {
            "NoTask"
        } else {
            pick(&["TaskAndOwn", "OwnAndThrough", "PolicyAlone"])
        };
        let made = pick(&["t names x", "sys needs x.y", "soc needs x!"]);
        let also = if no_task {
            if label.contains("programs an alarm") {
                "the system's own level programs an alarm"
            } else {
                "nothing"
            }
        } else {
            pick(&[
                "also t's zero period",
                "also t names other",
                "also t programs an alarm",
                "also t's deadline from another event",
                "also t's jitter past the arithmetic",
                "also x names m",
            ])
        };
        let (namers, from_tasks): (Vec<&str>, Vec<&str>) = match served {
            "TaskAndOwn" => (vec!["n", "n.s", "n!"], vec!["n", "n.s"]),
            "OwnAndThrough" => (vec!["n", "n.s"], vec!["n.s"]),
            "PolicyAlone" => (vec!["n!"], vec!["n!"]),
            _ => (vec!["n"], vec![]),
        };
        let mut hardware: BTreeSet<String> = BTreeSet::from(["d".to_string()]);
        match made {
            "t names x" => hardware.insert("x".to_string()),
            "sys needs x.y" => hardware.insert("x.y".to_string()),
            "soc needs x!" => hardware.insert("x!".to_string()),
            _ => false,
        };
        let namings = |namer: &str| -> Vec<(String, bool)> {
            let side = &d
                .services
                .iter()
                .find(|s| s.name == namer)
                .expect("a namer")
                .side;
            side.uses
                .iter()
                .map(|n| (n.clone(), false))
                .chain(side.needs.iter().map(|n| (n.clone(), true)))
                .collect()
        };
        let mut pairs = Vec::new();
        let mut served_namings = Vec::new();
        for namer in &namers {
            for (named, needs) in namings(namer) {
                served_namings.push(((*namer).to_string(), named.clone()));
                if !hardware.contains(&named) {
                    pairs.push(((*namer).to_string(), named, needs));
                }
            }
        }
        let unserved_name_outside = ["n", "n.s", "n!"]
            .iter()
            .filter(|n| !namers.contains(n))
            .any(|n| {
                namings(n)
                    .iter()
                    .any(|(named, _)| !hardware.contains(named))
            });
        let x_served =
            hardware.contains("x") || served_namings.iter().any(|(_, named)| named == "x");
        let outcome = if also == "also t's zero period" {
            "rejected"
        } else if !pairs.is_empty() {
            "outside"
        } else if also == "also t names other" {
            "naming"
        } else if also == "also x names m" && x_served {
            "operand"
        } else if also == "also t's jitter past the arithmetic" {
            "past"
        } else if no_task {
            "no task"
        } else if also == "also t's deadline from another event" {
            "deadline"
        } else if also == "also t programs an alarm" {
            "receivers"
        } else {
            "lowered"
        };
        Self {
            served,
            made,
            also,
            outcome,
            served_names_hardware: served_namings
                .iter()
                .any(|(_, named)| hardware.contains(named)),
            pairs,
            served_namings,
            from_tasks: from_tasks.into_iter().map(str::to_string).collect(),
            hardware,
            unserved_name_outside,
        }
    }

    fn served_names(&self, block: &str) -> bool {
        self.served_namings.iter().any(|(_, named)| named == block)
    }
}

#[test]
fn one_service_each_serving_system_s_conflict_alone() {
    let cases = one_service_two_systems();
    // What `log` names, of 6; who uses it, of 5; each description lowered for each system.
    assert_eq!(cases.len(), 6 * 5 * 2);
    for pair in cases.chunks(2) {
        assert_eq!(
            pair[0].description.text(),
            pair[1].description.text(),
            "one description, lowered for each of its systems"
        );
    }
    let (tally, plans) = run_keeping("one service, two systems", &cases, true);
    // Unresolved `infeasible-configuration`, model §8 rule 7's conflict, each system serving `log` — 3 of the 5 users:
    // its task, both tasks or its own level — where `log` names what its hardware does not hold: of `sa`, `uart.b` and
    // `pb` (3 × 2), of `sb`, `uart.a` by either clause and `pa` (3 × 3): 15. `log` requires no fact, so none is
    // `missing-fact`. Lowered: the other 45. Each count read from the label and the system lowered.
    hold_unresolved("one service, two systems", &tally, &cases, (15, 0), |_| {
        false
    });
    let named = |c: &Case| {
        let side = &c.description.services[0].side;
        side.uses
            .iter()
            .chain(&side.needs)
            .next()
            .cloned()
            .unwrap_or_default()
    };
    let by_needs = |c: &Case| !c.description.services[0].side.needs.is_empty();
    let inside = |c: &Case| {
        let (platform, block) = if c.description.system == "sa" {
            ("pa", "uart.a")
        } else {
            ("pb", "uart.b")
        };
        [platform, block].contains(&named(c).as_str())
    };
    let platform_named = |c: &Case| ["pa", "pb"].contains(&named(c).as_str());
    let task_serves = |c: &Case| {
        c.description.tasks[0]
            .side
            .uses
            .contains(&"log".to_string())
    };
    let serves = |c: &Case| task_serves(c) || !c.description.own.uses.is_empty();
    let names_any = |c: &Case| !named(c).is_empty();
    let conflict = |c: &Case| serves(c) && names_any(c) && !inside(c);
    let node = |c: &Case| serves(c) && inside(c);
    hold(
        "one service, two systems",
        &tally,
        (60, 45, 0, 15, 0),
        &[
            counted(SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE, &cases, conflict),
            counted("outside: named by a service", &cases, conflict),
            counted("outside: named by `uses`", &cases, |c| conflict(c) && !by_needs(c)),
            counted("outside: named by `needs`", &cases, |c| conflict(c) && by_needs(c)),
            counted("outside: its namer served by a task", &cases, |c| conflict(c) && task_serves(c)),
            counted("outside: its namer served by the system's own level alone", &cases, |c| {
                conflict(c) && !task_serves(c)
            }),
            counted("outside: a platform another system uses", &cases, |c| conflict(c) && platform_named(c)),
            counted("outside: a block a platform outside the hardware composes", &cases, |c| {
                conflict(c) && !platform_named(c)
            }),
            counted("named: a block a service names, bound for nothing, a node", &cases, |c| {
                node(c) && !platform_named(c)
            }),
            counted("named: a platform a service names, bound for nothing, a node", &cases, |c| {
                node(c) && platform_named(c)
            }),
            counted(
                "outside: a block outside the hardware named by a service or a policy the system does not serve, no \
                 refusal",
                &cases,
                |c| !serves(c) && names_any(c) && !inside(c),
            ),
            counted("named: hardware a service or a policy the system does not serve names, no node", &cases, |c| {
                !serves(c) && names_any(c)
            }),
            exactly("task: deadline-from written", 45),
        ],
    );
    // "a system the name leaves inside its hardware is not refused for it": the 5 descriptions whose both tasks use
    // `log` naming anything, one system refused and the other's plan holding the name, a node.
    let mut one_of_each = 0;
    for (pair, plans) in cases.chunks(2).zip(plans.chunks(2)) {
        if !(task_serves(&pair[0]) && task_serves(&pair[1]) && names_any(&pair[0])) {
            continue;
        }
        let (lowered, plan, refused) = match (&plans[0], &plans[1]) {
            (Some(plan), None) => (&pair[0], plan, &pair[1]),
            (None, Some(plan)) => (&pair[1], plan, &pair[0]),
            _ => panic!("{}: one system lowered and one refused", pair[0].label),
        };
        assert!(
            inside(lowered) && conflict(refused),
            "{}: the name inside the system lowered, outside the one refused",
            lowered.label
        );
        assert!(
            plan.lines()
                .any(|l| l.starts_with(&format!("(context {} ", named(lowered)))),
            "{}: the name a node\n{plan}",
            lowered.label
        );
        one_of_each += 1;
    }
    assert_eq!(one_of_each, 5);
}

#[test]
fn ranks_that_start_past_one_or_skip() {
    let cases = ranks();
    // Every three distinct spellings of 7, in every order; 4 kinds.
    assert_eq!(cases.len(), 7 * 6 * 5 * 4);
    let tally = run("ranks", &cases);
    // Lowered: every periodic and every `a`-sporadic system (2 × 210). Refused: each deadline from activation and each
    // set of alarms (2 × 210).
    // Each count read from the three spellings and the kind: no three of the values consecutive; a rank spelled
    // otherwise than in decimal wherever `+4`, `007`, `0x0A` or `1_000` is among them; past 32 bits wherever
    // `9223372036854775807` is; the order by value against the spellings' texts' order; the deadline refusal naming
    // the least rank's task, `a` declared first; the receivers in runtime order, unlike `a`, `b`, `c` but where the
    // ranks rise with them.
    let spellings = |c: &Case| -> Vec<&'static str> {
        c.description
            .tasks
            .iter()
            .map(|t| t.spelled_priority.expect("spelled"))
            .collect()
    };
    let values = |c: &Case| -> Vec<u128> { spellings(c).into_iter().map(integer_value).collect() };
    let kind = |c: &Case| c.label.splitn(4, ' ').nth(3).expect("a kind").to_string();
    let lowered = |c: &Case| ["periodic", "a sporadic"].contains(&kind(c).as_str());
    let rising = |c: &Case| values(c).windows(2).all(|w| w[0] < w[1]);
    hold(
        "ranks",
        &tally,
        (840, 420, 0, 0, 420),
        &[
            counted("rank: a rank past 1 first", &cases, |c| lowered(c) && !values(c).contains(&1)),
            counted("rank: ranks not consecutive", &cases, lowered),
            counted("rank: a rank spelled otherwise than its decimal `INT`", &cases, |c| {
                lowered(c) && spellings(c).iter().any(|s| integer_value(s).to_string() != *s)
            }),
            counted("rank: a rank past 32 bits", &cases, |c| {
                lowered(c) && values(c).iter().any(|v| *v > u128::from(u32::MAX))
            }),
            counted("rank: runtime order unlike the ranks' spellings' order", &cases, |c| {
                let mut by_value: Vec<(u128, &str)> = values(c).into_iter().zip(spellings(c)).collect();
                by_value.sort_unstable();
                let mut by_text = by_value.clone();
                by_text.sort_by_key(|(_, s)| *s);
                lowered(c) && by_value != by_text
            }),
            counted("refusal: a deadline measured from another event", &cases, |c| kind(c) == "deadline from activation"),
            counted(
                "deadline: the first in runtime order named, an earlier declared one measured from another event too",
                &cases,
                |c| kind(c) == "deadline from activation" && values(c)[0] != *values(c).iter().min().expect("three"),
            ),
            counted("receivers: three", &cases, |c| kind(c) == "alarms"),
            counted("receivers: an alarm programmed itself", &cases, |c| kind(c) == "alarms"),
            counted("receivers: runtime order unlike the tasks' text order", &cases, |c| {
                kind(c) == "alarms" && !rising(c)
            }),
            counted("trigger: every, a longer separation", &cases, |c| kind(c) == "a sporadic"),
            counted("trigger: its own source", &cases, |c| kind(c) == "a sporadic"),
            counted("trigger: its own source, no other reaching it", &cases, |c| kind(c) == "a sporadic"),
            exactly("task: deadline-from written", 420),
        ],
    );
}

/// Re-purposed by `M1.50` (`RTP-H23`). This held §1's refusal of two tasks of one name in its place before every later
/// refusal, naming the system and each name by its text (round 23, R23-D1; round 24, R24-R2). The bullet has left §1
/// for the language, whose check comes before the search and §1 alike: no order among refusals is left to hold and no
/// message of the lowering's to read, so the classes of both went with it. What stays is the language's verdict on
/// every such system, whatever else it writes, beside its twin, which it accepts and which fares by what else it writes
/// alone. `M1.42` (`RTP-H14`) moved one of those later refusals, a period of zero, to the language as well, so a twin
/// writing it is rejected too, by that rule alone.
#[test]
fn two_tasks_of_one_name_rejected_by_the_language_whatever_else_they_write() {
    let cases = one_name_two_tasks();
    // 5 name patterns, each with its twin; 4 binders; what else it writes, of 10: a period of zero, the language's; 7
    // later refusals of §1; none; and a name outside the hardware.
    assert_eq!(cases.len(), ONE_NAME.len() * 2 * 4 * 10);
    let tally = run("one name, two tasks", &cases);
    // Rejected, `invalid-description`, so no search, record, configuration checker or lowering reads one: every system
    // declaring two tasks of one name (5 × 4 × 10), whatever else it writes, and each twin writing a period of zero (5
    // × 4; model §4 rule 8). 200 + 20 = 220. Of the twins' other 5 × 4 × 9, none rejected, so that a system's names
    // alone are what the language refuses of them: unresolved `infeasible-configuration`, model §8 rule 7's conflict,
    // each writing `w`, which names `out.b`, outside the hardware (5 × 4), so no record of it reaches §1 or the
    // configuration checker; refused, each writing a later refusal of §1 (5 × 4 × 7); lowered, each writing none (5 ×
    // 4). 220 + 20 + 140 + 20 = 400. Each count read from the label's `one-name` or `distinct`, the binder and the
    // later refusal its description writes.
    let one_name = |c: &Case| c.label.starts_with("one-name ");
    let word = |c: &Case, key: &str| -> String {
        c.label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a choice")
            .to_string()
    };
    let later = |c: &Case| word(c, "later:");
    let binder = |c: &Case| word(c, "binder:");
    let alone = |l: &'static str| move |c: &Case| !one_name(c) && later(c) == l;
    assert_eq!(
        tally
            .verdicts
            .get("rejected invalid-description")
            .map(|v| v.0),
        Some(220),
        "the language refuses two tasks of one system carrying one name, and a period of zero: {:?}",
        tally.verdicts
    );
    hold_unresolved("one name, two tasks", &tally, &cases, (20, 0), |_| false);
    hold(
        "one name, two tasks",
        &tally,
        (400, 20, 220, 20, 140),
        &[
            counted(LANGUAGE_REJECTS_TWO_TASKS_OF_ONE_NAME, &cases, one_name),
            // A period of zero, in both of each pair of systems (5 × 2 × 4 = 40): the 20 declaring two tasks of one
            // name are rejected by both rules, the 20 twins by this one alone.
            counted(LANGUAGE_REJECTS_A_TASK_S_DURATION, &cases, |c| {
                later(c) == "zero"
            }),
            counted("language: a period of zero", &cases, |c| later(c) == "zero"),
            counted(
                SEARCH_REFUSES_A_NAME_OUTSIDE_THE_HARDWARE,
                &cases,
                alone("outside"),
            ),
            // The twins, each refused by its later refusal alone.
            counted(
                "refusal: a requester naming a system",
                &cases,
                alone("system"),
            ),
            counted(
                "naming: the lowered system itself named",
                &cases,
                alone("system"),
            ),
            counted(
                "refusal: hardware naming what is not hardware",
                &cases,
                alone("operand"),
            ),
            counted(
                "refusal: a duration past the arithmetic",
                &cases,
                alone("past"),
            ),
            counted(
                "refusal: a deadline measured from another event",
                &cases,
                alone("activation"),
            ),
            counted("receivers: one", &cases, |c| {
                alone("alarm")(c) || alone("alarm2")(c)
            }),
            counted("receivers: two", &cases, alone("alarms")),
            counted("multiplex: a channel with one user", &cases, |c| {
                alone("none")(c) && ["first", "second"].contains(&binder(c).as_str())
            }),
            counted("multiplex: multiplexed by two jobs", &cases, |c| {
                alone("none")(c) && binder(c) == "both"
            }),
            exactly("task: deadline-from written", 5 * 4),
        ],
    );
}

#[test]
fn a_service_and_a_policy_naming_themselves() {
    let cases = self_naming();
    // `s`: 3 names of itself, a channel or not, the source or not, 4 namers; `p`: 3, a channel or not, 4 namers, and
    // naming `s` or not.
    assert_eq!(cases.len(), (3 * 2 * 2 * 4) * (3 * 2 * 4 * 2));
    let (tally, plans) = run_keeping("self-naming", &cases, true);
    // Every system lowered: no refusal of §1 applies. Each count read from what `s` (0) and `p` (1) write and what
    // names them: `s` is served where a task or the system's own level names it, or `p` names it and is served (model
    // §8 rule 7's closure); a task reaches `s` naming it or naming `p` that names it; and, of a node taking a channel
    // of the timer, its users are the tasks reaching it, the source's route where it handles the source, and its own
    // route, the timer's, where `p` runs there — taking a channel too — and names `s` (§2.9: "another node's code there
    // calling it is"), a name of itself never one.
    let declaration = |c: &Case, i: usize| -> Service { c.description.services[i].clone() };
    let by = |c: &Case, i: usize| -> &'static str {
        let d = declaration(c, i);
        if d.side.uses.contains(&d.name) {
            "uses"
        } else if d.side.needs.contains(&d.name) {
            "needs"
        } else {
            "none"
        }
    };
    let writes =
        |c: &Case, i: usize, fact: &str| declaration(c, i).side.needs.iter().any(|n| n == fact);
    let names_s = |c: &Case| declaration(c, 1).side.uses.iter().any(|n| n == "s");
    let named_by = |c: &Case, i: usize, side: &Side| side.uses.contains(&declaration(c, i).name);
    let served_p = |c: &Case| {
        c.description.tasks.iter().any(|t| named_by(c, 1, &t.side))
            || named_by(c, 1, &c.description.own)
    };
    let served = |c: &Case, i: usize| {
        c.description.tasks.iter().any(|t| named_by(c, i, &t.side))
            || named_by(c, i, &c.description.own)
            || (i == 0 && names_s(c) && served_p(c))
    };
    let tasks_reaching = |c: &Case, i: usize| {
        c.description
            .tasks
            .iter()
            .filter(|t| {
                named_by(c, i, &t.side) || (i == 0 && names_s(c) && named_by(c, 1, &t.side))
            })
            .count()
    };
    let node_naming_itself = |c: &Case, i: usize| by(c, i) != "none" && served(c, i);
    let taking = |c: &Case, i: usize| node_naming_itself(c, i) && writes(c, i, "relative-delay");
    let called_on_its_route =
        |c: &Case, i: usize| i == 0 && names_s(c) && served_p(c) && writes(c, 1, "relative-delay");
    let users = |c: &Case, i: usize| {
        tasks_reaching(c, i)
            + usize::from(writes(c, i, INTERRUPT_FACT))
            + usize::from(called_on_its_route(c, i))
    };
    let either = |c: &Case, f: &dyn Fn(&Case, usize) -> bool| f(c, 0) || f(c, 1);
    hold(
        "self-naming",
        &tally,
        (2_304, 2_304, 0, 0, 0),
        &[
            counted("self: a service naming itself by `uses`, a node", &cases, |c| {
                by(c, 0) == "uses" && served(c, 0)
            }),
            counted("self: a service naming itself by `needs`, a node", &cases, |c| {
                by(c, 0) == "needs" && served(c, 0)
            }),
            counted("self: a policy naming itself by `uses`, a node", &cases, |c| {
                by(c, 1) == "uses" && served(c, 1)
            }),
            counted("self: a policy naming itself by `needs`, a node", &cases, |c| {
                by(c, 1) == "needs" && served(c, 1)
            }),
            counted(
                "self: a service or a policy naming itself, named by nothing else, no node",
                &cases,
                |c| either(c, &|c, i| by(c, i) != "none" && !served(c, i)),
            ),
            counted("self: a node naming itself, no edge from it to itself", &cases, |c| {
                either(c, &node_naming_itself)
            }),
            counted("self: a node naming itself, in no job", &cases, |c| {
                either(c, &|c, i| node_naming_itself(c, i) && tasks_reaching(c, i) == 0)
            }),
            counted("self: a node naming itself, run in a source's interrupt", &cases, |c| {
                node_naming_itself(c, 0) && writes(c, 0, INTERRUPT_FACT)
            }),
            counted(
                "self: a node naming itself on its own timer route, no user of itself there",
                &cases,
                |c| either(c, &|c, i| taking(c, i) && !called_on_its_route(c, i)),
            ),
            counted(
                "self: a node naming itself on its own timer route, another node there calling it a user",
                &cases,
                |c| taking(c, 0) && called_on_its_route(c, 0),
            ),
            counted("self: a node naming itself, taking a channel, with no user", &cases, |c| {
                either(c, &|c, i| taking(c, i) && users(c, i) == 0)
            }),
            counted("self: a node naming itself with one user, not multiplexed", &cases, |c| {
                either(c, &|c, i| taking(c, i) && users(c, i) == 1)
            }),
            counted("self: a node naming itself, multiplexed by its other users", &cases, |c| {
                either(c, &|c, i| taking(c, i) && users(c, i) >= 2)
            }),
        ],
    );
    // A name of itself gives nothing (rule 12, §4's `.2`): each system naming itself writes its twin's plan, the twin
    // naming itself not at all, but for the two digests, its program's and its record's.
    let at: BTreeMap<&str, usize> = cases
        .iter()
        .enumerate()
        .map(|(i, c)| (c.label.as_str(), i))
        .collect();
    let mut pairs = 0;
    let mut unlike = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        if by(case, 0) == "none" && by(case, 1) == "none" {
            continue;
        }
        let twin = case
            .label
            .replacen(&format!("s:{} ", by(case, 0)), "s:none ", 1)
            .replacen(&format!("p:{} ", by(case, 1)), "p:none ", 1);
        let (Some(plan), Some(twin_plan)) = (&plans[i], &plans[at[twin.as_str()]]) else {
            panic!("`{}` and its twin lowered", case.label);
        };
        pairs += 1;
        if common::without_digests(plan) != common::without_digests(twin_plan) {
            unlike.push(format!(
                "== {}\n{}-- its twin, `{twin}`, naming itself not at all\n{}",
                case.label,
                common::without_digests(plan),
                common::without_digests(twin_plan)
            ));
        }
    }
    assert_eq!(
        pairs,
        2_304 - 2_304 / 9,
        "every system naming itself has its twin"
    );
    assert!(
        unlike.is_empty(),
        "{} plans a name of itself changes:\n{}",
        unlike.len(),
        unlike.join("\n")
    );
}

#[test]
fn an_obliged_bound_written_exactly() {
    let cases = exactly_bounds();
    // 5 requesters, each writing one of 11.
    assert_eq!(cases.len(), 5 * 11);
    let tally = run("exactly", &cases);
    let writing = |c: &Case| {
        c.label
            .split_once(" writes ")
            .expect("a writing")
            .1
            .to_string()
    };
    let requester = |c: &Case| {
        c.label
            .split_once(" writes ")
            .expect("a requester")
            .0
            .to_string()
    };
    // The side reader's three refusals, each the search's answer (the adapters record §1, item 5): an `exactly` bound
    // beside a bound it contradicts, `invalid-description`; beside one past the arithmetic, `unsupported-profile`.
    let refused_by_search: BTreeMap<String, String> = cases
        .iter()
        .filter_map(|c| {
            let verdict = match writing(c).as_str() {
                "exactly 5 ms, at-most 3 ms" | "exactly 5 ms and 3 ms" => "InvalidDescription",
                "exactly 5 ms, at-most past" => "UnsupportedProfile",
                _ => return None,
            };
            Some((c.label.clone(), verdict.to_string()))
        })
        .collect();
    assert_eq!(
        tally.unresolved_systems, refused_by_search,
        "the search resolves no plan where the side reader refuses the bounds"
    );
    // Lowered: each of the four requesters a plan carries, writing one of the seven the search accepts within the
    // arithmetic. Refused: each requester's `exactly` past the arithmetic (5), and the system's own level's other
    // seven, the receiver of its instance's expiry (§1).
    let past = |c: &Case| writing(c) == "exactly past";
    let resolved = |c: &Case| !refused_by_search.contains_key(&c.label);
    let lowered = |c: &Case| resolved(c) && !past(c) && requester(c) != "sys";
    let written = |c: &Case, w: &[&str]| lowered(c) && w.contains(&writing(c).as_str());
    let with_exactly = [
        "exactly 5 ms",
        "exactly 5 ms, at-most 10 ms",
        "exactly 5 ms, at-most 5 ms",
        "exactly 5 ms and 5000 us",
        "exactly at the edge",
    ];
    let mut bounded = with_exactly.to_vec();
    bounded.push("at-most 5 ms");
    hold(
        "exactly",
        &tally,
        (55, 28, 0, 15, 12),
        &[
            counted("instance: an obliged bound written `exactly`, its value taken", &cases, |c| {
                written(c, &with_exactly)
            }),
            counted(
                "instance: an `exactly` bound beside a looser `at-most`, the `exactly`'s value taken",
                &cases,
                |c| written(c, &["exactly 5 ms, at-most 10 ms"]),
            ),
            counted("instance: an `exactly` bound beside an equal `at-most`", &cases, |c| {
                written(c, &["exactly 5 ms, at-most 5 ms"])
            }),
            counted("instance: one `exactly` value in two spellings", &cases, |c| {
                written(c, &["exactly 5 ms and 5000 us"])
            }),
            counted("exact: an obliged bound at the arithmetic's edge", &cases, |c| {
                written(c, &["exactly at the edge"])
            }),
            counted("instance: an obliged fact with its bound", &cases, |c| written(c, &bounded)),
            counted("instance: an obliged fact bare", &cases, |c| written(c, &["bare"])),
            counted("instance: a service's obliged bound", &cases, |c| {
                requester(c) == "dl" && written(c, &bounded)
            }),
            counted("instance: a policy's obliged bound", &cases, |c| {
                requester(c) == "pol" && written(c, &bounded)
            }),
            counted("instance: a platform's obliged bound", &cases, |c| {
                requester(c) == "soc" && written(c, &bounded)
            }),
            counted("instance: a block's obliged bound", &cases, |c| {
                requester(c) == "blk" && written(c, &bounded)
            }),
            counted("refusal: a duration past the arithmetic", &cases, past),
            counted("past: an obliged bound written `exactly`", &cases, past),
            counted("past: an obliged bound of the system's own level", &cases, |c| {
                past(c) && requester(c) == "sys"
            }),
            counted("past: an obliged bound of a service", &cases, |c| {
                past(c) && requester(c) == "dl"
            }),
            counted("past: an obliged bound of a policy", &cases, |c| {
                past(c) && requester(c) == "pol"
            }),
            counted("past: an obliged bound of a platform", &cases, |c| {
                past(c) && requester(c) == "soc"
            }),
            counted("past: an obliged bound of a block", &cases, |c| {
                past(c) && requester(c) == "blk"
            }),
            counted("receivers: the system's own level alone", &cases, |c| {
                resolved(c) && !past(c) && requester(c) == "sys"
            }),
            exactly("task: deadline-from written", 28),
        ],
    );
}

/// A plan's lines with its two digests written `…` (`common::without_digests`), but those `skip` takes.
fn lines_but(plan: &str, skip: impl Fn(&str) -> bool) -> Vec<String> {
    common::without_digests(plan)
        .lines()
        .filter(|line| !skip(line))
        .map(str::to_string)
        .collect()
}

#[test]
fn which_task_names_a_block_and_what_leaves_its_trigger_unknown() {
    let cases = which_task_names();
    // `t` bound to `blk` or not; 2 release kinds; what is named, of 4; `dev`'s separation, of 5; by which clause, of 3;
    // by which task, of 4.
    assert_eq!(
        cases.len(),
        2 * 2
            * WHICH_NAMED.len()
            * WHICH_SEPARATIONS.len()
            * WHICH_CLAUSES.len()
            * WHICH_NAMERS.len()
    );
    let (tally, plans) = run_keeping("which task names", &cases, true);
    // Every system lowered. Each count read from the label. A task naming `hnd`, the requester `dev`'s route names,
    // reaches the source (§2.2: "the external source whose route names the task or a requester it reaches"); naming
    // `dev`, `blk` or `soc` it reaches none — hardware reaches only what its own requirements are bound to (§2), and
    // `dev`'s route names `hnd` alone. A sporadic task reaching it is released as `dev`'s separation decides against
    // its own, 20 ms for `t` and 40 ms for `u`, and one reaching none is `(unknown unreached)`. What a task names is a
    // node, a member of its `(uses …)` bound to it or not: `blk` bound for nothing, a provider of `t` where `t` binds
    // it; `soc` bound for nothing; `dev` a provider of `hnd` alone; `hnd` bound for its own requirement.
    let word = |c: &Case, key: &str| -> String {
        c.label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a choice")
            .to_string()
    };
    let sporadic = |c: &Case| c.label.starts_with("sporadic ");
    let named = |c: &Case| {
        c.label
            .split(' ')
            .nth(1)
            .expect("what is named")
            .to_string()
    };
    let separation = |c: &Case| word(c, "dev:");
    let binds = |c: &Case| word(c, "t-binds-blk:") == "true";
    let clause = |c: &Case| word(c, "by:");
    let namer = |c: &Case| word(c, "named-by:");
    let naming = |c: &Case| namer(c) != "neither";
    let names_it = |c: &Case, task: &str| namer(c) == task || namer(c) == "both";
    let reaches = |c: &Case, task: &str| sporadic(c) && named(c) == "hnd" && names_it(c, task);
    let reaches_any = |c: &Case| reaches(c, "t") || reaches(c, "u");
    let block = |c: &Case| ["blk", "dev", "hnd"].contains(&named(c).as_str());
    hold(
        "which task names",
        &tally,
        (960, 960, 0, 0, 0),
        &[
            counted("uses: a block a task names, a member", &cases, |c| {
                naming(c) && block(c)
            }),
            counted("uses: a platform a task names, a member", &cases, |c| {
                naming(c) && named(c) == "soc"
            }),
            counted("uses: hardware a task names by `uses`, a member", &cases, |c| {
                naming(c) && clause(c) == "uses"
            }),
            counted("uses: hardware a task names by `needs`, a member", &cases, |c| {
                naming(c) && clause(c) != "uses"
            }),
            counted("uses: a node a task names twice, written once", &cases, |c| {
                naming(c) && clause(c) == "both"
            }),
            counted(
                "uses: hardware a task names and is not bound to, a member",
                &cases,
                |c| naming(c) && !(binds(c) && named(c) == "blk" && namer(c) == "t"),
            ),
            counted(
                "uses: hardware a task names and is bound to, a member",
                &cases,
                |c| binds(c) && named(c) == "blk" && names_it(c, "t"),
            ),
            exactly("node: a block", 960),
            counted("node: a platform", &cases, |c| {
                naming(c) && named(c) == "soc"
            }),
            counted("named: a block a task names, bound for nothing, a node", &cases, |c| {
                naming(c) && named(c) == "blk" && !binds(c)
            }),
            counted("named: a platform a task names, bound for nothing, a node", &cases, |c| {
                naming(c) && named(c) == "soc"
            }),
            counted(
                "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                &cases,
                |c| naming(c) && (named(c) == "dev" || (binds(c) && named(c) == "blk" && namer(c) == "u")),
            ),
            counted("trigger: `(unknown unreached)`, no source reaching it", &cases, |c| {
                sporadic(c) && !(reaches(c, "t") && reaches(c, "u"))
            }),
            counted("trigger: `(unknown (faster ITEM))`, a shorter separation", &cases, |c| {
                (reaches(c, "t") && separation(c) == "Stated(10)")
                    || (reaches(c, "u") && ["Stated(10)", "Stated(30)"].contains(&separation(c).as_str()))
            }),
            counted("trigger: every, a longer separation", &cases, |c| {
                reaches(c, "t") && ["Stated(30)", "Stated(40)"].contains(&separation(c).as_str())
            }),
            counted("trigger: every, equal separations", &cases, |c| {
                reaches(c, "u") && separation(c) == "Stated(40)"
            }),
            counted("trigger: `(unknown (unstated ITEM))`, no separation stated", &cases, |c| {
                reaches_any(c) && separation(c) == "Unstated"
            }),
            counted("trigger: `(unknown (absent ITEM))`, the separation absent", &cases, |c| {
                reaches_any(c) && separation(c) == "Absent"
            }),
            counted("trigger: a source a block or a platform handles", &cases, reaches_any),
            exactly("task: deadline-from written", 960),
        ],
    );
    // Each system naming it from `t` beside its twin naming it from `u` (R24-D1, R24-R1): their plans differ, each task
    // line saying which node its job calls and what leaves its trigger unknown, and nothing differs but those lines
    // and, where one twin's task is released every arrival and the other's not, that task's owed trigger.
    let task_or_trigger = |line: &str| {
        line.starts_with("(task ")
            || (line.starts_with("(owed (task ") && line.ends_with(" trigger (by description))"))
    };
    let released_by = |plan: &str| -> Vec<String> {
        plan.lines()
            .filter(|l| l.starts_with("(task "))
            .map(|l| {
                l.split(" (released-by ")
                    .nth(1)
                    .and_then(|r| r.rsplit_once(" (uses"))
                    .expect("a trigger and a `uses` item")
                    .0
                    .to_string()
            })
            .collect()
    };
    let (mut compared, mut task_lines_alone, mut triggers_differ, mut causes_alone) = (0, 0, 0, 0);
    let mut unlike = Vec::new();
    for (pair, plans) in cases
        .chunks(WHICH_NAMERS.len())
        .zip(plans.chunks(WHICH_NAMERS.len()))
    {
        let (from_t, from_u) = (&pair[0], &pair[1]);
        assert_eq!(
            from_t.label.strip_suffix("named-by:t"),
            from_u.label.strip_suffix("named-by:u"),
            "a system and its twin"
        );
        let (Some(a), Some(b)) = (&plans[0], &plans[1]) else {
            panic!("`{}` and its twin lowered", from_t.label);
        };
        compared += 1;
        let (a_lines, b_lines) = (lines_but(a, |_| false), lines_but(b, |_| false));
        let only_a: Vec<&String> = a_lines.iter().filter(|l| !b_lines.contains(l)).collect();
        let only_b: Vec<&String> = b_lines.iter().filter(|l| !a_lines.contains(l)).collect();
        if only_a.is_empty() && only_b.is_empty() {
            unlike.push(format!(
                "== {}: one plan for both twins, which task names `{}` unsaid\n{a}",
                from_t.label,
                named(from_t)
            ));
            continue;
        }
        if !only_a.iter().chain(&only_b).all(|l| task_or_trigger(l)) {
            unlike.push(format!(
                "== {}: its twin naming it from `u` differs beyond its task lines and owed triggers\n\
                 -- from `t`\n{}\n-- from `u`\n{}",
                from_t.label,
                only_a.iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n"),
                only_b.iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n")
            ));
            continue;
        }
        if only_a
            .iter()
            .chain(&only_b)
            .all(|l| l.starts_with("(task "))
        {
            task_lines_alone += 1;
        }
        let (a_triggers, b_triggers) = (released_by(a), released_by(b));
        if a_triggers != b_triggers {
            triggers_differ += 1;
            // Every trigger unknown in both: before round 24, `unknown` alone in each, so the cause alone tells them
            // apart.
            if a_triggers
                .iter()
                .chain(&b_triggers)
                .all(|trigger| unknown_trigger(trigger))
            {
                causes_alone += 1;
            }
        }
    }
    assert!(unlike.is_empty(), "{}", unlike.join("\n"));
    // 2 bindings of `t` × 2 kinds × 4 named × 5 separations × 3 clauses. Differing beyond the task lines: the 2 × 6
    // sporadic pairs naming `hnd` before 30 ms or 40 ms, where `t` reaching it is released every arrival and `u` not,
    // or the other way. The triggers differ in the 2 × 15 sporadic pairs naming `hnd`; in 2 × 9, every trigger unknown,
    // the causes alone tell them apart — a source faster than both, one stating none, and one declaring it absent.
    assert_eq!(compared, 2 * 2 * 4 * 5 * 3);
    assert_eq!(task_lines_alone, compared - 2 * 6);
    assert_eq!(triggers_differ, 2 * 5 * 3);
    assert_eq!(causes_alone, 2 * 3 * 3);
}

#[test]
fn each_node_kind_written_with_its_node() {
    let cases = node_kinds();
    // What `k` requires, of 3; what names it, of 4; by `uses` or `needs`; and its kind, of 4.
    assert_eq!(
        cases.len(),
        K_REQUIRES.len() * K_NAMERS.len() * 2 * K_KINDS.len()
    );
    let (tally, plans) = run_keeping("node kinds", &cases, true);
    // Every system lowered. Each count read from the label: `k` a node but where it is a block or a platform requiring
    // nothing that the system's own level alone names (§2: a block or a platform is a node as a provider, bound for
    // requirements of its own, or named by a served task, service or policy — "the system's own level is no such
    // namer"); `timer.d` or `dev` a node where `k` is bound to it; the namer `s` or `p` a node where it names `k`.
    let word = |c: &Case, key: &str| -> String {
        c.label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a choice")
            .to_string()
    };
    let kind = |c: &Case| word(c, "kind:");
    let requires = |c: &Case| word(c, "requires:");
    let namer = |c: &Case| word(c, "named-by:");
    let hardware = |c: &Case| ["block", "platform"].contains(&kind(c).as_str());
    let k_node = |c: &Case| !(hardware(c) && requires(c) == "nothing" && namer(c) == "sys");
    hold(
        "node kinds",
        &tally,
        (96, 96, 0, 0, 0),
        &[
            counted("node: a block", &cases, |c| {
                (kind(c) == "block" && k_node(c)) || requires(c) != "nothing"
            }),
            counted("node: a platform", &cases, |c| {
                kind(c) == "platform" && k_node(c)
            }),
            counted("node: a service", &cases, |c| {
                kind(c) == "service" || namer(c) == "s"
            }),
            counted("node: a policy", &cases, |c| {
                kind(c) == "policy" || namer(c) == "p"
            }),
            counted("uses: a block a task names, a member", &cases, |c| {
                namer(c) == "t" && kind(c) == "block"
            }),
            counted("uses: a platform a task names, a member", &cases, |c| {
                namer(c) == "t" && kind(c) == "platform"
            }),
            exactly("task: deadline-from written", 96),
        ],
    );
    // The four kinds of one requirement, named alike: one plan but for `k`'s `(node …)` line, which says its kind
    // (R24-D2) — but where the system's own level alone names `k` requiring nothing, a block or a platform no node and
    // a service or a policy one.
    let mut groups = 0;
    let mut unlike = Vec::new();
    for (group, plans) in cases.chunks(K_KINDS.len()).zip(plans.chunks(K_KINDS.len())) {
        let plans: Vec<&String> = plans
            .iter()
            .map(|p| p.as_ref().expect("every system lowered"))
            .collect();
        let node_lines: Vec<Vec<String>> = plans
            .iter()
            .map(|plan| {
                plan.lines()
                    .filter(|l| l.starts_with("(node k "))
                    .map(str::to_string)
                    .collect()
            })
            .collect();
        let rest: Vec<Vec<String>> = plans
            .iter()
            .map(|plan| lines_but(plan, |l| l.starts_with("(node k ")))
            .collect();
        groups += 1;
        let label = group[0]
            .label
            .strip_suffix("kind:block")
            .expect("block first");
        let kinds: Vec<String> = group.iter().map(|c| word(c, "kind:")).collect();
        assert_eq!(kinds, K_KINDS, "{label}: the four kinds in order");
        if label.starts_with("requires:nothing named-by:sys ") {
            if !(node_lines[0].is_empty()
                && node_lines[1].is_empty()
                && node_lines[2] == ["(node k service)"]
                && node_lines[3] == ["(node k policy)"]
                && rest[0] == rest[1]
                && rest[2] == rest[3])
            {
                unlike.push(format!("== {label}: {node_lines:?}"));
            }
            continue;
        }
        let expected: Vec<Vec<String>> = K_KINDS
            .iter()
            .map(|k| vec![format!("(node k {k})")])
            .collect();
        if node_lines != expected || rest.iter().any(|r| *r != rest[0]) {
            unlike.push(format!(
                "== {label}: the kinds' plans differ beyond `k`'s node line, or it says no kind\n{}",
                plans
                    .iter()
                    .map(|p| common::without_digests(p))
                    .collect::<Vec<_>>()
                    .join("-- the next kind\n")
            ));
        }
    }
    assert!(unlike.is_empty(), "{}", unlike.join("\n"));
    assert_eq!(groups, 3 * 4 * 2);
}

/// Each task's trigger in a plan, `(released-by TRIGGER)`'s `TRIGGER`, by the task's name.
fn triggers_of(plan: &str) -> BTreeMap<String, String> {
    plan.lines()
        .filter(|line| line.starts_with("(task "))
        .map(|line| {
            let name = line
                .split(" (task ")
                .nth(1)
                .and_then(|rest| rest.split_once(')'))
                .and_then(|(identity, _)| identity.split(' ').nth(1))
                .expect("a task's identity");
            let trigger = line
                .split(" (released-by ")
                .nth(1)
                .and_then(|rest| rest.rsplit_once(" (uses"))
                .expect("a trigger and a `uses` item")
                .0;
            (name.to_string(), trigger.to_string())
        })
        .collect()
}

#[test]
fn the_task_s_own_source_written_apart_from_the_routes() {
    let cases = own_causes();
    // `dev.y` none, or of 6 separations, after `dev.x` or before it; `dev.x` of 6; `u`'s reach of 3; `t`'s of 4.
    assert_eq!(
        cases.len(),
        Y_SOURCES.len() * SEPARATIONS.len() * U_REACH.len() * T_REACH.len()
    );
    let (tally, kept) = run_kept("own causes", &cases, true);
    // Each class counted from the label and the records: where the search writes a plan, the source it binds each task
    // binding `interrupt-source` itself to ([`own_causes_classes`]); and, where the unamended rules gave a record of
    // their own, the source that record binds each to ([`own_causes_rule_16`]): the assignment is input.
    let fixed: Vec<BTreeSet<&str>> = cases
        .iter()
        .zip(&kept)
        .map(|(case, kept)| {
            let mut fixed = kept.plan.as_deref().map_or_else(BTreeSet::new, |plan| {
                own_causes_classes(
                    &case.label,
                    own_source_bound(plan, "t").as_deref(),
                    own_source_bound(plan, "u").as_deref(),
                )
            });
            fixed.extend(own_causes_rule_16(
                &case.label,
                kept.plan.is_some(),
                kept.unamended
                    .as_deref()
                    .map(|record| (own_source_bound(record, "t"), own_source_bound(record, "u"))),
            ));
            fixed
        })
        .collect();
    let count = |class: &'static str| fixed.iter().filter(|f| f.contains(class)).count();
    let fixed_count = |class: &'static str| exactly(class, count(class));
    // Refused by the search where model §8 rule 16 leaves a task bound itself no source (rules 16 and 6): with `dev.x`
    // alone, 28 of its 72 systems; with `dev.y` declared, of the 36 pairs of separations in each order, the 9 where
    // neither admits `t`'s 20 ms, 8 of their 12 choices of `t` and `u`, and the 7 where one admits `t` and neither `u`'s
    // 40 ms, the 4 where `u` binds itself — 200.
    let unresolved = count(RULE_16_SEARCH_CONFLICT) + count(RULE_16_SEARCH_UNKNOWN);
    assert_eq!(unresolved, 28 + 2 * (9 * 8 + 7 * 4));
    hold(
        "own causes",
        &tally,
        (936, 936 - 228, 0, 228, 0),
        &[
            fixed_count(RULE_16_SEARCH_CONFLICT),
            fixed_count(RULE_16_SEARCH_UNKNOWN),
            fixed_count(RULE_16_RECORD),
            fixed_count(RULE_16_UNAMENDED),
            fixed_count(RULE_16_SHORTER),
            fixed_count(RULE_16_ABSENT),
            fixed_count(RULE_16_UNSTATED),
            fixed_count(RULE_16_CONFLICT_BEFORE_UNKNOWN),
            fixed_count(RULE_16_ONE_OF_TWO),
            fixed_count(RULE_16_REBOUND),
            fixed_count("trigger: every, equal separations"),
            fixed_count("trigger: every, a longer separation"),
            fixed_count("trigger: `(unknown unreached)`, no source reaching it"),
            fixed_count("trigger: `(unknown (several ITEM …))`, several sources"),
            fixed_count("trigger: several sources, by their texts unlike their declaration order"),
            fixed_count("trigger: `(unknown (unstated ITEM))`, no separation stated"),
            fixed_count("trigger: `(unknown (absent ITEM))`, the separation absent"),
            fixed_count("trigger: `(unknown (faster ITEM))`, a shorter separation"),
            fixed_count("trigger: its own source"),
            fixed_count("trigger: its own source, no other reaching it"),
            fixed_count("trigger: its own source, beside another reaching it"),
            fixed_count(
                "trigger: its own source releasing it every, beside one that alone would leave it unknown",
            ),
            fixed_count(
                "trigger: its own source releasing it every, beside another that alone would release it every from \
                 itself",
            ),
            fixed_count("trigger: its own source, reached through a requester its route names too"),
            fixed_count("trigger: binding none, beside a task bound to its own source"),
            fixed_count("trigger: binding none, reaching the handler of another task's own source"),
            fixed_count("trigger: two tasks bound themselves to one source"),
            fixed_count("rank: two stated separations tied"),
            exactly("task: deadline-from written", 936 - 228),
        ],
    );
    // The five causes the routes leave reached, each by a class naming it; the task's own source's three stated
    // unreachable — §2.2: "no record the generator accepts binds a task's own source the analysis does not admit" — no
    // system reaching one, which `hold` fails in every universe, and the record that gave each before rule 16 landed
    // refused by the checker and the lowering instead (R29-R2).
    let reached = |cause: &str| {
        tally.classes.iter().any(|(class, count)| {
            *count > 0
                && (class.contains(&format!("`(unknown ({cause} "))
                    || class.contains(&format!("`(unknown {cause})`")))
        })
    };
    for cause in &CAUSES[..5] {
        assert!(
            reached(cause),
            "own causes: no system's trigger is `(unknown …)` of the cause `{cause}`"
        );
    }
    for (cause, (unreachable, refused)) in
        CAUSES[5..].iter().zip(OWN_CAUSES_UNREACHABLE.iter().zip([
            RULE_16_UNSTATED,
            RULE_16_ABSENT,
            RULE_16_SHORTER,
        ]))
    {
        assert!(
            unreachable.starts_with("unreachable: ")
                && unreachable.contains(&format!("`(unknown ({cause} ITEM))`")),
            "`{cause}` stated unreachable by its class: {unreachable}"
        );
        assert!(
            !reached(cause),
            "own causes: a plan's trigger is `(unknown ({cause} …))`"
        );
        assert!(
            tally.classes.get(refused).copied().unwrap_or(0) > 0,
            "own causes: no record the unamended rules gave of `{cause}`'s input refused: {refused}"
        );
    }
    // With `dev.x` alone the construction fixes every binding: `t`, bound itself alone or reaching `evt.x` too, beside
    // `u` bound itself, both to `dev.x` stating 20 ms — two systems: the search refuses each, `u` released less often
    // than `dev.x` arrives (`unsupported-profile`), and the record the unamended rules gave names `u` alone.
    let alone = |label: &str| label.ends_with(" dev.y:none first:dev.x");
    assert_eq!(
        cases
            .iter()
            .zip(&fixed)
            .filter(|(case, f)| {
                alone(&case.label)
                    && f.contains(RULE_16_ONE_OF_TWO)
                    && f.contains(RULE_16_SEARCH_CONFLICT)
            })
            .count(),
        2
    );
    // Each system beside its twins (R28-D2, R28-R7), plans the reading has already held line by line, read again since
    // rule 16 landed. With `dev.x` alone: `t` bound itself beside its twin reaching `dev.x` only through `evt.x` — one
    // trigger where both are lowered, every arrival releasing it; and where the twin's trigger is the routes'
    // `(unknown (CAUSE …))`, the system bound itself refused by the search, the cause's code — `faster` and `absent`
    // `unsupported-profile`, `unstated` `missing-fact`: the task's own `(needs interrupt-source)` "repairs a source the
    // routes leave and not one it binds itself" (§2.2), and rule 16 now refuses that one. And `t` bound itself and
    // reaching `dev.x` through `evt.x` too beside its twin bound itself alone, one trigger. With `dev.y` declared, `t`
    // bound itself: to `dev.x`, while the handler it uses reaches `dev.y`, beside its twin with `dev.x` alone where that
    // twin is lowered, its own source deciding alone, one trigger; or to `dev.y`, whose handler it uses, beside its twin
    // binding none, which that handler alone reaches, one trigger. `u`'s trigger is its twin's wherever both bind the
    // same source, or none.
    let plan_of: BTreeMap<&str, Option<&str>> = cases
        .iter()
        .zip(&kept)
        .map(|(case, kept)| (case.label.as_str(), kept.plan.as_deref()))
        .collect();
    let mut unlike = Vec::new();
    let mut twin = |label: &str, other: String, what: &str| -> bool {
        let (Some(plan), Some(theirs_plan)) = (plan_of[label], plan_of[other.as_str()]) else {
            return false;
        };
        let (mine, theirs) = (triggers_of(plan), triggers_of(theirs_plan));
        let u_alike = own_source_bound(plan, "u") == own_source_bound(theirs_plan, "u");
        if mine["t"] != theirs["t"] || (u_alike && mine["u"] != theirs["u"]) {
            unlike.push(format!(
                "== {label}, beside {other} ({what}): `t` {} and `u` {}, its twin's {} and {}",
                mine["t"], mine["u"], theirs["t"], theirs["u"]
            ));
        }
        true
    };
    let mut refusals_unlike = Vec::new();
    let (mut by_handler, mut through_both) = (0, 0);
    let (mut refused_by_rule_16, mut faster) = (0, 0);
    let (mut beside_another, mut by_its_handler, mut twin_refused) = (0, 0, 0);
    for case in &cases {
        let label = case.label.as_str();
        if alone(label) && label.contains(" t:own ") {
            let other = label.replace(" t:own ", " t:handler ");
            if twin(
                label,
                other.clone(),
                "reaching `dev.x` only through `evt.x`",
            ) {
                by_handler += 1;
            } else if let (None, Some(theirs)) = (plan_of[label], plan_of[other.as_str()]) {
                // `(released-by TRIGGER)`'s text, its closing parenthesis read with it.
                let routes_leave = &triggers_of(theirs)["t"];
                let code = match routes_leave.strip_suffix(')').unwrap_or(routes_leave) {
                    "(unknown (faster (source dev.x)))" => "UnsupportedProfile",
                    "(unknown (absent (source dev.x)))" => "UnsupportedProfile",
                    "(unknown (unstated (source dev.x)))" => "MissingFact",
                    other => panic!("{label}: refused, its twin's `t` released by {other}"),
                };
                if tally.unresolved_systems.get(label).map(String::as_str) != Some(code) {
                    refusals_unlike.push(format!(
                        "== {label}: the search gives {:?}, its twin's `t` {routes_leave}, rule 16 `{code}`",
                        tally.unresolved_systems.get(label)
                    ));
                }
                refused_by_rule_16 += 1;
                faster += usize::from(routes_leave.starts_with("(unknown (faster "));
            }
        }
        if alone(label) && label.contains(" t:both ") {
            let other = label.replace(" t:both ", " t:own ");
            assert_eq!(
                plan_of[label].is_some(),
                plan_of[other.as_str()].is_some(),
                "{label}: lowered as its twin bound itself alone is"
            );
            through_both += usize::from(twin(label, other, "bound itself alone"));
        }
        if !alone(label) && label.contains(" t:own ") {
            let Some(plan) = plan_of[label] else {
                continue;
            };
            match own_source_bound(plan, "t").as_deref() {
                Some("dev.x") => {
                    let (head, _) = label.split_once(" dev.y:").expect("`dev.y`");
                    if twin(
                        label,
                        format!("{head} dev.y:none first:dev.x"),
                        "`dev.x` alone",
                    ) {
                        beside_another += 1;
                    } else {
                        twin_refused += 1;
                    }
                }
                Some("dev.y") => {
                    assert!(
                        twin(
                            label,
                            label.replace(" t:own ", " t:none "),
                            "reaching `dev.y` only through `evt.y`"
                        ),
                        "{label}: its twin binding none lowered"
                    );
                    by_its_handler += 1;
                }
                other => panic!("{label}: `t` bound itself to {other:?}"),
            }
        }
    }
    unlike.extend(refusals_unlike);
    assert!(unlike.is_empty(), "{}", unlike.join("\n"));
    // With `dev.x` alone, of its 6 separations × 3 of `u`: lowered beside the twin where `dev.x` admits `t` — 20 ms with
    // `u` binding none itself, 40 and 80 ms with any `u`, 8 systems; refused where it states 10 ms, none or declares it
    // absent, `u` binding none itself, 6, the 2 of 10 ms `(faster …)` in the twin. With `dev.y` declared, the systems of
    // `t` bound itself the search lowers — of the 36 pairs of separations in each order, the 20 where one admits `u`'s
    // 40 ms, every `u`, and the 7 where one admits `t` and none `u`, `u` binding none itself — beside one twin or the
    // other as the record binds it, some to `dev.x`, the twin with `dev.x` alone refused where `u` binds itself and
    // `dev.x` does not admit it.
    assert_eq!(by_handler, 2 + 3 + 3);
    assert_eq!(through_both, 2 + 3 + 3);
    assert_eq!(refused_by_rule_16, 3 * 2);
    assert_eq!(faster, 2);
    assert_eq!(
        beside_another + by_its_handler + twin_refused,
        2 * (20 * 3 + 7 * 2)
    );
    assert!(
        beside_another > 0 && by_its_handler > 0,
        "no task bound itself to a source while the handler it uses reaches another, or to that source"
    );
    eprintln!(
        "own causes: `t` bound itself beside `dev.y`: to `dev.x` in {} systems ({twin_refused} beside a twin the search \
         refuses), to `dev.y` in {by_its_handler}",
        beside_another + twin_refused
    );
}

/// The classes of model §8 rule 16 the own-causes universe's construction fixes, from its label, whether the search
/// wrote a plan, and, where the record the unamended rules gave binds a task's own source the rule does not admit, the
/// source it binds each task binding `interrupt-source` itself to — `t` at 20 ms and `u` at 40 ms, each may take `dev.x`
/// and, where it is declared, `dev.y`: the search refusing the system where a task binding itself has no source stating
/// at least its own separation, `unsupported-profile` where some such task's every source is a conflict, else
/// `missing-fact` (rules 16 and 6); and the records binding each such task to each source, every assignment of them:
/// each source the rule does not admit for a task, by its separation; the conflict before the unknown where one task
/// may take a source refused as a conflict and the other one stating none; two tasks bound to one source, one admitted;
/// and the unamended rules' record among them.
fn own_causes_rule_16(
    label: &str,
    lowered: bool,
    unamended: Option<(Option<String>, Option<String>)>,
) -> BTreeSet<&'static str> {
    let word = |key: &str| {
        label
            .split(' ')
            .find_map(|w| w.strip_prefix(key))
            .expect("a parameter")
    };
    let (x, t, u, y) = (word("dev.x:"), word("t:"), word("u:"), word("dev.y:"));
    let y = (y != "none").then_some(y);
    let candidates: Vec<&str> = std::iter::once(x).chain(y).collect();
    let binding: Vec<u64> = [(matches!(t, "own" | "both"), 20), (u == "own", 40)]
        .iter()
        .filter(|(bound_itself, _)| *bound_itself)
        .map(|(_, ms)| *ms)
        .collect();
    let mut out = BTreeSet::new();
    let mut refused = None;
    for &ms in &binding {
        if candidates.iter().any(|s| admits(s, ms)) {
            continue;
        }
        if candidates.iter().all(|s| *s != "Unstated") {
            refused = Some(RULE_16_SEARCH_CONFLICT);
        } else {
            refused.get_or_insert(RULE_16_SEARCH_UNKNOWN);
        }
    }
    out.extend(refused);
    for &ms in &binding {
        for separation in &candidates {
            if !admits(separation, ms) {
                out.extend([RULE_16_RECORD, rule_16_cause(separation)]);
            }
        }
    }
    if let [first, second] = binding.as_slice() {
        let conflict = |ms: u64| {
            candidates
                .iter()
                .any(|s| !admits(s, ms) && *s != "Unstated")
        };
        if candidates.contains(&"Unstated") && (conflict(*first) || conflict(*second)) {
            out.insert(RULE_16_CONFLICT_BEFORE_UNKNOWN);
        }
        if candidates
            .iter()
            .any(|s| admits(s, *first) != admits(s, *second))
        {
            out.insert(RULE_16_ONE_OF_TWO);
        }
    }
    if let Some((t_source, u_source)) = unamended {
        let separation = |source: &str| match source {
            "dev.x" => x,
            "dev.y" => y.expect("`dev.y` declared"),
            other => panic!("no source `{other}` in the own-causes universe"),
        };
        assert!(
            [(&t_source, 20), (&u_source, 40)]
                .iter()
                .any(|(source, ms)| source
                    .as_deref()
                    .is_some_and(|s| !admits(separation(s), *ms))),
            "{label}: the unamended rules' record kept, binding a source rule 16 does not admit"
        );
        out.insert(RULE_16_UNAMENDED);
        if lowered {
            out.insert(RULE_16_REBOUND);
        }
    }
    out
}

#[test]
fn a_task_s_own_source_at_the_arithmetic_s_edge() {
    let cases = own_source_at_the_edge();
    // Each of 3 separations of `t`, each of 4 of `dev.x`, by `t`'s first.
    assert_eq!(cases.len(), EDGE_TASKS.len() * EDGE_SOURCES.len());
    let tally = run("own source at the edge", &cases);
    // Derived by hand from the times, `dev.x`'s against `t`'s (model §8 rule 16): for 20 ms, only 20 ms is as far apart;
    // for `WIDE`, 20 ms, `WIDE` itself and `WIDE_LONGER`; for `WIDE_LONGER`, 20 ms and itself, `WIDE` shorter; and
    // `PAST` for none, a comparison the exact arithmetic cannot decide. `WIDE` and `WIDE_LONGER` are compared past 128
    // bits wherever they meet.
    const ADMITTED: [[bool; 4]; 3] = [
        [true, false, false, false],
        [true, true, true, false],
        [true, false, true, false],
    ];
    let at = |c: &Case| {
        let i = cases
            .iter()
            .position(|d| d.label == c.label)
            .expect("a case");
        (i / EDGE_SOURCES.len(), i % EDGE_SOURCES.len())
    };
    let admitted = |c: &Case| {
        let (t, x) = at(c);
        ADMITTED[t][x]
    };
    let refused = |c: &Case| !admitted(c);
    hold(
        "own source at the edge",
        &tally,
        (12, 6, 0, 6, 0),
        &[
            counted(RULE_16_SEARCH_CONFLICT, &cases, refused),
            counted(RULE_16_RECORD, &cases, refused),
            counted(RULE_16_UNAMENDED, &cases, refused),
            counted(RULE_16_UNDECIDABLE, &cases, |c| at(c).1 == 3),
            counted(RULE_16_SHORTER, &cases, |c| refused(c) && at(c).1 != 3),
            counted("trigger: its own source", &cases, admitted),
            counted(
                "trigger: its own source, no other reaching it",
                &cases,
                admitted,
            ),
            counted("trigger: every, equal separations", &cases, |c| {
                admitted(c) && at(c).0 == at(c).1
            }),
            counted("trigger: every, a longer separation", &cases, |c| {
                admitted(c) && at(c).0 != at(c).1
            }),
            counted(
                "exact: a source's and a task's separations compared past 128 bits",
                &cases,
                |c| admitted(c) && at(c).0 > 0 && at(c).1 > 0,
            ),
            exactly("task: deadline-from written", 6),
        ],
    );
}

#[test]
fn the_reading_s_arithmetic_is_exact() {
    // The reading's seconds, held to values derived by hand from §2.2's rule — "one reduced rational number of
    // seconds ... `N` and `D` lie within ... signed 128-bit integers" — so that a fault of the reading's arithmetic shows
    // here, never as a verdict on the code.
    let seconds = |spelling: &str| Ratio::of(spelling).map(Ratio::text);
    assert_eq!(seconds("20 ms").as_deref(), Some("1/50 s"));
    assert_eq!(seconds("1500 us").as_deref(), Some("3/2000 s"));
    assert_eq!(seconds("0 ms").as_deref(), Some("0 s"));
    assert_eq!(
        seconds(WIDE).as_deref(),
        Some("9223372036854775807/100000000000000000000000000000000000000 s")
    );
    assert_eq!(seconds(WIDE_IN_MS), seconds(WIDE));
    assert_eq!(
        seconds(WIDE_LONGER).as_deref(),
        Some("922337203685477581/10000000000000000000000000000000000000 s")
    );
    assert_eq!(
        seconds(REDUCED).as_deref(),
        Some("1/100000000000000000000000000000000000000 s")
    );
    // 5^13/10^47 = 1/(2^13·10^34); 5^12/10^47 = 1/(2^12·10^35), past `i128::MAX` = 170141183460469231731687303715884105727.
    assert_eq!(
        seconds(TIGHT_WITHIN).as_deref(),
        Some("1/81920000000000000000000000000000000000 s")
    );
    assert_eq!(seconds(TIGHT_PAST), None);
    assert_eq!(seconds(PAST), None);
    // Exact comparison, where cross-multiplication in 128 bits overflows.
    let of = |spelling: &str| Ratio::of(spelling).expect("within");
    assert!(of(WIDE) < of(WIDE_LONGER));
    assert!(past_128_bits(of(WIDE), of(WIDE_LONGER)));
    // 1/10^38 < 1/(8.192·10^37) < 9223372036854775807/10^38.
    assert!(of(REDUCED) < of(TIGHT_WITHIN) && of(TIGHT_WITHIN) < of(WIDE));
    assert!(of(WIDE_LONGER) < of("20 ms"));
}

#[test]
fn the_message_readers_keep_the_order_a_message_gives() {
    // The checker's readers of a refusal's prose, held to messages written here in words of their own, so that a
    // fault of a reader shows here, never as a verdict on the code: each returns what a message names in the order it
    // names it, which the checker then holds to §1's order — so a message sorted by its words, `y.b` before `y`, is
    // read as written and differs from the reading.
    let declared: BTreeSet<String> = ["m", "m.s", "y", "y.b", "y!", "(task sys t!)"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let pair = |namer: &str, named: &str| (namer.to_string(), named.to_string());
    let pairs = [
        pair("(task sys t!)", "y"),
        pair("m.s", "y.b"),
        pair("m.s", "y"),
        pair("m.s", "y!"),
    ];
    // Read by what it names and in what order, whatever its words (round 20, R20-R7): a namer before each name, or
    // once before several.
    for message in [
        "refused: `(task sys t!)` names the system `y`; `m.s` names `y.b`, `y` and `y!`: no system is used",
        "`(task sys t!)` uses `y`, and `m.s` needs `y.b`, `y`, `y!`: a system is no thing to use",
        "of `(task sys t!)`: `y`; of `m.s`: `y.b`; of `m.s`: `y`; of `m.s`: `y!`",
    ] {
        assert!(
            names_pairs(&names_mentioned(message, &declared), &pairs),
            "{message}"
        );
    }
    // Misordered, short of a pair, a pair's two names the other way about, or naming another: each a finding.
    for message in [
        "`(task sys t!)` names `y`; `m.s` names `y`, `y.b` and `y!`",
        "`(task sys t!)` names `y`; `m.s` names `y.b` and `y`",
        "`y` is named by `(task sys t!)`; `m.s` names `y.b`, `y` and `y!`",
        "`(task sys t!)` names `y`; `m.s` names `y.b`, `y` and `y!`; `m` names `y`",
    ] {
        assert!(
            !names_pairs(&names_mentioned(message, &declared), &pairs),
            "{message}"
        );
    }
    // A word that is a name but not written as one is no name: `y` in prose.
    assert_eq!(
        names_mentioned("y is `m`'s, and `(task sys t!)` too", &declared),
        ["m", "(task sys t!)"]
    );
    // Model §8 rule 16's findings (`M4.1.2.3`), as the configuration checker names one and as §1 has the lowering name
    // the generator's: each clause a task, its source and a code, whatever its words; the conflicts before the unknowns,
    // each group whole in any order; a clause naming two tasks, or a source but no code, says not which is refused.
    let description = Description {
        blocks: vec![
            source_block("d", Separation::Unstated, ""),
            source_block("d.x", Separation::Stated(10), ""),
        ],
        tasks: ["t", "t!", "u"]
            .iter()
            .map(|name| Task {
                name: (*name).to_string(),
                release: Release::MinSeparation(Dur::Ms(20)),
                side: Side {
                    needs: names(&[INTERRUPT_FACT]),
                    ..Side::default()
                },
                ..Task::default()
            })
            .collect(),
        ..Description::default()
    };
    let finding = |task: &str, source: &str, code: &'static str| Unadmitted {
        task: format!("(task sys {task})"),
        source: source.to_string(),
        code,
        cause: if code == "missing-fact" {
            "unstated"
        } else {
            "shorter"
        },
    };
    let findings = [
        finding("t", "d.x", "unsupported-profile"),
        finding("t!", "d.x", "unsupported-profile"),
        finding("u", "d", "missing-fact"),
    ];
    for message in [
        "refused (unsupported-profile: `(task sys t)` arrives too often from `d.x`; unsupported-profile: \
         `(task sys t!)` from `d.x` too; missing-fact: `d` says nothing of `(task sys u)`'s arrivals)",
        "`(task sys t!)` and `d.x`, `unsupported-profile`; `d.x` with `(task sys t)`, `unsupported-profile`; then \
         `missing-fact` of `(task sys u)` at `d`",
    ] {
        assert_eq!(joint_disagrees(message, &description, &findings), None, "{message}");
    }
    for message in [
        "missing-fact: `(task sys u)` at `d`; unsupported-profile: `(task sys t)` at `d.x`; unsupported-profile: \
         `(task sys t!)` at `d.x`",
        "unsupported-profile: `(task sys t)` at `d.x`; missing-fact: `(task sys u)` at `d`",
        "unsupported-profile: `(task sys t)` and `(task sys t!)` at `d.x`; missing-fact: `(task sys u)` at `d`",
        "unsupported-profile: `(task sys t)` at `d.x`; unsupported-profile: `(task sys t!)` at `d`; missing-fact: \
         `(task sys u)` at `d`",
        "unsupported-profile: `(task sys t)` at `d.x`; `(task sys t!)` at `d.x`; missing-fact: `(task sys u)` at `d`",
    ] {
        assert!(
            joint_disagrees(message, &description, &findings).is_some(),
            "{message}"
        );
    }
    assert_eq!(
        rule_16_named(
            "`(task sys t!)` is bound to `d.x`, whose arrivals are closer: a conflict, `unsupported-profile` (rule 16)",
            &description
        ),
        Ok(Some((
            "(task sys t!)".to_string(),
            "d.x".to_string(),
            "unsupported-profile".to_string()
        )))
    );
    let hardware: BTreeSet<String> = ["d", "d.x"].iter().map(|s| (*s).to_string()).collect();
    let expiry = "(adapter deadline-over-delay d (task sys t))";
    assert_eq!(
        received_named(
            &format!(
                "`{expiry}` delivers an expiry to `(task sys t)`; `(task sys t!)` programs an alarm at `d.x` \
                 itself; `sys` programs alarms at `d` and `d.x`: none is placed"
            ),
            "sys",
            &hardware
        ),
        Ok(vec![
            pair("(task sys t)", expiry),
            pair("(task sys t!)", "d.x"),
            pair("sys", "d"),
            pair("sys", "d.x"),
        ])
    );
    assert!(received_named(
        "`(task sys t)` and `sys` program an alarm at `d`",
        "sys",
        &hardware
    )
    .is_err());
    assert_eq!(
        durations_named(
            "past it: `(source dev!)` states a separation; `(source dev)` too; \
             `(adapter deadline-over-delay timer.d dl!)` is obliged to a `delivery-bound`"
        ),
        [
            "(source dev!)",
            "(source dev)",
            "(adapter deadline-over-delay timer.d dl!) delivery-bound"
        ]
    );
    assert!(clause_named(
        "`(task sys t)` writes `(period 20 ms)` and is bound to a source",
        "period"
    ));
    assert!(!clause_named(
        "`(task sys t)` is bound to a source itself",
        "period"
    ));
    // The reading's order, by texts compared byte by byte, against the words a message names them with.
    let texts =
        |items: &[&str]| -> Vec<String> { items.iter().map(|s| (*s).to_string()).collect() };
    assert!(worded_otherwise(&texts(&["y", "y.b"]), |t| format!(
        "`{t}`"
    )));
    assert!(worded_otherwise(&texts(&["m", "m!"]), |t| format!("`{t}`")));
    assert!(!worded_otherwise(&texts(&["m!", "m.s"]), |t| format!(
        "`{t}`"
    )));
    let tasks = texts(&["(task sys t!)", "(task sys t)"]);
    assert!(
        tasks[0] < tasks[1],
        "`!` lies below the `)` that closes `(task sys t)`"
    );
    assert!(worded_otherwise(&tasks, |t| t
        .trim_end_matches(')')
        .rsplit(' ')
        .next()
        .expect("a name")
        .to_string()));
}

#[test]
fn the_reading_transcribes_the_shipped_vocabulary_and_catalog() {
    // The reading's constants are its transcription of `vocabulary.eadl` and `adapters.eadl`, held here to the shipped
    // vocabulary and catalog, so that a change to either fails here rather than in the reading's verdicts.
    // Every unit the language accepts is read, a time or not a time (model §4 rule 8; round 20, R20-R7).
    let time: Vec<&str> = eadl_model::quantity::UNITS
        .iter()
        .filter(|u| u.dimension == eadl_model::quantity::Dimension::Time)
        .map(|u| u.name)
        .collect();
    assert_eq!(time, TIME_UNITS.map(|(unit, _)| unit));
    let other: Vec<&str> = eadl_model::quantity::UNITS
        .iter()
        .filter(|u| u.dimension != eadl_model::quantity::Dimension::Time)
        .map(|u| u.name)
        .collect();
    assert_eq!(other, OTHER_UNITS);
    let vocabulary = common::vocabulary();
    let consumed: BTreeSet<&str> = vocabulary
        .facts()
        .filter(|f| f.usage == Usage::Consumed)
        .map(|f| f.name.as_str())
        .collect();
    assert_eq!(consumed, BTreeSet::from(CONSUMED));
    let catalog = common::catalog();
    assert_eq!(
        catalog
            .kinds
            .iter()
            .map(|k| k.name.as_str())
            .collect::<Vec<_>>(),
        KINDS.iter().map(|k| k.name).collect::<Vec<_>>()
    );
    for kind in &KINDS {
        let shipped = catalog.kind(kind.name).expect("a shipped kind");
        let obliged: Vec<&str> = shipped
            .offers
            .iter()
            .filter(|o| o.offered == Offered::Obliged)
            .map(|o| o.fact.as_str())
            .collect();
        assert_eq!(obliged, kind.obliged, "{}", kind.name);
        assert_eq!(shipped.beneath_names, kind.beneath, "{}", kind.name);
        let names = |owed: &[eadl_resolve::adapter::Owed]| -> Vec<String> {
            owed.iter().map(|o| o.name.clone()).collect()
        };
        assert_eq!(
            names(&shipped.obligations),
            kind.obligations,
            "{}",
            kind.name
        );
        assert_eq!(names(&shipped.costs), kind.costs, "{}", kind.name);
    }
}

/// The record's worked plans (§3), in its order, each with its digests elided.
fn worked_plans() -> Vec<String> {
    let record = the_record();
    let mut out = Vec::new();
    let mut open: Option<String> = None;
    for line in record.lines() {
        match (&mut open, line.trim_start()) {
            (None, "```text") => open = Some(String::new()),
            (Some(block), "```") => {
                // A worked plan, not §3's template of the forms: its digests are elided.
                if block.starts_with("(format \"archogen-runtime-plan/1\")")
                    && block.contains("(program \"…\")")
                {
                    out.push(std::mem::take(block));
                }
                open = None;
            }
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            (None, _) => {}
        }
    }
    out
}

/// The record's text.
fn the_record() -> String {
    std::fs::read_to_string(common::root().join("docs/decisions/decision_runtime-plan.md"))
        .expect("the record")
}

/// The words a text writes between backquotes, in its order.
fn backquoted(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// The facts §2.10's table owes `subject`, as its row writes them: `| SUBJECT | FACTS | OWNER |`, with that owner.
fn owed_row(record: &str, subject: &str) -> (Vec<String>, String) {
    let rows: Vec<Vec<&str>> = record
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| line.split('|').map(str::trim).collect::<Vec<_>>())
        .filter(|cells| cells.len() == 5 && cells[1] == subject)
        .collect();
    assert_eq!(rows.len(), 1, "§2.10's one row for {subject}");
    let owner = backquoted(rows[0][3]);
    assert_eq!(owner.len(), 1, "{subject}'s one owner");
    (backquoted(rows[0][2]), owner[0].clone())
}

/// The heads of §3's template, "The forms come in the order below": the first `text` block after §3's heading, one form
/// on each line.
fn template_heads(record: &str) -> Vec<String> {
    let from = record.find("### 3. The format").expect("§3's heading");
    let block = record[from..]
        .split("```text\n")
        .nth(1)
        .and_then(|rest| rest.split("```").next())
        .expect("§3's template");
    block
        .lines()
        .map(|line| {
            line.trim_start_matches('(')
                .split([' ', ')'])
                .next()
                .expect("a head")
                .to_string()
        })
        .collect()
}

/// The words §3 writes between backquotes after `opening`, up to `closing`, each with its whitespace collapsed: a list
/// such as "`NODEKIND` one of `block`, ... or `policy`".
fn sentence_list(record: &str, opening: &str, closing: &str) -> Vec<String> {
    let from = record
        .find(opening)
        .unwrap_or_else(|| panic!("§3's `{opening}`"))
        + opening.len();
    let to = from
        + record[from..]
            .find(closing)
            .unwrap_or_else(|| panic!("§3's `{opening}` closing at `{closing}`"));
    backquoted(&record[from..to])
        .into_iter()
        .map(|word| word.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

/// §3's `FACT` list: "`FACT` in `owed` is one of: ...", up to "or an obliged fact's name".
fn fact_list(record: &str) -> BTreeSet<String> {
    let opening = "- `FACT` in `owed` is one of:";
    let from = record.find(opening).expect("§3's `FACT` list") + opening.len();
    let to = from
        + record[from..]
            .find("or an obliged")
            .expect("its last member, an obliged fact's name");
    backquoted(&record[from..to]).into_iter().collect()
}

#[test]
fn the_reading_gives_the_record_s_worked_texts() {
    // The reading's facts of a task, a source and a timer route, held to §2.10's table and §3's `FACT` list as the
    // record writes them (round 19, R19-D2: a task's `os-calls-category`).
    let record = the_record();
    let listed = fact_list(&record);
    for (subject, facts, owner) in [
        ("each task", &TASK_FACTS[..], "application"),
        ("each source", &SOURCE_FACTS[..], "realization"),
        ("each timer route", &TIMER_ROUTE_FACTS[..], "realization"),
    ] {
        assert_eq!(
            owed_row(&record, subject),
            (names(facts), owner.to_string()),
            "§2.10's row for {subject}"
        );
        for fact in facts {
            assert!(listed.contains(*fact), "§3's `FACT` list lacks `{fact}`");
        }
    }
    // The reading's forms, node kinds and causes, held to §3's text (round 24, R24-D2, R24-R1): the heads of its
    // template, in "The forms come in the order below"; "`NODEKIND` one of ..."; and "`CAUSE` one of ...", each cause
    // by its head, the eight of round 28 (R28-D2) in §3's order.
    assert_eq!(template_heads(&record), FORMS, "§3's forms, in its order");
    assert_eq!(
        sentence_list(&record, "and `NODEKIND` one of", ".\n"),
        NODE_KINDS,
        "§3's node kinds"
    );
    let causes: Vec<String> = sentence_list(&record, "`CAUSE` one of", "each `ITEM` a source")
        .iter()
        .map(|cause| {
            cause
                .trim_start_matches('(')
                .split([' ', ')'])
                .next()
                .expect("a head")
                .to_string()
        })
        .collect();
    assert_eq!(causes, CAUSES, "§3's causes");
    // And §2.2's pairing of them (round 28, R28-D2): "for its one source `(unstated ITEM)` ..., `(absent ITEM)` ...,
    // `(faster ITEM)` ..., each written `(own-unstated ITEM)`, `(own-absent ITEM)` or `(own-faster ITEM)` where that
    // source is the one the task binds itself" — the routes' three, then the task's own source's, in one order.
    let paired: Vec<String> = sentence_list(
        &record,
        "and for its one source",
        "where that source is the",
    )
    .iter()
    .map(|cause| {
        cause
            .trim_start_matches('(')
            .split([' ', ')'])
            .next()
            .expect("a head")
            .to_string()
    })
    .collect();
    assert_eq!(
        paired.len(),
        6,
        "§2.2's three causes of one source, each written apart for the task's own"
    );
    for (routes, own) in paired[..3].iter().zip(&paired[3..]) {
        assert_eq!(
            one_source_cause(routes, false),
            routes,
            "§2.2's cause of a source the routes leave"
        );
        assert_eq!(
            one_source_cause(routes, true),
            own,
            "§2.2's cause of the source the task binds itself"
        );
    }
    // And §2.2's statement that no accepted record reaches the own three (R29-R2): "no record the generator accepts binds
    // a task's own source the analysis does not admit: `(own-faster ITEM)`, `(own-absent ITEM)` and `(own-unstated
    // ITEM)` stay in the grammar" — the three the reading states unreachable, each by its class, and no other.
    let stated: BTreeSet<String> = sentence_list(
        &record,
        "binds a task's own source the analysis does not admit:",
        "stay in the grammar",
    )
    .into_iter()
    .collect();
    let unreachable: BTreeSet<String> = CAUSES
        .iter()
        .filter(|cause| {
            OWN_CAUSES_UNREACHABLE
                .iter()
                .any(|class| class.contains(&format!("`(unknown ({cause} ITEM))`")))
        })
        .map(|cause| format!("({cause} ITEM)"))
        .collect();
    assert_eq!(
        (stated.len(), &stated),
        (3, &unreachable),
        "§2.2's causes no accepted record reaches, the reading's unreachable ones"
    );
    assert!(OWN_CAUSES_UNREACHABLE
        .iter()
        .all(|class| class.starts_with("unreachable: ")));

    // The reading held to the plans the record writes in full (§3), from their inputs transcribed here and their own
    // `bind` lines, for every kind of line it derives: a fault of the reading shows on the record's own text, never on
    // the code's.
    let task =
        |name: &str, release: Release, priority: u32, deadline_from: bool, uses: &[&str]| Task {
            name: name.to_string(),
            release,
            priority,
            deadline_from: deadline_from.then_some("release"),
            side: Side {
                uses: names(uses),
                ..Side::default()
            },
            ..Task::default()
        };
    let plain = |name: &str, offers: &[&str]| Block {
        name: name.to_string(),
        offers: names(offers),
        ..Block::default()
    };
    let uses = |declarations: &[&str]| Side {
        uses: names(declarations),
        ..Side::default()
    };
    let heartbeat = Description {
        system: "heartbeat".to_string(),
        blocks: vec![plain("console.uart", &["(observable-output true)"])],
        services: vec![Service {
            name: "console.write".to_string(),
            policy: false,
            side: Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
        }],
        tasks: vec![
            task(
                "beat",
                Release::Period(Dur::Ms(10)),
                1,
                true,
                &["console.write"],
            ),
            task(
                "chime",
                Release::Period(Dur::Ms(30)),
                2,
                true,
                &["console.write"],
            ),
        ],
        own: uses(&["console.write"]),
        ..Description::default()
    };
    let extender = Description {
        system: "short-counter".to_string(),
        blocks: vec![plain("timer.short", &["(relative-delay true)"])],
        services: vec![service(
            "time.monotonic",
            Counter::Extender,
            false,
            Vec::new(),
        )],
        tasks: vec![task(
            "beat",
            Release::Period(Dur::Ms(10)),
            1,
            true,
            &["time.monotonic"],
        )],
        own: uses(&["time.monotonic"]),
        ..Description::default()
    };
    let sporadic = Description {
        system: "sporadic".to_string(),
        blocks: vec![source_block("dev.e", Separation::Stated(20), "")],
        services: vec![service("evt", Counter::None, true, Vec::new())],
        tasks: vec![task(
            "s",
            Release::MinSeparation(Dur::Ms(10)),
            1,
            false,
            &["evt"],
        )],
        own: uses(&["evt"]),
        ..Description::default()
    };
    // Each worked text's `/2` record, as the record states it (round 20, R20-R7: the test had passed each plan as its own
    // record): the heartbeat's "assignment the one line `(bind console.write observable-output console.uart)`"; the
    // adapter's, which "binds `time.monotonic`'s four facts to `(adapter epoch-extender timer.short)` and the instance's
    // five to `timer.short`" — the four its requirement writes, the five its kind's `beneath`
    // (`docs/semantics/adapters/adapters.eadl`); and the source's, "the one line `(bind evt interrupt-source dev.e)`".
    // Each is written in `/2`'s order, "the bindings sorted by requester, every name, then every task, then every
    // instance, each by its text, then by fact" (`eadl_resolve::plan`), its digest elided as the worked texts elide it.
    let record_of = |system: &str, binds: Vec<String>| -> String {
        let mut record = format!(
            "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system {system})\n(program \"…\")\n\
             (adapters {})\n",
            eadl_resolve::adapter::VERSION
        );
        for bind in binds {
            record.push_str(&bind);
            record.push('\n');
        }
        record
    };
    let instance = "(adapter epoch-extender timer.short)";
    let monotonic = &extender.services[0].side;
    let mut four: Vec<String> = monotonic
        .needs
        .iter()
        .cloned()
        .chain(
            monotonic
                .other
                .iter()
                .map(|clause| text_of(&items(&form(clause))[0])),
        )
        .collect();
    four.sort();
    assert_eq!(four.len(), 4, "`time.monotonic`'s four facts");
    let mut five: Vec<&str> = kind_of(instance).beneath.to_vec();
    five.sort_unstable();
    let records = [
        record_of(
            "heartbeat",
            vec!["(bind console.write observable-output console.uart)".to_string()],
        ),
        record_of(
            "short-counter",
            four.iter()
                .map(|fact| format!("(bind time.monotonic {fact} {instance})"))
                .chain(
                    five.iter()
                        .map(|fact| format!("(bind {instance} {fact} timer.short)")),
                )
                .collect(),
        ),
        record_of(
            "sporadic",
            vec!["(bind evt interrupt-source dev.e)".to_string()],
        ),
    ];
    let worked = worked_plans();
    assert_eq!(worked.len(), 3);
    for ((description, plan), record) in [heartbeat, extender, sporadic]
        .iter()
        .zip(&worked)
        .zip(&records)
    {
        let judged = judge_elided(description, "…", record, plan, true);
        let shown: Vec<String> = judged
            .disagreements
            .iter()
            .map(|d| {
                format!(
                    "{}: the reading\n{}\nthe record\n{}",
                    d.kind,
                    d.expected.join("\n"),
                    d.written.join("\n")
                )
            })
            .collect();
        assert!(
            shown.is_empty(),
            "{}:\n{}",
            description.system,
            shown.join("\n")
        );
    }
}
