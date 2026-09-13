# Cost-accounting contract `cost-accounting/1`

The versioned record `ROADMAP.md` §7.4.1 requires of every admitted runtime analysis. It is
published here and declared as data in `crates/rt-analysis/src/cost.rs`; a test fails if the two
diverge, so this page cannot become a description of something the engine no longer does.

> **Version:** `cost-accounting/1` · **Status:** active · **Established:** 2026-09-13 by leaf `M2.4`

⚠️ **A total is only meaningful under the accounting rules it was computed with.** §15 versions
evidence formats separately from everything else for this reason: changing a rule below
invalidates every total previously stated under `cost-accounting/1`, and the new rules must be
published as `cost-accounting/2` rather than edited into this page.

## The seven identifications

§7.4.1 requires an analysis to identify each of these. What this contract says about each is
binding on any total stated under it.

| Identification | This contract |
| --- | --- |
| timing observation boundary | a job's response interval ends at the end of its useful computation; the switch away from it is outside that job's interval and remains interference for others |
| what task execution bounds include | useful computation only — no dispatch, no interrupt service, no context save or restore is inside a declared `C` |
| how releases become ready | a release is signalled by an interrupt; the task becomes ready at ISR completion, and its deadline still refers to the nominal release, so the readiness delay is inside the response time |
| which operations are preemptible | task computation is preemptible; switches and interrupt service are not. Arrivals during them are latched and serviced before the next task computation interval |
| the number and kind of context transitions | one initial dispatch from idle, and one two-part switch (save outgoing, restore incoming) per task-to-task transition, charged in both directions |
| interrupt arrival and service assumptions | every modelled source declares its arrivals and its service cost; an undeclared source is `unmodeled-interrupt-load`, which the profile excludes |
| dispatch, critical-section, instrumentation and idle/wakeup costs | each has its own category and is charged to exactly one interval; a cost with no category is an omission, not a zero |

## One interval, one category

> Every physical execution interval in a fixed trace has **one primary ledger category**. […]
> **Charge only mutually disjoint intervals** when asserting exact totals.

This is enforced, not reviewed. A ledger is a set of half-open intervals, and sealing one as an
**exact trace** refuses:

- an **overlap** — time charged twice. This is §7.4.1's own example ("do not count the same
  interrupt entry instructions once in an ISR term and again inside a context-switch term") and
  fixture F29's fourth control. The total still looks plausible when it happens, which is exactly
  why a reviewer misses it.
- a **gap** — time charged to nothing. An omitted cost *is* a hole: the trace says the processor
  was busy and the ledger says nothing was spent. F29's second and third controls are omissions,
  and each turns a real deadline miss into a false pass.

The categories are: `initial dispatch`, `<task> useful execution`, `interrupt service`,
`task switch`, `critical section`, `instrumentation`, `idle/wakeup`.

## Three kinds of total, deliberately not interchangeable

| Kind | Disjointness required | What it supports |
| --- | --- | --- |
| `exact-trace` | **yes** | an exact statement about one fixed trace |
| `safe-envelope` | no | an analytical upper bound, whose over-counting is declared |
| `observed-maximum` | no | an empirical observation, and nothing more |

§7.4.1 permits deliberate pessimism and forbids hiding it:

> For analytical upper bounds, conservative over-counting may be intentional […] Document that
> conservatism separately. The requirement is no *undocumented* omission or duplicate charge, not
> a ban on sound pessimism.

So an envelope may charge the same interval twice — that is what makes it safe — and it must say
which kind it is. An observed maximum with a safety multiplier remains an empirical assumption
(§7.3), whatever it is multiplied by.

## What is stated under this contract

- Fixture **F29** (§13.4), the repeated-preemption trace whose exact ledger totals 23 — leaf
  `M2.5`.
- The runtime-applicable response-time analysis of §7.4 — leaf `M2.6`.

The idealized baseline in `crates/rt-analysis/src/response.rs` is **not** stated under this
contract: it charges no overhead at all, which is its own declared assumption, and §7.4 forbids
selecting it for a runtime whose overheads are omitted.
