# Task-tree sealing: a short resume surface, and not one byte of history lost

A portable doctrine for any project that tracks its work in **task trees**: one plain-text file per top-level goal,
broken recursively into **leaves**, each leaf carrying its status, its acceptance checklist and its evidence. It
needs only files and a version-control system with commit hooks and CI. It assumes no particular agent, harness,
language or hosting service: humans, any AI agent, or both can follow it, because the rules are enforced at the
repository level, not by whoever happens to be working.

Written `2026-10-23` by leaf `PROGRAM.106` from archogen's own records, which are cited where a figure or a lesson
comes from (§11). Archogen's implementation is one instance of it, not a requirement of it.

---

## 0. In one paragraph

A task tree does two jobs that pull apart as a project ages: it is the **resume surface** every session reads to
learn what is next, and the **evidence record** of every leaf ever finished. Left alone, finished work comes to
dominate the file, and every session pays to read it. **Sealing** moves each finished part of the tree — the
outermost subtree whose leaves are all done — byte for byte into an archive file of its own, leaves a two-line
**stub** where each leaf stood, records the archive file's size and digest in an append-only **index**, and proves,
before writing anything, that putting the bodies back rebuilds the tree exactly. A **gate** that runs on every commit,
locally and in CI, re-proves every sealed file against the commit that wrote it, so the history cannot be edited,
forged or lost. Nothing is summarised, nothing is deleted, and a closed leaf is one link away.

---

## 1. The problem, and what set it off

### 1.1 Two jobs in one file

A leaf's lifecycle is: filed → active → done, with its root cause, fix, verification and commit written into it as
it closes. That record is valuable — it is how a later reader learns why the code is as it is — and it is written
once. The tree's *other* readers want the opposite: the open leaves, the frontier, the next action. Every session
that resumes the work, human or agent, reads the tree first.

So the tree grows by exactly the material its most frequent readers do not need, and that material is permanent.

### 1.2 The trigger, measured

Archogen's trees grew with no lifecycle until `2026-09-30`, when a measurement went to the project's director
(§11, the findings record §8):

| Tree | Size then | Finished work in it |
| --- | --- | --- |
| `M1` | 717 804 bytes, 7 633 lines | 70 of 74 leaves done; their bodies 77% of the file |
| `PROGRAM` | 378 708 bytes | 54 of 63 leaves done; 79% of the file |

In the 82 commits before the finding was written, `M1` had grown from 1 864 to 6 928 lines, 5 328 of them the bodies
of closed leaves; by the ruling, the same day, it held the 7 633 above. The
project's size-containment doctrine forbids one thing outright: a document that is both a growing history and
something every session must read. The task trees were becoming exactly that.

Three answers were weighed: (A) keep growing, as a recorded choice; (B) rotate the changelog-like files as
ledgers; (C) seal closed leaves out of the trees. (C) was ruled for the trees and is this doctrine. (B) is its
sibling for chronological files and is not described here.

### 1.3 The fixes that do not work

- **Deleting finished leaves.** The evidence is the point of the record; version-control history is not a reading
  surface, and nothing would prove what was removed.
- **Summarising them.** A summary is an edit of evidence. Evidence edited on its way into an archive is no longer
  evidence.
- **Moving text by hand into another file.** Archogen did this under ceiling pressure before its log rows were
  sealed (§8.6): text moved into an unbounded file, each move audited by hand. The bytes were relocated, not
  contained, and the destination grew without a bound.

---

## 2. Principles

1. **Move, never rewrite.** A finished leaf leaves the tree byte for byte. Nothing in it is reworded, re-wrapped or
   shortened, ever.
2. **Prove before writing.** The seal computes what it will write and checks that the stubs, expanded back into
   their bodies, rebuild the tree exactly. If not, it writes nothing.
3. **Sealed means immutable.** A sealed file never changes. A correction is a new entry in the live tree, which a
   later seal moves in turn.
4. **The stub stays where the leaf stood.** Every check, script and reader that looks a leaf up by its identifier
   still finds it, with its status and a link to its body.
