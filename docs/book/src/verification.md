# Verifying the toolchain

The project's own checks are organised into the five tiers of `ROADMAP.md` §14.3, and each one
is a named command. The runner is the workspace's `xtask` member — `xtask/src/main.rs` declares every
tier and every step in it, and `make tiers` prints them from there. The outside tools the tiers run, and
the versions they run at, are in [What this project relies on from outside](ledger.md): [QEMU](ledger.md#qemu),
[mdBook](ledger.md#mdbook), the [Rust toolchain and Cargo](ledger.md#rust-toolchain) and, once adopted,
[Miri](ledger.md#miri).

```console
$ cargo xtask verify --tier focused      # or: make focused
$ cargo xtask verify --tier integration  # or: make integration
$ cargo xtask verify --list              # or: make tiers
```

| Tier | When | What it covers |
| --- | --- | --- |
| `focused` | each edit loop, and every ordinary commit | format, lints, the whole contract suite |
| `integration` | before a push, and before closing a milestone | the above, plus the doctrine enforcer, every doctrine gate's own RED arms, the book build, the `no_std` build and the pinned emulator |
| `extended` | scheduled, or when a change touches parsing, arithmetic or event ordering | fuzzing, mutation, Miri |
| `hardware` | a change to target support, and every release gate | board regressions and timing observations |
| `assurance` | every supported release | trust inventory, claim completeness, source and binary identity |

## A tier has three outcomes, not two

§14.3 ends with a sentence that is easy to agree with and hard to implement:

> A required tool skipped or unavailable is reported as such, **not a passed check**.

A runner that silently drops a step it cannot run converts an *absence of evidence* into an
*appearance of evidence* — and it does so exactly when the missing step is the one that mattered.
So the runner has a third verdict:

| Verdict | Exit | Means |
| --- | --- | --- |
| `passed` | `0` | every step ran and succeeded |
| `failed` | `1` | a step ran and failed |
| `incomplete` | `20` | nothing failed, and something could not be run |

`incomplete` is not a pass and not a failure. Nothing is broken; the evidence is simply not
there. And the two reasons a step cannot run are kept apart, because the response differs:

- **unavailable** — the step exists, and a *tool* is missing from this machine. Install it.
- **not built** — the step does not exist yet, and it names the **task-tree leaf** that owns
  building it, so a reader learns where the work is tracked rather than concluding the project
  forgot.

## What that looks like today

Rendered from a run, not retyped:

```console
$ cargo xtask verify --tier integration
tier: integration — before a push, and before closing a milestone
  ✅ fmt                  0.18s  every Rust source is in canonical format
  ✅ clippy               0.49s  no lint fires anywhere, including in tests and examples
  ✅ tests                3.07s  every contract test passes, F28 and the semantic corpus included
  ✅ doctrines            4.22s  every repository invariant holds on the working tree
  ✅ self-tests          33.87s  every doctrine gate's RED arms still fire — a gate that stopped being able to fail is caught here
  ✅ book                 0.08s  the mdBook builds — it is the director's window, so a broken book is a broken deliverable
  ✅ no-std-build         0.04s  the runtime core compiles for a bare-metal target (§14.3's "compile targets")
  ❌ emulator             0.12s  FAILED
     target-emulator: found: QEMU emulator version 11.1.1
     target-emulator: TARGET_VERIFIED=no — this configuration is still a PROPOSAL
     target-emulator:   leaf M2.8 owns flipping it, with the evidence that justifies it
     re-run it directly: scripts/target_emulator.sh --check
tier integration: failed — 7 passed, 1 failed, 0 unavailable, 0 not built
$ echo $?
1
```

The one failure is honest and owned. QEMU is installed and pinned now, so the emulator step *runs* — and
refuses, because the target configuration it checks still says `TARGET_VERIFIED=no`: it is a proposal until
leaf `M2.8` supplies the evidence to flip it. A tier that turned that into a pass would be claiming a target
nobody has verified.

⭐ **The `self-tests` step is the newest, and it answers a different question from `doctrines`.** The
doctrines say the tree is clean; the self-tests say each doctrine gate can still *fail* — every gate's own
RED arms, discovered by census from `scripts/`, plus `scripts/selftest_spine.sh`, which arms the gates the
scaffold owns from outside. Until leaf `PROGRAM.28` nothing ran them, so an arm that broke, or began to pass
for the wrong reason, stayed invisible until someone happened to invoke it. It takes about forty seconds,
which is why it lives here and not on every commit.

⭐ **Two of the five tiers are incomplete, and saying so is the runner's most useful output.** Before
the runner existed, the fuzz corpus, the mutation harness, the Miri wiring, the board and the whole
assurance story were not *reported as missing*. They were simply not mentioned, which reads exactly
like being covered. Each gap was given an owner. The `extended` tier's three steps were built under
`PROGRAM.9`, and on `2026-09-30` it reported **passed** for the first time: `fuzz` in 3.75 s,
`mutation` in 8.00 s, `miri` in 996.69 s. The remaining gaps are named: `M5.1` for the board, and
`M3.6` / `M4.8` / `M4.7` for the assurance tier.

### The `miri` step proves it can fail before it passes

The workspace contains no `unsafe` code, so a Miri run over it that passes looks exactly like a run
in which Miri saw nothing at all. `scripts/extended_miri.sh` therefore starts by building a
throwaway crate under `target/` whose one test reads through a dangling pointer. If Miri does not
refuse that test, the step fails before touching the real code. Miri comes from the `nightly`
toolchain, and until this step existed the tier asked the pinned `stable` toolchain for it and
reported Miri **unavailable** on a machine where it was installed.

Every test target was measured under Miri on its own: 34 targets, **541 tests passed, 4 ignored,
none failed**. The four ignored tests run cargo as a child process, which Miri cannot do, and each
says so where it is written. Five targets that walk a whole corpus take more than 300 seconds each
under Miri (two did not finish in 600). They are left out of the default run, named with their
measurements, and `--all` runs them. What remains took **851 seconds** end to end through the tier. Miri keeps its
sysroot under `target/`, not in a cache under the user's home directory.

```console
$ bash scripts/extended_miri.sh             # the step: the arm, then every crate
$ bash scripts/extended_miri.sh --arm-only  # only the seeded dangling-pointer read
$ bash scripts/extended_miri.sh --all       # every test target, the corpus walks included
```

### The `fuzz` step, and what it found on its first run

`scripts/extended_fuzz.sh` runs a seeded fuzz harness over the reader and the exact arithmetic
(`crates/eadl-model/tests/fuzz.rs`). It has no dependency and does not use `cargo-fuzz`. Every run
uses a fixed seed and a fresh one, 100,000 cases per property. Eight properties must hold:

- the reader never panics, and every span it reports lies on a character boundary;
- canonical text reads back to the same document;
- checked arithmetic stays normalized;
- small results are exactly right;
- ordering agrees with equality;
- a printed value reads back as itself;
- for small values, the ordering is the sign of the difference;
- and an inverse undoes its operation.

The harness first **arms** itself with six claims known to be false, such as "no input contains a
multi-byte character" and "no two values have both cross products beyond `i128`". It fails unless the
generator refutes each one, because a property passes meaninglessly if the generator never reaches the
inputs where it could fail.

Designing it, and running it once, found three defects, each now fixed and pinned by a test:

| Leaf | What was wrong | How it showed |
| --- | --- | --- |
| `M1.34` | two different amounts compared as **equal** | a deadline longer than its period was admitted |
| `M1.35` | the reader **crashed** on `"a\éb"` | `archogen check` exited 101 |
| `M1.36` | printing `1/2^100` overflowed | a panic in debug, a **wrong number** in release |

Each of the three, put back as a deliberate mutation, makes this step fail on the property that
concerns it. The step runs with overflow checks on: a release build wraps silently, and its first
release run passed straight over the third defect.

```console
$ bash scripts/extended_fuzz.sh                 # the fixed seed and a fresh one
$ FUZZ_SEED=42 bash scripts/extended_fuzz.sh    # a chosen seed; a failure prints its replay command
```

### The `mutation` step: each defect, put back, must still be caught

Every leaf that fixed a defect proved its test by breaking the code on purpose and watching the test
fail. That was done by hand, once. `cargo xtask mutate` does it again on every run, from a catalog
(`xtask/mutations.txt`). Each entry is an exact piece of code, a deliberate defect to swap in, and
the tests that must catch it. The harness checks each step:

- the text occurs exactly once;
- the defect really landed in the file;
- the tests failed, and which ones;
- the file was restored byte for byte.

A defect that does not even compile is a broken entry, never a "kill". A run that is interrupted
leaves a marker, and the next run refuses to start until the file is restored.

One entry is expected to **survive**. S0's first oracle could not tell the hyperperiod (`lcm`) from
the longest period (`max`), because all its fixtures were harmonic. That blind spot is kept as a
reproduction you can run: the harmonic tests alone let the defect through, and the full suite
catches it with the non-harmonic case added for exactly that reason.

```console
$ cargo xtask mutate                        # the whole catalog
$ cargo xtask mutate --only lcm-to-max      # one entry
```

## What the `tests` step is a suite *of*

The row above says "every contract test passes", and for the language that means a declared population.
`docs/semantics/conformance.md` is the manifest: which descriptions
are `eadl/1`'s conformance suite, what each part of it proves, and what is deliberately outside it. The
tables in it are **machine-read**, and one module — `crates/eadl-front/tests/common/suite.rs` — walks
them, so every leg that needs the suite enumerates the same population instead of restating its own idea
of it.

| Root | What it proves |
| --- | --- |
| `docs/semantics/boundary` | the controlling boundary — functionality versus implementation — in paired accept/reject cases, each carrying its verdict in a comment header that **F27** reads |
| `docs/semantics/cases` | the worked semantic cases, each declaring the §5.5 verdict it expects in its own header, so a driver that computed the expectation could not agree with itself |
| `docs/semantics/kinds` | the `defkind` modules the registry is built from, which makes the language's own definitions conformance cases of the language |
| `examples` | the descriptions a reader copies, checked end to end and — for the S0 fixture — built and run |

⭐ **The manifest carries no count.** A size is a property of the roots, so the roots are where it is
measured: a leg pins the population and refuses an empty one, and the chapters that publish a figure
publish the measured one under a leg comparing prose to walk.

**What is outside the suite, and why.** Everything under `docs/feedback` — the reproductions of defects
reported to another project. Their bytes *are* the reproduction, `FEEDBACK-SELF-CONTAINED` seals them,
and several are deliberately malformed, so sweeping them in would turn one project's bug reports into
conformance cases of a language version.

**Four rules the manifest's legs enforce**, in `crates/eadl-front/tests/conformance_suite.rs`:

1. the population is exactly what the declared roots hold — checked in **both** directions against a
   walk that shares no root list with the manifest, because one direction alone cannot see a root that
   stopped existing;
2. a root that reaches an excluded path is a **violation**, not a silent skip;
3. no suite file is byte-identical to an excluded one, which is what a path prefix cannot see — a frozen
   reproducer copied into a walked root;
4. no description in the repository is silently outside the suite, so a new one under a new directory is
   a manifest decision rather than an accident.

⚠️ **Honest limit.** This is one implementation checking itself against its own specification. §12 M2's
standard — a checker "sharing the same erroneous recurrence with its reference does not qualify as
independent" — is not met by it, and leaf `M1.22` owns the question of a third recognizer.

## What is frozen, and what a digest can prove

The constructs of `eadl/1` are digested into `docs/semantics/BASELINE.txt`, one line per construct, in
the form `<sha256 of the construct's text>  <construct id>`. Three commands, and no output is quoted
here because a digest is exactly the kind of figure that rots silently:

```console
$ scripts/language_baseline.sh --list    # the construct ids alone
$ scripts/language_baseline.sh --print   # the baseline, on stdout, touching nothing
$ scripts/language_baseline.sh --emit    # rewrite docs/semantics/BASELINE.txt — an explicit act
```

Three classes of construct are frozen: the EBNF fence of `docs/semantics/grammar.md`, **every**
machine-read table `docs/semantics/reference.md` carries, and the **canonical form** of every
description the manifest declares. ⛔ Enumerated at run time rather than listed, in the instrument
(`crates/eadl-front/examples/language_freeze.rs`) and again in the script: the decomposition that
planned this recorded "five machine-read tables" and the reference carries six, because §8 added one —
a list written into a tool is a stale figure with a compiler behind it.

⭐ **Canonical form, and not file bytes.** A corpus file's comment header carries its `case:`, `why:` and
`rationale:` prose, and editing a rationale is not a language change. Canonical form is the language's
own normative printer and drops comments, so a digest moves exactly when the described system moves — and
it is the same artifact §12 M4 hashes, so the baseline and the build agree on what "the same
description" means instead of maintaining two definitions of it.

⚠️ **What a digest proves, and what it does not.** That a construct *moved*. It cannot prove that the
migration note covering the movement is correct, or complete, or that every description the change
invalidates was found — and because canonical form carries no comment, a change to a corpus file's
*header* is invisible here. The headers that carry data are pinned elsewhere: by `corpus.rs` against
`docs/semantics/boundary/README.md`'s counts and by `reference.rs` against the reference's
`comment-headers` table. So the baseline proves the described systems did not move, not that the files
did not.

⛔ **Writing the baseline and gating it are different tools, and different commits.** A gate written
beside the baseline it enforces has no prior state to differ from, so no RED arm can fire against the real
tree. `M1.13.4.5` wrote the file and the comparator that classifies a difference as *moved*, *added* or
*removed*; `scripts/check_language_freeze.sh` is the gate.

## The freeze, and the note it demands

`LANGUAGE-FREEZE` runs with the other doctrines — on every commit, and in the `integration` tier — and has
**three legs**:

| Leg | Asks | Catches |
| --- | --- | --- |
| integrity | does the tracked baseline agree with a fresh run over the working tree? | a frozen construct edited and the baseline left alone |
| explicitness | does the tracked baseline agree with `HEAD`'s — or does a **pending** migration note name every construct the amendment moves? | a baseline regenerated because something moved, with nothing written down |
| spent notes | does a note that `HEAD` already carries as pending still say `pending`? | a note left open after its movement landed, which would cover a later movement silently |

The second leg is the one that makes the freeze mean something. Without it, "amend the baseline" is the
waiver: edit the construct, re-run `--emit`, and the first leg goes green having proved nothing. So
regeneration is an **explicit act** that fails until a note exists — and the third leg makes a note a
permission for **one commit**: a migration is the commit that lands the movement with its note `pending`,
then the commit that flips the note to `applied`, and the gate refuses the second commit until it does.

⛔ **Until leaf `PROGRAM.27` the second leg could not fail on the real tree.** The notes directory's own
`README.md` documents the note form with the lines `- status: pending | applied` and
`- constructs: … — or: all`, and the gate read any line *starting* `- status: pending` as a pending note
and any `constructs:` line *containing* `all` as covering everything. So the README was a pending note
that covered every construct there is: with the baseline amended and no note at all, the gate printed
`OK`. Its RED arms all used a scratch notes directory holding only their own fixture, so they proved the
mechanism and never the directory the gate actually reads. A pending note is now a line that is exactly
`- status: pending`, `constructs:` is a list compared exactly, and one arm runs against the deployed
directory itself. See `docs/semantics/migrations/README.md` for the two-commit workflow.

⛔ **Any movement needs a note, including a correction.** A note may say *"correction: the specification
was wrong and no description changes meaning"* — that is still explicit, which is the whole requirement,
and it costs one paragraph. A gate that tried to distinguish a bug fix from a language change would need
judgement it cannot have, so it does not try. The form, the workflow and the four sections a note carries
are in `docs/semantics/migrations/README.md`.

```console
$ scripts/check_language_freeze.sh              # the gate: green when every frozen construct agrees
$ scripts/check_language_freeze.sh --self-test  # its RED arms, one of them against the real tree
```

⭐ Neither command's output is quoted here, deliberately. Both print counts — how many constructs agree,
how many arms passed — and a count in prose is a count nothing re-derives: the repository's own register
of that defect class (`PROGRAM.20` in `docs/tasks/PROGRAM.md`) exists because figures copied into chapters
went stale for dozens of commits with every gate green. Run the command; do not read a number here.

⚠️ **Honest limit.** A digest proves a construct *moved*. It cannot prove that the note covering the
movement is correct, or complete, or that every description the change invalidates was found — that
residue is review, exactly as `BOOK-ANCHORS` states its own. The gate removes the cheapest failure, a
frozen construct edited silently, and leaves the expensive one to the reader.

## Every crate is in this book

`BOOK-ANCHORS` checks that what a chapter cites exists; `BOOK-COVERAGE` checks the other direction — that every
crate of the workspace appears in some chapter **beside a path into it**, so no capability exists that this
book never shows you. The crates are read from `Cargo.toml` rather than listed, so a new one is covered the day
it is added. A name alone does not count: a list of crate names would pass a name-only rule and tell you
nothing about where anything lives. When it was written, the scheduling checker — the `rt-analysis` crate —
was in no chapter at all; [What the scheduling checker establishes](analysis.md) now says which file holds what.

```console
$ bash scripts/check_book_coverage.sh
```

## Other repositories are read-only

`REPOSITORY-BOUNDARY` runs with the other doctrines on every commit. It checks the one place this
repository can see another: each vendored checkout it pins — today `vendor/linkedspec` — must be **at
its pin**, with **no commit made there**, **no file changed** and **no file created**. Reading a vendor,
building it by its documented route and moving our own pin to a commit it has published are all fine;
writing into it is not, and neither is accepting a change another project's agent wrote into this one
without a task-tree leaf that records who authorized it (`CLAUDE.md`, and
`docs/decisions/decision_repository-boundary-read-only.md` for the one time it happened).

```console
$ bash scripts/check_repository_boundary.sh              # the gate
$ bash scripts/check_repository_boundary.sh --self-test  # its RED arms, on scratch repositories
```

⚠️ It does not look inside the checkouts *nested* in a vendor. The vendor's own published bootstrap
moves and dirties those — thousands of entries, measured — and running a documented build is
consumption, so a gate that counted them would refuse every commit for doing what the rules allow.

## Scratch stays on this volume

Every temporary file this repository's scripts create lives under its own `target/`, never in the
system's temporary directory. `SCRATCH-LOCALITY` reads every tracked shell script and Rust source and
refuses a `mktemp` with no template under `target/`, a line naming the system temporary directory, and a
Rust `temp_dir()` call. When it was written, one ordinary run of the doctrines and their self-tests made
**26** temporary directories and files, **every one** of them off this volume, from 18 places in 15
files. Four of those places are in files this project takes from its scaffold. A local edit there would
be erased by the next sync, so the check lists them without refusing them, and the change they need is
written down for the scaffold's owner in `docs/decisions/decision_scratch-on-the-repository-volume.md`.

Moving a fixture changes what surrounds it. Two self-tests broke when their scratch moved under
`target/`: one because git ignores that whole directory, and one because Cargo, walking up from the
fixture, now found this workspace. The second is why the root `Cargo.toml` excludes `target`.

```console
$ bash scripts/check_scratch_locality.sh              # the gate
$ bash scripts/check_scratch_locality.sh --self-test  # its RED arms, on scratch repositories
```

## What comes from outside is written down

`SOURCE-LEDGER` keeps [What this project relies on from outside](ledger.md) true to the repository.
Every version this repository pins must appear in that chapter at the same version, and a chapter or
decision that names an outside source must link to its entry. When the check was written, no chapter
and no decision linked to anything outside, and the pins it found showed that the Rust compiler,
mdBook and the CI's actions are not pinned to exact versions at all (`PROGRAM.30`).

```console
$ bash scripts/check_source_ledger.sh              # the gate
$ bash scripts/check_source_ledger.sh --self-test  # its RED arms, on scratch repositories
```

## What is versioned is written down

`VERSION-REGISTER` keeps [What is versioned, and what changing it costs](versions.md) true to the code.
Every version the code declares must be on that page at the same value: each format identifier, each
version constant, each profile id, and the engine version, which every crate must agree on. When the
check was written, it found one version that the census planning it had missed, the model the
scheduling analysis reasons in, because the census looked for version constants by name and the gate
looks at what the value is.

```console
$ bash scripts/check_version_register.sh              # the gate
$ bash scripts/check_version_register.sh --self-test  # its RED arms, on scratch repositories
```

## Why `focused` runs the whole suite

§14.3 defines the focused tier as "format/type checks and **affected** contract tests", and
selecting affected tests needs change-impact machinery. Measured warm on this tree, the three
focused steps take **2.9 seconds** together. At that size the machinery would cost more than it
saves and would be one more thing to be wrong — so `focused` runs everything, and the measurement
is written into the runner's own source so the decision can be re-taken against a number rather
than re-argued from memory.
