# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — one function, one provider

`ARCHOGEN-M3-0870` (leaf `M3.3.6.2`), `2026-10-22`.

- The joint check keeps one requester's facts of one function at one provider: a counter's width and rate split
  across two timers is refused, or reported `missing-fact` where describing a fact would make one timer whole. A
  qualifier, such as a power state, is judged at every provider the requester relies on.
- A census over the suite finds no function split across hardware and lists five counters whose horizon or wrap
  behaviour no block states yet (the design's approximate count was nine). Model §8 rule 13 and the book explain it.

## archogen — each vocabulary fact names its function

`ARCHOGEN-M3-0869` (leaf `M3.3.6.1`), `2026-10-22`.

- `deffact` gains `function`: every guarantee of `/1` names the function it describes — `counter`, `interrupt`,
  `output` and the rest — or `each` for a qualifier; the vocabulary refuses one missing, misplaced or joining another
  function's facts. The book's matching chapter explains functions; a pending migration note covers the two files.

## archogen — provider bundles: the design, reviewed

`ARCHOGEN-M3-0868` (leaf `M3.3.6`), `2026-10-22`.

- Design `SR-H14`'s bundles: each vocabulary fact names the function it describes, and one requester's facts of one
  function are met by one provider; where describing a fact would make one provider whole the finding is an unknown,
  not a conflict. Qualifiers — a power state — hold at every provider the requester relies on.
- One independent review, seven defects and eight remarks answered; a census over the suite's 38 systems finds no
  function split across hardware, and names the 9 counters whose horizon no block states yet.

## archogen — a block no system uses decides no description's code

`ARCHOGEN-M3-0867` (leaf `M3.4.1`), `2026-10-22`.

- `SR-H18` amended as `M3.4` decided: when the search decides a description's code, a block or platform outside a
  system's hardware takes no part, as an unused device costs nothing; the book's account of `M3.4` says so.

## archogen — the runtime analysis design moves to the specifications

`ARCHOGEN-PROGRAM-0866` (leaf `PROGRAM.102`), `2026-10-22`.

- `docs/decisions/` held 392950 of the director's 393216 bytes. By the ruling of `2026-10-01`, the runtime analysis
  variant's accepted design moves to `docs/specs/analysis/`, every path citing it repointed, `MODEL`'s among them:
  355040 bytes now, room for `M3.3.6` and `M3.4`'s amendment.
- `docs/specs/` had 42 bytes to spare, so its ceiling rises one 64 KiB step, 589824, by its record's measured paragraph.

## archogen — the version register reads a declaration rustfmt wraps

`ARCHOGEN-PROGRAM-0865` (leaf `PROGRAM.104`), `2026-10-22`.

- `VERSION-REGISTER` read declarations a line at a time, so `fixed-priority-with-overheads/2` — the runtime analysis
  variant's model, bumped from `/1` by `M2.11`, wrapped by rustfmt — was never registered. It now reads each declaration
  whole; three arms hold the wrapped shapes and a profile's `id:`.
- The book's register gains `runtime-analysis-model`, pinned by a test that holds the constant to the identifier and
  the path of the record that defines it.

## archogen — uc3 states its level

`ARCHOGEN-M3-0862` (leaf `M3.3.4.5`), `2026-10-22`.

- `examples/alternative-timer`'s timer, console and platform each offer `(reachable-at-privilege machine)`, as every
  other suite description's hardware does; the privilege census now excludes nothing, and `M3.3.4` closes.
- No requirement moved: `archogen check` still answers `13 infeasible-configuration`, its locations one line down. The
  seal that uc3 builds unchanged now says it changed once, before `M3.2`, and that the language freeze refuses any later
  change without a note.

## archogen — the director's delegation: six findings decided

`ARCHOGEN-M3-0861` (leaf `M3.3.4.5`), `2026-10-22`.

- Decided under the director's delegation of low-level technical findings: uc3 gains its privilege level (a); the
  profile's Privilege row stands; `docs/decisions/` gains room by moving specifications (`PROGRAM.102`); the changelog
  becomes curated, per user-visible change (`PROGRAM.103`); `M3.4`'s three questions answered; `M3.6.9.7` and `.8`
  stay postponed with the mutation work. Each decision, its reason and its alternatives are in its leaf.

## archogen — file: the decisions folder at its byte ceiling

`ARCHOGEN-PROGRAM-0860` (leaf `PROGRAM.102`), `2026-10-22`.

- `docs/decisions/` holds 392915 of its ruled 393216 bytes, so `M3.3.6`'s vocabulary rows have nowhere to go; filed
  at the frontier's head with its remedy — records that are specifications moved to `docs/specs/` — and its candidates.

## archogen — the initialization graph, derived

`ARCHOGEN-M3-0859` (leaf `M3.3.7`), `2026-10-22`.

- From one system's assignment the joint check derives what initialises before what, for the plan: providers before
  the requesters bound to them, used services before their users, each edge with its reason; composed parts exempt,
  adapters reached through their bindings.
- Nothing is refused: a cycle is a component the engine initialises as one unit. Four tests fail first under five arms;
  model §8 rule 12 and the book explain it.

## archogen — the initialization graph: the design, reviewed

`ARCHOGEN-M3-0858` (leaf `M3.3.7`), `2026-10-22`.

- Design initialization order as a graph derived from one system's assignment, for `M4.1`'s plan: providers before the
  requesters bound to them, used services before their users, each edge with its reason; nothing refused, a cycle a
  component the engine initialises as one unit, since initialisation runs masked before every release.
- One independent review, five defects and four remarks answered; taken before `M3.3.6`, whose bundles wait on a
  vocabulary decision and on `M3.2`'s adapters.

## archogen — a required ordering is unsupported under this profile

`ARCHOGEN-M3-0857` (leaf `M3.3.5.2`), `2026-10-22`.

- `ordering` keeps no vocabulary entry: `rt-static-up-v1` excludes `dma`, whose obligations name ordering, so its domain
  is the later profile's; a requirement on it in a closure is `unsupported-profile`, pinned by a test.
- The boundary case requests nothing and keeps its frozen verdict; model §8 rule 11 and the book say so.

## archogen — placement, judged once per system

`ARCHOGEN-M3-0856` (leaf `M3.3.5.1`), `2026-10-22`.

- A region's sub-forms follow one grammar in the joint rules and the target agreement: each once, a flag `true` or
  `false`; the agreement refuses `reserved`, which it does not compare.
- Each system's memory map is judged once: reserved ranges in placement memory are holes reported for the plan; a
  device or a second placement region on its bytes, or memory all holes, is `invalid-description`; a placement the plan
  is given must be placement memory. Model §8 rule 10 and the book explain it.

## archogen — placement: the design, reviewed

`ARCHOGEN-M3-0855` (leaf `M3.3.5`), `2026-10-22`.

- Design memory placement: executable, unreserved regions are placement memory; reserved ranges inside it are holes
  the plan avoids, reported as an obligation; a device's registers inside it are a contradictory map, refused once per
  system; M3 chooses the region, `M4.1` the offsets.
- One independent review, five defects and four remarks answered; `SR-H13` amended so `region` stays outside `/1`, its
  sub-forms normative in model §8; ordering is `.2`'s own design.

## archogen — the target states its level, and its agreement reads every claim

`ARCHOGEN-M3-0854` (leaf `M3.3.4.4`), `2026-10-22`.

- `riscv-virt-up` requires `machine` (`REQUIRES_PRIVILEGE`), and its blocks and platform state it; the agreement
  compares each, from the privileged specification's reset and PMP sentences and the spike's measured run at M.
- The agreement now refuses an offer or a clause it does not read, as it refused an uncompared region: the target
  claims nothing it cannot check. The emulator check and the spike pass on the pinned QEMU.

## archogen — every suite system's hardware states its level

`ARCHOGEN-M3-0853` (leaf `M3.3.4.3`), `2026-10-22`.

- 45 blocks and platforms in 21 suite descriptions gain `(reachable-at-privilege machine)`, so the joint rule will refuse
  none of them once wired; a census over each system's hardware holds it, `examples/alternative-timer` excluded by name
  until the director decides.
- A pending migration note names the 21 moved constructs; the frozen verdicts and the third opinion are unchanged; the
  tour's copies of `s0`'s description and the sporadic case moved with them.

## archogen — the privilege level is the profile's

`ARCHOGEN-M3-0852` (leaf `M3.3.4.2`), `2026-10-22`.

- `rt-static-up-v1` gains a Privilege row: the runtime, its tasks and its services run at one level, `machine`, which a
  RISC-V hart starts in at reset — the privileged specification's §2.1.4, quoted and ledgered — and the playground
  models it; `Profile::caller_level` carries it for the search.
- The port record's citation for machine mode is corrected to that row, that sentence and the spike's own instructions.

## archogen — privilege, judged given a level

`ARCHOGEN-M3-0851` (leaf `M3.3.4.1`), `2026-10-22`.

- Given the level a plan runs one system's callers at, the joint check refuses a caller stating another level
  (`unsupported-profile`, first), and a caller bound to a provider not reachable at that level or stating none.
- A block needing a sibling's fact is no caller; routes carry the level; with no level given, nothing is said.
- Seven tests with literal expectations, failing first under five arms; model §8 rule 9 and the book explain it.

## archogen — privilege: the design, reviewed and restructured

`ARCHOGEN-M3-0850` (leaf `M3.3.4`), `2026-10-22`.

- Design the privilege rules: a plan binds one system's callers to one level; a caller stating another level is
  `unsupported-profile`; a caller bound to a provider whose set lacks the level, or states none, is refused.
- One independent review — ten defects, four remarks — showed the level's source unestablished and unwired; it becomes
  its own child, beside the suite's and the target's migrations; uc3's change goes to the director (findings §15).

## archogen — connectivity and interrupt routes, judged

`ARCHOGEN-M3-0849` (leaf `M3.3.3`), `2026-10-22`.

- The joint check judges one system's assignment: a block or platform bound for a requester the system serves must be
  part of its hardware — its platform and what that names through `uses` and `needs`, `refines` excepted — or the
  assignment is refused, naming both ends and the repair.
- Each source and each timer an assignment relies on is reported as a route obligation, a source with its own
  separation, for the plan to enable and the analysis to read.
- Eight tests with literal expectations, each rule failing first when removed; `model.md` §8 rules 7 and 8 and the
  book's matching chapter explain both.

## archogen — connectivity and interrupt routes: the design, reviewed

`ARCHOGEN-M3-0848` (leaf `M3.3.3`), `2026-10-22`.

- Design the joint rules for §5.4's device connectivity and interrupt routes: an assignment is one system's; a binding
  to a block or platform outside the system's hardware, read along `SR-H15`'s edges, is refused, the description's
  code left to `SR-H18`; each source and each timer an assignment relies on is reported as a route obligation for the
  plan and the analysis to discharge.
- One independent review: seven defects and four remarks, each verified and answered; two questions filed for `M3.4`.

## archogen — a failed tier step's whole output is kept

`ARCHOGEN-PROGRAM-0847` (leaf `PROGRAM.100`), `2026-10-22`.

- When a step fails, the tier runner keeps its whole output in `target/verify-logs/<tier>/<step>.log` — the command,
  its status, then each stream in full — and names it under the same 40-line tail; a step's log is removed when the
  step next runs, so a log that exists is of a failed last run.
- Hold it with a synthetic step through `run_step`, failing first with the keeping or the removal taken out.

## archogen — the fourteenth daily artifact cleanup

`ARCHOGEN-PROGRAM-0846` (leaf `PROGRAM.101`), `2026-10-22`.

- Remove only the idle main and native-sweep incremental caches — 5382 files, 1181873277 logical bytes — after two
  handoff censuses and an inventory whose identities were rechecked first; every other entry under `target/` is
  identical or accounted, the cited evidence keeps its digests, and the tracked inputs restore.
- Overwrite `docs/ARTIFACT_CLEANUP.md`, add the run's section to the repository annex, and seal `PROGRAM.99`.

## archogen — a test's deadline is a safety net, never the property

`ARCHOGEN-PROGRAM-0845` (leaf `PROGRAM.99`), `2026-10-22`.

- Run the focused tier at `eaf71286` on a quieter host — exit code 0 — and record it in `M3.3.1` and `M3.3.2`.
- Add `scripts/exec_pressure.sh`, which re-creates a loaded host and refuses to claim a load it did not measure. Under
  it, three bounded children ran past their 30 s limit without hanging, and two sweep controls raced a baseline with 1 s.
- A bounded child's limit is now `ARCHOGEN_PARENT_TEST_LIMIT` seconds or ten minutes; `cargo xtask mutate` and the
  sweep keep the 30 s their own limits were sized for. Every control's finishing side gets a deadline only a hang
  reaches, its hang side a tight one against a hang that outlasts the bound. Green under the same loads.
- State the rule in the book's verification chapter; file `PROGRAM.100` (a failed step's output kept whole) and
  `M3.6.9.7` (a slow child counted as a kill in a loaded sweep, blocked with the mutation work).

