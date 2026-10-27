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

## Round 3 — `2026-10-27`: not acceptable, 32 findings, answers pending

The reviewer lowered the heartbeat and `adapter-epoch-extender.eadl` on `targets/riscv-virt-up.eadl` by the record's
rules alone. The heartbeat matched the worked text, under two readings the rules do not force. The second system needed
choices in six places. 18 defects, 14 remarks.

- **The defects:**
  - the target's timer ignored by the release-timer rule;
  - `Joint::map` undefined over a target;
  - a counter, not an alarm, chosen as release timer, its channel used twice;
  - contexts inherited by callers rather than callees, and an instance's missed;
  - no `ITEM` for a timer route;
  - the condition-6 refusal without a mechanism;
  - `every` and `some` unsound;
  - the epoch's bound held by no record;
  - the image able to overlap the stacks;
  - the allocation's arithmetic unfixed;
  - the fault path missing from the interrupt stack;
  - the owed table incomplete;
  - three obligations handed to no one;
  - `M4.1.2`'s leaf contradicting the record;
  - `obligation` lines underivable;
  - §7.5's plan hash unhanded;
  - `M4.3`'s and `M4.4`'s duties in no ledger row;
  - the memory-section rules contradicting each other.
- **The remarks:**
  - the Davis claims overreaching and the ledger's page count wrong;
  - a misattributed rule;
  - order and naming ambiguities;
  - deliverability without a token;
  - identity and evidence of catalog numbers;
  - the graph's edges dropped;
  - `M4.12` without a goal;
  - the boot stack's owner;
  - the owed list's gaps;
  - multiplex's source;
  - a missing refusal;
  - `RTP-H4`'s order;
  - a phrase said to be in the adapters record that is not.

**Method, after three rounds that did not converge** (18, 17 and 18 defects): the record designed questions other leaves
own — how timer-released tasks and an adapter share an alarm (the joint rules'), how a system's devices correspond to a
target's (`M4.4`'s), and sizes only catalog records state. The answer narrows format `/1` to what is determinate today,
moves the target, the memory layout, the epoch and stack allocation to `/2` (a later child of `M4.1`, after `M4.4`'s
correspondence), gives the release alarm a design of its own (`M4.1.2`), and pins `/1` by two complete worked texts that
become its goldens. Paused `2026-10-27` for the director's ruling on findings §11.
