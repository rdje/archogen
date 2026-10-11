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

Daily artifact cleanup has a task-tree owner and a metadata inventory before deletion. The latest date and summary
live in `docs/ARTIFACT_CLEANUP.md`; the owning leaf keeps the deletion list, regeneration commands, retained evidence
and cold verification. Compiler caches may be regenerated; cited review evidence, tools and dependency stores are
retained unless their removal is proven safe. Cleanup respects the same read-only boundary around other Git
repositories. The record is overwritten after each run rather than growing into another history.

## Before handing the workspace over

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

A subsequent bounded `split_command` measurement at `5d51a17` selected every current helper edit: 29 attempts,
13 named finishing kills and 16 survivors, with no unviable, timeout or unjudged results. Its strict exit was one;
complete measurement is not adequacy success. The maintained input census verified all 1003 captured regular
inputs restored and unchanged gitlink metadata; the copied source also matched. The task record preserves
source-bound initial/current edit bindings and the native reproduction command. No survivor closes from this run.

A separate real Cargo verbose-output probe found a checkout-path defect. Ordinary and space-bearing output
paths parse correctly; with a literal apostrophe, Cargo prints a close-quote/escaped-quote/reopen-quote sequence.
The current helper toggles at the escaped quote and retains its backslash, folding following flags into the
output-directory token. Cargo builds successfully, but inventory collection can then seek dependency files in
a nonexistent directory. This is a collection failure, without an authenticity-bypass claim. The exploratory
probe producer is untracked; `M3.6.9.6.3.2.2.10.2` owns a permanent public reproduction and repair before
further helper survivor judgments. No general shell-conformance guarantee follows from the finite probe.

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
  work, the list of what is next and the logs stay where they were.
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

Each tree also keeps a Commit Log, one row per commit that worked on it, and `COMMIT-LOG-ROWS`
(`scripts/check_commit_log_rows.sh`) holds every commit whose subject names a work unit to having its row —
the pending commit included, so a commit that forgot its row is refused before it lands rather than found by
hand a commit later, as two were on `2026-10-01` (`PROGRAM.46`). The rows missing when the gate landed are a
backlog listed in the script, which may only shrink.

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

## Where the landing page sends things

A size limit on `README.md` does not remove the need to write things down; it moves it somewhere else. So
`README_POLICY.md` also asks that every place the README points a reader to, and every place its checks tell an
author to move detail to, is itself kept in bounds. `README-ROUTES` works that list out from the README's links and
from what its two checks actually print. It follows each place on to wherever that place's own check sends overflow,
and holds each one to the ceiling registered for it in the policy. The changelog and the development notes are
bounded as rolling ledgers (above). A design's review history, which grows by a round at a time, overflows from the
decisions folder to `docs/reviews/`, which has ceilings of its own, and a closed review's history overflows on to
`docs/review-history/` (above). The task trees are bounded too, and their
finished parts overflow to `docs/task-history/`, as settled sections of a decision record do to
`docs/decision-history/` (above). A folder grown too large is split by subject into sub-folders, each with ceilings
of its own, while the folder's own limits still count everything in them, so a split adds no room; and no ceiling
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
