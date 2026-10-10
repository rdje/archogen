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
| `rt-static-up-v1-faults-reviews.md` | 384 | 44524 | `5df5e66854f18b152bdc544d3af856f88986804112404e17ce7e603aa1ced7b9` | `2026-10-06` |
| `decision_decisions-folder-ceiling-reviews.md` | 49 | 4560 | `590b12df8d1d64461b13121481d9c960eddb61d4afb04dc0c359887f1a4d8802` | `2026-10-10` |
| `decision-history-reviews.md` | 50 | 5379 | `6f85f76ee26efc72dcdaeeee8b8205268f0bfb522dfd3007a348138d37d592f9` | `2026-10-10` |
| `review-history-reviews.md` | 47 | 5040 | `6480f641ef6c9e11d1bb0c420bb221daa6b84492fe5374d8125a659f7c8f944c` | `2026-10-10` |
| `decision_catalog-records-port-reviews.md` | 220 | 28075 | `3ba9dd5d3d70a6b56cef1ca49621899835ec8e6d42c185ade7012c74e9beee99` | `2026-10-10` |
| `decision_catalog-records-port-statement-reviews.md` | 255 | 31550 | `ab2d8a6ede55bc579d2516ffa528c74b2d19f29badc9c7989301b655396497cf` | `2026-10-10` |
| `decision_runtime-composite-inputs-reviews.md` | 297 | 34860 | `81443a606b74fe913f7528c103b83ec680d4d5bac14bacc264c0d075e510f0a6` | `2026-10-10` |
| `rt-static-up-v1-faults-observation-reviews.md` | 195 | 21087 | `16a11c4162bc141d65c97de6bfbb87fee66b6c9c53fa75e04761983ca579a53f` | `2026-10-10` |
| `decision_substitutability-relation-reviews.md` | 674 | 91093 | `bfdba54dfcaaa28ac9a9ed39cf3ea21260d85ef12cbdb39096ca2a94e25b40a2` | `2026-10-10` |
| `decision_trust-inventory-reviews.md` | 311 | 38300 | `23b043196156b3db33ed0a726a5646343926f7c0a3152257b47a62ccf8f0e764` | `2026-10-10` |
| `decision_trust-generated-sources-reviews.md` | 438 | 55702 | `047051cb5759a5fc084e6fd70704909e2c75e98745ff8549720ddfa9b598716e` | `2026-10-10` |