## archogen — the host handed over for the quiet run

`ARCHOGEN-PROGRAM-0844` (leaf `PROGRAM.99`), `2026-10-22`.

- Record that the director stopped the host's other projects' jobs so a fresh session can run the quiet-host focused
  tier `M3.3.1` and `M3.3.2` owe; syspolicyd still sampled at up to 61 % with no build left, so that run measures the
  host first. The resume pointer now names `PROGRAM.99` before `M3.3.3`.

## archogen — the joint checker: channels and owned regions

`ARCHOGEN-M3-0843` (leaf `M3.3.2`), `2026-10-21`.

- Add `crates/eadl-resolve/src/joint.rs`: one complete assignment of requesters to providers judged as a whole — the
  requesters taking a provider's channels through consumed facts at most its count (F08), and no two bound providers
  owning overlapping bytes unless one composes the other (F09) — with provider-level refusals, `missing-fact` for an
  undescribed count or size, and a service's multiplexing obligation to the tasks that share it.
- Hold both cases with descriptions under `crates/eadl-resolve/tests/joint/`: every assignment refused with literal
  expectations, beside controls that hold; each rule's test failing with the rule removed.
- State the rules as `docs/semantics/model.md` §8 and explain them in the book's matching chapter. Every crate but
  `xtask` passes; `xtask`, which names no `eadl-resolve`, waits on a quiet host (`PROGRAM.99`).

