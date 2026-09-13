//! The cost-accounting contract of `ROADMAP.md` §7.4.1, and the ledger that obeys it.
//!
//! § 7.4.1 asks every admitted runtime analysis to carry a **versioned** record of how time is
//! charged, and then states the rule that makes such a record worth having:
//!
//! > Every physical execution interval in a fixed trace has **one primary ledger category**. For
//! > example, do not count the same interrupt entry instructions once in an ISR term and again
//! > inside a context-switch term. […] **Charge only mutually disjoint intervals** when asserting
//! > exact totals.
//!
//! ⭐ **That rule is structural here, not a review instruction.** A [`Ledger`] is a set of
//! half-open intervals over one trace, and [`Ledger::seal`] refuses a trace with an overlap or a
//! hole. Double-charging an interval is therefore not a mistake an analysis can make quietly: the
//! ledger will not close. §13.4's fourth control — "charge the two ISR intervals again inside
//! task cost" — is exactly this failure, and it is the one an eyeballed spreadsheet misses,
//! because the total still looks plausible.
//!
//! # Exact accounting and safe envelopes are different things
//!
//! §7.4.1 is careful to permit deliberate pessimism and to forbid hiding it:
//!
//! > For analytical upper bounds, conservative over-counting may be intentional […] Document that
//! > conservatism separately. The requirement is no *undocumented* omission or duplicate charge,
//! > not a ban on sound pessimism. Distinguish exact trace accounting, safe analytical envelopes,
//! > and observed maxima.
//!
//! So [`Accounting`] names which of the three a total is, and the disjointness rule is enforced
//! **only** for [`Accounting::ExactTrace`]. An envelope is allowed to double-count — that is what
//! makes it safe — and it must say so.

use core::fmt;
use std::collections::BTreeMap;

/// The contract's version. §15 versions evidence formats separately from everything else: a
/// changed accounting rule invalidates every total previously stated under it.
pub const CONTRACT_VERSION: &str = "cost-accounting/1 (ROADMAP.md §7.4.1)";

/// What a total is, and therefore what may be concluded from it.
///
/// ⛔ The three are not interchangeable, and §7.1 keeps them apart for the same reason: an
/// observed maximum with a safety factor is still an empirical assumption, whatever it is
/// multiplied by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accounting {
    /// Exact accounting over one fixed trace. Intervals must be disjoint and cover the trace
    /// with no hole; the total is the trace's own length.
    ExactTrace,
    /// A safe analytical envelope. Deliberate over-counting is permitted and must be declared,
    /// so disjointness is *not* required — but the conservatism is recorded rather than implied.
    SafeEnvelope,
    /// An observed maximum. Not a bound, however many runs produced it.
    ObservedMaximum,
}

impl Accounting {
    /// The machine-readable name used in reports.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::ExactTrace => "exact-trace",
            Self::SafeEnvelope => "safe-envelope",
            Self::ObservedMaximum => "observed-maximum",
        }
    }

    /// Whether disjoint, gapless coverage is required. True only for exact accounting.
    #[must_use]
    pub const fn requires_disjoint_coverage(self) -> bool {
        matches!(self, Self::ExactTrace)
    }
}

/// The primary category an interval is charged to.
///
/// One interval, one category — that is the whole point. The set is the §13.4 ledger's, extended
/// with the three §7.4.1 names an analysis may also need to charge.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    /// Getting from idle to the first task. §13.4 charges this separately from a task switch
    /// because there is no outgoing context to save.
    InitialDispatch,
    /// Useful computation of a named task. Excludes every transition around it.
    TaskExecution(String),
    /// Interrupt service. §13.4: "ISR and switch costs are disjoint from each other and from
    /// useful task computation."
    InterruptService,
    /// A task-to-task switch: saving the outgoing context and restoring the incoming one.
    TaskSwitch,
    /// A bounded kernel critical section (§3.1's "bounded kernel critical sections").
    CriticalSection,
    /// Instrumentation or tracing cost. §7.4.1 names it because an unaccounted tracer is a
    /// classic source of a total that does not reproduce off the bench.
    Instrumentation,
    /// Leaving or entering idle.
    IdleWakeup,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitialDispatch => write!(f, "initial dispatch"),
            Self::TaskExecution(task) => write!(f, "{task} useful execution"),
            Self::InterruptService => write!(f, "interrupt service"),
            Self::TaskSwitch => write!(f, "task switch"),
            Self::CriticalSection => write!(f, "critical section"),
            Self::Instrumentation => write!(f, "instrumentation"),
            Self::IdleWakeup => write!(f, "idle/wakeup"),
        }
    }
}

