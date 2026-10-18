- ID: `PROGRAM.90`
  Status: `done` — recurrent placement repaired and live working/indexed tables enforced
  Goal: preserve prior row text and headed order, repair displaced records, and prevent recurrence.
  Reproduce/root: f0fd3fd and97189a0 inserted exact .89.1/.89.2 rows above Metadata. The closure
    adapter matched an inline heading mention in .87, found no table row (-1), and used the first
    newline as fallback insertion. Existing presence-only gate accepted this. Initial live census
    found only these two misplaced commit records across docs/tasks; all other live records headed.
  - [x] **ROOT CAUSE** — actual native Git introductions/7130730 lookup replay in root-cause.json;
    native old gate exit 0 with both records misplaced. Maintained placement extension then exits 1
    naming both preamble records, retained maintained-red.log. First draft also refused M3's legitimate
    pre-table scope note; root-bounded table search and a permanent positive control correct that.
  - [x] **ADDRESSED** — exact row relocation restores native introduction order after all205 headed
    prior rows. python3 -I -B target/program-log-placement/audit.py, exit 0: actual pinned renderer
    before205/after207, separated-row negative renders1; full commit-row Counter, headed sequence and
    verification rows preserved. Gate requires one unfenced root heading, three-column header/delimiter
    and contiguous live records in working and indexed copies. Scope notes/inline mentions pass;
    missing/fenced/duplicate headings, preamble/Changelog/separated records, delimiter omission and
    an indexed bad table hidden by a working correction refuse. Existing presence/backlog cases hold.
  - [x] **NO REGRESSION** — bash scripts/check_commit_log_rows.sh --self-test, exit 0:
    21 pass / 0 fail (21 arms), including all prior9. Native byte checks preserve250 tracked archive/
    index files and engine/proof/catalogue/native sources. Closure adds only this unit's headed rows;
    final actual render/pinned book/staged gates required. No Rust source/focused run warranted;
    existing structural verification method applies, no new DEV_NOTES lesson. Full integration is
    still M3 .13.3.2. A separate actual probe confirms an unstaged-only pending row passes; PROGRAM.91
    owns that P1 indexed-presence gap and is next before integration. Frozen archive scope stays exact.
  Verification: `2026-10-18`, target/program-log-placement retains native root/introductions/census,
    old gate acceptance/new refusal, actual full-document render fixtures, source/row preservation,
    complete self-test and index-presence-probe.json. Ignored extraction adapters supplement native
    Git, pinned renderer and maintained gate. Pre-closure audit.json SHA256
    e50e64ea5fa6d1a1af0aac2bd6aae0bbb00149c27c792749425899ddd8f99262.
    Final pinned render205 to208 including this unit; separated-row negative1; all prior rows/archives
    preserve. Final audit SHA2561cbecf0b0225a99d23701d8ce3fc967c9bd758b7c56357931ee475d64cb32245.
    Actual full pinned book exits 0; staged gates bind the final unit record separately.
  Commit: `ARCHOGEN-PROGRAM-0735 (leaf PROGRAM.90)`