## archogen — file the xtask suite's failures under exec saturation

`ARCHOGEN-PROGRAM-0842` (leaf `PROGRAM.99`), `2026-10-21`.

- File, at P1, what `M3.3.1`'s gate met: the `xtask` suite passing in 240 s, then failing 73 and 31 tests that pass
  alone, with syspolicyd saturated by the host's exec load. The quiet-host focused run that commit owes comes first,
  then the measured cause and its repair; the director's pasted diagnosis is kept as hypotheses to test.
- Mark `M3.3.1`'s migration note `applied`, as the freeze gate requires of the commit after it lands.

## archogen — a fact can be consumed, and a provider states its channels

`ARCHOGEN-M3-0841` (leaf `M3.3.1`), `2026-10-21`.

- Write `M3.3`'s design — an assignment of requesters to providers, a channel count over consumed facts (F08), region
  ownership among bound providers (F09) — and answer its one independent review: three defects, three remarks.
- Give `deffact` an optional `use` clause, `shared` by default or `consumed` on a boolean fact or a group's head; mark
  `relative-delay` and `absolute-deadline` consumed; add the entry `channels`. The record's rows, the model's table, a
  migration note and the frozen baseline move with it; no tracked verdict changes.
- Explain consumed facts and channels in the book's matching chapter, with an example.

