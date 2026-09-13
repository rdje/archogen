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
  it stop?". `src/module.rs` elaborates imports into a tree of INSTANCES — importing a module
  twice yields two, with independent bindings — and reports a cycle as its whole chain.
- `crates/eadl-model/` — the typed eADL model (§4.2): declarations, units, contract IDs and
  profile definitions. `src/profile.rs` holds `rt-static-up-v1` as data — 13 decisions, 18
  named exclusions — and its test fails if `docs/profiles/rt-static-up-v1.md` drifts from it.
- `crates/archogen-evidence/` — the evidence, claim and trust vocabulary (§7.1, §7.3, §4.4).
  `claim.rs` makes §7.1's "one global verified flag is prohibited" structural: no aggregate
  verdict type, a report that refuses to render while a property is unanswered, and no
  conclusion constructible without its qualifier. `bound.rs` keeps an observation an
  observation whatever the safety factor. `trust.rs` is the §4.4 root/role/drift vocabulary.
- `crates/eadl-model/src/check.rs` — the frontend pipeline `archogen check` runs: read → boundary
  → schema → profile admission → workload admission → presence → refinement, returning a §5.5
  verdict chosen by what-to-fix-first precedence. `docs/semantics/cases/` is its 29-case corpus.
  `src/workload.rs` is the §3.1 *Workload* row made mechanical — unique priorities, constrained
  deadlines, exactly one declared release model — and its census test counts how many of the
  profile's thirteen `decisions` rows are still prose that nothing consults (twelve).
- `crates/archogen-cli/` — the `archogen` binary and its library. `src/spec.rs` declares the
  `ROADMAP.md` §10.2 command surface as data (help and parsing both derive from it), including
  each command's `Maturity` — built, **experimental**, or unimplemented, the third state existing
  because `build` runs over a narrower path than its §10.2 contract; `src/status.rs` is the §5.5
  outcome vocabulary and the stable exit-code contract; `src/cli.rs` is the dependency-free
  parser and help renderer. `src/check_cmd.rs` and `src/build_cmd.rs` are thin translations onto
  the crates that do the work.
- `crates/archogen-s0/` — ⛔ **the temporary S0 realization, and everything in it is meant to be
  deleted** (§12 S0: "no hidden special-case generator is grandfathered into the release"). It is
  deliberately not named `archogen-plan`/`archogen-emit`, the §4.2 names `M4` builds under.
  `interpret.rs` turns a checked description into a `Plan` and refuses in two families — inputs
  that describe no buildable system, and requests the engine cannot realize, the second naming
  the missing capability and its leaf per §5.4. `runtime/rt.rs` and `runtime/service.rs` are the
  engine-owned Rust every generated crate contains: compiled and tested here, copied there
  verbatim, with a test asserting the two are identical. `emit.rs` writes a manifest, one
  specialized table, and that copy — it generates no behavior. `provenance.rs` writes
  `provenance.json`, the §10.1 mapping from each generated declaration to its source span and the
  engine rule that produced it; every record is resolved from **both** ends by
  `crates/archogen-cli/tests/s0_provenance.rs`, because a record pointing at the wrong line is
  worse than none.
- `crates/rt-analysis/` — §7.4's idealized zero-overhead response-time analysis (F18).
  `model.rs` makes the eight applicability conditions a **constructor precondition**, so the
  analysis cannot return a number for a system the model does not describe; `response.rs` is the
  recurrence with checked arithmetic, two named limits, and the iterate sequence kept as a
  witness. ⭐ A positive answer exists only as `Conclusion::HoldsUnderAssumptions`, so "the
  deadlines are met" detached from "no overhead" is not constructible. ⚠️ Nothing here may be
  cited for a runtime claim until `M2.6`. Its F18 suite **parses §13.2's table out of
  `ROADMAP.md`** rather than copying it, which is §14.1's "cannot silently adjust expected oracle
  results" made mechanical. `cost.rs` + `docs/analysis/cost-accounting-v1.md` are §7.4.1's
  versioned accounting contract, held together by a drift test; ⭐ its "one interval, one
  category" rule is a **constructor precondition** — an exact-trace ledger with an overlap or a
  hole does not seal — because a double-charged total still looks plausible. `trace.rs` simulates
  §13.4's operational model for **F29** and `tests/f29_preemption.rs` compares it against the
  roadmap's own expected-trace table, parsed — two sources neither derived from the other. ⛔ Its
  controls **re-simulate**: deleting the resume switch does not shorten the response by four, it
  makes a whole interfering job disappear (23 → 14, not 23 → 19).
