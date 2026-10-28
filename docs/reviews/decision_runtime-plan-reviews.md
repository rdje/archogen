# Review history — `decision_runtime-plan.md`

The independent review of [`docs/decisions/decision_runtime-plan.md`](../decisions/decision_runtime-plan.md), leaf
`M4.1.1`'s design of the runtime plan. Each round is a reader who had not seen the reasoning that wrote the record, asked
whether it is true to `ROADMAP.md`, the model, the earlier decisions and the code, and complete for what `M4.1` and the
obligations written into it ask — never a hunt for code mutants.

## Round 1 — `2026-10-27`: not acceptable, 28 findings

The record then listed only the routes' sources as interrupts, derived no stack, left section offsets to the linker,
refused a cycle at lowering, gave the plan no grammar and put the engine's version in its text.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R1-1 | defect | the enabled set left out the release timer and the platform's non-maskable interrupts and traps, so `periodic-three` would have none | A release timer, its rule, and the enabled set as `runtime.rs` defines it, the catalog's interrupts owed (§2.5) |
| R1-2 | defect | the plan ordered timers, whose place is the hardware's (`external-before-timer`) | Only external sources are ranked (§2.5) |
| R1-3 | defect | an adapter's cost interrupt listed as a second source duplicates its provider's route | A cost adds terms to its provider's route, owed by the realization (`M4.2`); a shared instance counts once (§2.5) |
| R1-4 | defect | a tie broken by name is not the controller's, which may lack the levels | Distinct ranks; the emitter maps them and refuses `unsupported-profile` on too few levels, written into `M4.2` (§2.5) |
| R1-5 | remark | the ordering key was undefined for a source that releases no task, and soundness's dependence on the emitted configuration unstated | Deadline-monotonic over each source's separation, a heuristic; soundness resting on the emitted ranks, checked by `M4.10` (§2.5) |
| R1-6 | defect | `infeasible-configuration` for a cycle contradicts `resolve`'s `ok` and rule 6 | Rule 12 amended so a cycle is a conflict of that assignment (`M4.1.2`); the lowering, given one, refuses `tool-failure` (§2.7) |
| R1-7 | defect | `M3.3.8`'s execution contexts and its no-waiting rule unanswered | A contexts section; initialisation's no-waiting an obligation on the record's entry, owed (§2.6) |
| R1-8 | defect | the stack allocation given to the application, against `stack-allocation.eadl` and §7.3 | The engine derives it from the application's bound plus frames and guard; the application inputs an optional input (§1, §2.8) |
| R1-9 | defect | "section offsets are the linker's" contradicts rule 10's "`M4.1` the offsets" | Allocations with offsets, from the free ranges, `M4.4` checking the linked image against them (§2.8) |
| R1-10 | defect | placement memory with no size was not handled | Rule 10 amended: `missing-fact` (`M4.1.2`, §2.8) |
| R1-11 | defect | the unsized hole's rule decided in the lowering alone, disagreeing with `resolve` and the checker | Rule 10 amended for the search, the checker and the lowering together (`M4.1.2`, §2.8) |
| R1-12 | defect | `Joint::map` was said to be uncalled and to take bindings | Only `Joint::initialization` is uncalled; the map is the search's, per system (§2.7, §2.8) |
| R1-13 | defect | no refusal for another catalog, a wrong producer or a build subject; no input carries the program's profile | Each refused; the two records' profiles compared (§1) |
| R1-14 | defect | the lowering cannot see that the result came from a verified checker | The plan's provenance, *checked by the front* or *taken as text*; the emitter re-lowers to accept a plan, written into `M4.2` (§1) |
| R1-15 | defect | work handed to leaves whose goals do not hold it, and written into none | Each hand-off listed and written into its receiving leaf with the record (§4) |
| R1-16 | defect | `no-lock-data` moved to a leaf that writes no lock data | To `M4.7`, with the whole lock data and `--locked`; the code comments moved with the rows (§4) |
| R1-17 | defect | the format had no grammar | A canonical-text block, its orders and its forms (§3) |
| R1-18 | defect | a golden "never rewritten" cannot hold the engine's version and the checker's sha256 | The text holds the semantic inputs alone; the provenance beside it; the golden from frozen inputs (§2.1, §3) |
| R1-19 | defect | released "after the last step" does not establish `releases-after-initialisation` | The release epoch, the first enabling of interrupts; the image's half `M4.10`'s (§2.7) |
| R1-20 | remark | refusing a period not whole in ticks brings S0's restriction back at tick grain | Exact nominal releases, compares at their ceiling, recommended to `M4.2` (§2.2) |
| R1-21 | remark | "no consumer re-derives the index" clashes with `from_eadl_ranks` | Both carried; the emitter calls `from_eadl_ranks`, a test holding the two equal (§2.2) |
| R1-22 | remark | the scheduling checker would share the generator's reader | Its own reader held to the golden, written into `M2.21.2` (§3, §4) |
| R1-23 | remark | records selected, masking and nesting, a source with no separation, and region ownership omitted | Records with `M3.7`; nesting excluded by the variant's condition 5; `unknown` separation ranked last; an ownership section (§2.5, §2.9) |
| R1-24 | remark | "the realization" named no leaf, and two claims overreached | Each discharge names its leaf; the claims narrowed to the bindings (§1, §2.10) |
| R1-25 | remark | the JSON rationale was false: `archogen-result/1` is canonical JSON | The true reason: the plan quotes eADL values and `/2` lines as written |
| R1-26 | remark | durations as canonical text keep their spelling; absent jitter is zero | One reduced rational of seconds; jitter written as zero (§2.2) |
| R1-27 | remark | the topological order, instance names, a step, the release trigger, the program input and the reader's result were ambiguous | Each fixed (§1, §2.2, §2.7, §3) |
| R1-28 | remark | style: an unsourced claim, a misattributed phrase, an ambiguous "record §9" and an uncited claim | Sourced, attributed, named the adapters record, cited |

