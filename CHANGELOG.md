# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — iterator-chain candidates preserve multiline call structure and ordering

`ARCHOGEN-M3-0628` (leaf `M3.6.9.2.3.1`), `2026-10-11`.

- Chain candidates narrow empty-argument iter/into_iter or filter calls to first/last, or drop a filter/chain call,
  using lexical balanced multiline call spans. Comments/literals and nonempty iter arguments are excluded. Dropped
  calls retain an inert comment: a discovered standalone-removal edge otherwise violates strict nonempty equivalents.
- Validation: three methods fail first; initial emission exposes the real CLI empty-field refusal. Repaired twelve
  Python methods pass, with 44 emitted chain candidates plus eight baselines compiled, 196 order/filter observations
  and three multiline CLI landing/restoration cases. Three strict named mutation kills; 15 tool tests pass. All
  earlier-operator projections, including iter/guard, remain byte-identical across four files; new lists are planning.
  Syntax, book and final staged doctrines before commit. No production viability or adequacy claim; remaining
  membership, closure and per-operator count slices owned separately.

## archogen — actual guarded-branch candidates preserve complete conditions and body work

`ARCHOGEN-M3-0627` (leaf `M3.6.9.2.2.2.2`), `2026-10-11`.

- Guard candidates pair a rewritten loop header with one eligible branch gated to first/last/ends; all other body
  work stays, and the original condition evaluates before the flag. Nested loop/closure branches are excluded from
  an outer loop's candidates. Encountered literal-span defect repaired: final quotes/comments stay in the condition.
- Validation: four methods fail before repair/emission; nine Python methods pass afterwards. 102 emitted guards
  and 22 baselines compile with 372 semantic observations, plus four actual-candidate CLI landing/restoration cases.
  Three strict catalogue controls are killed by named tests; 15 tool tests pass. An initial catalogue usage error
  is not counted. Four complete old-operator projections, including iter, remain byte-identical; new guard-only
  lists are planning, with no production viability or adequacy claim. Syntax, book and staged doctrines before commit.
- Book examples, accepted design status, toolbox, leaf/parent closures, logs and resume pointers agree; chain,
  membership, closure operators and production measurement remain pending.

## archogen — committed closed units and completed reviews are archived

`ARCHOGEN-PROGRAM-0626` (leaf `PROGRAM.83.2`), `2026-10-11`.

- Published seals move 15 PROGRAM subtrees (17 leaves) and four M3 slices (four leaves), reconstructing both trees
  byte for byte. The closed sealing and adequacy reviews move unchanged behind their original citation paths;
  copies read back exactly 98 522 and 9 565 bytes. Open implementation and other pending work stay live.
- Validation: both archive tools prove source/copy identity and their provenance gates; final staged task/review
  histories, review counts, size, book and doctrine gates before commit. Existing PROGRAM.70 parent commit warning
  retained explicitly, never replaced by an invented reference. Two frozen task copies retain original EOF blank
  lines; exact provenance passes and only those copies are excluded from the remaining whitespace check.
  No implementation behavior or new suite claim.

## archogen — completed-history archival is prepared

`ARCHOGEN-PROGRAM-0625` (leaf `PROGRAM.83.1`), `2026-10-11`.

- Own the committed closed PROGRAM/M3 task units and two completed review histories under PROGRAM.83.2. The
  adequacy index status now uses the published archive contract's exact `closed: ` prefix; closure date/result
  retained, frozen source history untouched. Trim the reviews index's trailing blank line.
- Validation: the archive command first refuses with exit 1 and explicitly writes nothing; corrected prefix and
  source identity checked, staged review/count, book and doctrine gates before commit. No implementation changes.

## archogen — task-tree sealing review closes at round 37

`ARCHOGEN-PROGRAM-0624` (leaf `PROGRAM.69`), `2026-10-11`.