## archogen — mutation testing waits until the functionality is locked

`ARCHOGEN-PROGRAM-0840` (leaf `PROGRAM.98`), `2026-10-21`.

- Asked whether the project was progressing, measure it: since `2026-10-11`, 158 of 173 commits went to `M3.6.9`, the
  mutation adequacy of the trust gate's tests, and none to the product.
- Record the director's ruling as findings §14 and `decision_mutation-testing-after-completion.md`: mutation testing
  stops until the functionality is complete and locked, and every decision advances the project first.
- Postpone every pending `M3.6.9` leaf, `blocked` on the ruling with its contract and results kept; keep the suspected
  generator target-directory defect `.8.2` owned at P2; move the frontier to `M3.3`.

## archogen — assess the own-package completion check

`ARCHOGEN-M3-0839` (leaf `M3.6.9.6.3.2.2.15.2.1.8.0`), `2026-10-21`.

- Select the check that stops the inventory unjudged when a successful build compiled nothing of the program's own
  package, and map the historical survivor of the source-`069d8e59` run, `L911.cond.332`, to `L894.cond.257` by
  exact text.
- Replay all 6 of the check's candidates against all 209 parents with strict timeouts: `if false {` survives, so no
  current parent reaches the check; the 5 edits that make it fire where the own package was compiled are killed.
  All inputs restore; the audit refuses nine altered receipts. Measurement only.