## Round 2 — `2026-10-27`: not acceptable, 25 findings

The record then refused a cycle as a conflict of the assignment, put the first release at the first enabling, listed
an adapter's cost on external sources only, had no target, wrote no hand-off into a leaf, and gave a grammar without
absent cases or lexical rules.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R2-1 | defect | the hand-offs written into no receiving leaf | A machine-read ledger, `RTP-H1`–`RTP-H7`, each quoted word for word by its leaf, `HANDOFF-LEDGER` holding them (§4) |
| R2-2 | defect | catalog facts sent to `M3.7`, and the application inputs' format to `M4.10`, whose scopes do not hold them | The catalog facts to `M4.10` as an amendment of the catalog record's §12 and §14.4 (`RTP-H6`); the format to a new leaf, `M4.12` (`RTP-H5`) |
| R2-3 | defect | the enabled set contradicts rule 8's "and no other" | Rule 8 amended to the plan's enabled set (`M4.1.2`, §2.5) |
| R2-4 | defect | an adapter's cost had no carrier on a timer route, where the `/1` kinds put it | Costs written on the release timer's or the timer route's line; a cost at the release timer refused by the analysis's condition 6; the adapters record's §9 and its faces amended (§2.5) |
| R2-5 | defect | the release-timer refusals made at lowering after `resolve` said `ok`; the "none" case not conditioned; a task's own binding unstated | Several devices written `unknown`, left to the analysis and the emitter as rule 8 leaves them; "none" a refusal only with a target, which the description check never sees; a task's own bindings counted (§1, §2.5) |
| R2-6 | defect | the rule 12 amendment contradicts `M3.3.7`'s reviewed reading | Rule 12 kept; a component one step, its members in name order; no member calling another an obligation, refused only where a record states such a call (§2.7) |
| R2-7 | defect | a first release at the first enabling cannot be written to the compare before that enabling | The epoch `E`: the counter read plus a catalog bound on the path to the first enabling, rounded up to a tick; timer-released tasks alone (§2.7) |
| R2-8 | defect | the source trigger lacked "every arrival" or "some", and the case of several or indirect sources | `(source NAME every|some)`; reached through `uses` and `needs` at any depth; several, or no separation, `unknown` (§2.2) |
| R2-9 | defect | contexts missed alarms; the no-wait rule handed to the catalog alone; the set of lines undefined | `(interrupt NAME)` for timer routes too; the no-wait rule handed to the emitter as well (`RTP-H2`); the lines defined (§2.6) |
| R2-10 | defect | no boot stack, a guard counted twice, ranges and refusals undefined, the image's size owed for ever, one interrupt-stack figure | The boot stack; a guard beside its stack, once; the allocation's ranges, order and refusal; the image's end an `M4.4` check; the interrupt stack derived (§2.8) |
| R2-11 | defect | the stack bound's evidence category not carried | Owed beside the bound (§2.2, the owed table) |
| R2-12 | defect | §7.3's per-source cost, masking and nesting not stated | `(interrupt-model (nesting no) (masking all))`; each source's service cost owed (§2.5) |
| R2-13 | defect | the owed application inputs omitted the masked runs and task facts | Owed, and in `RTP-H5`'s format (§2.2) |
| R2-14 | defect | the plan had no target, so `periodic-three` got no memory section | A target as an optional input, in the identity, laid out in its placement memory when the system has none (§1, §2.8) |
| R2-15 | defect | other places naming `M4.1` neither discharged nor moved | Each listed and moved by `.5`, or kept where it states this leaf's role truly (§4) |
| R2-16 | defect | the grammar left absent cases, operand forms, orders and lexical rules open | Each defined; the heartbeat's plan written out whole (§3) |
| R2-17 | defect | the index rows contradicted the record and the history's counts | Both corrected |
| R2-18 | remark | the heuristic's claims: rate-monotonic, not deadline-monotonic; the non-optimality misstated | Restated, with the primary account ledgered (`davis-speedup-factors`) and Audsley's algorithm named as the optimal one, out of the generator's role (§2.5) |
| R2-19 | remark | `compose.rs` refuses more than a non-strict order; deliverability stretched to the timer | Both corrected (§2.5) |
| R2-20 | remark | "assumes no extent" inaccurate | "the conservative extent, the largest the region allows" (§2.8) |
| R2-21 | remark | ownership missed regions of unbound platforms | Every region of the hardware, as `Joint::map` reads it (§2.9) |
| R2-22 | remark | instance lines lacked their obliged values and their costs' terms | Obliged values written; cost terms owed by the realization (§2.4) |
| R2-23 | remark | a rule change would rewrite a golden "never rewritten" | §2's rules are part of the format: a change is a new format number (§3) |
| R2-24 | remark | provenance lacked the generator's sha256; a claim held only for one way of acceptance | Added; the claim narrowed to each way (§1) |
| R2-25 | remark | `/1` can never be complete; `M4.1`'s goal and `M4.2`'s did not hold what moved to them | Said: `RTP-H5`'s format brings `/2`; the goals widened by the ledger's quotes (§2.2, §4) |