5. **Seal the largest finished unit.** The unit is the outermost subtree whose every leaf is done — not the leaf
   alone, not only the top level (§4.1).
6. **Enforce it below the agent.** The proof and the immutability are checked by a gate in the commit hook and in
   CI. A forgetful or mistaken author — human or agent — is caught by the repository, not by diligence.
7. **Fail closed.** A seal that cannot finish undoes what it wrote; a gate that cannot read what it must judge
   refuses rather than passing.
8. **Read with no tool.** A sealed file is a plain file and a stub is a link. Reading history needs a text viewer,
   not the tool that wrote it.

---

## 3. Vocabulary

- **Task tree** — one file per top-level goal, for example `docs/tasks/M3.md`, holding leaves with hierarchical
  identifiers (`M3`, `M3.5`, `M3.5.2`, `M3.5.2.1` …) and, around them, sections: the goal, a frontier table, decisions,
  a verification log, a commit log.
- **Leaf** — one unit of work. Textually, its identifier line (`- ID: \`M3.5.2.1\``) and every line after it up to
  the next identifier line or section heading: its **span**.
- **Done** — the leaf's status field says so.
- **Subtree of a leaf** — the leaf and every leaf whose identifier extends its own (`M3.5.2` holds `M3.5.2.1`,
  `M3.5.2.1.4` …).
- **Closed subtree** — one whose every leaf is done.
- **Outermost closed subtree** — a closed subtree whose nearest ancestor leaf is not closed (or that has none).
  Two closed leaves with no leaf between them and an open ancestor seal apart, each its own unit.
- **Seal** — the operation that moves units out of a tree.
- **Sealed file** — one per unit, holding its leaves' spans exactly, in tree order, with no header.
- **Stub** — two lines left where a sealed leaf stood: its identifier, and a done status that links the sealed file
  and names the leaf's commit.
- **Index** — one table per tree, one row per sealed file: unit, leaf count, lines, bytes, digest, date. Append-only.
- **Log rows** — the rows of a tree's verification and commit logs; those naming a sealed leaf are sealed too
  (§4.7).
- **Gate** — the check that re-proves all of the above on every commit.

---

## 4. The rules

The words MUST, MUST NOT and SHOULD are used as in RFC 2119.

### 4.1 What seals: the outermost closed subtree

- A seal MUST take each outermost closed subtree whole — all its leaves together, in one sealed file.
- It MUST NOT seal part of a closed subtree apart from the rest, nor seal together leaves that no single leaf holds.
- A leaf with an open leaf below it MUST stay live; its finished children seal as their own units.

*Why the outermost, and not only top-level subtrees:* archogen first sealed only top-level subtrees, assuming they
close soon after their parts. They do not when one leaf waits — on a decision, a person, hardware. A top-level
subtree stayed open for weeks while one leaf waited on the director, and every finished leaf below it stayed live:
274 483 bytes of done leaves under open top-level subtrees, 261 610 of them in 28 subtrees closed below the top
level, with the task-tree folder at its ceiling (§11, amended `2026-10-10`). The cost of the finer unit: one
feature's history can span several sealed files — its own and one per part that closed earlier. Each is still one
closed subtree, whole, and the stubs, in tree order, say where each leaf is.

*Why a subtree and not a leaf:* sealing leaf by leaf either changes a sealed file as its siblings close, or scatters
one feature over many files. A closed subtree seals once and never changes.

### 4.2 Where sealed files go

- One file per unit, at a path derived from the tree and the unit, for example
  `docs/task-history/<TREE>/<UNIT>.md`.
- The archive MUST live **outside** any folder that the project's live-document checks scan as new text. Archogen
  chose `docs/task-history/` rather than `docs/tasks/sealed/` because checks reading `docs/tasks/` would have judged
  every moved line as added, and its rolling ledgers' folder had a shorter line ceiling (§8.3).
- A path already taken MUST be refused before any write.

### 4.3 The stub

```text
- ID: `M1.13.2`
  Status: `done` — sealed in [`M1/M1.13.md`](../task-history/M1/M1.13.md); commit `ARCHOGEN-M1-0082`
```

