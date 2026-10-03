# The trust-dependency inventory: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.1` (`docs/tasks/M3.md`). This is the review history of
  [[decision_trust-inventory]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M3.6.1`'s acceptance is a review, by a context that did not write the record, finding that the gate as specified
exercises each of `ROADMAP.md` §14.4's five cases, that no input a root's build reads can be shared without being
reported, and that the baseline cannot be accepted by its author. Each round below is one such review, with every
finding and the answer the record gives it.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `ROADMAP.md` §4.2, §4.4, §10.3 and §14.4;
- `docs/decisions/decision_zero-dependency-engine-core.md` and `decision_findings-for-director-review.md` §11;
- `xtask/src/catalog_build.rs`, which reads cargo's metadata and the compiler's dependency information today;
- the workspace's manifests, and cargo's and rustc's documentation for `cargo metadata` and dependency information.

It is barred from other implementation, and may check external documentation on the web.

## Rounds