/// One charged interval `[start, end)` in abstract integer time units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    /// Inclusive start.
    pub start: u64,
    /// Exclusive end.
    pub end: u64,
    /// The single primary category this interval is charged to.
    pub category: Category,
    /// What happened, for the human reading the ledger.
    pub activity: String,
}

impl Interval {
    /// Its duration.
    #[must_use]
    pub const fn duration(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }
}

/// Why a ledger does not close.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// An interval ends before it starts, or is empty.
    NotAnInterval { start: u64, end: u64 },
    /// ⭐ Two intervals claim the same time. This is §7.4.1's duplicate charge, and §13.4's
    /// fourth control is exactly it.
    Overlap {
        /// Where the overlap begins.
        at: u64,
        /// The category already holding it.
        held_by: Category,
        /// The category trying to claim it again.
        claimed_by: Category,
    },
    /// ⭐ Time nobody claimed. §13.4's second and third controls are omissions, and an omission
    /// is exactly a hole: the trace says the processor was doing something and the ledger says
    /// nothing was charged.
    Gap { from: u64, to: u64 },
    /// The ledger is empty, so there is nothing to total.
    Empty,
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnInterval { start, end } => {
                write!(f, "[{start}, {end}) is not a forward, non-empty interval")
            }
            Self::Overlap {
                at,
                held_by,
                claimed_by,
            } => write!(
                f,
                "time {at} is charged twice: to `{held_by}` and again to `{claimed_by}`. §7.4.1: \
                 every physical execution interval has ONE primary ledger category — do not count \
                 the same instructions in two terms"
            ),
            Self::Gap { from, to } => write!(
                f,
                "[{from}, {to}) is charged to nothing. An exact trace accounts for every unit it \
                 spans; a hole is an omitted cost, which is how a real miss becomes a false pass"
            ),
            Self::Empty => write!(f, "the ledger is empty, so there is no total to assert"),
        }
    }
}

/// A closed ledger over one trace: every unit charged exactly once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ledger {
    intervals: Vec<Interval>,
    accounting: Accounting,
}

impl Ledger {
    /// Seal a set of intervals as a ledger.
    ///
    /// For [`Accounting::ExactTrace`] the intervals must be disjoint and cover `[first, last)`
    /// with no hole. For the other two, they are sorted and checked for well-formedness only:
    /// an envelope is *allowed* to over-count, which is what makes it safe.
    ///
    /// # Errors
    ///
    /// The first [`LedgerError`] found, in time order.
    pub fn seal(mut intervals: Vec<Interval>, accounting: Accounting) -> Result<Self, LedgerError> {
        if intervals.is_empty() {
            return Err(LedgerError::Empty);
        }
        for interval in &intervals {
            if interval.end <= interval.start {
                return Err(LedgerError::NotAnInterval {
                    start: interval.start,
                    end: interval.end,
                });
            }
        }
        intervals.sort_by_key(|interval| (interval.start, interval.end));

        if accounting.requires_disjoint_coverage() {
            for window in intervals.windows(2) {
                let (earlier, later) = (&window[0], &window[1]);
                if later.start < earlier.end {
                    return Err(LedgerError::Overlap {
                        at: later.start,
                        held_by: earlier.category.clone(),
                        claimed_by: later.category.clone(),
                    });
                }
                if later.start > earlier.end {
                    return Err(LedgerError::Gap {
                        from: earlier.end,
                        to: later.start,
                    });
                }
            }
        }

        Ok(Self {
            intervals,
            accounting,
        })
    }