- File `.8.1`, the producer routes — a rerun into a kept target directory, an own unit refused for its owner — then
  a control or a proof; and `.8.2`, generator target directories that two generators can share. Seal the closed
  configuration absolute paths.

## archogen — refuse an outside path dependency before compilation

`ARCHOGEN-M3-0838` (leaf `M3.6.9.6.3.2.2.15.2.1.7.2`), `2026-10-21`.

- Root cause: the metadata reader read a package's directory as its manifest path less the canonical
  tree prefix, and as the root's empty one when the prefix did not match, so a path dependency outside
  the written tree passed the manifest rules as the root's; the build then stopped on compiler
  provenance and the run ended unjudged, exit 2.
- Refuse every package whose manifest lies outside the written tree right after the metadata is read,
  naming it, before anything is compiled: exit 1, the commit's own fault.
- The control `a_path_dependency_outside_the_written_tree_is_refused_before_any_compilation` fails first
  on the provenance error and then holds an absolute path, a climbing one, two at once and one under a
  directory repeating the tree's path; all 7 edits of the refusal are killed against 209 parents. The
  three proofs pinning the production are revalidated. Close `.7`; own-package completion is next.

## archogen — hold the configuration refusal directly

`ARCHOGEN-M3-0837` (leaf `M3.6.9.6.3.2.2.15.2.1.7.3`), `2026-10-21`.

- Extract the refusal of a unit configured with a path outside the written tree into
  `checked_configuration`, which returns the configuration it checks; the unit reader binds its
  configuration through it, so the check cannot be dropped without the configuration.
- Add the control `a_configuration_naming_an_outside_path_is_refused_at_every_position`: a token that
  begins with `/` and one that holds `=/`, each first, in the middle and last, two at once, and none, with
  literal refusals. Against all 208 parents every edit of the function is killed by it or refused by the
  compiler — 14 and 5, strict exit 0 — the ten historical survivors among the killed. No reason used.
- Revalidate the three proofs pinning the production and re-bind their digests; the trust chapter states
  the refusal, that no route reaches it, and the control.

## archogen — measure the configuration's producer routes

`ARCHOGEN-M3-0836` (leaf `M3.6.9.6.3.2.2.15.2.1.7.1`), `2026-10-21`.

- Ask the real pinned Cargo, through the unchanged inventory, which route can leave a path outside the
  written tree in a unit's configuration: none does. A commit's cargo configuration, workspace
  `rustflags`, an inherited `RUSTFLAGS` and build scripts are closed before rustc; `trim-paths` is
  unstable in Cargo 1.95.0.
- An outside path dependency, absolute or climbing, is built and the run then stops unjudged: the
  outside crate's compiler record carries its own directory, which the provenance session refuses.
  That is the commit's own fault and should be a refusal; owned by `.7.2`.
- With no route reaching it, the configuration refusal is defence in depth; `.7.3` holds it directly.
  Measurement only; all inputs restore; the audit refuses six altered receipts.

## archogen — assess the configuration absolute-path loop

`ARCHOGEN-M3-0835` (leaf `M3.6.9.6.3.2.2.15.2.1.7.0`), `2026-10-21`.

