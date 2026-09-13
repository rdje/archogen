<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

Responsibility boundaries follow `ROADMAP.md` §4.2. A crate is created when a real consumer
justifies the split — the rows below appear as that happens.

- `crates/eadl-front/` — the reader (§4.2): S-expressions, byte spans with character columns,
  and caret diagnostics carrying a repair direction. Purely syntactic and float-free; comments
  are kept because the boundary corpus carries its metadata in them.
  `cargo run -q -p eadl-front --example diagnose -- <file>` is the tool that answers "where did
  it stop?".
- `crates/eadl-model/` — the typed eADL model (§4.2): declarations, units, contract IDs and
  profile definitions. `src/profile.rs` holds `rt-static-up-v1` as data — 13 decisions, 18
  named exclusions — and its test fails if `docs/profiles/rt-static-up-v1.md` drifts from it.
- `crates/osgen-evidence/` — the evidence, claim and trust vocabulary (§7.1, §7.3, §4.4).
  `claim.rs` makes §7.1's "one global verified flag is prohibited" structural: no aggregate
  verdict type, a report that refuses to render while a property is unanswered, and no
  conclusion constructible without its qualifier. `bound.rs` keeps an observation an
  observation whatever the safety factor. `trust.rs` is the §4.4 root/role/drift vocabulary.
- `crates/osgen-cli/` — the `osgen` binary and its library. `src/spec.rs` declares the
  `ROADMAP.md` §10.2 command surface as data (help and parsing both derive from it);
  `src/status.rs` is the §5.5 outcome vocabulary and the stable exit-code contract;
  `src/cli.rs` is the dependency-free parser and help renderer.
- `ROADMAP.md` — the program's direction, milestone exit gates, and the F01–F30 acceptance
  matrix. The single source of what "done" means.
- `docs/tasks/PROGRAM.md` — the roadmap-unit → tree map and the fixture-ownership map.
  Start here to find which tree owns a given roadmap item.
- `docs/semantics/boundary/` — the 21-case boundary corpus (accept/reject pairs, 5 ambiguous),
  the input fixture F27 mechanizes. Its README fixes the case format.
- `docs/usecases/` — the four systems the toolchain must build or refuse; `docs/evaluation/` —
  the sealed reuse-measurement set and the check that keeps it sealed and unseen.
- `targets/riscv-virt-up.env` + `scripts/target_emulator.sh` — the pinned emulator
  configuration and the only tool that renders it. `--check` exits 20 when QEMU is absent
  rather than reporting a skipped check as a pass. `docs/targets/first-target.md` records the
  board decision: none procured.
- `scripts/check_doctrines.sh` — the doctrine enforcer (git hook + CI). `make gate`.
  Project doctrines live in `scripts/check_doctrines.project.sh`.
