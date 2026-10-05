# The eADL conformance suite

What **`eadl/1`**'s conformance suite is: which descriptions are in it, what each part of it proves,
what is deliberately outside it, and who reads this file. The language's rules are not here —
[`grammar.md`](grammar.md) states what a well-formed description *is* and
[`reference.md`](reference.md) states what each literal is *worth*. This file states the **population**
those rules are checked over.

⭐ **This document is the manifest, and it is machine-read.** The population is enumerated from the
tables below by `crates/eadl-front/tests/common/suite.rs`, and every walk that needs the suite reads it
through that one module: `crates/eadl-front/tests/conformance.rs` (the recognizer derived from the
grammar against the reader), `crates/eadl-front/tests/reference.rs` (the reference's stated values
against the frontend) and `crates/eadl-front/tests/conformance_suite.rs` (the manifest's own rules).
⛔ Two walks that each name their own roots are two definitions of the suite, and they drift — which is
what this file replaced: `conformance.rs` and `reference.rs` each hardcoded `docs/semantics` and
`examples`, so the suite's scope was stated twice and enforced nowhere.

⚠️ **No count appears in this file, deliberately.** A size is a property of the roots, so the roots are
where it is measured: `conformance_suite.rs`'s census leg pins the population and refuses an empty one,
and `docs/book/src/checking.md` publishes the measured figure under a leg that compares the prose to the
walk. A number copied into a manifest is a number nothing re-derives — the defect class
`docs/tasks/PROGRAM.md`'s `PROGRAM.20` exists to register, and `M1.13.4.3` found two live instances of
it in the book while measuring this suite.

## The version this suite conforms to

<!-- machine-read: suite-version -->
| version | what conforming to it means |
| --- | --- |
| `eadl/1` | every description below reads cleanly under `grammar.md`, denotes the values `reference.md` §1–§3 state, produces the verdict its own header declares, and states its version as §8 requires |

## The roots

Every `.eadl` file under a root is in the suite, recursively. A root is a directory whose *purpose* is
the same for every file in it, which is what makes "what this root proves" a statement about all of them
rather than about one.

<!-- machine-read: suite-roots -->
| root | what this root proves |
| --- | --- |
| `docs/semantics/boundary` | the controlling boundary: paired accept/reject cases for functionality versus implementation, each carrying its verdict and its failing test in a comment header. **F27** reads exactly these |
| `docs/semantics/cases` | §12 M1's worked semantic cases, each declaring the §5.5 verdict it expects in its own header — positive, missing-fact, contradictory, unsupported, infeasible |
| `docs/semantics/kinds` | the declared vocabulary itself: the `defkind` modules the registry is built from, so the language's own definitions are conformance cases of the language |
| `docs/semantics/modules` | §6 modules and imports: a module path in the sense of §6 rule 7, holding library modules and one root per case — the file whose header carries `expect:` and, for a refusal, the one `code:` it must produce. **F01** and **F02** are its `app.*` and `bad.circular-import` / `bad.conflicting-export` cases, and every other `module-` code of §4 but one has a case, driven through `archogen check` and `archogen build` by `crates/archogen-cli/tests/module_cases.rs` |
| `examples` | the descriptions a reader copies — the M0 use cases, the S0 fixture and the refusal fixture, each checked end to end and, for S0, built and run |

## What is outside the suite, and why

<!-- machine-read: suite-exclusions -->
| excluded path | why it is not a conformance case |
| --- | --- |
| `docs/feedback` | outbound bug reports to another project. Every description under it is a **frozen reproducer**: its bytes are the reproduction, `FEEDBACK-SELF-CONTAINED` seals them, and some are deliberately malformed — which is what makes them evidence. A suite that swept them in would make one project's bug reports into conformance cases of a language version, and a malformed one would fail the accept legs for a reason that is not a language defect |
| `targets` | a target's platform description, beside the `.env` that pins the target. Its facts are the platform's, not the language's: it changes when a pinned emulator or board does, so freezing its canonical form as a conformance case would tie a language version to a QEMU release. It is checked where its meaning is: `cargo xtask target-agreement` compares every fact with the device tree and requires `archogen check` to admit it (`docs/decisions/decision_target-platform-description.md`), and the frozen-verdict table and the third reader hold it as they hold every description |
| `trust` | the trust gate's configuration (`docs/specs/trust/decision_trust-inventory.md` §3): which programs are trusted roots, which harness compares two of them, and each admission of a site the default-deny catalog refuses. It is written in eADL's syntax with kinds `eadl/1` does not declare — `defroot`, `defharness`, `defadmit`, `defrole` — so `archogen check` refuses it `schema-unknown-kind`, and a root would make a language version answer for a project's trust decisions. It is checked where its meaning is: `cargo xtask trust-inventory` reads it and refuses a form it does not know or a clause a form lacks (`xtask/src/trust.rs`'s `read_roots`), and the frozen-verdict table holds its refusal as it holds every description |

