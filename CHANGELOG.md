# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — replay the complete dependency block

`ARCHOGEN-M3-0830` (leaf `M3.6.9.6.3.2.2.15.2.1.6.2.0`), `2026-10-21`.

- Define the dependency block by structure, not by the lines a repair changed: the reader, its result
  type and the environment records' normalisation, and the unit reader from opening the record to the
  end of its prerequisite loop. Select every candidate the sweep's verified discovery lists there: 64 of
  1430, the repair's 37 and 27 neighbours.
- Replay all 64 against all 207 parents with strict timeouts under the write-fenced sweep: 58 killed,
  each finishing in at least one of the block's own controls, and 6 refused by the compiler, each with
  its actual error; raw strict exit 0, no survivor, timeout, unjudged or equivalent outcome.
- Map the initial assessment's four survivors to current candidates by exact text; all four are killed,
  and the repair's 37 edits keep their outcomes. 1172 inputs restore in the tree and the copy; the audit
  refuses ten altered receipts. Its first run refused one kill until two omitted dependency-record
  controls joined its list; no outcome changed. Production, tests, reasons and the catalogue unchanged.
- Split the checkpoint: the catalog builder's own dependency reader is measured next, `.6.2.1`.

## archogen — the thirteenth daily artifact cleanup

`ARCHOGEN-PROGRAM-0829` (leaf `PROGRAM.96`), `2026-10-21`.

- Remove only the idle main and owned native-sweep incremental caches, 2849 files / 1017454565 logical
  bytes (360 cached `.bin`, no `.log`), after physical and native inventories that agree, a handoff
  census before the inventory and again before the deletion, and an identity recheck of every
  inventoried entry. Dependency stores, final products and every cited evidence directory are retained.
- Compare, for the first time, the identity of every other entry under `target/` before and after:
  584886 of 584889 identical; the three rows the strict comparison refused are the two removed
  caches' parents and the scratch directory the handoff census makes and removes its guard in, each
  accounted. 580 evidence digests and 1168 tracked inputs restore; an audit refuses four altered receipts.
- Seal what closed at `HEAD`, `M3`'s `.6.1` subtree and `PROGRAM.95`, with their log rows; overwrite
  the latest-cleanup record and add the run's section to the repository annex. The external policies
  re-read at session start still hash as adopted.

## archogen — hold Cargo's terminal-backslash refusal as a control

`ARCHOGEN-M3-0828` (leaf `M3.6.9.6.3.2.2.15.2.1.6.1.1.2`), `2026-10-21`.