- `ROADMAP.md` — the program's direction, milestone exit gates, and the F01–F30 acceptance
  matrix. The single source of what "done" means.
- `docs/tasks/PROGRAM.md` — the roadmap-unit → tree map and the fixture-ownership map.
  Start here to find which tree owns a given roadmap item.
- `docs/semantics/kinds/` — the language declared in itself: `core.eadl` (the five surface
  kinds) and `os-rt.eadl` (the `task` kind of the workload module). Only `defkind` is Rust.
- `examples/` — the three M0 use cases as real descriptions, checked by
  `crates/eadl-model/tests/examples.rs`: they validate, and none carries an execution bound, an
  entry point or an allocation.
- `examples/s0-heartbeat/` + `crates/archogen-cli/tests/s0_oracle.rs` — the F28 corpus and its
  oracle. Three descriptions (base, one period changed, one sporadic) and the observations they
  must produce, frozen in `expected/` before any emitter existed. The oracle re-derives each
  expectation from its description through its own implementation of the five-rule observation
  contract in that directory's README, so an edit to either side fails. It sits in `tests/`
  deliberately: Rust cannot link an integration test into a library, so no emitter can call it.
  Its `f28_*` tests are the gate itself — clean directory, generate, compile, run, compare — and
  `s0_reader.rs` asserts the diagnostics half: a corrupted description is refused *where it
  broke*, with the expected line computed from the fixture rather than pinned.
  ⛔ `system-non-harmonic.eadl` exists because the three F28 cases are **harmonic**, so the gate
  could not tell a hyperperiod from a longest period; it has no frozen expectation on purpose.
- `docs/semantics/boundary/` — the 21-case boundary corpus (accept/reject pairs, 5 ambiguous),
  the input fixture F27 mechanizes. Its README fixes the case format.
- `docs/usecases/` — the four systems the toolchain must build or refuse; `docs/evaluation/` —
  the sealed reuse-measurement set and the check that keeps it sealed and unseen.
- `targets/riscv-virt-up.env` + `scripts/target_emulator.sh` — the pinned emulator
  configuration and the only tool that renders it. `--check` exits 20 when QEMU is absent
  rather than reporting a skipped check as a pass. `docs/targets/first-target.md` records the
  board decision: none procured.
- `xtask/` — the §14.3 tiered verification runner, reached as `cargo xtask verify --tier <t>`
  through the committed `.cargo/config.toml` alias. Tiers are data. ⭐ Its verdict has **three**
  states, because §14.3 requires an unavailable tool to be "reported as such, not a passed
  check": `passed` (0), `failed` (1), `incomplete` (20). Four of the five tiers are incomplete
  today and each names the leaf that closes it — which is the runner's most useful output.
- `scripts/check_doctrines.sh` — the doctrine enforcer (git hook + CI). `make gate`.
  Project doctrines live in `scripts/check_doctrines.project.sh`: `check_frozen_evaluation.sh`
  keeps the reuse measurement sealed and unseen, and `check_s0_retirement.sh` keeps the S0
  prototype from quietly becoming permanent — every `S0-ASSUMPTION:` marker listed with an owning
  leaf, and no new consumers of the crate — and `check_book_anchors.sh` keeps the mdBook's claims
  about the code resolvable: every behavior chapter cites a repository path, and every cited path
  exists. All three carry `--self-test` RED arms.
