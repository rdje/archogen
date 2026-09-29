# The scaffold updater is upstream's, and it never overwrites

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.26`. Adopted, not re-derived, from the read-only `bedrock` template at revision
  `5af0c1c5b9cc2a65f51ecda1c0937fbc234c84da` (`bedrock-scaffold 0.10.0`), whose own leaves `BEDROCK-MAINTENANCE.2.10`
  and `.2.11` made the change after a sync wiped a project's task-tree index on `2026-09-21`
- **External sources:** [bedrock](../book/src/ledger.md#bedrock) — version, scope and limits in the ledger

## The fact / decision

`scripts/update_scaffold.sh` is `bedrock` `5af0c1c`'s file with **one** local hunk: its scratch directory is made
under `target/doctrine_scratch/`, as `SCRATCH-LOCALITY` requires, and not with a bare `mktemp -d`. The file is not
in its own `NEUTRAL` array, so it is project-owned, and the next adoption must carry that hunk forward.

What the adopted updater does to a file that differs from the template: **nothing**. The template's copy goes to
`.bedrock-incoming/<path>`, which is ignored by git, and is reported. With `--merge` it offers a three-way merge into a
side file, `.bedrock-incoming/<path>.merged`, and never into ours. An absent file is seeded, a `SEED_ONCE` file the
project has changed is kept, and a dirty tree is refused.

The same commit brought the spine to `0.10.0`. `DOCTRINE_VERSION` says so, and `VISIBILITY.md` was seeded.

## Why

- **The version it replaced copied over project content**, one unconditional `cp` for each of the 29 files in its `NEUTRAL` array.
  Seven of those files carry this project's own content. A `make update-scaffold` would have replaced the
  task-tree index, the tiered commit workflow and the tool registry with the template's text, and `make gate`
  would have passed afterwards.
- **Adopting the fix, not re-deriving it.** Upstream measured the failure and fixed it. A local rewrite would be
  one more fork of a portable tool.
- **The census, `2026-09-30`.** Each neutral file was compared with upstream `HEAD` and with `8222e97`, the
  revision that introduced the `0.8.1` this project recorded:
  - **21 identical** to upstream;
  - **7 carrying project content**, where only this project moved: `AGENTS.md`, `COMMIT.md`, `TOOLBOX.md`,
    `DOCTRINE_ENFORCEMENT.md`, `docs/TASK_TREE.md`, `.doctrine/README.md` and `scripts/check_task_acceptance.sh`;
  - **behind**, only `DOCTRINE_VERSION`, plus the updater itself, which both sides had moved.

  Between `8222e97` and `5af0c1c`, upstream's spine changed nowhere else. That is why adopting the updater brings
  the neutral set to `0.10.0` and why `DOCTRINE_VERSION` can say so truthfully. It matters mechanically too: the
  updater resolves its merge base by searching upstream's history for this exact string (`git log -S` →
  `6bbbf82` for `0.10.0`), so a suffixed version such as "0.8.1 plus the updater" would leave every future merge
  without a base.
- **`VISIBILITY.md` is carried out, not decided.** It is a `SEED_ONCE` file that was already in `0.8.1` and never
  reached this project, because the old updater had no seed-once category. It declares the posture *public* for
  every project spawned from the template, "unless that project deliberately changes the line", by the
  maintainer's instruction of `2026-09-21`. That matches the host: `gh repo view` reports `rdje/archogen` as
  `PUBLIC`. The director can change the line; this project has not.

## The dry run, measured

The updater ran on a clone of this repository with the adopted updater staged, against `../bedrock`:
`✓ 21 already current, 1 seeded, 8 differ, 0 merged to a side file — nothing of yours was modified`. The seeded
file was `VISIBILITY.md`. The eight that differ were the seven project-carrying files and `DOCTRINE_VERSION`, each
reported as "yours is UNTOUCHED". The clone's status afterwards: `?? VISIBILITY.md` and the ignored
`.bedrock-incoming/`, and nothing else.

## How to apply

- **To sync:** `make update-scaffold URL=<bedrock>` on a clean tree. Read `.bedrock-incoming/`, take what you want
  by hand, and delete it. The seven project-carrying files will always differ. The template's version is not
  automatically the better one: improvement flows both ways.
- **To adopt a newer updater:** copy upstream's file, re-apply the scratch hunk, and re-run the census and the dry
  run above. `scripts/selftest_spine.sh` arms it from outside: a project-carrying file survives a sync byte for byte,
  a seed-once file is seeded or kept, and a dirty tree is refused. All of those arms fail against the version this
  replaced.
- Related: [[decision_scratch-on-the-repository-volume]], [[decision_repository-boundary-read-only]] — `bedrock` is
  read, never written.
