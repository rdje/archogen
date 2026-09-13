# archogen — generate a specialized operating system from a functional description

**archogen** is a deterministic toolchain that turns **eADL** descriptions of a platform, a
workload, services, and policy into a complete specialized operating system, a matching
development simulator, and a report stating what has been checked and under which
assumptions.

The controlling boundary: **eADL describes functionality — what a platform offers and what a
system requires. It contains no implementation.** Algorithms, device implementations,
register sequences, simulator models, provider selection, lowering rules, and code
generation belong to the engine and its versioned knowledge bases.

The first supported family is `rt-static-up-v1`: a small, single-core, statically configured
real-time executive. Broader OS functionality is admitted later through additional profiles
with their own contracts and acceptance evidence.

## Status

Early. The roadmap is seeded into task-trees and the discipline spine is enforced; the
engine is being built one milestone gate at a time. Nothing here claims a verified OS —
see the assurance model in the roadmap before reading any result as a guarantee.

Current progress: [`LIVE_STATUS.md`](LIVE_STATUS.md). Next action:
[`MEMORY.md`](MEMORY.md).

## Quick start

```bash
git config core.hooksPath .githooks   # once per clone — activates the doctrine gate
make focused                          # §14.3 focused tier: format, lints, the whole suite
make integration                      # §14.3 integration tier: + doctrines, book, emulator
make tiers                            # list every tier and what each step proves
```

A tier can exit **20 = incomplete**: nothing failed, and something could not be run. That is not
a pass — see [Verifying the toolchain](docs/book/src/verification.md).

## Architecture at a glance

```text
eADL description ─┐
                  ├─► elaboration ─► joint resolution ─► specialization ─► generated OS
target facts   ───┘        │              │                    │            + simulator
   + catalogs              └──────────────┴─► independent checker ─► assurance report
```

Responsibility names (crates appear when a consumer needs them): `eadl-front`,
`eadl-model`, `eadl-resolve`, `archogen-plan`, `archogen-emit`, `archogen-check`, `rt-analysis`,
`rt-core`, `arch-*`, `device-*`, `sim-*`, `xtask`. `archogen-s0` sits outside that list on
purpose: it is the temporary S0 prototype, with an expiry enforced by a doctrine check
([`decision_s0-retirement.md`](docs/decisions/decision_s0-retirement.md)).

## Where things live

| Topic | Canonical home |
| --- | --- |
| Direction, milestones, exit gates, acceptance matrix | [`ROADMAP.md`](ROADMAP.md) |
| Execution tracking (every change is owned by a leaf) | [`docs/TASK_TREE.md`](docs/TASK_TREE.md) |
| Durable decisions and facts | [`docs/decisions/`](docs/decisions/) |
| User-facing documentation | [`docs/book/`](docs/book/) |
| Release history | [`CHANGELOG.md`](CHANGELOG.md) |
| Engineering notes | [`DEV_NOTES.md`](DEV_NOTES.md) |

## Working in this repository

This project runs on a portable discipline spine, enforced at the git level (hooks + CI) so
it holds for any agent or human identically:

- Nothing changes without a **task-tree leaf** first ([`docs/TASK_TREE.md`](docs/TASK_TREE.md)).
- Durable facts go to [`docs/decisions/`](docs/decisions/); the resume pointer is
  [`MEMORY.md`](MEMORY.md) ([`MEMORY_ARCHITECTURE.md`](MEMORY_ARCHITECTURE.md)).
- Commit per [`COMMIT.md`](COMMIT.md); for any unknown, reach for a tool first
  ([`TOOLBOX.md`](TOOLBOX.md)). The enforced doctrines are listed in
  [`DOCTRINE_ENFORCEMENT.md`](DOCTRINE_ENFORCEMENT.md).
- Agent bootstrap: [`CLAUDE.md`](CLAUDE.md) / [`AGENTS.md`](AGENTS.md).

This README is a landing page, governed by [`README_POLICY.md`](README_POLICY.md) and
mechanically capped. Route changing detail to its canonical home above.

## License

MIT OR Apache-2.0.
