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
SHA-256 tool agrees. Parent semantic tests have not been strengthened, and parent outcomes remain pending.
A pending cell is no verdict or adequacy claim.

The suffix below selects `route-<suffix>` or `parent-<suffix>`. Each exact entry retains the final expected-killed
criterion. Initial parent survivors, if measured, remain repair obligations of `M3.6.9.6` after operator
measurements in `.9.5`; their expected verdict is never weakened to make calibration pass.

| Suffix | Route | Initial parent |
| --- | --- | --- |
| `the-first-generator-alone` | `trust_generated::route_tests::route_a_chain_judges_every_step_s_generator`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `the-last-generator-alone` | `trust_generated::route_tests::route_a_chain_judges_every_step_s_generator`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `the-first-reader-alone` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `the-last-reader-alone` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `a-program-target-judged-for-its-first-reader-alone` | `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `a-program-target-judged-for-its-last-reader-alone` | `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `chain-and-blob-rules-on-each-clause-s-first-entry-alone` | `trust_generated::route_tests::a_vendored_executable_a_script_runs_uncopied_is_believed`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `chain-and-blob-rules-on-each-clause-s-last-entry-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `both-clauses-the-first-input-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `both-clauses-the-last-input-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `both-clauses-the-last-generator-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-program-target-judged-when-first-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-program-target-judged-when-last-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-program-target-built-when-first-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-program-target-built-when-last-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-lone-rs-refused-when-first-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-lone-rs-refused-when-last-alone` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-middle-script-never-judged` | `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `a-middle-reader-never-judged` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `a-middle-input-never-reached` | `trust_generated::route_tests::route_a_chain_made_in_one_generation`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | pending |
| `an-implementation-reader-never-judged` | `trust_generated::route_tests::route_a_generator_is_judged_against_every_reader`, `trust_generated::route_tests::route_a_program_target_generator_is_judged_against_every_reader` | pending |
| `a-marked-build-read-dropped-beside-other-generators` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read`, `trust_generated::route_tests::route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | pending |
| `only-a-form-s-first-input-reached` | `trust_generated::route_tests::route_a_chain_made_in_one_generation`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | pending |
| `an-input-reached-through-a-link` | `trust_generated::route_tests::route_a_live_input_link_keeps_its_symbolic_link_refusal` | pending |
| `held-judged-for-the-first-alone` | `trust_generated::route_tests::route_a_generator_build_is_judged_against_every_held_role_package` | pending |
| `a-harness-never-judged-against-a-program-target` | `trust_generated::route_tests::route_a_harness_uses_both_pair_roles_and_judges_foreign_program_targets` | pending |
| `the-last-live-form-alone` | `trust_generated::route_tests::route_every_live_form_contributes_its_generator_provenance` | pending |
| `a-harness-judged-by-its-first-role-alone` | `trust_generated::route_tests::route_a_harness_uses_both_pair_roles_and_judges_foreign_program_targets` | pending |
| `a-data-reader-never-judged` | `trust_generated::route_tests::route_a_runtime_data_reader_is_judged_for_scripts_and_program_targets` | pending |
| `a-build-read-naming-its-first-generator-alone` | `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read` | pending |
| `a-middle-generator-never-reached` | `trust_generated::route_tests::route_a_vendored_file_copied`, `trust_generated::route_tests::route_middle_generators_and_inputs_remain_in_provenance` | pending |
| `a-transitive-generator-build-read-dropped` | `trust_generated::route_tests::route_a_table_a_dependency_s_library_holds_is_made_by_a_package_not_depending_on_it`, `trust_generated::route_tests::route_every_generator_names_a_shared_transitive_build_read` | pending |

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

## Effects and remaining scope

Two exact mutations deliberately hold diagnostics. The input-link mutation still refuses a linked input,
changing its reason from symbolic link to no blob. Naming the first generator alone still refuses a chain,
but loses another generator's attribution. Native original/exact-mutant comparisons establish these effects;
they must not be described as changed acceptance in the later operator-witness table.

Operator reach remains unmeasured. The next stage runs every candidate on every census replacement span against
both populations before adding parent semantic tests. An actual viable semantic witness, or an explicit absence
with its reason, is required; a desired verdict pair alone does not establish the same effect.

The final stage strengthens parent tests and runs complete production sweeps of `trust.rs`, `trust_generated.rs`
and `trust_gate.rs`, plus the shared operators' compatibility sweep of `generated_header.rs`. Every exact parent
entry must have a finishing kill; every sweep survivor needs a justified equivalence record. These stages and
the independent implementation review remain open. No completed catalogue subset establishes full adequacy.
