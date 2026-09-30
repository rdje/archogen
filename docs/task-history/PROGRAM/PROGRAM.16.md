- ID: `PROGRAM.16`
  Status: `done`
  Goal: adopt the director-mandated **claim-verification policy** (§17 of the standing session
  instructions) into this repository under a repository-relative path, so the rule survives the
  session that carries it — the same failure `PROGRAM.11` was opened for, arriving from the other
  direction.
  Reproduce / issue: the policy is not here. `ls docs/CLAIM_VERIFICATION.md` → no such file, and
  `git grep -lni 'claim verification' -- '*.md'` → only `DEV_NOTES.md`, and only in the note that
  records this gap. THE GAP: a standing instruction is satisfied only inside a session prompt, so it
  is enforced nowhere in the repository and dies with the session. The read-only source is another
  repository's `docs/CLAIM_VERIFICATION.md` (285 lines, 18 166 bytes, read `2026-09-27`); §12's
  exception permits copying it **into** this repository and forbids writing to it. Measured
  consequence on the same day: leaf `M1.20.7` entered its reconciliation carrying an unverified
  premise about `docs/TASK_TREE.md` and caught it only by running the grep — exactly the behaviour
  the policy exists to require.
  Acceptance: the policy is copied to a repository-relative path with its provenance (source
  repository, path, date read) recorded **in the file**; nothing outside this repository is written
  to; the adoption is registered where a reader will find it — the `CLAUDE.md`/`AGENTS.md` pointer set
  and, for any mechanically checkable clause, `DOCTRINE_ENFORCEMENT.md`; the copy is compared against
  the source and any later change is applied or recorded as deliberately not applied with a reason;
  the leaf records which existing practices here already satisfy the policy and which it changes.
  Priority: **high** — it is the "rule enforced nowhere" shape this tree exists to eliminate, and the
  session that must follow it is the one that cannot see whether the last one did.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0058 (leaf PROGRAM.16)`
  promotion: declined (the lesson's canonical home is the adopted standard itself — §7 of
  `docs/CLAIM_VERIFICATION.md` *is* the adoption checklist, and it is now in this repository and in the
  bootstrap reading order, which is more discoverable than a knowledge card restating it. The local
  finding the sweep produced is owned as work, not prose: `PROGRAM.18`.)

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the repository's own bootstrap lists four spine
    documents and the policy is not among them — `grep -n 'CLAIM' CLAUDE.md` → no match at the parent
    commit — and `git grep -ni 'portable architecture' -- '*.md'` → no match, `rc=1`, so nothing here
    even had the vocabulary the policy uses to describe itself as the fifth. WHY it matters: the
    policy is the one that governs whether the *other four* are trustworthy, since a gate built on an
    unverified measurement enforces the wrong thing precisely and forever. It had been followed in
    practice all day — every leaf box cites a command, every new instrument carries RED arms — and
    followed for exactly the reason `PROGRAM.11` was opened for: it lived in a session prompt, so
    nothing in the tree would have noticed if a session stopped.
  - [x] **ADDRESSED (verified)** — `docs/CLAIM_VERIFICATION.md` now exists, 406 lines / 27 263 bytes:
    a 121-line adoption record (provenance, the §A local restatement, the §B sweep) followed by the
    policy body. **The body was copied, not retyped, and the copy was verified rather than read:**
    `tail -n +122 docs/CLAIM_VERIFICATION.md | diff -q - <source>` → identical, `rc=0`, and both sides
    digest to `9f99df25209c43af…`. A hand-copied policy would have been an unverified transcription of
    the document that defines verification. Registered where a reader meets it: `CLAUDE.md`'s spine
    sentence now names it and its reading order gains step 5, and `DOCTRINE_ENFORCEMENT.md`'s E1
    discovery list carries it with the sibling relationship stated. Nothing outside this repository was
    written to — the source was opened read-only, and `git -C <source-repo> status` was never invoked
    with a write intent.
  - [x] **NO REGRESSION** — the adoption adds one document and edits three; it changes no check, no
    gate and no build input. `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` on the
    staged set, still 13 doctrines (the policy is deliberately **not** registered as a doctrine: its
    §5 mechanizations are a separate decision, and adding a gate that nothing needs yet is how a
    registry accumulates checks nobody can explain). `make focused` → exit `0`; `cargo test --all` →
    **421 passed, 0 failed** over 36 suites. The new file carries no checkout-specific absolute path
    (`DOCPATH` green is the proof, and the source's own examples are domain-free), and
    `check_table_arity`'s ratchet accepts its three new tables because every row matches its header.
  - [x] **FIX** — copy plus restate plus sweep, which is what §7 of the policy itself asks of an
    adopter, in its own order. §A restates all three legs in archogen's terms with **this
    repository's** measured instances rather than the source's — the vacuous `xargs sha` digest that
    would have made "regenerated" indistinguishable from "UNCHANGED", the classifier that reported a
    symptom over a clean run, the leaf premise that a grep disproved — because §7.6 says a rule you
    cannot restate in your own terms is under-specified for you. §B records the sweep, and step 5 is
    adopted **by mapping**: the leaf acceptance box *is* this repository's claim tag and is already
    gated, so a second inline tag syntax is deliberately not added.
    ⛔ **The sweep found a real gap and it was filed, not footnoted.** Step 4 — fire every control —
    measured 18 registered checks, 8 with RED arms (all passing) and **10 without**, including
    `check_task_acceptance.sh`, whose box-scoping was validated once during development and is
    re-fired by no arm. `PROGRAM.18` owns it, medium-high, that gate first. Publishing the adoption
    while hiding that result would have been the exact failure the policy describes.
    ⛔ **`GAP-CLAIM-CENSUS` blocked the first commit attempt, correctly.** The frontier row summarising
    `PROGRAM.18` said the property "is re-fired by nothing" — a whole-tree quantifier in a section
    carrying no census command, which is exactly the shape that doctrine exists to stop, and the census
    was one section away in the leaf. Reworded to name the missing `--self-test` arm: the same fact,
    stated as a count rather than as an absolute, and the count is the thing the leaf measured.
  - [x] **LOCKSTEP** — `docs/CLAIM_VERIFICATION.md` (new); `CLAUDE.md` (spine sentence + reading
    order); `AGENTS.md`; `DOCTRINE_ENFORCEMENT.md` (E1 list + the sibling question); this leaf,
    `PROGRAM.18`, the frontier, the Children line and both logs; `MEMORY.md`, `LIVE_STATUS.md`,
    `CHANGELOG.md`, `DEV_NOTES.md`.
    ⛔ **`AGENTS.md` was nearly missed, and the draft of this box is why it is recorded.** The first
    version of this leaf asserted that `AGENTS.md` "names the discipline documents generically, so it
    inherits the addition — verified by reading it rather than assuming". Reading it says the
    opposite: it carries an **explicit** list — `grep -n 'MEMORY_ARCHITECTURE' AGENTS.md` → line 11,
    `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`, and `COMMIT.md` — so a fifth
    spine document that is not added there is invisible to every harness that reads `AGENTS.md`
    instead of `CLAUDE.md`. The list now carries `docs/CLAIM_VERIFICATION.md`. The claim was written
    from the shape of the file remembered at session start, not from the file; leg 1 applies to a
    leaf's own prose about the repository exactly as it applies to a number.
    No book chapter changes: the book documents eADL and the engine, and this is spine documentation —
    `git grep -ln 'CLAIM_VERIFICATION' -- docs/book` → no match, `rc=1`.
