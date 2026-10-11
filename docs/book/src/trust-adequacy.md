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
all 32 exact route entries have named finishing kills in one strict
native run, exit 0. No entry times out, fails compilation or is unjudged. The complete result identities match
the reviewed census; independent malformed-result checks reject missing, duplicate, foreign and unnamed results.
All three source files restore to their original bytes and match committed source at `2b49ed5`; a separate system
SHA-256 tool agrees. Parent semantic tests have not been strengthened.
The initial parent results below record the remaining repair obligations, not a passing adequacy criterion.

Measured `2026-10-11`, `M3.6.9.4.2.2.2`, on the same native host/toolchain: the exact unmutated parent
command passes 91 tests. Of its 32 exact mutations, 2 have named finishing kills and 30 survive. The raw strict
exit is 1: 30 entries do not meet their expected killed verdict. Every entry compiles and finishes; none times out
or is unjudged. Complete identities, independent result falsifications and source-restoration checks pass.
Original/restored/committed/system source digests match the route measurement and committed `7206818` source.
All 30 survivors have priority P1 and are owned for repair in `M3.6.9.6`, after `.9.5` measures operator reach.
Expected parent verdicts stay killed; calibration does not weaken them.

The suffix below selects `route-<suffix>` or `parent-<suffix>`. Each exact entry retains the final expected-killed
criterion. Initial parent survivors, if measured, remain repair obligations of `M3.6.9.6` after operator
measurements in `.9.5`; their expected verdict is never weakened to make calibration pass.

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
they must not be described as changed acceptance in the later operator-witness table.

## Planning the operator measurements

Generate the candidate manifest with the tracked diagnostic:

```bash
python3 scripts/trust_adequacy.py --self-test
python3 scripts/trust_adequacy.py --output target/trust-adequacy/plan.json
```

The diagnostic reads the reviewed census and exact catalogue twins, and loads the actual sweep's discovery
functions. It compares all raw candidate identities and text with the complete real `--list` output, without
an operator filter. This preserves the global identities that later `--only` commands select. Input hashes
bind the manifest to the source, census, catalogue and sweep. Its boundary and malformed-input controls refuse
population, twin, identity, replacement, ordering and span drift.

Each census block selects the source lines touched by its original characters, including unchanged anchor
context. A terminal newline does not add the next untouched line; an inline fragment selects its enclosing
line. A candidate is selected when its half-open line interval overlaps that span. Execution is deduplicated
by file/candidate identity while every census-row membership remains in the manifest. Such overlap can affect
another judgment on the same lines; it is not a semantic witness.

Measured `2026-10-11`, `M3.6.9.5.1`: the complete listings contain 1,420 candidates for `trust.rs` and 314 for
`trust_generated.rs`. Span selection yields 58 and 61 respectively, 119 unique file/candidates, with 269 row
memberships across the census. Every census row has at least one overlapping candidate. These are planning
counts only: no production compilation, mutation verdict or semantic reach is established here.

## Initial source/provenance operator outcomes

Measured `2026-10-11`, `M3.6.9.5.2.1`, native `aarch64-apple-darwin`, Rust `1.95.0`, production source
identical to committed `5663aa9` and the source digests above: all selected `trust_generated.rs` candidates run
against the route population. The unmutated copy passes. Outcomes: 36 named kills, 17 survivors and 8 unviable
candidates; no timeout or unjudged outcome, raw strict sweep exit 1. This is a completed initial measurement,
not completed adequacy. The copied source restores exactly; all three population source digests independently
match their original, committed and system-tool values. Seven altered actual-result sets are refused: missing,
duplicate, foreign, changed replacement, unnamed kill, wrong terminal count and wrong raw exit.

Measured `2026-10-11`, `M3.6.9.5.2.2`, with the same native host/toolchain and production source identical
to committed `5e78044`: the exact unmutated parent command passes 91 tests. All the same selected candidates
then run against that parent population: 20 named kills, 33 survivors and 8 unviable, raw strict sweep exit 1;
no timeout or unjudged result. Route/parent identity sets, manifest hash and all input digests agree. Original,
restored, committed and independent system digests agree for all three population source files; the copied
source restores exactly. Seven altered actual parent result sets are refused. Parent semantic tests have not
been strengthened. These survivors remain owned for semantic examination and repair, not accepted equivalences.

The tracked runner's seven control methods are also discovered through `scripts/trust_adequacy.sh` by the
integration self-test runner. The selected discovery check passes; a full integration run is still pending.
All survivors are owned for semantic examination in `M3.6.9.5.4`; parent behavioral gaps then belong to `.9.6`.
Both columns now contain actual results for the same candidate identities and manifest. No desired pair of
verdicts establishes a witness. Every unviable candidate below reports `error[E0277]`: `String: Copy` is not
satisfied for the copied-element membership spelling. An unviable attempt proves no semantic coverage.

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

Measured `2026-10-11`, `M3.6.9.5.3.1`, native `aarch64-apple-darwin`, Rust `1.95.0`, production source
identical to committed `5942b3d` and the source digests above: all selected `trust.rs` candidates run against
the route population. The unmutated copy passes. Outcomes: 36 named kills, 18 survivors and 4 unviable;
no timeout or unjudged result, raw strict sweep exit 1. Every selected identity and replacement text is
accounted for. Seven altered actual-result sets refuse. All three population source digests agree across
original/restored/committed/system values, the native producer matches committed bytes and the copy restores.

Measured `2026-10-11`, `M3.6.9.5.3.2`, on the same native host/toolchain and production source identical
to committed `c9ea429`: the exact unmutated parent command passes 91 tests. The same selected candidates
then give 18 named kills, 36 survivors and 4 unviable, raw strict sweep exit 1; no timeout or unjudged result.
Complete route/parent identities, manifest hash and input digests agree. All three population source digests
agree across original/restored/route/committed/system values; the copied source restores and the producer
matches committed bytes. Seven altered actual parent-result sets refuse. All initial dual-population runs
are complete before any parent semantic strengthening; semantic witnesses and behavioral repairs remain open.

Two copied-String membership spellings report `error[E0277]` (`String: Copy`); two direct borrowed-package
comparisons report `error[E0308]` (mismatched types). The original package set contains borrowed strings,
so its direct endpoint comparison has a different reference depth; the copied-element variants compile.
These unviable attempts prove no coverage. Semantic examination of survivors belongs to `.9.5.4`, with parent
behavioral repairs in `.9.6`. Both populations now have actual results; no witness is inferred from a desired verdict pair.
This census-span selection is distinct from the later complete production-range sweep.

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

## Remaining measurements

Use the same manifest for the two populations; each invocation re-derives the full plan and refuses stale
inputs or a changed selection before executing the actual sweep. For example:

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
semantic witness classification remains pending, before adding parent semantic tests. An actual viable semantic witness, or an explicit absence
with its reason, is required; a desired verdict pair alone does not establish the same effect.

The final stage strengthens parent tests and runs complete production sweeps of `trust.rs`, `trust_generated.rs`
and `trust_gate.rs`, plus the shared operators' compatibility sweep of `generated_header.rs`. Every exact parent
entry must have a finishing kill; every sweep survivor needs a justified equivalence record. These stages and
the independent implementation review remain open. No completed catalogue subset establishes full adequacy.
