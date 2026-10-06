# Verifying the toolchain

## The idea, in plain words

A tool that makes promises about other people's systems has to be checked itself, and more strictly than they are.
archogen's own checks come in **tiers**, from quick to thorough. The quickest runs on every change, in seconds: is
the code formatted, does it pass its tests. The next runs before work is shared: does the book still build, does the
code still compile for the emulated machine and for a web browser. Slower ones search for inputs that break the
code (*fuzzing*), plant bugs on purpose to see whether the tests notice (*mutation*), and run the code under a tool
that catches undefined behaviour (*Miri*). Each tier reports one of three outcomes — passed, failed, or *incomplete*
when something could not be run — because "nothing failed" is not the same as "everything passed".

Besides the code, the repository checks itself: its book, its records and its task trees are held to the code and
to each other by checks of their own, which [Annex B](annex-repository.md) describes.

> **In one minute, for engineers.** `ROADMAP.md` §14.3's five tiers — `focused`, `integration`, `extended`,
> `hardware`, `assurance` — are declared once in `xtask/src/main.rs` and run as `cargo xtask verify --tier …`. A tier
> exits passed, failed or incomplete (exit 20, never a pass), and CI passes an incomplete tier only with each owned
> gap annotated. `extended` runs fuzzing, mutation against catalogued defects and Miri, each shown able to fail. The
> engine compiles for `wasm32-unknown-unknown` and its browser module answers as the CLI does; the product spawns no
> process; the language is frozen at `eadl/1` with migration notes. The repository-integrity gates are in Annex B.

## How it works

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
| `integration` | before a push, and before closing a milestone | the above, plus the doctrine enforcer, every doctrine gate's own RED arms, the book build, the `no_std` build, the pinned toolchain's premises the port's catalog record rests on, the browser (`wasm32`) build, the pinned emulator, and the architecture spike run on it |
| `extended` | scheduled, or when a change touches parsing, arithmetic or event ordering | fuzzing, mutation, Miri |
| `hardware` | a change to target support, and every release gate | board regressions and timing observations |
| `assurance` | every supported release | trust inventory, claim completeness, source and binary identity |

## The precise rules

### A tier has three outcomes, not two

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
- **quarantined** — the step exists, its tool is present, it ran, and it could not reach a verdict
  because what it compares against is owned by a leaf that has not delivered it. §14.3 allows that
  only on terms: "Quarantine requires a named issue, owner, affected claim, and bounded scope." So a
  quarantine is one row of `QUARANTINES` in `xtask/src/main.rs` carrying exactly those four, and it
  covers one step's exit `20` and nothing else — the same step exiting `1` still fails, a step no row
  names exiting `20` still fails, and a quarantined step that *passes* is refused as a stale
  quarantine, so the leaf that closes the gap deletes the row in the same commit (leaf `PROGRAM.10.1`).

### What that looks like today

Rendered from a run, not retyped:

```console
$ cargo xtask verify --tier integration
tier: integration — before a push, and before closing a milestone
  ✅ fmt                  0.59s  every Rust source is in canonical format
  ✅ clippy              11.09s  no lint fires anywhere, including in tests and examples
  ✅ tests              117.82s  every contract test passes, F28 and the semantic corpus included
  ✅ doctrines           40.93s  every repository invariant holds on the working tree
  ✅ self-tests         167.49s  every doctrine gate's RED arms still fire — a gate that stopped being able to fail is caught here
  ✅ book                 0.23s  the mdBook builds, with its pinned release — it is the director's window, so a broken book is a broken deliverable
  ✅ no-std-build         0.24s  the runtime core compiles for a bare-metal target (§14.3's "compile targets")
  ✅ pin-premises         2.51s  every premise the catalog rests on about the pinned toolchain still holds: the port record's, each probe it prints read at its hash (§14.1–§14.4), and the gate's three claims (§3)
  ✅ wasm-build           0.65s  the engine's I/O-free crates, derived from the workspace, compile for the browser target
  ✅ wasm-binding         1.24s  the browser artifact imports nothing, exports what its record lists, and answers every tracked description byte for byte as the host build does, with archogen check's exit code
  ✅ emulator             0.33s  the pinned riscv-virt-up configuration renders and its toolchain is present (§3.2)
  ✅ spike                0.22s  code runs on the verified target: boot, a timer interrupt taken and returned from, the context preserved, output on the UART — and a clobbered context is caught (M2.8.4)
tier integration: passed — 12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
$ echo $?
0
```