    /// The intervals, in time order.
    #[must_use]
    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    /// What kind of total this is.
    #[must_use]
    pub const fn accounting(&self) -> Accounting {
        self.accounting
    }

    /// The total charged time.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.intervals.iter().map(Interval::duration).sum()
    }

    /// The span the ledger covers, `[first start, last end)`.
    #[must_use]
    pub fn span(&self) -> (u64, u64) {
        let first = self
            .intervals
            .first()
            .expect("a sealed ledger is non-empty");
        let last = self.intervals.last().expect("a sealed ledger is non-empty");
        (first.start, last.end)
    }

    /// Time charged per category, in category order.
    #[must_use]
    pub fn by_category(&self) -> BTreeMap<Category, u64> {
        let mut totals: BTreeMap<Category, u64> = BTreeMap::new();
        for interval in &self.intervals {
            *totals.entry(interval.category.clone()).or_default() += interval.duration();
        }
        totals
    }

    /// Render the ledger as the table §13.4 publishes, so a reader can compare line by line.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = format!(
            "ledger ({}) under {CONTRACT_VERSION}\n",
            self.accounting.slug()
        );
        for interval in &self.intervals {
            out.push_str(&format!(
                "  [{}, {}) {:<34} {:>3}  {}\n",
                interval.start,
                interval.end,
                interval.activity,
                interval.duration(),
                interval.category
            ));
        }
        out.push_str(&format!("  total {}\n", self.total()));
        out
    }
}

/// The versioned cost-accounting contract: the seven things §7.4.1 requires an analysis to
/// identify, as data.
///
/// Declared here rather than only in prose so a report can carry it, and so the drift test in
/// this module fails if the published page and this record diverge — the same device
/// `crates/eadl-model/src/profile.rs` uses for the profile.
pub struct Contract {
    /// The version this contract is published under.
    pub version: &'static str,
    /// §7.4.1's required identifications, each with what this contract says about it.
    pub terms: &'static [(&'static str, &'static str)],
}

/// The contract the §13.4 fixture and the runtime-applicable analysis are stated under.
pub const COST_ACCOUNTING_V1: Contract = Contract {
    version: CONTRACT_VERSION,
    terms: &[
        (
            "timing observation boundary",
            "a job's response interval ends at the end of its useful computation; the switch away \
             from it is outside that job's interval and remains interference for others",
        ),
        (
            "what task execution bounds include",
            "useful computation only — no dispatch, no interrupt service, no context save or \
             restore is inside a declared C",
        ),
        (
            "how releases become ready",
            "a release is signalled by an interrupt; the task becomes ready at ISR completion, \
             and its deadline still refers to the nominal release, so the readiness delay is \
             inside the response time",
        ),
        (
            "which operations are preemptible",
            "task computation is preemptible; switches and interrupt service are not. Arrivals \
             during them are latched and serviced before the next task computation interval",
        ),
        (
            "the number and kind of context transitions",
            "one initial dispatch from idle, and one two-part switch (save outgoing, restore \
             incoming) per task-to-task transition, charged in both directions",
        ),
        (
            "interrupt arrival and service assumptions",
            "every modelled source declares its arrivals and its service cost; an undeclared \
             source is `unmodeled-interrupt-load`, which the profile excludes",
        ),
        (
            "dispatch, critical-section, instrumentation and idle/wakeup costs",
            "each has its own category and is charged to exactly one interval; a cost with no \
             category is an omission, not a zero",
        ),
    ],
};

#[cfg(test)]
mod tests {
    use super::{
        Accounting, Category, Contract, Interval, Ledger, LedgerError, COST_ACCOUNTING_V1,
    };

    fn interval(start: u64, end: u64, category: Category) -> Interval {
        Interval {
            start,
            end,
            category,
            activity: "x".into(),
        }
    }