- Independent round 37 finds zero new defects, actionable regression-arm gaps or remarks. All prior findings are
  answered; the review and owning leaf close. Pending index-order, subfolder, unforeseen-exception and interrupt-
  timing repairs remain separately owned. Closed histories are ready for pure archival commits.
- Validation: 347 synthetic checker invocations match expectations (260 core, 87 pinned shell); separately 4 096
  finite unit/oracle comparisons agree. Root recount/authentication of 17 source snapshots exits 0. All 37 summary
  rows checked before counted-list removal; staged history, book and doctrine gates before commit. No complete
  native suite replay, native transaction/signal proof or particular real historical byte-total claim is added.
- Lockstep: frozen review/history/index, design, book annex, leaf/logs and resume snapshots. Two older design-summary
  verdicts compacted without altering its counts, key results or full historical round to retain the decisions cap.

## archogen — guard branch scope support precedes candidate rewriting

`ARCHOGEN-M3-0623` (leaf `M3.6.9.2.2.2.1`), `2026-10-11`.

- Structural lexical scopes locate a loop's one-line condition spans outside nested loops and block/expression
  closures, retaining ordinary nested if branches. Literal operand marks, raw identifiers, labels, multiline loop
  heads, block/if/match operands, struct patterns and const generics are held by a compiled stable-Rust fixture.
- Validation: old support errors first; 15 tool tests pass, including six Python methods. Two strict catalogue
  controls killed by their named test after correcting one broken anchor's indentation. All four complete candidate
  lists remain byte-identical; scope probes accept current production headers. Syntax/book/staged gates before
  commit. No guard candidates or production adequacy result yet. Orphan M3 log rows rejoined without text changes.

## archogen — census spans and unusual paths are measured exactly

`ARCHOGEN-PROGRAM-0622` (leaf `PROGRAM.69`), `2026-10-11`.

- Round 36's two census defects are repaired: source-byte boundaries preserve physical EOF, and NUL-delimited UTF-8
  paths retain Unicode/control-character names instead of filtering Git's quoted display text. Invalid listing
  encoding/terminators refuse. Empty pipe rows in review tables now produce named refusals instead of tracebacks.
- Validation: fourteen census and six count controls; unchanged readers give 133/13 of 146 and 153/3 of 156,
  then repaired complete suites pass 146/0 and 156/0. Independent literal spans and single/partial path counterparts
  distinguish exact totals. Reviewer evidence is untracked; maintained regression controls are tracked. Syntax,
  book and final staged gates before commit; no Rust changed or new historical aggregate claimed. Round 37 next.

## archogen — loop-position mutations gain compiled semantic controls

`ARCHOGEN-M3-0621` (leaf `M3.6.9.2.2.1`), `2026-10-11`.

- The sweep adds first/last/ends variants of one-line loop headers, preserving binding patterns, labels and comments
  with fresh helper names. A linear Rust lexical mask excludes headers inside literals and nested comments.
- Validation: old discovery fails both new methods; twelve forms compile baseline plus three actual candidates,
  passing 144 semantic observations. Three further literal/comment candidates compile; 15 tool tests pass, and
  two strict catalogue controls are killed by their named test. Old-operator projections remain byte-identical
  across three criterion files and the recognizer. Syntax, book and staged doctrine checks before commit.
  Guard scopes, remaining operators and production adequacy measurements are pending; no list is called coverage.

## archogen — failed census evidence and incomplete summary headers refuse

`ARCHOGEN-PROGRAM-0620` (leaf `PROGRAM.69`), `2026-10-11`.

- Round 35's selected-tree read defect is repaired: failed blobs name commit/path and refuse without a final
  aggregate, rather than contributing zero to a successful partial measurement. Its grammar remark is also
  addressed: every recognized summary header needs its next separator, at end of input and before non-rows/headers.
