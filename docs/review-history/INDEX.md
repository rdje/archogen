# docs/review-history/INDEX.md — closed review histories archived out of docs/reviews/

Each file below is a review history whose review closed, moved here byte for byte by
`bash scripts/check_review_history.sh --seal <FILE>` and never edited again (`PROGRAM.55`,
`docs/decisions/decision_reviews-folder-ceiling.md`). A three-line stub stays at its old path in `docs/reviews/`,
linking here, so a citation of it still finds it. A row records the file's lines, bytes and sha256, and the day it
was archived. The rows are append-only, and `REVIEW-HISTORY` checks every file against its row, its stub and the
history it came from, on every commit.

To prove a file, compare `sha256sum docs/review-history/<FILE>` with its row.

| File | Lines | Bytes | sha256 | Archived |
| --- | --- | --- | --- | --- |
| `decision_catalog-records-reviews.md` | 541 | 75383 | `d4fbec48d178a9c74e3e47de1df287f9ac6092a98208449bc7f85a20fa3cd811` | `2026-10-05` |