- Select the loop that refuses a unit configured with a path outside the written tree — a token that
  begins with `/` or holds `=/` — and map the ten historical survivors of the source-`069d8e59` run to
  current candidates by exact text.
- Replay all 18 of the loop's candidates against all 207 parents with strict timeouts: 10 survive, the
  ten historical ones, `if false {` among them, so no current test reaches the refusal; 4 widenings are
  killed and 4 narrowings refused by the compiler. All inputs restore; the audit refuses eight altered
  receipts. Measurement only.
- File `.7.1`: which producer route can leave an outside path in a configuration, then a control per
  form at every position, or a proof.

## archogen — the colon path limit surfaced to the director

`ARCHOGEN-PROGRAM-0834` (leaf `PROGRAM.97`), `2026-10-21`.

- Record as §13 of the findings register that the repository cannot live under a path holding a colon:
  the pinned Cargo fails every build under a target directory holding one, against the standing rule
  that the repository may be moved anywhere. It cites the measurement, what the catalog gate already
  does about it and the characters measured harmless, and recommends accepting the toolchain's limit;
  nothing waits on the answer.

## archogen — a colon in the target directory is unjudged

`ARCHOGEN-M3-0833` (leaf `M3.6.9.6.3.2.2.15.2.1.6.2.3`), `2026-10-21`.

- Measure where the pinned Cargo cannot build: a trivial crate fails, exit 101, exactly when its target
  directory's path holds a colon — Cargo joins `<target>/<profile>/deps` into the library search path,
  and std refuses a path holding its separator — and builds when only the crate's directory or Cargo's
  home holds one.
- Stop the catalog gate's `check` before it builds anything when std cannot join its target directory,
  unjudged and naming the directory: it used to refuse the package as not building, blaming a package
  for its directory. The control `a_target_directory_holding_the_search_path_separator_is_unjudged`
  fails first without the check; §3 is unchanged. The caller-suffix proof's premise is revalidated.
- State the limit where the director reads it: the repository cannot live under a path holding a colon.
  Close `.6.2`, the dependency-block checkpoint, and `.6`, the dependency absolute paths; the
  configuration absolute paths `.7` are next.

## archogen — the catalog reader unescapes nothing

`ARCHOGEN-M3-0832` (leaf `M3.6.9.6.3.2.2.15.2.1.6.2.2`), `2026-10-21`.

- Remove from `dep_info` in `xtask/src/catalog_build.rs` a replacement of `\ ` that could never apply:
  it ran on pieces already split at spaces. The reader now states the domain the assessment measured:
  relative paths in the catalog's grammar, the record's own path written literally, anything else
  refused when it is judged.
- Add the permanent control `catalog_build::tests::a_root_s_name_never_reaches_what_its_builds_read`:
  the harness's workspace built through `check` under nine root names holding a space, backslashes, a
  tab, a line break, a hash and a quote, each record's rule held exactly. It passes on the old and the
  new reader and fails on a seeded first-space split, which refuses the spaced root naming a fragment.
- Every catalog-builder test passes, 15 before and 16 after; the catalog chapter states the reader's
  rule and names the control.

## archogen — measure the catalog builder's dependency reader

`ARCHOGEN-M3-0831` (leaf `M3.6.9.6.3.2.2.15.2.1.6.2.1`), `2026-10-21`.

- Build the catalog test harness's two-module workspace through the unchanged production `check`, with
  the real pinned producer, under ten directory names: ordinary, a space, a colon, three backslash
  shapes, a hash, a quote, a tab and a line break, each where production writes it.
- Nine are accepted with all 36 records exact: a record names the package's files relatively and its
  own path literally, so the root's name never reaches the prerequisites. A colon is Cargo's own error,
  `failed to join paths`, before any record is written. Outside inputs cannot be reached: the loader
  refuses the tokens that would name them. No false refusal, fragment or dropped input.
- File what was found: the reader's unescape can never apply (`.6.2.2`, P2, next), and a colon's
  outcome is reported as the package not building (`.6.2.3`, P3). The catalog chapter says what a build
  may have read; the annex keeps every record and the probe.