- Add the permanent control `trust::tests::a_final_prerequisite_ending_in_a_backslash_is_the_builds_own_error`:
  three outside inputs whose names end in a backslash make rustc's first rule end in a backslash, and
  the pinned Cargo 1.95.0 refuses its own dependency record as an internal error, `malformed dep-info
  format, trailing \`, before the inventory reads anything. The control holds Cargo's exact message
  naming the record, no inventory and no refusal list, and the reader's own refusal of the record rustc
  left, so the inventory does not rest on Cargo's refusal.
- Rule that a build the pinned Cargo refuses is reported as the error it is, never as a refusal: a
  refusal is the inventory's judgment of a record it read; the outcome is unjudged, not passed.
  Production, reasons, catalogue and premises are unchanged; the parent population grows by the
  control alone; the audit rejects 21 altered receipts.
- Close `.6.1.1`, the path-domain follow-up, and `.6.1`, the survivor witnesses; the full
  dependency-block checkpoint `.6.2` is next. The trust chapter and the annex carry the decision and
  the record; the seal takes the rows of `M3`'s repaired Verification Log table.


## archogen — a sealed leaf's log rows follow it

`ARCHOGEN-PROGRAM-0827` (leaf `PROGRAM.95`), `2026-10-20`.

- Extend the task-history seal: every Verification Log and Commit Log row naming a stub, and held by
  the committed tree, moves byte for byte and in the tree's order into numbered log-row files under
  `docs/task-history/<TREE>/`, as many as a byte budget needs, listed in the index per tree; nothing
  is left where a row stood. The seal's proof, gate, rollback and signal rules cover the rows.
- Add the gate's seventh leg, provenance of rows against the tree's logs just before their seal, and
  the census's measure of rows a seal would take; 24 RED arms fail first and pass with the tool.
- Seal every tree's sealed-leaf rows, 1122 rows into seven files, taking `docs/tasks/` from 815 600 to
  465 813 bytes; repair the broken Verification Log tables of `M3` and `PROGRAM`, whose rows the next
  seal takes; seal the findings register's ruled §6 to make room for the ruling in `docs/decisions/`.


## archogen — refuse ambiguous dependency records

`ARCHOGEN-M3-0826` (leaf `M3.6.9.6.3.2.2.15.2.1.6.1.1.1`), `2026-10-20`.

- Repair the inventory's dependency-record reader by refusal: mark only the written tree's known
  raw and canonical prefixes before splitting a rule, refuse a backslash anywhere else, and require
  rustc's per-file targets to re-spell exactly the list read. A record that fails is refused naming
  the raw rule, and no inventory is written.
- Replay the prefix collision, the spaced outside inputs under the raw and a symbolic output root
  and the aliased in-tree inputs as permanent real-Cargo controls with literal expectations, and
  the record grammar as a synthetic control; re-bind the catalogue entry that quotes the reader.
- Every mutant of the changed production lines is killed by an intended control or refused by the
  compiler against the full parent population; reasons unchanged and the retained proofs' pinned
  premises re-bound; all1156 inputs restored. The terminal-backslash control and the
  dependency-block checkpoint stay separately owned.


## archogen — assess the dependency path domain

`ARCHOGEN-M3-0825` (leaf `M3.6.9.6.3.2.2.15.2.1.6.1.1.0`), `2026-10-20`.

- Probe six outside-path variants of the public-inventory refusal parent with real pinned
  Cargo on the plain copy; production, every test and every reason stay unchanged.
- Ordinary and symlinked output roots refuse exactly. The P0 prefix collision is observed:
  a trailing-backslash outside input read as an in-tree path, the inventory written twice
  and naming a decoy blob. Spaced outside paths refuse under fragment names; the pinned
  Cargo refuses a terminal backslash before the inventory reads anything.
- Restore all1156 main and copied inputs; retain receipts, audit, falsifiers and adapters
  in the unbounded book. Own the refusal repair and the terminal-backslash control as
  separate children; no repair is claimed here.


## archogen — hold every outside-tree dependency refusal

`ARCHOGEN-M3-0824` (leaf `M3.6.9.6.3.2.2.15.2.1.6.1.0`), `2026-10-20`.

- Add one real-Cargo/public-inventory parent holding three complete literal outside-tree
  data refusals and no inventory. Preserve every old test, production byte and reason.
- Exact initial4S becomes native4K/full202 strict0; the new parent is the sole intended
  finishing kill for each edit. Restore all1155 original/copied inputs and parent names.
- Bind actual producer order,48 source bindings and37+2 producer sources; retain full
  portable evidence, exact two-seal history and verbatim live contracts in the unbounded
  book. Path-domain/full-block follow-up stays open; focused tier required before commit.


## archogen — verify the approved disposable fixture workflow

`ARCHOGEN-M3-0823` (leaf `M3.6.9.6.3.2.2.15.2.1.6.3`), `2026-10-20`.

- Verify the director-approved ignored target fixtures through actual boundary9/0
  controls and the normal hook-route check; five false receipts refuse.
- Confirm ignored/untracked fixture paths, unchanged policy/checker sources and a
  clean main tree after execution. Normal assessment commit hooks passed; no bypass.
- Retain complete commands/output, approval and auditor in the unbounded book;
  close only this policy verification and resume dependency-path survivor witnesses.


## archogen — assess absolute dependency paths

`ARCHOGEN-M3-0822` (leaf `M3.6.9.6.3.2.2.15.2.1.6.0`), `2026-10-20`.

- Reconcile all four historical/current dependency-absolute edits: native4S/full201,
  strict1 unmet; all1153 original/copied inputs and exact names restored.
- Preserve production, tests and reasons; reject14 native/binding and4 mapping false
  receipts. Retain full portable inputs/output/adapters, six pre-approval task states
  and the approved state in the unbounded book; all332 prior/new333rd archives exact.
- Own every survivor; record the director's ignored project-owned Git-fixture exception
  and preserve the external-repository boundary. Verify that exception next, then repairs.


## archogen — close the current environment-tail checkpoint

`ARCHOGEN-M3-0821` (leaf `M3.6.9.6.3.2.2.15.2.1.5.2`), `2026-10-20`.

- Reconcile the historical/current tail with the complete source proof and exact
  committed cleanup; no equivalence follows merely from candidate absence.
- Fresh current24/full201 gives1K/23E, strict0: every unchanged reason holds and the
  mapped non-tail continue has actual finishing kills. A proposed generic reason refuses.
- Revalidate5 actual source/producer/close controls,48 bindings and37+2 producer sources;
  preserve every reason, source and semantic test. The201 selected names contain no ignored tests.
- Restore all1152 original/copied inputs, exact parent names and the reconstructible
  held root; preserve331 prior archives/new cleanup seal and full book evidence/adapters.
  Close only .1/.5; dependency paths next, compiler/caller/global/review still open.


## archogen — remove the proved redundant environment tail

`ARCHOGEN-M3-0820` (leaf `M3.6.9.6.3.2.2.15.2.1.5.1.2`), `2026-10-20`.

- Remove only the separately proved terminal continue. Preserve every semantic test,
  normalizer/allowlist, caller statement and reason record; no file-wide continue reason.
- Qualify both full-prefix guard refusals and the additionally caught newline-trimmed
  close-caller guard. Update only3 hash literals; re-run all50 actual proof/operator controls.
- Reconcile1414 to1413 current mutations: tail alone retired;303 exact fresh-name
  renames follow physical lines. All23 reason sites persist; L1309.stmt.451 is retained
  for fresh native replay, with no projected current kill. Four altered mappings refuse.
- Preserve330 prior archives/new permanent-control seal, exact source/task evidence and
  the complete failed/recovered controls in the unbounded book. Final focused staged-tree
  stamp, book and doctrine gates precede this commit; the current24 native checkpoint is next.


## archogen — retain permanent environment-tail proof controls

`ARCHOGEN-M3-0819` (leaf `M3.6.9.6.3.2.2.15.2.1.5.1.1`), `2026-10-20`.

- Add maintained EnvironmentTailTests outside the semantic population: exact original/
  clean source shapes, normalizer/allowlist/type boundary and7 rejecting changed premises.
- Compile17 independent literal cases in3 forms, destructor traces and a distinguishing
  suffix at O0/O3; all8 wrong oracles fail. Full50 operator tests pass; old49 bytes exact.
- Fix the initial drop-fixture overlap: canonical C correctly strips CARGO's C;
  use non-overlapping fixture prefixes. Production,201-parent sources and23 reasons unchanged.
- Preserve329 prior archives/new exact proof seal and publish complete maintained
  controls, results and reconstruction adapters in the unbounded book. Cleanup remains next.


## archogen — prove the environment-loop tail cleanup

`ARCHOGEN-M3-0818` (leaf `M3.6.9.6.3.2.2.15.2.1.5.1.0`), `2026-10-20`.

- Prove the complete local loop has the same next-iteration and exited drop scopes
  with its tail continue, actual comment edit or removed line, from pinned Rust contracts.
- Actual pinned compiler O0/O3:17 independent literal cases in3 forms, exact drop traces,
  a distinguishing reachable-suffix counterexample, and all8 wrong oracles fail101.
  Clippy1.95 independently flags the original tail; finite agreement is not the proof.
- Keep production,201 semantic parents and all23 reasons unchanged. Separately own
  permanent controls, source cleanup/guard revalidation and strict current-reason replay.
- Preserve328 prior archives/new exact assessment seal and route full active container
  contracts verbatim to the unbounded book, with independently reconstructible task states.


## archogen — assess the current environment-loop tail

`ARCHOGEN-M3-0817` (leaf `M3.6.9.6.3.2.2.15.2.1.5.0`), `2026-10-20`.

- Bind the exact historical/current complete loop and re-audit the retained old83/153
  before record; fresh L772.stmt.221 survives all201, raw strict1 remains unmet adequacy.
- Fix the draft program/report digest confusion: published5899b233 identifies audit.py,
  report557d838f has its own identity;20 retained receipt digests and8 false bindings verify.
- The continue text key matches22 sites and a real known other-site kill refuses an
  unscoped reason. P1 proof/control .1 owns sound site scope before any reason; none added.
- Restore1148 original/copied inputs and exact held-root reconstruction; preserve
  source/reasons,48 bindings/37+2 producer sources,327 old archives/new suffix seal and
  complete parent/future contracts. Full evidence and adapters remain in the unbounded book.


## archogen — close the current caller suffix checkpoint

`ARCHOGEN-M3-0816` (leaf `M3.6.9.6.3.2.2.15.2.1.4.4`), `2026-10-20`.

- Freshly bind both historical/current caller edits and replay all201 current semantic
  parents: 1K/1E, strict0; contains fails the intended regression and the shorter
  prefix retains its exact guarded reason. No new E or broader closure is inferred.
- Revalidate actual producer/constructor controls,18 source/pin/wiring premises and
  changed-premise/wrong-oracle controls; preserve48 bindings and37+2 producer sources.
- Restore1147 original/copied inputs; reconstruct their exact identities/root tree
  from the retained commit and portable delta, rejecting five incomplete derivations.
- Retain326 prior archives and the exact new two-node cache seal. Publish complete
  native/proof/mapping/input evidence and adapters in the unbounded book; advance to
  the environment-loop tail. Full caller/global/review and renderer adoption stay open.


## archogen — close the repaired cache and current-reason checkpoint

`ARCHOGEN-M3-0815` (leaf `M3.6.9.6.3.2.2.15.2.1.4.3.2.3`), `2026-10-20`.

- Complete all43 conservative cache/current-reason edits against every current201
  semantic parent: 18K/23E/2U, strict0; no survivors, timeouts or unjudged outcomes.
- Retain exact reasons and complete intended finishing names; fresh whole-component
  baseline/mutant/recovery compilations qualify both U with intended diagnostics.
- Restore all1146 original/copied inputs; preserve production/reasons,48 source
  bindings/37+2 producer sources and18 proof premises. Exact held inputs/root tree
  reconstruct from the retained commit and portable delta, with false controls.
- Publish complete results, manifests, compiler streams and executable adapters in
  the unbounded book; retain325 prior archives and the exact new326th cleanup seal.
  Close only this cache checkpoint and advance to the bounded current suffix replay.


## archogen — remove only idle regenerable compiler caches

`ARCHOGEN-PROGRAM-0814` (leaf `PROGRAM.94`), `2026-10-20`.

- Physical/native inventories agree on five same-volume/no-link/no-Git locations.
  A no-background-job census and immediate identity recheck precede deletion.
- Remove only main/native-sweep incremental caches:2434 files/828209661 logical bytes,
  including60 cached bin files; no logs. Retain evidence, stores and build products.
- Immediate Python/native residue confirms absence; three other inventories remain exact.
  All176 protected proof files and1145 inputs restore; focused regeneration required.
- Seal committed prior cleanup/regression exactly and retain every older archive.
  Synchronize the latest-only record/book; return to M3's frozen complete final replay.


## archogen — guard the middle shared-manifest admission count

`ARCHOGEN-M3-0813` (leaf `M3.6.9.6.3.2.2.15.2.1.4.3.2.2`), `2026-10-20`.

- Add one bounded semantic parent using all12 Cargo-qualified fixture files, targets
  and middle shared closures; both caller orders exercise accepted/refused/recovered phases.
- All seven initial cache survivors now finish as7 intended K/full201, strict0; every kill
  names only the new parent. Production, all older source bytes/operators/reasons unchanged.
- Restore1143 original/copied inputs; preserve48 bindings/37+2 producer sources and18
  premise guards. Reject16 false native/binding,4 fixture and4 input-derivation receipts.
- Publish the missing legacy copied-input helper source and inline maintained checks
  in the new runner; all1143 records agree. Prior measured results remain unchanged.
- Preserve322 prior archives and the exact new323rd seal. Freeze the complete43/full201
  final replay plan in the unbounded book; focused/publication gates required before commit.
  The full cache checkpoint, suffix/global sweeps and independent review remain open.


## archogen — assess current cache edits and own seven survivors

`ARCHOGEN-M3-0812` (leaf `M3.6.9.6.3.2.2.15.2.1.4.3.2.1`), `2026-10-20`.

- Complete43/full200 native assessment:23 existing E,11 intended K,7 S,2 U; strict1.
  All1141 original/copied inputs restore, with opaque gitlink metadata only.
- Independently qualify both U through actual component baseline/mutant/recovery builds;
  twelve false diagnostics refuse. Retain all outcomes, why text and finishing names.
- Cargo-qualified in-memory actual-method observations distinguish all seven survivors;
  own a bounded permanent regression and complete final replay before cache closure.
- Fifteen false native/binding receipts, twelve diagnostic receipts and four input
  reconstruction controls refuse; preserve48 bindings and37+2 producer snapshots.
- Seal the completed plan exactly; preserve prior archives, full parent record and
  complete public evidence/adapters. Remove the stale hand-written planning control count.
  Production, previous tests and reasons unchanged; final publication gates required.


## archogen — freeze the complete cache and current-reason replay plan

`ARCHOGEN-M3-0811` (leaf `M3.6.9.6.3.2.2.15.2.1.4.3.2.0`), `2026-10-20`.

- Actual1414 discovery and anchored capture/lookup/insertion blocks select20 conservative
  cache overlaps; all23 current reason edits form the exact43-entry native plan/full200.
- Retain source blocks, why text, old/current production delta, actual tool identities,
  48 critical bindings/37 producer pins and complete native argv in the unbounded book.
- Reject16 false plans,3 history/6 routing falsifiers; preserve320 prior archives and
  exact new321st seal. Private parent/assessment records route verbatim and reconstruct.
- Source/reasons unchanged; this unit claims no native outcome. Commit reviewed planning
  before the long initial assessment, with all later obligations owned by its parent.

