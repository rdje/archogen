# Findings that need the director's judgement

- **Type:** `project`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** raised during the M0 + M1 build; recorded here so they survive the session
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

Twelve items are recorded here so they survive the session. Three are outside an implementer's
authority to settle; the fourth (§4) is a measurement about the programme's own evidence that you
should see even though it is already fixed; the sixth (§6) is a review of **your own amendment**
by the model that has never read the implementation. Each is tracked as work, so nothing here
depends on this file being read. The settled items, §2, §4, §8 and §10, are sealed in
`docs/decision-history/`, each under its heading here (`PROGRAM.41`).

## 1. No physical board — the one M0 obligation that cannot be closed in software

§3.2 requires M0 to name the exact board, processor revision, debug interface, executable memory
region, clock configuration, and available specifications. **None can be recorded truthfully**,
so all seven are recorded as `unrecorded` in `docs/targets/first-target.md`, with the selection
criteria written out so the decision is ready the moment a board is.

Naming a plausible board from memory would be worse than naming none: every row is a fact a later
timing claim would rest on, and §9 is explicit that extraction output is "a proposal with a source
location, not automatically an accepted fact".

**What it blocks:** tree `M5` entirely, the board half of `M2.8`, and any `target-evidence` claim.
**What it does not block:** everything through M4.

**The decision needed:** procure a board, or record a deliberate decision to ship M7 without
physical-target support. §12 M5 is explicit that M7 cannot claim board support without M5
evidence, so the choice surfaces at the release gate whether or not it is made sooner. Making it
sooner is cheaper.

## 2. QEMU RISC-V is not installed on this machine — **RESOLVED `2026-09-27`; the pin remains**

Sealed, byte for byte, in [`decision_findings-for-director-review/02.md`](../decision-history/decision_findings-for-director-review/02.md) — settled, and never edited again.

## 3. A spine gate has no seam for cross-tree documentation

`TASK-ACCEPTANCE` requires a complete acceptance checklist from **every** staged
`docs/tasks/*.md`, not from the leaf that owns the staged code. Propagating a blocker into a
second tree alongside code is therefore refused. It has happened three times and the interim
convention — split the commit, same work-unit id on both — works.