- Validation: eight new finite-shim census and twelve summary-boundary controls; unchanged readers give 129/3 of
  132 and 140/10 of 150, then repaired complete suites pass 132/0 and 150/0. Positive/empty/listing/UTF-8 counterparts
  distinguish the evidence boundary. Review report/producers are untracked; maintained regressions are tracked.
  Syntax, book, final staged count/history and doctrine checks before commit; independent round 36 next. No Rust
  changed or new full Rust-suite result claimed. Book/record/history/task/resume contracts synchronized.

## archogen — mutation source spans and validated equivalence records

`ARCHOGEN-M3-0619` (leaf `M3.6.9.2.1`), `2026-10-11`.

- Sweep candidates can replace source-line intervals; existing one-line lists and IDs are byte-identical. Explicit
  `from-json` / `to-json` fields support multiline equivalents while raw backslashes remain literal. All entries,
  including EOF, require one source, one replacement and a nonempty reason; malformed and duplicate records refuse
  before commands. Encountered failed-name defect repaired in both harnesses: diagnostic output is not a failure list.
- Validation: old record/name controls fail first; three Python methods cover 28 synthetic CLI cases and restoration;
  15 mutation-tool tests pass, including 15 outcome controls. Two strict catalogue controls killed by the named test.
  Three old production lists compare byte for byte (932 candidates, planning only); syntax, book and final focused/
  doctrine gates before commit. No new operator production viability or adequacy result claimed; loops/guards next.

## archogen — unconditional review-link grammar and parenthesized destinations gain controls

`ARCHOGEN-PROGRAM-0618` (leaf `PROGRAM.69`), `2026-10-11`.

- Round 34 found no production defect and two arm gaps: removing unconditional file-cell validation and rejecting
  permitted parentheses inside angle destinations both survived all 126 controls. Twelve new arms hold listed and
  closed cells, the empty-counted-list boundary, and parenthesized angle paths across titles/primary obligations.
  The production reader is unchanged; book examples and review/task/resume summaries agree.
- Validation: both mutants reproduce 126/0 natively before the new controls, then fail three and eight respectively
  out of 138. All 138 real-reader arms pass; syntax, book and final staged count/history/doctrine gates. Mutation
  producers are untracked evidence; their distinguishing regression arms are tracked. Independent round 35 next.

## archogen — mutation kills require finishing test evidence, with opt-in strict timeouts

`ARCHOGEN-M3-0617` (leaf `M3.6.9.1`), `2026-10-11`.

- Both harnesses now report invocation/crash/incomplete output without a failed libtest summary and failure-list
  name as unjudged, exit 2. `--timeouts-fail` refuses hangs in both tools, for either catalogue expectation and
  regardless of sweep equivalents; ordinary reached-hang behavior stays unchanged. CLI help and parser included.
- Validation: the old real-sweep control reproduces false kills; 14 mutation-tool tests pass, including 13 synthetic
  sweep cases, eight catalogue command outcomes and failed spawn, with restoration. Six new durable catalogue
  controls are killed by named tests; real lcm kill and harmonic survivor behave as expected. Focused tier passes
  3/0 after correcting this session's PATH setup, which had bypassed fixture toolchain selection; final staged rerun
  stamps the committed tree. Book/toolbox/task contracts agree. Throwaway logs remain untracked evidence.
  PROGRAM.69 round 34 found two arm gaps, now owned for the next slice; no production defect found.

## archogen — the parent-test criterion's design review closes at zero defects

`ARCHOGEN-M3-0616` (leaf `M3.6.9`), `2026-10-11`.

- Independent round 3 accepts the 32-pair census, unique planned anchors, complete scope, bound seam and actual-
  witness method: zero defects and one remark. The existing harnesses count unattributed invocation failures as
  kills; M3.6.9.1 owns their repair before measurement. A finishing kill will require a failed libtest summary and
  failure-list name; other non-compilation failures are unjudged. Six implementation slices are tracked in order.
- Validation: independent static/source parsing and 932 existing-operator planning candidates, with no proposed
  compilation, viability or verdict claimed; the pinned helper reproduces killed/empty names for synthetic exit 101.
  Report and producers are untracked; book, review-count and final staged doctrine gates before commit.
  The design review is closed; implementation remains pending. The decisions index's stale M3.1.1 review status
  now agrees with its canonical record and review index: closed `2026-10-05`, round 31.

