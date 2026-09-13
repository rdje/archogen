# Findings that need the director's judgement

- **Type:** `project`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** raised during the M0 + M1 build; recorded here so they survive the session

Four items are recorded here so they survive the session. Three are outside an implementer's
authority to settle; the fourth (§4) is a measurement about the programme's own evidence that you
should see even though it is already fixed. Each is tracked as work, so nothing here depends on
this file being read.

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

## 2. QEMU RISC-V is not installed on this machine

`scripts/target_emulator.sh --check` exits `20`. The configuration is pinned as data and carries
`TARGET_VERIFIED=no` until an installed QEMU confirms it.

**What it blocks:** `M2.8`'s emulator spike and `M4.9`. **Cheap to resolve** — install it, then
pin the release in `targets/riscv-virt-up.env`. Flagged only because it is a dependency on
something outside the repository.

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
