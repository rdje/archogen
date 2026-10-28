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

## Round 6 — `2026-10-28`: not acceptable, 20 findings

The reviewer re-derived all three worked texts with a program calling the real search and joint rules, the installed
command's records identical to the program's, and lowered 37 probe systems. 10 defects and 10 remarks, each with a narrow
fix, all at edges the goldens did not reach: a tie in rank, the timer routes' place, the trigger's reading beside a
timer route, an alarm a task programs itself, multiplexing by `needs`, a provider's call cost, and the two roles
disagreeing.

**Fence.** One search, `grep -rln … --include=*.md docs`, walked `docs/evaluation/` before its output was filtered;
nothing from that folder was printed or opened. Every history read stayed within `6d61f65..HEAD`. No case was read.

**Method.** Six rounds have found 18, 17, 18, 16, 8 and 10 defects, each reviewer rebuilding the lowering by hand in a
scratch program to re-derive the texts. That is the pattern `decision_executable-design-reviews.md` answers: a design
whose subject is a decision procedure is reviewed with its executable form. So, before round 7, `M4.1.3` and `M4.1.4`'s
lowering are built as the record's transcription, rule by rule, and every probe of rounds 4 to 6 becomes a fixture with
the text the record gives it. Round 7 reviews the record, the code and the corpus together, and a later finding is a
defect when it carries a reproducer.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R6-D1 | defect | route ranks undetermined for two sources of one separation, round 5's answer having dropped the tie-break | Ties broken on the interrupt's text (§2.5) |
| R6-D2 | defect | the timer-route rule is the alternative the record rejects: a `programmed` separation ranks after every source | Said so: the timer routes ranked last in `/1`, their key `M4.1.6`'s; the alternative and the hardware quote reworded (§2.5) |
| R6-D3 | defect | the trigger reads two ways when a timer route also reaches a sporadic task | "Several sources' routes"; a timer route releases no task (§2.2) |
| R6-D4 | defect | a task or the system's level programming an alarm itself is not refused; "the interrupt releases the task" false | Refused beside the expiry case, how a job receives either `M4.1.6`'s; the sentence reworded to release only where the trigger names `X` (§1, §2.6) |
| R6-D5 | defect | multiplexing ignores users naming the service by `needs` | `.2` counts users by `uses` or `needs`; the probe a golden (§2.9, §4) |
| R6-D6 | defect | `call-cost` not owed for a provider node running in a job | Owed for every node whose context line holds `job` (§2.10) |
| R6-D7 | defect | no refusal when the generator's joint rules reject the accepted record | `tool-failure`, naming both verdicts (§1) |
| R6-D8 | defect | the instance bullet also puts an instance in a source's interrupt at the same provider | "For a timer route at `X`" (§2.6) |
| R6-D9 | defect | `RTP-H6` keeps the route latency round 5 dropped; `RTP-H9` names one platform fact of many | The latency dropped from `RTP-H6`; `RTP-H9` states every platform input and fact the variant and the composition read, `releases-after-initialisation` among them |
| R6-D10 | defect | `M4.1.5`'s text holds less than §4 gives it; `.7` cites `RTP-H6` for sizes it does not hold | `M4.1.5`'s text widened; the sizes owed by a hand-off `.7` writes |
| R6-R1 | remark | a task's `uses` items hold services only, and a name in both lists twice | Services and policies, each once (§2.2) |
| R6-R2 | remark | "the joint rules as they stand" contradicts §2.9 until `.2` | "As `.2` amends them" |
| R6-R3 | remark | "both writers" against "one writer" | "Its writer" (§3) |
| R6-R4 | remark | the Theorem 1 quote elides its multiprocessor scope | Condition 1 quoted from its general statement in the 2016 review, ledgered; Davis and Burns kept for the open problem alone |
| R6-R5 | remark | `multiplex` discharged by a design leaf | By `M4.2 M2.21.2`, `RTP-H4` charging the term `M4.1.6` designs |
| R6-R6 | remark | the execution bound's evidence category not owed | `execution-category` owed and in `RTP-H5` |
| R6-R7 | remark | `M4.1.1`'s status stale | Updated |
| R6-R8 | remark | `RTP-H9`'s register mapping has no input; no source before the first enabling | The mapping dropped; arrivals only after the first enabling |
| R6-R9 | remark | the vocabulary in neither the identity nor what §2 reads | Named among what §2 reads (§3) |
| R6-R10 | remark | ten more probes for the goldens | Named in §3 |

## Round 7 — `2026-10-28`: not acceptable, 19 findings

The first round with the executable form. The reviewer ran the crate's tests, the worked texts and the corpus green,
then built a probe calling the language's check, the search, the `/2` writer, the configuration checker and the
lowering, and ran 19 probe systems through it. 9 defects and 10 remarks, each with a reproducer under the reviewer's
scratch folder; three serious: a frequency read as seconds, multiplexing lost through a service in between, and a
binding the checker forbids. That last one was the search's, `M4.1.8`'s class, and `M4.1.9` fixed it at once.

**Fence.** One search, `grep -rln "defadapter\|adapter-kind" --include=*.eadl .`, walked `docs/evaluation/` before its
output was filtered; it printed three paths outside that folder, and nothing from it was printed or opened. The one
history read was `git log --oneline 6d61f65..HEAD`. No case was read.