## archogen — primary review links keep authority across destinations and titles

`ARCHOGEN-PROGRAM-0615` (leaf `PROGRAM.69`), `2026-10-11`.

- Round 33 found ordinary angle-destination/title links skipped for no record or a secondary explanation. The
  reader now locates the first unescaped balanced label, supports explicit bare/angle destinations and optional
  titles, and refuses unsupported first syntax in both file cells. Literal path treatment and the bracket-free
  no-file boundary are explicit; counted pairs remain authoritative.
- Validation: 35 tracked controls, 96 pass / 30 fail before repair, then 126 pass / 0 fail. The original five shell
  cases now refuse the primary table; selecting a later link fails 33 arms and omitting unsupported-first refusal
  fails six. An initial syntax-error mutant is excluded. Producers are untracked, not durable enforcement; the
  regression arms are tracked. Shell syntax, final staged count/history/book/doctrine checks before commit.
  Record, review history, book and resume pointers agree on round 33's answer and round 34 next.

## archogen — parent-test criterion gains a source-linked census and complete production scope

`ARCHOGEN-M3-0614` (leaf `M3.6.9`), `2026-10-11`.

- Round 2's two defects and four remarks answered in the design: 32 specified route/parent pairs (23 existing plus
  nine additions), whole production ranges including callers, a bound source/configuration seam, actual operator
  outcomes and semantic witnesses, borrowed membership variants, and strict timeout modes in both harnesses.
  Implementation still waits for a zero-defect review. The earlier `ARCHOGEN-M3-0601` entry's exhaustive claim was
  too wide; it and that Commit Log row stand as historical statements, corrected by this answer.
- Validation: the census matches all 23 existing IDs/files and resolves every finding reference; current operators
  list 932 planning candidates (656 / 98 / 178); standalone membership probes pass eight narrowing assertions and
  two controls, untracked evidence. Book gates and final staged counts/doctrines run before commit. Two stale parent
  review-status index rows corrected. PROGRAM.69's newly reported primary-link defect is owned and scheduled next.

## archogen — sealed-evaluation custody review closes at zero defects

`ARCHOGEN-PROGRAM-0613` (leaf `PROGRAM.76`), `2026-10-11`.

- Independent round 15 finds zero defects or actionable arm gaps on the committed reader. Its 157 synthetic probes
  comprise 151 repaired-reader checks and six old-reader controls; the runner and report are untracked evidence.
  The tracked suite remains 143 passing arms, with the filesystem-specific skip explicit.
- Validation: the real-tree guard, shell syntax and book gates pass; the final staged hook runs doctrines. No reader
  or Rust changes. The book clarifies independent source/copy endings and disk/index judgments; the task records
  closure and the live pointers remove this leaf from active work.

## archogen — a staged read failure cannot silently substitute disk evidence

`ARCHOGEN-PROGRAM-0612` (leaf `PROGRAM.69`), `2026-10-11`.

- REVIEW-COUNTS falls back to disk only after a successful index census and path-byte comparison prove absence. A present
  unreadable staged entry or failed proof refuses with a reason; the shared reader covers index, history and record.
- Validation: ten new controls, including inherited pathspec settings and complementary present/absent bracketed
  filenames: 83 pass / 8 fail on the old reader, then 91 / 0; a glob-comparison mutation fails both filename
  controls. Staged count/history gates, book checks, stated order and task acceptance pass; final doctrines run in
  the commit hook. Record, history, book and resume pointers agree on round 32's answer and round 33 next.

## archogen — the doctrine registry's explanatory header stays literal

`ARCHOGEN-PROGRAM-0611` (leaf `PROGRAM.82`), `2026-10-11`.

