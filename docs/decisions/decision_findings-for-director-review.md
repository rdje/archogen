# Findings that need the director's judgement

- **Type:** `project`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** raised during the M0 + M1 build; recorded here so they survive the session
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

Fourteen items are recorded here so they survive the session. Three are outside an implementer's
authority to settle; the fourth (§4) is a measurement about the programme's own evidence that you
should see even though it is already fixed; the sixth (§6) is a review of **your own amendment**
by the model that has never read the implementation. Each is tracked as work, so nothing here
depends on this file being read. The settled items, §2, §4, §6, §8, §10 and §12, are sealed in
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

Sealed, byte for byte, in [`decision_findings-for-director-review/06.md`](../decision-history/decision_findings-for-director-review/06.md) — settled, and never edited again.

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

**Added `2026-10-03` (`M3.6.1`; amended the same day after its third and fourth reviews).** The trust-dependency
gate (fixture F30, `decision_trust-inventory.md`) reports what two of the programs whose independence a claim relies
on share; a shared item, a root form and a classification are accepted only by the review this section asks for, so
`trust/` and the gate's code join the code-owned paths above (`M2.7.6.5`). Nothing new is asked: the same settings
and the same second reviewer. Until then nothing is accepted — today's shared items are the build configuration every
root is compiled under and the comparison harness the reference model shares with the runtime it validates, by
design — every property resting on a role's independence is `not-established` (`M4.8`), and the assurance tier's
trust step is not built for the acceptance it lacks (`M3.6.3`, `M3.6.5`). When `M2.7.5` makes the scheduling checker
read the catalog, which reaches the generator's own reader and model, that sharing is reported the same way.

## 12. The fault contract's narrowings — **approved `2026-10-02`**

Sealed, byte for byte, in [`decision_findings-for-director-review/12.md`](../decision-history/decision_findings-for-director-review/12.md) — settled, and never edited again.

## 13. The repository cannot live under a path holding a colon — the toolchain's limit (`2026-10-21`)

Your standing rule is that the repository may be moved anywhere, another filesystem included, without breaking
anything. One place breaks it, and the cause is upstream. The pinned [Cargo](../book/src/ledger.md#rust-toolchain)
(1.95.0) joins its target directory into
the library search path, whose separator on this host is the colon, and fails **every** build under a target directory
whose path holds one — a trivial crate with no dependency included: ``failed to join paths from
`$DYLD_FALLBACK_LIBRARY_PATH` together``, caused by ``path segment contains separator `:` ``. A colon in the
crate's own directory or in Cargo's home is harmless; only the target directory matters. This project keeps its target directory inside the
repository, so the limit is the repository's.

What is already done (`M3.6.9.6.3.2.2.15.2.1.6.2.3`): the catalog gate used to refuse the package as not building when
its scratch directory held a colon — blaming a package for its directory. It now stops before building anything,
unjudged, naming the directory, held by a permanent control. The limit is stated in the book's verification chapter
under *Where the repository may live*, with the measurement in the trust annex.

Measured harmless in the same assessment: spaces, backslashes (a trailing one included), tabs, line breaks, hashes and
quotes in the path.

**Recommendation:** accept it as a known limit of the toolchain, as stated. The alternative, a target directory outside
the repository, would break the rule that every project path is repository-relative. Nothing waits on your answer.

## 14. The mutation-adequacy lane had stopped the product's progress (`2026-10-21`) — **ruled the same day**

From `2026-10-11` to `2026-10-21`, 158 of 173 commits went to `M3.6.9`, the mutation adequacy of the trust gate's own
tests in `xtask/src/trust.rs`, and none to the product; the other 15 were upkeep. Each surviving edit opened further
children — leaf identifiers twelve levels deep — and the pending lanes still listed about 1 100 candidate edits, so no
honest finish date could be given. The roadmap's F30 exit criterion asks that the trust gate surface unreviewed sharing
and drift and block acceptance, and that gate exists and runs; the exhaustive adequacy of its tests was filed by an
internal review (`M3.6.6.4`, round 14), and finishing it before `M3.3` was a project choice, not a roadmap one.

**Ruled `2026-10-21`:** mutation testing stops now and resumes, extensively, once the functionality is complete and
locked; every decision advances the project toward its goals first. Every pending `M3.6.9` leaf is postponed, `blocked`
on [the ruling](decision_mutation-testing-after-completion.md); the frontier moves to `M3.3`. A suspected real defect the
lane's reading found, two generators given one target directory (`M3.6.9.6.3.2.2.15.2.1.8.2`), stays owned at P2.

## 15. `examples/alternative-timer` and a privilege level (`2026-10-22`)

`SR-H12` (`M3.3.4`) refuses a provider a plan binds that states no level. uc3 states none: its timer, and its
`console.uart`, bound through `observable-output`. `SR-H10` gives the timer's level to `M3.2` and asks that its
description change only with your yes; nothing covers the console, and `examples/README.md` seals that uc3 builds
unchanged. **Your call:** (a) uc3 gains `(reachable-at-privilege machine)` on each provider a plan binds, in `M3.3.4`'s
migration; or (b) it stays unchanged and is refused once `M3.4` wires the rule, until `M3.2` states the timer's level and
you rule on the console's. **Decided (a), `2026-10-22`**, under the director's delegation; `M3.3.4.5`, done.

## 16. `archogen analyze` and the plan check cannot be subcommands of `archogen` (`2026-10-22`)

`ROADMAP.md` §10.2's interface target writes `archogen analyze …` as a subcommand, and an independently checked plan
needs a configuration checker (`M3.5`). Two recorded rules make both impossible as subcommands of the `archogen`
executable: one program may not run two roles (§4.4; `decision_trust-inventory.md` §2, `trust-shared-program`), and
the product spawns no process (§10.4; `NO-SUBPROCESS`), so `archogen` can neither link nor launch the scheduling
checker or the configuration checker. **Your call:** (a) amend §10.2 — each checker is an executable of its own,
`archogen-analyze` and `archogen-check`, run by the user or the build orchestration, each writing a result record that
`archogen build` consumes and refuses without (recommended: both rules keep their full strength, and every result
names the program that produced it, `TI-H14`); (b) keep one executable and accept a reviewed exception to
`trust-shared-program` for the two checkers — §4.4's independence then rests on code review alone; (c) let `archogen`
launch its role executables — §10.4's safety argument then no longer holds for the programmatic surface.
`M3.5.1` and `M2.21` wait on it; the checker's library work (`M3.5.2`–`.5`) holds under every answer and proceeds.
**Ruled (a), `2026-10-22`**, with a git-style front: "Let's trust you and pick (a) Recommended: give each checker its
own executable"; then, on `archogen analyze` running the executable `archogen-analyze` as git runs `git-foo`, the
human CLI alone allowed to run its own sibling executables and the programmatic surface still running nothing: "Then
if we agree then go ahead". `M3.5.1` designs both, `M2.21` taking the scheduling checker's shape from it.

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
