# Findings that need the director's judgement

- **Type:** `project`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** raised during the M0 + M1 build; recorded here so they survive the session
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

Eight items are recorded here so they survive the session. Three are outside an implementer's
authority to settle; the fourth (§4) is a measurement about the programme's own evidence that you
should see even though it is already fixed; the sixth (§6) is a review of **your own amendment**
by the model that has never read the implementation. Each is tracked as work, so nothing here
depends on this file being read.

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

⛔ **Resolved. The heading and the original text below are kept as the record rather than
rewritten**, because a register that silently edits its own history cannot be audited. Measured on
`2026-09-27`: `qemu-system-riscv64` resolves on `PATH`, and `scripts/target_emulator.sh --check`
reports `found: QEMU emulator version 11.1.1` and exits **`1`** with `NO RELEASE IS PINNED YET
(QEMU_VERSION_PINNED=none-yet)` — not exit `20`.

**So the dependency on something outside the repository is discharged, and what remains is inside
it:** pinning the release in `targets/riscv-virt-up.env`, and the §3.2 check that QEMU's generated
device tree agrees with the eADL platform fixture. Both are `M2.8`'s own work and need no ruling,
which is why this item is no longer one of the director's. `M2.8` does have one external dependency
left: `REQ-007`, the QEMU `virt` machine documentation and its device-tree bindings, requested from
the source described in [[reference_external-document-source-chipdoc]] and recorded in that
repository's ledger as `requested`.

Original finding, unchanged:

> `scripts/target_emulator.sh --check` exits `20`. The configuration is pinned as data and carries
> `TARGET_VERIFIED=no` until an installed QEMU confirms it.
>
> **What it blocks:** `M2.8`'s emulator spike and `M4.9`. **Cheap to resolve** — install it, then
> pin the release in `targets/riscv-virt-up.env`. Flagged only because it is a dependency on
> something outside the repository.

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

⚠️ **Not a decision to make; a calibration to carry.** The S0 gate (F28) generates, compiles and
runs a described system and compares its output to an observation frozen before the emitter
existed. It passed. It also could not distinguish a **hyperperiod** from a **longest period**.

Both original fixtures are *harmonic* — periods 10 and 30, then 10 and 20 — so one period divides
the other and `lcm` equals `max` on both. Replacing the least common multiple with the maximum in
`crates/archogen-s0` left **all twelve oracle tests green**. It was found only because the fix was
mutation-tested rather than assumed correct.

**Fixed in the same leaf (`S0.4`):** a unit test on a non-harmonic set (`lcm(10, 15) = 30` against
a longest period of `15`), and an end-to-end case,
`examples/s0-heartbeat/system-non-harmonic.eadl`, that the same mutation now fails.

**Why it is worth your attention anyway.** The generalisation is not about hyperperiods. A fixture
set can be *complete against its own specification* and *blind to a class of error*, and a green
gate says nothing about which. The programme's acceptance matrix is thirty such fixtures, and
§13.1 is careful to call them "a minimum practical corpus, not a proof of completeness" — this is
the first measured instance of what that sentence costs. The habit it argues for is mutation
testing at every gate, which §13.3 already lists and which is otherwise easy to defer forever.

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
| 3 | what does priority rank `0` mean? | refuse it in a description, **and write down the off-by-one** (the runtime's highest rank is `0`, the language's is `1`, and that is recorded nowhere) |
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

## 6. The §3.1.1 amendment was reviewed by the independent model — and it found five problems

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

The live-document size-containment doctrine you mandated, adopted as `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` by
`PROGRAM.17`, forbids one thing outright: a document that is both a growing history and something every
session must read. Nothing here does that today. None of these files is in the bootstrap reads, and the three
status pages that had turned into histories are fixed and bounded (`PROGRAM.17.2`). But these documents are
what you browse, they grow fast, and nothing bounds them. Measured over the 82 commits since `2026-09-27`:

| Document | Lines then → now | What is growing |
| --- | --- | --- |
| `CHANGELOG.md` | 1 464 → 3 459 | about 24 lines per commit |
| `DEV_NOTES.md` | 687 → 1 602 | one entry per lesson, each also promoted to `docs/knowledge/` |
| `docs/tasks/M1.md` | 1 864 → 6 928 | closed leaves: **5 328 of the 6 928 lines** are the bodies of its 55 closed leaves |
| `docs/tasks/PROGRAM.md` | 796 → 3 158 | closed leaves: 2 375 of 3 158 lines, 33 leaves |

The doctrine offers three answers, and each changes what you see when you open these files. Its adoption
guide says to stop and ask before making a change like that, so it is waiting for you:

- **(A) Keep them growing.** This is the status quo, and it is recorded as a decision rather than left to
  drift. The cost is that the files you open keep lengthening. `M1.md` is already mostly finished work.
- **(B) The changelog and the development notes as rolling ledgers.** At each month's end, the finished
  month moves, byte for byte, into a sealed file such as `docs/history/changelog/2026-09.md`, with its digest
  recorded. `CHANGELOG.md` keeps the current month and a short index of the sealed ones. A check proves that
  sealed months never change and that the index lists all of them. Nothing is deleted, and a past month is
  one click away.
- **(C) Seal closed leaves out of the task trees.** A finished leaf's body, its checklist and evidence, moves
  byte for byte to a sealed file per subtree, with a digest. The tree keeps one line per closed leaf naming
  its commit and linking to the sealed text. Open leaves, the Current Frontier and the logs stay where they
  are. `M1.md` would drop from 6 928 lines to roughly 1 600 of live work, with a closed leaf one click away.

**The recommendation:** (B) for both the changelog and the development notes, monthly; (C) for the task
trees, starting with `M1` and `PROGRAM`. Both are lossless and checked mechanically, and both change what you
browse, which is why neither has been done. **The decision needed:** accept, amend (a different rotation
period, or trees kept whole), or keep (A). The work is filed as `PROGRAM.31` (B) and `PROGRAM.32` (C), both
`blocked` on this ruling.

**Ruled `2026-09-30`, by delegation.** The director asked the engineer to decide and act, to the state of the art
and at sign-off quality. The decision, and why it departs from the recommendation above:

- **(B), with an entry-count boundary instead of a monthly one.** Measured that day, the whole history is one
  month, at up to 75 commits a day. A monthly window would seal nothing now and bound nothing later. So the oldest
  block of 20 changelog entries, or 10 development notes, is sealed, byte for byte, into the next numbered segment
  under `docs/history/` whenever the live file reaches twice that. An index lists each segment with its range,
  counts and digest, and a gate proves segments unchanged, the index complete and append-only, the order continuous
  and the window bounded. `PROGRAM.31` does it, and `docs/decisions/decision_history-ledgers.md` records it.
- **(C) as recommended**, after (B): closed leaves sealed out of the task trees, `M1` and `PROGRAM` first
  (`PROGRAM.32`).

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
