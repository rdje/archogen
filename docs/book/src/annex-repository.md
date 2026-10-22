# Annex B: The checks that keep the repository honest

[Verifying the toolchain](verification.md) explains how archogen's code is tested. This annex is about the
repository itself — its book, its records, its task trees — and the checks that stop them drifting from the code or
from the truth. The doctrine checks run before every commit by the doctrine enforcer
(`scripts/check_doctrines.sh`) and again in CI; the handoff census runs before handing the workspace over. Each is
a script under `scripts/` with RED arms proving it can refuse. Registry entries carry a check identifier, a literal
text description and its script path; the descriptions execute no shell commands. The full doctrine register is
`DOCTRINE_ENFORCEMENT.md`. The doctrine accounts below moved here from the chapter so
that the chapter can stay readable; the move changed nothing in them but two links to the
[ledger](ledger.md) the source-ledger gate asked for.

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
`target/`: one because git ignores that whole directory, and one because [Cargo](ledger.md#rust-toolchain), walking up from the
fixture, now found this workspace. The second is why the root `Cargo.toml` excludes `target`. For the same reason
the CI rehearsal checks out beside this repository rather than under it, the one exception the director ruled, and
the repository's own data store is a git-ignored `.archogen-data/`
(`docs/decisions/decision_ci-rehearsal-beside-the-repository.md`).

```console
$ bash scripts/check_scratch_locality.sh              # the gate
$ bash scripts/check_scratch_locality.sh --self-test  # its RED arms, on scratch repositories
```

### Daily artifact cleanup

Daily artifact cleanup has a task-tree owner and a metadata inventory before deletion. The latest date and summary
live in `docs/ARTIFACT_CLEANUP.md`; the owning leaf keeps the deletion list, regeneration commands, retained evidence
and cold verification. Compiler caches may be regenerated; cited review evidence, tools and dependency stores are
retained unless their removal is proven safe. Cleanup respects the same read-only boundary around other Git
repositories. The record is overwritten after each run rather than growing into another history.

Cleanup (`2026-10-18`, `PROGRAM.86`) removed only the main and empty wasm debug incremental caches:
928 files totaling 344389410 logical bytes, without an allocated disk-space claim. Independent
filesystem metadata checks agreed; immediate residue and maintained tracked-input restoration passed before
and after focused/wasm regeneration checks. Existing dependency/build products, cited evidence and Git
workspaces were retained. The metadata/deletion adapters are untracked supplementary evidence; this verifies
regeneration after incremental removal, not a complete rebuild from source.

Cleanup (`2026-10-19`, `PROGRAM.93`) removed 2863 files totaling 929596234 logical
bytes from the main and owned native-sweep incremental caches, plus the empty wasm cache.
This is logical file size, with no allocated disk-space claim. The
[Cargo cache reference](https://doc.rust-lang.org/cargo/reference/build-cache.html)
identifies incremental output as a compiler cache; actual locations and behavior here were
checked with the pinned toolchain.

| Repository-relative location | Regular files | Logical bytes | `.bin` / `.log` | Action |
| --- | --- | --- | --- | --- |
| `target/debug/incremental` | 965 | 415272869 | 40 / 0 | Remove cache |
| `target/wasm32-unknown-unknown/debug/incremental` | 0 | 0 | 0 / 0 | Remove empty cache |
| `target/mutation-sweep/target/debug/incremental` | 1898 | 514323365 | 42 / 0 | Remove owned cache |
| `target/debug/deps` | 800 | 671631278 | 0 / 0 | Retain build products |
| `target/release` | 117 | 17080485 | 0 / 0 | Retain build products |

Physical metadata checks every parent, directory and file for the repository volume,
regular-file/directory type, symlinks and nested Git markers. Independent native `find` and
`/usr/bin/stat` agree on every location's counts and logical bytes. Before deletion, the native
process census reports no project background jobs; repeating the complete inventory must match
all paths, identities, modes and sizes. Only the fixed incremental directories are deleted,
and immediate existence checks prove each gone. Raw evidence, dependency stores, final build
products and Git workspaces remain. The ignored metadata/deletion adapters are supplementary;
maintained input checks, native system tools and documented builds are primary verification.

All 1098 captured regular inputs restore after deletion and again after the workflows under
Python/system hashes, bytes and modes; the gitlink remains opaque metadata. Focused verification
passes formatting, strict lint and complete tests in 153.07 seconds; the documented wasm build
also succeeds. Retained build products may be reused, so this checks workflow after cache
removal without claiming a full dependency rebuild from source.

```sh
make focused
bash scripts/wasm_build.sh
```

Held activation tree `c64735bfcf5bf93f561625a132221cbbb4a35303` covers these checks; final
book/snapshot staging is a later documentation epoch. Receipts are under
`target/artifact-cleanup/2026-10-19/`. The previous task seals exactly with all 288 older
archives unchanged. The task-tree root's child summary now includes the already defined
children beyond its former `.89` endpoint. The latest-only record retains the removal receipt's
exact recorded time. Cleanup ran shortly before the daily boundary ahead of the next long
controller sweep; current controller measurement resumes next.

## Before handing the workspace over

Prepare the temporary environment in each new shell before diagnostics, including shell input that
may be materialized before its consumer starts. Run from the repository root:

```bash
mkdir -p target/cargo-home target/cache target/tmp
export CARGO_HOME="$PWD/target/cargo-home" XDG_CACHE_HOME="$PWD/target/cache"
export TMPDIR="$PWD/target/tmp" TMP="$PWD/target/tmp" TEMP="$PWD/target/tmp"
```


```console
$ bash scripts/handoff_census.sh             # the project handoff command
$ bash scripts/handoff_census.sh --self-test # synthetic failures and real process controls
```

The command checks for project work still running. A process named by the native census counts when a second
sample still holds its process number and command line, eight seconds later by default. A short-lived terminal helper can
drop out; a persistent job prevents handoff. Each sample must complete its actual OS process snapshot and
open-handle read. If either fails, is denied or is missing, the wrapper exits **2**, says the outcome is unknown,
and withholds a `handoff: OK` verdict. A process that exits during an ancestry lookup does not invalidate the full snapshot.
The native scaffold file stays unchanged; the wrapper is the project command (`PROGRAM.80`).

Exit **0** means no project job persisted across the samples; exit **1** names one that did. The check is a snapshot:
a job starting after the first sample is not counted unless another run finds it, and a job whose process number and command
line both change can escape the comparison. Run it after the work and its background checks finish.

The handoff self-tests (`PROGRAM.92`, `2026-10-18`) use acknowledged sampling phases. A worker is
ready before the first snapshot; the transient worker is released and reaped between snapshots,
while the persistent worker stays held through both. A deliberately delayed real snapshot checks
this protocol. Large sampled output is matched from an explicit project-local file, avoiding a
`pipefail` false failure when a short-circuit consumer closes a pipeline early. Missing evidence,
persistent PIDs and unknown reader status have rejecting controls. Required OS-read failure controls
remain part of the suite. The aggregate CI report did not retain its sampled process text, so the
reproduced mechanisms are not attributed as the exact historical occurrence. Full integration
subsequently passes in `M3.6.9.6.3.2.2.13.3.2.2`; [the final checkpoint](annex-trust-proofs.md#final-prefixhelper-checkpoint)
records its activation and scope.

### Shell input storage and invocation environment

`PROGRAM.92` (`2026-10-18`) measures implicit shell input storage on installed Bash 5.3.15.
A protected large here-string supplies a regular input file of 1600001 bytes under `target/tmp`;
native descriptor names and filesystem identity confirm its location. The inspected task-history
here-document instead supplies an anonymous pipe. A pipe's device identity differs from the repository
without establishing an off-volume filesystem write. Determine storage type before interpreting it.
These are host observations, not a new shell-version pin or a claim about every redirection.

Early diagnostic invocations omitted an explicit shell temporary environment. Read-only metadata
confirms the inherited host default is on another volume. Off-volume scratch for those untraced
large-input invocations is an inference from the observed shell behavior; individual historical
filenames were not captured. No shared temporary data was deleted, and no historical residue-absence
claim follows. The protected file observed above is gone after its command completes.

Every diagnostic invocation prepares `target/tmp` and exports `TMPDIR`, `TMP` and `TEMP` from the
current root before redirections or child shells. Prior tool calls do not retain those exports.
The handoff self-test also seeds its own temporary environment and reads large sampled output from
an explicit project-local file. `CLAUDE.md` and `TOOLBOX.md` carry the invocation requirement so it
survives context recovery. No foreign repository or scaffold-owned file is changed.

## A commit holds what was run

Every check before a commit reads the working tree, and a commit records the index. A new file left untracked is
in the first and not the second, so a commit can record a tree nobody ran: one did, a module declared and its file
not committed, so the committed tree did not build. `UNTRACKED-CODE` refuses a commit while a file in a code path —
what `.doctrine/code_paths.txt` says is code — is untracked; an ignored file, or a note outside every code path,
passes. A leaf's evidence can be stale the same way: a box that cites a passing `make focused` run, made before the
last file changed. So the focused tier, when it passes on a tree that did not move during the run, writes a stamp
naming that tree, and the same check refuses a staged leaf that cites the run unless the staged tree has its stamp
(`PROGRAM.53`). Successful evidence may use the legacy arrow, or an explicit successful exit:

```text
- [x] NO REGRESSION — `make focused`, exit code 0: all steps passed
```

The exit may continue on the immediately following added line. Plans and explicit failed or incomplete
exit claims do not assert success. A continuation cannot borrow a result from another file or hunk.
The matcher consumes the full staged diff: closing its pipe early could make Git fail under `pipefail`
and skip the stamp check for a large diff. Permanent controls hold missing, wrong and matching stamps
for both evidence styles and the large-diff case (`M3.6.9.6.3.2.2.9.5`).

```console
$ bash scripts/check_untracked_code.sh              # the gate
$ bash scripts/check_untracked_code.sh --self-test  # its RED arms, on scratch repositories
```

## What comes from outside is written down

`SOURCE-LEDGER` keeps [What this project relies on from outside](ledger.md) true to the repository.
Every version this repository pins must appear in that chapter at the same version, and a chapter or
decision that names an outside source must link to its entry. When the check was written, no chapter
and no decision linked to anything outside, and the pins it found showed that the Rust compiler,
[mdBook](ledger.md#mdbook) and the CI's actions are not pinned to exact versions at all (`PROGRAM.30`).

```console
$ bash scripts/check_source_ledger.sh              # the gate
$ bash scripts/check_source_ledger.sh --self-test  # its RED arms, on scratch repositories
```


Three of those pins were once moving targets: the Rust channel `stable`, the CI actions' `v4` tags and whatever
mdBook was installed. Each now names one release (leaf `PROGRAM.30`). `rust-toolchain.toml` says `1.95.0`, with
the components and the bare-metal target the tiers use, and CI installs exactly that file with
`rustup toolchain install`, so the version is written once. The actions are referenced by commit. The `book` step
builds only with the pinned mdBook, because every `ledger.md#…` link depends on how mdBook derives a heading's
anchor. The day the Rust pin was set, `1.98.0` passed the same format check, lints and test suite, so a bump
starts from evidence rather than from hope.

## JSON used by the tooling

The tooling's JSON reader serves Cargo metadata, build-message inventory inputs and the package verifier.
`json::parse` returns a value tree or a diagnostic; unexpected-end and unterminated-string/escape errors
omit a byte offset. `json::write` deterministically encodes that tree with ordered object keys and escaped
strings. Their API documentation is attached to the corresponding functions (`.9.3`).

Number reading follows [RFC 8259](ledger.md#rfc-8259) §6: an optional minus, an integer, then an optional
fraction and exponent. Fractions and exponents require digits; leading zeros and missing delimiters are
refused. The reader keeps the complete original spelling as text, without floating-point conversion:

| Example input | Number behavior |
| --- | --- |
| `0`, `-0`, `12.50e+3` | accepted, spelling retained |
| `1e400`, `123456789012345678901234567890.123456789` | accepted as text, without range/precision rounding |
| `01`, `1.`, `1e+`, `1-2` | refused |
| `[1e]`, `{"n":1e}` | refused in arrays/objects as at the document root |

Permanent controls hold valid spellings and aggregate delimiters. The exact old number reader reversal is
killed by the malformed-number control; `.9.6.1` records the unchanged old code/test bodies outside its numeric
block at that commit. Run the reader's checks with `cargo test -q -p xtask json::tests::`.

Unescaped U+0000–U+001F string characters are refused, following §7. That applies at the document root,
in arrays, in object values and in object keys. Escapes may still represent those characters:

| String input | Behavior |
| --- | --- |
| `"a\nb"`, `"a\u000Ab"` | accepted; the decoded line feed is preserved |
| `"a\u0000b"` | accepted; the decoded NUL is preserved |
| A quoted string containing a literal line feed or NUL | refused; it needs an escape |
| Raw space, U+007F, U+0085, `é` and `😀` | accepted |

Offsets for unescaped-control errors count bytes in the UTF-8 input, including the opening quote. Permanent controls
hold the entire forbidden range, valid escaped forms, object-key decoding and permitted raw boundaries;
deleting only the guard gives the named finishing kill (`M3.6.9.6.3.2.2.9.6.2`).

Each `\u` escape consumes exactly four ASCII hexadecimal digits, as §7 requires. A leading plus or minus,
non-hexadecimal text and a short escape are refused. An ordinary character following those digits remains
part of the string: `"\u00410"` decodes to `A0`, while `"\\u+123"` decodes to the literal text `\u+123`.

The reader stores Rust scalar strings. A high-surrogate escape must therefore be followed immediately by
another `\u` escape in the low-surrogate range. The decoder validates that range before joining the pair:

| Unicode escape input | Behavior |
| --- | --- |
| `"\uAbCd"`, `"\uE000"`, `"\uffff"` | accepted; mixed hexadecimal case and scalar boundaries preserved |
| `"\ud800\udc00"`, `"\uDBFF\uDFFF"` | accepted; smallest and largest paired scalar values |
| `"\ud83d\ude00"` | accepted as `😀` |
| `"\u+123"`, `"\u00g0"`, `"\u123"` | refused; invalid escape digits |
| `"\ud800\u0041"`, `"\udbff\ue000"` | refused; the second escape is not a low surrogate |
| `"\ud800"`, `"\udc00"` | refused under the scalar-string policy |

These checks apply in values and object keys. Permanent controls also hold decoded values and writer round
trips; independently restoring the old surrogate join or deleting the hexadecimal guard gives the corresponding
named finishing failure (`M3.6.9.6.3.2.2.9.6.3`). Lone-surrogate refusal is the reader's existing narrower policy:
§8.2 permits unpaired surrogate spellings in the grammar. This is not a complete conformance claim.

The package verifier requires valid UTF-8 for both `manifest.json` and `trust/trust-dependencies.json`.
It refuses invalid inventory bytes before parsing them, with `trust-inventory-stale`; the manifest's existing
strict decoding follows its unreadable-manifest route. An encoded replacement character is ordinary valid text:

| Bytes within a quoted string | Package JSON behavior |
| --- | --- |
| `EF BF BD` | accepted as `U+FFFD`, when the remaining package checks agree |
| `FF`, a lone `80`, an overlong sequence or an encoded surrogate | refused as invalid UTF-8 |
| A truncated multibyte sequence | refused as invalid UTF-8 |

The report still names the hash of the original inventory bytes. A matching hash does not make malformed text
valid. The public verifier reproduction in `M3.6.9.6.3.2.2.9.6.4` admitted an invalid inventory before strict
decoding and refuses it afterward, with report binding and shipped artifact bytes held correctly. The exact old
lossy admission is killed by the permanent package control; valid encoded replacement characters remain accepted.
This closes the caller-byte assessment before `.9.4` integration. It establishes malformed-input admission,
without demonstrating an artifact-hash bypass or authenticity. Encoding follows [RFC 8259](ledger.md#rfc-8259)
§8.1's interchange context; these local checks are not a complete conformance claim.

## Configuration checkpoint integration

The local integration checkpoint passed on `2026-10-12` at commit
`1cf0aafb9f134fb108852f6b91984f99d0550188`, after the configuration identity, JSON boundary and effects
self-test repairs. Every required step passed, including the complete contract suite, doctrine self-tests,
pinned book build, bare-metal and browser builds, browser binding, emulator check and architecture spike.
The integration stamp identifies that committed tree; later checkpoint documentation is recorded separately.

This is local integration evidence. Complete current-source mutation adequacy and runner-baseline compatibility
remain open. Supplemental restoration counts at this checkpoint used an ignored one-off helper. The maintained
replacement and subsequent native replay are recorded below; historical helper limitations remain explicit.

## Checking restored inputs

The adequacy diagnostic can capture the inputs in the Git index, then check that a native run restored them:

```sh
bash scripts/trust_adequacy.sh --snapshot-inputs --output target/trust-inputs.json
# Run the task's native command and consume its own verdict.
bash scripts/trust_adequacy.sh --check-inputs target/trust-inputs.json --output target/trust-input-check.json
```

Snapshots use `archogen-tracked-inputs/1`; check receipts use `archogen-tracked-input-check/1`. A field's meaning
or required shape changing requires a new schema identifier; the snapshot reader refuses unknown schemas,
missing/extra fields and duplicate JSON keys. These formats do not accept the earlier one-off helper dictionaries.
The snapshot records the commit, producer hash, indexed path/mode/blob identity, working file permissions,
byte size and SHA-256. Every regular file's Python hash must agree with the system hash, using `sha256sum`
or `shasum -a 256` on standard input. This avoids parsing filenames from hash-tool output: spaces,
quotes, tabs, newlines, backslashes and Unicode stay exact. The Git index and commit must remain stable while
the census runs. Gitlinks contribute their indexed object identities only; their directories and implementation
are never read. Ignored and untracked files, including a sweep's copied workspace, are outside this population.

Snapshot and result paths must be canonical repository-relative paths under `target/`, on the repository's
volume. Symlink components, other volumes and nonregular evidence files refuse. Git, Python and an independent
system hash executable are required runtime tools; those external executables are used read-only. Generated
evidence and synthetic Git self-test workspaces remain project-local.

Outputs cannot overwrite indexed inputs, enter a gitlink or cross a nested Git boundary. Snapshot aliases refuse.
Result files are replaced atomically, so an existing hardlink cannot cause a tracked input to be truncated.

Exit zero means capture or complete restoration succeeded. Changed bytes, permissions, index entries, commit,
producer or gitlink identities refuse, as do malformed snapshots and unavailable tools. Refusal exits two and
does not count as a partial pass. A started check writes a `pending` result and replaces it with `restored` or
`refused`; an interruption can leave `pending`. Consume the exit status and the receipt's snapshot hash together.
Verification cannot overwrite its snapshot. It compares boundaries, restores no files itself, and requires the
caller to hold tracked inputs fixed throughout the run; it is not a continuous change monitor.

`bash scripts/trust_adequacy.sh --self-test` exercises this interface on synthetic Git workspaces, including
intentional byte/index/mode/pin changes, absent files/tools, malformed evidence, special filenames and storage
refusals. The existing planning, progress traces, population measurements and equivalence checks retain their
interfaces. Input restoration evidence establishes no mutation kill, semantic classification or complete
production adequacy: retain the native command's actual finishing verdict separately.

Earlier supplemental restoration counts used ignored one-off helpers, as their task records qualify. This
maintained producer supports fresh measurements; it does not retroactively make those historical helpers durable.

The native replay on `2026-10-12` used committed code, catalogue and producer `d33a6e0`. Each entry below
produced its named finishing kill with strict success, and the committed census verified complete input restoration
after each entry, including unchanged gitlink metadata. The task record binds the snapshot, producer, source and
catalogue hashes and retains per-case native exits and restoration receipts.

| Native entry | Regression exercised |
| --- | --- |
| `json-number-grammar-unrestricted` | Incomplete numeric grammar admitted |
| `json-raw-string-controls-admitted` | Unescaped controls admitted in strings |
| `json-surrogate-low-half-unchecked` | A high surrogate joined to an invalid low half |
| `json-unicode-escape-plus-admitted` | A leading plus accepted as a hexadecimal digit |
| `trust-verifier-lossy-inventory-text` | Malformed inventory bytes accepted through lossy decoding |

For example, run the numeric reversal between snapshot and restoration checks with repository-local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" \
  XDG_CACHE_HOME="$PWD/target/cache" ARCHOGEN_MUTATE_TIMEOUT=600 \
  cargo xtask mutate --timeouts-fail --only json-number-grammar-unrestricted
```

This closes the scoped restoration durability repair. Complete current-source mutation adequacy remains open;
these native boundary regressions do not close additional rows of the initial `trust.rs` survivor census.

The helper's historical measurements, exact initial/current edit bindings and restoration receipts remain
in `docs/tasks/M3.md` and its linked archives. A real Cargo apostrophe path exposed a collection failure:
the old helper mistook an escaped apostrophe for a quote delimiter and corrupted the output-directory token.

The repair decodes backslash escapes outside single quotes and preserves literal backslashes inside them.
For example, the quoted path fragment `'with'\''quote'` becomes `with'quote`; `'with\slash'` retains its
backslash. Arguments following either path remain separate. The existing token-vector interface and
configuration normalization rules are unchanged.

A permanent public inventory test now builds two roots with a shared dependency using ordinary, space,
apostrophe, space-plus-apostrophe, backslash and backslash-plus-apostrophe output directories. Every build must
retain both units per root, identical normalized configurations and identical shared items. It failed with the
old helper and passes with the repair. Run it with project-local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::cargo_quoted_output_paths_preserve_inventory_units_and_configuration -- --exact
```

The exact old-helper reversal `trust-cargo-apostrophe-escape-unhandled` gives the permanent test's named
failure. This holds the [pinned producer](ledger.md#rust-toolchain), without a general shell guarantee.
Configuration comparisons alone missed a consistently omitted dependency argument; independent expectations
are therefore required alongside them. Full current-source adequacy remains open.

The independent retained-argument control now builds two roots that call a shared `common` library and requires
these arguments in each root's own normalized unit configuration on the pinned producer:

| Root | Required dependency argument |
| --- | --- |
| Binary `gen` | `--extern common=libcommon.rlib` |
| Library `chk` | `--extern common=libcommon.rmeta` |

Run the public control with the same repository-local stores shown above:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::a_root_s_configuration_retains_its_dependency_argument -- --exact
```

The exact final-push reversal `trust-cargo-final-argument-dropped` and both actual final-token candidates
now yield named failures, with strict exit zero and maintained input restoration. Remaining survivors need
precise domain judgments and a final all-helper replay.

Inventory compilation sets `RUSTC=rustc` under the same pinned environment as its recorded compiler identity.
Cargo's formatter leaves its program unquoted; this fixed name prevents a spaced installation path from being
mistaken for arguments. Inherited compiler overrides remain cleared. The command helper and normalization
rules are unchanged.

The permanent Unix public control uses separate child environments with ordinary, space, apostrophe, backslash
and combined toolchain homes. It requires each root to retain exactly its own and the shared package's units,
plus the independently expected library dependency argument shown above. Local symlinks access the installed
OS/toolchain dependency read-only; no other repository is written. Run it with project-local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::compiler_installation_paths_preserve_inventory_units_and_configuration -- --exact
```

The exact compiler-selection reversal and actual statement deletion both give this control's named failure,
with strict exit zero and maintained input restoration.

Actual successful Cargo rustc builds can also print empty arguments: `-A "" -W ""` produces unknown-lint
warnings but compilation succeeds on the pin. The permanent Unix parent control compares these empty operands
with nonempty `dead_code` controls and requires the independent final sequence `[-A, operand, -W, operand]`,
preserving both middle and final empty tokens. These extra diagnostic flags exercise the helper's documented
Cargo-format promise; inventory builds do not admit caller-supplied rustc flags. Run it with local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::cargo_verbose_empty_arguments_keep_their_token_boundaries -- --exact
```

The exact reversal `trust-cargo-empty-arguments-lost` and actual empty-token-loss edits yield this test's
named failures. Earlier failing check-cfg probes established no absence premise. Delimiter proofs and final
helper closure remain open.

The dependency reader matches the known raw first `.d` target and consumes complete space-escaped scratch/
canonical prefixes before splitting catalog-grammar suffixes. Relative sources and absolute in-tree
`include_str!` inputs retain line breaks; missing/empty targets refuse. No general Makefile grammar is claimed.
Run the permanent producer control:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::cargo_dependency_information_retains_actual_source_paths -- --exact
```

Independent source sets hold across ordinary, line-break, space, backtick, tab, backslash and hash paths.
A synthetic malformed-target control includes colon-space: actual Cargo on this host refuses colon output paths
when joining its dynamic-library search path. Exact reversal `trust-dependency-first-physical-line` gives the
producer's named failure; maintained census verifies originals restored.

Environment records escape backslashes and line breaks. Normalization encodes only known scratch/canonical
prefixes before replacement. A real newline becomes backslash-n; literal backslash-n becomes escaped
backslash followed by n. Neither is decoded into the other.

The permanent producer control requires relative `CARGO_MANIFEST_DIR=crates/a` and exact encoded description/
unset records across ordinary, line-break, backslash, literal-backslash-n and source-marker roots.
A separate target permits the marker's colon in the source root; a distinct canonical-prefix control also passes:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::cargo_dependency_environment_normalizes_escaped_scratch_paths -- --exact
```

Exact reversal `trust-dependency-environment-prefix-unescaped` gives its named failure; originals restore.
These controls bind the [pinned writer](ledger.md#rust-toolchain).

Complete Cargo commands are framed before token decoding. Quoted newlines, carriage returns and backticks
stay inside arguments. Cargo's Compiling/Checking status prints package paths without quoting. The reader
consumes complete known scratch/canonical prefixes in those messages before physical line breaks, so path
text resembling a Running command cannot create another unit. Empty/unclosed records, unquoted breaks and trailing junk refuse without panics.

Run the public actual-producer control:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  cargo test -p xtask trust::tests::cargo_multiline_output_paths_preserve_complete_inventory -- --exact
```

It independently requires root/dependency units, source/data hashes, environment and retained dependency
arguments across ordinary, line-break, backtick, fake-Running and combined apostrophe/backslash paths.
Configurations/shared items match ordinary. Synthetic quote/canonical/malformed controls and exact reversals
`trust-cargo-command-first-physical-line` and `trust-cargo-status-path-split` hold both boundaries; originals
restore (`M3.6.9.6.3.2.2.10.3.3.3.2.3`). Delimiter proofs and their limits are
[source-bound](ledger.md#rust-toolchain); bounded replay/integration pass; full-production adequacy remains open.

## Historical effects and current self-tests

The effects diagnostic keeps the [initial semantic witness review](trust-adequacy.md#semantic-witnesses-in-the-initial-cohort)
bound to its original source. Candidate IDs encode source positions and operator order; a current plan can have
different IDs after a repair. Feeding it to the fixed initial witness table makes the self-test fail at cohort
membership, before the semantic trace runs.

The self-test now uses a compact tracked fixture, `scripts/fixtures/trust_witness_effects_initial.json`, for those
historical compiler controls. It retains the actual replacement text needed by the controls, the original input
hashes and the reviewed commit identity. The diagnostic pins the fixture's bytes and refuses altered fixture
data. The positive trace and both seeded wrong claims generate the same Rust programs as the original manifest;
their assertions and contrasted behavior are unchanged. The fixture holds selected compiler traces, without
native outcomes or a complete cohort, and does not classify the current production source.

Spelling controls still discover the current plan and check its source headers, first/last/ends forms and refusal
arms. Historical semantic traces and live spelling checks therefore use their own inputs. Run both through
`bash scripts/trust_witness_effects.sh --self-test`. A clean checkout needs no ignored initial evidence or old Git
history for this self-test. Measurement mode retains its original source/native evidence checks.

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

## A bug report says what its issue says

The bugs this project reports to the tools it uses live in `docs/feedback/`, one register per vendor
and one directory per bug. `FEEDBACK-REGISTER` checks that each register row states what its bug's
own page states, that a `verified` bug was re-measured on the date the register gives, and that every
total is a recount. When it was written, the index above the registers still said [LinkedSpec](ledger.md#linkedspec) had five
open bugs and two blockers, three days after all five had been verified.

```console
$ bash scripts/check_feedback_register.sh              # the gate
$ bash scripts/check_feedback_register.sh --self-test  # its RED arms, on scratch repositories
```

## The status pages stay short

`LIVE_STATUS.md` and `docs/TASK_TREE.md` show where each part of the work stands now. They grew into
something else. Each finished task appended a note to its area's row, until `LIVE_STATUS.md` held
42,110 bytes in 21 lines, one row alone being 30,256 bytes long. What each task did already lives in
its tree's Commit Log and in `CHANGELOG.md`, so the rows were cut back to status, next task and any
blocker, after checking that every task they mentioned had its own record. `LIVE-SNAPSHOTS` now
bounds these pages, `MEMORY.md` and `README.md` on lines, bytes and longest line. The longest line is
counted separately because a page can keep a short line count while one row grows without limit. The
landing page's limits were measured from the page itself, after a review against `README_POLICY.md`,
rather than taken from the generous defaults the project's template started with.

```console
$ bash scripts/check_live_snapshots.sh              # the gate
$ bash scripts/check_live_snapshots.sh --self-test  # its RED arms, on scratch repositories
```

## The histories are sealed as they grow

`CHANGELOG.md` and `DEV_NOTES.md` gain an entry with nearly every commit. They are rolling ledgers: once one holds
twice its window of entries, its oldest window is moved, byte for byte, into the next numbered file under
`docs/history/`, and `docs/history/INDEX.md` lists each such file with its range, its size and a fingerprint.
Nothing is rewritten. Reading the live file and then the sealed files, newest first, gives the whole history
exactly as it was written (`docs/decisions/decision_history-ledgers.md`). `HISTORY-LEDGERS` checks on every commit
that no sealed file has changed since it was sealed, that the index lists every one and has lost or changed no row it ever held, that the order runs on
unbroken, and that neither live file has outgrown its window.

```console
$ bash scripts/check_history_ledgers.sh              # the gate
$ bash scripts/check_history_ledgers.sh --seal       # a rollover the gate asks for, with its proof
$ bash scripts/check_history_ledgers.sh --self-test  # its RED arms, on scratch repositories
```

## Finished work leaves the task trees

A task tree records every leaf of its work, with its checklist and evidence. Most of it is finished work: on
`2026-09-30`, 77% of `docs/tasks/M1.md` and 79% of `docs/tasks/PROGRAM.md`.
- **What moves.** Once every leaf of one of a tree's top-level parts is done, those leaves move, byte for byte, into
  one file under `docs/task-history/`. A top-level part can stay open for long — one of its leaves waiting on a
  decision — so, below an open one, each finished part moves the same way, the largest it can, into a file of its own;
  when the open part closes, the rest of it follows into its own file (`PROGRAM.69`).
- **What stays.** Each leaf leaves two lines in the tree, its name and a link to where its text now is. The live
  work and the list of what is next stay where they were.
- **The logs follow** (`2026-10-20`, `PROGRAM.95`). A tree also keeps two logs, one row per check and one per
  commit. Once a leaf has moved, its rows in both are finished work too, and the same tool moves them, byte for
  byte and in the tree's order, into numbered files beside the leaf files, as many as a byte budget needs, each
  listed in the index with its digest; a row naming a leaf still in the tree, no leaf or several stays, and so
  does a row not yet committed, until the next run (`docs/decisions/decision_task-log-row-sealing.md`).
- **The proof.** Before anything is written, the tool checks that putting every leaf back would give the tree as it was,
  byte for byte (`docs/decisions/decision_task-tree-sealing.md`). After writing, it runs the check below on the result,
  and undoes every write if the check refuses or an error stops it — each file written whole or not at all, and whatever
  it could not undo named, as on a full disk, or a folder it made that another writer has put a file in. A request to
  stop that comes once the tool has begun writing waits until it is done, so the history is whole either way; only a
  process killed outright — by `SIGKILL`, one of the few fault signals left unheld or, on Linux, a signal the C library
  keeps for itself — aborting, crashing or the machine stopping leaves its writes, for the next check to name. So the
  history admits every line a tree may hold: its line ceiling is the trees' own
  (`docs/decisions/decision_task-history-line-ceiling.md`).
- **What it still carries.** A sealed leaf keeps the hand-offs it quoted; the check that each design's hand-off is
  quoted word for word by its leaf reads the sealed text, never the two-line placeholder (`PROGRAM.63`).
- **The first seal**, on `2026-09-30`: 112 leaves left `M1` and `PROGRAM`. At the sealing commit, `M1.md` went
  from 7 633 lines to 2 278, and `PROGRAM.md` from 4 280 to 978.

`TASK-HISTORY` checks on every commit, in CI as well as before a commit:
- that no sealed file has changed since the commit that sealed it;
- that `docs/task-history/INDEX.md` lists every sealed file and has lost or changed no row it ever held;
- that a failed read of a historical index is refused unless Git independently proves the path absent at that commit;
- that every sealed leaf has exactly one two-line placeholder in its tree, linking the file that holds it;
- that every sealed leaf is, byte for byte, what its tree held just before it was sealed, so a seal made by hand or
  edited is refused when its files, its placeholders' links or its parts differ from what the tool makes — each file
  the largest finished part, whole. A placeholder's commit, a row's date and the rule against a line at column 0 are
  the tool's alone, which the check does not read, and so are where a placeholder stands in its tree, any text after
  the name on its first line, a seal of only some finished parts, and prose in the index outside its tables;
- that no live leaf sits inside a part that is sealed, in any tree directly under `docs/tasks/` (one in a sub-folder is
  `PROGRAM.72`'s), and that each tree holds only its own leaves, each named once; that every sealed file holds a leaf;
  and that the index keeps one table per tree, its rows under it.
- that every file of moved log rows is the form the tool writes and matches its index row; that its rows were the
  tree's own just before the commit that moved them, in that order and under that log's header; that every row
  that log then held for a sealed leaf is still in the tree or in those files, none invented; and that each row
  names one sealed leaf of that tree.

```console
$ bash scripts/check_task_history.sh                  # the gate
$ bash scripts/check_task_history.sh --seal M1        # seal what has closed in a tree, with its proof
$ bash scripts/check_task_history.sh --census HEAD    # done leaves under open top-level parts, in bytes
$ bash scripts/check_task_history.sh --self-test      # its RED arms, on scratch repositories
```

`--census` requires a successful tree listing and a readable blob for every selected tree. A failed selected read
names its commit and path, exits 1 and produces no final aggregate; earlier per-tree rows do not make it a successful
measurement. A genuinely empty selected tree remains valid and contributes zero bytes. Paths come from a
NUL-delimited UTF-8 listing, so Unicode, tabs, backslashes and quotes cannot disappear through Git's display quoting;
invalid encodings and missing terminators refuse. Leaf spans count only their actual source bytes, including real
line-feed separators and preserving whether the end of the file has a final line feed (`PROGRAM.69`, review round 36).
The amendment's independent review closed on round 37 with no new defect, actionable control gap or remark. Its
synthetic reader and task-unit probes leave native sealing transactions, signals and real historical totals outside
that round's evidence; the frozen review history records the exact scope.

The decisions folder is kept the same way. A numbered section of a decision record that is settled, such as a
finding the director has ruled on, moves byte for byte into `docs/decision-history/`. Its heading stays in the
record, above one line linking the sealed file, so a citation of the section by its number still finds it.
`DECISION-HISTORY` checks what `TASK-HISTORY` checks, for sections: every sealed file against its row and the
commit that sealed it, the index append-only, one placeholder per sealed section, and every sealed section what its
record held just before. The first seal, on `2026-09-30`, took four settled items of the director's findings
register out of the decisions folder (`PROGRAM.41`).

Keep Verification and Commit Log rows contiguous. Put each work-unit record under `## Commit Log`; retain legacy filing references.
`COMMIT-LOG-ROWS` (`scripts/check_commit_log_rows.sh`) requires each work-unit commit, including the
pending one, to have a pipe-prefixed record. Missing rows refuse before commit; two were found by hand
on `2026-10-01` (`PROGRAM.46`). The script's measured backlog of earlier missing rows may only shrink.

Since `PROGRAM.90` (`2026-10-18`), live task records must also belong to the contiguous table under
one actual root Commit Log heading. Both working and indexed copies are checked. Inline mentions and
fenced examples cannot supply that heading; historical scope notes before the table remain allowed.
A preamble record, a record after Changelog or after a blank separator, an absent table delimiter,
duplicate headings and a bad indexed copy hidden by an unstaged correction all refuse. For example,
writing a new row below Changelog leaves its text present but outside the log; move it into the
existing table before committing. Sealed task history keeps its immutable presence-only contract.
`PROGRAM.91` (`2026-10-18`) also requires every historical and pending work-unit record in the
index as well as the working copy. An unstaged-only row or an untracked new task cannot supply a
commit's record. Stage that task document before committing. A staged removal concealed by a working
copy refuses for live and archived records; an indexed restored backlog record must leave the backlog.
Archived records keep their presence-only placement contract, while their content remains sealed.
Source evidence establishes each record's content.

```console
$ bash scripts/check_commit_log_rows.sh               # presence and live placement
$ bash scripts/check_commit_log_rows.sh --self-test   # positive and rejecting controls
```

```console
$ bash scripts/check_decision_history.sh                                   # the gate
$ bash scripts/check_decision_history.sh --seal <RECORD> <N>...            # seal settled sections, with the proof
$ bash scripts/check_decision_history.sh --self-test                       # its RED arms, on scratch repositories
```

A design's review history only grows while its review is open. Once the review closes, the whole file can move, byte for
byte, into `docs/review-history/`, leaving a short stub at its old path that links to it, so every citation of it still
resolves. `REVIEW-HISTORY` checks every archived file against its row and the history it came from, and holds the
review's row in `docs/reviews/INDEX.md` to `closed`, the history to what it was when it closed, and every later commit
at that path to the stub, so a round merged in afterwards cannot be lost. It reads the staged copies, so a file left out
of the commit is refused before it lands. Every closed history it accepts is archived (`PROGRAM.55`, `PROGRAM.70.1`); a
stub is no history, so the folder's file ceiling skips it, not its bytes (`PROGRAM.70.2`).
For example, the accepted M3 generated-adequacy design review is archived behind its original review path, while
its ongoing implementation remains live in `docs/tasks/M3.md`; closing a review does not close that implementation.

```console
$ bash scripts/check_review_history.sh                     # the gate
$ bash scripts/check_review_history.sh --seal <FILE>       # archive a closed review's history, with the proof
$ bash scripts/check_review_history.sh --self-test         # its RED arms, on scratch repositories
```

What a review round found is counted by hand in its design record's summary row, and the reviews index counts the
rounds; the history's own table lists each finding. For each review the reviews index lists under *Counted by
`REVIEW-COUNTS`* — every open review whose record keeps a table of its rounds, from its first round, which the check
itself demands — `REVIEW-COUNTS` holds the counts to the table, round by round, and refuses a listed review a round of
which it cannot judge; a review leaves the list when it closes, which the check demands too (`PROGRAM.69`).
The four-column index table runs from its header through its first blank line. Its rows may be indented and omit
edge pipes; an escaped pipe stays inside its cell. A malformed row, a missing separator or a missing or duplicate
header is refused. Every stated count is judged, including repeated statements of the same kind: a correct first
count cannot hide an incorrect second one. The same cell reader serves index, summary and findings tables: an
escaped pipe inside a summary cell cannot hide a later count. Each recognized summary header needs a separator on its
next line, including a header at end of input or before a blank line, prose or another header. The counted history/record pair names the record
whose counts are held. For an unlisted row, its first Design record link selects the record whose round table
decides whether it must be listed; later explanatory links do not change that obligation. In both file cells the
first unescaped bracketed label must be a supported inline link. Labels may contain balanced brackets; destinations
may be bare without spaces, parentheses or angle brackets, or enclosed in angle brackets with spaces/parentheses.
An optional title uses double quotes, single quotes or non-nesting parentheses, separated by whitespace. For example,
`[record](../decisions/a.md "title")`, `[record](<../decisions/a space.md>)` and
`[record](<../decisions/a(example).md>)` name their primary files. Paths are
literal: Markdown escapes, entities and percent escapes are not decoded. Unsupported first syntax, including
reference and shortcut links, refuses before any secondary link can substitute. A cell with no unescaped opening
bracket names no file. This grammar applies to both file cells in every row, including listed and closed rows;
an empty counted list does not switch it off. Files are read from the
index. A failed staged read permits a disk fallback only after a successful index listing proves the exact path
absent. Complete path bytes are compared directly, so inherited Git pathspec settings or filename wildcards cannot
change that comparison. An unreadable present entry or failed proof refuses; disk cannot silently replace staged evidence.

```console
$ bash scripts/check_review_counts.sh                      # the gate
$ bash scripts/check_review_counts.sh --self-test          # its RED arms, on a scratch repository
```

`PROGRAM` log repair (`2026-10-18`, `PROGRAM.87`): all 204 prior commit-log records and 154 verification records remain exact; the headed tables render 205 and 155 rows with the repair record.
The cleanup record that had followed Changelog returns between its native predecessor and successor.
Blank separators are removed while each record's text and relative history stay intact. Actual pinned
rendering and deliberately separated-row controls verify the complete tables in `docs/tasks/PROGRAM.md`.

## Where the landing page sends things

A size limit on `README.md` does not remove the need to write things down; it moves it somewhere else. So
`README_POLICY.md` also asks that every place the README points a reader to, and every place its checks tell an
author to move detail to, has an explicit policy. `README-ROUTES` works that list out from the README's links and
from what its two checks actually print. It follows each place on to wherever that place's own check sends overflow,
and checks each policy registered for it. **The book grows as needed:** it is the only user-facing surface,
so it has no size, chapter-count, file-count, line-count or line-width limits. Explanations, examples and
evidence must remain complete. Its chapters and annexes follow the subject matter and readers' needs.
Accuracy, glossary, index, anchors, coverage, sources, figures and the book build still have their checks.
The director's `2026-10-18` ruling is `docs/decisions/decision_book-unbounded.md`; `README-ROUTES` requires
`unbounded` for every book growth dimension, including nested routes. Other surfaces keep their controls.
Book files are excluded from every growth metric of a non-book ancestor route. For example, a bounded
`docs/` route can govern other documentation without counting the book against its file or byte budget.
Other material still follows that route's limits and archive-stub rules. Routing self-tests also refuse
failed fixture edits before trusting a verdict.
The changelog and the development notes are
bounded as rolling ledgers (above). A design's review history, which grows by a round at a time, overflows from the
decisions folder to `docs/reviews/`, which has ceilings of its own, and a closed review's history overflows on to
`docs/review-history/` (above). The task trees are bounded too, and their
finished parts overflow to `docs/task-history/`, as settled sections of a decision record do to
`docs/decision-history/` (above). A folder grown too large is split by subject into sub-folders, each with ceilings
of its own, while the parent still counts their non-book material, so an ordinary split adds no room.
Book files remain excluded from those ancestor measurements. No ceiling
may rise above what a decision fixes, which the policy lists and the check enforces. The decisions folder was split
this way on `2026-09-30`, and `DECISION-INDEX` keeps every record in a sub-folder listed in its index.

```console
$ bash scripts/check_readme_routes.sh              # the gate
$ bash scripts/check_readme_routes.sh --self-test  # its RED arms, on scratch repositories
```

## "Next" means what the task tree says

A page that names the next task is repeating what that task's tree says, and a repeat goes stale the
moment the tree moves without it. It has happened here several times: a page naming a task already
finished, a list of what comes next naming the current task as its own successor, and a changelog whose
header says "newest first" opening with an entry seven commits old. `STATED-ORDER` compares every such
repeat with its source on every commit. On its first run it found one more: a task whose own status said
it was in progress while its tree's list said it was blocked.

```console
$ bash scripts/check_stated_order.sh              # the gate
$ bash scripts/check_stated_order.sh --self-test  # its RED arms, on scratch repositories
```

## A number added to this book says what keeps it true

The error this project has made most often is a number written into a page that later stopped being
true: a count of descriptions over a directory that had grown, a list introduced as `Three rules` above
four. `FIGURE-REGISTER` stops a commit from adding such a number to a live page without saying what keeps
it true. The number must either be compared with a measurement by a named test that reads the page, carry
the date it was measured on the same line, or be marked as not a count (`docs/figures.md`). Numbers
already on these pages are counted and reported, and a commit may only lower their count in a page it
touches. Its first run refused a claim written while the check was being built, which named a test that
never reads the page.

```console
$ bash scripts/check_figure_register.sh              # the gate
$ bash scripts/check_figure_register.sh --self-test  # its RED arms, on scratch repositories
```

Fenced code is literal evidence, so the figure gate excludes its contents. A fence may use
backticks or tildes; the closing run uses the opening marker kind and is at least as long.
Opening and closing lines may be indented by up to three spaces. A shorter run or a different
marker kind inside the block remains literal. For example, this Markdown source keeps the
embedded backtick block inside its surrounding tilde block:

~~~~markdown
```text
seven tests
```
~~~~

A count written after the proper closing fence is prose again and needs its own classification.
Backtick info strings containing a backtick do not open a fence. Unclosed fences keep their
remaining contents literal. These rules follow the
[CommonMark fenced-code grammar](https://spec.commonmark.org/0.31.2/#fenced-code-blocks)
for the standalone forms covered here.

The pinned mdBook renderer currently differs from that specification for bare carriage-return
line endings and tabs after closing markers. The figure gate follows the renderer's current
boundary, in which a tab-suffixed closing run stays literal. The supported build command,
`bash scripts/build_book.sh`, checks source forms before rendering and refuses bare CR or a
tab after matching standalone closing markers. Its error names the repository-relative source
path and line and explains the correction: use LF or CRLF line endings, and spaces after closing
markers. `scripts/check_book_sources.py` performs that read-only check.

This conservative source check preserves shorter or mismatched marker lines inside a surrounding
literal fence. To show such a marker as data, use an outer fence of a different kind or sufficient
length; control-byte evidence can also use explicit JSON escapes, as in the
[compatibility finding](annex-trust-proofs.md#renderer-compatibility-finding).
The [publication guard record](annex-trust-proofs.md#publication-guard-for-renderer-control-bytes)
holds actual refusal/recovery results and the upstream-adoption task. These standalone checks
do not claim complete Markdown container parsing. The book is unbounded in size, chapter count
and detail.

## Every diagnostic shown in this book is a real run

When this book shows `archogen check` refusing a description, a test re-runs that command and requires
the book to show exactly what it prints: every label, the final summary line and the exit code, with
nothing shortened or re-wrapped (`crates/archogen-cli/tests/book_transcripts.rs`). Before that test,
five of the twelve checkable examples differed from a real run. Two had silently dropped the "first
declared here" label that the text around them said was there. All five are re-rendered from runs.
Eleven older examples use a file that is not in the repository, so no one can re-run them. They are
listed in the test, and the list may shrink but never grow. Each is listed by what it shows, its first
error and its first location, and not by the line it sits on. The first version used line numbers, and a
paragraph added above one of them made the test fail for an example that had not changed (leaf
`PROGRAM.33`).


### Daily artifact cleanup on 2026-10-20

Measured `2026-10-20T00:58:20+00:00`, `PROGRAM.94`. Only idle main and owned native-sweep incremental caches were removed. Physical metadata and independent native find/stat inventories agreed. Immediate identity rechecks and a successful native process census preceded deletion. Raw evidence, dependency stores, final products and the empty wasm cache were retained.

The deletion removed `2434` regular cache files totaling `828209661` logical bytes, including `60` cached `.bin` files. The surveyed locations held no `.log` files. This measures logical size; no allocated-space recovery is claimed.

| Repository-relative location | Regular files | Logical bytes | `.bin` / `.log` | Action |
| --- | --- | --- | --- | --- |
| `target/debug/incremental` | 938 | 411763556 | 24 / 0 | Remove idle cache |
| `target/wasm32-unknown-unknown/debug/incremental` | 0 | 0 | 0 / 0 | Retain |
| `target/debug/deps` | 800 | 673065094 | 0 / 0 | Retain |
| `target/release` | 117 | 17080485 | 0 / 0 | Retain |
| `target/mutation-sweep/target/debug/incremental` | 1496 | 416446105 | 36 / 0 | Remove idle cache |

Immediate Python existence checks and native missing-path statuses confirm both admitted directories gone. The other surveyed locations remain identity/size equal to their inventories. Every protected evidence file retains its captured digest. Maintained Python/system checks restore all `1145` captured regular inputs; the submodule remains opaque metadata.

The focused tier must pass on the final staged cleanup record after cache removal. Existing products may be reused, so this verifies workflow regeneration after incremental removal. The wasm cache was retained and no new wasm-build result is claimed.


<details>
<summary>Exact admitted deletion</summary>

```json
{
  "qualification": "Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git boundary rechecked immediately before removal; only fixed admitted regenerable incremental directories.",
  "removed": [
    {
      "path": "target/debug/incremental",
      "status": "gone",
      "files": 938,
      "logical_bytes": 411763556
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "status": "gone",
      "files": 1496,
      "logical_bytes": 416446105
    }
  ]
}
```

</details>

<details>
<summary>Immediate residue and retention</summary>

```json
{
  "status": "verified",
  "timestamp_utc": "2026-10-20T00:58:20+00:00",
  "files": 2434,
  "logical_bytes": 828209661,
  "residue": [
    {
      "path": "target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/debug/incremental: stat: No such file or directory\n"
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/mutation-sweep/target/debug/incremental: stat: No such file or directory\n"
    }
  ],
  "retained_evidence_files": 176,
  "untouched_surveyed_locations": 3,
  "regenerable_only": true,
  "qualification": "Logical bytes, not allocated-space recovery; metadata/native inventory, exact identity recheck and successful no-job census precede removal. Focused regeneration required separately."
}
```

</details>

<details>
<summary>Protected evidence digest census</summary>

```json
{
  "target/cache-current-checkpoint/apply-publication.py": "c60fc0b2cb899f0e6419b90a97b3005491df20009dc41477e6645e1611aedf6d",
  "target/cache-current-checkpoint/assessment-book-complete.md": "2e6dd42dd0372228be9b810b06debfd73716193765ae522b8bf95242cd15034c",
  "target/cache-current-checkpoint/assessment-book-fragment.md": "2a2c33c63db30b3df7a1859736e6d4915a1d5ecbd67fbf41df31b4d541c25fd6",
  "target/cache-current-checkpoint/assessment-history-before.json": "8cc4a78655c5d02de60332d286ccebc3c6cfc9a4a082162fbc64e2ca4b33eae7",
  "target/cache-current-checkpoint/audit-native.py": "578cbdc84c5303a46ba327eb7549c8af1221b4fa894df04f511a533f687acac4",
  "target/cache-current-checkpoint/bindings.json": "b923dba94e54dc9a97b52918982a1c3c64cb6424db772490518ef88490d862d7",
  "target/cache-current-checkpoint/book-before-publication.md": "84eb5d38e1a47e57f6f694338b74084e6a967e181ce53387a426ac2c96acdce7",
  "target/cache-current-checkpoint/book.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-current-checkpoint/book.log": "4d1971183cdf7dfc0319510ffbc13ed6696c07b84103532678e9a529b98635f7",
  "target/cache-current-checkpoint/cache-diagnostic-audit.json": "cd10168b50957616f10cac1c59687f2e0e5c21545432cc13a77776e09d5ec580",
  "target/cache-current-checkpoint/complete-records.py": "e8c09d6e48aa124511656eff772b4cdf659728dc8236d336b8c031c67204ca3a",
  "target/cache-current-checkpoint/copied-parent-list.log": "decc560e17134ce87d345b03cd2c823f33cf53531ce54cb769d4c8ed99f214d8",
  "target/cache-current-checkpoint/copy-restoration.json": "b00d5c62f353516f665e83e4a66de8232588fdfb01bad2aa152a5d07755d668a",
  "target/cache-current-checkpoint/diagnostic-approval-rejection.json": "0926631d6f543f65f6b11d736c8ad62d3b1b5716f6736d0af2c24ccaced55380",
  "target/cache-current-checkpoint/doctrines.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-current-checkpoint/doctrines.log": "939d542fd71db6aa36406a7055d2000b2b2c33037c50fb6d0042a25da374737c",
  "target/cache-current-checkpoint/execution.json": "0868833d927dc87eaac97c5ea5fd40d21163040775845e40152a2ff866ca1fcb",
  "target/cache-current-checkpoint/final-audit.json": "8c766015a5fb82fc13f481ac502041555c5f0ab4e3f58cea1e56fa9753756e09",
  "target/cache-current-checkpoint/final-audit.py": "3d9082f41e5117924945fb79fbbed311cb16beb9aa77aff37452514415280996",
  "target/cache-current-checkpoint/first-book.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-current-checkpoint/first-book.log": "4d1971183cdf7dfc0319510ffbc13ed6696c07b84103532678e9a529b98635f7",
  "target/cache-current-checkpoint/first-doctrines.exit": "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865",
  "target/cache-current-checkpoint/first-doctrines.log": "a57411771602f5d453bb53859f0a2b4afe023666f4b429cb90fc8946069c7e0c",
  "target/cache-current-checkpoint/history-before.json": "d9e2f01197a9cdc30d6e7366dafd43d33c21769d3d306ab0109e875afaa4466a",
  "target/cache-current-checkpoint/input-reconstruction.json": "230fa8e1933751ebe9958e288bdf134e72df118ddb4f9ea1cf777401754fe111",
  "target/cache-current-checkpoint/listing.log": "c4108906fd550906b7f7c36e6801a71d854827bfa8c198c341f664806c3bcb3c",
  "target/cache-current-checkpoint/measurement.json": "85cc6df0dc154e212ee2e32db0381fd3e6c52f8be6617d6e1eb36de30d664d5a",
  "target/cache-current-checkpoint/mutation-volume.sb": "2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd",
  "target/cache-current-checkpoint/native-audit.json": "c04d444f638ad4a862c9fae4495e5638799b4fbe874c8e3ea30f361c0f0ce3b8",
  "target/cache-current-checkpoint/native-inputs.json": "e4ce6ccee4f561e4aa8bd6c5ce11968f6d38e6114884910926974d9fc681353e",
  "target/cache-current-checkpoint/native.exit": "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865",
  "target/cache-current-checkpoint/native.log": "ae457bb5904df32cc9d1f9c56fb35e80f40ed95db1c183d9f3cc2875879e2f06",
  "target/cache-current-checkpoint/old-source.rs.txt": "90c1078daba457e76f6564dcaf19557013b30e2275921a2fd25ca000c16d10ef",
  "target/cache-current-checkpoint/original-restoration.json": "57e81c39b1940949a48071dec02ba4f380618420bf67acde8c57ce9a09d7c71b",
  "target/cache-current-checkpoint/parent-list.log": "decc560e17134ce87d345b03cd2c823f33cf53531ce54cb769d4c8ed99f214d8",
  "target/cache-current-checkpoint/parent-names.json": "b736917e6e5b7f61defdef98baf2af133dbd6cd691f49a8bcd3f690a03d0ea04",
  "target/cache-current-checkpoint/plan-audit.json": "a61813d841bafa4189bd6ab0dc1f63e5d1cabc67bb858bfd99bd7d4cad35981f",
  "target/cache-current-checkpoint/plan-book-fragment.md": "f68191eb8971b2adc12b5f01574c514e66cb2f6f126ac7e97d2c09a2694a10e2",
  "target/cache-current-checkpoint/plan-verification-pre-native.log": "c60fa2f88b664601d9f1b96e496efc4e5ce15701144c5b6c4ddef7980e1037a4",
  "target/cache-current-checkpoint/plan.json": "fe8b8f2a06641fa4a4d3b6fa59c718f6ec85c40e8b616b4274b8e6f63f47ff04",
  "target/cache-current-checkpoint/premise-parent-record-before.md": "7f78b35bcb4feb5f32a6ef32865e5dd26bfaa731e60c00422ee3b1a6b8d462c5",
  "target/cache-current-checkpoint/premise-parent-record-compact.md": "f185ccdfb31f4b7261fda7dc3dcbd524caac45847869cb7c7efeefe6e4deece3",
  "target/cache-current-checkpoint/prepare-book.py": "e6ad78012a3c5bb2c12ee83eadad59a649bad2c97339c4174750ea7bfef5d6c5",
  "target/cache-current-checkpoint/prepare-component.py": "24c267a5fda7778a17654bcb2b36792c1f0167b0652b8e0ed8aad4307066ea51",
  "target/cache-current-checkpoint/prepare-plan.py": "53b0e11cc3d8427769a6f9c724f545c961cc6a6c99414ca661534f6f7eb18268",
  "target/cache-current-checkpoint/probe-cache-function-full.py": "7a2a39c612c33b7061b0ed63ce68cd9fa1bfb92955a7c3dcb31281fefa2af0a3",
  "target/cache-current-checkpoint/probe-cache-function-qualified.py": "8e0de37f7f22cb7b4e3f5cec9d06c4d6924364050fbe29e2a4983123466a2bd4",
  "target/cache-current-checkpoint/probe-cache-function.py": "9eceb05f4a37d6f464983e94791457e481960cd18625102947da98c34cb1896a",
  "target/cache-current-checkpoint/producer-bindings.json": "4c43d07e791bd9c7adb5904eb96733367a37bbb3d45dac3dfc083a8655cc4298",
  "target/cache-current-checkpoint/producer-identity.json": "b25447deebeb0b69164dcffe73d6b2b5797a03129a33a3e054af6116a7cd2d36",
  "target/cache-current-checkpoint/progress.py": "309a6c6399678fd3c501c18764297e42d6d0fc7dc00311389e33fbc114825e83",
  "target/cache-current-checkpoint/publication-audit-calibration.json": "43fa7ed9c5024f8726969bae5cae0976d94b004449351eca24b6ae7d146ffa61",
  "target/cache-current-checkpoint/publication-routing.json": "13c993b6e26279aec7653e53da5a99d0fd05966db78d2037c4364210f104f6ce",
  "target/cache-current-checkpoint/qualify-cache-graph.py": "c9e2ceb25e5b6228a90a0e174f99c494b7fcb638994d003ad61c0782e7c82287",
  "target/cache-current-checkpoint/qualify-copied.py": "0b6287b787964f26a80e896cbdf402e4dbd112b3b985bfada495a1246a351e89",
  "target/cache-current-checkpoint/reconstruct-inputs.py": "7e81b620077926bbe3ff3499768e43dde0ff5bc4b223e0e48d61959bbf1411be",
  "target/cache-current-checkpoint/repair-task-full.md": "57004b3566dcc7a2bd2df4eb1d711c132d5d8594390a3687f5787ca5e256019c",
  "target/cache-current-checkpoint/replay-task-full.md": "d7b696b2ae4574e084658cba141a65ecb0cb3daed5985bd961453ba9d56ea8ab",
  "target/cache-current-checkpoint/run-native.py": "7c74fa6d7e818380a18c45bc26f2b4ae01399dc6b8d1c8b05c56e9e8bf6cd559",
  "target/cache-current-checkpoint/source-before.rs.txt": "c02f30e133a9ef8b54f237dd5aa2cb0c8b55041fbdd304290d5109d65277aa49",
  "target/cache-current-checkpoint/task-after-parent-routing.md": "e884dd50d6696abb198b9c70dcc6ddea25529bfcc3c98b75e0ebf826a1087596",
  "target/cache-current-checkpoint/task-after-routing.md": "db09effe8382afb9c7cc1c5ebc3658795c13262f66b222921e5c76ab4d42d800",
  "target/cache-current-checkpoint/task-before-parent-routing.md": "6f394854cffad5b74371dd2704d3804d57d050608138317a5996aff16ddd949c",
  "target/cache-current-checkpoint/task-before-routing.md": "acc6544ec9afb83536e9b3d90ab20a469d64db5906969d20143c7267d05aaf24",
  "target/cache-current-checkpoint/task-routing.json": "5666177cc765b7f4897fa3a535c9adcd1994ccdf704d922622c0f845df2bf969",
  "target/cache-current-checkpoint/verify-cache-diagnostic.py": "3788ac5dfefa3f34bcd634fb25e77fc33f41dc481e30a593d687fd751e041596",
  "target/cache-current-checkpoint/verify-components.py": "057a3662aa57bcb1bac1f24041c6584921565a20c1c1d9cf999cac4115f445c7",
  "target/cache-current-checkpoint/verify-plan.py": "d6461c20f2b7e069e5b71a901d3e355462b7a1469b6179effcbd022015702723",
  "target/cache-middle-regression/add-parent.py": "3db4117798a05c27b50c2c7c6baed3db8284a9cb666c16bdcb35b81605e3c803",
  "target/cache-middle-regression/audit.json": "95ca17332ec5eb15ef26374f9add5a3c1ec9e50fc8fd7c330d0e1cd8b2734a4d",
  "target/cache-middle-regression/audit.py": "a072ad8ef81f3d41faa58a0f7a27555b8a5f67d7d159383e2691d72bac12ca93",
  "target/cache-middle-regression/bindings.json": "27bf63ebccdd8a9c7d735fe8e94d46f1958ccbea28f36d3acdb664a4ae96cd05",
  "target/cache-middle-regression/book-before.md": "f062cef582c1f51e4859c7b41de078acae4498392509ae84101d7bcdf3beeae7",
  "target/cache-middle-regression/book-fragment.md": "944a8f7f33ea0f225ecc69ab8a8ec6bf122eea8f72a68ccf022e25148e42dcaf",
  "target/cache-middle-regression/book.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-middle-regression/book.log": "4d1971183cdf7dfc0319510ffbc13ed6696c07b84103532678e9a529b98635f7",
  "target/cache-middle-regression/compiled-fixture": "9bcdc2508b25f319efdacc33bda1799d33aeb59299c559cc3337953fd71fb63c",
  "target/cache-middle-regression/compiled-fixture.json": "5c1fe2f4d1e7db195799bdf9b1e0a4cc7e59cbd115ebaf777cfa10eae8c234c8",
  "target/cache-middle-regression/compiled-fixture.rs": "b9ab0f2763cfe8eefa8b965114df2c649a1149afb47da20e87808fa3756fc721",
  "target/cache-middle-regression/copied-parent-list.log": "86f2f6e72535d15365b37d6f32a02da02fc904f236066086d134e0a526a71cde",
  "target/cache-middle-regression/copy-adapter-finding.json": "21930fe56ea916a879f04818513025b0bbcb6f2e9c28622d64c09a817e88d741",
  "target/cache-middle-regression/copy-adapter-parity.json": "c1cfa372f200f336ab167926662d0cf1e206a60d1c2e5148c570093e052bbd24",
  "target/cache-middle-regression/copy-restoration.json": "ee88385fe92661635b39ff757669ae02568637e6b0ca08f3679bd972b9edc78f",
  "target/cache-middle-regression/doctrines.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-middle-regression/doctrines.log": "939d542fd71db6aa36406a7055d2000b2b2c33037c50fb6d0042a25da374737c",
  "target/cache-middle-regression/execution.json": "c25764e095fc4c8889cfaf4a51551547b9d94f64552c395b752f702eca95e4ab",
  "target/cache-middle-regression/final-audit.json": "98db96674600d719d8c3ffa8fdf4d405e9b8f9aa90afd46cfbefb7cc2badf165",
  "target/cache-middle-regression/final-audit.py": "f8a72f3c46fbc6e60a8ec0255859261f19e4cedef7541fb0fa6190ada5281c20",
  "target/cache-middle-regression/final-plan.json": "7d957bb57dc36d98cab7d60814b092c555f17ce1ed976d8312cfb5b05f681367",
  "target/cache-middle-regression/final-task-compact.md": "03cfc06bec0d1697abf81b7b76b554a4a12436ebd82f94856c529679bad7d871",
  "target/cache-middle-regression/final-task-record.md": "2a40b448f84eddb5fac7e7c6abba48046002f4c352d9ac4e66bae5a6891d9a72",
  "target/cache-middle-regression/first-compiled-fixture.rs": "5852bf229cbb63dabaf4a122a9867ec9d285354065e180bd025f4f0a61d914f2",
  "target/cache-middle-regression/first-fixture-compile.stderr": "5ac73ca72862bae2ad2a61d281026b81e8aed75c68c5b559e7a7fcc2891d2ec8",
  "target/cache-middle-regression/fixture-audit.json": "87eef7ff90db0922c3b1b5a73f789c131a4859a62c67b4f45c0241d9e6d44111",
  "target/cache-middle-regression/fixture-compile.stderr": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "target/cache-middle-regression/focused-execution.json": "9273274104ab8f6153ab9b17abea7805a34d52330be24b0aae8677bdffb09d90",
  "target/cache-middle-regression/focused-inputs.json": "baa9786e6f5acf26464933bd58714543dad57f4b55efeceb7d41bb1dd502a427",
  "target/cache-middle-regression/focused-restoration.json": "5ee94ff37f7bfe0bed4a993363c1545cb12bf3c94a9238437b388dc5ffdf0d1e",
  "target/cache-middle-regression/focused.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-middle-regression/focused.log": "46f3d4c16f963779351b0741a07b50d5c00674831dbe597a6977a8bff92f1402",
  "target/cache-middle-regression/history-before.json": "12992383d9ce2cb52c28f4524372e3343074e188d34ccbced027a899d91f1e8a",
  "target/cache-middle-regression/input-reconstruction-delta.json": "29f084acf45657dc6ba8021f95f378d8924eebce77bf5149bf945496cfd2fdd9",
  "target/cache-middle-regression/input-reconstruction.json": "2e976d20f9106a95fa564a8a821632b9f640445825943ffd4e2cd79401380eb9",
  "target/cache-middle-regression/legacy-copy-restoration.json": "7eb2f030a8c8b9cee11bc7713d0fdc060f91df72d5581bd013f9101e9f222d60",
  "target/cache-middle-regression/listing.log": "c4108906fd550906b7f7c36e6801a71d854827bfa8c198c341f664806c3bcb3c",
  "target/cache-middle-regression/measurement.json": "7080e9d59f399b630f1c55899e505a35bd069df9d591e413928b592e237be33c",
  "target/cache-middle-regression/mutation-volume.sb": "2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd",
  "target/cache-middle-regression/native-inputs.json": "b1fde505a14999b360fe72d8137c9c057a693d04d9e4be63af5d2f0944c47419",
  "target/cache-middle-regression/native.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-middle-regression/native.log": "d549c1092ab985192c5891a44773387c67d418265ac5ee78c9fd061a7b3113e2",
  "target/cache-middle-regression/original-restoration.json": "51a3c493b1d926001f3448797819ddcc93627d140212a2b43a99ec3a66519089",
  "target/cache-middle-regression/parent-first-compile.log": "b1d9e60ca6467054ebe8edfa40b501a1ee128b8116586582a93cb49bb240ed5d",
  "target/cache-middle-regression/parent-list.log": "86f2f6e72535d15365b37d6f32a02da02fc904f236066086d134e0a526a71cde",
  "target/cache-middle-regression/parent-names.json": "546a0333ef79dbb511be1372a29b3cc0453b57796560aed487060d1da01e49b1",
  "target/cache-middle-regression/parent-unformatted.rs.txt": "4e3c6eda4d437e2e68118b97b619d044b7d880fda56d841654b45028cb330ee1",
  "target/cache-middle-regression/parent.exit": "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa",
  "target/cache-middle-regression/parent.log": "25efff3a64ae66cc265511fa93dd07295207827296de60e3391dcd86688461d0",
  "target/cache-middle-regression/parent.rs.txt": "3e27e1946617c44e7c877aa0a82cd72db73c0e91018a0785ddcd1219ff688ea9",
  "target/cache-middle-regression/plan.json": "8f6d1f2166214f9fa1f442b8df49cd90a531948c581dca7e99a480c7c4ccca12",
  "target/cache-middle-regression/preparation-attempt-inputs.json": "b1fde505a14999b360fe72d8137c9c057a693d04d9e4be63af5d2f0944c47419",
  "target/cache-middle-regression/preparation-refusal.json": "b8d4871f98d98d9d8758f501bb13df415dec259605ffc37356d4f7955176ede8",
  "target/cache-middle-regression/preparation.json": "2327ce33b722ac24aef62b13aa7ed027b16fa815f21fdecb99fb572a82e9d4e6",
  "target/cache-middle-regression/prepare.py": "0c5168cf08a65871da049ece417676030dbbfec14dd91ae0cde2dd811b99658e",
  "target/cache-middle-regression/producer-bindings.json": "4c43d07e791bd9c7adb5904eb96733367a37bbb3d45dac3dfc083a8655cc4298",
  "target/cache-middle-regression/progress.py": "49e51a1a7e9a2d84ee99a4eb656acc4bfc7010855d4d8e27491cac7746ea3ae5",
  "target/cache-middle-regression/publish.py": "5ae7b771779a663d54d8099c603819a91dbc37949518fe21698d53db0e23189a",
  "target/cache-middle-regression/reconstruct-inputs.py": "19fa920492589b16f1247b006762c44cd8e344d634d7801aa6dc30304bac05a6",
  "target/cache-middle-regression/replay-inputs.py": "cca55dd9dc8a155c43dd758635ab0ef5663bca6262c3f2038d1f53924180a208",
  "target/cache-middle-regression/replay-task-routing.json": "7d1cbc243d3802b1c2d6b73a66f3738fca4da7495b9b90176d583a91a931cf74",
  "target/cache-middle-regression/run-focused.py": "441c79d867167cd664681bfbd7860fc34e439668b54977e8dca04d0e516fe38c",
  "target/cache-middle-regression/run-native.py": "9fac25447f39b926e2fea1350119f893ff883a4cab41b652995582e5e61a507e",
  "target/cache-middle-regression/shared-copy-input-adapter.py.txt": "1354aa97640a202b7f79f1d05a3953dd5cfc8d338f3c10c30f31d7e5880a5225",
  "target/cache-middle-regression/source-before.rs.txt": "c02f30e133a9ef8b54f237dd5aa2cb0c8b55041fbdd304290d5109d65277aa49",
  "target/cache-middle-regression/source-native.rs.txt": "9616b59ff2bc394e6471570bfaf857971671fda4a12c4fb745e557d2d21a1751",
  "target/cache-middle-regression/task-after-replay-routing.md": "b9197b328583109b46be57dd5100120c1ef924baaa585d5c7c4291b91d6ab1cd",
  "target/cache-middle-regression/task-before-replay-routing.md": "24764a117131c4b998dad8eff612f5ff2d4b6714b5ee150da0f8ff63761b929d",
  "target/cache-middle-regression/verify-fixture.py": "399212854d22b93c0671260cb1f52da1227af4ed50537f87657d1c962bbc585f",
  "target/caller-suffix-assessment/compilation_files.rs.txt": "5ab7f5e394d69a042c13229d59689c3b037eef966bfb0bad8098a713bc82d910",
  "target/caller-suffix-literal/cargo-lto.rs.txt": "97652b448f525af54f1baee199722cd1534768927a2d4deeae924504cade64d6",
  "target/cargo-absolute-prefix-proofs/package.rs.txt": "c69d85227b154d36e828acdbc3705ef921a4919775dd2a9e1c0eed912287cff7",
  "target/cargo-absolute-prefix-proofs/workspace.rs.txt": "fa321dfbd2857f42b4a1dd37054dd749a860f664baa89fc55e76978da32abbd2",
  "target/compile-command-assessment/pinned/src/cargo/core/compiler/custom_build.rs": "2a2feaff913b3c63152ba5d7dc4328e73c01aeb239f3c9ca570f8fbf39db115e",
  "target/compile-command-assessment/pinned/src/cargo/core/compiler/job_queue/job_state.rs": "37e47e0d5a1f598318f67e1187ed76fd5f91378cd79786bce0037e437f8ea025",
  "target/compile-command-assessment/pinned/src/cargo/core/compiler/job_queue/mod.rs": "0cccd1337e007e233670579b1fa2aa236e776bbe72372f97209f9fcfe3e61260",
  "target/compile-command-assessment/pinned/src/cargo/ops/cargo_doc.rs": "1a18e4baf03c50b51e78382879b9cf8a3ab7de99bb0bf23af5b689650e00f1db",
  "target/compile-command-assessment/pinned/src/cargo/ops/cargo_run.rs": "50b5800f9fa660be672e7fe3092ff53a649afc232c8e99ed5999c1858e591c51",
  "target/compile-command-assessment/pinned/src/cargo/ops/cargo_test.rs": "38e2c4236f85caf3de3a63e5d7604c472af914dd40b869555072a21e91efbf72",
  "target/compile-command-assessment/pinned/src/cargo/sources/git/utils.rs": "7c23a3bd16810eb65001a86c93db14f2302b61b63efec9534ee270cbee23ca96",
  "target/m369-color-equivalence/environment-variables.md": "6f665b89e5a164da0526f22602105ba9dfb16928d76e028bf2fc4a30e0d37323",
  "target/m369-flags-domain-completion/build-cli.rs.txt": "10435788c80cc418639d2a649acc58efd1d3fb25818fefb9fc78b9d0a9c0bce2",
  "target/m369-flags-domain-completion/compile-ops.rs.txt": "e66a00c5fdd1ff9c27f3f3a3d07326226b26355867535d08659ed4ccb379f04f",
  "target/m369-flags-domain-completion/context.rs.txt": "30310d878bb05ffed2db0c55699619bb1452834d4db5b2166ec2f2aeceebda62",
  "target/m369-flags-domain-completion/git-source.rs.txt": "aa8e1bd019c845a62b6754f78d7e9f424f56187f31cdf1d6e26db29a6a8a73ce",
  "target/m369-flags-domain-completion/registry-remote.rs.txt": "01b9bb8e68446ad0d4feceeff02b084913d2afd6e067dc34ca56ce9f89a51b65",
  "target/m369-flags-domain-completion/registry-source.rs.txt": "20715404e59e0fe47f68de6c016c0ebc6e84aebedc740e046f12657147fd148f",
  "target/m369-flags-domain-completion/source-config.rs.txt": "f3cbe62254b8b4fb512c64abf8d435a8ed0e7e65397c2e9af4be1133884677c8",
  "target/m369-flags-domain-completion/standard-lib.rs.txt": "17cbb514863daad6d6e872750401b1f012a7ca416d401989637a480f5960ee94",
  "target/m369-flags-domain-completion/test-cli.rs.txt": "fe1944aa2e06daadc06fbeb078484b1303e13bb2f31cf2be367a394761e1491d",
  "target/m369-flags-domain-completion/unit-dependencies.rs.txt": "17563082c4c7f47084272cf838400abc3c904de582da570bc473a1bb94d82896",
  "target/m369-flags-domain/cargo-build-context.rs.txt": "26ad05905afb9d4946d1087d3291c3bd398468082f77420ed79f2e0610585898",
  "target/m369-flags-domain/cargo-compilation.rs.txt": "143263b660cd89e6ae2a0ecca33333e23d4254d134f6336a2ae7f36621435753",
  "target/m369-flags-domain/cargo-compiler.rs.txt": "0676b172894d2d0e8536bb5197019f2c147f9aa3e4587e13909e73512fa9d023",
  "target/m369-flags-domain/cargo-features.rs.txt": "e07a9570fde7d272c5205579855b9942011275bb1730f56db0b686d6cfe168d6",
  "target/m369-flags-domain/cargo-manifest.rs.txt": "a4ab9a2e3f7177322b659e21dace551f784b709e1041757412eb107e690fc5f9",
  "target/m369-flags-domain/cargo-profiles.rs.txt": "df4b7e2d13f25efa7bb892a7efd31c82e549f0c80a138af361f7ab4e4252cfb7",
  "target/m369-flags-domain/cargo-target-info.rs.txt": "6ea904f8285cda1f8014b4eb2ed92abc61bdf5abc74101da3b802f2f95b6431c",
  "target/m369-flags-domain/cargo-toml.rs.txt": "0b7e604cabcedb7d9e62942f5004625930220b2b1eef85cc1c71b0c4dfb7bb30",
  "target/m369-flags-domain/cargo-unit.rs.txt": "edcc3fb5d266cd1de90b2e630cafea34c0bbda6722a860d3614a0f60b0862d8b",
  "target/m369-flags-domain/cargo-util.rs.txt": "0d11f8f8643282099151a2a28a8505adf7b5bbbcd1760d59e295adcc64fae9cf",
  "target/m369-flags-domain/cargo-workspace-util.rs.txt": "a8ab44391e698a50d1b8f57a33ff6205866560e7f49520c1334aa8db2f419f83",
  "target/m369-reset-revalidation/build-config.rs.txt": "e5f4eeac36687832cd19bd958028bd3811b6757a4d39158b2a1e8bca03586a6f",
  "target/m369-reset-revalidation/cargo-cli.rs.txt": "9648dbe8147e7475ca14b453aba92f0bca9a7741abb9a66d65ee0becc9cae560",
  "target/m369-reset-revalidation/http-registry.rs.txt": "91b958e7137b894ce8594b961fc76e96f7ead76beaf880dc30f86fe65356faad",
  "target/m369-reset-revalidation/registry-download.rs.txt": "5c51eb90daf504013b55af13ea4aa04532ee683ce4c87522a78b4e91eb4609c5",
  "target/m369-reset-revalidation/registry-index.rs.txt": "b5437ca4bdc24a731ff869c1e827412705f1578d4c537639a3a1ba011a45fca7",
  "target/m369-reset-revalidation/source-id.rs.txt": "fc2c38a770e91f66c45503a3824213c2b0881ffa0a83fef41bb474a1e412d294"
}
```

</details>

<details>
<summary>Supplemental adapter: census.py</summary>

```python
from pathlib import Path
import os,stat,subprocess,json
root=Path.cwd();out=root/'target/artifact-cleanup/2026-10-20';device=root.stat().st_dev
locations=['target/debug/incremental','target/wasm32-unknown-unknown/debug/incremental','target/debug/deps','target/release','target/mutation-sweep/target/debug/incremental']
def metadata(path):
 info=path.lstat();assert info.st_dev==device and not stat.S_ISLNK(info.st_mode),str(path)
 assert stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode),str(path)
 return {'path':str(path.relative_to(root)),'device':info.st_dev,'inode':info.st_ino,'mode':stat.S_IMODE(info.st_mode),'bytes':info.st_size,'kind':'file' if stat.S_ISREG(info.st_mode) else 'directory'}
def inventory(name):
 base=root/name
 for n in range(1,len(Path(name).parts)+1):
  ancestor=root.joinpath(*Path(name).parts[:n])
  if not os.path.lexists(ancestor):return {'status':'absent','count':0,'bytes':0,'bin_count':0,'log_count':0,'files':[],'directories':[]}
  info=metadata(ancestor);assert info['kind']=='directory';assert not os.path.lexists(ancestor/'.git'),str(ancestor)
 files=[];directories=[]
 for directory,dirs,names in os.walk(base,followlinks=False):
  assert '.git' not in dirs+names,directory
  directories.append(metadata(Path(directory)))
  for entry in dirs+names:
   p=Path(directory)/entry;info=metadata(p)
   if info['kind']=='file':files.append(info)
 sizes=list(map(int,subprocess.check_output(['find',name,'-type','f','-exec','/usr/bin/stat','-f','%z','{}','+'],text=True).splitlines()))
 assert len(sizes)==len(files) and sum(sizes)==sum(f['bytes'] for f in files),name
 return {'status':'present','count':len(files),'bytes':sum(sizes),'bin_count':sum(f['path'].endswith('.bin') for f in files),'log_count':sum(f['path'].endswith('.log') for f in files),'independent_system_find_stat_agrees':True,'files':sorted(files,key=lambda r:r['path']),'directories':sorted(directories,key=lambda r:r['path'])}
if __name__=='__main__':
 records={name:inventory(name) for name in locations}
 (out/'before.json').write_text(json.dumps({'qualification':'Ignored supplementary metadata adapter; native system find/stat independently agrees. Only main repository named locations traversed; no links, cross-volume entries or nested Git ancestors/children. Logical bytes, not allocated disk space.','locations':records},indent=2)+'\n')
 for name,record in records.items():print(name,{k:v for k,v in record.items() if k not in ('files','directories')})
```

</details>

<details>
<summary>Supplemental adapter: delete.py</summary>

```python
from pathlib import Path
import importlib.util,json,shutil,os
out=Path('target/artifact-cleanup/2026-10-20');spec=importlib.util.spec_from_file_location('census',out/'census.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
before=json.loads((out/'before.json').read_text());names=['target/debug/incremental','target/mutation-sweep/target/debug/incremental'];removed=[]
for name in names:assert m.inventory(name)==before['locations'][name],name
for name in names:
 expected=before['locations'][name]
 if expected['status']=='present':shutil.rmtree(name)
 assert not os.path.lexists(name),name
 removed.append({'path':name,'status':'gone','files':expected['count'],'logical_bytes':expected['bytes']})
(out/'removed.json').write_text(json.dumps({'qualification':'Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git boundary rechecked immediately before removal; only fixed admitted regenerable incremental directories.','removed':removed},indent=2)+'\n');print(json.dumps(removed))
```

</details>

Receipts and complete pre-deletion metadata are under `target/artifact-cleanup/2026-10-20/`. These adapters are supplemental; maintained input checks, native system inventories and the focused tier supply primary verification.

<details>
<summary>Cleanup audit adapter</summary>

```python
from pathlib import Path
import copy,hashlib,json,subprocess
out=Path('target/artifact-cleanup/2026-10-20');base='e56e0af218a9df4e84606a0674540324e4870583'
def sha(data):return hashlib.sha256(data).hexdigest()
def prior(name):return subprocess.check_output(['git','show',base+':'+name])
before=json.loads((out/'before.json').read_text())['locations']
removed=json.loads((out/'removed.json').read_text())['removed']
receipt=json.loads((out/'residue.json').read_text())
assert [r['path'] for r in removed]==['target/debug/incremental','target/mutation-sweep/target/debug/incremental']
assert sum(r['files'] for r in removed)==receipt['files']==2434
assert sum(r['logical_bytes'] for r in removed)==receipt['logical_bytes']==828209661
assert sum(before[r['path']]['bin_count'] for r in removed)==60
assert all(before[name]['independent_system_find_stat_agrees'] for name in before)
assert all(before[name]['log_count']==0 for name in before)
assert all(r['python_absent'] and r['native_status']!=0 and 'No such file' in r['native_stderr'] for r in receipt['residue'])
assert 'handoff: OK' in (out/'process-before.log').read_text()
evidence=json.loads((out/'retained-evidence.json').read_text());assert len(evidence)==176
for name,digest in evidence.items():assert sha(Path(name).read_bytes())==digest,name
restored=json.loads((out/'restoration.json').read_text())
assert restored['status'] in ['restored','passed','verified']
inputs=json.loads((out/'inputs.json').read_text());assert len(inputs['regular'])==1145 and len(inputs['gitlinks'])==1
paths=set(subprocess.check_output(['git','ls-tree','-r','--name-only',base,'docs/task-history'],text=True).splitlines())-{'docs/task-history/INDEX.md'}
assert len(paths)==323
for name in paths:assert Path(name).read_bytes()==prior(name),name
new=[('PROGRAM','PROGRAM.93'),('M3','M3.6.9.6.3.2.2.15.2.1.4.3.2.2')]
B=chr(96);false=[]
for tree,leaf in new:
 data=prior('docs/tasks/'+tree+'.md')
 a=data.index(('- ID: '+B+leaf+B).encode())
 # PROGRAM.93 was the last node immediately before Current Frontier.
 pos=data.find(b'\n- ID: ',a+1)
 heading=data.find(b'\n## ',a+1)
 b=min(n for n in [pos,heading] if n!=-1)+1
 archive=Path('docs/task-history')/tree/(leaf+'.md');payload=archive.read_bytes()
 assert payload+b'\n'==data[a:b],leaf
 for label,bad in [('truncated',payload[:-1]),('altered',payload.replace(b'- ID:',b'- iD:',1))]:
  assert bad+b'\n'!=data[a:b];false.append(leaf+':'+label)
current={str(p) for p in Path('docs/task-history').glob('*/*.md')}
assert current==paths|{'docs/task-history/'+tree+'/'+leaf+'.md' for tree,leaf in new}
assert len(current)==325
assert Path('xtask/src/trust.rs').read_bytes()==prior('xtask/src/trust.rs')
assert sha(Path('xtask/src/trust.rs').read_bytes())=='9616b59ff2bc394e6471570bfaf857971671fda4a12c4fb745e557d2d21a1751'
book=Path('docs/book/src/annex-repository.md').read_text()
assert (out/'book-before.md').read_bytes()==prior('docs/book/src/annex-repository.md')
assert book==(out/'book-before.md').read_text()+(out/'book-fragment.md').read_text()
for filename in ['census.py','delete.py','audit.py']:assert (out/filename).read_text() in book,filename
for title,filename in [('Exact admitted deletion','removed.json'),('Immediate residue and retention','residue.json'),('Protected evidence digest census','retained-evidence.json')]:
 marker='<summary>'+title+'</summary>';a=book.index(marker)+len(marker)
 opening=__import__('re').match(r'\s*'+B*3+r'json\n',book[a:]);assert opening
 a+=opening.end();b=book.index('\n'+B*3,a)
 assert json.loads(book[a:b])==json.loads((out/filename).read_text())
record=Path('docs/ARTIFACT_CLEANUP.md').read_text()
assert record.count('- **')==1 and receipt['timestamp_utc'] in record and 'PROGRAM.94' in record
assert sum(p.stat().st_size for p in Path('docs/tasks').rglob('*') if p.is_file())<=819200
result=dict(status='passed',scope='Only admitted idle incremental-cache removal; logical bytes, retained products may be reused; focused/book verification required separately',files_removed=2434,logical_bytes=828209661,cached_bin_files=60,logs_removed=0,retained_evidence_files=176,regular_inputs_restored=1145,opaque_gitlinks=1,old_source_unchanged=True,previous_archives=323,current_archives=325,two_seals_exact=True,history_falsifiers=false,latest_only_record_matches=True,previous_book_bytes_retained=True)
(out/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
print('Cleanup audit passed:2434/828209661 logical bytes;176 evidence/1145 inputs;323 old archives/two new exact; source unchanged; sha256',sha((out/'audit.json').read_bytes()))
```

</details>


### Daily artifact cleanup on 2026-10-21

Measured `2026-10-21T12:06:25+00:00`, `PROGRAM.96`. Only the idle main and owned native-sweep incremental caches were removed, as on every earlier run. Physical metadata and independent native find/stat inventories agreed; the handoff census reported no project job before the inventory and again immediately before the deletion, which rechecked every inventoried identity first. Dependency stores, final products, the empty wasm cache and every cited evidence directory were retained.

The deletion removed `2849` regular cache files totaling `1017454565` logical bytes, including `360` cached `.bin` files. The surveyed locations held no `.log` files. This measures logical size; no allocated-space recovery is claimed.

| Repository-relative location | Regular files | Logical bytes | `.bin` / `.log` | Action |
| --- | --- | --- | --- | --- |
| `target/debug/incremental` | 1353 | 598594688 | 324 / 0 | Remove idle cache |
| `target/wasm32-unknown-unknown/debug/incremental` | 0 | 0 | 0 / 0 | Retain |
| `target/debug/deps` | 800 | 673267478 | 0 / 0 | Retain |
| `target/release` | 117 | 17080485 | 0 / 0 | Retain |
| `target/mutation-sweep/target/debug/incremental` | 1496 | 418859877 | 36 / 0 | Remove idle cache |

This run also compared the identity of every other entry under `target/` — device, inode, kind, mode, size and modification time — before and after the deletion, the run's own scratch excluded: `584889` entries. The first comparison refused, because it held directory metadata that must change. Each row that moved is accounted: the parent of each removed cache, `target/debug` and `target/mutation-sweep/target/debug`, lost exactly that child, so its size dropped and its time moved; and `target/doctrine_scratch` is where `scripts/handoff_census.sh` makes and removes its guard directory, so the census the procedure requires before a deletion moved that directory's time alone — its time matches the census log's within a millisecond. The other `584886` entries, every file among them, are identical, and none appeared or disappeared.

Immediate Python existence checks and native missing-path statuses confirm both admitted directories gone. The three retained surveyed locations keep their inventories exactly. Every one of the `580` files in the five evidence directories the leaves since the last run cite keeps its captured digest. Maintained Python/system checks restore all `1168` captured regular inputs; the submodule remains opaque metadata.

The focused tier must pass on the final staged cleanup record after cache removal. Existing products may be reused, so this verifies workflow regeneration after incremental removal, not a rebuild from source.


<details>
<summary>Exact admitted deletion</summary>

```json
{
  "qualification": "Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git boundary rechecked immediately before removal; only fixed admitted regenerable incremental directories.",
  "removed": [
    {
      "path": "target/debug/incremental",
      "status": "gone",
      "files": 1353,
      "logical_bytes": 598594688
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "status": "gone",
      "files": 1496,
      "logical_bytes": 418859877
    }
  ]
}
```

</details>

<details>
<summary>Immediate residue and retention</summary>

```json
{
  "status": "refused",
  "timestamp_utc": "2026-10-21T12:06:25+00:00",
  "files": 2849,
  "logical_bytes": 1017454565,
  "residue": [
    {
      "path": "target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/debug/incremental: stat: No such file or directory\n"
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/mutation-sweep/target/debug/incremental: stat: No such file or directory\n"
    }
  ],
  "other_target_entries_identity_equal": false,
  "other_target_entries": 584889,
  "other_target_entries_sha256": "1635e85193143f13c322c903aced069ce127051a001973f8d50a6043ec3e04a5",
  "untouched_surveyed_locations": 3,
  "regenerable_only": true,
  "qualification": "Logical bytes, not allocated-space recovery; metadata/native inventory, exact identity recheck and successful no-job census precede removal. Every other entry under target/ but this run's scratch keeps its device, inode, kind, mode, size and mtime. Focused regeneration required separately."
}
```

</details>

<details>
<summary>Every identity difference under target/, accounted</summary>

```json
{
  "status": "accounted",
  "entries_compared": 584889,
  "entries_identical": 584886,
  "changed": [
    {
      "path": "target/debug",
      "changed": [
        "mtime_ns",
        "size"
      ],
      "size": [
        512,
        480
      ],
      "mtime_ns": [
        1792565418857713575,
        1792584365833719016
      ]
    },
    {
      "path": "target/doctrine_scratch",
      "changed": [
        "mtime_ns"
      ],
      "size": [
        1376,
        1376
      ],
      "mtime_ns": [
        1792584320586854764,
        1792584365473755372
      ]
    },
    {
      "path": "target/mutation-sweep/target/debug",
      "changed": [
        "mtime_ns",
        "size"
      ],
      "size": [
        320,
        288
      ],
      "mtime_ns": [
        1792458855841278461,
        1792584365939397537
      ]
    }
  ],
  "handoff_log_mtime_ns": 1792584365474958118,
  "qualification": "The first comparison (residue.json) refused because it held directory metadata the deletion and the pre-deletion handoff census must change. Each of the three rows is classified here; every other entry, every file among them, is identical in device, inode, kind, mode, size and mtime, and none appeared or disappeared."
}
```

</details>

<details>
<summary>Supplemental adapter: census.py</summary>

```python
"""Thirteenth daily artifact cleanup (PROGRAM.96): pre-deletion inventories.

Supplementary, ignored adapter. Two inventories, both taken from the repository root:

* the five surveyed locations, as every earlier run took them: each file and directory on the
  repository's volume, regular or directory, no symbolic link, no nested Git marker, with an
  independent native `find`/`/usr/bin/stat` count and byte sum that must agree;
* the identity of every other entry under `target/` (device, inode, kind, mode, size, mtime_ns),
  the two admitted deletion directories and this run's own scratch excluded, so a later
  comparison proves the deletion touched nothing else.

Logical bytes, never allocated disk space.
"""
from pathlib import Path
import hashlib, json, os, stat, subprocess

root = Path.cwd()
out = root / 'target/artifact-cleanup/2026-10-21'
device = root.stat().st_dev
LOCATIONS = ['target/debug/incremental', 'target/wasm32-unknown-unknown/debug/incremental',
             'target/debug/deps', 'target/release', 'target/mutation-sweep/target/debug/incremental']
ADMITTED = ['target/debug/incremental', 'target/mutation-sweep/target/debug/incremental']
# The evidence the leaves committed since the last run cite (M3-0822 to M3-0828), digested whole.
EVIDENCE = ['target/dependency-absolute-assessment', 'target/dependency-absolute-refusal-witness',
            'target/dependency-path-domain-assessment', 'target/dependency-path-domain-repair',
            'target/dependency-terminal-backslash-control']
# Written during the run by this adapter and by the shell's temporary files.
SCRATCH = ['target/artifact-cleanup/2026-10-21', 'target/tmp']


def metadata(path):
    info = path.lstat()
    assert info.st_dev == device and not stat.S_ISLNK(info.st_mode), str(path)
    assert stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode), str(path)
    return {'path': str(path.relative_to(root)), 'device': info.st_dev, 'inode': info.st_ino,
            'mode': stat.S_IMODE(info.st_mode), 'bytes': info.st_size,
            'kind': 'file' if stat.S_ISREG(info.st_mode) else 'directory'}


def inventory(name):
    base = root / name
    for n in range(1, len(Path(name).parts) + 1):
        ancestor = root.joinpath(*Path(name).parts[:n])
        if not os.path.lexists(ancestor):
            return {'status': 'absent', 'count': 0, 'bytes': 0, 'bin_count': 0, 'log_count': 0,
                    'files': [], 'directories': []}
        info = metadata(ancestor)
        assert info['kind'] == 'directory'
        assert not os.path.lexists(ancestor / '.git'), str(ancestor)
    files, directories = [], []
    for directory, dirs, names in os.walk(base, followlinks=False):
        assert '.git' not in dirs + names, directory
        directories.append(metadata(Path(directory)))
        for entry in dirs + names:
            info = metadata(Path(directory) / entry)
            if info['kind'] == 'file':
                files.append(info)
    sizes = list(map(int, subprocess.check_output(
        ['find', name, '-type', 'f', '-exec', '/usr/bin/stat', '-f', '%z', '{}', '+'], text=True).splitlines()))
    assert len(sizes) == len(files) and sum(sizes) == sum(f['bytes'] for f in files), name
    return {'status': 'present', 'count': len(files), 'bytes': sum(sizes),
            'bin_count': sum(f['path'].endswith('.bin') for f in files),
            'log_count': sum(f['path'].endswith('.log') for f in files),
            'independent_system_find_stat_agrees': True,
            'files': sorted(files, key=lambda r: r['path']),
            'directories': sorted(directories, key=lambda r: r['path'])}


def target_identities():
    """Every entry under target/ but the admitted and scratch directories, as lstat sees it."""
    skip = {str(root / name) for name in ADMITTED + SCRATCH}
    rows = []
    for directory, dirs, names in os.walk(root / 'target', followlinks=False):
        dirs[:] = sorted(d for d in dirs if os.path.join(directory, d) not in skip)
        for entry in sorted(dirs) + sorted(names):
            path = os.path.join(directory, entry)
            info = os.lstat(path)
            rows.append([os.path.relpath(path, root), info.st_dev, info.st_ino, stat.S_IFMT(info.st_mode),
                         stat.S_IMODE(info.st_mode), info.st_size, info.st_mtime_ns])
    rows.sort(key=lambda r: r[0])
    text = ''.join(json.dumps(r) + '\n' for r in rows)
    return {'entries': len(rows), 'sha256': hashlib.sha256(text.encode()).hexdigest()}, text


if __name__ == '__main__':
    records = {name: inventory(name) for name in LOCATIONS}
    (out / 'before.json').write_text(json.dumps({
        'qualification': 'Ignored supplementary metadata adapter; native system find/stat independently agrees. '
                         'Only the named locations traversed; no links, cross-volume entries or nested Git '
                         'ancestors/children. Logical bytes, not allocated disk space.',
        'locations': records}, indent=2) + '\n')
    evidence = {}
    for name in EVIDENCE:
        for directory, dirs, names in os.walk(root / name, followlinks=False):
            dirs.sort()
            for entry in sorted(names):
                path = Path(directory) / entry
                if stat.S_ISREG(path.lstat().st_mode):
                    evidence[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()
    (out / 'retained-evidence.json').write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'retained_evidence_files': len(evidence)}))
    summary, text = target_identities()
    (out / 'target-before.jsonl').write_text(text)
    print(json.dumps({'target_identities': summary}))
    for name, record in records.items():
        print(name, {k: v for k, v in record.items() if k not in ('files', 'directories')})
```

</details>

<details>
<summary>Supplemental adapter: delete.py</summary>

```python
"""Thirteenth daily artifact cleanup (PROGRAM.96): the admitted deletion, then its residue.

Rechecks each admitted directory's complete inventory against `before.json` immediately before
removing it, removes only the two fixed incremental-cache directories, and proves each gone by a
Python existence check and a native `stat` failure. Then it re-takes the identity of every other
entry under `target/` and requires it equal to the pre-deletion inventory.
"""
from pathlib import Path
from datetime import datetime, timezone
import importlib.util, json, os, shutil, subprocess

out = Path('target/artifact-cleanup/2026-10-21')
spec = importlib.util.spec_from_file_location('census', out / 'census.py')
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)

before = json.loads((out / 'before.json').read_text())['locations']
for name in census.ADMITTED:
    assert census.inventory(name) == before[name], name
removed = []
for name in census.ADMITTED:
    expected = before[name]
    if expected['status'] == 'present':
        shutil.rmtree(name)
    assert not os.path.lexists(name), name
    removed.append({'path': name, 'status': 'gone', 'files': expected['count'], 'logical_bytes': expected['bytes']})
(out / 'removed.json').write_text(json.dumps({
    'qualification': 'Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git '
                     'boundary rechecked immediately before removal; only fixed admitted regenerable incremental '
                     'directories.',
    'removed': removed}, indent=2) + '\n')

residue = []
for name in census.ADMITTED:
    native = subprocess.run(['/usr/bin/stat', name], capture_output=True, text=True)
    residue.append({'path': name, 'python_absent': not os.path.lexists(name), 'native_status': native.returncode,
                    'native_stderr': native.stderr})
summary, text = census.target_identities()
(out / 'target-after.jsonl').write_text(text)
untouched = text == (out / 'target-before.jsonl').read_text()
retained = {name: census.inventory(name) == before[name] for name in census.LOCATIONS if name not in census.ADMITTED}
(out / 'residue.json').write_text(json.dumps({
    'status': 'verified' if untouched and all(retained.values()) and all(
        r['python_absent'] and r['native_status'] != 0 for r in residue) else 'refused',
    'timestamp_utc': datetime.now(timezone.utc).isoformat(timespec='seconds'),
    'files': sum(r['files'] for r in removed),
    'logical_bytes': sum(r['logical_bytes'] for r in removed),
    'residue': residue,
    'other_target_entries_identity_equal': untouched,
    'other_target_entries': summary['entries'],
    'other_target_entries_sha256': summary['sha256'],
    'untouched_surveyed_locations': sum(retained.values()),
    'regenerable_only': True,
    'qualification': 'Logical bytes, not allocated-space recovery; metadata/native inventory, exact identity recheck '
                     'and successful no-job census precede removal. Every other entry under target/ but this run\'s '
                     'scratch keeps its device, inode, kind, mode, size and mtime. Focused regeneration required '
                     'separately.'}, indent=2) + '\n')
print((out / 'residue.json').read_text())
```

</details>

<details>
<summary>Supplemental adapter: delta.py</summary>

```python
"""Thirteenth daily artifact cleanup (PROGRAM.96): classify every identity difference under target/.

`delete.py` compared the whole identity of every other entry under `target/` and refused: three
rows differ. This names each one and the only change it may carry, and refuses anything else:

* the parent directory of each removed cache loses exactly that child, so its size drops by the
  one entry and its mtime moves; its device, inode, kind and mode stay;
* `target/doctrine_scratch` is where `scripts/handoff_census.sh` makes and removes its
  `handoff_census.XXXXXX` guard, and the census ran between the two inventories (before the
  deletion, as the procedure requires), so only that directory's mtime moves.

Every file, link and other directory must be identical, and no entry may appear or disappear.
"""
from pathlib import Path
import json

out = Path('target/artifact-cleanup/2026-10-21')
FIELDS = ['path', 'device', 'inode', 'kind', 'mode', 'size', 'mtime_ns']
EXPECTED = {
    'target/debug': {'size', 'mtime_ns'},
    'target/mutation-sweep/target/debug': {'size', 'mtime_ns'},
    'target/doctrine_scratch': {'mtime_ns'},
}


def rows(name):
    return [dict(zip(FIELDS, json.loads(line))) for line in (out / name).read_text().splitlines()]


before, after = rows('target-before.jsonl'), rows('target-after.jsonl')
assert [r['path'] for r in before] == [r['path'] for r in after], 'an entry appeared or disappeared'
changed = []
for old, new in zip(before, after):
    moved = {k for k in FIELDS if old[k] != new[k]}
    if moved:
        assert moved == EXPECTED.get(old['path']), (old['path'], sorted(moved))
        changed.append({'path': old['path'], 'changed': sorted(moved),
                        'size': [old['size'], new['size']], 'mtime_ns': [old['mtime_ns'], new['mtime_ns']]})
assert [c['path'] for c in changed] == sorted(EXPECTED), changed
for c in changed:
    if c['path'] != 'target/doctrine_scratch':
        assert c['size'][1] < c['size'][0], c
handoff = (out / 'process-predelete.log').stat().st_mtime_ns
scratch = next(c for c in changed if c['path'] == 'target/doctrine_scratch')['mtime_ns'][1]
assert abs(scratch - handoff) < 10_000_000, (scratch, handoff)
result = {
    'status': 'accounted',
    'entries_compared': len(before),
    'entries_identical': len(before) - len(changed),
    'changed': changed,
    'handoff_log_mtime_ns': handoff,
    'qualification': 'The first comparison (residue.json) refused because it held directory metadata the deletion '
                     'and the pre-deletion handoff census must change. Each of the three rows is classified here; '
                     'every other entry, every file among them, is identical in device, inode, kind, mode, size '
                     'and mtime, and none appeared or disappeared.',
}
(out / 'identity-delta.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({k: v for k, v in result.items() if k != 'qualification'}, indent=1))
```

</details>

The evidence digests, the complete pre-deletion metadata and both identity inventories are under `target/artifact-cleanup/2026-10-21/`. These adapters are supplemental; maintained input checks, native system inventories and the focused tier supply primary verification.


### Daily artifact cleanup on 2026-10-22

Measured `2026-10-22T15:54:45+00:00`, `PROGRAM.101`. Only the idle main and owned native-sweep incremental caches were removed, as on every earlier run. Physical metadata and independent native find/stat inventories agreed; the handoff census reported no project job before the inventory and again immediately before the deletion, which rechecked every inventoried identity first. Dependency stores, final products, the empty wasm cache and every cited evidence directory were retained.

The deletion removed `5382` regular cache files totaling `1181873277` logical bytes, including `255` cached `.bin` files. The surveyed locations held no `.log` files. This measures logical size; no allocated-space recovery is claimed.

| Repository-relative location | Regular files | Logical bytes | `.bin` / `.log` | Action |
| --- | --- | --- | --- | --- |
| `target/debug/incremental` | 3886 | 760689034 | 219 / 0 | Remove idle cache |
| `target/wasm32-unknown-unknown/debug/incremental` | 0 | 0 | 0 / 0 | Retain |
| `target/debug/deps` | 804 | 680451726 | 0 / 0 | Retain |
| `target/release` | 117 | 17080485 | 0 / 0 | Retain |
| `target/mutation-sweep/target/debug/incremental` | 1496 | 421184243 | 36 / 0 | Remove idle cache |

This run also compared the identity of every other entry under `target/` — device, inode, kind, mode, size and modification time — before and after the deletion, the run's own scratch excluded: `590426` entries. The first comparison refused, because it held directory metadata that must change. Each row that moved is accounted: the parent of each removed cache, `target/debug` and `target/mutation-sweep/target/debug`, lost exactly that child, so its size dropped and its time moved; and `target/doctrine_scratch` is where `scripts/handoff_census.sh` makes and removes its guard directory, so the census the procedure requires before a deletion moved that directory's time alone — its time matches the census log's within a millisecond. The other `590423` entries, every file among them, are identical, and none appeared or disappeared.

Immediate Python existence checks and native missing-path statuses confirm both admitted directories gone. The three retained surveyed locations keep their inventories exactly. Every one of the `4862` files in the `14` evidence directories the leaves since the last run cite keeps its captured digest. Maintained Python/system checks restore all `1205` captured regular inputs; the submodule remains opaque metadata.

The focused tier must pass on the final staged cleanup record after cache removal. Existing products may be reused, so this verifies workflow regeneration after incremental removal, not a rebuild from source.


<details>
<summary>Exact admitted deletion</summary>

```json
{
  "qualification": "Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git boundary rechecked immediately before removal; only fixed admitted regenerable incremental directories.",
  "removed": [
    {
      "path": "target/debug/incremental",
      "status": "gone",
      "files": 3886,
      "logical_bytes": 760689034
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "status": "gone",
      "files": 1496,
      "logical_bytes": 421184243
    }
  ]
}
```

</details>

<details>
<summary>Immediate residue and retention</summary>

```json
{
  "status": "refused",
  "timestamp_utc": "2026-10-22T15:54:45+00:00",
  "files": 5382,
  "logical_bytes": 1181873277,
  "residue": [
    {
      "path": "target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/debug/incremental: stat: No such file or directory\n"
    },
    {
      "path": "target/mutation-sweep/target/debug/incremental",
      "python_absent": true,
      "native_status": 1,
      "native_stderr": "stat: target/mutation-sweep/target/debug/incremental: stat: No such file or directory\n"
    }
  ],
  "other_target_entries_identity_equal": false,
  "other_target_entries": 590426,
  "other_target_entries_sha256": "da8e908fbe9a64a9021d607c0a159c577cd5d43188aa40f471bbc52bf4642c96",
  "untouched_surveyed_locations": 3,
  "regenerable_only": true,
  "qualification": "Logical bytes, not allocated-space recovery; metadata/native inventory, exact identity recheck and successful no-job census precede removal. Every other entry under target/ but this run's scratch keeps its device, inode, kind, mode, size and mtime. Focused regeneration required separately."
}
```

</details>

<details>
<summary>Every identity difference under target/, accounted</summary>

```json
{
  "status": "accounted",
  "entries_compared": 590426,
  "entries_identical": 590423,
  "changed": [
    {
      "path": "target/debug",
      "changed": [
        "mtime_ns",
        "size"
      ],
      "size": [
        512,
        480
      ],
      "mtime_ns": [
        1792684087741431498,
        1792684468935316556
      ]
    },
    {
      "path": "target/doctrine_scratch",
      "changed": [
        "mtime_ns"
      ],
      "size": [
        1408,
        1408
      ],
      "mtime_ns": [
        1792684423846932508,
        1792684468400779067
      ]
    },
    {
      "path": "target/mutation-sweep/target/debug",
      "changed": [
        "mtime_ns",
        "size"
      ],
      "size": [
        320,
        288
      ],
      "mtime_ns": [
        1792585260236519722,
        1792684469011749780
      ]
    }
  ],
  "handoff_log_mtime_ns": 1792684468401734440,
  "qualification": "The first comparison (residue.json) refused because it held directory metadata the deletion and the pre-deletion handoff census must change. Each of the three rows is classified here; every other entry, every file among them, is identical in device, inode, kind, mode, size and mtime, and none appeared or disappeared."
}
```

</details>

<details>
<summary>Supplemental adapter: census.py</summary>

```python
"""Fourteenth daily artifact cleanup (PROGRAM.101): pre-deletion inventories.

Supplementary, ignored adapter. Two inventories, both taken from the repository root:

* the five surveyed locations, as every earlier run took them: each file and directory on the
  repository's volume, regular or directory, no symbolic link, no nested Git marker, with an
  independent native `find`/`/usr/bin/stat` count and byte sum that must agree;
* the identity of every other entry under `target/` (device, inode, kind, mode, size, mtime_ns),
  the two admitted deletion directories and this run's own scratch excluded, so a later
  comparison proves the deletion touched nothing else.

Logical bytes, never allocated disk space.
"""
from pathlib import Path
import hashlib, json, os, stat, subprocess

root = Path.cwd()
out = root / 'target/artifact-cleanup/2026-10-22'
device = root.stat().st_dev
LOCATIONS = ['target/debug/incremental', 'target/wasm32-unknown-unknown/debug/incremental',
             'target/debug/deps', 'target/release', 'target/mutation-sweep/target/debug/incremental']
ADMITTED = ['target/debug/incremental', 'target/mutation-sweep/target/debug/incremental']
# The evidence the leaves committed since the last run cite (M3-0830 to PROGRAM-0845), digested whole.
EVIDENCE = ['target/catalog-dependency-assessment', 'target/catalog-reader-cleanup', 'target/colon-path-assessment',
            'target/configuration-path-assessment', 'target/configuration-refusal-control',
            'target/configuration-route-assessment', 'target/dependency-absolute-assessment',
            'target/dependency-block-checkpoint', 'target/dependency-path-domain-repair', 'target/m331',
            'target/m33-design-review', 'target/outside-dependency-refusal', 'target/own-package-assessment',
            'target/program99']
# Written during the run by this adapter and by the shell's temporary files.
SCRATCH = ['target/artifact-cleanup/2026-10-22', 'target/tmp']


def metadata(path):
    info = path.lstat()
    assert info.st_dev == device and not stat.S_ISLNK(info.st_mode), str(path)
    assert stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode), str(path)
    return {'path': str(path.relative_to(root)), 'device': info.st_dev, 'inode': info.st_ino,
            'mode': stat.S_IMODE(info.st_mode), 'bytes': info.st_size,
            'kind': 'file' if stat.S_ISREG(info.st_mode) else 'directory'}


def inventory(name):
    base = root / name
    for n in range(1, len(Path(name).parts) + 1):
        ancestor = root.joinpath(*Path(name).parts[:n])
        if not os.path.lexists(ancestor):
            return {'status': 'absent', 'count': 0, 'bytes': 0, 'bin_count': 0, 'log_count': 0,
                    'files': [], 'directories': []}
        info = metadata(ancestor)
        assert info['kind'] == 'directory'
        assert not os.path.lexists(ancestor / '.git'), str(ancestor)
    files, directories = [], []
    for directory, dirs, names in os.walk(base, followlinks=False):
        assert '.git' not in dirs + names, directory
        directories.append(metadata(Path(directory)))
        for entry in dirs + names:
            info = metadata(Path(directory) / entry)
            if info['kind'] == 'file':
                files.append(info)
    sizes = list(map(int, subprocess.check_output(
        ['find', name, '-type', 'f', '-exec', '/usr/bin/stat', '-f', '%z', '{}', '+'], text=True).splitlines()))
    assert len(sizes) == len(files) and sum(sizes) == sum(f['bytes'] for f in files), name
    return {'status': 'present', 'count': len(files), 'bytes': sum(sizes),
            'bin_count': sum(f['path'].endswith('.bin') for f in files),
            'log_count': sum(f['path'].endswith('.log') for f in files),
            'independent_system_find_stat_agrees': True,
            'files': sorted(files, key=lambda r: r['path']),
            'directories': sorted(directories, key=lambda r: r['path'])}


def target_identities():
    """Every entry under target/ but the admitted and scratch directories, as lstat sees it."""
    skip = {str(root / name) for name in ADMITTED + SCRATCH}
    rows = []
    for directory, dirs, names in os.walk(root / 'target', followlinks=False):
        dirs[:] = sorted(d for d in dirs if os.path.join(directory, d) not in skip)
        for entry in sorted(dirs) + sorted(names):
            path = os.path.join(directory, entry)
            info = os.lstat(path)
            rows.append([os.path.relpath(path, root), info.st_dev, info.st_ino, stat.S_IFMT(info.st_mode),
                         stat.S_IMODE(info.st_mode), info.st_size, info.st_mtime_ns])
    rows.sort(key=lambda r: r[0])
    text = ''.join(json.dumps(r) + '\n' for r in rows)
    return {'entries': len(rows), 'sha256': hashlib.sha256(text.encode()).hexdigest()}, text


if __name__ == '__main__':
    records = {name: inventory(name) for name in LOCATIONS}
    (out / 'before.json').write_text(json.dumps({
        'qualification': 'Ignored supplementary metadata adapter; native system find/stat independently agrees. '
                         'Only the named locations traversed; no links, cross-volume entries or nested Git '
                         'ancestors/children. Logical bytes, not allocated disk space.',
        'locations': records}, indent=2) + '\n')
    evidence = {}
    for name in EVIDENCE:
        for directory, dirs, names in os.walk(root / name, followlinks=False):
            dirs.sort()
            for entry in sorted(names):
                path = Path(directory) / entry
                if stat.S_ISREG(path.lstat().st_mode):
                    evidence[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()
    (out / 'retained-evidence.json').write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'retained_evidence_files': len(evidence)}))
    summary, text = target_identities()
    (out / 'target-before.jsonl').write_text(text)
    print(json.dumps({'target_identities': summary}))
    for name, record in records.items():
        print(name, {k: v for k, v in record.items() if k not in ('files', 'directories')})
```

</details>

<details>
<summary>Supplemental adapter: delete.py</summary>

```python
"""Fourteenth daily artifact cleanup (PROGRAM.101): the admitted deletion, then its residue.

Rechecks each admitted directory's complete inventory against `before.json` immediately before
removing it, removes only the two fixed incremental-cache directories, and proves each gone by a
Python existence check and a native `stat` failure. Then it re-takes the identity of every other
entry under `target/` and requires it equal to the pre-deletion inventory.
"""
from pathlib import Path
from datetime import datetime, timezone
import importlib.util, json, os, shutil, subprocess

out = Path('target/artifact-cleanup/2026-10-22')
spec = importlib.util.spec_from_file_location('census', out / 'census.py')
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)

before = json.loads((out / 'before.json').read_text())['locations']
for name in census.ADMITTED:
    assert census.inventory(name) == before[name], name
removed = []
for name in census.ADMITTED:
    expected = before[name]
    if expected['status'] == 'present':
        shutil.rmtree(name)
    assert not os.path.lexists(name), name
    removed.append({'path': name, 'status': 'gone', 'files': expected['count'], 'logical_bytes': expected['bytes']})
(out / 'removed.json').write_text(json.dumps({
    'qualification': 'Supplemental adapter; every inventoried file/directory identity and same-volume/no-link/no-Git '
                     'boundary rechecked immediately before removal; only fixed admitted regenerable incremental '
                     'directories.',
    'removed': removed}, indent=2) + '\n')

residue = []
for name in census.ADMITTED:
    native = subprocess.run(['/usr/bin/stat', name], capture_output=True, text=True)
    residue.append({'path': name, 'python_absent': not os.path.lexists(name), 'native_status': native.returncode,
                    'native_stderr': native.stderr})
summary, text = census.target_identities()
(out / 'target-after.jsonl').write_text(text)
untouched = text == (out / 'target-before.jsonl').read_text()
retained = {name: census.inventory(name) == before[name] for name in census.LOCATIONS if name not in census.ADMITTED}
(out / 'residue.json').write_text(json.dumps({
    'status': 'verified' if untouched and all(retained.values()) and all(
        r['python_absent'] and r['native_status'] != 0 for r in residue) else 'refused',
    'timestamp_utc': datetime.now(timezone.utc).isoformat(timespec='seconds'),
    'files': sum(r['files'] for r in removed),
    'logical_bytes': sum(r['logical_bytes'] for r in removed),
    'residue': residue,
    'other_target_entries_identity_equal': untouched,
    'other_target_entries': summary['entries'],
    'other_target_entries_sha256': summary['sha256'],
    'untouched_surveyed_locations': sum(retained.values()),
    'regenerable_only': True,
    'qualification': 'Logical bytes, not allocated-space recovery; metadata/native inventory, exact identity recheck '
                     'and successful no-job census precede removal. Every other entry under target/ but this run\'s '
                     'scratch keeps its device, inode, kind, mode, size and mtime. Focused regeneration required '
                     'separately.'}, indent=2) + '\n')
print((out / 'residue.json').read_text())
```

</details>

<details>
<summary>Supplemental adapter: delta.py</summary>

```python
"""Fourteenth daily artifact cleanup (PROGRAM.101): classify every identity difference under target/.

`delete.py` compared the whole identity of every other entry under `target/` and refused: three
rows differ. This names each one and the only change it may carry, and refuses anything else:

* the parent directory of each removed cache loses exactly that child, so its size drops by the
  one entry and its mtime moves; its device, inode, kind and mode stay;
* `target/doctrine_scratch` is where `scripts/handoff_census.sh` makes and removes its
  `handoff_census.XXXXXX` guard, and the census ran between the two inventories (before the
  deletion, as the procedure requires), so only that directory's mtime moves.

Every file, link and other directory must be identical, and no entry may appear or disappear.
"""
from pathlib import Path
import json

out = Path('target/artifact-cleanup/2026-10-22')
FIELDS = ['path', 'device', 'inode', 'kind', 'mode', 'size', 'mtime_ns']
EXPECTED = {
    'target/debug': {'size', 'mtime_ns'},
    'target/mutation-sweep/target/debug': {'size', 'mtime_ns'},
    'target/doctrine_scratch': {'mtime_ns'},
}


def rows(name):
    return [dict(zip(FIELDS, json.loads(line))) for line in (out / name).read_text().splitlines()]


before, after = rows('target-before.jsonl'), rows('target-after.jsonl')
assert [r['path'] for r in before] == [r['path'] for r in after], 'an entry appeared or disappeared'
changed = []
for old, new in zip(before, after):
    moved = {k for k in FIELDS if old[k] != new[k]}
    if moved:
        assert moved == EXPECTED.get(old['path']), (old['path'], sorted(moved))
        changed.append({'path': old['path'], 'changed': sorted(moved),
                        'size': [old['size'], new['size']], 'mtime_ns': [old['mtime_ns'], new['mtime_ns']]})
assert [c['path'] for c in changed] == sorted(EXPECTED), changed
for c in changed:
    if c['path'] != 'target/doctrine_scratch':
        assert c['size'][1] < c['size'][0], c
handoff = (out / 'process-predelete.log').stat().st_mtime_ns
scratch = next(c for c in changed if c['path'] == 'target/doctrine_scratch')['mtime_ns'][1]
assert abs(scratch - handoff) < 10_000_000, (scratch, handoff)
result = {
    'status': 'accounted',
    'entries_compared': len(before),
    'entries_identical': len(before) - len(changed),
    'changed': changed,
    'handoff_log_mtime_ns': handoff,
    'qualification': 'The first comparison (residue.json) refused because it held directory metadata the deletion '
                     'and the pre-deletion handoff census must change. Each of the three rows is classified here; '
                     'every other entry, every file among them, is identical in device, inode, kind, mode, size '
                     'and mtime, and none appeared or disappeared.',
}
(out / 'identity-delta.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({k: v for k, v in result.items() if k != 'qualification'}, indent=1))
```

</details>

The evidence digests, the complete pre-deletion metadata and both identity inventories are under `target/artifact-cleanup/2026-10-22/`. These adapters are supplemental; maintained input checks, native system inventories and the focused tier supply primary verification.