- The commit named is the first work-unit identifier the leaf's commit field names; failing that, "in the tree's
  commit log" when the log has a row for it; failing that, "not recorded". The seal SHOULD warn about a done leaf
  whose commit field names no commit (§8.5).
- The identifier and status lines are what every check that reads a leaf needs; nothing else stays.

### 4.4 What a sealable leaf looks like

A leaf's span MUST be its identifier line followed only by indented or blank lines. A line at column 0 inside a
leaf — a fence, a heading, prose — would be torn by the line slicing every check shares, so the seal MUST refuse
such a leaf, naming each offending line, and write nothing.

### 4.5 What stays live

The tree's root leaf, every open leaf, every leaf with an open leaf below it, and every section around the leaves:
goal, frontier, decisions, logs (but for sealed rows, §4.7), changelog.

### 4.6 The index

- One table per tree, one row per sealed file: unit, leaf count, lines, bytes, digest (archogen uses SHA-256), date.
- Rows MUST only be appended. A row, once committed, never changes and is never removed.

### 4.7 Log rows follow their leaf

A tree's verification and commit logs grow one or more rows per leaf. Once the leaves were sealed, those rows were
archogen's next largest mass: 433 254 bytes of rows naming sealed leaves, with the task-tree folder at its ceiling
(§11, `2026-10-20`). The rule:

- every log row whose leaf cell names exactly one leaf that stands in the tree as a stub — and that the committed tree
  holds — moves, byte for byte and in order, into a numbered row file per tree (`rows-0001.md`, `rows-0002.md` …),
  one or more per seal under a size budget, each indexed like a sealed file;
- a row naming an open leaf, a done leaf not yet sealed, no leaf or several stays live;
- nothing is left where a row stood — no pointer row, which would keep most of the row's bytes and say nothing the
  leaf's stub does not.

### 4.8 Never reopened

- A sealed leaf MUST NOT be edited, and MUST NOT own a new change. New work under a sealed heading is a new leaf
  outside the sealed subtree (a sibling, if the sealed part sits under an open parent).
- A live leaf inside a sealed subtree MUST be refused.

### 4.9 Ceilings agree

If the project bounds its files (lines, bytes, longest line), the archive's ceilings MUST admit everything a live
tree may hold — otherwise a leaf that met every ceiling of its tree cannot be sealed byte for byte. Archogen learned
this when a seal wrote a leaf with 2 871-byte lines into an archive whose line ceiling was 2 048, while the trees'
was 3 072; the archive's was raised to the trees' (§11, `2026-10-06`). The archive's file count and total are
unbounded by design: it grows with what is finished, and no mandatory read includes it.

---

## 5. The seal, step by step

What a sealing tool does, in any language:

1. **Read the tree as committed and as it stands.** A unit is judged closed against the *committed* tree: a leaf
   marked done only in the working copy is not yet sealable (§8.4).
2. **Find the units** (§4.1): every outermost closed subtree not already a stub.
3. **Check every span's shape** (§4.4); refuse with the offending lines named.
4. **Compute the outputs** in memory: each sealed file (the spans, in order), the new tree (spans replaced by stubs),
   the moved log rows and their files, the index rows (counts, sizes, digests).
5. **Prove the reconstruction**: expand every new stub back into its span and every moved row back into its place;
   the result MUST equal the input tree byte for byte. Check that each sealed file holds exactly its unit's spans.
   If any check fails, write nothing.
6. **Refuse unsafe destinations**: a path already taken; a path or a folder above it that is a symbolic link; a
   temporary path already occupied.
7. **Write fail-closed**: note each write before making it; create new files exclusively; rewrite the tree and the
   index whole (temporary file, then rename) or not at all.
8. **Run the gate** (§6) on what was written. If it refuses — or if any error interrupts the seal — undo every write,
   each step on its own, and say exactly what could not be undone.
9. **Hold interruptions** that can be held while writing and proving, so the seal ends either fully done or fully
   undone; report which.

