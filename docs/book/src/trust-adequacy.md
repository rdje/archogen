# Generated-sources test adequacy

The [trust gate](trust.md) judges committed generated sources, their generators and inputs, each reader's roles,
chains and provenance. This chapter separates what its mutation tests have measured from what remains to establish.
The controlling design is `docs/specs/trust/decision_trust-generated-adequacy.md`, accepted after review round 3.

## The criterion and populations

The source-linked census specifies 32 exact route/parent mutation pairs. They are individual replacements,
including diagnostic regressions, rather than 32 independent defect families. Each pair preserves the same
file, original text and replacement; only its test population differs:

| Population | Cargo test arguments | Final criterion |
| --- | --- | --- |
| Route | `-p xtask route_tests` | named finishing kill |
| Parent | `-p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests` | named finishing kill |

These commands use the [pinned Rust toolchain](ledger.md#rust-toolchain).
Both catalogue populations use `--timeouts-fail`: a hang fails, and a command failure without a failed libtest
summary and failure-list name is unjudged. Compilation refusal is an invalid exact catalogue entry, not coverage.
The native harness verifies each replacement lands once and restores the source byte for byte.

The catalogue contract checks both populations against the reviewed identities and frozen finding rows. It
rejects missing, extra or duplicate entries, wrong commands or expected verdicts, and changed twin file/from/to.
Run it separately:

```bash
cargo test -p xtask mutation::adequacy_tests
```

A passing contract establishes catalogue consistency. The actual mutated programs still need their own runs.

## Exact-entry measurements

Measured `2026-10-11`, `M3.6.9.4.2.2.1`, native `aarch64-apple-darwin`, Rust `1.95.0`:
all 32 exact route entries have named finishing kills, strict exit 0. Measured `2026-10-11`,
`M3.6.9.4.2.2.2`, on the same host/toolchain: the exact unmutated parent command passes 91 tests;
its 32 exact mutations give 2 named kills and 30 survivors. Raw strict exit 1 means
30 entries do not meet their expected killed verdict. Every candidate compiles and finishes;
none times out or is unjudged. Complete identities match the reviewed census, and independent
falsifications reject missing, duplicate, foreign or unnamed results. All three source digests agree across
original/restored/committed/system values: route source `2b49ed5`, matching parent source `7206818`.

These initial measurements precede parent strengthening. Every survivor has priority P1 and an owner in
`M3.6.9.6`, after `.9.5`'s operator measurement; calibration never weakens the expected killed verdict.
The suffix selects `route-<suffix>` or `parent-<suffix>`; the following table retains every initial outcome.

| Suffix | Route | Initial parent |
| --- | --- | --- |
| `the-first-generator-alone` | `trust_generated::route_tests::route_a_chain_judges_every_step_s_generator`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `the-last-generator-alone` | `trust_generated::route_tests::route_a_chain_judges_every_step_s_generator`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `the-first-reader-alone` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `the-last-reader-alone` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `a-program-target-judged-for-its-first-reader-alone` | `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `a-program-target-judged-for-its-last-reader-alone` | `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `chain-and-blob-rules-on-each-clause-s-first-entry-alone` | `trust_generated::route_tests::a_vendored_executable_a_script_runs_uncopied_is_believed`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `chain-and-blob-rules-on-each-clause-s-last-entry-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `both-clauses-the-first-input-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `both-clauses-the-last-input-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `both-clauses-the-last-generator-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-program-target-judged-when-first-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-program-target-judged-when-last-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-program-target-built-when-first-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-program-target-built-when-last-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-lone-rs-refused-when-first-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-lone-rs-refused-when-last-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-middle-script-never-judged` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `a-middle-reader-never-judged` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `a-middle-input-never-reached` | `trust_generated::route_tests::route_a_chain_made_in_one_generation`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | survived |
| `an-implementation-reader-never-judged` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | survived |
| `a-marked-build-read-dropped-beside-other-generators` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | survived |
| `only-a-form-s-first-input-reached` | `trust_generated::route_tests::route_a_chain_made_in_one_generation`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | [named kill](#initial-parent-killing-tests) |
| `an-input-reached-through-a-link` | `trust_generated::route_tests::route_a_live_input_link_keeps_its_symbolic_link_refusal` | survived |
| `held-judged-for-the-first-alone` | `trust_generated::route_tests::route_a_generator_build_is_judged_against_every_held_role_package` | survived |
| `a-harness-never-judged-against-a-program-target` | `trust_generated::route_tests::route_a_harness_uses_both_pair_roles_and_judges_foreign_program_targets` | survived |
| `the-last-live-form-alone` | `trust_generated::route_tests::route_every_live_form_contributes_its_generator_provenance` | [named kill](#initial-parent-killing-tests) |
| `a-harness-judged-by-its-first-role-alone` | `trust_generated::route_tests::route_a_harness_uses_both_pair_roles_and_judges_foreign_program_targets` | survived |
| `a-data-reader-never-judged` | `trust_generated::route_tests::route_a_runtime_data_reader_is_judged_for_scripts_and_program_targets` | survived |
| `a-build-read-naming-its-first-generator-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read` | survived |
| `a-middle-generator-never-reached` | `trust_generated::route_tests::route_a_vendored_file_copied`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | survived |
| `a-transitive-generator-build-read-dropped` | `trust_generated::route_tests::route_a_table_a_dependency_s_library_holds_is_made_by_a_package_not_depending_on_it`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read` | survived |

Source SHA-256 values before and after restoration:

| File | SHA-256 |
| --- | --- |
| `xtask/src/trust.rs` | `99ecb50451358f3654538b52f72ab959e0a2629849237779bc69a34bfaea9718` |
| `xtask/src/trust_generated.rs` | `cf10f442e767ada6ae4ee4836a079e41779a16b026555bc42c12dfc185ac1cec` |
| `xtask/src/trust_gate.rs` | `1fddd5ba4c7d996d5ba736c73b6f7d51c7e7ed317f58e26d2135a32ae4f85193` |

To rerun one entry, name its full identity. For example:

```bash
cargo xtask mutate --timeouts-fail --only route-the-first-generator-alone
cargo xtask mutate --timeouts-fail --only parent-the-first-generator-alone
```

The first command runs the route population; the second runs the parent population. Neither single-entry run
establishes the result for the rest of the census.

## Initial parent killing tests

`parent-only-a-form-s-first-input-reached`:

- `trust_generated::provenance_tests::a_data_file_one_side_reads_and_a_copy_of_an_input_are_shared_and_a_plain_copy_is_not`

`parent-the-last-live-form-alone`:

- `trust_generated::gate_tests::an_edit_to_a_declared_generator_is_reported_and_to_an_undeclared_script_is_unchanged`
- `trust_generated::generator_tests::a_generator_build_failing_leaves_step_4_unable_to_judge_with_its_refusals_beside_it`
- `trust_generated::order_tests::step_5_refuses_a_chain_through_a_generator_build_and_an_undeclared_marked_file_one_message_a_file`
- `trust_generated::provenance_tests::a_generator_shared_over_two_inputs_or_with_none_is_shared`
- `trust_generated::provenance_tests::a_third_program_reaching_the_same_provenance_makes_items_of_its_own_and_moves_none`
- `trust_generated::provenance_tests::case_3_two_generated_sources_sharing_an_input_through_two_generators_share_it`
- `trust_generated::provenance_tests::the_harness_reaches_provenance_through_the_units_beside_its_pair_alone`

## Effects and remaining scope

Two exact mutations deliberately hold diagnostics. The input-link mutation still refuses a linked input,
changing its reason from symbolic link to no blob. Naming the first generator alone still refuses a chain,
but loses another generator's attribution. Native original/exact-mutant comparisons establish these effects;
their operator witnesses must retain those diagnostic effects.

## Planning the operator measurements

Generate the candidate manifest with the tracked diagnostic:

```bash
python3 scripts/trust_adequacy.py --self-test
python3 scripts/trust_adequacy.py --output target/trust-adequacy/plan.json
```

The diagnostic binds reviewed census/catalogue twins and source hashes to candidates loaded from the actual
sweep functions. Complete real `--list` output must agree in raw identities, text and order. No operator filter
changes the global identities later selected by `--only`; controls refuse population/twin/span/replacement drift.

Each original character span selects its enclosing half-open source lines, including unchanged anchors.
A terminal newline does not select the next untouched line. Overlapping candidate intervals are deduplicated
by file/identity for execution; every row membership remains. Overlap can change another judgment and is not a witness.

Measured `2026-10-11`, `M3.6.9.5.1`: the complete listings contain 1,420 candidates for `trust.rs` and 314 for
`trust_generated.rs`. Span selection yields 58 and 61 respectively, 119 unique file/candidates, with 269 row
memberships across the census. Every census row has at least one overlapping candidate. These are planning
counts only: no production compilation, mutation verdict or semantic reach is established here.

## Initial source/provenance operator outcomes

All four initial operator runs below use native `aarch64-apple-darwin`, Rust `1.95.0`, and unchanged parent
semantic tests. Each unmutated copy passes; all selected IDs/replacements and finishing names/terminal counts
are audited. Each run rejects seven altered actual-result sets: missing, duplicate, foreign, changed replacement,
unnamed kill, wrong count and wrong raw exit. Every source/copy restores exactly; all three population source
hashes agree across original, committed and independent system values. Paired populations share the manifest,
input digests and candidate identities. No run has a timeout or unjudged result; each raw strict exit is 1.
These completed initial measurements do not establish final adequacy or accepted equivalences.

Measured `2026-10-11`, `M3.6.9.5.2.1`, production source `5663aa9`: all selected `trust_generated.rs`
route candidates give 36 named kills, 17 survivors and 8 unviable. Measured `2026-10-11`, `M3.6.9.5.2.2`,
matching production source `5e78044`: the exact parent baseline passes 91 tests; the same candidates give
20 named kills, 33 survivors and 8 unviable. The latter run's producer matches its committed bytes.
All unviable spellings report `error[E0277]`, `String: Copy`, from copied-element membership comparisons;
compilation refusal proves no coverage. Actual paired outcomes and killing-test keys follow.

The runner's seven control methods are discovered through `scripts/trust_adequacy.sh`; selected discovery
passes, full integration remains pending. Semantic classifications are below; behavioral repairs belong to `.9.6`.

| Operator | Route attempted | Route unviable | Parent attempted | Parent unviable |
| --- | --- | --- | --- | --- |
| `chain` | 13 | 0 | 13 | 0 |
| `cond` | 2 | 0 | 2 | 0 |
| `contains` | 16 | 8 | 16 | 8 |
| `guard` | 12 | 0 | 12 | 0 |
| `iter` | 18 | 0 | 18 | 0 |

Killing-test labels name actual finishing tests in the key below. Identities refer to the unfiltered complete
listing, not an operator-specific list. Each selected candidate appears once; the manifest retains all census
rows it overlaps. Exact span overlap alone does not classify its semantic effect.

| Candidate | Initial route | Initial parent |
| --- | --- | --- |
| `L172.guard.67` | survived | survived |
| `L172.guard.68` | survived | survived |
| `L172.guard.69` | survived | survived |
| `L271.iter.100` | killed (gr19, gr23) | killed (gp18) |
| `L271.iter.101` | survived | survived |
| `L271.iter.102` | survived | survived |
| `L272.iter.103` | killed (gr4, gr9, gr10, gr17, gr19, gr23) | killed (gp7, gp12, gp18) |
| `L272.iter.104` | killed (gr1, gr2, gr4, gr10, gr23) | killed (gp18) |
| `L272.iter.105` | survived | survived |
| `L273.iter.106` | killed (gr1, gr23) | survived |
| `L273.iter.107` | killed (gr23) | survived |
| `L273.iter.108` | killed (gr23) | survived |
| `L273.guard.109` | survived | survived |
| `L273.guard.110` | survived | survived |
| `L273.guard.111` | survived | survived |
| `L273.guard.112` | killed (gr23) | survived |
| `L273.guard.113` | killed (gr23) | survived |
| `L273.guard.114` | killed (gr23) | survived |
| `L273.guard.115` | killed (gr23) | survived |
| `L273.guard.116` | killed (gr23) | survived |
| `L273.guard.117` | killed (gr23) | survived |
| `L278.cond.118` | killed (gr1, gr2, gr3, gr4, gr5, gr6, gr7, gr8, gr9, gr10, gr11, gr12, gr13, gr14, gr15, gr16, gr17, gr18, gr19, gr20, gr21, gr22, gr23) | killed (gp1, gp2, gp3, gp4, gp5, gp6, gp7, gp8, gp9, gp10, gp11, gp12, gp13, gp14, gp15, gp16, gp17, gp18, gp19) |
| `L278.cond.119` | killed (gr9) | killed (gp18) |
| `L339.iter.130` | killed (gr19, gr21) | survived |
| `L339.iter.131` | killed (gr21) | survived |
| `L339.iter.132` | survived | survived |
| `L340.iter.133` | killed (gr2, gr4, gr9, gr10, gr12, gr17, gr18, gr19, gr22) | killed (gp14, gp17) |
| `L340.iter.134` | killed (gr1, gr2, gr4, gr5, gr6, gr8, gr9, gr10, gr12, gr13, gr14, gr15, gr16, gr18, gr19, gr20, gr21, gr22) | killed (gp3, gp15, gp16, gp17, gp19) |
| `L340.iter.135` | survived | survived |
| `L341.iter.136` | killed (gr4, gr5, gr10, gr12, gr18, gr19, gr20, gr22) | killed (gp14) |
| `L341.iter.137` | killed (gr4, gr5, gr10, gr12, gr18, gr20, gr22) | killed (gp14) |
| `L341.iter.138` | killed (gr4, gr18, gr22) | survived |
| `L191.chain.219` | killed (gr5, gr23) | survived |
| `L191.chain.220` | killed (gr23) | survived |
| `L192.chain.221` | survived | survived |
| `L192.chain.222` | survived | survived |
| `L192.chain.223` | killed (gr1, gr2, gr3, gr4, gr5, gr6, gr7, gr8, gr9, gr10, gr11, gr12, gr13, gr14, gr15, gr16, gr17, gr18, gr19, gr20, gr21, gr22, gr23) | killed (gp1, gp2, gp3, gp4, gp5, gp6, gp7, gp8, gp9, gp10, gp11, gp12, gp13, gp14, gp15, gp16, gp17, gp18, gp19, gp20, gp21, gp22, gp23, gp24) |
| `L192.contains.224` | killed (gr23) | survived |
| `L192.contains.225` | unviable (E0277) | unviable (E0277) |
| `L192.contains.226` | killed (gr23) | survived |
| `L192.contains.227` | unviable (E0277) | unviable (E0277) |
| `L253.chain.243` | killed (gr19, gr21, gr23) | killed (gp3, gp6, gp15, gp16, gp17, gp18, gp19) |
| `L253.chain.244` | killed (gr21) | killed (gp3, gp6, gp13, gp15, gp16, gp17, gp19) |
| `L253.chain.245` | survived | killed (gp6, gp11, gp18) |
| `L253.contains.246` | killed (gr7, gr8, gr11, gr14, gr15, gr16, gr21) | killed (gp6, gp10, gp19) |
| `L253.contains.247` | unviable (E0277) | unviable (E0277) |
| `L253.contains.248` | killed (gr1, gr2, gr3, gr4, gr5, gr6, gr9, gr10, gr12, gr13, gr17, gr18, gr19, gr20, gr21, gr22, gr23) | killed (gp3, gp4, gp5, gp6, gp7, gp8, gp9, gp11, gp12, gp13, gp14, gp15, gp16, gp17, gp18, gp19) |
| `L253.contains.249` | unviable (E0277) | unviable (E0277) |
| `L278.contains.252` | survived | survived |
| `L278.contains.253` | unviable (E0277) | unviable (E0277) |
| `L278.contains.254` | survived | survived |
| `L278.contains.255` | unviable (E0277) | unviable (E0277) |
| `L339.chain.265` | killed (gr19, gr21) | killed (gp3, gp15, gp16, gp17, gp19) |
| `L339.chain.266` | killed (gr21) | killed (gp3, gp15, gp16, gp17, gp19) |
| `L339.chain.267` | killed (gr19, gr21) | survived |
| `L339.chain.268` | killed (gr21) | survived |
| `L339.chain.269` | survived | killed (gp14, gp15, gp16, gp17, gp19) |
| `L339.contains.270` | killed (gr8, gr14, gr15, gr16, gr21) | killed (gp19) |
| `L339.contains.271` | unviable (E0277) | unviable (E0277) |
| `L339.contains.272` | killed (gr1, gr2, gr4, gr5, gr6, gr9, gr10, gr12, gr13, gr17, gr18, gr19, gr20, gr21, gr22) | killed (gp3, gp14, gp15, gp16, gp17, gp19) |
| `L339.contains.273` | unviable (E0277) | unviable (E0277) |

### Source/provenance route killing tests

| Label | Actual finishing test |
| --- | --- |
| gr1 | `trust_generated::route_tests::a_vendored_executable_a_script_runs_uncopied_is_believed` |
| gr2 | `trust_generated::route_tests::no_route_for_a_marked_crate_root_but_the_parent_s_belief_in_a_script` |
| gr3 | `trust_generated::route_tests::route_a_chain_judges_every_step_s_generator` |
| gr4 | `trust_generated::route_tests::route_a_chain_made_in_one_generation` |
| gr5 | `trust_generated::route_tests::route_a_file_one_step_runs_and_another_reads_is_named_once` |
| gr6 | `trust_generated::route_tests::route_a_generator_build_is_judged_against_every_held_role_package` |
| gr7 | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader` |
| gr8 | `trust_generated::route_tests::route_a_harness_uses_both_pair_roles_and_judges_foreign_program_targets` |
| gr9 | `trust_generated::route_tests::route_a_live_input_link_keeps_its_symbolic_link_refusal` |
| gr10 | `trust_generated::route_tests::route_a_marked_input_or_script_remade_in_the_generation` |
| gr11 | `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` |
| gr12 | `trust_generated::route_tests::route_a_program_target_step_reads_the_intermediate_at_run_time` |
| gr13 | `trust_generated::route_tests::route_a_program_that_generates_its_own_source` |
| gr14 | `trust_generated::route_tests::route_a_runtime_data_reader_is_judged_for_scripts_and_program_targets` |
| gr15 | `trust_generated::route_tests::route_a_table_a_dependency_s_library_holds_is_made_by_a_package_not_depending_on_it` |
| gr16 | `trust_generated::route_tests::route_a_table_its_package_s_library_holds_is_made_by_another_package` |
| gr17 | `trust_generated::route_tests::route_a_tool_pin` |
| gr18 | `trust_generated::route_tests::route_a_vendored_file_copied` |
| gr19 | `trust_generated::route_tests::route_an_intermediate_a_program_reads_is_remade_by_the_later_step` |
| gr20 | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read` |
| gr21 | `trust_generated::route_tests::route_every_live_form_contributes_its_generator_provenance` |
| gr22 | `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` |
| gr23 | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` |

### Source/provenance parent killing tests

| Label | Actual finishing test |
| --- | --- |
| gp1 | `trust::tests::a_file_a_root_compiles_whose_header_marks_it_generated_is_refused_until_declared` |
| gp2 | `trust_generated::gate_tests::a_defgenerated_form_added_changed_or_removed_is_in_the_change_part_and_every_one_is_standing` |
| gp3 | `trust_generated::gate_tests::an_edit_to_a_declared_generator_is_reported_and_to_an_undeclared_script_is_unchanged` |
| gp4 | `trust_generated::generator_tests::a_crate_root_of_two_targets_is_judged_by_the_union_of_their_role_packages` |
| gp5 | `trust_generated::generator_tests::a_crate_root_that_is_not_rs_is_refused_before_its_build` |
| gp6 | `trust_generated::generator_tests::a_generator_build_failing_leaves_step_4_unable_to_judge_with_its_refusals_beside_it` |
| gp7 | `trust_generated::generator_tests::a_generator_that_is_a_root_is_its_own_build_and_one_a_live_form_s_refusal_names_is_never_built` |
| gp8 | `trust_generated::generator_tests::an_allowed_generator_is_built_and_its_sites_judged_with_the_programs_a_shared_site_admitted_once` |
| gp9 | `trust_generated::generator_tests::another_role_s_program_target_script_or_library_is_refused_as_a_generator` |
| gp10 | `trust_generated::generator_tests::the_harness_may_use_its_pair_s_roles_and_no_other` |
| gp11 | `trust_generated::order_tests::a_form_no_program_reads_is_stale_on_the_baseline_s_host_alone` |
| gp12 | `trust_generated::order_tests::a_refusal_at_each_of_the_five_steps_ends_the_run_and_no_form_is_stale` |
| gp13 | `trust_generated::order_tests::step_5_refuses_a_chain_through_a_generator_build_and_an_undeclared_marked_file_one_message_a_file` |
| gp14 | `trust_generated::provenance_tests::a_data_file_one_side_reads_and_a_copy_of_an_input_are_shared_and_a_plain_copy_is_not` |
| gp15 | `trust_generated::provenance_tests::a_generator_shared_over_two_inputs_or_with_none_is_shared` |
| gp16 | `trust_generated::provenance_tests::a_third_program_reaching_the_same_provenance_makes_items_of_its_own_and_moves_none` |
| gp17 | `trust_generated::provenance_tests::case_3_two_generated_sources_sharing_an_input_through_two_generators_share_it` |
| gp18 | `trust_generated::provenance_tests::step_3_refuses_a_live_form_s_non_blob_and_its_chain_and_holds_a_form_no_program_reads_to_its_text` |
| gp19 | `trust_generated::provenance_tests::the_harness_reaches_provenance_through_the_units_beside_its_pair_alone` |
| gp20 | `trust_generated::tests::a_departure_found_once_the_file_is_read_has_every_refusal_beside_it` |
| gp21 | `trust_generated::tests::a_refusal_of_the_forms_text_ends_the_run_before_anything_is_written_or_built` |
| gp22 | `trust_generated::tests::a_well_formed_declaration_is_read_and_refuses_nothing` |
| gp23 | `trust_generated::tests::every_text_only_construction_is_a_coded_refusal` |
| gp24 | `trust_generated::tests::the_parent_s_reader_stops_at_another_form_with_the_refusals_before_it_beside_it` |

## Initial role and build operator outcomes

Measured `2026-10-11`, `M3.6.9.5.3.1`, production source `5942b3d`: all selected `trust.rs` route
candidates give 36 named kills, 18 survivors and 4 unviable. Measured `2026-10-11`, `M3.6.9.5.3.2`,
matching production source `c9ea429`: the exact parent baseline passes 91 tests; the same candidates give
18 named kills, 36 survivors and 4 unviable. Both producers match committed bytes; the shared audits above
hold for each. These span-cohort results precede parent strengthening and the complete production sweeps.

Two copied-String spellings report `error[E0277]` (`String: Copy`); two direct borrowed-package comparisons
report `error[E0308]` (mismatched types). The package set contains borrowed strings, so direct endpoints have
an extra reference depth; copied-element variants compile. Unviable attempts and desired verdict pairs prove
no semantic witness. Both populations' actual outcomes and killing names follow.

| Operator | Route attempted | Route unviable | Parent attempted | Parent unviable |
| --- | --- | --- | --- | --- |
| `chain` | 9 | 0 | 9 | 0 |
| `clogic` | 2 | 0 | 2 | 0 |
| `cond` | 2 | 0 | 2 | 0 |
| `contains` | 8 | 4 | 8 | 4 |
| `eq` | 3 | 0 | 3 | 0 |
| `guard` | 12 | 0 | 12 | 0 |
| `iter` | 21 | 0 | 21 | 0 |
| `logic` | 1 | 0 | 1 | 0 |

Killing-test labels reuse the earlier `gr` key. The identities come from the unfiltered complete listing. The parent column records the matching native measurement.

| Candidate | Initial route | Initial parent |
| --- | --- | --- |
| `L1838.eq.734` | killed (gr8) | killed (gp10) |
| `L1838.eq.735` | killed (gr8) | killed (gp10) |
| `L1838.logic.736` | killed (gr8) | killed (gp10) |
| `L1851.iter.740` | killed (gr3, gr20, gr23) | survived |
| `L1851.iter.741` | killed (gr3, gr20, gr23) | survived |
| `L1851.iter.742` | killed (gr23) | survived |
| `L1851.guard.743` | survived | survived |
| `L1851.guard.744` | survived | survived |
| `L1851.guard.745` | survived | survived |
| `L1855.iter.746` | killed (gr7, gr11) | survived |
| `L1855.iter.747` | killed (gr7, gr11) | survived |
| `L1855.iter.748` | killed (gr7, gr11) | survived |
| `L1857.iter.749` | killed (gr6) | survived |
| `L1857.iter.750` | survived | survived |
| `L1857.iter.751` | survived | survived |
| `L1890.iter.769` | survived | survived |
| `L1890.iter.770` | survived | survived |
| `L1890.iter.771` | survived | survived |
| `L1890.guard.772` | survived | survived |
| `L1890.guard.773` | survived | survived |
| `L1890.guard.774` | survived | survived |
| `L1890.guard.775` | survived | survived |
| `L1890.guard.776` | survived | survived |
| `L1890.guard.777` | survived | survived |
| `L1940.iter.803` | killed (gr15, gr16, gr20) | killed (gp13) |
| `L1940.iter.804` | killed (gr15, gr16, gr20) | killed (gp8, gp13) |
| `L1940.iter.805` | killed (gr15, gr20) | survived |
| `L1941.iter.806` | killed (gr15, gr16) | survived |
| `L1941.iter.807` | killed (gr23) | killed (gp13) |
| `L1941.iter.808` | survived | survived |
| `L1954.eq.809` | killed (gr1, gr2, gr3, gr4, gr5, gr7, gr8, gr9, gr10, gr12, gr14, gr17, gr18, gr19, gr21, gr22, gr23) | killed (gp1, gp2, gp3, gp6, gp9, gp10, gp11, gp13, gp14, gp15, gp16, gp17, gp18, gp19) |
| `L2006.guard.823` | killed (gr20) | killed (gp13) |
| `L2006.guard.824` | killed (gr20, gr23) | killed (gp13) |
| `L2006.guard.825` | killed (gr20) | survived |
| `L2009.cond.826` | killed (gr6, gr8, gr12, gr13, gr15, gr16, gr18, gr20) | killed (gp4, gp7, gp8) |
| `L2009.cond.827` | killed (gr20, gr23) | killed (gp13) |
| `L2014.iter.829` | killed (gr20) | survived |
| `L2014.iter.830` | killed (gr20) | survived |
| `L2014.iter.831` | survived | survived |
| `L1838.chain.1214` | killed (gr8) | survived |
| `L1838.chain.1215` | killed (gr8) | killed (gp10) |
| `L1838.chain.1216` | killed (gr8) | killed (gp10) |
| `L1849.chain.1219` | killed (gr7, gr11) | survived |
| `L1849.chain.1220` | killed (gr7, gr11) | survived |
| `L1849.chain.1221` | killed (gr6, gr8, gr13, gr15, gr23) | killed (gp7, gp10) |
| `L1849.contains.1222` | killed (gr7, gr8, gr11, gr14) | killed (gp10) |
| `L1849.contains.1223` | unviable (E0277) | unviable (E0277) |
| `L1849.contains.1224` | killed (gr3, gr6, gr23) | killed (gp4, gp9, gp12) |
| `L1849.contains.1225` | unviable (E0277) | unviable (E0277) |
| `L1877.chain.1236` | killed (gr6) | survived |
| `L1877.chain.1237` | survived | survived |
| `L1877.chain.1238` | survived | survived |
| `L1877.contains.1239` | unviable (E0308) | unviable (E0308) |
| `L1877.contains.1240` | killed (gr6, gr8, gr11, gr14) | survived |
| `L1877.contains.1241` | unviable (E0308) | unviable (E0308) |
| `L1877.contains.1242` | killed (gr11, gr23) | killed (gp4, gp9, gp12) |
| `L1838.clogic.1410` | killed (gr8) | killed (gp10) |
| `L1838.clogic.1411` | killed (gr8) | survived |

Parent killing-test labels reuse the earlier `gp` key.

## Syntax evidence before semantic classification

Measured `2026-10-11`, `M3.6.9.5.4.1`: `scripts/trust_witness.py` re-audits all four initial native
logs against the complete plan, exact commands, source hashes, result identities, replacement text, finishing
names and terminal counts. Current, recorded-commit and restored-copy bytes agree for all three population
sources. Its output retains every exact-entry and operator source diff, mutated-source digest and paired outcome.
It also records the formatter version, Cargo edition and toolchain-input digests.

The diagnostic feeds each full mutated source to `rustfmt --emit stdout` on stdin, using the xtask crate's
declared edition and `skip_children=true`. All 32 exact replacements and 119 selected candidates parse and
format. This does not establish compilation: the previously measured type refusals still remain unviable.
Four entries have a candidate with identical formatted full-source bytes:

| Exact-entry suffix | Candidate | Initial route / parent |
| --- | --- | --- |
| `the-first-reader-alone` | `L1849.chain.1219` | killed / survived |
| `both-clauses-the-last-generator-alone` | `L191.chain.220` | killed / survived |
| `the-last-live-form-alone` | `L253.chain.244` | killed / killed |
| `a-harness-judged-by-its-first-role-alone` | `L1838.clogic.1411` | killed / survived |

Syntax identity alone does not complete the semantic relation. Position-flag loops may have the same effect
with different syntax; overlapping edits may change another judgment. A missing syntax match proves no semantic
absence. The explicit effect comparisons are in the next section.

Reproduce after the four preceding native measurements, with their original production sources restored:

```bash
python3 scripts/trust_witness.py --plan target/trust-adequacy/plan.json \
  --measurement target/trust-adequacy/generated-route.json \
  --measurement target/trust-adequacy/generated-parent.json \
  --measurement target/trust-adequacy/trust-route.json \
  --measurement target/trust-adequacy/trust-parent.json \
  --output target/trust-adequacy/witness-evidence.json
bash scripts/trust_witness.sh --self-test
```

The controls distinguish layout from changed literal contents or conditions, reject parser errors as identity,
and refuse wrong original spans, replacement text, populations, selections, source bindings or altered results.
The diagnostic never changes production or fixture sources. Its recorded historical runner hashes are retained
as provenance; re-auditing logs does not reconstruct or authenticate an earlier uncommitted runner version.

## Semantic witnesses in the initial cohort

Reviewed `2026-10-11`, `M3.6.9.5.4.2`: 13 rows have viable same-effect witnesses; 19 rows have none
in their measured replacement-span cohorts. This is a bounded relation between the actual edits above,
not a claim that another edit elsewhere could never express the defect. All initial candidate outcomes and
killing-test names remain in the preceding tables, including different defects and type refusals.

The classifications are explicit reviewed assertions in `scripts/trust_witness_effects.py`. The diagnostic
re-audits the original source/syntax/native evidence, checks complete census membership, requires each witness
to belong to its row's cohort and have viable outcomes in both populations, and retains those actual outcomes.
It never selects a witness from a desired verdict pair. Its compiler controls use actual exact/operator
replacement text in typed selection traces; these are bounded controls, separate from the native inventory runs.

| Exact-entry suffix | Viable witness and initial route / parent, or scoped absence |
| --- | --- |
| `the-first-generator-alone` | `L1851.iter.740`; effect; killed / survived |
| `the-last-generator-alone` | `L1851.iter.741`; effect; killed / survived |
| `the-first-reader-alone` | `L1849.chain.1219`; syntax; killed / survived |
| `the-last-reader-alone` | `L1849.chain.1220`; effect; killed / survived |
| `a-program-target-judged-for-its-first-reader-alone` | none; reader-specific result filtering absent; guards change build continuation |
| `a-program-target-judged-for-its-last-reader-alone` | none; reader-specific result filtering absent; guards change build continuation |
| `chain-and-blob-rules-on-each-clause-s-first-entry-alone` | `L273.iter.106`; effect; killed / survived |
| `chain-and-blob-rules-on-each-clause-s-last-entry-alone` | `L273.iter.107`; effect; killed / survived |
| `both-clauses-the-first-input-alone` | `L192.contains.224`; effect; killed / survived |
| `both-clauses-the-last-input-alone` | `L192.contains.226`; effect; killed / survived |
| `both-clauses-the-last-generator-alone` | `L191.chain.220`; syntax; killed / survived |
| `a-program-target-judged-when-first-alone` | none; if-let branch absent from one-line boolean guard shapes |
| `a-program-target-judged-when-last-alone` | none; if-let branch absent from one-line boolean guard shapes |
| `a-program-target-built-when-first-alone` | none; generator-file position gate absent; cohort narrows owned targets or build guards |
| `a-program-target-built-when-last-alone` | none; generator-file position gate absent; cohort narrows owned targets or build guards |
| `a-lone-rs-refused-when-first-alone` | none; multiline else-if guard absent; only extension equality changes |
| `a-lone-rs-refused-when-last-alone` | none; multiline else-if guard absent; only extension equality changes |
| `a-middle-script-never-judged` | none; script-specific position predicate absent; iterator ends also drop middle Rust targets |
| `a-middle-reader-never-judged` | none; chain has first/last but no two-ends variant; other candidates change generators or membership |
| `a-middle-input-never-reached` | none; input-only position predicate absent; iterator ends narrow both roles |
| `an-implementation-reader-never-judged` | none; implementation-role predicate absent; iterator positions are role-independent |
| `a-marked-build-read-dropped-beside-other-generators` | none; form-generator association predicate absent; guards narrow by build-read position |
| `only-a-form-s-first-input-reached` | none; input-only first predicate absent; iterator first narrows both roles |
| `an-input-reached-through-a-link` | none; input-only link predicate absent; guards/endpoints/constants also affect generator links |
| `held-judged-for-the-first-alone` | `L1857.iter.749`; effect; killed / survived |
| `a-harness-never-judged-against-a-program-target` | none; harness-reader predicate absent; cohort changes held packages or continuation |
| `the-last-live-form-alone` | `L253.chain.244`; syntax; killed / killed |
| `a-harness-judged-by-its-first-role-alone` | `L1838.clogic.1411`; syntax; killed / survived |
| `a-data-reader-never-judged` | none; runtime-data reader predicate absent; chain/membership endpoints are role-independent |
| `a-build-read-naming-its-first-generator-alone` | `L2014.iter.829`; effect; killed / survived |
| `a-middle-generator-never-reached` | none; generator-only position predicate absent; iterator ends narrow inputs too |
| `a-transitive-generator-build-read-dropped` | none; dependency-depth predicate absent; cohort narrows unit/file positions |

The `syntax` rows have identical formatted full-source replacements and viable native outcomes. The `effect`
rows have these additional source-level arguments:

- The generator, clause-file, held-package and attribution loops iterate immutable borrowed vectors or sets.
  Their iterator advancement and lookahead have no user code or observable item destruction. First/last flags
  therefore run exactly the retained element's original body in the same order as the corresponding
  `take(1)` or `rev().take(1)` edit. Helper names are fresh; every other body statement remains unchanged.
  The attribution witness retains the first refusal while losing later generator names, the diagnostic effect
  established by the native shared-read fixture. It does not demonstrate acceptance of that build read.
- The last-reader chain reverses the filtered borrowed program vector directly; the exact entry first collects
  that same filtered sequence and then reverses it. The predicate only reads membership in immutable sets.
  Both choose the same last matching reader, with no predicate side effects or owned-item destruction.
- The direct input-membership candidates compare the same first/last `String` value as the exact entries.
  The `Some(g)` argument coerces to the same borrowed string as explicit `Some(*g)`; the actual native
  candidates compile. The copied alternatives fail `String: Copy` and provide no witness evidence.

Compiled selection controls exercise the actual loop edits over empty through five-element vectors/sets,
reader masks and membership masks, including unmatched input endpoints and repeated input values.
They also retain concrete distinctions behind the absence rows:

- A middle `.rs` generator remains under the exact middle-script omission; a generic two-ends generator loop
  drops it. A middle-reader omission retains both end readers; the two chain candidates retain one end each.
- The exact middle-input, first-input and middle-generator provenance edits retain every entry of the other
  role. The generic file loop narrows both roles. Distinct three-entry generator/input clauses expose the
  difference for all three edits; the initially killed exact first-input parent remains valid evidence.
- A depth gate retains own/direct units and drops transitive ones. Three units with one file each expose the
  difference from each of the three unit-position and three file-position candidates in that cohort.

The remaining absence reasons follow the actual changed conditions. Program-target reader edits change the
refusal set, while the overlapping generator guard changes continuation after refusals have been recorded.
A target-build gate tests the generator file's position, while the owned-target cohort tests target positions,
deduplication or manifest refusal. Role/data/harness/link and form-association edits introduce predicates absent
from their candidates; permuting positions cannot reproduce a predicate tied to that role or association.
For example, a singleton linked input still reaches every first/last/ends guard, while the exact input-role edit
skips its link diagnostic. A constant false link condition also loses generator-link diagnostics. Both still
reach the no-blob refusal where applicable. The multiline lone-`.rs` branch only has an equality-flip candidate,
which changes the file kind independently of its position. The `if let` branches are outside the promised
one-line boolean guard shape. None of these reasons treats a type refusal or an overlapping kill as coverage.

Reproduce against the unchanged initial sources and the preceding evidence artifact:

```bash
python3 scripts/trust_witness_effects.py \
  --evidence target/trust-adequacy/witness-evidence.json \
  --output target/trust-adequacy/witness-classification.json
bash scripts/trust_witness_effects.sh --self-test
```

The controls compile and finish successfully; a substituted last-element candidate for a first-element
claim, and an inverted equality assertion, each compile but fail their trace assertion. This holds the
controls' sensitivity. It is not an exhaustive semantic prover. All exact parent expected-killed entries
remain required, including those without operator witnesses, before the final complete production sweeps.

## Promised narrowing spellings

Verified `2026-10-11`, `M3.6.9.5.4.3`: every selected iterator and guard family has distinct first,
last and two-ends spellings, each viable in both initial native populations. The 13 iterator families contribute
39 candidates; the eight guard families contribute 24. This accounts for 126 native attempts, with no compilation
refusal, timeout or unjudged result among those spellings. It does not turn their survivors into successful kills.

| Source | Iterator families | Guard families | Viable spellings per population |
| --- | --- | --- | --- |
| `xtask/src/trust.rs` | 7 | 4 | 33 |
| `xtask/src/trust_generated.rs` | 6 | 4 | 30 |

The tracked audit independently enumerates the plain one-line for headers inside the reviewed exact-entry
spans and matches all of them to the selected iterator families. It checks the actual position gates and all
paired native outcomes; full source/plan/log binding runs first. Its controls refuse a missing position or
entire iterator family, duplicate position, missing population, unviable spelling, timeout and unjudged outcome.
This header census is specific to these reviewed source spans; the sweep's general lexical controls remain separate.

```bash
python3 scripts/trust_witness_effects.py --spellings \
  --evidence target/trust-adequacy/witness-evidence.json \
  --output target/trust-adequacy/narrowing-spellings.json
```

No spelling repair is needed in this cohort. The semantic table's absent role predicates, multiline and
if-let shapes, and two-ends chain variant remain explicit limits rather than missing promised loop spellings.
The initial measurements preceded the parent repairs below; final census and full production sweeps remain.

## Parent repairs

Verified `2026-10-11`, `M3.6.9.6.1`: the eight owned source/blob/chain/provenance parent entries now have
named finishing kills in one strict native run, exit 0. `xtask/src/trust_generated_adequacy_tests.rs` adds parent
controls for overlap at every generator/input position; plain versus missing, linked, declared or marked entries
in both clauses; and complete provenance paths, content hashes, merged roles and reached declaration hashes.
A link is absent from the regular-file tree: the test requires its link diagnostic, while the no-blob fallback
would still refuse it. Reached forms include other-program and dormant controls. All source bytes restore exactly.

```bash
cargo test -p xtask trust_generated::adequacy_tests
cargo xtask mutate --timeouts-fail --only parent-a-middle-generator-never-reached
```

Verified `2026-10-11`, `M3.6.9.6.2.2`: all nine generator-position parent entries now have named finishing
kills, strict exit 0. `xtask/src/trust_adequacy_parent_tests.rs` compares complete refusal sets for three scripts,
foreign target entry points and lone Rust files. Neutral/own-role scripts pass; foreign targets contain invalid
code and must refuse before compilation. All three allowed targets build: each unadmitted source site refuses
with its identity/hash, then separate admissions are all used and the inventory counts every site. The parent
baseline passes 98 / 0 failed; source/system digests agree after restoration. Production code is unchanged.

```bash
cargo test -p xtask trust::adequacy_tests
cargo xtask mutate --timeouts-fail --only parent-a-program-target-built-when-first-alone
```

Verified `2026-10-11`, `M3.6.9.6.2.3`: all ten reader/role entries now have named finishing parent kills,
strict exit 0. The same parent test module requires every compiled reader's script/target refusals, including
implementation and middle readers; a later foreign held package after an allowed one; both harness pair roles
allowed and both third-role generator kinds refused; and runtime-only data readers judged for both kinds.
Complete diagnostic sets hold attribution even when another refusal remains. The parent baseline passes
102 / 0 failed, all source hashes restore and production judgments stay unchanged.

Verified `2026-10-11`, `M3.6.9.6.2.4`: all three build-read entries have named finishing parent kills,
strict exit 0. Three tools share a transitive dependency beside a plain script; the plain dependency passes.
Marked and declared variants require every tool's exact chain refusal, while no reading root compiles that file.
The parent baseline passes 103 / 0 failed; source/system digests agree after restoration. All initial survivor
repairs are individually verified. The combined all-32 exact census is next; complete sweeps and review remain.

The initial tables remain historical measurements of the earlier parent population. Production judgments and
route controls are unchanged. The final exact census/full sweeps remain open.

## Remaining measurements

The span-cohort procedure uses the same manifest for both populations; each invocation re-derives the plan and
refuses stale inputs or a changed selection before executing the actual sweep. For example:

```bash
python3 scripts/trust_adequacy.py --plan target/trust-adequacy/plan.json \
  --measure xtask/src/trust_generated.rs --population route \
  --output target/trust-adequacy/generated-route.json
python3 scripts/trust_adequacy.py --plan target/trust-adequacy/plan.json \
  --measure xtask/src/trust_generated.rs --population parent \
  --output target/trust-adequacy/generated-parent.json
```

Run them sequentially: both use the sweep's same copy and build directory. The runner stores the exact command,
input/producer/manifest digests, source commit and raw log beside a structured result. It verifies every selected
identity and actual replacement text, requires finishing test names for kills, compares terminal outcome and
per-operator counts, preserves the raw strict sweep exit, and checks source/copy restoration. The process status
of this diagnostic reports whether measurement and audit completed; the JSON's `raw_exit_code` is the sweep's
criterion status. A completed measurement with survivors remains an unmet adequacy criterion.

Operator reach remains incomplete. All initial source/provenance and trust.rs populations are measured above;
semantic witnesses, scoped absences and promised-spelling viability are recorded above. No operator repair was
needed before parent strengthening; these initial results do not satisfy the final adequacy criterion.

Parent repairs precede the final combined exact census and complete production sweeps of `trust.rs`,
`trust_generated.rs` and `trust_gate.rs`, plus the shared operators' compatibility sweep of `generated_header.rs`. Every exact parent
entry must have a finishing kill; every sweep survivor needs a justified equivalence record. These stages and
the independent implementation review remain open. No completed catalogue subset establishes full adequacy.