- Replaced backticks and table pipes in the REVIEW-COUNTS description with words. Bash formerly tried to execute the
  header while initializing the registry, emitting a syntax error and dropping the example; parse-only validation
  passed. Check commands and verdict rules are unchanged, as are the held doctrine driver and hook.
- Validation: the exact array's initialization before/after gives diagnostics then empty stderr; all 29 entries retain
  three fields and the repaired description is complete. Shell syntax and the held-file diff pass; final doctrines
  run in the commit hook. Book, task logs and resume/frontier pointers agree.

## archogen — seventh daily artifact cleanup, with cold-cache verification

`ARCHOGEN-PROGRAM-0610` (leaf `PROGRAM.81`), `2026-10-11`.

- Removed only the main checkout's hosted and wasm incremental caches: 2 022 861 425 logical bytes in 18 540 files,
  including 1 245 `.bin`. Cited review evidence, nested read-only Git workspaces, tools and dependency products stay.
  The latest-only record and book explain the cleanup boundary and verification.
- Validation: OS census clear before removal; both exact paths gone afterward; focused tier 3 passed / 0 failed and
  wasm build exit 0 with those caches absent; book gates pass. Dependencies were retained, so this is cold-cache
  verification rather than a full dependency rebuild; the final staged tree must have its focused stamp at commit.

## archogen — sealed source lines recognize LF, CRLF and CR before eligibility

`ARCHOGEN-PROGRAM-0609` (leaf `PROGRAM.76`), `2026-10-11`.

- The shared reader now splits source cases at LF, CRLF or CR before measuring each complete line. A CR-only
  case formerly became one case-sized pattern, missing a quote of a single eligible line on disk and in the index.
  The original line thresholds, case-sensitive word comparison and partial-quote limits remain unchanged.
- Validation: six new source-ending controls; the two CR-only regressions failed first (141/2 of 143 arms).
  All 143 repaired arms pass; the filesystem case-distinction arm is explicitly skipped on this host. The real
  sealed-tree guard, syntax, review counts, book checks and doctrines pass.

## archogen — an unlisted review names its design record by the first link

`ARCHOGEN-PROGRAM-0608` (leaf `PROGRAM.69`), `2026-10-11`.

- The open-review list rule now uses the same first-link helper for the Design record cell as for the history
  cell. Secondary explanatory links no longer falsely require counting or produce missing-record refusals. The
  primary record's round table still requires listing; counted pairs remain their own declared authority.
- Validation: two new regression arms failed first, then all 81 count-check arms passed; shell syntax, staged real
  gates, book checks and doctrines. The prior first-link claim is marked; book and resume pointers agree.

## archogen — handoff refuses failed or missing OS census reads

`ARCHOGEN-PROGRAM-0607` (leaf `PROGRAM.80`), `2026-10-11`.

- The project handoff wrapper observes the native census's actual process snapshot and open-handle reads, in each
  sample. Denied, failed or missing reads, and an empty process snapshot, exit 2 with an unknown outcome and no
  OK verdict. A process that exits during an ancestry lookup is allowed; persistent-job detection is preserved.
- The native scaffold file is unchanged. The bootstrap, toolbox and book use the guarded wrapper; the book states
  the sampling limits. Private receipts stay on the repository volume and are removed on exit.
- Validation: six regression arms failed against the old core; all nine arms pass after repair. A real restricted
  run refuses, a real OS-access run passes; zero receipt directories remain; syntax and book gates pass. No Rust
  source changed. The next review answer, PROGRAM.69 round 31, is owned in the task tree.

## archogen — escaped summary-cell pipes cannot hide later counts

`ARCHOGEN-PROGRAM-0606` (leaf `PROGRAM.69`), `2026-10-11`.

- One cell reader now serves index, summary and findings tables. An escaped pipe stays inside a summary's defects
  cell, so every later stated count remains visible. Eight false-count arms failed first; all 78 arms now pass.
- The previous commit's confirmatory runner is complete: 47 of 49 suites passed restricted; the two process suites
  passed with OS access through the same runner (2/2 handoff and 39/39 spine arms). Those two runs cover all 49
  suites before this parser repair; the parser's focused 78-arm run is current. No Rust source changed.
