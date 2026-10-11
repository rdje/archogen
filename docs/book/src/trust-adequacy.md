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

## Remaining measurements

Operator reach remains unmeasured. The next stage runs every candidate on every census replacement span against
both populations before adding parent semantic tests. An actual viable semantic witness, or an explicit absence
with its reason, is required; a desired verdict pair alone does not establish the same effect.

The final stage strengthens parent tests and runs complete production sweeps of `trust.rs`, `trust_generated.rs`
and `trust_gate.rs`, plus the shared operators' compatibility sweep of `generated_header.rs`. Every exact parent
entry must have a finishing kill; every sweep survivor needs a justified equivalence record. These stages and
the independent implementation review remain open. No completed catalogue subset establishes full adequacy.