Nothing is absent, and nothing is owed. The emulator step runs against the installed QEMU:
- it finds the pinned release and the pinned machine;
- it dumps the device tree and finds it identical to the recorded one (`M2.8.2`);
- it finds the target's eADL description in agreement with that tree (`M2.8.3.2`);
- it finds `TARGET_VERIFIED=yes`, which leaf `M2.8.3.4` set on that evidence.

The `spike` step then runs code on that target (leaf `M2.8.4`,
[Where generated systems run](targets.md)).

Until then the step was **quarantined**. It had run, nothing had disagreed, and the agreement it needed was not
written. So the tier said `incomplete`, exit `20`, and named the owning leaf. That was right both ways: a pass
would have claimed a target nobody had verified, and a failure would have called a missing comparison a
disagreement. `COMMIT.md` permits proceeding past `incomplete` after reading what it names, and never past
`failed`. When the gap closed, the quarantine row went with it in the same change, because a quarantined step
that passes is refused as a stale quarantine.

⭐ **The `self-tests` step is the newest, and it answers a different question from `doctrines`.** The
doctrines say the tree is clean; the self-tests say each doctrine gate can still *fail* — the RED arms of
every script that carries them, discovered by census from `scripts/` (the gates, the emulator tool, and the
self-test runner itself), plus `scripts/selftest_spine.sh`, which arms the gates the scaffold owns from
outside. Each runs as a CI runner would: the global and system git configuration hidden, and no identity
it did not set itself, so an arm that leans on the developer's `~/.gitconfig` fails here first. Until leaf `PROGRAM.28` nothing ran them, so an arm that broke, or began to pass
for the wrong reason, stayed invisible until someone happened to invoke it. It takes about forty seconds,
which is why it lives here and not on every commit.

⭐ **Of the five tiers, `hardware` and `assurance` are incomplete everywhere, and `integration` is incomplete
on a machine missing one of its tools. Saying so is the runner's most useful output.** Before
the runner existed, the fuzz corpus, the mutation harness, the Miri wiring, the board and the whole
assurance story were not *reported as missing*. They were simply not mentioned, which reads exactly
like being covered. Each gap was given an owner. The `extended` tier's three steps were built under
`PROGRAM.9`, and on `2026-09-30` it reported **passed** for the first time: `fuzz` in 3.75 s,
`mutation` in 8.00 s, `miri` in 996.69 s. The remaining gaps are named: `M5.1` for the board, and
`M3.6` / `M4.8` / `M4.7` for the assurance tier.

#### The `miri` step proves it can fail before it passes

The workspace contains no `unsafe` code, so a Miri run over it that passes looks exactly like a run
in which Miri saw nothing at all. `scripts/extended_miri.sh` therefore starts by building a
throwaway crate under `target/` whose one test reads through a dangling pointer. If Miri does not
refuse that test, the step fails before touching the real code. Miri comes from the `nightly`
toolchain, and until this step existed the tier asked the repository's own toolchain (then the `stable` channel) for it and
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

#### The `fuzz` step, and what it found on its first run

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

#### The `mutation` step: each defect, put back, must still be caught

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

### What CI does with an `incomplete` tier

It passes the job, and says so on every run. The policy is recorded in
`docs/decisions/decision_incomplete-blocking-policy.md` and carried out by `scripts/ci_integration.sh`:

| Tier exit | Verdict | The CI job |
| --- | --- | --- |
| `0` | `passed` | passes |
| `1` | `failed` | fails |
| `20` | `incomplete` | passes, with a warning annotation for each owned gap and a job summary headed "not a pass" |

Blocking on `incomplete` would keep CI red until another leaf lands. No change to the commit under test could
fix it, and a permanent red hides the next real failure. What makes a green `incomplete` safe is one flag. CI
runs the tier `--provisioned`: the environment claims to supply every tool the tier needs, so a missing tool is
that claim failing, and the job **fails**. A workflow that forgot to install `mdbook` cannot pass with a
warning. What is left that can make CI `incomplete` is only a gap a task-tree leaf owns: a quarantine, with its
four fields, or a step not built. Each gets there through a reviewed code change. On a developer's machine
nothing changes: a missing tool there is an absence, reported as one.

The script's arms run inside the real job, from the `self-tests` step. The first real run showed why that
matters. While the arms shared the job's log file, they truncated it under the running job, and the one real
gap went unnamed. They now keep their own log, and an arm checks that the real one is left untouched.