- The denied OS reads exposed a separate pre-existing defect: the native handoff census and its wrapper say OK
  without a process snapshot. Owned as high-priority `PROGRAM.80`, the wrapper repair next, the held native file
  untouched. Earlier repeated-count claims are marked; the book and resume pointers agree.

## archogen — review counts judge repeated statements, and history reads must prove absence (narrowed by round 30)

`ARCHOGEN-PROGRAM-0605` (leaf `PROGRAM.69`), `2026-10-11`.

- The reviews index's four-column table accepts indentation and optional edge pipes, keeps escaped pipes inside
  their cell, and refuses malformed rows, missing or duplicate headers and missing separators. Every stated count
  in either summary shape is judged, so a correct first count cannot hide a false second one.
- A historical task-history index blob that cannot be read is refused unless a separate successful Git tree read
  proves that path absent at the commit. A real historical deletion remains valid. The leaf's stale Status is current.
- Validation: new regressions first failed (13 count-check arms, two history-check arms), then all 69 and 124 arms
  passed; syntax, real gates, census, book gates and doctrines green. The broader self-test runner was pending at
  commit; its result is recorded by `ARCHOGEN-PROGRAM-0606`. The earlier claims are marked in the book and records.
- Correction by round 30: an escaped pipe in a summary cell still hid later stated counts; the shared cell reader
  in `ARCHOGEN-PROGRAM-0606` repairs that case.

## archogen — the sealed-set guard recognizes comment and string wraps, with an explicit quote boundary

`ARCHOGEN-PROGRAM-0604` (leaf `PROGRAM.76`), `2026-10-11`.

- Whole cases and complete long source lines are matched as case-sensitive words after prose markers and word-edge
  punctuation are removed. Doc comments, Lisp comments, no-space quote markers, continued strings and adjacent
  string literals no longer hide those patterns, on disk or in the index.
- A partial quote across source lines can remain unfound even when longer than either line; encodings that change
  word boundaries and rewordings are also stated limits. The draft's arbitrary word windows are not adopted: an
  overlap alone cannot establish which document quoted which. No case text was displayed or exposure inferred.
- The error paths have separate arms for the disk and index searches; recursion arms put a synthetic case's name
  inside an active registered submodule. The top-level `.git` case distinction has a conditional arm, explicitly
  skipped on a case-insensitive filesystem.
- Validation: new arms on the previous reader, 127 passed / 10 failed; revised reader, 137 passed / 0 failed; real-tree
  check; shell syntax; book anchors and glossary; review counts; doctrine gate. No Rust source changed.

## archogen — the sealed cases' guard: a review's findings and the answer's direction, recorded at a hand-off

`ARCHOGEN-PROGRAM-0603` (leaf `PROGRAM.76`), `2026-10-11`.

- A review found the guard missing a quote wrapped in some comment and string styles, and a quote running across two
  of a case's lines. The answer under way compares words, not lines; its first run found three tracked documents
  sharing a run of eight words or more with a case, which the next session settles before the guard changes. The
  guard itself is unchanged.
- Validation: the guard on the tree as committed; the no-background check; the focused tier on the staged tree.

## archogen — the review-count check reads index rows and says which reviews must be listed (narrowed by round 29)

`ARCHOGEN-PROGRAM-0602` (leaf `PROGRAM.69`), `2026-10-11`.

- A review found the count check's rule that open reviews be listed passing a review whose index row was written a
  little differently, and the texts stating the rule wider than it was. The check now reads every row of the index by
  its cells and links, and refuses one it cannot read; every open review whose record keeps a table of its rounds, of
  any shape, must be listed from its first round, and a closed one must leave the list. A round's findings table must
  be the first table after its line, with no row of dashes or row with no kind, and every count a summary row states
  is judged, whichever shape the record keeps.
