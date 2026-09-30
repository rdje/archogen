- ID: `PROGRAM.25`
  Status: `done`
  Goal: **own the two findings raised in conversation on `2026-09-28`/`29` and left unowned.** §15 is
  explicit that reporting an issue for the director to review, or mentioning it in a summary, is
  incomplete — an issue raised and not owned is a complaint. Both of these were raised in a reply and
  recorded nowhere in the tracked tree, so if the session ended they would have ended with it.
  Reproduce / issue: two separate failures of the same rule, and the second one is the worse shape.
  **(1)** Answering a question about the target ISA, `read_file` on
  `$ARCHOGEN_CHIPDOC_ROOT/sifive/fe310/current/FE310-G002_datasheet_v1p2.pdf` returned
  `pdftotext is not installed. Install poppler-utils…`, and that was reported onward as "this machine
  has no PDF text extractor, so §3.2's board facts cannot be read here" — with a promise to log it as
  its own commit. ⛔ **The premise is false, and measuring before filing is what caught it:**
  ```text
  $ command -v pdftotext pdftk mutool qpdf gs
  /opt/homebrew/bin/pdftotext
  /opt/homebrew/bin/pdftk
  $ pdftotext -v | head -1
  pdftotext version 4.06 [www.xpdfreader.com]
  $ pdftotext -f 1 -l 6 <that datasheet> - | head -1
  SiFive FE310-G002 Datasheet v1p2            ← the route works
  ```
  So the host has an extractor (Xpdf's, not poppler's) and the **`read_file` PDF bridge cannot see the
  host `PATH`**. The consequence is the *opposite* of the one reported: chipdoc's board PDFs **are**
  readable here via `run_shell_command`, and §3.2's `board-first` rows are **not** blocked on tooling.
  Census of what that unblocks: `sifive/fe310/current` holds **3** PDFs and `sifive/hifive1/current`
  **2**, all named as *present* by `docs/decisions/reference_external-document-source-chipdoc.md`, which
  says nothing about whether they can be read — so the record needs the route as well as the inventory.
  **(2)** Told that a sibling repository `../semulith` exists, the reply analysed it and then, on being
  told the analysis was not what was asked for, deleted it and reported "nothing filed, nothing changed
  in the repo". Correct response to "don't analyse it"; wrong response to §9, which requires a novel
  finding to be recorded in a **durable, tracked file** so it survives the message. Census, both
  directions, so the claim is checkable:
  ```text
  $ git grep -il "semulith" -- ':!vendor' | wc -l
  0                     ← archogen's own tracked files name it nowhere
  $ grep -ril "semulith" vendor/ | wc -l
  40                    ← the vendored LinkedSpec submodule names it, as a fellow consumer
  ```
  ⛔ Note the census trap this records: `vendor/linkedspec` is a **submodule** (`.gitmodules`), so
  `git grep` skips it entirely and a `git grep` alone would have reported "nothing anywhere mentions
  it" — the two projects have already met, through the vendor's own issue ledger, and archogen's side
  of that meeting never says so.
  Acceptance: `TOOLBOX.md` carries the PDF route with the misreport named, so the next session does not
  re-conclude the datasheets are unreadable; `reference_external-document-source-chipdoc.md` gains the
  readability fact beside its inventory; a `docs/decisions/reference_*` record states what `semulith`
  is, that it is **read-only** under §21, and which archogen leaves would need to know it exists —
  with no analysis of it, which is what was declined; the record is indexed; `KNOWLEDGE_MAP.md`
  regenerated; `make gate` green.
  Priority: **medium** — neither finding blocks the current frontier. Filed ahead of it anyway, because
  the alternative is a false statement ("no PDF extractor here") standing as the last word in a
  conversation nobody can search, and a sibling project that names archogen as its consumer while
  archogen's tree has never heard of it.
  Verification: docs-only, so no tier step governs the change itself; what was measured is in the
  Reproduce block above and was re-run rather than recalled — `command -v pdftotext` →
  `/opt/homebrew/bin/pdftotext`, `pdftotext -v` → `version 4.06 [www.xpdfreader.com]`, and
  `pdftotext -f 1 -l 6 <the FE310-G002 datasheet> -` → its first line, so the route is proven and not
  assumed. Both censuses re-run: `git grep -il semulith -- ':!vendor' | wc -l` → **0**,
  `grep -ril semulith vendor/ | wc -l` → **40**, and `.gitmodules` confirms `vendor/linkedspec` is a
  submodule, which is why the first census alone would have been a false negative. Every citation in
  the new record resolves (`decision_repository-boundary-read-only`,
  `decision_emulator-independence-retained`, `reference_external-document-source-chipdoc`,
  `an-oracle-is-independent-by-construction`), and the semulith path is written as `../semulith` rather
  than an absolute one so §12 holds if the repository moves volume. `make gate` → `13/13 green`;
  `KNOWLEDGE_MAP.md` regenerated for the new record.
  Commit: `ARCHOGEN-PROGRAM-0083 (leaf PROGRAM.25)`