    #[test]
    fn an_exact_ledger_closes_only_when_every_unit_is_charged_once() {
        let ledger = Ledger::seal(
            vec![
                interval(0, 1, Category::InitialDispatch),
                interval(1, 4, Category::TaskExecution("L".into())),
                interval(4, 5, Category::InterruptService),
            ],
            Accounting::ExactTrace,
        )
        .expect("disjoint and gapless");
        assert_eq!(ledger.total(), 5);
        assert_eq!(ledger.span(), (0, 5));
    }

    #[test]
    fn a_duplicate_charge_cannot_be_sealed() {
        // ⭐ §7.4.1's example, and §13.4's fourth control: "do not count the same interrupt entry
        // instructions once in an ISR term and again inside a context-switch term". The total
        // would still look plausible, which is why this must be structural rather than reviewed.
        let error = Ledger::seal(
            vec![
                interval(0, 4, Category::TaskExecution("L".into())),
                interval(3, 5, Category::InterruptService),
            ],
            Accounting::ExactTrace,
        )
        .expect_err("time 3 is charged twice");
        let LedgerError::Overlap { at, .. } = error else {
            panic!("expected an overlap, got {error:?}");
        };
        assert_eq!(at, 3);
        assert!(
            error.to_string().contains("ONE primary ledger category"),
            "{error}"
        );
    }

    #[test]
    fn an_omitted_cost_is_a_hole_and_cannot_be_sealed_either() {
        // §13.4's second and third controls are omissions. An omission is a gap: the trace says
        // the processor was busy and the ledger charges nothing.
        let error = Ledger::seal(
            vec![
                interval(0, 4, Category::TaskExecution("L".into())),
                interval(5, 7, Category::TaskSwitch),
            ],
            Accounting::ExactTrace,
        )
        .expect_err("[4, 5) is charged to nothing");
        assert_eq!(error, LedgerError::Gap { from: 4, to: 5 });
        assert!(error.to_string().contains("false pass"), "{error}");
    }

    #[test]
    fn a_safe_envelope_may_over_count_and_says_so() {
        // §7.4.1: "conservative over-counting may be intentional […] Document that conservatism
        // separately. The requirement is no UNDOCUMENTED omission or duplicate charge, not a ban
        // on sound pessimism." So the same intervals that fail as an exact trace seal fine here.
        let overlapping = vec![
            interval(0, 4, Category::TaskExecution("L".into())),
            interval(3, 5, Category::InterruptService),
        ];
        assert!(Ledger::seal(overlapping.clone(), Accounting::ExactTrace).is_err());
        let envelope =
            Ledger::seal(overlapping, Accounting::SafeEnvelope).expect("pessimism is allowed");
        assert_eq!(envelope.accounting(), Accounting::SafeEnvelope);
        assert!(!envelope.accounting().requires_disjoint_coverage());
        assert_eq!(
            envelope.total(),
            6,
            "4 + 2, deliberately more than the 5 units spanned"
        );
    }

    #[test]
    fn an_observed_maximum_is_never_an_exact_trace() {
        assert!(!Accounting::ObservedMaximum.requires_disjoint_coverage());
        assert_ne!(Accounting::ObservedMaximum, Accounting::ExactTrace);
    }

    #[test]
    fn a_backwards_or_empty_interval_is_refused_before_anything_else() {
        assert!(matches!(
            Ledger::seal(
                vec![interval(4, 4, Category::TaskSwitch)],
                Accounting::ExactTrace
            ),
            Err(LedgerError::NotAnInterval { .. })
        ));
        assert!(matches!(
            Ledger::seal(
                vec![interval(5, 2, Category::TaskSwitch)],
                Accounting::ExactTrace
            ),
            Err(LedgerError::NotAnInterval { .. })
        ));
        assert_eq!(
            Ledger::seal(vec![], Accounting::ExactTrace),
            Err(LedgerError::Empty)
        );
    }