- Validation: 51 arms; thirteen deliberate breaks of the new rules and round 27's eleven, each caught by its own arm,
  by untracked runners, so not durable; the sealing tool's 121 arms; every self-test; the focused tier on the staged
  tree.

- Correction by review round 29: indented and malformed index rows were skipped, and only the first stated count
  of a kind was judged. `ARCHOGEN-PROGRAM-0605` repairs both.

## archogen — the design for the generated-sources rules' tests holds every recorded defect exactly

`ARCHOGEN-M3-0601` (leaf `M3.6.9`), `2026-10-11`.

- The design's first review found its new kinds of mutation reaching 7 of the 23 one-way defects on record, and its
  sweep's copy placed outside the repository by an exception only the director grants. Now each defect on record is
  itself a catalogue entry the rules' own tests must catch, and the sweep, with more kinds of mutation, carries the
  check past the record — how far, measured defect by defect when it is built. The copy stays inside the repository:
  the tests admit, in the tests alone, the configuration of the repository the copy was made from. The tests, the
  code swept and the commands are named in full.
- Validation: the parent's tests timed, 88 in 12.5 s; the record's, the reviews' and the book's gates; the focused
  tier on the staged tree.

## archogen — the accepted designs' folder has room again

`ARCHOGEN-PROGRAM-0600` (leaf `PROGRAM.79`), `2026-10-11`.

- The folder of accepted designs had under 4 KB left, and the answer to the open adequacy design's first review needs
  7 KB. Three trust designs have moved in since its last raise five days ago. Its ceiling rises from 384 to 512 KiB,
  measured and recorded in the decision that governs it; compacting it was weighed and refused, since nothing there is
  history.
- Validation: the routes check and its self-test; the folder measured at this commit's parent and at the last raise;
  the focused tier on the staged tree.

## archogen — the reviews the count check judges are listed in the reviews index

`ARCHOGEN-PROGRAM-0599` (leaf `PROGRAM.69`), `2026-10-11`.

- The count check's list of reviews was written in its script, so a design review in any other part of the project
  could join it only by a change to that script, owned by another leaf. The list now sits in the reviews index, which
  every review's answer already edits, under its own heading; a missing heading, a line there that is no entry, and a
  review listed twice stop the commit.
- Validation: 38 arms; four deliberate breaks of the list's rules, each caught by its own arm, and round 27's twelve,
  by untracked runners, so not durable; every self-test; the focused tier on the staged tree.

## archogen — the review-count check judges every round of the reviews it lists, and every open one is listed (not so: round 28)

`ARCHOGEN-PROGRAM-0598` (leaf `PROGRAM.69`), `2026-10-11`.