It is the conservative closure of a **measured** hole upstream (a co-staged tree supplying
another leaf's evidence), so the strictness may well be intended. Tracked as `PROGRAM.8` with
routing evidence, and the first output of that leaf may be an upstream report rather than a
change. No judgement needed unless you want it prioritised.

## 4. F28 was green and could not see a whole class of error — found, fixed, worth knowing

Sealed, byte for byte, in [`decision_findings-for-director-review/04.md`](../decision-history/decision_findings-for-director-review/04.md) — settled, and never edited again.

## 5. Five runtime-semantics questions the contract does not decide — **your call on four**

⚠️ **This one needs a decision, and it is the highest-value thing in this record.**

`M2.2` built an independent reference model of the runtime, derived by a separate context that
never read the implementation. The two agree exactly over 16 000 randomised events — and disagree
in **five** places, every one of which turned out to be a question `ROADMAP.md` does not actually
answer. A single author had resolved all five silently, and the resolutions looked like the
specification.

They are set out with both readings and a recommendation in
[`decision_runtime-contract-gaps.md`](decision_runtime-contract-gaps.md). In brief:

| # | Question | Recommendation |
| --- | --- | --- |
| 1 | does detecting an overrun apply its policy, and can a fault attach to a non-running task? | §3.1 states the mapping; an overrun applies on detection, to the overrunning task |
| 2 | is an empty task set admissible? | refuse — a vacuous schedulability result is what §7.1 exists to prevent |
| 3 | what does priority rank `0` mean? | refuse it in a description, **and write down the off-by-one** (the runtime's highest rank is `0`, the language's is `1`, and that is recorded nowhere) — written down `2026-09-13`, generalised to `runtime index = \|hp(i)\|` on `2026-10-01` (the priority record's item 3) |
| 4 | a containable fault raised inside a masked region? | adopt the reference's reading — no contract change needed, it is an `rt-core` defect |
| 5 | what bounds mask nesting? | refuse beyond the bound rather than saturating or wrapping |

**The decision needed:** 1, 2, 3 and 5 change `ROADMAP.md` §3.1, the published profile, or the
priority decision record, and §14.1 makes that a reviewed change rather than an implementer's.
Leaf `M2.9` is open and blocked on it. Nothing else is blocked — the disagreements are asserted
on both sides as ratchet tests, so they cannot drift while they wait.

⭐ The generalisable part is worth more than any one of the five: **a specification gap is
invisible while one person implements it**, because they resolve it and the resolution looks like
the specification. It only becomes visible when a second reader derives the same thing without
seeing the first. That is what §12 M2 is asking for, and it worked.

> *`2026-10-01`:* "§3.1.1" below means the profile's fault contract, `docs/profiles/rt-static-up-v1-faults.md`,
> where the text moved on the director's ruling (leaf `M2.19`).

## 6. The §3.1.1 amendment was reviewed by the independent model — and it found five problems — **ruled `2026-10-01`**

⚠️ **This needs a decision, and two of the five change behaviour.**

`M2.9` resolved the five contract gaps of §5 by amending `ROADMAP.md` with a new **§3.1.1**
(fault classification, attribution and containment). That amendment was then sent to the same
isolated model that built `rt-reference` — which still has never read `crates/rt-core`. It
applied the amendment (42 tests pass, up from 30) and returned five criticisms **of the
amendment itself**. They are recorded here because they are `ROADMAP` changes, and §14.1 makes
those a reviewed change rather than an implementer's.

| # | Problem with §3.1.1 as written | Consequence |
| --- | --- | --- |
| a | Rule 3's second ground is **wider than rule 3's scope** | a gap, and it is the likelier event |
| b | The latched-slot case falls **between rules 1 and 3** | the fork is fatal vs silently lost |
| c | The `UnexpectedTrap` row does not actually pick a class | the four-to-three mapping stays partial |
| d | Two phrasings for one attribution rule | coincide today, separable in a later profile |
| e | Ground 2 argues against the wrong thing | wording only |

**(b) is the sharpest, and it has no safe default.** Rule 1's trigger is "a release for a task
that still owes a **job**". When the latch slot is full the task may be in `Created` — it owes an
undelivered *release*, not a job. If that counts, rule 3 escalates it and a doubled arrival
inside a critical section kills the system; if it does not, the second arrival is simply lost.
There is no third behaviour available and the text supports neither side.

**(a) is the one I would fix first anyway.** Ground 2 says containment "means changing the
schedule, which is the structure a kernel critical section exists to protect". That reasoning
does not stop at faults: `complete()` inside a masked region also changes the schedule's
occupant, and a job that merely *ends* while holding the mask leaves "the nesting depth above
zero with no owner" in ground 1's own words. So the contract as written escalates a *contained
overrun* inside a critical section to fatal while saying nothing about an *ordinary completion*
in the same region — and the completion is far the more likely of the two. The reference model
lets the completion through unchanged and records a silent-note saying why.

**(c)** "Deliberate fatal trap, or a surprise outside the model" is a disjunction, and "a
surprise outside the model" is not one of §8.1's three classes. Since the amendment exists to
make the four-to-three mapping a function, that row leaves it partial. No behavioural
consequence — the row's *Containable* column is "No" either way — but it is the one cell that
does not do what the table is for.

**(d)** The table attributes a stack-guard breach to "the task whose guard was breached"; rule 2
attributes the non-overrun faults to the executing context. These coincide in `rt-static-up-v1`
(separate static stacks, one execution context), so nothing breaks — but they are different words
for what must be one rule, and a later profile with a shared or guard-page stack pulls them apart.

**(e)** Ground 2 reads as if any schedule change were the hazard, when the argument is really
against *resuming* scheduling from a state the critical section had not finished making
consistent — the fatal path is itself the largest possible change to the schedule. Rewording it
as "resuming the schedule" closes that off.

### Three new silent gaps the amendment opened

Recorded so they are not rediscovered: `Runtime::release` (does a task whose latch is full "owe a
job"? — this is (b) seen from the model's side), `ReleaseEffect::Overrun` (rule 1 settles that the
policy applies, not what becomes of the release that triggered it), and `Runtime::complete` (this
is (a)). Fifteen `CONTRACT SILENT` notes remain in `rt-reference` in total.

**The decision needed:** (a) and (b) change behaviour and belong to you; (c), (d) and (e) are
drafting fixes I can apply under the same leaf once you rule on the first two. `M2.9` is open and
is where this lands.

**Ruled `2026-10-01`, by the director's delegation** ("What is your option on this. you know the codebase and the
roadmap and task-trees, so please decide (sota, signoff)"). `M2.9` carries it out.

- **(b) A second release while one is latched is recorded, never lost, and never fatal for its timing alone.**
  *(Narrowed `2026-10-01` by the fault contract's rule 1: "never lost" holds for arrivals the platform delivers as
  distinct requests; an external source that does not count arrivals during a pending request can lose one.)* The
  latch keeps the overrun beside the release it holds; at delivery, outside every masked region, the task's
  declared overrun policy applies, as rule 1 asks, and rule 3 does not, as its own note says of delivery. The same
  doubling one instruction after the region was already contained by that policy; the outcome may not depend on
  which side of an unmask an interrupt lands. Mature kernels defer rather than halt: FreeRTOS holds the ticks that
  arrive while its scheduler is suspended and processes them on resume, "This ensures the tick count does not slip"
  ([`tasks.c`](../book/src/ledger.md#freertos-kernel)), and OSEK reports an activation beyond a task's limit to its
  `ErrorHook` rather than halt ([OS 2.2.3 §13.2.3.1](../book/src/ledger.md#osek-os)). ⛔ *Corrected `2026-10-01`
  against the sources:* this sentence said OSEK and AUTOSAR "report an activation … rather than drop it". OSEK does
  drop it — "If E_OS_LIMIT is returned the activation is ignored" — so the precedent supports reporting without
  halting, not keeping; keeping the overrun is this profile's own choice, because a dropped release is a missed
  deadline nobody would see. The release that triggers an overrun is the policy's: under `SkipLateJob` it becomes
  the task's next job, and under `Fault` it goes with the faulted task.
  *(Superseded in its detail by the fault contract's rule 1: the latch holds releases, not overruns, and its arrivals
  are judged in arrival order at delivery.)*
- **(a) A job may complete inside a masked region it opened, and its completion closes every section it opened.**
  The nesting depth returns to zero with the job, latched releases are delivered as at the outermost unmask, and the
  schedule is decided after. The accepted composition already charges this run (`docs/specs/catalog/
  decision_runtime-composite-inputs.md`, `CS_i`'s "run ending at completion"), and the variant's condition 5 says
  every transition ends unmasked. Rule 3's grounds do not reach it: the depth is the job's and ends with it, and the
  job's own code has finished, so no region is left half-made. AUTOSAR OS likewise enables interrupts again when a
  task returns with them disabled ([SWS_Os_00239](../book/src/ledger.md#autosar-os)). ⛔ *Qualified `2026-10-01`
  against the source:* AUTOSAR treats that return as an error it recovers from — it reports `E_OS_MISSINGEND`
  (SWS_Os_00069) and ignores a `TerminateTask` called with interrupts disabled (SWS_Os_00093) — where rule 4 makes
  the completion an ordinary one. The precedent supports the recovery, not the ordinariness; the ordinariness rests
  on the composition's `CS_i` and condition 5 above, and whether such a completion is *reported* is `M2.15`'s.
  *(Superseded in its detail by the fault contract's rule 4: the depth is the job's by its Terms, and whether the
  decision precedes the delivery is the port's.)*
- **(c), (d) and (e) as drafted:** the trap row's class is the deliberate fatal trap; attribution is one rule, the
  executing context, which for a stack guard is the task whose guard was breached while every task has its own
  static stack; ground 2 speaks of resuming the schedule.
  *(Superseded in its detail by the fault contract's rule 2: attribution is the context that raises the fault, and a
  stack guard names whose guard was hit apart from whom it is attributed to.)*

**Carried out `2026-10-01`** (`ARCHOGEN-M2-0264`, leaf `M2.9`): both models carry (a)–(e), the reference re-derived
by a context that never read `crates/rt-core`, and the two agree over a randomised comparison widened to everything
§3.1.1 now decides. Rewriting the comparison's tests found five more things. Three were `rt-core` departing from
text the contract already had, and were fixed with the text cited: a trap or an assertion reported against no task
where the table says "the running task"; an overrun raised without a release (an execution-budget monitor's) that
started a job no release paid for, and a skipped job reported as an ordinary release; and a later fault overwriting
the one that halted the runtime. Two needed a decision, and I took them under the same delegation — **for your
review, and reversible:**

- **Ranks need not be contiguous.** A description may give its tasks priorities `1, 5, 9`; nothing in the language
  forbids it, and fixed priority uses only the order. `rt-core` refused such a set, the reference accepted it. The
  priority record now states `runtime index = |hp(i)|`, which is `rank − 1` when the ranks are exactly `1, 2, …, n`, and
  `rt-core` lowers by it. Refusing gaps instead would narrow the language by inference.
- **What a halted runtime leaves in its task table is left to the implementation.** Both models halt on the same
  faults and attribute them alike; `rt-core` then marks the attributed task faulted, the reference freezes the table
  as it stood. §8.1 asks for evidence, not a format. Recorded in `decision_runtime-contract-gaps.md` and asserted on
  both sides.

**Reviewed again `2026-10-01`, the text alone.** A new context read §3.1.1 as amended, and neither model: 25
findings, 13 defects, triaged in `decision_runtime-contract-gaps.md`. Most ask the text to say what both models
already do. One more needs your review, decided meanwhile so `M2.9` can proceed: **the runtime detects no deadline
miss** — rule 1's overrun is its only timing fault, §13.1 F26's "missed deadline" is exercised as an overrun with
`D = T`, and a miss with `D < T` is the timing analysis's to exclude and a trace's to observe. A deadline monitor
would be a new interrupt source and a new fault, which the profile does not list.

**Reviewed a third time `2026-10-01`, the text alone** (`M2.9` step 6d): 28 findings more, triaged in the gaps
record. One more needs your review, decided meanwhile: **`rt-static-up-v1` has no execution-budget monitor.** Rule
1a, added this morning, let one raise an overrun without a release; the review showed it cannot be raised inside a
masked region if it is masked with the region, that the two models then disagree on another task's, and that no
declared source or composition covers one. An overrun is detected by a release alone, as rule 6 already says of a
deadline, and a monitor is a later profile's.

Of the reference's `CONTRACT SILENT` notes, four stayed open, and none was a disagreement between the models: the
domain of overrun policies, what a later idle-to-task dispatch costs, whether a runtime must raise a fault on an
unbalanced unmask, and when a periodic task's first release falls. *(Superseded `2026-10-01`: the first is decided
by §3.1.1 rule 5 and `M2.14`, the third by §3.1.1's second decision — an assertion failure; the other two are on
§3.1.1's still-open list.)*

⭐ The generalisable part: **§5's lesson repeated one level up.** A second reader found gaps in
the specification; the amendment that closed them was written by one author, and a second reader
found gaps in *that*. The mechanism is not a one-off audit — it is worth running at every point
where a single author's resolution gets to look like the specification.

## 7. How a module declaration refers to its parameter — a surface choice on a frozen language (`2026-09-29`)

`ROADMAP.md` §5.1.1 asks for **typed parameters** and §10.1 step 1 for "bounded deterministic expansion".
Today a module parameter is bound and then consumed by nothing, and — measured — its bound value has three
different shapes depending on whether it came from a default, a one-form argument or a two-form argument, so
no consumer could use it as it stands (`M1.29.4` in `docs/tasks/M1.md` carries the probe output). `eadl/1` is
frozen, so whatever lands needs a migration note, and a surface spelling is expensive to change once
descriptions use it.

**The proposal on the leaf:** a parameter binds the forms after its name; it carries a type from the schema's
existing value types, `(param rate (holds quantity) (default 10 MHz))`; and a declaration refers to it with
an explicit `(param-value rate)` form, spliced in place, only in clause operands. Rejected: a bare-name
reference (captures vocabulary words, invisible to `grep`) and a sigil `$rate` (a character-level special
case in an otherwise uniform symbol grammar).

**The decision needed:** accept the proposal, or name a different spelling. `M1.29.4` waits on it; nothing
else does, and the leaf is medium priority, so the frontier has moved past it rather than stalling.

## 8. The histories grow without a lifecycle — the changelog, the development notes, the task trees (`2026-09-30`)

Sealed, byte for byte, in [`decision_findings-for-director-review/08.md`](../decision-history/decision_findings-for-director-review/08.md) — settled, and never edited again.

## 9. "Every programmatic response carries §5.5's verdict" — how the engine API reads it (`2026-09-30`)

Your ruling of `2026-09-28` says every programmatic response carries §5.5's verdict. `ROADMAP.md` §10.4 says
"for what it reports". The engine API (`decision_engine-api.md`, leaf `API.3.1`) has to decide what a response
carries when archogen did not judge the description at all. There are two such cases today: a kind module,
which no command loads yet (`M1.32`), and an import that exists but cannot be read.

**What it does:** every response carries the exit contract's one outcome vocabulary, `Status`. When the
description was judged, the status is its §5.5 verdict. When it was not, the status says `unimplemented` or
`usage`. It is never absent and never `ok`. The failure your ruling guards against is a missing outcome read
as success, and it needs a response with no status or a default one. This design has neither, and the CLI's
exit code becomes a projection of the response rather than a second opinion.

**The alternative:** force a §5.5 verdict onto both cases. The only candidate is `unsupported-profile`
("requested behavior lies outside implemented semantics"), which would tell an author their description
asks for something the profile refuses when it does not. **The decision needed:** accept the reading, or
rule that a response the API did not judge must still carry a §5.5 verdict, and which one. `API.3.2` builds
on the reading. Changing it later is a change to one enum and its projection, before the version is
fixed at `API.3`'s close.

## 10. How far the hold on `scripts/` reaches (`2026-09-30`) — **ruled the same day**

Sealed, byte for byte, in [`decision_findings-for-director-review/10.md`](../decision-history/decision_findings-for-director-review/10.md) — settled, and never edited again.

## 11. The catalog's premise 3 needs `main` protected, and that is not how this project commits today (`2026-09-30`)

The catalog design's threat model, which you ruled, assumes that the published main line is protected (§0, premise
3). Its tenth review showed what that takes on the hosting side. CI that runs after a push to `main` cannot block
that push. Blocking needs:
- pull requests required for `main`, administrators included;
- required checks on the merge result, with branches up to date or a merge queue;
- merge commits as the only merge method;
- force-pushes disabled.

**Measured today:** `origin/main` holds the initial commit alone, and local `main` is 232 commits ahead of it, with
no merge ever. The project commits directly to `main` and pushes in batches. No catalog record exists yet
(`catalog/` is empty), so nothing depends on the premise now. It binds from the first record, which `M2.7.4` writes.

The design already checks what it cheaply can: a production claim refuses any first-parent commit of `main`
after the named commit that is not a merge the hosting made and signed, and the loader re-checks every review
where it was ledgered.

**The decision needed, before `M2.7.4`:** turn on those settings for `main` (recommended), which changes the
commit workflow to branches merged by pull request; or rule a weaker premise 3 that records CI's verdict after the
fact, whose limit the design would then state. The catalog's eleventh review added two points the settings need:
- the required checks pinned to the CI provider's app, with their definitions protected from the pull request they
  judge;
- `main` never deleted.

The design now holds premise 3 from a named commit, the first after you confirm the settings, which is recorded then.

Its twelfth and thirteenth reviews found that the check itself needs two settings more:
- the check builds its checker from the commit the pull request merges into, in a required workflow or ruleset the
  pull request cannot change, so no pull request's code runs in its own check;
- code-owner review of the checker's closure: the packages the checker is built from and what they depend on
  (today `xtask`, `archogen-api`, `eadl-front` and `eadl-model`, with the catalog crate and `archogen-evidence` from
  `M2.7.3`), the root manifest, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `.github/` and `scripts/`.

The fourteenth added three. Code-owner approval must come from an identity other than the author's, and one
identity makes every commit here today, so it needs a second reviewer identity, which only you can provide. The
check must run on the hosting's own ephemeral runners, restoring no cache a pull request's job can write. And with
a merge queue, the check must be a ruleset-required workflow pinned outside the pull request's tree.

The fifteenth added three settings: a new push dismisses stale approvals, and the most recent push needs approval
from someone other than its pusher; the workflow token is read-only by default; and only the hosting makes merge
commits on `main`, signed with its key. How the check itself is protected is now `M2.7.6`'s design, with a test for
every construction the reviews found.

**Since `2026-10-02` the review of that design waits on your reviewer too** (`M2.7.6.4`). Its first round, by an
agent, found two defects, both answered. Its second and third, each a new agent context, the third asked only to
read the gate beside the YAML and GitHub references, were stopped by the agent harness's own safety screening before
they reported. No agent launches a fourth: the next round is the reviewer you name, with the review history
(`docs/reviews/catalog-check-protection-reviews.md`) as its brief.

Until the named commit exists, no production claim can be made. One new role is yours: a ledger line that fails its
check, through a defect in the checker for instance, is repaired only by a waiver you rule on, and a waiver can
only weaken what the catalog says.

**Added `2026-10-03` (`M3.6.1`).** The trust-dependency gate (fixture F30, `decision_trust-inventory.md` §5) accepts
its baseline and its root set by this same rule, so `trust/` and the gate's code join the code-owned paths above.
Nothing new is asked: the same settings and the same second reviewer. Until then the gate passes only while no two of
its roots share anything, as is the case today, and the first shared item — when `M2.7.5` makes the scheduling
checker read the catalog, which reaches the generator's own reader and model — waits on them.

## 12. The fault contract's narrowings — **approved `2026-10-02`**

The fault contract (`docs/profiles/rt-static-up-v1-faults.md`) lists in its header, under *Narrowings*, each claim its
amendments made that the earlier text did not limit, "for the director's review" (§14.1). Asked on `2026-10-02`, after
its fifteenth independent review found no defect — "The narrowings listed in the contract's header await your
approval." — **the director answered: "APPROVED"**.

What was approved is that list as it stood when asked, R15's answers included:
- no overrun is raised without a release (rule 1a);
- a missed deadline is reported only as rule 6 says, and §13.1 F26 with it;
- an externally released task's second arrival during a pending request can be lost where its source does not count
  arrivals (rule 1);
- a `mask` or `unmask` outside a job is an assertion failure, initialisation's included (Terms);
- the cases the Terms list that the trap path, a service or the runtime API's entry can see — an external trap's empty
  first claim, a timer trap or further timer service that finds no release due, a claim returning an undeclared
  source, an API entry naming neither a primitive nor the completion path — are unexpected traps, halting when raised;
- a port's runtime-API trap serves no interrupt, whatever its entry names (rule 2);
- the image's panic handler and its panic strategy are the port's, another refused at build, and a panic is an
  assertion failure unless a check of the runtime's, a port's, a catalog record's or generated code finds a guard or an
  unexpected trap, an application's check being classified by how it ends;
- what can happen before a fault a check finds, or a panic, is raised is the port's to state, whole, in its catalog
  record — a primitive left consistent or its job not abandoned, and a job not abandoned run on at its task's priority,
  before any later job of its task starts, to the raising (Terms, rule 5);
- application initialisation, like application code after it, never calls the fault path (Terms);
- a policy applied before its check in the trap of an API entry naming neither a primitive nor the completion path, or
  in an `unmask` at depth zero, abandons the job and raises nothing (rule 5).

**How it is applied.** The contract's header records the approval when `M2.9`'s step 6v lands, and the rules that said
"for the director's review" say approved. A later amendment that narrows a claim again is a new item for review; this
approval covers the list above and nothing added after it.

**A mechanism stated after the approval, `2026-10-02` (`M2.12.3`) — for your review.** The item "the image's panic
handler and its panic strategy are the port's, another refused at build" stands in effect: the handler still does
nothing but the port's work, an application's own still does not build, and the strategy is still the port's — on the
pinned compiler `abort`, the only one that builds a `no_std` binary. What changed is how the handler is supplied. The
catalog's own rules (§3) refuse `#[panic_handler]` in any record's code, since it is a global hook another package
could supply in its place. So the image generates the handler, holding nothing but a call of the entry the port's
record names, and the fault contract's Terms and rule 7 now raise a panic at that entry's first act rather than the
handler's. Two small consequences: the raising comes later by the handler's own call, so rule 7's "from the raising,
no pending interrupt is taken" starts that much later; and the window before the raising now holds that generated
code, which the port states whole without holding it. It narrows no approved claim, so it is not a new narrowing under
the rule above; it is shown to you because it rewords an approved item. **If you want it treated as one**, say so and
it waits for your approval.

## A note on what "done" means so far

M0 and M1 are complete, and the shape of the claim matters. What exists is a **frontend**: a
description can be read, type-checked, admitted or refused against a profile. Since then S0 has
added a **prototype generation path**: `archogen build` emits a Rust crate that compiles and runs,
and F28 is green. 301 tests hold that behavior.

What still does not exist is resolution, provider search, analysis, or any claim about timing —
and the S0 path is explicitly temporary (`crates/archogen-s0`, marked experimental everywhere it
surfaces, replaced wholesale by M4).

`archogen check` says this itself on every acceptance — *"this checks the description, not a system:
no resolution, generation or analysis has run"* — because the gap between "the description is
well-formed" and "a system built from it will behave" is the whole remaining programme.
