- ID: `PROGRAM.76`
  Status: `done` — closed `2026-10-11` after independent round 15 found zero defects or actionable arm gaps at
  `0b3a056`. The custody change and all fourteen repair rounds are committed; the checker is unchanged at closure.
  Goal: the sealed evaluation set guarded against being read, not only against being edited or named, so no search
  of the working tree made for other work can reach a case before `M6.5`, nor, in a working tree that carries the
  mark, a diff of a commit that holds one show it.
  Reproduce / issue: `M3.3`'s survey, a delegated read-only search told to sweep the whole tree, opened one sealed
  case; a search in the working session printed one of its lines. The seal's check let it be: `git ls-files docs/evaluation/frozen` at `4caef04` → the manifest and five
  cases, the doctrine gate green.
  Direction: the director chose, of three, that the digests stay and the text leaves: custody by the sealing commit,
  `b05fadb`, which holds each case as a blob; the manifest its commitment; `--restore` the reveal, refused while
  sealed; the exposure recorded in the manifest, the one place a case may be named, and counted apart at `M6.5`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`), round 14 answer:**
  - [x] **REPRODUCE / ISSUE** — six new source-ending controls on the prior reader, `bash
    scripts/check_frozen_evaluation.sh --self-test` → `141 pass / 2 fail (143 arms)`
    (`target/p76/r14-before.log`): LF and CRLF controls pass; a complete line from a CR-only source is missed on
    disk and in the index independently. The reviewer separately reproduced the same source-side distinction.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_frozen_evaluation.sh` derived eligible patterns with
    `c.split(b"\n")`, then called `norm`, which already recognized CR. CR-only physical lines were joined before
    eligibility, so the purported single-line pattern became the whole case. The two named failures isolate that
    source-side split; the copied line and its word boundaries are unchanged. `bash
    scripts/check_frozen_evaluation.sh --self-test` → `141 pass / 2 fail (143 arms)` on the prior reader.
  - [x] **FIX** — `physical_lines` recognizes LF, CRLF and CR and is shared by prose normalization and source
    pattern extraction. Per-line normalization, the 60-character/8-word thresholds, whole-case comparison and
    case-sensitive word matching are unchanged. The six tracked arms separately own disk/index consumers.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_frozen_evaluation.sh --self-test` → `143 pass / 0 fail
    (143 arms)` (`target/p76/r14-after.log`), the two former omissions now refused. The filesystem-conditional
    case-distinction arm remains explicitly skipped on this host; the suite does not claim that branch ran here.
  - [x] **NO REGRESSION** — `bash -n scripts/check_frozen_evaluation.sh`, the real-tree guard → exit 0; review
    counts → OK, 2 reviews / 32 rounds; book anchors, glossary and index → OK; doctrines → all green
    (`target/p76/r14-doctrines-prestage.log`), repeated on the final staged tree by the commit hook. The first
    pre-stage pass refused undefined book abbreviations; the passage now spells out the three line endings.
    The first staged hook required the reproduction output inside this root-cause bullet; it is now included.
    No Rust source changes; `COMMIT.md` step 2 requires the focused tier when Rust changes.
  - [x] **LOCKSTEP** — the checker, evaluation contract, book, leaf and logs, changelog and resume/frontier
    snapshots. README, development notes and product milestone states unchanged.
  Round 14, `2026-10-11`, of `c4c6120`: 1 defect, the pre-existing source-side CR split. Its read-only report is
  `target/review-p76-r14/report.txt` (untracked); the reproduction and repair above are durable tracked arms. No
  actual sealed case text was displayed. Round 13's wrapper repair and explicit partial-quote limits still apply.
  Round 15, `2026-10-11`, of `0b3a056`: zero defects or actionable arm gaps; seven remarks affirm the repair,
  retained limits, tracked controls, evidence and review boundaries. The report is
  `target/review-p76-r15/report.txt`; its synthetic runner, fixtures and outputs are untracked, not durable coverage.
  `python3 target/review-p76-r15/probes.py` → `157 pass / 0 fail`: 151 repaired-reader checks plus six controls
  expecting the prior CR-only miss, not 157 repaired-reader successes. The independent matrix covers source/copy
  line endings, final-newline presence, disk/index consumers, original eligibility thresholds and retained wrappers.
  The reviewer recounted the parent logs (`141 / 2` then `143 / 0`) and verified pinned/current checker identity;
  the filesystem case-distinction skip remains explicit. No actual sealed case text was read. All review jobs ended.
  Closure verification: real-tree guard, shell syntax and book gates pass; the final staged doctrine hook is required.
  No Rust or checker change, so no redundant broad suite or focused build is claimed for this documentation slice.
  Round 13's findings (2 defects, 4 arm gaps, 8 remarks) survive in `target/p76/r13report.txt` (untracked) and the
  answer here. D1 fixed by word comparison and widened markers; D2 by accurately bounding the quote patterns,
  instead of adopting arbitrary word windows. AG1 has disk/index wrapper arms; AG2 has a filesystem-conditional
  arm, its skip named; AG3 has each search fail alone; AG4 has a name in the submodule the search must not enter.
  R1–R4's stale comments and setup removed; R5's altered word boundaries and R6's encoded copies are stated limits;
  R7's custody row agrees with the reader; R8's resume pointer was already corrected at this answer's start.
  Review round 1, `2026-10-10`, of `2f6f331`, by a read-only context fenced from the cases' text: 5 defects, 2 arm
  gaps. The change's own diff showed the cases' text as deleted lines, and the check sought a case at its own path
  alone; answered by mechanism — the sealed paths marked `-diff`, so no diff, log patch or `git grep` of a commit that
  holds them prints a line (measured on `2f6f331`: *"case text lines shown … 0"*); custody by content, a case's whole
  text anywhere in the index or the unignored working tree and a long line of it quoted in any file; completeness at
  any depth; the manifest pinned to the sealing commit's own, that commit an ancestor of `HEAD`; a malformed entry
  refused; arms for the blob check once unsealed and for a restore with no sealing commit while the index holds a
  case — and the texts narrowed to it: "no search of the working tree", the honest limit naming a blob shown by path,
  a diff forced to text, checkouts before `2f6f331` and other clones; the exposure a rule, not a proof; the hint at the
  case's content dropped from the exposure record, the script and `M3.3`'s survey line; `CLAUDE.md`'s bootstrap rule;
  the doctrine registry's line. The 365 copies of the set that local clones and tool-made trees under `target/` held,
  untracked, were deleted, `find target <scratchpad> -path '*/docs/evaluation/frozen/ev-0*' -not -path '*/.git/*'` →
  none left. Evidence: `--self-test` → *"40 pass / 0 fail (40 arms)"*; beside `2f6f331`'s core → *"27 pass / 13 fail
  (40 arms)"*; `python3 target/p76/mutate2.py` → *"11 of 11 killed"* — both runners untracked, so not durable.
  Verification: `2026-10-10` — the Verification Log's rows
  Review round 2, `2026-10-10`, of `6d61f65`: 5 defects, 1 arm gap. The check could print a case's line — echoing a
  manifest line that is no entry, and under a shell trace — and missed a copy staged and gone from disk, in the
  manifest, under a quoted name, in an unreadable file or a nested repository; the diff guard holds only in a working
  tree carrying the mark. Answered: a bad line named by its number; `set +x`, no case's line ever a command's argument,
  each held by an arm reading the script; the quote search over the index and the disk, the manifest included; paths
  read NUL-separated, each untracked or modified file hashed on its own, one that cannot be read refused, a nested
  repository refused; the attribute checked staged and on disk; arms for a dangling link at a case's path and a case
  that is a link once unsealed; the texts given `6d61f65` as the boundary and every act that still reaches a case
  (`git blame`, an external diff driver, git without this working tree). The 47 local clones whose index still held the
  set, every one under `target/` or the session scratchpad, were deleted — none left, by `git ls-files` in each. Evidence:
  `--self-test` → *"53 pass / 0 fail (53 arms)"*; beside `6d61f65`'s core → *"44 pass / 9 fail (53 arms)"*; `python3
  target/p76/mutate3.py` → *"10 of 10 killed"* — both runners untracked, so not durable; the real check in 0.9 s.
  Review round 3, `2026-10-10`, of `00e08cc`: 5 defects, 2 arm gaps. The check still echoed the seal's and the sealing
  commit's values, so a case's line there printed; a folder git could not open, a tracked file under a folder that could
  not be searched, and a registered submodule passed unseen; three texts stayed unconditional; the tree's changelog
  said the leaf done. Answered by method: no value read from the manifest is echoed, an unmatched exposure named by its
  line's number, and one arm seeds a case's line into eight places of the manifest; git's own warnings while listing,
  and a folder that cannot be searched, refused; a submodule, a link to a folder outside and a folder an untracked
  ignore file hides named in the honest limit; the trace guard held by a run of a copy with tracing forced on and by
  `set +x`'s place before any code; the book, the doctrine row and `CLAUDE.md` made conditional, `-a` and a viewer
  that diffs blobs itself named; the reopening noted. The four review clones at commits older than `6d61f65` deleted —
  none left, `git merge-base --is-ancestor 6d61f65 HEAD` in each. Evidence: `--self-test` → *"56 pass / 0 fail (56
  arms)"*; beside `00e08cc`'s core → *"52 pass / 4 fail (56 arms)"*; `python3 target/p76/mutate4.py` → *"7 of 7
  killed"* — both runners untracked, so not durable.
  Review round 4, `2026-10-10`, of `a9ec254`: 4 defects. "No manifest value is echoed" was false of names and a commit
  id; the seal's, the sealing commit's and an exposure's values sat in shell variables a forced trace printed; a
  nested repository staged as a gitlink passed; the README said the box named a viewer it did not. Answered: each
  field's value checked inside its pipeline before any variable holds it, an exposure's name matched in awk; the
  forced-trace arm run over nine places; a gitlink `.gitmodules` does not register refused, a registered one passing;
  the texts say what is printed — paths, line numbers, names, a commit id, git's warnings — and the box and
  `CLAUDE.md` name the viewer, an archive and a revert; the remarks' wordings. Evidence: `--self-test` → *"58 pass / 0
  fail (58 arms)"*; beside `a9ec254`'s core → *"56 pass / 2 fail (58 arms)"*; `python3 target/p76/mutate5.py` → *"4
  of 4 killed"* — both runners untracked, so not durable.
  Review round 5, `2026-10-10`, of `04e306c`: 5 defects. Paths were said to be tokens with no space; git's warnings
  while listing modified files made a malformed attribute line a refusal, and the verdict hang on a racy file; the
  scratch held every manifest line; the README's limit left out the archive and the revert; a stray `Commit:` line.
  Answered: paths printed as their files are named, the tokens named apart; git's warnings taken from listing
  untracked files alone; the entries read straight from the manifest, no other line written; the acts that reach a
  case put as two kinds — writing a commit older than `6d61f65` to disk, and handing a sealed blob to a reader or a
  tool — in the box, the limit, the script and `CLAUDE.md`, `git difftool`, a restore and a reset among them; the stray
  line removed; registration read from the staged `.gitmodules`, a path with a space whole; the forced-trace arm also
  requiring each place's refusal. Evidence: `--self-test` → *"63 pass / 0 fail (63 arms)"*; beside `04e306c`'s core →
  *"58 pass / 5 fail (63 arms)"*; `python3 target/p76/mutate6.py` → *"4 of 4 killed"* — both runners untracked, so
  not durable.
  Review round 6, `2026-10-10`, of `fa0ec3f`: 4 defects. Two legs printed paths C-quoted; the forced-trace arm took
  `--restore`'s refusal for each place's; the kept-scratch arm left its output unchecked; and the published default
  branch, `origin/main` at `7ac8b8b`, still holds the set, which no text said. Answered: both legs read paths
  NUL-separated, an arm each; the traced arm requires each place's own refusal from the normal run alone; the kept
  copy's output checked; the honest limit, the box's first kind narrowed to a commit that holds the set, the book and
  `CLAUDE.md` say the published branch reaches a case until it is pushed past `6d61f65`, web searches and fetches of
  this repository fenced until then; transcripts and editor histories made before custody moved named, theirs the
  director's. Evidence: `--self-test` → *"65 pass / 0 fail (65 arms)"*; beside `fa0ec3f`'s core → *"63 pass / 2 fail
  (65 arms)"*; `python3 target/p76/mutate7.py` → *"4 of 4 killed"* — both runners untracked, so not durable.
  The published branch: on `2026-10-10`, on the director's word (*"Push now"*, the one early push, *"once done you
  need to return honoring the define push cadence"*), `main` was pushed to `a543d10`, past `6d61f65` — `git ls-tree
  --name-only origin/main docs/evaluation/frozen/` → the manifest alone.
  Review round 7, `2026-10-10`, of `ff0a180`: 2 defects, 1 arm gap. Round 6's first kind, "a commit that holds the
  set", let out `2f6f331`, whose diff shows the cases, and `421ce47`, at which a diff of `2f6f331` shows them (R8 R1);
  three texts still said reading one takes an act or
  a deliberate read; four legs' "as named" had no arm. Answered: the first kind "a commit older than `6d61f65` whose
  history holds the set", in the box, the script and `CLAUDE.md`; the published branch said moved past `6d61f65`
  since `a543d10`, older copies — forks, mirrors, caches, archives — named and web searches kept fenced; transcripts
  and saved scrollback bounded at `6d61f65`; four arms naming a non-ASCII path in each leg, with git's path quoting
  forced on in the self-test. Evidence: `--self-test` → *"69 pass / 0 fail (69 arms)"*; beside `ff0a180`'s core →
  *"69 pass / 0 fail"*, the four arms holding rules already kept; `python3 target/p76/mutate8.py` → *"4 of 4 killed"*
  — both runners untracked, so not durable.
  Review round 8, `2026-10-10`, of `8f4afe2`: 1 defect, 2 arm gaps, 5 remarks, and residual paths. The README still
  named two routes to a case where the honest limit names more; the modified-file listing and the on-disk half of the
  `-diff` mark had no arm, a mutant reading each plain passing every arm; a caller's `git -c` could undo the
  self-test's forced quoting. Answered: the README names the honest limit's copies; the limit names a harness's
  persistent memory, a model or an index made from the branch while it held the set — from `a3c0cbd`, pushed
  `2026-10-06`, to `a543d10` — `M6.5` recording the models used, and a short quote committed outside the folder before
  the custody check, no whole copy elsewhere (`git log --all --find-object` per case blob → `b05fadb` and `2f6f331`);
  two arms; `GIT_CONFIG_PARAMETERS` unset in the self-test; the changelog's heading and round-6 entry exact; the
  use cases chapter's limit and `M6.5`'s unsealing step name the models. Evidence:
  `--self-test` → *"71 pass / 0 fail (71 arms)"*; `python3 target/p76/mutate_r8.py` → each mutant *"70 pass / 1
  fail (71 arms)"*, its own arm the one failing, *"2 of 2 killed"* — the runner untracked, so not durable.
  Review round 9, `2026-10-10`, of `0658beb`: 7 defects, 6 remarks. Five rules failed open: an entry named `.git`
  hid a copy or a quote; a file marked assume-unchanged or skip-worktree hid a whole copy; a mark from
  `.git/info/attributes` or `core.attributesFile` passed for one no commit carries; the names leg read the disk alone;
  the check's own script was exempt from it. The README's list of copies still left three out, and the changelog said
  two commits hold a case where 539 do. Answered: an entry named `.git` in any case, but the top level's, outside
  ignored folders and registered submodules, refused, found by a walk that asks git what it ignores; a flagged file
  hashed; the mark also judged by the staged and the on-disk `.gitattributes` alone, read in an empty repository made in
  the check's own scratch, which its listings skip; the names searched staged and on disk, the script included; the README's list whole (not so: rounds 10 and 11 found it short); "a file whose
  content is a case's text, byte for byte"; the quote's bound `6d61f65`; the window bounded by GitHub's own record of
  pushes, `gh api repos/{owner}/{repo}/activity` → the branch created at `32e6b14` on `2026-09-13`, pushed next at
  `a3c0cbd`; an arm holding the self-test's quoting over a caller's `git -c`. Evidence: `--self-test` → *"81 pass / 0
  fail (81 arms)"*; `python3 target/p76/mutate_r9.py` → six mutants, each putting a hole back, each failing its own
  arm, *"6 of 6 killed"* — the runner untracked, so not durable; the check on the tree → exit 0 in 1.1 s.
  Review round 10, `2026-10-10`, of `c4523f5`: 9 defects, 1 arm gap, 4 remarks. Round 9's rules held by their arms, but
  the check still leaned on git's view of the disk, which settings blind — a file-system monitor or the laxest stat
  settings hid a whole copy, a clean filter changed what was hashed, a template marked the empty repository — and on
  details: a path `.gitmodules` named was pruned with no gitlink there, the names leg read a flagged file's index copy,
  the walk's failure passed, a comment and two texts overclaimed. Answered by method: git runs with the settings every
  leg rests on forced; one walk reads the working tree's bytes itself — every file of a case's size hashed raw as git
  hashes a blob, whatever git says of it — pruning only ignored folders (and ignored files, round 11 found), registered submodules held as gitlinks, links
  and the check's own scratch, and a walk that cannot finish refuses; the empty repository takes no template; the names
  leg reads flagged files from the disk and tracked paths; the README says what the check holds against — accident —
  and what not — concealment on purpose — and its list of copies names what the walk does not reach, the repository's
  own `.git` among them. Remark 4, the use-cases chapter and the doctrine row carrying less: declined, each a summary
  pointing to the README, the row under its file's byte ceiling. Evidence: `--self-test` → *"92 pass / 0 fail (92
  arms)"*; `python3 target/p76/mutate_r10.py` → eight mutants, each putting a hole back — the old copy leg with the
  settings unforced among them — each failing its own arms, *"8 of 8 killed"* — the runner untracked, so not durable;
  the check on the tree → exit 0 in 1.3 s.
  Review round 11, `2026-10-10`, of `5bb19ec`: 7 defects, 1 arm gap, 4 remarks. Round 10's holes stayed shut, but two
  settings still changed what the check read — submodule recursion stopped `git grep --untracked` silently, and a
  per-user or local exclude file widened what was skipped — and texts overclaimed: "ignored folders" where ignored files
  were skipped too, searches said to find a case "under any term", the book's and the doctrine row's claims unbounded,
  two earlier claims unmarked, a comment. Answered by method: the walk now also finds a case's long line in every file
  it reads, so no `git grep` of the disk remains for a quote (round 12: the names leg keeps one); what is ignored is what the `.gitignore` files in the tree ignore,
  listed once; git runs with submodule recursion off; any `git grep` that cannot run refuses; a long line is matched
  stripped of a list, quote, heading or table marker; python runs isolated; every text bounded to what the check walks
  and finds — a whole copy, a long line, a name. Evidence: `--self-test` → *"105 pass / 0 fail (105 arms)"*;
  `python3 target/p76/mutate_r11.py` → nine mutants, one per hole, *"9 of 9 killed"* — the runner untracked, so not
  durable; the check on the tree → exit 0 in 1.7 s.
  Review round 12, `2026-10-10`, of `6bb44ee`: 5 defects, 2 arm gaps, 5 remarks. A long line quoted but wrapped across
  two lines went unfound, and a case with no long line went unfound inside a larger file, its newline dropped or its
  line endings changed — "whole text" true only of a byte-for-byte copy; a case-blind ignore match hid a file on
  macOS; texts claimed a name in any file it walks, no `git grep` of the disk, and a shorter quote refused. Answered by
  method: one reader, the check's own, for the disk and the index alike — every staged blob read raw, no `git grep`
  for a quote — reading as prose: line endings, indents and leading markers aside, white space collapsed; refusing a
  case's whole text so read, and a long line so read, wrapped or not; the ignored listing matched as spelled; the
  top level's own `.git` alone exempt; every text bounded — a whole text or a long line in the files it walks or
  stages, a name in a tracked file. Evidence: `--self-test` → *"113 pass / 0 fail (113 arms)"*; `python3 target/p76/mutate_r12.py` →
  five mutants, each putting a hole back, *"5 of 5 killed"* — the runner untracked, so not durable; the check on the
  tree → exit 0 in 1.9 s.
  Commit: `ARCHOGEN-PROGRAM-0542 (leaf PROGRAM.76)`; `ARCHOGEN-PROGRAM-0544 (leaf PROGRAM.76)`, review round 1;
  `ARCHOGEN-PROGRAM-0547 (leaf PROGRAM.76)`, review round 2; `ARCHOGEN-PROGRAM-0550 (leaf PROGRAM.76)`, round 3;
  `ARCHOGEN-PROGRAM-0554 (leaf PROGRAM.76)`, round 4; `ARCHOGEN-PROGRAM-0558 (leaf PROGRAM.76)`, round 5;
  `ARCHOGEN-PROGRAM-0562 (leaf PROGRAM.76)`, round 6; `ARCHOGEN-PROGRAM-0565 (leaf PROGRAM.76)`, round 7;
  `ARCHOGEN-PROGRAM-0572 (leaf PROGRAM.76)`, round 8; `ARCHOGEN-PROGRAM-0579 (leaf PROGRAM.76)`, round 9;
  `ARCHOGEN-PROGRAM-0584 (leaf PROGRAM.76)`, round 10;
  `ARCHOGEN-PROGRAM-0588 (leaf PROGRAM.76)`, round 11;
  `ARCHOGEN-PROGRAM-0596 (leaf PROGRAM.76)`, round 12;
  `ARCHOGEN-PROGRAM-0604 (leaf PROGRAM.76)`, round 13;
  `ARCHOGEN-PROGRAM-0609 (leaf PROGRAM.76)`, round 14;
  `ARCHOGEN-PROGRAM-0613 (leaf PROGRAM.76)`, round 15 and closure