The job is `integration` in `.github/workflows/rust.yml` (leaf `PROGRAM.10.4`). It installs its tools at their
pins, not the runner image's versions. The image's QEMU is another release, which `--check` would refuse, so
`scripts/ci_provision.sh` builds QEMU `11.1.1` from its release tarball, and installs mdBook `0.5.2` from its
release asset. It refuses any download whose sha256 differs from the one recorded in `.github/ci-tools.env`.
Those pins are held to [the ledger](ledger.md#qemu) like every other. A cache keeps the built tools between
runs, keyed on the files that pin them.

Nothing on this machine can watch that workflow run, so `scripts/ci_rehearse.sh` runs the job's steps against
the repository as the runner receives it: one commit, checked out as [`actions/checkout`](ledger.md#github-actions) checks it out, nothing
untracked or built, no submodule, no global git configuration. Its first rehearsal, `2026-09-30`, passed:
every step green but the quarantined emulator, the gap annotated, the summary written. What it cannot
reproduce is the runner's own userland, GNU `sed` and `awk` where this machine has BSD ones. Leaf
`PROGRAM.10.5` reads the first real run for that, after the next push.

Every workflow runs on `push`, which a pull request's branch here triggers, so every workflow runs code a pull
request controls. `WORKFLOW-TOKENS` (`scripts/check_workflow_tokens.sh`, leaf `M2.7.6.1`) holds each to a read-only
token: a top-level `permissions:` granting no write scope and no job granting one, `persist-credentials: false` on
every `actions/checkout`, and no `pull_request_target` or `workflow_run` trigger, both of which run with the base
repository's token. It reads a plain subset of YAML and refuses the rest, so a file it cannot read is never passed.

The catalog check's harness, `scripts/catalog_check.sh` (`M2.7.6.2`), runs from a clean checkout of the commit a
pull request merges into. It writes the base and the judged tree from git's stored blobs, builds the checker from
the base under an allowlisted environment and the base's toolchain pin, and runs it from a target directory of its
own; every cargo configuration on the build's path must hold aliases only. Its self-test plants a build script, a
cargo wrapper, a toolchain file, a copy of the harness and hostile environment variables, and checks that none ever
runs; removing each protection turns an arm red. The checker is `xtask catalog-check` (`M2.7.4.2`); the tests use a stub.

`.github/workflows/catalog-check.yml` (`M2.7.6.3`) runs it on a pull request: the hosting's merge is checked out
where nothing runs from it, and the base is cloned beside it. The checker judges the merge against its base; on an
empty catalog the job passes. Whether a pull request's own copy of the file can satisfy the required check is for the hosting's rules,
which only the director can set, as is the repository's default token. The design's first independent review found
two ways past `WORKFLOW-TOKENS`, an escaped YAML key and a setting on a neighbouring step; both are now refused,
and so are a quoted key and a key written twice, or twice but for case, in one mapping
(`docs/reviews/catalog-check-protection-reviews.md`). Its next two rounds, each by a new agent context, were stopped
by the agent harness's own safety screening before they reported, so the next round is a reviewer the director
names.

### The engine compiles for the browser

The programmatic-interface decision promises a wasm binding, so the `integration` tier measures whether
the engine compiles for `wasm32-unknown-unknown` (leaf `API.1`). Its `wasm-build` step does not list the
crates. It takes every workspace member whose production code, the source before its first
`#[cfg(test)]`, names none of `std::fs`, `std::process`, `std::net` or `std::env`. It names each member
it leaves out, with the line that excluded it. A crate that uses the filesystem compiles for wasm32 all
the same, and fails when it runs. That is why such a crate is excluded rather than counted.

On `2026-09-30` the measured answer was yes: `eadl-model`, `archogen-evidence`, `rt-analysis`, `rt-core`
and `rt-reference` compile. Four members are excluded: the command-line tool, the runner, the S0 emitter,
which writes the crate it generates, and `eadl-front`, whose module loader reads files. The last is the
one a browser needs most, and its reads already go through one `ModuleSource` implementation, so the wasm
binding (`API.5`) can supply its own. It does: the engine API, `archogen-api`, and the binding, `archogen-wasm`, have since joined the set, and
the binding hands `eadl-front` its modules from memory.

### The browser module answers as the command line does

Compiling is not running. The `wasm-binding` step builds the module a web page loads
(`docs/decisions/decision_wasm-binding.md`) and runs it in [Node](ledger.md#node), through the same loader a
page uses (`crates/archogen-wasm/js/archogen.mjs`). It checks three things:

- **the module imports nothing.** A WebAssembly module can reach the outside world only through its imports,
  so one with none cannot touch a file, the network or a clock, whatever page loads it. This is read from the
  compiled module by the runtime's own parser, not from the source;
- **every tracked description gets the same answer.** Each one's response from the module is compared byte for
  byte with the one the same code gives when built for this machine;
- **and the same verdict as the command line.** Each response's exit code equals what `archogen check` exits
  with for that file.

It also runs the page's own code (`crates/archogen-wasm/page/page.mjs`) against a stand-in for the browser's
document. The page must load this module, answer its default description on load, and show exactly what the
engine-API chapter says it shows for that chapter's example. The director has since opened the page in a real
browser, Chrome `154.0.8037.58`. On load it showed `invalid-description (exit 10)` for the page's default
description, and after Check with the chapter's example it showed the chapter's answer line for line (leaf
`API.5.5`).

```console
$ bash scripts/wasm_binding.sh              # the step
$ bash scripts/wasm_binding.sh --self-test  # its RED arms: a module with an import, one with an unlisted export,
                                            # a response that differs, an exit that differs, a missing answer
```

Without Node the step is unavailable, which the tier reports and never counts as a pass.

The engine API (`archogen-api`, `crates/archogen-api/src/lib.rs`, leaf `API.3.2`) joined the compiled set the
day it was written, derived like the rest. It depends on `eadl-front`, which still compiles for wasm32 as its dependency. It never
calls the directory loader: a caller hands it a `ModuleSource`, and an in-memory one is what a browser
would pass.

### A third reader

`docs/semantics/grammar.md` defines what a well-formed description is, and until `2026-09-30` it had two
opinions about that: archogen's reader, and a recognizer the conformance suite derives from the grammar. Both
are this project's code. `scripts/third_opinion.sh` adds a third that nobody here wrote: [LinkedSpec](ledger.md#linkedspec)'s
`SExprDocumentV1` recognizer, run as an outside program (leaf `M1.22`). Each side is reduced to the same
shape, the forms of a document as raw source text and nesting. The script records, for every tracked
description, whether they agree. It compares token boundaries and whether the document is complete. It
never compares values: whether `-1` is an integer is eADL's own rule.

The first run agreed on every description but one. That one was a bug report's evidence file,
`(a" b"[c]{d})`, which the other recognizer refuses and archogen's reader accepted. The grammar-derived
recognizer sides with the outsider: the grammar requires a delimiter after every atom, and a string
followed directly by `[` has none. So archogen's reader had been more permissive than its own grammar, and
neither of the two existing opinions could see it. Since leaf `M1.37` the reader refuses it with
`read-missing-delimiter`, a conformance probe holds both of them to that, and all three readers agree on
every tracked description. The record
(`docs/semantics/third-opinion.txt`) fails the script the moment any document's agreement changes.

### The product runs nothing

archogen describes, checks and generates, and it executes nothing. `archogen build` writes a crate and stops.
Nothing in the product compiles, links or runs what it wrote, and nothing it does starts another process. That
is the property that makes it safe to hand archogen to an agent that did not write the description. The
`NO-SUBPROCESS` doctrine (`scripts/check_no_subprocess.sh`, leaf `API.2`) holds it on every commit: no
production code under `crates/*/src` may name `Command`, `spawn`, `exec` or `fork`. Test modules, `tests/`,
`xtask/` and `scripts/` compile and run things on purpose, and they are outside what it reads. A
`#[cfg(test)]` on a lone helper does not hide the lines below it. Only one that opens a test module does.

### When a push is due

The director ruled a push cadence on `2026-09-28`: push once the branch is a set number of commits ahead of
`origin/main`. The number has one copy, the **Threshold** field of `docs/decisions/decision_push-cadence.md`,
and `bash scripts/push_cadence.sh` reads it and reports the live distance (leaf `PROGRAM.23`). It exits `3`
when a push is due and `2` when it cannot tell, for example with no `origin/main` or with a record that gives
two answers. It is a report and never a gate. A push needs the director's authorization, so a check that
refused commits at the threshold would strand the work it is meant to protect.

### What the `tests` step is a suite *of*

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

### What is frozen, and what a digest can prove

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

### The freeze, and the note it demands

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

### Why `focused` runs the whole suite

§14.3 defines the focused tier as "format/type checks and **affected** contract tests", and
selecting affected tests needs change-impact machinery. Measured warm on this tree, the three
focused steps take **2.9 seconds** together. At that size the machinery would cost more than it
saves and would be one more thing to be wrong — so `focused` runs everything, and the measurement
is written into the runner's own source so the decision can be re-taken against a number rather
than re-argued from memory.
