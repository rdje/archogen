# docs/decision-history/INDEX.md — settled sections sealed out of decision records

Each file below holds one numbered section of a decision record, moved here byte for byte by
`bash scripts/check_decision_history.sh --seal <RECORD> <N>...` once it was settled, and never edited again
(`PROGRAM.41`, `docs/decisions/decision_decisions-folder-ceiling.md`). Its heading stayed in the record, above a
one-line stub that links here, so a citation of the section by number still finds it. A row records the file's lines,
bytes and sha256, and the day it was sealed. The rows are append-only, and `DECISION-HISTORY` checks every file
against its row, its stub and the record it came from, on every commit.

To prove a file, compare `sha256sum docs/decision-history/<RECORD>/<NN>.md` with its row.

| Record | Section | Lines | Bytes | sha256 | Sealed |
| --- | --- | --- | --- | --- | --- |
| `decision_findings-for-director-review` | `02` | 24 | 1578 | `f145069a0ab90ff207546352ef9155c68951ba1930831407a9b0e0ae26e2feaa` | `2026-09-30` |
| `decision_findings-for-director-review` | `04` | 21 | 1543 | `e3a309b27e3e91b95b1fd27bee6b6db02be43a1971725518036e6d328d94ec31` | `2026-09-30` |
| `decision_findings-for-director-review` | `08` | 47 | 3857 | `121aff31e0b4ab4b8162f630506bc33303bd41057a13b1240707a3677b06a80b` | `2026-09-30` |
| `decision_findings-for-director-review` | `10` | 44 | 2767 | `9591881df3c96e96aa0fd96ea06a7dbce7d56463688e228bb1b3e059c963e752` | `2026-09-30` |
