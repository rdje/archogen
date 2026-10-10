# Sealing closed subtrees: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.32.4` (`docs/tasks/PROGRAM.md`). This is the review history of
  [[decision_task-tree-sealing]], kept apart from it as `docs/reviews/INDEX.md` describes.
  Reopened `2026-10-10` by leaf `PROGRAM.69` for the record's amendment, from round 2.

## The fact / decision

**Round 1**, `2026-09-30`, of the tool, its gate and the first seal (`940baf1`), by a read-only context that had not
written them. It re-derived both trees with its own code:
- `M1` equals its parent byte for byte;
- `PROGRAM` equals its parent apart from the commit's own closure of `PROGRAM.32.3`;
- all 69 rows matched `shasum`, `wc` and a leaf count.

It confirmed that exactly the closed subtrees were sealed, 62 and 50 leaves, and checked all 112 stubs' commits.
Re-running `--seal` in a clone at `15c61b4` reproduced the seal. It ran a 25-case mutation of the gate and 9 seal
scenarios. Its verdict: the seal "correct and lossless", and the tool to be hardened before another tree is sealed,
P2 and P3 first. The answering context rebuilt the tool's core and took the self-test from 14 arms to 25, each new
leg proven by a mutation that turns its arm red. The real history passes every leg, all 112 sealed leaves proven
against the trees `940baf1`'s parent held.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| P1 | defect | the record said the fallback was `not recorded`; the tool wrote `in the tree's Commit Log`, unchecked; the example stub's commit was invented | the fallback is `in the tree's Commit Log` only when that log has a row for the leaf, else `not recorded`, and the seal warns; the example is the real stub (the record, the tool) |
| P2 | gap | the seal sliced by lines, so a column-0 fence, heading or prose tore a leaf, and one tear passed the gate | the seal refuses a leaf with a line at column 0 after its `ID`, writing nothing, and rolls back if the gate refuses what it wrote; RED arm (the tool) |
| P3 | gap | the gate never re-proved a seal: a hand-made seal from an open subtree, an edited body, a stub moved to another tree all passed | leg 5, provenance: every sealed leaf equals, byte for byte, its tree's leaf just before its sealing commit, `done`, with its whole subtree; stubs are tied to their tree and leaves to their subtree's file; RED arms (the tool) |
| P4 | ambiguity | the HEAD comparison is vacuous in CI, so a committed forgery passed there; the honest limit understated it | leg 3 is history-wide: every row any committed index held, and every sealed file against the commit that added it; RED arm for a committed forgery. `HISTORY-LEDGERS` has the same blind spot, filed as `PROGRAM.40` |
| P5 | gap | 8 of 15 breach paths had no RED arm | an arm for each, 25 in all |
| P6 | gap | reopening a sealed subtree was undecided | forbidden: new work opens a new top-level subtree, and leg 6 refuses a live leaf in a sealed subtree (the record, the tool) |
| P7 | defect | the rule said a sealing commit changes nothing else, and the first one also closed its leaf | the rule reworded: the sealing commit may close the leaf that ran it, and leg 5 proves every sealed leaf against its parent, so other edits do not weaken the proof |
| P8 | defect | figures that did not reproduce at their commit | corrected at the commit: `PROGRAM.md` 384 721 bytes before, 978 lines and 113 467 bytes after; the inventory's aggregate re-measured; the correction noted in `PROGRAM.32.3` and the tree's changelog |
| P9 | nit | the book overstated what the index check covers | "has lost or changed no row it ever held", and "since the commit that sealed it" |
| P10 | nit | a traceback in place of a named breach; the work-unit pattern's missing boundary; a stray file at the folder's root; the folder's size; a silent seal of a `done` leaf with a `pending` commit | named breaches; a boundary; stray files refused; the size stated as unbounded by design; a warning (the tool, the record) |