Archogen's tool reached steps 8 and 9 over 37 independent review rounds (§8.8). A first adoption can stop at a
smaller, still fail-closed version (§7.3).

---

## 6. The gate

Run on every commit — in the pre-commit hook and in CI, so a commit made without the hook is still caught. Its legs,
as archogen numbers them:

1. **Each sealed file matches its index row**: leaf or row count, lines, bytes, digest; and it holds at least one leaf.
2. **Files and rows correspond one to one**; nothing else lives in the archive; row files are numbered without gaps.
3. **Across history**: every index row any committed version ever held is still present and unchanged, and every
   sealed file is byte for byte what the commit that added it wrote. Read the full history, merges included; refuse
   a shallow clone rather than judging part of it.
4. **Stubs and bodies agree**: every sealed leaf has exactly one stub, in its own tree, done, linking its file; every
   stub links a file that holds its leaf; each leaf sits in its own unit's file.
5. **Provenance of leaves**: every sealed leaf is byte for byte the leaf its tree held just before the sealing commit,
   done there, and sealed with the rest of its outermost closed subtree; the file is exactly those spans in tree
   order. A seal made by hand passes only if it is byte for byte what the tool would make.
6. **No live leaf in a sealed subtree**, at any depth, in any tree file.
7. **Provenance of rows**: every sealed row was a row of the same log just before the sealing commit, in order; none
   dropped, none invented; each names a leaf that is a stub at that commit and was done before it.

The gate also SHOULD make other checks seal-aware: a check that finds a commit's owning leaf by identifier should
say "closed and sealed — a change needs an open leaf" when it lands on a stub.

---

## 7. Adopting it

### 7.1 Prerequisites

- Task trees in plain text with a recognisable leaf delimiter, hierarchical identifiers and a status field.
- Version control with commit hooks and CI that runs the same checks.
- A place for durable decisions, so the adoption itself is recorded with its measurements.

### 7.2 Steps

1. **Measure first.** For each tree: total size, and the bytes in done leaves' spans, under open and closed parents.
   Archogen's tool has a census mode for this. Seal only when finished work is the larger part, or a ceiling presses.
2. **Fix the grammar** of a leaf, its span, its status and its commit field in one written place (§3, §4.4).
3. **Choose the archive location** outside every folder your live checks scan (§4.2), and set its ceilings to admit
   anything a tree may hold (§4.9).
4. **Write the seal** (§5) and **the gate** (§6), with tests that make each rule fail when removed.
5. **Wire the gate** into the commit hook and CI, the same script in both.
6. **Seal one tree**, review the result — every digest, every stub, the reconstruction — and commit it alone or with
   the leaf that ran it.
7. **Write the rule into the project's working instructions** — the file every agent and contributor reads first —
   in one line: after a subtree closes, run the seal and commit its result; a sealed leaf is never edited.
8. **Record the decision** with its before-and-after measurements.

### 7.3 A smaller first version

A project can adopt the doctrine in two steps without weakening its promise:

- **First:** units (§4.1), stubs (§4.3), sealed files and an append-only index (§4.2, §4.6), the reconstruction proof
  before any write (§5 step 5), and gate legs 1, 2, 4 and 6. This already makes sealing lossless and the archive
  self-checking.
- **Then:** history-wide immutability and provenance (legs 3, 5, 7), log-row sealing (§4.7), and the hardened writes
  of §5 steps 6 to 9.

### 7.4 Harness and agent independence

- Every rule above is a property of files and commits. The gate runs where commits are made and where CI runs; it
  does not care which tool or person made the commit.
- An agent needs one instruction (§7.2 step 7) and a way to run a script. A harness that cannot run scripts still
  meets the gate when its commit reaches CI.
- Reading history needs nothing: open the stub's link, or the index.

---

## 8. Lessons archogen paid for

1. **The top-level unit was too coarse** (§4.1). A long-waiting leaf kept a whole subtree's finished work live.
2. **Each seal proves its own stubs.** The stubs earlier seals wrote stay proven by those seals and by the gate's
   history leg, so a new seal neither re-derives nor rewrites an older one — a review finding, not a design
   foresight.