    #[test]
    fn intervals_need_not_be_supplied_in_order() {
        // A trace assembled from several sources arrives unordered, and requiring the caller to
        // sort is an invitation to sort wrongly.
        let ledger = Ledger::seal(
            vec![
                interval(1, 4, Category::TaskExecution("L".into())),
                interval(0, 1, Category::InitialDispatch),
            ],
            Accounting::ExactTrace,
        )
        .expect("sorted internally");
        assert_eq!(ledger.intervals()[0].start, 0);
    }

    #[test]
    fn per_category_totals_add_up_to_the_whole() {
        let ledger = Ledger::seal(
            vec![
                interval(0, 1, Category::InitialDispatch),
                interval(1, 4, Category::TaskExecution("L".into())),
                interval(4, 5, Category::InterruptService),
                interval(5, 7, Category::TaskSwitch),
            ],
            Accounting::ExactTrace,
        )
        .expect("closed");
        let by_category = ledger.by_category();
        assert_eq!(by_category.values().sum::<u64>(), ledger.total());
        assert_eq!(by_category[&Category::TaskSwitch], 2);
        assert_eq!(by_category[&Category::TaskExecution("L".into())], 3);
    }

    #[test]
    fn the_contract_identifies_every_term_seven_point_four_point_one_requires() {
        // §7.4.1 lists seven identifications. A contract missing one is a contract with an
        // undeclared cost, which is the thing it exists to prevent.
        let Contract { version, terms } = COST_ACCOUNTING_V1;
        assert!(version.contains("§7.4.1"), "{version}");
        assert_eq!(terms.len(), 7);
        for (name, says) in terms {
            assert!(!name.is_empty());
            assert!(
                says.len() > 60,
                "term `{name}` is identified but not actually specified: {says}"
            );
        }
    }

    #[test]
    fn the_contract_is_versioned_because_a_changed_rule_invalidates_old_totals() {
        // §15 versions evidence formats separately. A total is only meaningful under the
        // accounting rules it was computed with.
        assert!(super::CONTRACT_VERSION.starts_with("cost-accounting/1"));
    }
}

#[cfg(test)]
mod published {
    use super::COST_ACCOUNTING_V1;
    use std::path::Path;

    /// The published page, read from the repository.
    fn page() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crates/<name>/ is two levels below the root")
            .join("docs/analysis/cost-accounting-v1.md");
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
    }

    /// Markdown code spans removed. The page sets `C` and `unmodeled-interrupt-load` in
    /// backticks because a reader is reading a table; the declared string flows into reports as
    /// prose. That is presentation, not meaning — and comparing raw text would force the page to
    /// be worse in order to keep the test green, which is how a drift test starts being worked
    /// around instead of satisfied.
    fn without_code_spans(text: &str) -> String {
        text.replace('`', "")
    }

    #[test]
    fn the_published_contract_and_the_declared_one_are_the_same_contract() {
        // ⭐ The same device `eadl-model::profile` uses against the published profile page. A
        // contract that says one thing to a reader and another to the engine is worse than an
        // unpublished one: the reader cites the page, and the total was computed under the code.
        let page = without_code_spans(&page());
        for (name, says) in COST_ACCOUNTING_V1.terms {
            assert!(
                page.contains(&without_code_spans(name)),
                "the published contract does not identify `{name}`"
            );
            assert!(
                page.contains(&without_code_spans(says)),
                "the published contract's wording for `{name}` differs from the declared one.\n\
                 declared: {says}"
            );
        }
    }

    #[test]
    fn the_published_page_carries_the_version_totals_are_stated_under() {
        assert!(page().contains("cost-accounting/1"));
    }

    #[test]
    fn the_published_page_names_every_category_an_interval_can_be_charged_to() {
        // A category that exists in code but not on the page is a cost a reader cannot account
        // for when they check a ledger by hand.
        let page = page();
        for name in [
            "initial dispatch",
            "useful execution",
            "interrupt service",
            "task switch",
            "critical section",
            "instrumentation",
            "idle/wakeup",
        ] {
            assert!(page.contains(name), "the page does not name `{name}`");
        }
    }
}