⛔ **An exclusion is a path prefix, and the matcher has no glob grammar.** A pattern matches a path when
the pattern's path segments are the leading segments of the path's, so `docs/feedback` excludes
everything beneath it. There is no `*`, no `**` and no character class, because a matcher with the full
glob grammar is a second thing to be wrong about beside the manifest — and the narrower spelling the
first draft of this leaf carried (`docs/feedback/linkedspec/issues/*/evidence`) needs one. Measured
before choosing: every excluded description is under an `evidence/` directory, and nothing else under
`docs/feedback` is a description at all, so the wider prefix excludes nothing that should be in — and the
leg `every_exclusion_matches_a_real_description` counts them instead of this file.

## How the population is enumerated

Four rules, each executed by `crates/eadl-front/tests/conformance_suite.rs` rather than promised here:

1. **Every `.eadl` file under a declared root, recursively, and nothing else.** Both directions are
   checked against an independent walk, because a population that is only "everything the roots
   contain" cannot see a root that was renamed, and one that is only "nothing outside the roots" cannot
   see a root that stopped being walked.
2. **A root that reaches an excluded path is a violation, not a silent skip.** Skipping quietly would
   let a too-wide root narrow the suite by accident; reporting it makes the manifest's two tables answer
   to each other.
3. **No description in the suite is byte-identical to an excluded one.** This is the leg that stops a
   frozen reproducer becoming a conformance case *by being copied into a walked root*, where rule 2's
   path prefix cannot see it. ⭐ It is usable only because it is false-by-construction nowhere: measured
   at `M1.13.4`'s decomposition, `docs/feedback/linkedspec/issues/LS-002-multi-form-truncation/evidence/system.eadl`
   was byte-identical to `examples/s0-heartbeat/system.eadl`, and `M1.13.4.2`'s version identifier
   diverged them — the copy stays a four-form reproduction, which is what the issue it reproduces is
   about. A future legitimate duplicate therefore has to be declared rather than assumed.
4. **No description in the repository is silently outside the suite.** A walk of the repository — minus
   the scratch and vendor roots named in that leg — requires every `.eadl` it finds to be either in the
   population or matched by an exclusion, so a description added under a new top-level directory is a
   manifest decision and not an accident.

## The frozen baseline

The population declared here is what `eadl/1`'s frozen baseline digests: `docs/semantics/BASELINE.txt`
holds one digest per frozen construct, and one of its three classes is the **canonical form** of every
description this manifest declares. `scripts/language_baseline.sh --emit` rewrites it — an explicit act,
never a side effect — and `crates/eadl-front/examples/language_freeze.rs` is the instrument that
enumerates the constructs, reading this file for the population rather than carrying a list of its own.

⭐ **So a change here moves the baseline, and that is the point.** Adding a root, widening one, or adding
a description changes which constructs are frozen, and `scripts/check_language_freeze.sh` refuses the
movement until a note in [`migrations/README.md`](migrations/README.md) covers it. Nothing in this file
is a count, so nothing here can go stale silently — the population is walked, and
`crates/eadl-front/tests/conformance_suite.rs` pins the census.

## What the suite proves, and what it does not

It proves that one version's rules and this repository's descriptions agree: that the reader and the
recognizer derived from `grammar.md` accept the same files and segment them into the same tokens, that
every literal a description writes is one `reference.md` states a value for, that every description
states its language version, and that each semantic case produces the verdict its own header declares.

⚠️ **Honest limits, stated rather than left to be discovered:**

- **It is one implementation checking itself against its own specification.** §12 M2's standard — a
  checker "sharing the same erroneous recurrence with its reference does not qualify as independent" —
  is not met here, and `M1.22` owns the question of a third recognizer.
- **It does not interpret meaning.** `reference.md` §7 rule 5 is explicit that a schema validates a
  declaration's frame; whether `(unambiguous-horizon (at-least 60 s))` is the right horizon for a system
  is nobody's claim here.
- **The corpora are a sample of what authors wrote, not of what the language admits.** The literal space
  is covered by the reference's executed rows and by probes derived from them, not by these files — see
  the "Conformance" section of `grammar.md`, which says why a suite that only walks a corpus proves
  less than it looks like it does.