3. **The archive's location matters as much as its format.** Moved text inside a scanned folder is "new" text to every
   check that looks for additions (§4.2).
4. **Seal the committed tree.** A seal judges closedness against the last commit. Close a leaf, commit, then seal —
   or seal in the next leaf's commit. If the edits are already made: set the edited tree aside, restore the committed
   one, seal, and re-apply the edits, with log rows appended at the ends of their tables, since the seal moves the
   rows the edits were anchored to.
5. **Fill a parent's commit field when it closes.** A parent closed with its fields left pending seals as "commit not
   recorded" — faithfully, and forever, since a sealed body is never edited. Archogen has two such stubs; the one
   found on `2026-10-23` has its commit named in a later leaf's record instead.
6. **Seal the log rows with their leaves** (§4.7) — and never by moving text by hand into an unbounded file, which only
   relocates the growth.
7. **Ceilings must agree** between the trees and the archive (§4.9).
8. **A seal's tool is infrastructure that protects evidence; review it like any.** Archogen's review ran 37 rounds
   before closing at zero findings. The tool's own findings were mostly in failure handling — atomic writes, rollback,
   symbolic links, signals — and in the gate's reading of history, not in the happy path; the later rounds also built a
   checker of the review's own counts.

---

## 9. What it does not do

- It does not shorten an open leaf. A leaf too long to read is a leaf to split, not to seal.
- It does not bound the archive. The archive is terminal history: immutable, indexed, never a mandatory read.
- It does not judge whether a leaf is truly done. It trusts the status field; the project's acceptance rules decide
  what "done" requires.
- It does not replace a changelog or the commit history. It keeps the evidence each leaf wrote, where its stub points.

---

## 10. Checklist

- [ ] Trees measured; finished work's share recorded.
- [ ] Leaf grammar, span, status and commit field written down.
- [ ] Archive outside every scanned folder; its ceilings admit any live line and file.
- [ ] Seal: outermost closed subtrees, committed tree, span shape, reconstruction proof before any write, fail-closed
      writes, gate run after writing, rollback.
- [ ] Gate: legs 1, 2, 4 and 6 at least; 3, 5 and 7 when ready; in the commit hook and in CI.
- [ ] Each rule's test fails when the rule is removed.
- [ ] Log rows of sealed leaves sealed with them.
- [ ] One-line rule in the instructions every contributor and agent reads first.
- [ ] Decision recorded with before-and-after measurements.

---

## 11. Sources in this repository

- The trigger and the ruling: [`decision_findings-for-director-review.md`](decisions/decision_findings-for-director-review.md)
  §8, sealed in [`decision-history/decision_findings-for-director-review/08.md`](decision-history/decision_findings-for-director-review/08.md)
  (`2026-09-30`).
- The rules, the tool and the gate, with the amendment to the outermost unit (`2026-10-10`) and 37 review rounds:
  [`decision_task-tree-sealing.md`](decisions/decision_task-tree-sealing.md).
- Log rows: [`decision_task-log-row-sealing.md`](decisions/decision_task-log-row-sealing.md) (`2026-10-20`).
- The line ceiling: [`decision_task-history-line-ceiling.md`](decisions/decision_task-history-line-ceiling.md)
  (`2026-10-06`).
- The lifecycle class it applies, `archive_terminal`:
  [`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`](../LIVE_DOCUMENT_SIZE_CONTAINMENT.md).
- The implementation: `scripts/check_task_history.sh` (`--seal <TREE>`, `--census <COMMIT>`, the gate), and the
  archive with its index: [`task-history/INDEX.md`](task-history/INDEX.md).
- **Measured `2026-10-23`**, at `b9d52e1c`: 394 files and 3 248 964 bytes in the archive against 554 725 bytes in the
  live trees; `M1` from 7 633 lines before its first seal (`2026-09-30`) to 989, 71 of its 76 leaves stubs; `M3` 2 574
  lines, 339 of its 404 leaves stubs.