**Round 2**, `2026-10-10`, of `PROGRAM.69`'s amendment — the outermost closed subtree as the unit, the tool and its
gate, the record's text and the seal of `M1`, `M2` and `M3`, staged on `fea69ad` — by a read-only context that had not
written them. With its own parser it re-derived the seal: the 28 new files byte for byte `fea69ad`'s leaf spans and
exactly the outermost closed set there, every stub two lines linking its file, the stubs expanded giving back each
tree, every index row matching its file. It ran nine fixtures and ten mutations of its own. Verdict: 5 defects. The
answering context made leg 5's outermost rule an equality, added five arms (34 in all), and ran ten mutations of the
tool, each killed — the three that had survived among them.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R2-1 | defect | no arm held the unit's main rule: a mutation choosing the deepest closed leaf survived every arm, as did one deleting the missing-intermediate rule and one confining the outermost check to a top-level outermost | arms for a closed subtree below an open one sealed whole in one file, two leaves with no leaf between them and an open subtree sealed apart, and a part sealed by hand apart from its closed subtree below an open one; the three mutations killed |
| R2-2 | defect | leg 5 refused only a seal too narrow, so a hand-made `X/X.1.2.md` over `X.1.2.1` and `X.1.2.2`, with no leaf `X.1.2`, passed while the tool seals them apart | leg 5 requires each sealed leaf's outermost closed subtree, in the tree before the seal, to be its file's; an arm for parts sealed together that no one leaf holds; the record states the missing-intermediate rule |
| R2-3 | defect | the census figures did not reproduce from `fea69ad` as bytes: `M1` and `M2` matched as characters, `M3` matched neither | re-measured in bytes at `fea69ad` by `target/m369/census.py`: 274 483 bytes, `M1` 85 113, `M2` 105 946, `M3` 83 424, 261 610 in 28 subtrees, and the seal's 261 582, one trailing blank line per subtree apart; corrected in the record, the script and the leaf |
| R2-4 | defect | the record's unamended sentences contradicted the amendment: a subtree with an open leaf staying whole, what stays live, reopening's "new top-level subtree", legs 5 and 6 | each sentence kept with a dated amendment beside it |
| R2-5 | defect | `DOCTRINE_ENFORCEMENT.md`'s `TASK-HISTORY` row still named top-level subtrees alone, and its legs lacked the outermost clause and "at any depth" | the row updated |
| R2-6 | remark | the book said "the largest part" moves, in the singular, and listed no new check | "each finished part … the largest it can"; the outermost clause and leg 6 listed |
| R2-7 | remark | the seal judges the working tree and leg 5 the commit, so an uncommitted new open leaf makes a seal refuse itself | stated in "How to apply": commit such an edit first |
| R2-8 | remark | the whole-subtree arm passed for leg 6's sake as well; a mutation checking one sealed leaf's outermost survived | the arm drops the open child, so leg 5 alone refuses it; the one-leaf mutation is equivalent while leg 4 holds — every leaf of a file lies under its key, and below an outermost closed key every leaf's outermost is that key |
| R2-9 | remark, not this change's | leg 6 compared a live leaf with its own tree file's seals alone | leg 6 now reads every tree's seals; an arm for a live leaf under a sealed subtree filed in another tree |