- A review found the count check still passing a review it could not judge: a round whose findings table had another
  header was counted as having none, and its counts went unread. The check is now strict: every round of a listed
  review must keep a findings table in the one form, or the commit stops; a malformed table — no separator, a row with
  no id, a row of dashes — stops it too (not so, review round 28 found: a row of dashes in a findings table was
  counted); and an open review whose record keeps a count per round must be on the list from its first round (not so,
  review round 28 found: one whose index row's link had no backticks was passed). The five texts the review found
  still claiming what earlier reviews refuted, and three more like them found while answering, now say so.
- Validation: 35 arms; twelve deliberate breaks, each caught by its own arm, by an untracked runner, so not durable;
  the sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — a design for holding the generated-sources rules to their tests, by mutation

`ARCHOGEN-M3-0597` (leaf `M3.6.9`), `2026-10-10`.

- The generated-sources rules are held by the tests of their instrument only at the instances those tests build, and
  every attempt to name where else they must be held left a place out. The design decided here names none: every
  mutant the project's mutation sweep makes of the judgment code must be caught by those tests or shown harmless. It
  adds the three kinds of mutation the defects found so far were (not so, review round 1 found: they made 7 of the
  23), and runs the sweep's copy beside the repository, where the trust gate's own rules let its tests run (withdrawn
  by review round 1: the copy stays under `target/`). Its review is open.
- Validation: the record's and the book's gates; the focused tier on the staged tree.

## archogen — the sealed cases' guard reads every file as prose, staged or on disk

`ARCHOGEN-PROGRAM-0596` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found a quoted line of a case missed when the quote was re-wrapped across two lines — as every document
  here is wrapped — and a case missed inside a larger file, or with its line endings changed. The guard now reads
  every file it walks and every staged one itself, as prose: line breaks, indents and list or comment markers set
  aside. It finds a case's whole text, and a long line of one, however wrapped. On macOS it no longer takes a file
  as ignored by a case-blind match that Linux would not make.
- Validation: the self-test's arms; five deliberate breaks, each caught by its own arms, by an untracked runner, so
  not durable; the check on the tree in about two seconds; every self-test; the focused tier on the staged tree.

## archogen — the generated-sources rules' one-way defects are in the mutation catalogue

`ARCHOGEN-M3-0594` (leaf `M3.6.8`), `2026-10-10`.

- A review of the generated-sources record found, round after round, rules held only one way — the first generator
  judged and not the last, one reader and not another — each shown by a throwaway script. The 23 such mutations are
  now entries of the project's mutation catalogue, each run by the harness and each caught by the route tests, so the
  record of them outlives the scripts.
- Validation: the 23 entries through the harness, each killed; the catalogue's own tests; the focused tier on the
  staged tree.

## archogen — the generated-sources record's review closes, and generated sources are done

`ARCHOGEN-M3-0591` (leaf `M3.6.6.4`), `2026-10-10`.

- The record deciding which generated-source shapes stay refused, and the routes around them, passed its twenty-third
  review round with no defect. With it, the work on committed generated sources — the design, the instrument, the
  chapter and these decisions — is done. The leaf for the parent's own tests stays open, with its evidence in the
  review history.
- Validation: sixteen route tests; the crate's 183 tests; the review-count check; the book's gates; the focused tier
  on the staged tree.

## archogen — the review-count check judges a list of reviews, and refuses any it cannot judge (not so: round 27)

`ARCHOGEN-PROGRAM-0590` (leaf `PROGRAM.69`), `2026-10-10`.

- Three reviews in a row found the count check deriving which reviews to read and leaving one unread or unnamed. It
  no longer derives anything: its script lists the reviews it judges — the open ones whose records keep a count per
  round — and any listed review it cannot judge, a file missing or a table empty, stops the commit (not so, review
  round 27 found: a round whose table had another header was passed unjudged). A review leaves the list when it
  closes. The texts that said more are marked as narrowed (not so, review round 27 found: five were not).
- Validation: 25 arms; eight deliberate breaks, each caught by its own arm, by an untracked runner, so not durable;
  the sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — the sealed cases' guard finds quotes in what it reads, and decides alone what is ignored

`ARCHOGEN-PROGRAM-0588` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found two more settings that changed what the guard saw: one switched off its search of the disk without a
  word, and personal ignore files hid paths from it. The guard's own walk of the files now finds a case's long line
  as well as a whole copy, so no setting can switch that off; what is ignored is decided by the project's own ignore
  files alone; and any search that fails stops the commit. Its documents now say exactly what it finds — a whole copy,
  a long line, a name — in exactly which files (narrowed by review round 12: a name only in a tracked file).
- Validation: the self-test's arms; nine deliberate breaks, one per hole, each caught by its own arms, by an untracked
  runner, so not durable; the check on the tree in under two seconds; every self-test; the focused tier on the staged
  tree.

## archogen — one correction in the generated-sources review history set right

`ARCHOGEN-M3-0587` (leaf `M3.6.6.4`), `2026-10-10`.

- The review history's answer to a remark of round 21 misattributed which rounds had found what; round 22's entry
  corrects it, round 21's standing as written. One sentence of the record now carries the qualifier the rest of it
  uses: the parent's tests hold its rules at the instances they build.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