**Method.** The executable form did what `decision_executable-design-reviews.md` asked of it: each defect came with a
system and two texts, and none needed a ruling on what a sentence meant. The corpus, though, held what the code wrote,
so a fixture contradicting the record stayed green (D4, D8): its harness now runs the language's check first, and its
header says that an outcome is evidence, judged against the record, never a rule.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R7-D1 | defect | a task's period, separation, deadline or jitter of any dimension read as seconds: `(period 100 Hz)` lowered to `100 s`, `(deadline 10 kHz)` to a deadline longer than its period | Each must be a time; another dimension refused `tool-failure` (§1, §2.2), and `M1.42` filed to make the language refuse it |
| R7-D2 | defect | `multiplex` lost where two tasks reach a channel through a service or a policy: §2.9 counted the declarations naming the service, §2.6 and §2.10 run what a policy uses | A service is multiplexed where it takes a channel and two or more of the system's tasks reach it, as [OSEK](../book/src/ledger.md#osek-os) §8.5 counts a resource's users by the tasks that access it; the lowering derives it so, `.2` amends rule 7 to match (§2.9, §4) |
| R7-D3 | defect | the search and the rebuild bound `reachable-at-privilege`, which the checker forbids | The search's, not the record's: `M4.1.9` |
| R7-D4 | defect | a task's `uses` list held facts and blocks, against §2.2 and §3 | The code filters to services and policies, four fixtures re-blessed (`M4.1.4`); §2.2 says a fact or a block named is no member |
| R7-D5 | defect | the program's digest and declarations taken from the caller, so a plan could name one program and hold another's task | `lower` takes the `Program` and derives both (§1, `M4.1.4`) |
| R7-D6 | defect | a system with no task lowered, and `RTP-H1`'s scheduler has no empty form | Refused `unsupported-profile` (§1) |
| R7-D7 | defect | §2.6's node bullet ignored the route's kind, putting a source's handler in its provider's timer route | The interrupt fact where the route is a source, a consumed fact where it is a timer route (§2.6, `M4.1.4`) |
| R7-D8 | defect | `dup-prio`, which the language's check refuses three times over, pinned as a plan | The corpus runs the language's check first and records its verdict; `dup-prio` off the open edges (`M4.1.4`) |
| R7-D9 | defect | a refusal of the joint rules named neither verdict | Names the checker's acceptance and each of the generator's findings (§1, `M4.1.4`) |
| R7-R1 | remark | the corpus pins the code, not the record | Its header says so; a review judges an outcome against the record (How to apply) |
| R7-R2 | remark | cases where code and record agree but no fixture holds them | Added to the corpus, with a refusals test of §1's records (§3, `M4.1.4`) |
| R7-R3 | remark | §1 lists no refusal for a record that reads but does not rebuild, and fixes no order among refusals | `tool-failure` for it; refusals taken in the order written, the profile ahead of the rules (§1) |
| R7-R4 | remark | an obliged value that is not a duration is written bare without being owed | Refused `tool-failure` (§2.4) |
| R7-R5 | remark | the expiry refusal reads only what an exclusive instance serves | Also a requester bound to a shared instance (§1) |
| R7-R6 | remark | `RTP-H1` refuses every `unknown` trigger `missing-fact`, though `eadl/1` cannot write some of them | `missing-fact` where the description leaves it out, `unsupported-profile` otherwise (`RTP-H1`) |
| R7-R7 | remark | `RTP-H9` lets a source arrive sooner than the plan's separation | It may not (`RTP-H9`) |
| R7-R8 | remark | `M4.1.5`'s text out of step with §4 `.5` | Synced; the register entry `.4`'s, written with the executable form |
| R7-R9 | remark | the composite-inputs record calls `leaves-interrupt-hardware-alone` "a task fact" against its own row | Not this record's: `M2.23` filed |
| R7-R10 | remark | a golden's input is another suite's file | Each golden's inputs the crate's own fixtures (§3, §4 `.4`) |

Round 6's answers: all met in the record; in the code `R6-R1` was not (R7-D4), and `R6-D5`, `R6-D7` and `R6-D8` partly
(R7-D2, R7-D9, R7-D7), each now answered as above.

## Round 8 — `2026-10-28`: not acceptable, 10 findings

The reviewer ran the crate's tests and the checker's sweep green, adapted round 7's probe program, and found the code
transcribing §2–§3 faithfully on every fixture and on probes of its own. 4 defects and 6 remarks: a rule wrong against
its own source, a hand-off naming the wrong model rule, a zero release separation lowered, and the record's reader
accepting records the checker refuses.