## Round 3 — `2026-10-27`: not acceptable, 32 findings

The reviewer lowered the heartbeat and `adapter-epoch-extender.eadl` on `targets/riscv-virt-up.eadl` by the record's
rules alone. The heartbeat matched the worked text, under two readings the rules do not force. The second system needed
choices in six places. 18 defects, 14 remarks.

**Fence.** One of the reviewer's searches, `grep -rln "executable true" --include=*.eadl .`, walked `docs/evaluation/`.
The sealed text is not in the working tree (that folder's README), the search matched nothing there, and no command read
history. No case was read.

**Method, after three rounds that did not converge** (18, 17 and 18 defects): the record designed questions other
leaves own. Those are how timer-released tasks and an adapter share an alarm (the joint rules'), how a system's devices
correspond to a target's (`M4.4`'s), and sizes only catalog records state. The answer, `2026-10-28`:
- narrows format `/1` to the hosted playground, where every rule is determined today;
- moves the target, the memory layout, the epoch on a target and stack allocation to `/2`, a later child of `M4.1`
  (`M4.1.7`), after `M4.4`'s correspondence (`RTP-H8`);
- gives the release alarm a design of its own (`M4.1.6`);
- pins `/1` by two complete worked texts that become its goldens, the heartbeat and the epoch extender, hosted.

The findings moved to `/2` are kept here as `M4.1.7`'s questions.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-1 | defect | the target names a timer, and the timer rule ignores it; nothing relates a system's devices to a target's | No target in `/1`. The correspondence is `M4.4`'s (`RTP-H8`), the target timer `M4.1.6`'s and `M4.1.7`'s (§4) |
| R3-2 | defect | `Joint::map` cannot be run over a target, which has no `defsystem`; its refusals and region choice missing | Moved to `M4.1.7`, with the reviewer's fix as its question (§4) |
| R3-3 | defect | the release-timer rule picks a counter, not an alarm, and ignores channels, so one channel is used twice | In `/1` the release timer is the simulator's clock (§2.5, `RTP-H9`). A device's alarm, shared with an adapter's as channel takers, is `M4.1.6`'s design |
| R3-4 | defect | the context rule gives callers the interrupt rather than callees, and misses instances | Rewritten: a task `job` alone; `(interrupt X)` for the code a route's interrupt runs, an instance's, a requester's through `expiry`, and what they reach (§2.6) |
| R3-5 | defect | no `ITEM` for a timer route, so a second timer folds into `timer` | Each timer route's provider an `ITEM` of its own (§2.5, §3) |
| R3-6 | defect | the condition-6 refusal had no mechanism | No cost can be at the simulator's clock. A timer route is refused by condition 5, "unmodeled-interrupt-load", the checker declaring no source for it (`RTP-H4`, §2.5) |
| R3-7 | defect | `some` is unsound when the source is faster than the task, and wrong when it is slower | `every` when the source's separation is at least the task's; otherwise `unknown`, the trigger owed to the description (§2.2) |
| R3-8 | defect | no catalog record can state the epoch's bound | `/1`'s epoch is the simulator's time zero after initialisation (`RTP-H9`). A target's, read at the transition into idle, is `M4.1.7`'s question |
| R3-9 | defect | the image can overlap the stacks | No memory section in `/1`; the fix is `M4.1.7`'s question |
| R3-10 | defect | the allocation's arithmetic not fixed | `M4.1.7`'s question, with the reviewer's formula |
| R3-11 | defect | the interrupt stack's size leaves out the fault path | `M4.1.7`'s question |
| R3-12 | defect | the owed table missed stack use and route latency | `route-latency` for every route's provider (§2.10). Stack use comes with `/2`'s stacks |
| R3-13 | defect | three obligations handed to no one: a member's calls, waiting in initialisation, judging contexts | `RTP-H6` extended with both image checks and a record's stated calls; `RTP-H10` to `M4.10` for judging contexts once `M3.7` lets a record be a provider (§4) |
| R3-14 | defect | `M4.1.2`'s leaf contradicts the record; `M4.1`'s copy of `M3.2.1`'s obligation not annotated | `M4.1.2` restated as §4's `.2`; the copy annotated |
| R3-15 | defect | `obligation` lines not derivable: `initialisation` produced by nothing, `discharged-by` prose | A table of kind, subject and leaves; `initialisation` dropped (§2.9) |
| R3-16 | defect | §7.5's plan hash handed to no leaf | `RTP-H7` carries the plan's sha256 (§1, §4) |
| R3-17 | defect | duties placed on `M4.3` and `M4.4` in no ledger row | `RTP-H9` to `M4.3`; `RTP-H8` to `M4.4`, since round 4 to its child `M4.4.1`, the correspondence before `M4.1.7`; `M4.4`'s layout checks after `M4.1.7` (R4-D14) |
| R3-18 | defect | the memory-section rules contradict each other | `/1` has no memory section; placement memory refused `unsupported-profile` (§1, §2.8) |
| R3-R1 | remark | the Davis claims overreach; Audsley's algorithm called optimal; the ledger's page count | No optimality claimed. Audsley's algorithm needs Davis and Burns's Condition 1, and the composition's bound depends on the order ahead (§2.5; ledgered as `davis-burns-priority-assignment`). The ledger read eighteen pages already; its URL now redirects, recorded |
| R3-R2 | remark | "ordinary value" misattributed to rule 15 | Attributed to the adapters record's §1, item 5 (§2.4) |
| R3-R3 | remark | `costs` orders contradict; "canonical name" undefined | A node's text defined (§2); each list's order stated apart (§3) |
| R3-R4 | remark | deliverability has no token; `nmi-and-traps` a placeholder | `(deliverable yes)` on each source line; no `nmi-and-traps` on the playground (§2.5, §3) |
| R3-R5 | remark | "no requester, a bound block or platform" contradicts `Kind::Other` | Nodes defined as `Joint::initialization`'s three kinds (§2) |
| R3-R6 | remark | the catalog's numbers lack identity and evidence category | No catalog number in `/1`; `M4.1.7`'s question |
| R3-R7 | remark | the plan keeps steps and drops the graph's edges | `depends` lines with their reasons (§2.7, §3) |
| R3-R8 | remark | `M4.12` has no goal; `M4`'s children line stale | Both written |
| R3-R9 | remark | the boot stack's owners | `M4.1.7`'s question |
| R3-R10 | remark | one phase unsaid; `unknown` with no owed line; services' contexts a provider's | One phase said; `trigger` owed; a service's contexts its realizing record's (§2.7, §2.10) |
| R3-R11 | remark | multiplex read from rule 4 alone | "As `Joint::check` reports it"; the heartbeat owes none (§2.9) |
| R3-R12 | remark | a missing refusal in §1 | Listed (§1) |
| R3-R13 | remark | `M2.21.2` not ordered after `M4.1.4` | `RTP-H4` says so; the leaf too |
| R3-R14 | remark | "charged as a source" is not in the adapters record's §9 | The two places that hold it named, and the record's own wording amended by `.2` (§4) |

## Round 4 — `2026-10-28`: not acceptable, 27 findings

The reviewer re-derived both worked texts byte for byte from their inputs, the joint rules' real output confirming
them, and lowered `adapter-deadline-over-delay.eadl` by the rules as well. The defects lay at the edges the two texts
did not reach, found with five probe systems: a provider raising both kinds of route, a system with only a sporadic
task, a multiplexed alarm, a `deadline-over-delay` instance with no bound, and a block named `timer`. 16 defects, 11
remarks.

**Fence.** The reviewer disclosed one recursive search of `*.md` under `docs/` that walked `docs/evaluation/`, its
output filtered so that nothing from that folder was printed or opened, and two commands that listed every tracked path
before filtering, printing none of that folder's. Every history read stayed within `6d61f65..HEAD`. No case was read.

**Method.** The answer pins the edges as it pinned the middle: a third worked text, a system released by a source, and
the five probe systems made goldens of `M4.1.4`.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R4-D1 | defect | interrupts named by provider alone collide, a provider with both route kinds or a block named `timer`, and the timer route then escapes condition 5 | Each interrupt named by its kind: `timer`, `(timer-route NAME)`, `(source NAME)`, in `enabled`, contexts, obligations and owed lines; `RTP-H4` gives each its own identifier (§2, §3) |
| R4-D2 | defect | a system with no timer-released task is refused by condition 5, the analysis declaring `timer` always | The release timer always written and enabled, never taken when nothing is due (§2.5) |
| R4-D3 | defect | an obliged fact offered bare has no spelling | `(obliged FACT)`, its bound owed by the realization (§2.4, §2.10) |
| R4-D4 | defect | how the lowering runs the joint rules unstated, so a source's level undetermined | As the search runs them, at the profile's caller level, instances rebuilt from the catalog (§1); `M4.1.3`'s scope widened |
| R4-D5 | defect | the timer's place among sources on the playground cited to a row that does not hold it | `RTP-H9` states the simulator's `external-before-timer` (§2.5) |
| R4-D6 | defect | the first enabling of interrupts on the playground unfixed | `RTP-H9`: initialisation masked, interrupts enabled at time zero (§2.7) |
| R4-D7 | defect | no one realises a source-released task's trigger, nor handles an unknown one | `RTP-H9` releases on each arrival and no other time; `RTP-H1` refuses an `unknown` trigger `missing-fact` |
| R4-D8 | defect | two-phase initialisation sent to `M3.7` with no hand-off; rule 12 still leaves it to `M4.1` | `.2` writes this engine's answer, one phase, into rule 12; the `M3.7` sentence dropped (§2.7, §4) |
| R4-D9 | defect | the multiplex obligation's analysis half dropped | Discharged by `M4.2 M2.21.2`; the overhead owed by the realization, charged by `RTP-H4` (§2.9, §2.10) |
| R4-D10 | defect | the route row cites `RTP-H1` for a clause it lacks | `RTP-H1` configures the enabled set as the plan writes it |
| R4-D11 | defect | `leaves-interrupt-hardware-alone` filed under each task | Owed once, for the system (§2.2, §2.10) |
| R4-D12 | defect | `Obligation::Placement` attributed to `Joint::check` | To `Joint::map` (§2.9) |
| R4-D13 | defect | an unknown separation never owed; a separation declared absent unhandled | `separation` owed by the description; `(separation absent)` written, its tasks' triggers `unknown` (§2.2, §2.5) |
| R4-D14 | defect | R3-17's answer cites a sentence §4 lacks; `M4.4` and `M4.1.7` order each other | `M4.4.1`, the correspondence, before `M4.1.7`; `M4.4`'s layout checks after it, listed in `.7`; R3-17's answer corrected |
| R4-D15 | defect | the moved questions not all listed, and round 3's fixes kept nowhere | `.7`'s list completed; round 3's fixes and round 4's items kept below; rule 8's amendment names the target's non-maskable interrupts and traps |
| R4-D16 | defect | nothing defines what a hosted `/1` build runs as a task's code | Each job an engine-owned empty body that the release trace alone observes, as S0's does, the build saying so (`RTP-H1`, §2.2) |
| R4-R1 | remark | "optimal only for" overstates Davis and Burns | Theorem 1's sufficiency and §4.1's open problem quoted; the ledger's scope widened |
| R4-R2 | remark | the multiplex gloss wider than the code's users | "used directly, tasks or services, as `Joint::check` counts its users" (§2.9) |
| R4-R3 | remark | `RTP-H10` does not say which line is judged against which record | A record's supported contexts judged against each node it serves; reaching passes through bindings, so a provider runs in its requesters' contexts (§2.6) |
| R4-R4 | remark | the format number does not freeze the joint rules | A change to what §2 reads that moves a plan is a new format (§3) |
| R4-R5 | remark | catalog owners are target-shaped on the playground | Owners `realization` and `playground`; `/1` owes the catalog nothing (§2.10) |
| R4-R6 | remark | `s0.md`'s lock-data passages missing from the move | Listed with `no-lock-data` (§4) |
| R4-R7 | remark | `M4.1.3`'s scope too narrow | The record's bindings read back with instances and beneath bindings, the joint rules run over them (§4) |
| R4-R8 | remark | the interrupt model has no realiser | `RTP-H9` nests no interrupt and masks all in a masked section |
| R4-R9 | remark | no owed line for the stack | Each task's stack bound and its category owed again; the allocation `/2`'s (§2.2) |
| R4-R10 | remark | `RTP-H4` silent on unknowns | Each mapped to an input the analysis lacks, never a default |
| R4-R11 | remark | refusals for a foreign system and an unsupported profile missing | Both listed (§1) |

## Round 5 — `2026-10-28`: not acceptable, 18 findings

The reviewer re-derived all three worked texts with a scratch program calling the real search and joint rules, each
byte for byte, and lowered eighteen probe systems. 8 defects and 10 remarks, all at edges the goldens did not reach: a
requester that is no node, a task's `needs`, a timer route's place, the analysis's inputs on the playground, a deadline
reference other than the release, and multiplexing across systems. No fence was broken: every search excluded
`docs/evaluation`, and the one history read stayed within `6d61f65..HEAD`.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R5-D1 | defect | a route naming a task or the system's level: the context rules conflict, and an expiry to a task is undefined | The interrupt's provider runs in it itself; a task or the system's level adds no code; an instance serving either with `expiry` refused, how a job receives one `M4.1.6`'s (§1, §2.6) |
| R5-D2 | defect | a task's `needs` left out of reaching and of its `uses` items | A requester reaches what it names by `uses` or `needs`, as the joint rules take its names (§2, §2.2) |
| R5-D3 | defect | the analysis's platform inputs owed by no one on the playground; the opening contradicted §2.10 | The opening narrowed and a `/1` analysis said inconclusive by itself; the simulator states its model's platform figures (`RTP-H9`); each source's acknowledge point and deferred work owed; `route-latency` dropped, the variant having one `δ` |
| R5-D4 | defect | a timer route has no place in the playground's order | One ranked `(route …)` line for every route, a timer route ranked with the sources as a device alarm is in the hardware's order (§2.5, §3) |
| R5-D5 | defect | a timer route's service cost and arrival bound owed by no one | Both owed by the realization; their charge `M4.1.6`'s (§2.10) |
| R5-D6 | defect | a multiplexed service's users counted across systems | `.2` amends rule 7's counting, in the generator's joint rules and the checker's, to the system's own users; the probe a golden (§2.9, §4) |
| R5-D7 | defect | the multiplex overhead has no term to charge and cannot be reached in `/1`; a node's call cost owed by no one | The overhead's term `M4.1.6`'s; `call-cost` owed for each service, policy and instance node; `RTP-H4` reports such inputs inconclusive |
| R5-D8 | defect | a `deadline-from` other than `release` undetermined; `unstated` a symbol a description can write | Another event refused; none written is `(deadline-from)`, which no description can write (§1, §2.2, §3) |
| R5-R1 | remark | "computes nothing" for the release timer overstated | Only `Δ_timer` is skipped; `C_rel` still enters (§2.5) |
| R5-R2 | remark | `Joint::map` said to run over bindings | Over the system (§1, `.3`) |
| R5-R3 | remark | `Joint::map` exposes no regions; the form dropped executable and reserved | Regions moved to `/2`, with both flags (§2.8, `.7`) |
| R5-R4 | remark | three items lost from `/2`'s list | The target in the identity, compares at their ceiling, each source's controller line and identifier added to `.7` |
| R5-R5 | remark | `RTP-H5` omits `leaves-interrupt-hardware-alone` | Added |
| R5-R6 | remark | "inputs the analysis lacks" has no representation | The analysis reported inconclusive, the input named, never run (`RTP-H4`) |
| R5-R7 | remark | `periodic-three` cited though it does not resolve | The citation left with the regions |
| R5-R8 | remark | a provider takes every requester's contexts, an over-approximation | Said: it can refuse, never accept wrongly (§2.6) |
| R5-R9 | remark | a sporadic trigger is an inference | Said, and the build reports it (`RTP-H1`) |
| R5-R10 | remark | more goldens wanted | The six probes named in §3 beside the three texts |

## Kept for `M4.1.7`

The questions the narrowing moved to format `/2`, with the fix each reviewer proposed, so that none lives only in a
transcript:
- **The target's timer** (R3-1): with a target, take the release timer from the target's tick-rate block when the
  system binds none; where the system binds its own device, refuse until a stated correspondence between system device
  and target device exists (`M4.4.1`).
- **`Joint::map` over a target** (R3-2): a target file has no `defsystem`, so `System::read` cannot read it. Define
  the target's hardware as every block and platform its file declares, run `Joint::new` and `Joint::map` over it, list
  its findings (`MapContradiction`, `PlacementUnsized`, `MemoryAllHoles`) as target refusals, and pick a placement
  region by rule, such as the lowest base.
- **The epoch** (R3-8): read the counter and write the first compare as the last act before the transition into idle
  (`starts-by-transition`), so that the bound from the read to the first enabling is the switch record's.
- **The image and the stacks** (R3-9): the image ends at or below the lowest allocation's base in its range, which
  `M4.4` checks.
- **The allocation's formula** (R3-10): a unit is `[base, base + guard + bytes)`; the stack's top is aligned down from
  the cursor; tables carry `(guard 0)`; an owed unit makes every later unit owed; the cursor only descends.
- **The interrupt stack** (R3-11): the largest service's stack use plus the fault path's use and one more trap frame,
  the fault path owed to the switch record.
- **The memory section** (R3-18): written where a target is given, or the system has placement memory.
- **The catalog's numbers** (R3-R6): their records in the identity, and each number's origin and evidence category, as
  §7.3 asks of every bound.
- **The boot stack** (R3-R9): the port record calls the stack set-up the image's, and the boot stack holds the
  application's initialisation too, so its size has two owners.
- **What the narrowing dropped** (R4-D15): the platform's non-maskable interrupts and firmware traps in a target's
  enabled set; the source ranks mapped onto the controller's levels, or a refusal naming the level count; the guard,
  frame, alignment and table sizes owed to the catalog; a generated service's stack use; the "no room" refusal; `M4.4`
  naming an unsized hole that truncates an image; how the command line names a target.
- **What round 5 found unkept** (R5-R4): the target in the plan's identity (round 2's answer); each compare written
  at the ceiling of its release (round 1's, condition 6's "rounds up"); each source's controller line and identifier,
  for the priority writes. And with them the memory map's regions, moved from `/1` in round 5, each with whether it is
  executable or reserved.
