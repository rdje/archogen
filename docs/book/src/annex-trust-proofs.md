# Annex C: Trust proof records

The [trust gate](trust.md) states the rules. This annex retains detailed observations, commands and
scope limits for the [pinned Rust toolchain](ledger.md#rust-toolchain) and mutation measurements.

## Compiler records and normalization

The pinned Cargo producer's verbose path quoting is exercised by
`cargo_quoted_output_paths_preserve_inventory_units_and_configuration`
(`M3.6.9.6.3.2.2.10.2`, `2026-10-12`): actual public inventory builds with ordinary, space, apostrophe
and literal-backslash output paths and a shared dependency, preserving normalized configuration and shared
items. This is finite pinned-producer evidence, without a general shell grammar guarantee.

The separate `a_root_s_configuration_retains_its_dependency_argument` control
(`M3.6.9.6.3.2.2.10.3.2`) requires the fixture's binary and library units to retain their actual normalized
`common` dependency arguments, `libcommon.rlib` and `libcommon.rmeta` respectively. A final-token reversal
must fail this independent expectation; no universal argv-shape claim follows.

Producer-domain investigation (`M3.6.9.6.3.2.2.10.3.3.1`, `2026-10-12`) read Cargo's
[ProcessBuilder formatter at its installed commit](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/crates/cargo-util/src/process_builder.rs#L45-L68):
the executable is unescaped, while each argument follows one separator and an escape operation.
That commit's [lockfile](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/Cargo.lock)
selects shell-escape 0.1.5; its [Unix escape implementation](https://docs.rs/crate/shell-escape/0.1.5/source/src/lib.rs)
quotes empty arguments. No downloaded source hash was captured. Actual public inventory reproduction with a
spaced toolchain home fails collection after successful Cargo builds; an ordinary-home control passes.
Exploratory check-cfg probes emit an empty argument only in a measured failing build. Their adapters are
untracked and provide no universal absence proof. Revalidate formatting assumptions on a pin/producer change;
the executable-boundary repair and permanent controls are recorded by `.10.3.3.2`.

The repair fixes inventory compilation's compiler selector to `RUSTC=rustc` under the same pinned environment
as its identity command. The [pinned Cargo environment-variable contract](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/doc/src/reference/environment-variables.md#environment-variables-cargo-reads)
defines that selector. `compiler_installation_paths_preserve_inventory_units_and_configuration` is a permanent
Unix public control with isolated ordinary/space/quote/backslash/combined toolchain homes and independent unit
and dependency expectations. Its exact selector reversal gives a named failure. The command-format assumptions
still require revalidation on a producer change; finite controls do not prove universal empty-argument absence.

`cargo_verbose_empty_arguments_keep_their_token_boundaries` (`M3.6.9.6.3.2.2.10.3.3.3.1`, `2026-10-12`)
builds actual successful pinned Cargo rustc fixtures with nonempty and empty `-A`/`-W` operands. Empty lint
names warn; compilation succeeds. Independent expected middle/final tokens hold the helper's format promise,
without admitting extra flags to inventory builds. Exact native and actual systematic loss edits give named
finishing failures. This permanent control refutes a generic absence premise; no universal shell guarantee follows.

Multiline-path diagnosis (`M3.6.9.6.3.2.2.10.3.3.3.2.1`, `2026-10-12`) retains actual successful Cargo
output with line feeds, carriage returns and backticks, plus losslessly decoded dependency bytes. Copied public inventory control exposes
a physical-line command-slice panic. A diagnostic framing intervention exposes empty compiler-read file objects
from the separate first-target reader. Adapters/tests are untracked and qualified, not permanent controls;
then-unmodified production panicked. Dependency, environment and command repairs below precede further proofs.

The private `cargo_dependency_information_retains_actual_source_paths` control
(`M3.6.9.6.3.2.2.10.3.3.3.2.2`, `2026-10-12`) reads actual pinned rustc dependency output with relative and
absolute in-tree inputs, independently requiring their complete paths across ordinary, line-break, escaped-space,
backtick, tab, backslash and hash roots. Known-target/prefix framing repairs file collection; exact prior-reader
reversal gives a named failure. Synthetic colon-space target parsing is separate from Cargo's measured host refusal.
A qualified copied command-framing intervention then exposes escaped environment-value normalization; its repair
precedes public command framing below. This diagnostic establishes no universal producer grammar.

Environment investigation (`M3.6.9.6.3.2.2.10.3.3.3.2.5.1`, `2026-10-12`) verifies rustc commit
`59807616e1fa2540724bfbac14d7976d7e4a3860` against the installed pin and reads its
[dependency encoding](https://github.com/rust-lang/rust/blob/59807616e1fa2540724bfbac14d7976d7e4a3860/compiler/rustc_interface/src/passes.rs#L513-L531)
and [record writer](https://github.com/rust-lang/rust/blob/59807616e1fa2540724bfbac14d7976d7e4a3860/compiler/rustc_interface/src/passes.rs#L653-L686).
Actual Cargo/runtime controls distinguish real line breaks from literal backslash-n and unset values.
Encoding known scratch prefixes preserves the existing record representation. Exploratory adapters are
untracked; no downloaded source hash was captured. Isolated Rust/native getenv controls also trace an
equals-bearing name lookup to this host's boundary, without a universal name or host contract.

`cargo_dependency_environment_normalizes_escaped_scratch_paths`
(`M3.6.9.6.3.2.2.10.3.3.3.2.5.2`, `2026-10-12`) permanently holds encoded-prefix normalization against
actual Cargo dependency output. Independent relative manifest, encoded package metadata and unset expectations
cover line breaks, backslashes, literal backslash-n and a source-marker path, with a separate ordinary target.
Distinct canonical-prefix control passes; exact old replacement gives the producer control's named failure.
The existing record representation remains.

`cargo_multiline_output_paths_preserve_complete_inventory`
(`M3.6.9.6.3.2.2.10.3.3.3.2.3`, `2026-10-12`) permanently exercises actual public builds across
line-break, backtick, fake-Running and combined apostrophe/backslash paths. Independent unit identities,
source/data hashes, environment and dependency arguments hold; configurations/shared items remain stable.
Raw pinned Cargo stderr and JSON artifacts also identify unquoted Compiling package paths as the source of
a fake command under the first framing intervention. Complete quoted records and known raw status-prefix
handling repair both boundaries; synthetic malformed/canonical controls and both exact native reversals hold.
Exploratory raw status audit remains qualified; permanent public controls establish the repair, not full adequacy.

Delimiter E (`M3.6.9.6.3.2.2.10.3.3.3.2.4`, `2026-10-12`) concerns complete pinned Unix records
with fixed program `rustc`. Source construction gives one outside space before each nonempty encoding
(empty operands encode as `''`), without leading/trailing/repeated separators. At every push boundary,
decoded bytes or a visited quote already make the predicate true. Initial true converges at the first
reset; changing/deleting a reset cannot affect the next true predicate, and making either guard true
changes nothing. Reasons live in `xtask/trust.equivalents`, one text key covering both guards. This does
not cover arbitrary command text; compiled controls expose each difference outside the domain:

| Actual edit | Input outside the domain | Original tokens | Changed tokens |
| --- | --- | --- | --- |
| initialize the flag true | empty text | `[]` | `[""]` |
| make the separator guard true | `" rustc"` | `["rustc"]` | `["", "rustc"]` |
| reset the flag true | `"rustc "` | `["rustc"]` | `["rustc", ""]` |
| delete the flag reset | `"rustc '' "` | `["rustc", ""]` | `["rustc", "", ""]` |
| make the final guard true | empty text | `[]` | `[""]` |

`CommandDelimiterTests.test_actual_delimiter_candidates_hold_only_the_fixed_cargo_domain` discovers and
compiles the actual original/replacements. Independent token expectations cover encoded empty middle/final
operands, spaces, quoted line breaks/backticks, apostrophe/exclamation escapes, backslashes and Unicode.
Deliberately wrong positive/negative oracles refuse. The control binds the fixed selector and installed Cargo
commit; a producer/domain change requires revalidating the reasons. Actual producer/public controls above
remain distinct from these synthetic encoded-input controls. Reproduce with local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  python3 -I -B xtask/mutation_sweep_tests.py . CommandDelimiterTests.test_actual_delimiter_candidates_hold_only_the_fixed_cargo_domain
```

The same selected actual edits survive unlisted, then finish equivalent with the recorded reasons under the
parent population, strict exit 0. Both input captures and copied source restore; altered audit evidence refuses.
Selected outcomes establish these bounded judgments.

The final complete current helper replay (`M3.6.9.6.3.2.2.10.3.4`, `2026-10-12`) finishes with nineteen
named kills and five scoped equivalents, strict exit 0, with no survivor, unviable, timeout or unjudged result.
Every actual current helper edit is selected; whole-file adequacy is not inferred. Exact source/operator
bindings, the owned quote repair and guard-body context independently connect the original sixteen survivor
obligations to eleven current named kills and five current equivalents. These resolve the obligations after
caller repairs; they do not retrospectively classify an edit in the earlier buggy caller domain.

All original/copy regular inputs restore with independent hashes, sizes and modes; the opaque submodule
identity remains unchanged. Altered audit evidence refuses. Code, tests, catalogue and equivalent records stay
exact. The original full-sweep counts remain historical. Local helper integration passes below; final
whole-production measurements/review remain open. The complete current identities, witnesses and original
bindings are sealed in `docs/task-history/M3/M3.6.9.6.3.2.2.10.3.md`, reached from
`docs/tasks/M3.md`; re-discover them when production changes.

The local helper integration checkpoint (`M3.6.9.6.3.2.2.10.4`, `2026-10-12`) runs `make integration`
with pinned tools on `PATH` and repository-local stores. All twelve steps pass, with no failed or unavailable
step, including all 52 gate self-tests. The first restricted run failed process-detection controls: the
sandbox denied the required process snapshot. Direct OS probes and targeted passing controls isolate that
boundary; the complete rerun with OS reads available retains the runner's bare-Git self-test environment.
Both results stay recorded. All captured regular inputs restore with independent hashes/modes and the opaque
pin is unchanged. The stamp names the activation/diagnosis tree; subsequent closure documentation has its own
checks. This closes the bounded helper group. Dropped-flag obligations are next; complete production adequacy
and runner-baseline compatibility remain open. No remote CI outcome follows from this local checkpoint.


The dropped-flag table measurement (`M3.6.9.6.3.2.2.11.1`, `2026-10-12`) selects every actual table
edit: thirteen operand-consumption switches and the array-terminator deletion. The switches finish with two
named kills and eleven survivors; the terminator deletion fails compilation with an unclosed delimiter and
proves no coverage. Native strict exit 1 means the adequacy criterion remains unmet. Exact unchanged
table/operator/producer bindings retain all original table outcomes; the survivors stay owned for actual Cargo
producer/caller assessment before controls or scoped equivalents. Every captured original/copied regular input
restores with independent hashes/modes. Source, tests, catalogue and E records stay exact. This is a bounded
measurement, separate from the completed helper group and pending whole-production adequacy.


The actual producer assessment (`M3.6.9.6.3.2.2.11.2.1`, `2026-10-12`) accepts ordinary and manifest
allow/warn/deny/forbid cases: separate `--check-cfg` operands `cfg(docsrs,test)`/`cfg(feature, values())`,
and single `--level=lint` tokens. Independent raw/public records agree; altered records refuse and inputs
restore. Its copied diagnostics/adapters are ignored exploratory evidence, not maintained tests or an
absence theorem. The permanent parent `cargo_diagnostic_flags_do_not_enter_unit_configurations`
(`.11.2.2`, same date) requires actual producer operands and independently expected public identities,
source hashes/environment/retained arguments. The actual check-cfg switch changes from survivor to named
finishing kill/strict 0; exact operand-leak failure and restored pass agree. Only that original obligation closes.

Source/caller `.11.2.3` and completion `.11.2.6.3.1` bind package/cache/stdlib/channel and visited-token
premises to pinned sources in `docs/specs/trust/flag-normalization-domain.md`. Metadata repair `.11.2.5.1`
refuses reached scripts/macros despite admissions; `.11.2.5.2` revalidates delimiter scope. Checked cleanup
stops on Cargo-home (`.11.2.6.2`), inventory (`.11.2.7`) and proposal (`.11.2.8`) errors, held by public/native controls.

Lint proof `.11.2.4.1` (`2026-10-17`) covers eight separate lint-level consuming-bit edits. Pinned Cargo
emits joined manifest lint tokens; the scoped caller supplies no separate ones. `DiagnosticFlagTests`
compiles actual edits, requiring independent real argv and normalized outputs for ordinary/allow/warn/
deny/forbid builds. Successful `cargo rustc -- -W unused_variables` and the other seven separate forms
refute broader equivalence: each changed helper leaks the operand. Synthetic consumed-value and wrong
positive/negative controls hold the scope. Reproduce with the local-store environment above and
`python3 -I -B xtask/mutation_sweep_tests.py . DiagnosticFlagTests`. Eight unlisted survivors become eight
scoped E, strict 0; no kill is added. Final table replay and full adequacy remain open.

Cap-lints proof `.11.2.4.2` requires compiled local path packages outside the standard-library build,
which show warnings. Those premises make both pinned Cargo cap branches false. Actual local argv and
independent outputs agree; successful `cargo rustc -- --cap-lints allow` makes the variant retain `allow`.
Consumed-value and wrong-oracle controls hold the boundary. Selected survival becomes scoped E, strict 0;
changed caller/pin premises require revalidation.

Color boundary `.11.2.4.3.1` (`2026-10-18`): Cargo's human/short/json modes supply compiler JSON,
which conflicts with separate color. A test-owned `RUSTC` forwarder removes formatting options;
raw requests and actual compiler argv stay distinct, outside the inventory's fixed selector.
Permanent controls `.11.2.4.3.2` use successful override Cargo builds for always/never/auto, checking the
actual edit, library consumers, consumed values and positive/negative/forwarded-argv falsifiers.
Selected survival becomes scoped E, strict 0; changed caller/pin premises require revalidation.

Local integration `.11.2.6.3.2` passed `2026-10-17`, before `.11.2.7`–`.11.2.8` and these proof controls;
its leaf binds activation/input restoration. Closure prose is checked separately; no remote CI is claimed.


## Mutation measurements and controls

Whether the instrument's tests hold the generated-sources rules — what a program reads, the chain, the blob rule,
each generator's role against each reader — wherever the code judges them, not only at the instances they build, is to
be shown under an accepted design (`M3.6.9`, review closed at round 3): a source-linked census of 32 proposed route/parent mutation pairs and
sweeps of the complete production ranges in `xtask/src/trust.rs`, `xtask/src/trust_generated.rs` and
`xtask/src/trust_gate.rs`. Each exact parent mutation must be killed by finishing parent tests; every sweep survivor
needs an equivalence reason, and timeouts fail both strict modes. The design records actual operator reach and binds
the test copy to its source repository. Implementation begins with strict and attributed mutation outcomes: an
invocation error without a failed test summary and failure-list name is unjudged, never a kill or equivalent.
Those modes are built in `M3.6.9.1`; `M3.6.9.2.1` adds source-span replacements and validated multiline equivalents.
`M3.6.9.2` builds all five additional operators and actual per-operator compiler-refusal counts, with compiled
semantic and CLI restoration controls described in `docs/book/src/verification.md`. `M3.6.9.3.1` adds explicit
configuration-root lists: each outer configuration belongs to its innermost supplied root and must still match
the judged tree's tracked bytes and alias-only content rules. Intermediate target configurations and ancestors
above all supplied roots remain refused. The production gate supplies its repository alone. The tests' seam
(`M3.6.9.3.2`) admits `ARCHOGEN_SWEEP_SOURCE` only after canonicalizing it and the test root, verifying that the
source is its own Git top-level, and requiring the test root to be exactly its `target/mutation-sweep/tree` child.
Canonical aliases are accepted; unrelated roots, enclosing ancestors, deeper copies and redirected physical
locations refuse before inventory. The source hint cannot change production's root list. The sweep now supplies
its canonical own source root for every test command, replacing any inherited hint (`M3.6.9.3.3`). All three parent
populations pass in the actual unmutated copy with an invalid inherited hint; that zero-attempt baseline establishes
copy integration, not mutation adequacy. `M3.6.9.4.1.1.1` adds three source/provenance route controls: a live input
link retains its specific refusal, every live form contributes provenance, and middle generators remain present
beside their inputs. Each exact catalogue mutation compiles and is killed by a named finishing route test.
The input-link mutation changes the diagnostic alone: the original refuses a symbolic link, while the mutant still
refuses it as no blob because the regular-file tree excludes symlinks. It does not show changed link acceptance.
The reader/role controls (`M3.6.9.4.1.2`) hold every role package a generator target's build reaches, including a
foreign package after an allowed one; a harness may use both pair roles but refuses a third role's script or
program target; and a reader handed generated runtime data still judges its generators. Each of the four exact
mutations is killed by a named finishing route test. Build-read controls (`M3.6.9.4.1.3`) hold two generators
reaching a shared dependency through another package: plain reads pass; marked and declared reads refuse with
both generators named. Naming the first alone still refuses but loses attribution; dropping transitive units
admits the marked dependency in the native fixture. Both exact mutations produce named finishing kills.
All nine route additions are built. `M3.6.9.4.2.1` adds the 32 exact parent twins, each retaining the expected
killed criterion. The catalogue contract reads the design's census and frozen finding rows, holding exact
population membership, commands/verdicts and each twin's file/from/to identity. Independent falsifications
and a named compiled guard mutation hold those checks. Run the contract with:

```bash
cargo test -p xtask mutation::adequacy_tests
```

This verifies the catalogue, not semantic adequacy. The full strict route run now has 32 named finishing kills
and exact source restoration (`M3.6.9.4.2.2.1`); the [measurement chapter](trust-adequacy.md) records each actual
killing test and the source hashes. The initial parent baseline passed 91 tests; 2 of the 32 exact mutations had named
kills and 30 survived (`M3.6.9.4.2.2.2`). After the required initial operator measurements, parent controls repaired
those gaps. On `2026-10-11`, the combined final exact run kills all 32 entries with finishing named parent tests, strict exit 0
(`M3.6.9.6.3.1`). These historical measurements remain separate from full production-sweep adequacy.

The first full-range `trust.rs` attempt lists 1,420 candidates, lines 144–2493. Its baseline passes; it stops after 150: 78 named kills, 62 survivors, five compiler refusals, four strict timeouts and one unjudged result, raw exit 2. The incomplete-result auditor refuses; all captured original/copied inputs restore. This is not a full measurement or adequacy pass.

The five non-finishing candidates remove index progress in configuration normalization. A compiler-backed diagnostic
uses each actual edit with a trace that stops before a fourth loop-body evaluation: the baseline visits index 1 once,
while each mutant revisits it; the final mutant also appends the same flag repeatedly. Both deliberately wrong trace
oracles fail. This establishes non-progress, without assigning an OS crash cause to the unjudged command.

```console
bash scripts/trust_adequacy.sh --progress --output target/trust-progress.json
bash scripts/trust_adequacy.sh --self-test
```

The parent tests now execute their unchanged bodies in bounded children selected by the same exact libtest names
(`M3.6.9.6.3.2.1.2`). Each parent requires a successful child exit, one named passed test and a finishing single-test
summary. Zero selected tests, assertions, abrupt exits and a child that exceeds its limit become parent failures.
Child diagnostics are escaped so their failure lists cannot supply names to the outer native result. The test-only
worker in `xtask/src/trust_test_process.rs` reuses the mutation runner's direct system-call group cleanup.

The command's deadline stays active until its child exits and both captured output pipes close. An exited child
whose descendant retains stdout or stderr cannot bypass the deadline: descendants still in its process group
are stopped and the command reports a timeout. The leader stays unreaped while its pipes are open, retaining its
identity for group cleanup. A descendant that creates another group or session leaves this cleanup's scope and
can retain a pipe; independently grouped parent workers must finish under their own limits.

Exact guard reversal bypasses a one-second limit; reapplication times out near one second (`M3.6.9.6.3.2.1.5`, `2026-10-11`). Run the actual runner controls:

```bash
cargo test -p xtask mutation::tests::an_exited_command -- --nocapture
```

All 103 original parents remain apart from guards (`2026-10-11`). Workers have 30s bounds in their own groups. Selected native regressions allow 600s per candidate for successive waves; strict timeout rules remain.

```bash
cargo test -p xtask trust_test_process::tests
ARCHOGEN_PARENT_TEST_TIMINGS=1 cargo test -q -p xtask -- \
  trust:: trust_generated:: trust_gate:: --skip route_tests --nocapture
```

Measured `2026-10-11`: five non-progress mutations gain named finishing kills, strict 0; all 32 entries retain their killing-name sets. Inputs restore, production prefixes stay unchanged and focused gates pass, before the complete measurement below.

Measured `2026-10-11`, source 36154d6: the complete `trust.rs` retry covers 1,420 candidates, all operators, the original 103 parents and no E file. Baseline passes in 17s; results: 802 finishing named kills, 540 survivors, 78 compiler refusals, no timeout/unjudged result. Strict exit 1 records a complete measurement; adequacy remains unmet (`M3.6.9.6.3.2.1.3`).

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
  --timeout 600 --timeouts-fail --filter=trust:: \
  --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests
```

Measured `2026-10-11`: identities/edits/names/counts agree; altered evidence refuses. The first prefix remains, its five non-finishing cases now named kills. Compiler refusals prove no coverage. All 540 initial survivors stay owned in `docs/tasks/M3.md`, requiring repair or justified E before the final strict sweep.

Initial `.1`–`.7` assessments (`2026-10-11`) repair malformed names/placeholder leaves and reader/role/pair test gaps. Committed leaves hold kills, reversals and source boundaries.

Measured `2026-10-11`, `.8.2`: the ends rewrite preserves order/early returns for two borrowed String references, with no user callback/destructor. Alias/wider-array/wrong-oracle controls hold scoped E: native survivor (strict 1), reviewed E (strict 0); production unchanged, no kill added.

Historical runs restore 988 inputs under Python/system hashes, the new E separately hashed; leaves hold commands/identities. Listing `2026-10-11`: 1,416 candidates, production lines 144–2498. IDs need bound source/fresh discovery; final current-source results stay open.

Measured `2026-10-11`, `.9.1`: four environment survivors gain named kills, strict 0. Public controls hold three compiled values and record boundaries; production/old tests stay unchanged and all 990 captured inputs restore.

Measured `2026-10-11`, `.9.2`: marker-bearing values exposed a configuration-key collision. Keys now use
JSON `[command_tokens, environment_records]`, e.g. `[["--crate-name","common"],["CARGO_PKG_DESCRIPTION=alpha env-dep:omega"]]`,
preserving boundaries/order/empty values and public raw-array shapes. The key-only native reversal gains a
named public kill; all 990 inputs restore and domain/focused controls pass. Changed configuration digests
require Linux proposal compatibility in `M3.6.3.2.1.1`, pending an authorized runner/base checker and review.
The reproducer also changes content; it proves the field defect, not a complete-gate bypass.

The [tooling JSON contracts](annex-repository.md#json-used-by-the-tooling) separate encoding from reader diagnostics. Number grammar, raw controls, Unicode escapes and inventory-byte admission are repaired; `.9.4` local integration passed on `2026-10-12`. These defects establish no artifact-hash bypass.

The [focused-stamp guard](annex-repository.md#a-commit-holds-what-was-run) now holds explicit successful-exit claims and large diffs; `.9.5` records its RED controls and repair.

Timeout/unjudged results fail; full adequacy and independent review remain open. Native killing names
route from original `docs/tasks/M3.md` stubs to sealed task-history. Stubs retain commits, the index holds
digests, and the gate proves reconstruction/provenance.