**Round 3**, `2026-10-10`, a confirmation of round 2's answers and of the whole change again, by a read-only context
that had seen neither the change's writing nor round 2's reasoning. It re-derived the seal with its own code — the 28
files exactly the outermost closed set at `fea69ad`, byte for byte, the stubs expanded giving back each tree — and every
figure, in bytes, by a census over every tree file; ran seven edge fixtures, all as the record says; and ran twenty
mutations of its own, ten killed. Verdict: 2 defects. The answering context added six arms (38 in all) and two rules,
and ran seventeen mutations of its own, each killed; of the reviewer's twenty, fourteen are killed now and six survive,
each shown below to be equivalent or held by another leg.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-1 | defect | R2-3 said the figures were corrected in the leaf, and `PROGRAM.69` and its frontier row still held the first ones | a dated correction beside the leaf's figures; the frontier row rewritten at the close |
| R3-2 | defect | two corners of the unit's rule had no arm: a missing intermediate below depth 3 (`k == 3` survived, sealing `X.1.2.1.1` and `.2` together) and a top-level subtree with no leaf of its own (split, and the gate passed) | fixture leaves `T.2.2.5.1` and `.2` under an open leaf, sealed apart, and `T.4.1` and `.2` with no `T.4`, sealed together as `T.4`; both mutations killed |
| R3-3 | remark | survivors that are gaps: `blocked` in no fixture; the seal's filter of a subtree already sealed in no arm; `under`'s dot boundary held only by the real history | `T.2.2.1` `blocked`; an arm sealing over a late leaf under a sealed subtree, the file left as sealed; `T.2.10` beside `T.2.1`; each mutation killed. The six survivors: one leaf's outermost checked alone (equivalent while leg 4 holds), leg 4's placement removed (held by leg 5's outermost equality), `units` without its `seen` test (every candidate is the leaf's top-level key or a leaf of the tree, both seen), leg 5 passing a leaf with no closed subtree (held by its `done` and whole-subtree checks), leg 6 strictly below a key (the key's own leaf is held by the sealed-and-live check), `present` without stubs (a stub's key holds no live leaf but by leg 6's breach) |
| R3-4 | remark | a leaf filed under another tree's name made a seal misreport, `units` judging one file and leg 6 every file | refused by name: the gate notes a leaf not under its tree's name, and a seal writes nothing for such a tree; two arms |
| R3-5 | remark, not this change's | leg 5 compared leaf bodies, so a sealed file reordered or padded with blank lines, its row recomputed before its first commit, passed | leg 5 rebuilds the file from its leaves' spans in the base tree's order and compares bytes; the 170 sealed files pass it; an arm for a reordered file |
| R3-6 | remark | the record's "a feature's history in one file" unamended; "whose parent's is not" wrong for a parent that is no leaf; the script's header figure about top-level subtrees | an inline amendment; "nearest ancestor leaf's"; "open top-level subtrees" |

**Round 4**, `2026-10-10`, a second confirmation, by a read-only context that had seen neither the change's writing
nor any earlier round's reasoning. Its own parser and unit rule gave exactly the 28 staged units, every file, tree,
stub and row matching, and a clean clone's seal at `fea69ad` reproduced them; all 170 sealed files rebuilt byte for byte
from their trees before their seals, the outermost subtree equal to each key. It fuzzed the tool: 840 seals of random
trees over up to three rounds matched its oracle, every gate after each commit passing, no sealed file changing; and
of 890 seals made by hand the gate accepted exactly those the tool makes. Verdict: 3 defects. The answering context
reopened this history before appending the amendment's rounds; added three arms (41 in all)
and a census mode to the tool; ran twenty mutations of its own, each killed.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R4-1 | defect | rounds 2 and 3 were appended to this history while its row read closed, and `REVIEW-HISTORY` then refuses ever to archive it — a closed history is frozen | the row reopened by a commit of its own, `ARCHOGEN-PROGRAM-0510`, before the rounds were appended; a new history file was the other route, and would be `docs/reviews/`' seventeenth, past its ceiling of sixteen — a ceiling every archived history's stub keeps counting, filed as `PROGRAM.70` |
| R4-2 | defect | two of R3-3's six survivors are neither equivalent nor held: leg 6 strictly below a key passes a live leaf named as a sealed subtree that had no leaf of its own (`T.4`), and leg 5 passing a leaf with no closed subtree passes the tree's root leaf sealed by hand | an arm for each; both mutations killed; R3-3's claim for these two was wrong. The other four — one leaf's outermost checked alone, leg 4's placement, `units` without `seen`, `present` without stubs — re-argued equivalent by this round |
| R4-3 | defect | the census figures had an untracked producer, `target/m369/census.py`, against `docs/CLAIM_VERIFICATION.md`'s third leg | `bash scripts/check_task_history.sh --census <COMMIT>`, the tool's own mode on its own `units`, with an arm and a mutation; `--census fea69ad` reproduces every figure; the record, the script and the leaf cite it |
| R4-4 | remark | the leaf's 823 457 and 815 443 are a working tree's, and say not | marked so in the leaf's dated correction |
| R4-5 | remark | the record's example "`M3.6.1` with its children": `M3.6.1` had none | "`M3.6.2` with `M3.6.2.1`" |
| R4-6 | remark | "parts sealed together that no one leaf holds" is refused below the top level alone: a top-level subtree with no leaf of its own seals whole | "below the top level" in the record and the script |
| R4-7 | remark, not this change's | the gate accepts a hand seal of a leaf with a column-0 line, which the seal refuses, so the book's "as the tool would have sealed it" says more than the gate checks | the book's sentence narrowed; the column-0 rule stays the seal's, since `PROGRAM/PROGRAM.3.md` already holds such a line |
| R4-8 | remark | the "sealed and also live" note is equivalent to leg 6; the tree-name check's dot boundary is held only through the shared `under` | recorded; no change |

**Round 5**, `2026-10-10`, a third confirmation, by a read-only context new to the change. It re-derived the seal and
every figure with its own code, the census mode against its own parser; confirmed R4-1's route by archiving the history
in a clone once its row closed; and judged the four survivors H, L, M and R equivalent, each with its argument. Verdict:
2 defects. Since rounds 2 to 5 had each found one more rule of the tool no arm held, the answering context changed
method as well as answering: a mutation sweep of the tool's whole core, below.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R5-1 | defect | no arm held the census's byte figures: span bytes without the final newline, every live leaf counted closed, and characters for bytes — R2-3's own error — each passed, the arm checking the subtree count alone | the arm asserts both totals of the fixture, measured by an implementation apart from the tool, a closed leaf holding multi-byte characters so bytes and characters differ; the census runs inside the unreadable-file handling |
| R5-2 | defect | a first seal the gate refuses left its tree's new folder behind, and the gate then refused the repository; no arm wrote a seal and saw it refused | the seal notes the folders it makes and removes them on rollback; an arm for a first seal refused, the history left as it was and the gate passing after it, and one for a refused seal into an existing history, its index put back |
| R5-3 | remark | `--census` was named nowhere a reader looks for a tool | `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`'s row, the book's console block and the record |
| R5-4 | remark, not this change's | a stub's commit was free text to the gate | leg 5 compares each stub's commit with the one the seal derives — the leaf's `Commit:` field before the seal, else the Commit Log of the tree it ran on; two stubs of the first seal, `M1.26.2` and `PROGRAM.18`, said "in the tree's Commit Log" with no row there, round 1's P1 having fixed the fallback for later seals only, and each is corrected in its tree with a changelog line. A row's date stays unchecked: it is the day the seal ran, in UTC, which no commit records |
| R5-5 | remark | "a seal made by hand … is refused" says more than the gate checks, which accepts a hand seal byte for byte the tool's | the record and the book: a seal the tool would not make |

**The sweep**, `2026-10-10`, by the answering context after round 5: every refusal of the tool's Python core silenced,
every `if` made never and always true, every refusing `sys.exit` made a `print`, one at a time, each mutant's
self-test run in a scratch of its own. The runner, `target/m369/sweep.py`, is untracked scratch, so the counts below are
not durable and say so (`docs/CLAIM_VERIFICATION.md`, the third leg); its mutants are the operators just named, on the
lines the core holds. The first run, of 183 mutants, killed 136. Of the 47 others, 33 were rules no arm held, and each
gained one: a `Commit:` field naming its work unit on its second line; the Commit Log fallback, with the row the
sealing commit adds; a malformed row; a row's lines, bytes or sha256 alone wrong before the seal's first commit, when
leg 3 has no commit to compare; a stub linking another sealed file; a sealed leaf its tree never held; a sealed file
of a tree its base did not hold; a refused seal into an existing history, its index put back; the census of a commit
git does not know, and a template it passes over; the seal of a tree that does not exist; the seal's warning of a
leaf naming no commit; a failed `git log` of the index and of the history, git made to fail; a tree file that is not
UTF-8, to the gate and to the census. And the index's layout — one table per tree, opening with its header, its rows
under it — became a check of its own, since a row written into another tree's table passed. The second run, of 196,
killed 175; of the 21 others, seven were gaps — the layout's two other rules; two refusals whose mutants printed their
message and then a traceback, so every arm now also refuses a traceback; and the seal's own refusal of a foreign leaf,
which the gate and the rollback masked — each armed. The third run killed 182 of 196, the self-test at 63 arms; the 14
others are below, each with its reason.

| Line of the core | Mutant | Why it survives |
| --- | --- | --- |
| `if args not in _git:` | always true | the git cache bypassed: performance alone |
| `if subtree(lid, tree_name) is None:`, in `units` | never true | the tree's root leaf has no prefix of two components, so nothing is added: equivalent |
| leg 4's "not in subtree" | silenced | held by leg 5: a leaf's outermost closed subtree is an ancestor of it, so equality places it |
| "sealed twice" | silenced | the second file's key is not the leaf's outermost (leg 5), and the stub links one file only (leg 4) |
| `elif line.strip():`, reading the history's log | always true | a blank line of the log taken for a path no file has: equivalent |
| "sealed … and also live here" | silenced | leg 6 refuses the same live leaf |
| `if all(lid in was for lid in sealed_ids):` | always true | the byte rebuild run without a missing leaf adds a note only beside "did not hold": equivalent |
| `if status(blines, bf) != "done":` | never true, or its note silenced | a leaf not `done` has no closed subtree, so leg 5's equality refuses it |
| the census's `if not live:` | never true | a tree with nothing to count prints a line of zeros; the totals are the same: cosmetic |
| the seal's "would lose" | never true, or `exit` made `print` | defensive: it fires only if the seal's own slicing is wrong, which no input reaches |
| the seal's "would not reconstruct" | never true, or `exit` made `print` | defensive, likewise |

**Round 6**, `2026-10-10`, a fourth confirmation, by a read-only context new to the change, on the sweep's state. It
re-derived the seal with its own code and re-sealed `72bd446` with the tool in a work tree of its own, every file,
`M2` and `M3` byte for byte; re-ran the sweep in a clone, the same 182 of 196; and ran its own experiments on the
survivors. Verdict: 5 defects. Rounds 2 to 6 found 5, 2, 3, 2 and 5: the answers are committed as they stand, the
leaf open, and the review stays open until a round finds no defect.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R6-1 | defect | the sweep's reason for "sealed twice" held only for two files: a row repeated for one file, any date, was caught by that note alone, and passed with it silenced | a row twice in its tree's table refused by name; an arm. The sweep's reason for that survivor is corrected here: it is held by this refusal, and by legs 4 and 5 for two files |
| R6-2 | defect | a file the gate cannot read, met after the seal wrote, escaped the rollback: the seal's writes stayed | the seal's gate run inside the unreadable-file handling, so it rolls back as any refusal does; an arm sealing beside a tree that is not UTF-8, the trees and the history left as they were |
| R6-3 | defect | the two first-seal stubs R5-4 "corrected" were right: `PROGRAM.18`'s Commit Log row reads `` `PROGRAM.18` → `PROGRAM.18.1` ``, and `M1.26.2.2`'s row closes `M1.26.2`; the gate's matcher reads one row shape alone | the stub-commit check R5-4 added is withdrawn — whether a row is a leaf's is prose the gate cannot judge — the two stubs restored as the first seal wrote them, their changelog lines removed, and the limit stated in the record, the script and the book |
| R6-4 | defect | the record and the book said the gate refuses any seal the tool would not make, and it accepts a hand seal of a leaf with a column-0 line, and any row date | "when it differs from the tool's in its files, its stubs' links or its units", the commit text, the date and the column-0 rule named as the seal's alone |
| R6-5 | defect | figures naming no state: `docs/tasks/` "818 704 → 562 833" matched no commit, "256 KB below" its ceiling was false at the commit, and counts from untracked runners said so in the history alone | the seal's 255 871 bytes, a difference that holds in every state, and 818 985 at `72bd446`; the changelog's claim rewritten; every count from an untracked runner marked so where it appears |
| R6-6 | remark | the stub check's reading of the working tree raised a traceback on a tree removed | gone with the check (R6-3) |
| R6-7 | remark | the sweep called "would lose" unreachable, and a tree holding one id twice reaches it | a leaf named twice in a tree refused by name, by the gate and by the seal before it writes; an arm each. The survivor's reason is corrected here: reached only through that refusal's input, which no longer reaches the seal's proof |
| R6-8 | remark | "five review rounds" counted round 1, `PROGRAM.32.4`'s | "rounds 2 to 6" |
| R6-9 | remark | the census described as measuring "the finished leaves a seal would take" | the record's own words in `TOOLBOX.md` and the book: `done` leaves under open top-level subtrees, and those in closed subtrees below them |
| R6-10 | remark | the reviews index row said one round | six, open |
| R6-11 | nit | round 4's row: "890 hand seals as the tool makes them" | "of 890 hand seals the gate accepted exactly those the tool makes" |

**Round 7**, `2026-10-10`, a fifth confirmation, in a clone of `e624001`, by a read-only context new to the change. It
re-derived the seal with its own code and re-sealed `fea69ad` with the committed tool, every tree and file identical;
fuzzed the tool over three seeds; and judged each finding this change's or pre-existing. Verdict: 4 defects of this
change, and two pre-existing remarks. Every count in rounds 2 to 7 that a runner under `target/` produced — the
mutation counts, round 4's and this round's fuzzing, the sweep's runs — is from a runner not tracked, and so not
durable (`docs/CLAIM_VERIFICATION.md`, its third leg); this round's paragraph says so for all of them, since an earlier
round is not edited.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R7-1 | defect | a sealed file holding no leaf passed the gate (a gap since `fea69ad`), against the text R6-4 wrote, that a hand seal differing from the tool's in its files is refused | leg 1 refuses a sealed file with no leaf; an arm |
| R7-2 | defect | `PROGRAM.69`'s checklist still described the stub-commit check R6-3 withdrew, and named `M1`'s changelog | the three phrases removed |
| R7-3 | defect | R6-5 said every count from an untracked runner was marked, and round 4's fuzzing counts were not, in the record, the leaf and this history | marked in the record and the leaf; for this history, by this paragraph |
| R7-4 | defect | the rollback was whole for a file not UTF-8 alone: a file the gate cannot open left the seal's writes behind and a traceback | every write and the gate's proof in one guard, whatever stops the run rolling the seal back first, the run's handler naming a file it cannot read; an arm with a tree file it cannot open; the narrower inner catch, shown equivalent by a mutation, removed |
| R7-5 | remark | the changelog's "182 of 196" did not say on which tool | "on round 5's tool" |
| R7-6 | remark | no arm held the column-0 refusal below the top level, and a mutation confining it there passed | an arm: a column-0 line in the second leaf of a closed subtree below an open one, refused with nothing written — which also answers P2 below |
| R7-7 | remark | the record's legs, the script's, the doctrine row and the book omitted two refusals: a leaf named twice, and the index's layout | each names both |
| R7-8 | nit | `PROGRAM.70` was filed "by the fourth review" | "by the answer to the review's round 4" |
| R7-P1 | remark, pre-existing | the index's "append-only" holds presence, not order: a row inserted above committed rows passes | filed as `PROGRAM.71`, owned, not built into this change |
| R7-P2 | remark, pre-existing | the column-0 refusal was held for a unit's first leaf alone | R7-6's arm puts the line in a second leaf |

**Round 8**, `2026-10-10`, a sixth confirmation, in a clone of `235644e`, by a read-only context new to the change. It
re-derived the seal from `72bd446` with its own code and re-ran the committed tool there, identical; reproduced the census
and every figure; and fuzzed: 960 seals matched its oracle, and of 1 368 hand seals the gate accepted exactly those the
tool makes — its runner untracked, so not durable. Round 7's own counts, 795 seals and 1 551 hand seals, stand in the
record's row alone, from an untracked runner likewise. Verdict: 2 defects of this change, both in the text, and one
pre-existing finding.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R8-1 | defect | the doctrine row's "a leaf a file" read as one leaf to a sealed file, which `M2/M2.7.3.md`'s 18 leaves belie | "one row a file" |
| R8-2 | defect | "whatever stops the run rolls the seal back" was false for a kill signal: SIGTERM mid-gate left the writes | the claim narrowed, in the script, the record and the leaf, to any exception, a kill signal a stated limit whose writes the next gate run proves; the R7-4 answer above and `0514`'s Commit Log row say more, and are corrected here |
| R8-3 | remark | no arm held the guard's `raise`: without it a refused seal printed "sealed" | every arm expecting a refusal refuses a run that says "— sealed "; the mutation killed |
| R8-4 | remark | no arm held the table's separator line | an arm; the mutation killed |
| R8-5 | remark | the unreadable-file arm needed a non-root user | a directory named as a tree file, unreadable to root as to anyone |
| R8-6 | remark | the record's round 7 row used R6-11's corrected wording again | "of 1 551 hand seals the gate accepted exactly those the tool makes" |
| R8-7 | remark | `PROGRAM.71` filed and routed nowhere; no Verification Log row for round 7 | in the frontier's note, the index, the tree's changelog; a Verification Log row for rounds 7 and 8 |
| R8-8 | remark | `PROGRAM.71` cited a reviewer's runner unmarked | marked untracked, not durable |
| R8-9 | nit | an empty table passed; the folders were made outside the guard | a table with no row refused, with an arm; the folders made inside it |
| R8-P1 | remark, pre-existing | a tree file in a sub-folder escapes leg 6 and its name checks, while `TASK-ACCEPTANCE` reads owners there | filed as `PROGRAM.72`, owned |

**Round 9**, `2026-10-10`, a seventh confirmation, in a clone of `7ad8e6b`, by a read-only context new to the change. It
re-derived the seal from `fea69ad` with its own code, byte for byte `e624001`'s; re-ran the `7ad8e6b` tool on
`72bd446`, identical but for the index's header prose, which `e624001` set to the tool's `HEADER`; reproduced the census
and every figure; and fuzzed: 480 seals matched its oracle, and of 1 903 hand seals the gate accepted exactly the
oracle's 555 — its runner untracked, so not durable. Verdict: 1 defect of this change, and 2 arm gaps it counts as
defects by its own definition and as remarks by rounds 7 and 8's; 4 remarks; one pre-existing finding; two findings of
other leaves.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R9-1 | defect | the rollback was not whole when the failure defeated its own writes too: a read-only tree's rewrite raised again and the folders stayed; a file-size limit left the tree truncated | the tree and the index written whole or not at all, a temporary file renamed into place; the rollback undoes what was written alone, each step on its own, keeps the sealed files while the tree or the index names them, and names what it could not undo; two arms under a file-size limit — the seal stopped, the tree as it was; the rollback stopped, what it could not undo named, the sealed file kept |
| R9-2 | arm gap | "any exception" was held for the gate raising `Unreadable` or `OSError` alone | arms for an interrupt and for git output that is not UTF-8, mid-proof, each rolled back; a sealed file's path already taken, a dangling link among them, refused before any write, with an arm |
| R9-3 | arm gap | a refused first seal leaving an empty `docs/task-history/` passed every arm | the first-seal arm requires the folder gone |
| R9-4 | remark | ", or if any exception stops it" was inserted into a `2026-09-30` sentence unmarked | moved into that sentence's amendment |
| R9-5 | remark | the gate's unchecked parts were listed as if whole; it also accepts a stub moved within its tree and text after the name on a stub's ID line | both named among the stated limits, in the record, the script and the book |
| R9-6 | remark | no arm held the blank line between a tree's heading and its table | an arm; the mutation killed |
| R9-7 | remark | "in any tree file" did not point to `PROGRAM.72` | it does, in the record, the script and the book |
| R9-P1 | remark, pre-existing | the gate's own run ends in a traceback on an exception other than `Unreadable` or `OSError` | filed as `PROGRAM.73`, owned |
| R9-N1 | another leaf's | `MEMORY.md`'s next action still said `.2.5` follows | rewritten by `ARCHOGEN-M3-0518` |
| R9-N2 | another leaf's | `M3.6.6.2` closed and was not sealed | sealed by `ARCHOGEN-M3-0520`, its Commit field first (`ARCHOGEN-M3-0519`) |

**Round 10**, `2026-10-10`, an eighth confirmation, in a clone of `0a7ec7f`, by a read-only context new to the change.
It reproduced the gate, every self-test, the census and the seal's figures; re-sealed `72bd446` with `7ad8e6b`'s and
`0a7ec7f`'s tools, byte for byte `e624001`'s; ran round 9's new arms on `7ad8e6b`'s core, failing; and killed nine
mutations of its own — its runners untracked, so not durable. Verdict: 4 defects of this change, 3 arm gaps, 3 remarks
and one pre-existing finding.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R10-D1 | defect | an interrupt just after the tree's rename, the index's or a sealed file's creation — before the write was noted — left the rollback blind: the tree kept its stubs while the sealed files were removed | each write noted before it is made; a noted write undone only where it happened, a file that no longer holds its text before, a sealed file that exists; four arms, a hook raising the interrupt just after each of the three writes and just before a creation |
| R10-D2 | defect | "a sealed file's path already taken is refused before any write" held for a link to nothing alone: a file, a folder or a link to a file there was taken for a seal made, and skipped; R9-2's answer above said more | a unit the index rows is skipped, as sealed, for the gate to judge; any other unit's path must be free, any entry there refused; an arm with a file at the path |
| R10-D3 | defect | the book's "undoes every write if … anything stops it" repeated what R8-2 withdrew | "an error or an interrupt"; a process killed outright leaves its writes for the next check |
| R10-D4 | defect | round 9's counts: "nine … each caught" unmarked in the changelog; the Verification Log's "74 pass / 2 fail" from no committed state | marked untracked and not durable; the tally named as the working tree's first arms beside `7ad8e6b`'s core |
| R10-AG1 | arm gap | "each step on its own" held by no arm | the index made a folder mid-proof: its restore fails, named, and the tree's still runs |
| R10-AG2 | arm gap | a removal the rollback could not make went unnamed under a mutation | two sealed files made folders mid-proof: both removals fail, both named |
| R10-AG3 | arm gap | `put`'s "a link is followed" held by no arm | a tree or an index that is a link refused before any write, with an arm; `put` writes the path itself |
| R10-R1 | remark | the leaf's "OK (170 sealed file(s)" named no state | "at `e624001`" |
| R10-R2 | remark | `open(…, "x")` for `"w"` survives | equivalent outside a race, the taken check running first; kept as a defence |
| R10-R3 | remark | "a failure that stops one stops no other" overstated: a failed restore keeps the sealed files | the comment says what each step stops and what it keeps |
| R10-P1 | remark, pre-existing | the gate accepts a seal of only some closed units, each the tool's, and prose in the index outside its tables | both among the stated limits — the rest seal later as units of their own |

**Round 11**, `2026-10-10`, a ninth confirmation, in a clone of `bfabefa`, by a read-only context new to the change.
It reproduced the gate, every self-test, the census and the seal's figures; re-sealed `72bd446` with `bfabefa`'s tool,
byte for byte `e624001`'s; fuzzed 753 cases against an oracle of its own, every one agreeing; and killed eleven
mutations of round 10's rules, one more surviving as equivalent — its runners untracked, so not durable. Verdict: 2
defects of this change, 2 arm gaps, 4 remarks and 2 pre-existing findings.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R11-D1 | defect | a first interrupt during the rollback of a seal the gate refused escaped it, leaving the writes unnamed — the rollback ran outside the guard, its steps catching `Exception` alone — so "a seal interrupted at any instant is undone" and the book's account were false | a stop — SIGINT, SIGTERM, SIGHUP — is recorded by a handler, never raised mid-write, and answered after each write and after the proof by a rollback that runs to its end; what leaves writes is now what no handler sees, SIGKILL or the machine stopping; arms for SIGINT at each write and mid-proof, for SIGTERM, and for an interrupt during the rollback |
| R11-D2 | defect | the arms said to fail first were misnamed: the just-before-creation and failed-restore arms pass on `0a7ec7f`'s core | the five that fail named — the tree, index and create interrupts, the taken file, the linked tree — and the just-before-creation arm said to hold the existence guard |
| R11-AG1 | arm gap | no arm held the refusal of a linked index | an arm |
| R11-AG2 | arm gap | no arm held that a file another writer puts at a sealed path is left alone | a hook plants it just before the creation; an arm |
| R11-R1 | remark | an arm's name said "just after" for a stop before a creation | each arm's name says what it does |
| R11-R2 | remark | the sealed files are kept whenever a restore fails, not only while a file still names them | the record says so |
| R11-R3 | remark | the rollback read a file with its line ends translated, `before` raw | `current` reads as `read` does |
| R11-R4 | remark | the book's "an interrupt" indexed under the glossary's hardware term | "asked to stop" |
| R11-P1 | remark, pre-existing | the record's "every stub replaced" meant every new stub | amended |
| R11-P2 | remark, pre-existing | a linked history folder let the seal write outside the repository | refused before any write, with an arm |

**Round 12**, `2026-10-10`, a tenth confirmation, in a clone of `3ee1160`, by a read-only context new to the change.
It reproduced the gate, every self-test, the census and the seal's figures; re-sealed `72bd446`, byte for byte
`e624001`'s; and ran eleven mutations of round 11's rules, six killed — its runners untracked, so not durable. Verdict:
3 defects of this change, 4 arm gaps, 5 remarks. The defects were each in a claim wider than its code, so the answer
narrows the claims to what the code does and the arms hold, as round 3 of the generated-sources design did.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R12-D1 | defect | a stop recorded after the last answer, before the handlers were given back, was dropped: the seal kept, exit 0 | the handlers given back first, then any recorded stop answered by a rollback; a stop after that finds the seal proven and kept, said in the record, the script and the book; an arm sends SIGINT as they are given back |
| R12-D2 | defect | a link above the written folders — `docs/tasks`, `docs` — let the seal write outside the repository, though the texts said it never writes through one | any path the seal writes refused when its real path is not the root's own, a link on the file or on any folder above it; arms for a linked trees' folder and a linked history root |
| R12-D3 | defect | "rolled back" printed after a rollback that left what it named | the rollback says whether it was whole, and the refusal and the stop say "rolled back" of a whole one alone; an arm for each |
| R12-AG1 | arm gap | SIGHUP claimed, no arm | an arm |
| R12-AG2 | arm gap | the history root's link refusal, no arm | the linked history root's arm |
| R12-AG3 | arm gap | "answered after each write", no arm | the claim withdrawn: a stop is answered before the seal is declared, which the arms hold |
| R12-AG4 | arm gap | the rollback's existence guard unheld once stops were no longer raised mid-`open` | a folder gone just before a creation; an arm |
| R12-R1 | remark | a signal ignored on entry, as under `nohup`, was taken over | left ignored, with an arm |
| R12-R2 | remark | a stopped seal exited 1, as a breach | it exits 128 and the signal's number |
| R12-R3 | remark | round 11's hybrid tally holds with SIGINT at its default, in the foreground | said here |
| R12-R4 | remark | restoring the handlers is held by no arm | not claimed |
| R12-R5 | remark | the rest of the text checked and true | — |

**Round 13**, `2026-10-10`, an eleventh confirmation, in a clone of `1ad87e4`, by a read-only context new to the change.
It reproduced the gate, every self-test, the census and the seal; fuzzed 535 seals and 1 503 hand seals, every one as
its oracle predicts, and 502 late live leaves, every one refused; and ran nineteen mutations of round 12's rules — its
runners untracked, so not durable. Verdict: 4 defects of this change and 2 arm gaps. Rounds 11 to 13 each found a
moment between a write and the record of a stop; the answer changes method: a stop is held by the signal mask, not
recorded and answered, so no such moment exists.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R13-D1 | defect | a second stop during the rollback that answered a given-back stop cut it short | a stop is held by the signal mask through the writes, the proof and any rollback, taking effect once the seal is done; the given-back route is gone; arms for two stops mid-seal and for an interrupt during a refused seal's rollback |
| R13-D2 | defect | three texts put the "seal kept" boundary at the check, the code at the hand-back | one boundary, the seal done: the record, the book and the changelog say so |
| R13-D3 | defect | a link at `put`'s temporary path was written through, and the tree replaced by it | the temporary file created new (`open` exclusive), removed only by the call that created it; an arm plants the link |
| R13-D4 | defect | the changelog's "five failing on the old tool" | six |
| R13-AG1 | arm gap | only SIGINT's exit code was checked | each stop's arm expects its own: 130, 143, 129 |
| R13-AG2 | arm gap | "rolled back" on the given-back route unheld | the route is gone; the refusal's message keeps its arm |
| R13-R1 | remark | the leaf's hybrid tally was taken before the hang-up arm | said; this round's tally is of the arms as committed |
| R13-R2 | remark | a SIGKILL between a temporary write and its rename leaves the temporary file | within the stated limit |
| R13-R3 | remark | `MEMORY.md`'s line on `M3.6.6.4` | that leaf's matter |
| — | found while answering | an arm passed with the "sealed" line lost: bash's report of a job killed by a signal quotes the command's source, which holds the format string | every check of that line matches the tool's output line, anchored; the mutation dropping its flush killed |

**Round 14**, `2026-10-10`, a twelfth confirmation, in a clone of `6c79b36`, by a read-only context new to the change.
It reproduced the gate, every self-test, the census and the seal; fuzzed 300 trees, 2 293 units sealed as its oracle
predicts, and 1 662 hand seals, the gate taking exactly the tool's and their subsets; refused 445 late live leaves; and
ran thirteen mutations of round 13's rules, nine killed — its runners untracked, so not durable. Verdict: 4 defects of
this change and 2 arm gaps; `make focused` could not run in its clone, under the original's cargo configuration.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R14-D1 | defect | the interrupt's message said a seal was finished whatever happened: under a rollback that was not whole, after an error the held stop swallowed, and before any write | the seal records that it has said its outcome; the message is "once the seal was done; its outcome is said above" or "before the seal wrote anything"; an error is said, flushed, before it goes on; arms for each case |
| R14-D2 | defect | "what no mask holds, SIGKILL or the machine stopping": SIGQUIT and the other maskable signals left the writes | every signal a mask can hold is held, but the faults the interpreter raises on itself; arms for SIGQUIT, SIGUSR1, SIGUSR2, SIGALRM, each with its own exit code |
| R14-D3 | defect | "no link lies on a path it writes": a link at the temporary path was found after the first write | any entry at a rewritten file's temporary path refused before the first write; the arm expects nothing written |
| R14-D4 | defect | the record credited the temporary file created new to R12 | R13 D3 |
| R14-AG1 | arm gap | the rollback after an error, under a held stop, held by no arm | an arm: an error after a held interrupt, rolled back whole and said, then exit 130 |
| R14-AG2 | arm gap | "from before the first write": the folder's creation held by no arm | an arm: an interrupt just after the history folder's creation |
| R14-R1 | remark | dropping the refusal's flush survives: stderr is line-buffered | equivalent; kept |
| R14-R2 | remark | "every check of that line … anchored" was too broad: three exit-0 arms match a part of it | narrowed: the checks of a run a stop ends are anchored, where the shell's report can quote the source |
| R14-R3 | remark | the git the proof runs inherits the mask, so a stop does not end a hung git | said in the record and the script |
| R14-R4 | remark | round 13's account fitted rounds 11 and 12 loosely; the changelog's "three reviews in a row" | the changelog's entry reworded; round 13's paragraph stands as written, this row its correction |
| R14-R5 | remark | unblocking all three signals instead of restoring the mask survives | no text claims it; the mask is restored |

**Round 15**, `2026-10-10`, a thirteenth confirmation, in a clone of `421ce47`, by a read-only context new to the
change and fenced from the sealed evaluation set. It reproduced the gate, every self-test, the census, round 14's
figures and the seal; fuzzed 330 trees and 982 units, every seal as its oracle predicts, 1 354 hand seals, the gate
taking exactly the oracle's, and 235 late live leaves, every one refused; and ran fifteen mutations of round 14's
rules, nine killed — its runners untracked, so not durable. Verdict: 2 defects of this change and 4 arm gaps.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R15-D1 | defect | a stop after a seal whose report could not be written, its output a closed pipe, said "before the seal wrote anything" | the seal records that it started, under the mask, before the first write; the message is by both: before any write, the outcome said above, or the outcome could not be said; an arm with the output a pipe nobody reads |
| R15-D2 | defect | "every signal a mask can hold": the seven fault signals are left out whoever sends them, and a mask holds one another process sends | named: every signal but SIGKILL, SIGSTOP and the seven fault signals, whoever sends them — a fault the interpreter raises on itself while blocked would hang it, so the mask stays as it is |
| R15-AG1 | arm gap | the breadth of the mask held by seven named signals alone | an arm reads the mask in force at the first write and finds every signal but the nine |
| R15-AG2 | arm gap | "rolled back" of a whole rollback alone, on the error path | the full-disk rollback's arm requires "and its rollback left what it named above" and no "rolled back" |
| R15-AG3 | arm gap | "any entry at a temporary path": a link at the tree's alone | an arm puts a plain file at the index's temporary path |
| R15-AG4 | arm gap | the error named by the run's own handler, not only by the seal's line | the two error arms anchored on the gate's `TASK-HISTORY:` breach line |
| R15-R1 | remark | pre-existing: with the output closed and no stop, a kept seal ends in a broken-pipe breach | the report is outside the guard, as it was; nothing claims otherwise |
| R15-R2 | remark | the flush on the error path is equivalent, stderr line-buffered | kept |
| R15-R3 | remark | the changelog's "from before its first write" was not new in round 14 | reworded |

## Why

The record states the design as it stands, and this file keeps how it got there.

## How to apply

A later review appends here, and adds a row to the record's `## Review`.