**Fence.** No breach. Every search was scoped away from `docs/evaluation/`; the history reads stayed within
`6d61f65..HEAD`.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R8-D1 | defect | §2.9 dropped a channel's callers that run in an interrupt, and policies: a source's handler and a task sharing an alarm, two handlers sharing one, a policy taking one, each owed no `multiplex`, while OSEK's §8.6 counts "all tasks and interrupt routines" | A service or a policy taking a channel is multiplexed where it has two users: each task that reaches it, and each interrupt whose context line holds it, but its own channel's timer route; [OSEK](../book/src/ledger.md#osek-os) §8.6 quoted (§2.9, `M4.1.4`) |
| R8-D2 | defect | `.2` amended rule 7 for the users, which rule 4 defines | `.2` amends rule 4's users; rule 7 still resolves system by system (§4, `M4.1.2`) |
| R8-D3 | defect | `rebuild` accepted an instance no requester is bound to, and a binding to another requester's exclusive instance, both of which the checker refuses | `M4.1.10`: `rebuild` refuses both, the records held in `tests/records.rs` |
| R8-D4 | defect | a zero period or minimum separation lowered, which no run can release | Refused `tool-failure` (§1); `M1.42` widened to make the language refuse it |
| R8-R1 | remark | a periodic task bound to a source itself lowered with the binding meaning nothing | A second release model: refused `tool-failure` (§1), `M1.43` filed for the language |
| R8-R2 | remark | refusals the code applies that §1 does not list, or that no fixture can reach | A release or deadline none stated folded into §1's bullet; §2.4's refusal placed in §1's order; R7-R5's shared-instance branch kept as the rule, unreachable under `adapters/1` |
| R8-R3 | remark | `INDEX`, `PROFILE`, `VERSION`, `SYSTEM` and the `bind` line's tokens undefined | Defined (§3) |
| R8-R4 | remark | three wording slips | Fixed (§2.7, §2.9, §3) |
| R8-R5 | remark | the sporadic golden read from the record's block, not a crate fixture | The corpus's `sporadic.eadl` held equal to the block (`M4.1.4`) |
| R8-R6 | remark | cases the corpus misses, code and record agreeing | Added, and a module tree (§3, `M4.1.4`) |

Round 7's answers: held in both, but R7-D2, partly (R8-D1, R8-D2); R7-R4 and R7-R5 are rules no fixture can reach under
`adapters/1`.

## Round 9 — `2026-10-28`: not acceptable, 15 findings

The reviewer adapted round 8's probe and found the code transcribing §1–§3 faithfully on every section it probed,
co-applying refusals in the record's order among them. 7 defects and 8 remarks: a P1 shared by the search, the record's
writer, the reader and the lowering; three in §2.9's exception and its restatements; the owed application inputs after
`M2.27`; the reader judging no binding by the relation; and a duty given `M4.9` outside the ledger.

**Fence.** No breach. The sweep it ran listed the sealed set's paths before filtering them, printing nothing from it
(remark 8, fixed in the listing by `M4.1.12`).

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R9-D1 | defect | P1: a task named `SYSTEM/TASK`, a symbol a declaration may share; a service `sl/a` collided with task `a` of `sl` in the search, the writer, the reader and the lowering | `M4.1.11`: a task named `(task SYSTEM TASK)`, as the record and the checker write it; three fixtures |
| R9-D2 | defect | §2.9's own-route exception drops a user where another node that route runs calls the service | The exception holds only where nothing else that route runs reaches it (§2.9, `M4.1.4`) |
| R9-D3 | defect | `.2`, `M4.1.2` and the book restate the users without the exception | The exception quoted word for word in each |
| R9-D4 | defect | the code takes the route an instance programs as its requester's own, which §2.9 does not say | Said: "or that the instance whose channel it takes programs" (§2.9) |
| R9-D5 | defect | the owed application inputs miss `M2.27`'s two declarations and a masked run's evidence category | Owed per task, in §2.2, §2.10, §3 and `RTP-H5`, with the code (`M4.1.4`) |
| R9-D6 | defect | the reader rebuilds a binding the relation does not satisfy | `M4.1.12`: `rebuild` judges each binding by the relation |
| R9-D7 | defect | §2.5 gives `M4.9` a duty no ledger row holds | `RTP-H11`, quoted in `M4.9` |
| R9-R1 | remark | the "none stated" refusal is unreachable and cites no leaf | §1 says the language refuses it already, and `M1.42` and `M1.43` cover the other three; the code's message names no leaf (`M4.1.4`) |
| R9-R2 | remark | `Joint::check` counts no instance among a service's users | "The tasks and services naming it" (§2.9) |
| R9-R3 | remark | a zero deadline and a jitter longer than the period lower | Kept: the analysis judges them, not-established; no rule of `/1` refuses them |
| R9-R4 | remark | §3's order of `binding` before `named` never applies | Kept: an order fixed for a case the joint rules may give |
| R9-R5 | remark | no record holds two refusals applying at once | Added (`M4.1.4`) |
| R9-R6 | remark | the corpus's header and `M4.1.4`'s status say "rounds 4 to 7" | Updated (`M4.1.4`) |
| R9-R7 | remark | §3 does not say how many of each line appear | Each form's count stated (§3) |
| R9-R8 | remark | the sweep's `tracked()` filtered the sealed set after listing it | Excluded in the listing (`M4.1.12`) |

Better decision: a task's release source stated rather than inferred, as OSEK 2.2.3 §4.3 has it ("Task activation is
performed using the operating system services ActivateTask or ChainTask"): `M1.44` filed for the language; until it
lands, `RTP-H1`'s report of each inferred trigger stands.

Round 8's answers: R8-D1 and R8-D2 partly held (R9-D2 to R9-D4), R8-D3 held for its two classes (R9-D6 the third),
the rest held.

## Round 10 — `2026-10-28`: not acceptable, 11 findings

The reviewer enumerated three bounded universes of small systems, 16,640 in all, lowered each through the real search,
the checker and the lowering, and judged them against an independent reading of §2.2, §2.5, §2.6 and §2.9. Contexts
agreed on 10,496 systems and triggers on 6,144; multiplexing disagreed on 1,408, every one the same defect, on a node
lying on a `uses` cycle — which nine rounds of hand-made probes had not reached. 4 defects and 7 remarks.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R10-D1 | defect | the own-route exception read only the route's direct runners, so a node reached back through a `uses` cycle from its own route lost a user | The code reads everything the route runs, as §2.6 and §2.9 say; the cycles fixtures (`M4.1.4`) |
| R10-D2 | defect | a duration past the engine's exact arithmetic read as no duration: "not a time", `unknown`, "not a duration" | §2.2 states the range; refused `unsupported-profile`, naming it (§1, `M4.1.4`); `M1.45` filed |
| R10-D3 | defect | §1 gives `M4.1.6` how a job receives an alarm it programs itself, which `.6` and the leaf do not hold, nor the system's own level's requirement | `.6` and `M4.1.6` hold both, in §1's words; the code's message for the system's level reworded (`M4.1.4`) |
| R10-D4 | defect | a source's `one-request-per-arrival` and `no-application-code` owed by no one | Owed by the realization, `external` the simulator's under `RTP-H9`, with the code and the sporadic worked text (`M4.1.4`) |
| R10-R1 | remark | §1's order inside a bullet over tasks undetermined | Task by task in runtime order, each task's clauses in the bullet's order (§1) |
| R10-R2 | remark | a source's zero separation lowered and said nowhere | Written as stated, the analysis refusing it (§2.5); `M1.45` has the language refuse it |
| R10-R3 | remark | §2.7's "one line for each pair of nodes, the earlier first" means nothing inside a component | One line for each ordered pair an edge joins (§2.7, §3) |
| R10-R4 | remark | nine round-9 probes are not fixtures, and §3's list stops at round 8 | Added (`M4.1.4`); §3 lists rounds 9 and 10 |
| R10-R5 | remark | `rebuild` applies no mediation gate, unreachable under `adapters/1` | Kept unreachable: no kind of `adapters/1` mediates, and the gate comes with the first that does |
| R10-R6 | remark | `M2.30` may add an application fact the plan must owe | Held for `M2.30`, which moves §2.2, §2.10 and `RTP-H5` with it |
| R10-R7 | remark | the production half is not built | As declared; no finding |

Better decision, taken: the bounded exhaustive checker `decision_executable-design-reviews.md` item 1 asks for, which
this record never had — `M4.1.13`, built by a context fenced from the lowering's code, so that its reading of the record
is independent of the transcription it checks.

Round 9's answers: R9-D2 partly held (R10-D1); R9-D5 held for tasks, R10-D4 its sibling for sources; the rest held.

## Round 11 — `2026-10-28`: not acceptable, 11 findings

The first round with the bounded checker. The reviewer found every test green, extended the checker in a copy of its
own — the configuration checker called on all 57,277 generated records, §2.7's lines, each task's `uses` item and a
two-task universe, 66,484 plans agreeing — and found 5 defects and 6 remarks: two on the order inside §1's
interrupt-code refusal, a system named by `uses` making a node §2 does not give, a P1 in the search, and a hand-off held
by no leaf.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R11-D1 | defect | §1's interrupt-code refusal was taken clause by clause, not task by task | The refusal names every receiver, each task in runtime order and then the system's own level, so no order decides it (§1, `M4.1.4`) |
| R11-D2 | defect | §1 gave the system's own level no place in that bullet's order | As R11-D1: named last, with every receiver (§1) |
| R11-D3 | defect | a `uses` naming another system makes its own level a node, against §2's "three kinds of node" | Refused `tool-failure` (§1) until `M1.46` has the language refuse it; §2 says `Other` is a policy's (`M4.1.4`) |
| R11-D4 | defect | P1: the search drops a block's own requirements and the reader accepts the record, which the checker refuses | `M4.1.14` |
| R11-D5 | defect | R10-R6's answer gives `M2.30` a duty its text does not hold | `RTP-H12`, quoted in `M2.30` |
| R11-R1 | remark | `M4.1.13`'s goal has each record checked by the configuration checker, which `bounded.rs` never calls | `M4.1.13` reopened for it |
| R11-R2 | remark | the checker's universes are narrower than claimed: one task, no obliged bound, `deadline-from` always written, no two receivers, §2.7 unchecked | `M4.1.13` reopened for them |
| R11-R3 | remark | `bounded.rs` tells the system's own level from a task by substring | `M4.1.13` reopened for it |
| R11-R4 | remark | the corpus's header and `M4.1.4`'s status say "rounds 4 to 9" | Updated (`M4.1.4`) |
| R11-R5 | remark | the order among several durations past the arithmetic is unstated | The refusal names each, in a stated order (§1) |
| R11-R6 | remark | the language's edges are well guarded | No finding |

Better decisions, taken: §1's interrupt-code refusal names every receiver, as its joint-rules refusal names each
finding; and, after `ROADMAP.md` §7.3's "every numerical bound records … evidence category", each obliged bound is owed
by the realization with its evidence (§2.4, §2.10), and a source's stated separation is held by the run on the
playground (§2.5).

Round 10's answers: R10-R1 partly held (R11-D1, R11-D2), R10-R6 not (R11-D5); the rest held.

## Round 12 — `2026-10-28`: not acceptable, 12 findings

The reviewer found every test green, the bounded checker's reading faithful to what it reads, and every hand-off held.
4 defects and 8 remarks, three of the defects from one cause: `M4.1.14` made a block's and a platform's own
requirements requesters in the search and the reader, and nothing in this record followed — what such a requester is in
the plan was never derived, and the bounded checker asserted no block requires anything.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R12-D1 | defect | a block or platform with requirements nothing binds to got no context, step or owed fact, though an interrupt runs its code, against §2.6's "each node" and `M3.3.8`'s "the context of each requester" | Such hardware is a node (§2, §2.6, §2.10); rule 12 counts it (`.2`; `M4.1.15` in the joint rules) |
| R12-D2 | defect | a source separation whose spellings overflow the arithmetic read as none | Refused past the arithmetic, as §1 says (`M4.1.4`) |
| R12-D3 | defect | a block or platform taking a channel with several users owed no `multiplex`, narrower than OSEK §8.6 | Any node taking a channel, an instance aside (§2.9, `.2`, `M4.1.4`) |
| R12-D4 | defect | since `M4.1.14`, reaching followed a block's or a platform's composition `uses` | Reaching excludes it: hardware reaches only what its requirements are bound to (§2, `M4.1.4`); `M1.47` filed for the language |
| R12-R1 | remark | the naming-system refusal reached systems the lowered one does not serve | Scoped to the requesters the system serves (§1, `M4.1.4`) |
| R12-R2 | remark | §3's probe list stopped at round 10 | Rounds 11 and 12 listed (§3) |
| R12-R3 | remark | several durations past the arithmetic, a sporadic task released through hardware, a bare separation: no fixture | Added (`M4.1.4`) |
| R12-R4 | remark | the bounded checker reaches no requiring hardware, no composition, no past-arithmetic value and no second system, and hard-codes jitter, overrun and the deadline's reference | `M4.1.13` reopened for them |
| R12-R5 | remark | §2.5 claimed Audsley's optimality from Davis's Condition 1 alone | Claimed no further: Condition 1 shown, the other two not; soundness rests on `RTP-H4`'s confirmation (§2.5) |
| R12-R6 | remark | `M4.1.14` changed the lowering's inputs without this record | A change to what the lowering reads is re-derived here in the same change (How to apply) |
| R12-R7 | remark | every hand-off quoted word for word | No finding |
| R12-R8 | remark | the code and source claims checked true | No finding |

Round 11's answers: R11-D4 partly held (R12-D1, D3, D4), R11-R2 partly (R12-R4); R10-D2 partly (R12-D2); the rest
held.

## Round 13 — `2026-10-28`: not acceptable, 11 findings

The reviewer found the transcription faithful almost everywhere, every test green, the bounded checker's totals true,
and every hand-off held. 4 defects and 7 remarks, the defects again from round 12's answer: a block's or a platform's
names, called composition, are still read by the engine in places the record did not account for — a `needs` it names
a service by, a service it names that is served only through it, an edge rule 12 drops across that "composition".

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R13-D1 | defect | §2 excluded a block's or a platform's `uses` from reaching but not its `needs`, while the code excludes both | Both are composition, naming only hardware; a name of anything else refused (§1, §2, §2.7) |
| R13-D2 | defect | `.2` and `M4.1.2` gave rule 12's hardware nodes two criteria | `M4.1.2` reads "is bound for requirements of its own" |
| R13-D3 | defect | hardware naming a service by `uses` serves it and drops the edge to the timer it is bound to, inverting the initialisation order | Fail closed, as for `M1.46`: refused `tool-failure` until `M1.47`, widened to `needs` and corrected, has the language refuse it (§1, `M4.1.4`) |
| R13-D4 | defect | the timer routes' place in the order is handed to `.6`, which does not hold it | `.6` and `M4.1.6` hold it, against the release timer too |
| R13-R1 | remark | the naming-system refusal's order ambiguous, and the checker holding none | By the namer's text, then the system's (§1); `M4.1.13` reopened to hold it |
| R13-R2 | remark | the checker's reading departs from the record where its universes never reach | `M4.1.13` reopened |
| R13-R3 | remark | the checker compares lines kind by kind, dropping unknown heads and the order across kinds | `M4.1.13` reopened |
| R13-R4 | remark | four agreeing cases unfixtured | Added (`M4.1.4`) |
| R13-R5 | remark | the corpus's header and `M4.1.4`'s status carry a round range gone stale a third time | The range dropped: every round's probes, whatever the round (`M4.1.4`) |
| R13-R6 | remark | an obliged `State::Overflow` branch is unreachable | Kept, a refusal for a kind whose offers could overflow |
| R13-R7 | remark | `external-before-timer` applied to timer routes, though it speaks of external interrupts | The sources' alone; a timer route's place is `.6`'s, the simulator taking it first until then (§2.5, `RTP-H9`) |

Better decision, taken: refuse an operand that means something to some rules and nothing to others, rather than give
it a meaning rule by rule.

**Corrected with the answers' code** (`ARCHOGEN-M4-1076`): R13-D1's answer called both of hardware's lists
composition. The bounded checker's third widening found the code keeping an initialisation edge to a block a `needs`
names, and model §8 rule 5 sides with it — "`uses` alone is composition". The record now says so: a block's or a
platform's `needs` brings a block into the system's hardware (rule 7) without composing it, and neither list is a call.

Round 12's answers: R12-D1 held but for `M4.1.2`'s wording (R13-D2); R12-D4 partly (R13-D1, R13-D3); R12-R4 partly
(R13-R1 to R13-R3); the rest held.

## Round 14 — `2026-10-28`: not acceptable, 12 findings

The reviewer found the transcription faithful but in §1's messages, every test green, the bounded checker's totals
true — 105,226 systems, every record the search wrote for them accepted by the configuration checker — and every
hand-off held but `M1.47`'s text. 3 defects and 9 remarks: an order no fixture and no universe reached, names one of
which prefixes another; a sentence left of round 13's D2; and round 13's correction short of two places.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R14-D1 | defect | §1's two naming refusals sort their messages, not their texts: a closing backquote sorts after `.`, so `y.b` is named before `y` | Sorted by the texts, then worded (`M4.1.4`); §1 states it of every refusal naming several things, and the receivers' and the obliged bounds', sorted by their words too, get their orders (R14-R6); `M4.1.13` reopened |
| R14-D2 | defect | §2 gives nodes two criteria: "every requester the system serves is a node but its tasks and its own level" beside "bound for requirements of its own" | The first goes: a block or a platform bound for nothing runs no code the plan places, and is no node |
| R14-D3 | defect | round 13's correction missed `M1.47`'s text and the refusal's message, which call `needs` composition, and §1, which credits rule 1 with a restriction it does not state | `M1.47` and §1 state rule 1's `uses` and rule 7's `needs`, the restriction this record's reading; the message "name only hardware" (`M4.1.4`) |
| R14-R1 | remark | the checker's universes reach no prefixed name, no system named on hardware the system does not serve, no platform's `needs` naming a service or `uses` naming a policy | `M4.1.13` reopened; the agreeing probe a fixture (`M4.1.4`) |
| R14-R2 | remark | §3's list of probe systems stops at round 12 | Round 13's added; round 14's with its code |
| R14-R3 | remark | the book says the checker generates ten universes, and it has thirteen | The book names no count, which each widening would stale |
| R14-R4 | remark | the users rule's comment contradicts its code, which the record supports | The comment corrected, the case a fixture (`M4.1.4`) |
| R14-R5 | remark | the periodic-source refusal names no clause | It names the task's `period` (§1, `M4.1.4`) |
| R14-R6 | remark | within one receiver the order of what it receives is unstated, the code sorting its words | Its alarms, then its expiries, each by its text (§1); one instance's obliged bounds by the fact's text, found beside it |
| R14-R7 | remark | "Kept for `M4.1.7`" keeps a fix for the target's timer the record rejects | The item says so, the question `.6`'s; `M4.1.6` points to it |
| R14-R8 | remark | `RTP-H9` does not say how a source with no separation the plan states arrives | Only as the run's scenario states, named beside its result (`RTP-H9`, requoted in `M4.3`) |
| R14-R9 | remark | not this record's: a separation in `Hz` passes the check, and the search refuses it | `M1.48` filed |

Kept, not taken: the reviewer's better decision, that the bound counting each source ahead at its no-loss limit meets
the other two conditions of Davis et al.'s §V as well, so that the Audsley search `RTP-H4` asks for is optimal for it
where the tasks a source releases read that limit too — sources below one reaching its bound only through `L`, and a
source moved up losing a term. A claim of optimality is derived and tested before it is made: `M2.21.2` keeps it.

Round 13's answers: R13-D1 held but for `M1.47` and the message (R14-D3); R13-D2 partly (R14-D2); R13-D3 held, its
order wrong (R14-D1); R13-R1 partly (R14-D1, R14-R1); R13-R5 held but for §3 (R14-R2); the rest held.

**With the answers' code** (`ARCHOGEN-M4-1078`): one receiver receives at most one alarm or one expiry, the search
binding a requester's `counter` facts to one provider whole and refusing `missing-fact` otherwise, so R14-R6's order
within a receiver is reached by no resolved system; it holds as stated. The checker's fourth widening found the code
and its reading apart once, and the record sided with the code: a system named serves what it writes at its own level,
its `platform` clause included (model §8 rules 1 and 7), which §1 now says.

## Round 15 — `2026-10-28`: not acceptable, 13 findings

The first round to find the code faithful everywhere: every defect is in the record's text. Every test green, the
bounded checker's totals true — 110,144 systems, 109,678 records the configuration checker accepted, no disagreement —
and every hand-off quoted word for word. 4 defects and 9 remarks: a sentence of round 14's answer, two refusals stated
apart from the binding §2.6 reads, and two hand-offs written in prose, which no check sees.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R15-D1 | defect | §2's "nor is a block or a platform … bound for nothing" makes the heartbeat's `console.uart`, a provider, no node | Such hardware is a node only as a provider a binding names |
| R15-D2 | defect | the receivers' "bound to its provider itself" and the periodic task's "bound to an interrupt source itself" name no fact, where the code, §2.6 and model §8 rule 2 read one | A binding that takes a channel, as §2.6 reads it; a binding for `interrupt-source` met by `true`; the probes fixtures (`M4.1.4`) |
| R15-D3 | defect | §1 credits `M4.10` with verifying routes, multiplexing and initialisation, `M3.5` says `M4.4`, and neither holds it | `RTP-H13` to `M4.10`; `M3.5`'s owner line corrected |
| R15-D4 | defect | §1 says `M1.47` writes the reading into the model, and its goal does not | `M1.47`'s goal writes model §8 rules 1 and 7, carried as `RTP-H18` |
| R15-R1 | remark | the joint rules' findings have no stated order | Conflicts, unknowns, the map's refusals and unknowns, each in the joint rules' order (§1) |
| R15-R2 | remark | the naming-system refusal takes what the system serves, the hardware refusal everything | Both take what the system serves, as the bullet over tasks does (§1, `M4.1.4`); `M4.1.13` reopened |
| R15-R3 | remark | the users rule's outer comment still reads "a service or a policy" | Corrected (`M4.1.4`) |
| R15-R4 | remark | §2.5 credits rule 8 with "and kind" | Attributed to `joint.rs`; `M4.1.2` writes it into the rule |
| R15-R5 | remark | four reading branches no universe reaches: a platform naming a system by `uses` or `needs`, a job-context node naming another by `needs` | `M4.1.13` reopened |
| R15-R6 | remark | three agreeing cases unfixtured | Added (`M4.1.4`) |
| R15-R7 | remark | §2.7's one-step components were justified for services, not re-derived for hardware and instances | Re-derived: an alarm is run-time work, initialisation waits on no interrupt (§2.7) |
| R15-R8 | remark | How to apply names `.2` to `.7` alone | The leaves from `.8` on named |
| R15-R9 | remark | `rebuild` keeps a placement's region, not its provider | Kept for `M4.1.7` (below) |

Better decisions, taken: every hand-off to a leaf beyond `M4.1` in the ledger — `RTP-H13` to `RTP-H18`, the language
leaves `M1.42`, `M1.43`, `M1.45`, `M1.46` and `M1.47` among them — so that `HANDOFF-LEDGER` holds each
(`decision_executable-design-reviews.md` item 4); §1's two refusals stated by the binding §2.6 and model §8 rule 2
read, never restated (its item 5). The scope taken for R15-R2 is the narrower of the two the reviewer offered: §1's
bullet over tasks already takes only the system's own, and a plan refuses only what reaches it.

Round 14's answers: R14-D2 not held (R15-D1); R14-D3 partly, `M1.47`'s goal (R15-D4); R14-R4 partly (R15-R3); the
rest held.

## Round 16 — `2026-10-28`: not acceptable, 12 findings

The second round to find the code faithful everywhere, every defect the record's text. Every test green, the bounded
totals true — 113,230 systems, 112,268 records judged, the 136 the checker refuses rightly routed to `M3.4` — and every
hand-off quoted word for word. 4 defects, 6 remarks and 2 better decisions: hand-offs weaker than the sentences they
discharge, a code fact the composition reads and the plan never owes, and §2.6 left behind round 15's §1.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R16-D1 | defect | `RTP-H5` carries "hold under any preemption" without §2.2's "and from any entry state", and `figures-under-preemption` names only the first half | `RTP-H5` whole, requoted in `M4.12`; §2.10 says the fact is §2.2's declaration whole |
| R16-D2 | defect | `RTP-H16` names task, block and service quantities, while §1 refuses a platform's, a policy's and the own level's too | Any declaration's quantity, requoted in `M1.45`, its goal with it |
| R16-D3 | defect | the composition reads every `runtime-discipline.<id>`, and the plan owes none of the generated code's | Owed by each node, by the realization (§2.5, §2.10, §3), landing with the code and the worked texts (`M4.1.4`) |
| R16-D4 | defect | §1 credits §2.6 with reading a binding that takes a channel, and §2.6 says "for a consumed fact" | §2.6 reads model §8 rule 2's binding, and `interrupt-source` met by `true` |
| R16-R1 | remark | the checker reads an obliged bound's requester among services, tasks and hardware, never the own level | `M4.1.13` reopened |
| R16-R2 | remark | the 136 records move to `tool-failure` once the result record's intake lands, which the class does not say | `M4.1.13` reopened |
| R16-R3 | remark | `M1.44`, filed for a stated release source, is named nowhere | §2.2 names it, `RTP-H19` |
| R16-R4 | remark | "every probe a fixture" overstates: probes the language or the search refuses are not held | Every probe the lowering decides (How to apply, the book, `M4.1.4`, the corpus's header) |
| R16-R5 | remark | §1's component-call refusal has no hand-off | `RTP-H10` carries it, requoted in `M4.10` |
| R16-R6 | remark | the result record's intake, the provenance and the plan's reader are unbuilt | As declared: `M4.1.4`'s production half |
| R16-B1 | better decision | a sporadic task bound itself for `interrupt-source` is released by that source; the routes it reaches decide only where it binds none (the vocabulary; OSEK 2.2.3 §4.3) | Taken, landing with the code (§2.2, `M4.1.4`) |
| R16-B2 | better decision | rule 12 drops a binding's edge across composition, leaving a composite and its part ordered by their texts; Linux's device links hold a consumer until its supplier is bound | Taken: a binding keeps its edge, composition alone giving none — `M3.3.7`'s own reading, "the requester may call it while it initialises" — `M4.1.16` in the generator's joint rules, `M4.1.2` in the rule |

Round 15's answers: R15-D2 partly (R16-D4); the rest held.

**With the answers' code** (`ARCHOGEN-M4-1082`): the checker's sixth widening found no disagreement. Its builder ran
`git diff HEAD --stat` over `docs/` without excluding `docs/evaluation`; it compared the working tree with `HEAD`, whose
tree holds none of the sealed set's text, read no history and printed four other files: no case read.

## Round 17 — `2026-10-28`: not acceptable, 9 findings

One defect, the fewest yet, and the code faithful but where it transcribes rule 12 against §2's reaching. Every test
green, the bounded totals true — 116,238 systems, 115,276 records judged, the 136 the checker refuses held to `M3.4` —
and all nineteen hand-offs quoted word for word. 1 defect, 8 remarks and 3 better decisions: `M4.1.16`'s reasoning
carried no further than bindings, generated code's masked runs owed nowhere, and an obligation that lives in prose.

**Fence.** No breach. The reviewer prefixed every cargo run with the repository's scratch, and not several reads, the
heredocs that wrote its probes and its `git show`; those may have written temporary files to the session's default
temporary directory, off the repository's volume (`docs/decisions/decision_scratch-on-the-repository-volume.md`).

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R17-D1 | defect | a service or a policy naming a block or a platform gets no initialisation edge, though §2 makes that name a call: the order follows the names | An edge from the named hardware to its namer, `M3.3.7`'s reading of a call (`M4.1.17`), landing with the code; `M4.1.2` in the rule |
| R17-R1 | remark | `lib.rs`'s header, `Cargo.toml` and `M4.1.4` still say every probe is a fixture "with the text the record gives it" | Every probe the lowering decides, with the outcome it gives (`M4.1.4`) |
| R17-R2 | remark | generated node code in a job owes its call cost and not its masked runs, which the variant's condition 4 and the composite-inputs record §7 read | Owed by each such node, by the realization, landing with the code; `M4.1.6` designs their term |
| R17-R3 | remark | §3's new-format-number rule against round 16's 94 moved outcomes | It binds from acceptance; until then `/1` moves with its review |
| R17-R4 | remark | the 136 records' obligation lives in prose | `RTP-H20` to `M3.4` |
| R17-R5 | remark | the inferred trigger reaches through a provider's own requirements | `M1.44`'s, its issue naming the case |
| R17-R6 | remark | a service's bound within the arithmetic generated and counted by no class | `M4.1.13` reopened |
| R17-R7 | remark | §4 `.2`'s rule 12 silent on the edge across composition | Said, with `M4.1.17`'s |
| R17-R8 | remark | the result record's intake, the provenance and the plan's reader unbuilt | As declared |

The better decisions are D1's and R2's fixes, taken, and R5's, `M1.44`'s.

Round 16's answers: R16-R4 partly (R17-R1); R16-B2's reasoning not carried to named edges (R17-D1); the rest held.

## Round 18 — `2026-10-28`: not acceptable, 9 findings

The code faithful on every fixture and probe again, every defect the record's rules or text. Every test green, the
bounded totals true — 121,278 systems, 119,116 records judged, the 136 held to `M3.4` — and all twenty hand-offs quoted
word for word. 3 defects and 6 remarks: round 17's call carried only to hardware already a node, §4 short of §2.10, and
evidence categories owed for some realization bounds and not others.

**Fence.** No breach.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R18-D1 | defect | hardware a service, a policy or a task names, a call, and nothing binds places no code: no context, no step, nothing owed — and placed when an unrelated binding makes it a node | A node: the block or the platform the system serves that a served task, service or policy names (`M4.1.18`), landing with the code |
| R18-D2 | defect | §4 `.6` lists a node's call cost and not the masked runs §2.10 hands it | Added |
| R18-D3 | defect | `masked-runs` owed with `masked-category` beside `call-cost`, `service-cost`, `arrival-bound`, `overhead` and `cost-terms` with none, against `ROADMAP.md` §7.3's "evidence category" of every bound | Every bound owed to the realization is owed with §7.3's record, its category among it, which the realization states (`RTP-H2`); the application's categories stay facts of their own, as its format carries them (`RTP-H5`); a node's `masked-category` goes, landing with the code |
| R18-R1 | remark | §2.9 says `Joint::check` reports a service with two users, leaving out the channel | "a service taking a channel" |
| R18-R2 | remark | `tests/records.rs` lowers hand-written records no configuration checker judged | Each positive record judged (`M4.1.4`) |
| R18-R3 | remark | the ledger holds §8.6's sentence elided while §2.9 quotes it whole | Read whole from the PDF's text, the ledger's hash the same, and held whole |
| R18-R4 | remark | the `M3.4` routing sound | — |
| R18-R5 | remark | the checker reads the record faithfully, and D1's rule with it | `M4.1.13` reopened for D1 |
| R18-R6 | remark | rank ties broken by name, unreachable; a source device in the job of the task it releases, stated | — |

Better decision, taken: a name as a dependency, as the Linux kernel's `fw_devlink` infers "the consumer/supplier
relationships … suppliers first, then consumers".

Round 17's answers: R17-D1 partly (R18-D1); R17-R2 partly (R18-D2, R18-D3); the rest held.

## Kept for `M4.1.7`

The questions the narrowing moved to format `/2`, with the fix each reviewer proposed, so that none lives only in a
transcript:
- **The target's timer** (R3-1): which device's alarm releases timer tasks on a target is `.6`'s (`M4.1.6`), and the
  reviewer's fix, the target's tick-rate block where the system binds none, is rejected (the record's Alternatives
  weighed: `tick-rate` belongs to a counter, which may raise no interrupt; R14-R7). `.7` keeps the rest: where the
  system binds its own device, refuse until a stated correspondence between system device and target device exists
  (`M4.4.1`).
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
- **The placement's provider** (R15-R9): `eadl_resolve::plan::rebuild` keeps a placement's region and not its provider; `/2`, which writes placement, needs both.
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
