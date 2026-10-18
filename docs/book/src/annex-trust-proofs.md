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


Initial dropped-flag table measurement `.11.1` (`2026-10-12`) selected all fourteen edits: thirteen
consumption switches and the terminator deletion. Two switches gave named kills, eleven survived;
the unclosed-delimiter compiler refusal proved no coverage. Strict exit 1 left adequacy unmet.
Unchanged table/operator/producer bindings retained every original outcome. All original/copied regular
inputs restored with independent hashes/modes; source, tests, catalogue and E stayed exact. The survivors
were owned for Cargo producer/caller assessment; whole-production adequacy stayed open.


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
scoped E, strict 0; no kill is added. See the final table replay below.

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

Final table replay `.11.3` (`2026-10-18`) binds all fourteen original edits: three named kills,
ten scoped E and one compiler refusal, strict 0. All eleven initial survivor obligations resolve on the
repaired caller; original classifications remain historical. Every original/current copied input restores.
Milestone integration `.11.4` then passes all twelve local stages, with no failures or absences, after
output cleanup and proof controls. Its official activation stamp matches native Git; original/prior-copy
inputs restore. Closure prose is checked separately. The bounded flag group closes; full production
adequacy, independent review and runner compatibility remain open. No remote CI outcome is claimed.


Prefix/helper `.13.1` (`2026-10-18`): all 59 edits yield 46 named kills, 12 survivors and one compiler
refusal, strict 1. All captured originals/current copies restore; exact original bindings hold. Refusal
proves no coverage. The twelve survivors stay owned; full adequacy remains open.

Path controls `.13.2.2` (`2026-10-18`) close two of those measured survivors: `L590.dir.178` and
`L611.dir.213` now finish with their independently expected source/dependency parent failures.
Both exact catalogue entries and the selected systematic replay pass strict zero; the latter has
two named kills and no refusal, timeout, E or unjudged result. All 1029 captured originals/current
copies restore, and five altered-evidence cases refuse. Prior production/tests/catalogue bodies and E
are preserved. Closure documentation and the focused commit gate are separate from that restoration
capture. Ten other prefix/helper survivors remained after that slice; full adequacy stays open.

Codegen controls `.13.2.3` (`2026-10-18`) then kill actual `L595.int.187`, `L604.stmt.203` and
`L605.int.205` with the independent profile/operand parent. Both native harnesses pass strict zero;
the systematic run has three named kills, int 0/2 and stmt 0/1 unviable/attempted. All 1029 captured
originals/current copies restore and five false receipts refuse. Existing bodies and E are unchanged,
catalogue entries append. Closure docs and focused/staged checks are separate. Seven prefix/helper
survivors remain owned; final cohort replay and full production adequacy stay open.

### Codegen matcher domain and temporary-path prerequisite

Assessment `.13.2.4.1` (`2026-10-18`) keeps the remaining matcher results historical. Exact compiled
variants agree on ordinary Cargo-shaped metadata, filename and incremental operands. Valid wider
compiler invocations such as `-C llvm-args=--pass-remarks=metadata=inside` produce libraries consumed
by a separate executable; the original retains that complete option, while the corresponding contains
edit drops it. Extra-filename and incremental markers have matching wider controls. This does not prove
that the inventory caller can emit those arguments. Truncated-prefix helper counterexamples are
synthetic; compiler-refused bare forms establish no coverage or equivalence. Wrong oracles fail.

The source/caller assessment also finds that the shared cleared build environment omits temporary-path
variables. A passive compiled-API probe confirms a default outside the repository volume. It reads only
OS directory metadata and creates no off-volume project files. `PROGRAM.89` owns immediate local-temp
repair and caller/native/integration revalidation before more matcher classifications. The API repair
`.89.1` passes cleared-child path/file-effect and error/recovery controls; `.89.2` closes the caller
checkpoint below. Producer proof fixtures seed local temp variables. No current E record changes. The canonical source domains,
qualifications and evidence are in `docs/specs/trust/flag-normalization-domain.md` and the owning leaf
in `docs/tasks/M3.md`; full adequacy remains open.

### Postrepair caller checkpoint

`PROGRAM.89.2` (`2026-10-18`) revalidates the changed build environment with unchanged consumers and
compiler-selection/preflight rules. Local temp settings grant no new compiled inputs or compiler flags.
Actual-edit producer, wider-domain and wrong-oracle controls hold, and recorded pinned source identities
are checked again. The complete flag replay retains three named kills, ten scoped equivalents and a
compiler refusal, strict zero. The complete matcher replay still has six survivors, strict one; per-key
evidence remains required, and no matcher E is added. Refusal proves no coverage. Altered result receipts
refuse, and original/current-copy input populations restore.

All twelve steps of local integration pass. Its official stamp binds the activation tree recorded in
`docs/tasks/PROGRAM.md`; later closure prose has separate book/staged validation. No remote CI or full
production adequacy is claimed. The temp-path repair and its caller checkpoint close; M3 resumes the
owned metadata matcher evidence.

### Metadata matcher reasons

Metadata evidence `.13.2.4.2.1` (`2026-10-18`) closes `L597.dir.188` and `L597.lit.189` as scoped E.
Cargo constructs `metadata=<hash>`; other visited codegen keys are distinct. Already-discarded
extra-filename/incremental operands stay discarded. Retained scalar values must pass Cargo/compiler
validation. The current caller excludes linker overrides, build scripts, arbitrary rustflags and trailing compiler
arguments. These source premises, with the unchanged operand consumers, justify the reasons.

For example, successful `cargo rustc` can forward `-C llvm-args=--pass-remarks=metadata=inside`.
The original retains that full option; the contains edit drops it. This valid wider input is outside
the inventory caller, so the reason grants no arbitrary-argument equivalence. The truncated key also
differs for synthetic `metadataX=inside`, which has no successful compiler producer in this control.
Both real libraries are consumed and return the expected value 37; wrong argv/output oracles fail.

Cargo forwards profile `strip` strings, and the compiler rejects unsupported values. The refused
`strip=metadata=inside` case establishes a successful-build premise, not coverage from a failed build.
Exact compiled controls and native replay bind both E reasons, strict zero; original/current-copy
inputs restore and altered receipts refuse. Prior test bodies and reasons are preserved. Per-key
extra-filename/incremental evidence and the basename comparison remain required; full adequacy is open.

### Extra-filename matcher reasons

Extra-filename evidence `.13.2.4.2.2` (`2026-10-18`) closes `L598.dir.192` and `L598.lit.193` as scoped E.
When present, Cargo supplies `extra-filename=-<suffix>`; both edits still discard it. Other discarded
fields stay discarded, successful retained scalar values have disjoint domains, and the caller excludes
arbitrary linker/script/flag routes. Existing operand consumers remain unchanged.

For example, successful `cargo rustc` forwards `-C llvm-args=--pass-remarks=extra-filename=inside`.
The original retains that full option while contains drops it, so the reason is conditional on the
inventory caller. Real libraries are consumed with expected value 37. The truncated edit also differs
on synthetic `extra-filenameX=inside`, which is not a successful compiler input. Compiler-refused strip
and wrong argv/output oracles keep these domains separate. Exact native replay gives two scoped E,
strict zero; original/current-copy inputs restore, altered receipts refuse and prior test bodies/reasons
are preserved. Incremental evidence and the basename comparison remain open; full adequacy is pending.

### Incremental matcher reasons

Incremental evidence `.13.2.4.2.3` (`2026-10-18`) closes `L599.dir.196` and `L599.lit.197` as scoped E.
The permanent producer enables the real release cache:

```toml
[profile.release]
incremental = true
```

Its argv must contain the independently expected cache path, and the compiler must populate that
local directory. Both compiled libraries are consumed with expected value 37. Exact original/edited
normalization still discards the `incremental=<path>` operand. Other discarded fields, successful
scalar domains, excluded flag routes and unchanged consumers justify the narrow caller reason.

Successful `cargo rustc` also forwards `-C llvm-args=--pass-remarks=incremental=inside`; original and
truncated-prefix retain the full option, while contains drops it. This valid wider input is outside
the inventory caller. Synthetic `incrementalX=inside`, refused strip and wrong argv/output oracles
keep the premises explicit. Native selected replay gives two scoped E, strict zero; input populations
restore and altered receipts refuse. All codegen matcher obligations close, with prior bodies/reasons
preserved. The basename comparison and final cohort/full adequacy work remain required.

### Basename comparison reason

Basename evidence `.13.2.5` (`2026-10-18`) closes `L614.cmp.215` as source-context E. Normalization
finds the final dash and final dot in the same filename. Those distinct characters cannot occupy the
same byte position, so changing the guard from `<` to `<=` cannot change the selected branch. This
holds for the function's string inputs without requiring the restricted inventory caller.

The actual original and edited functions compile against independent literal examples:

| Extern operand | Normalized entry |
| --- | --- |
| `common=libcommon-abcd.rlib` | `--extern common=libcommon.rlib` |
| `common=cache=one=two/libcommon-abcd.rmeta` | `--extern common=libcommon.rmeta` |
| `common=a.b-c` | `--extern common=a.b-c` |
| `common=a-foo-123.ext.more` | `--extern common=a-foo.more` |
| `common=é-Ω.λ` | `--extern common=é.λ` |

Empty strings, absent/adjacent punctuation and path components have literal expectations too. A bounded
grid of 5461 strings supplements the source argument; it does not prove an unrestricted universe by
enumeration. Searching for the same character instead refutes the distinct-position predicate premise
only, without claiming a final-string counterexample. Equal numeric indices and a wrong negative output
oracle reject broader or inverted claims. Changed search patterns, receivers or source context require
revalidation.

Only after those controls pass does the exact reason enter the per-file record. Native selected replay
reports one E, strict zero; captured original/current copied inputs restore and altered receipts refuse.
All initial prefix/helper survivor obligations now have separate controls or proofs. Final complete
cohort replay and whole-production adequacy remain required.

### Complete prefix/helper replay

Complete replay `.13.3.1` (`2026-10-18`) binds all 59 actual prefix/helper edits to their original
source/operator/replacement identities and preserves their earlier outcomes. Its current result is
51 named kills, seven scoped E and one compiler refusal, strict zero. All twelve survivors from
`.13.1` resolve: five gain independent parent controls, six have pinned-caller codegen proofs, and
one has the source-context basename proof. The refusal proves no coverage. No survivor, timeout
or unjudged result remains in this helper region.

Captured original and current copied inputs restore; production, controls, reasons and native tools
remain unchanged. Altered result receipts refuse. The copier's borrowed index metadata does not prove
copied index or gitlink contents. Reason validators bind the selected text; the source/compiled proofs
establish each reason's substance. This result covers the complete helper region, with the rest of
the production file still open. A separate caller/premise and full local integration checkpoint follows;
whole-production adequacy, independent review and runner compatibility are still required.

### Caller binding and first checkpoint result

Checkpoint `.13.3.2.1` (`2026-10-18`) binds the completed helper replay and its reasons to unchanged
production callers, preflight/build inputs and pinned source identities. Existing compiled controls
remain source-bound. The earlier flag-table receipt is checked against unchanged source and reasons;
it is explicitly retained evidence, rather than a fresh flag measurement.

Full local integration fails its self-test step; every other declared step passes. The failing control
is the handoff census's first-sample-only process arm. No integration pass stamp is issued. Original
inputs restore, and the separately retained prior mutation copy also restores in its own epoch.
`PROGRAM.92` subsequently verifies the direct repair with acknowledged sampling and large-output
controls. Final checkpoint `.13.3.2.2` follows that committed repair below. The complete helper mutation
result remains valid within its recorded scope; this failed run closes neither cohort nor whole-production adequacy.

### Final prefix/helper checkpoint

Checkpoint `.13.3.2.2` (`2026-10-18`) passes full local integration after the committed handoff-control
repair. Format, lint, contract tests, doctrine checks, gate self-tests, the pinned book, target builds,
toolchain premises, browser binding, emulator and target spike all pass. The actual result is
`12 passed / 0 failed / 0 unavailable / 0 not built / 0 quarantined`. The official stamp and native Git
agree on activation `b551ad85dc21174e08c4e167d1a515708d886a3e`; the earlier failed run remains recorded.

Caller/source/proof bindings remain exact. All captured originals (1046) restore against this staged
epoch. The retained mutation copy (1043 regular inputs) restores against its prior complete-replay
epoch; this check borrows index metadata and does not prove copied Git index or gitlink contents.
Altered activation/result receipts reject, and production, controls, reasons and repaired census source
stay unchanged. Closure prose is checked separately with the pinned book and staged doctrine gates.

The prefix/helper cohort is now closed. The next source group is `run_cargo`: first bind its exact
edits and establish volume-local execution before measuring environment-omission variants. Its old
survivors are obligations, with no current classification inferred from historical results. Final
whole-file adequacy, remaining source groups and independent review remain open.

### run_cargo locality preflight

Preflight `.14.1` (`2026-10-18`) binds the complete current `run_cargo` region. Its eleven initial
edits map uniquely to twelve current edits; the extra statement selects the fixed compiler. Apart
from that three-line insertion, the function is byte-identical. Generated iterator identifiers carry
source line numbers, so only that verified identifier prefix is normalized for matching. Raw edits
remain recorded. The four initial survivors stay historical obligations; no current outcome is inferred.

This helper applies the cleared child environment. Its end-elements iterator edit removes middle
settings, including Cargo home, build target and temporary aliases. Before any altered helper can
launch a build, the measurement needs a write boundary that still holds when those settings disappear.
An exact compiled original and six actual edits are observed using a Rust executable named `cargo`
that only prints selected metadata. It creates no artifacts or caches and runs no Cargo/Rustup code.
The original forwards every local store and clears the parent sentinel. The end-elements edit drops
middle stores; last-only and no-environment edits cannot spawn the controlled child in this setup.
These observations are passive controls, not native kill, equivalence or compiler-refusal results.

On the installed macOS host, the preflight uses the OS `sandbox-exec` command and this exact profile.
The executable and toolchain are necessary read-only OS dependencies. `/dev/null` is the sole explicit
non-project output exception: descriptor metadata verifies it is a character device, not project storage.
The writable path is passed at runtime, so the profile contains no checkout-specific path.

```bash
mkdir -p target/run-cargo-preflight
cat > target/run-cargo-preflight/mutation-volume.sb <<'PROFILE'
(version 1)
(allow default)
(deny file-write*)
(allow file-write* (subpath (param "WRITABLE_ROOT")))
(allow file-write-data (literal "/dev/null"))
PROFILE
```

Profile SHA-256: `2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.
Prepare local shell stores as described in [verification](verification.md), before running
commands or redirections. Start with a fresh project-owned control fixture:

```bash
mkdir -p target/run-cargo-preflight/allowed target/run-cargo-preflight/denied
printf 'held\n' > target/run-cargo-preflight/denied/existing
ln -s ../denied target/run-cargo-preflight/allowed/escape
```

Save the following control as `target/run-cargo-preflight/guard-probe.py`. It permits one write,
requires permission errors for the listed forbidden operations and verifies held content. Every
probe target is inside the repository volume, including paths deliberately outside the narrower
writable subtree. No off-volume write is attempted.

```python
from pathlib import Path
import errno
import json
import os
import stat

root = Path.cwd()
fixture = root / "target/run-cargo-preflight"
allowed = fixture / "allowed"
denied = fixture / "denied"
assert allowed.is_dir() and denied.is_dir()

(allowed / "positive").write_text("allowed\n")
assert (allowed / "positive").read_text() == "allowed\n"

operations = [
    ("create", lambda: (denied / "new").write_text("forbidden")),
    ("truncate", lambda: (denied / "existing").write_text("forbidden")),
    ("mkdir", lambda: (denied / "new-dir").mkdir()),
    ("unlink", lambda: (denied / "existing").unlink()),
    ("symlink-target", lambda: (allowed / "escape/existing").write_text("forbidden")),
]
refused = []
for label, action in operations:
    try:
        action()
    except PermissionError as error:
        assert error.errno in (errno.EPERM, errno.EACCES)
        refused.append(label)
    else:
        raise AssertionError("write boundary failed: " + label)

assert (denied / "existing").read_text() == "held\n"
assert not (denied / "new").exists()
assert not (denied / "new-dir").exists()
with open(os.devnull, "wb") as stream:
    assert stat.S_ISCHR(os.fstat(stream.fileno()).st_mode)
    stream.write(b"null-device-control")

report = {
    "qualification": (
        "Installed-host parameterized policy controls only; forbidden targets remain "
        "within project volume. No off-volume write attempt or universal OS security proof."
    ),
    "positive_write": True,
    "refused": refused,
    "denied_fixture_exact": True,
    "null_is_character_device": True,
}
(allowed / "guard-observations.json").write_text(json.dumps(report, indent=2) + "\n")
print(
    "guard: allowed write succeeds; five forbidden operations refuse; "
    "held bytes exact; null is a character device"
)
```

Run the control with only its `allowed` directory writable:

```bash
sandbox-exec -D WRITABLE_ROOT="$PWD/target/run-cargo-preflight/allowed" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  python3 -I -B target/run-cargo-preflight/guard-probe.py
```

The measured control passes. Create, truncate, mkdir, unlink and a write through the symlink all
refuse, with held bytes unchanged. The identical operations succeed without the guard on those same
project-local paths, after which the held fixture is restored; ordinary permissions do not explain
the denials. These are installed-host observations, not a universal OS security proof.

For actual parent tests, pass the current repository as the writable root, keeping the same policy:

```bash
sandbox-exec -D WRITABLE_ROOT="$PWD" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  cargo test -q -p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests
```

The actual unmutated baseline reports `141 passed; 0 failed; 0 ignored`. This proves compatibility of
that baseline with the guard. The next measurement must retain its exact guarded native command,
profile hash, all candidate outcomes and input restoration. A guard denial is evidence to investigate,
never an equivalence waiver or an invented finishing kill. Changed or unsupported hosts must establish
an equivalent evidenced boundary before running environment-omission build variants. Source, existing
controls, equivalence reasons and production behavior remain unchanged by this preflight.

### Complete guarded run_cargo measurement

Measurement `.14.2` (`2026-10-18`) reconstructs the committed guard and control sources exactly,
rechecks the real write-boundary controls, then runs the unchanged native CLI and its parent selection
inside that guard. The unmutated copy passes. Every edit in the complete current helper region is judged:

```text
9 named finishing kills
3 survivors
0 equivalent, compiler-refused, timed-out or unjudged outcomes
strict native exit: 1
```

| Current edit | Change | Actual result |
| --- | --- | --- |
| `L678.stmt.231` | Drop argument, working-directory and cleared-environment setup | killed |
| `L679.iter.232` | Forward only the first environment entry | killed |
| `L679.iter.233` | Forward only the last environment entry | killed |
| `L679.iter.234` | Forward only the two end entries | killed |
| `L680.stmt.235` | Drop each environment assignment | killed |
| `L684.stmt.236` | Drop the fixed compiler selector | killed |
| `L688.cond.237` | Always take the unsuccessful-command branch | killed |
| `L688.cond.238` | Always take the successful-command branch | killed |
| `L688.not.239` | Reverse the success check | killed |
| `L691.dir.240` | Select a line containing `error` | survived |
| `L691.lit.241` | Select a line starting with `erro` | survived |
| `L691.lit.242` | Select a line starting with `rror` | survived |

The native log retains every finishing test name and the per-operator attempted/refused accounting.
The global listing is a catalogue, not attempted coverage outside this helper. The three survivors
are explicit obligations under `.14.3`; strict adequacy is still unmet. No equivalence is added.

#### Why the environment end-elements edit now fails

The original environment at the initial measurement ended with `CARGO_TARGET_DIR`. Its two-end-entry
edit retained that setting. The current environment ends with `TEMP`, so the same constructor now
omits the target setting as well as other middle entries. Source and raw edit identities remain bound;
the historical result describes its historical context.

The existing shared-package test passes with the original and fails with the current actual edit.
Inspection of the resulting public inventory shows no refusals: the first root has units for `a` and
`common`, while the second has only `b`. The shared dependency was silently absent from that second
root’s compiler records. Successful builds alone do not establish a complete inventory.

An independent public Cargo experiment isolates the target-directory choice. It builds identical
tracked fixture sources in a fresh workspace for each mode, keeping every home/temp setting local
and preserving the OS guard. Both root builds succeed in both modes:

| Target arrangement | Root | Cargo artifact `fresh` for `common` | New compiler command for `common` |
| --- | --- | --- | --- |
| Distinct target per root | `a` | false | present |
| Distinct target per root | `b` | false | present |
| Shared default target | `a` | false | present |
| Shared default target | `b` | true | absent |

The inventory reads verbose compiler invocations. A warm shared target lets Cargo reuse the dependency
without another invocation, so its second-root unit is missing. The actual mutated inventory, native
Cargo JSON/verbose records and target-only intervention establish this cause. The production helper
retains target isolation, and its existing parent test detects this omission. The guard-denial concern
was investigated before accepting this result as coverage.

All captured originals and the current mutation copy restore (1047 regular inputs each), with native
modes, sizes and Python/system hashes checked. The copy check borrows original index fields and does
not prove copied Git index or gitlink contents. Seven altered receipts reject; existing source, tests,
operator catalogue, equivalence reasons and native tooling stay exact. Later closure prose receives
separate pinned-book and staged checks. The next slice assesses the three diagnostic matcher survivors
against exact compiled helpers and supported pinned Cargo output, including independent expected lines.

<a id="diagnostic-selector-assessment-and-current-defect"></a>

### Diagnostic selector assessment

Assessment `.14.3.1` (`2026-10-18`) tests the exact original helper and its three current selector
edits with real pinned Cargo. The caller admits alias configuration only; general configuration-warning
examples fall outside that contract. The witness instead uses multiline scratch prefixes already
supported by public inventory controls and an ordinary Rust type error. All home/temp stores stay
local, and the OS write guard remains active.

Cargo first succeeds on the source, then fails after a normal type mismatch. Structured output names
E0308, and Cargo’s independent stderr diagnostic is:

```text
error: could not compile `a` (bin "a") due to 1 previous error
```

All compiled variants succeed with valid source. On the failure case whose physical path continuation
begins `erroneous-prefix`, the original returns the literal diagnostic. The actual `contains` edit
selects a status/path line, the `erro` edit selects the continuation and the `rror` edit returns an empty
fragment. An oracle accepting any error would pass every variant, so it proves no diagnostic precision.
The literal independent expectation distinguishes them. These observations are supported witnesses;
permanent finishing parent controls and repaired-source native outcomes are still required.

The same assessment exposed the original defect, repaired in the following section. A supported path continuation beginning
`error-path` is selected before Cargo’s genuine message:

```text
expected: error: could not compile `a` (bin "a") due to 1 previous error
reported: error-path/source/crates/common)
```

A standalone-helper control uses `error: decoy` in the workspace prefix while keeping artifacts in a
separate ordinary target. That Cargo build can succeed, but it is not the normal inventory geometry:
Cargo cannot join a Unix library-search path containing a colon in its artifact directory. The earlier
assessment’s inventory-layout implication was too broad. The repair keeps that observation qualified
and adds public operational-failure controls, rather than counting an unsupported positive build.

The cause is scanning physical stderr lines without distinguishing multiline path continuations.
Public dual-config warnings reproduce the colon-looking continuation before Cargo’s genuine setup
failure, so merely tightening the prefix to a colon still loses the actual reason. Repair `.14.3.2.1`
below adds permanent regressions. `.14.3.2.2` rebinds obligations against the repaired source. This assessment
changes no production code, existing controls or equivalence records. Its exact compiled source,
producer JSON/stderr, positive/negative controls and Python/system hashes are retained; the complete
native `9 killed / 3 survived / strict exit 1` remains the preceding measurement, not a new result.

### Cargo failure framing repair

Repair `.14.3.2.1` (`2026-10-18`) preserves Cargo’s genuine failure line by excluding candidates whose
byte positions fall inside known multiline workspace or artifact directory spellings. Ancestors cover
warnings about containing configuration directories; canonical spellings cover output aliases. Raw
status/warning paths and Cargo’s Unix quoted arguments both participate. The existing command decoder
and successful-build processing remain byte-identical.

The permanent public-inventory controls first build both roots successfully, then commit ordinary
type errors in both root kinds. They require the literal Cargo diagnostics, no inventory after the
failure and successful recovery after the sources are repaired. Paths include newline and carriage
return, quoting/backslashes, non-ASCII text and distinct aliased/canonical names. Target-only controls
exercise paths inside verbose command arguments. Admitted legacy/current alias configurations emit
real containing-directory warnings, including repeated path spellings in a message.

```text
accepted old-code baseline: 3 finishing parent failures
final repaired parents:    4 passed, 0 failed, 0 ignored
old-defect catalogue entry: killed by all 4 named finishing parents, source restored
```

The colon boundary is explicit. A colon in the artifact path causes a Unix Cargo library-path setup
failure; it is not successful compiler evidence. Public operational controls preserve that actual Cargo
reason even when a preceding dual-config warning has a diagnostic-looking continuation. The earlier
rejected colon-positive baseline remains retained and excluded from the repair’s failing baseline.
Probe setup and compiler stores are repository-local; targeted Cargo/control runs use the recorded
OS guard. The ordinary focused tier uses the repository-local store environment.

For example, `with\nerror-path` denotes a directory name with a real newline. Its continued path is
not used as the build diagnostic. A type mismatch still reports the pinned Cargo message:

```text
error: could not compile `a` (bin "a") due to 1 previous error
```

The maintained reversal command restores the original physical-line selector and proves the regression
parents detect it:

```console
$ cargo xtask mutate --only trust-cargo-failure-path-framing-removed
```

Every older production/test byte reconstructs after removing only the new helper, call replacement
and new regressions. The catalogue appends its defect entry; earlier operator controls and E reasons
stay exact. Final focused, pinned-book and staged gates bind publication. This is a verified defect
repair, not mutation adequacy: the repaired helper region has 51 current edits within a global listing
of 1526. The old three raw selector rows are retired by the production change. Their semantic
obligations and the new helper still require fresh binding, complete measurement and resolution under
`.14.3.2.2`, before the final cohort checkpoint.

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


### Complete repaired Cargo diagnostic measurement

Measurement `.14.3.2.2.1` (`2026-10-18`) binds the exact repaired source and all current edits in
`cargo_error_line` and `run_cargo`. The maintained CLI lists 1526 production candidates globally;
51 belong to this region: 42 in the new diagnostic helper and nine retained invocation edits. The
global listing is a scope census, not 1526 attempted mutations.

The eleven initial obligations bind through the preceding twelve-entry cohort. Eight initial edits
remain textually present, along with the later fixed-compiler deletion; the three original raw
selector edits are retired by the production repair. Retirement is not equivalence. Their current
semantic counterparts are measured: `contains("error")` and `starts_with("rror")` are killed;
`starts_with("erro")` survives and stays open on the repaired path-exclusion domain. Only generated
iterator identifier line numbers are normalized for historical matching; raw replacement text is kept.

The installed-host write guard and its probe are reconstructed from the preceding recipe. One allowed
write succeeds, five forbidden operations refuse, and those same operations succeed without the
policy on the same project-local fixture. Held bytes are restored. The exact profile hash remains
`2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.

Every selected edit is attempted through the maintained native command, with the same guard covering
the command and its descendants:

```console
$ sandbox-exec -D WRITABLE_ROOT="$PWD" -f target/cargo-error-measurement/mutation-volume.sb \
    bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask --timeout 600 \
    --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
    --filter=--skip --filter=route_tests --timeouts-fail \
    --equivalents xtask/trust.equivalents --only <each of the 51 IDs below>
```

The placeholder means 51 separate `--only ID` arguments in one invocation, not a literal argument.
The manifest holds every raw source span/replacement and the actual invocation. The unmutated copy
passes. The complete result is **38 named kills, nine survivors and four compiler refusals**, strict
native exit **1**. There is no equivalent, timeout or unjudged outcome. The criterion remains unmet.
All nine retained invocation edits are killed, with a finishing parent also present in the preceding
measurement. The earlier successful-build target-isolation diagnosis remains applicable: the framing
repair changes only failure-text selection, and the invocation/environment producers remain exact.

The following table covers every current candidate. A killed row shows one actual finishing parent;
the native receipt retains the full set of names. Compiler refusals are not semantic coverage.

| Current candidate | Native outcome | Finishing evidence or open obligation |
| --- | --- | --- |
| `L676.eq.231` | survived | Open obligation; assessment scheduled below |
| `L685.iter.232` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L685.iter.233` | survived | Open obligation; assessment scheduled below |
| `L685.iter.234` | survived | Open obligation; assessment scheduled below |
| `L687.iter.235` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.236` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.237` | survived | Open obligation; assessment scheduled below |
| `L688.iter.238` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.239` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.240` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.241` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.242` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.243` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.cond.244` | survived | Open obligation; assessment scheduled below |
| `L690.cond.245` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L691.stmt.246` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L692.stmt.247` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L706.iter.248` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.249` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.250` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L708.stmt.251` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.dir.252` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.lit.253` | survived | Open obligation; assessment scheduled below |
| `L714.lit.254` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.logic.255` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.not.256` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.cmp.257` | survived | Open obligation; assessment scheduled below |
| `L717.cmp.258` | survived | Open obligation; assessment scheduled below |
| `L717.logic.259` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L732.stmt.260` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.261` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.262` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.263` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L734.stmt.264` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L738.stmt.265` | killed | `trust::tests::cargo_diagnostic_flags_do_not_enter_unit_configurations` |
| `L742.cond.266` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L742.cond.267` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L742.not.268` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L676.chain.1145` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L676.chain.1146` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L687.chain.1147` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.contains.1148` | unviable | E0599: no `first` method on `Cow<str>` |
| `L690.contains.1149` | unviable | E0599: no `first` method on `Cow<str>` |
| `L690.contains.1150` | unviable | E0599: `Cow<str>` is not an iterator |
| `L690.contains.1151` | unviable | E0599: `Cow<str>` is not an iterator |
| `L698.chain.1152` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L698.chain.1153` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1154` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1155` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.clogic.1497` | survived | Open obligation; assessment scheduled below |
| `L717.clogic.1498` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |

The four refusals are generated collection operations on a string `Cow`, with actual E0599 diagnostics;
they are not changes to the executable program. No equivalent record was added. Nine survivor
obligations remain owned under `.14.3.2.2.2`: first the two end-elements edits over collections of at
most two entries, then the remaining path-selection, prefix, selector and span-boundary cases. Even
the finite-collection cases require exact source proofs before any E classification.

All 1047 original and current copied regular inputs restore, including modes and bytes, with
independent Python/system hashes. The submodule contributes one opaque Git-index identity; its
implementation is not opened. Seven altered receipts refuse: false pass exit, missing/duplicate
candidate, wrong replacement, wrong operator count, unlisted E and missing finishing kill names.
Production, tests, catalogue, native producer and existing equivalence reasons remain byte-identical
to the committed repair. This is a complete bounded assessment, not whole-file adequacy or remote CI.


### Finite Cargo path iterator proofs

Proof slice `.14.3.2.2.2.1` (`2026-10-18`) resolves only `L685.iter.234` and `L687.iter.237`.
Their end-elements rewrites preserve every element because the current constructors supply one or
two elements. This is a source argument; the finite compiled examples support it rather than define
the universe of inputs.

The outer loop consumes an owned `Vec<PathBuf>`. Its constructor starts with cwd and adds at most
one target, from the last matching environment entry. The exact complete constructor is held by a
permanent source guard. Relative targets are joined to cwd before recording; duplicate values do not
increase the constructor's number of positions.

The inner loop is the concrete chain `std::iter::once(path).chain(canonical)`, where `canonical` was
already computed as `fs::canonicalize(&path).ok()`, an `Option<PathBuf>`. There is always the raw
spelling and optionally its canonical spelling. A missing path supplies no canonical spelling.

For a sequence of length one, its sole element is both first and last. For length two, its positions
are first and last respectively. The rewrite therefore skips nothing and preserves order. Its counter
advances at most twice and cannot overflow. Peeking merely moves concrete owned path values; it calls
no user iterator or filesystem operation. Canonicalization remains before the spelling loop, and the
unchanged ancestor/prefix work receives exactly the same values in the same order. The argument also
holds when raw and canonical spellings coincide or the target equals cwd.

| Actual current edit | Proven constructor | Native replay |
| --- | --- | --- |
| `L685.iter.234` | cwd plus optional target; length one or two | equivalent |
| `L687.iter.237` | raw spelling plus optional canonical spelling; length one or two | equivalent |

The permanent control compiles the actual original helper and each actual native replacement. It
checks literal diagnostics and empty output, with line endings of either newline alone or carriage
return followed by newline. Separate traces hold path order.
Its filesystem cases include aliases, absent paths, relative targets and repeated values. These helper
records are constructed controls; the separate native parent replay exercises real Cargo builds.

Wider constructors mark the boundary concretely. The same actual edit compiles for a vector holding
`[cwd, target, third]` and produces `[cwd, third]`. A chain with a raw path followed by a vector of
two more spellings loses its middle too. First/last-only traces differ on distinct bounded values.
Wrong positive, negative and wider-domain oracles all fail. Independent source-premise controls also
reject insertion of another path or expansion of the canonical iterator before classification.

```console
$ python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
    IteratorTests.test_actual_cargo_path_end_loops_preserve_the_bounded_collections -v
$ python3 -I -B xtask/mutation_sweep_tests.py "$PWD" -q
```

The exact parent passes and the complete operator suite passes 33 controls. After the source/compiled
proof, the per-file record gains only these precise reasons. The guarded actual native replay attempts
both candidates and reports **two equivalent, strict exit 0**; `iter=0/2` compiler refusals. This
selected replay does not validate unrelated equivalents.

All 1051 original and copied regular inputs restore, with modes and Python/system hashes. Seven
altered receipts refuse. Earlier source, operator controls, catalogue and reason prefixes remain exact.
The production helper is unchanged. The preceding full measurement's nine survivors remain its
historical result; these proofs resolve two obligations, leaving seven for source/domain assessment.
Revalidate if a constructor, concrete type, iterator or loop body changes. No arbitrary vector or chain,
whole-file adequacy or remote CI claim follows.


### Relative gate caller assessment

Assessment `.14.3.2.2.2.2.1` (`2026-10-18`) checks a caller premise before classifying the
seven remaining diagnostic-helper survivors. The inventory and baseline CLIs derive paths from
`repo_root()`; the gate's `--repo` argument is retained as supplied. An absolute-path assumption
therefore cannot be taken across every gate invocation.

The original public CLI is compared on the same current main commit, with separate output
directories and the same installed-host write guard:

```console
$ cargo xtask trust-gate --repo "$PWD" --commit HEAD \
    --out target/cargo-caller-path-assessment/absolute
$ cargo xtask trust-gate --repo . --commit HEAD \
    --out target/cargo-caller-path-assessment/relative
```

The absolute invocation returns 0 and writes an inventory for the generator, scheduling checker,
reference model, implementation and differential harness. Its change comparison is explicitly
skipped because the baseline names the other host. That result establishes inventory production here;
it is not a successful comparison across hosts.

The relative invocation returns 1, writes no inventory and reports this false refusal:

```text
trust-undeclared-input: .cargo/config.toml: catalog-source [(config)] a cargo configuration above the repository, on a build's path (§3)
```

This is an original public defect, not a mutation result. `run_gate` preserves relative repo/output
spellings, while `configurations_on_paths` judges ownership with lexical prefix comparisons. At the
repository ancestor, the own configuration spelling does not match the supplied `.` root. The exact
maintained ownership closure, compiled independently with literal expectations, refuses that relative
pair and accepts the absolute pair. OS device/inode identity and equal bytes prove both name the same
configuration. The absolute inventory binds its configuration hash and current commit.

Repair `.14.3.2.2.2.2.2` owns runtime path resolution, public relative/absolute regression controls,
missing-path/error and output-location behavior, plus verification before survivor classification
resumes. No equivalent reason is added on the unproved caller premise.

The held activation input check restores all 1051 regular inputs with modes and Python/system hashes;
the gitlink remains an opaque index identity. The first post-run check correctly refused an intentional
task finding edit written too early. The exact held task bytes were restored for the full maintained
check, then the saved finding was reapplied byte-exactly. Both outcomes and their separate document
epochs are retained. Source, operator controls, catalogue and previous reasons remain unchanged.


### Repository path caller repair

Repair `.14.3.2.2.2.2.2.1` (`2026-10-18`) resolves the selected repository with the runtime
filesystem before deriving its output path or configuration-root list. Relative, absolute and aliased
repository spellings therefore share the same physical root. Relative output arguments remain relative
to that root; an absolute output argument retains its selected destination. Missing repositories and
non-directory paths remain unable-to-judge errors, exit 2, before output creation.

The public wrapper continues to supply no outer configuration roots. A private command worker shares
parsing and invocation with the regression controls, which use only the existing validated fixture
roots. A permanent public-wrapper control requires the containing test configuration to remain refused;
the explicit authorized fixture control succeeds. This test seam adds no CLI option or environment
route for authorizing a configuration above the selected repository.

The isolated command parents use small committed repositories. Their absolute control succeeds before
the fix; the relative call fails with the false above-repository diagnostic. After resolution, the
relative and absolute spellings target the **same physical output**, preserve every inventory byte and
retain the literal root/configuration identities and both expected fixture programs.

```text
before: 2 passed, 1 failed — relative repository/output command
after:  3 passed, 0 failed, 0 ignored
exact catalogue reversal: killed by the named relative command parent; source restored
```

The controls also require absolute-output behavior, missing/non-directory error status and no created
output on those errors. They run in the existing isolated child protocol, so their current-directory
change cannot affect other tests. The defect reversal is maintained as:

```console
$ cargo xtask mutate --only trust-gate-repository-resolution-removed
```

The real public CLI is then tested on the same frozen source commit as the original assessment.
Absolute, relative and repository-alias arguments all return 0 and write byte-identical inventories
at the same output location. Their inventory SHA-256 is
`73c4e740fc47d32b87b4d1a7a74164fec5daa62aa1933d781a332a36346489c6`.
The baseline's other-host comparison remains explicitly skipped; this is inventory/path evidence,
not successful comparison across hosts. Actual missing and file repository arguments return 2 and
create no output.

An initially overbroad oracle compared different physical output directories. Only the library
artifact hash differed: the retained rlibs have the same archive members and different embedded source
directory paths, verified with `ar`, `strings` and hashes. That rejected oracle is not evidence against
the repair or for reproducibility across arbitrary directories. The corrected unit and public controls
compare equivalent spellings of the same destination. An initial unavailable sandbox setup attempt is
also retained and excluded from the test baseline; the actual targeted runs use the installed guard.

All earlier production/test bytes reconstruct after removing exactly the wrapper/worker adaptation,
resolution block and new controls. The catalogue adds one entry; the diagnostic helper, operator
controls and prior E reasons stay exact. All 1051 public-experiment inputs restore with modes and
Python/system hashes. The final focused tier must pass on the held complete staged tree with its
matching official stamp before this repair is committed. That focused pass is recorded with the repair;
full integration and repaired-caller closure are recorded in the checkpoint below. The seven
diagnostic-helper judgments remain open.


### Repaired caller integration checkpoint

Checkpoint `.14.3.2.2.2.2.2.2.2` (`2026-10-18`) closes the caller repair after its focused
implementation and timing-comment correction. The complete local integration tier passes:

| Step | Result |
| --- | --- |
| format | passed |
| strict lint | passed |
| complete contract suite | passed |
| doctrines | passed |
| script self-tests | passed |
| pinned [mdBook](ledger.md#mdbook) | passed |
| no_std build | passed |
| pinned toolchain premises | passed |
| wasm build | passed |
| wasm binding | passed |
| emulator configuration | passed |
| target spike | passed |

```text
12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
activation tree: 180b081792ea1770b6ca8e79efcf11487e97cb21
```

The actual runner log, native Git activation identity and official integration stamp agree. All 1055
held regular inputs restore, including modes and Python/system hashes; the submodule supplies only its
opaque Git-index identity. Five altered bindings refuse: nonzero exit, a missing step, a false summary,
a wrong stamp and missing restoration. Completion prose is a later document epoch, with separate
pinned-book and staged doctrine checks. The integration stamp names the held activation above, rather
than claiming the later completion text itself was the tier's input.

The source/caller binding retains 28 pinned source identities and the 33 operator controls. The
environment constructor, compiled-input allowlist and recorded reasons remain exact. The repaired
command resolves its runtime repository before output/configuration roots, and its production wrapper
supplies no outer roots. The actual frozen-source public absolute/relative/alias and invalid-repository
receipts remain bound; their other-host baseline comparison skip remains explicit.

Historical measurements are rebound by exact source span, operator and raw edit: the prefix/helper
cohort59, flag cohort14, repaired diagnostic region51 and finite-iterator pair2. Their original outcomes
stay historical. No fresh mutation outcome or whole-file equivalent validity follows from this
integration pass. Seven diagnostic-helper obligations still need source/domain assessment and their
finishing controls or precise proofs, followed by the complete current replay.

This checkpoint is local integration evidence. It does not establish remote CI, physical-board
execution or F30 adequacy. The caller defect and stale timing rationale are resolved; the next activity
is the seven outstanding helper judgments.


### Seven diagnostic-helper obligations: domain assessment

Assessment `.14.3.2.2.2.2.3` (`2026-10-18`) compiles the unchanged actual helper and all seven
remaining edits, together with the actual shared environment constructor. It separates real
[Cargo](ledger.md#rust-toolchain) diagnostic witnesses from constructed records that mark proof boundaries.

| Edit | Assessment | Next verification |
| --- | --- | --- |
| `L676.eq.231` | A target alias has a multiline raw spelling and an ordinary physical destination. Selecting the last non-target environment entry loses that raw alias; its command continuation replaces the compiler error. | Permanent alias parent and native replay |
| `L685.iter.233` | A multiline cwd emits a dual-configuration warning; its artifact target has a separate ordinary path. Last-path-only processing loses cwd and selects the warning continuation. | Permanent independent-cwd parent and native replay |
| `L717.clogic.1497` | A missing-target manifest makes Cargo print its leading error before a multiline manifest path. Removing the lower bound suppresses that actual error. | Permanent leading-error parent and native replay |
| `L690.cond.244` | Absolute-path examples agree; relative constructed records differ. | Exact absolute-prefix/caller proof with permanent premise guards |
| `L717.cmp.257` | Absolute-path examples agree; relative constructed records differ. | Exact absolute-prefix/caller proof with permanent premise guards |
| `L714.lit.253` | A constructed non-Cargo `erro:` line distinguishes the shorter literal. | Actual producer evidence or supported witness |
| `L717.cmp.258` | A constructed path ending in newline immediately before an error distinguishes the closed upper bound. | Actual producer/path-boundary evidence or supported witness |

The compiler cases use the actual build arguments and independently retain structured E0308 and
the literal `error: could not compile … due to 1 previous error` diagnostic. OS device/inode identity
confirms the artifact alias and physical directory are the same destination. The configuration-warning
case keeps cwd and artifact prefixes independent, so a shared multiline ancestor cannot mask omission
of cwd. The manifest case exercises the failed-command contract directly; it is not an admitted
inventory or a successful public gate.

The helper can consume arbitrary strings, but constructed records are not automatically possible
outputs of the pinned producer. Conversely, agreement on a finite set cannot prove a supported-domain
equivalence. The next slices therefore add the three real-Cargo parents first, prove the two
absolute-prefix proposals separately, and resolve the final two producer-dependent obligations.

All 1055 held regular inputs restore with modes and Python/system hashes; the submodule remains an
opaque Git-index identity. Four falsified bindings refuse. The installed write guard permits its
positive control and refuses the five forbidden operations; the actual Cargo witnesses use that guard.
Initial scratch-template and stale-binary attempts are excluded from accepted final receipts.
Production, operator controls, catalogue and existing reasons remain unchanged. These are source-bound
witnesses and scheduled proof obligations, with **no fresh native kill or equivalence classification**.
The historical nine-survivor measurement and later two finite-iterator proofs retain their own epochs.


### Real Cargo regression parents and exact replay

Regression slice `.14.3.2.2.2.2.4` (`2026-10-18`) turns the assessment's actual
[Cargo](ledger.md#rust-toolchain) witnesses into permanent isolated parents. Each begins with a
successful build, requires the failure diagnostic, then restores valid source/manifest bytes and
requires successful recovery.

The artifact-alias parent separates the raw multiline alias from its ordinary canonical destination.
The environment's target-local temporary directory is canonical, so selecting its last non-target
entry cannot accidentally preserve the raw alias. The cwd parent moves the workspace to a multiline
path while keeping the target elsewhere; a dual-configuration warning then requires cwd coverage.
Both compiler parents independently inspect structured E0308 and the literal final compiler error.

The leading-error parent temporarily writes a manifest with no target. Cargo prints its genuine
leading error before the multiline manifest path. The parent requires that leading physical line,
then restores the original manifest and builds successfully. This is a direct failed-Cargo control,
not an admitted public inventory.

| Exact edit | Actual finishing parent |
| --- | --- |
| `L676.eq.231` | `trust::tests::cargo_failure_diagnostics_keep_an_independent_target_alias` |
| `L685.iter.233` | `trust::tests::cargo_failure_diagnostics_keep_cwd_beside_a_separate_target` |
| `L717.clogic.1497` | `trust::tests::cargo_failure_diagnostics_keep_an_error_before_a_multiline_path` |

The unmodified three-parent baseline passes. The actual guarded native replay reports **three killed,
strict exit 0**, with precisely the finishing names above and no other outcome. The selected replay
establishes these regression kills; four proof/producer obligations remain before the complete current
region replay. It establishes no whole-file equivalent validity or F30 adequacy.

All 1056 original and current copied regular inputs restore, with modes and Python/system hashes.
The copy check borrows index metadata and makes no copied-index or submodule-content claim. Seven
false receipts refuse. Exact reconstruction removes only the added three-parent/helper test block
to recover every prior source byte. Production behavior, earlier tests, catalogue, operator controls
and existing reasons remain unchanged. Final held focused verification and separate book/staged
checks bind publication.


### Absolute diagnostic-prefix proofs

Proof slice `.14.3.2.2.2.2.5` (`2026-10-18`) resolves `L690.cond.244` and `L717.cmp.257`.
The argument covers every stderr string and environment value with absolute cwd on the supported
Unix callers. It uses path and line boundaries, independently of the producer's diagnostic vocabulary.

The caller chain supplies that absolute cwd. Inventory and baseline start from
[Cargo's](ledger.md#rust-toolchain) absolute manifest directory; the gate canonicalizes its selected
repository before deriving output. Inventory joins output with `tree`, and compile is the sole
production caller of `run_cargo`. The helper retains absolute targets or joins relative targets to cwd.
Raw/canonical spellings and their ancestors remain absolute. Lossy conversion and apostrophe quoting
preserve the leading slash.

The pinned [workspace constructor](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/workspace.rs)
rejects relative manifest paths. [Package roots](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/package.rs)
come from their manifest's parent; the held compiler setup exports that root as
`CARGO_MANIFEST_DIR`. The supported normal workspace build is the premise here. A permanent
control also compiles the actual repository-root function through a relative manifest argument and
directory alias, requiring its absolute value and physical identity.

The unconditional prefix edit adds only prefixes without newline. Such a match cannot straddle a
physical line start: doing so would include the preceding newline. If the match starts exactly at
the candidate, its slash disagrees with the candidate's `error` bytes. Added prefixes therefore
cannot suppress any candidate. For example, `/workspace` cannot begin at the start of
`error: actual diagnostic`, nor cross the newline immediately before it.

The strict-begin edit changes only equality between span begin and candidate start. Every recorded
prefix begins with slash, while a candidate begins with `error`. Equality is impossible. An original
containing span already begins strictly before the candidate, so both predicates select the same
diagnostic and return identical bytes.

| Exact edit | Universal premise | Native selected replay |
| --- | --- | --- |
| `L690.cond.244` | No-newline absolute prefixes cannot cover candidate line starts | equivalent |
| `L717.cmp.257` | A slash-prefixed span cannot begin at an error-prefixed candidate | equivalent |

The permanent operator control holds broad source-premise guards for the complete trust/gate
production slices and repository-root/CLI entry functions. A change there deliberately requires
revalidation. Actual original/two edited helpers compile, with literal diagnostics, aliases,
relative targets, both line endings and 4681 generated strings. Three altered source/caller bindings
refuse. Relative wider records distinguish both edits, and three wrong oracles fail. Finite records
support the source proof; they do not define its universe.

The complete operator suite passes **34 controls**. Only after the proof, two precise reasons are
added. The actual guarded selected replay reports **two equivalent, strict exit 0**, with no other
outcome and a passing original baseline. It does not validate unrelated reasons.

All 1058 original and current copied regular inputs restore with modes and Python/system hashes;
the copy borrows index metadata, and the submodule remains opaque. Six false receipt bindings refuse.
Every earlier source/test/reason byte is preserved; production, catalogue and native operator remain
unchanged. The two additional pinned sources join the earlier 28, all 30 independently hash-checked.
Revalidate after changes to guarded callers/root, constructors, conversion, framing/predicates or
Unix/toolchain contracts. Relative cwd is outside these two reasons. The shorter error literal and
closed span upper bound remain open before the complete current replay.


### Remaining diagnostic-producer assessment

Assessment `.14.3.2.2.2.2.6.1` (`2026-10-18`) tests the final literal and upper-bound edits
with actual [Cargo](ledger.md#rust-toolchain) output. It makes no new native classification.

A quoted unused manifest key can contain an escaped newline. Cargo emits the decoded key in a
warning before the compiler failure. A continuation `erroneous-warning` distinguishes the shortened
`erro` literal: the original retains the compiler error, while the shorter edit selects warning text.
This is a direct failed-command helper witness, like the earlier missing-target manifest control.
It is **outside the inventory's admitted manifest grammar**.

The actual maintained parser refuses both quoted multiline-key fixtures; a bare-key positive parses.
Inventory treats parse refusal separately from admissions and returns before program compilation.
A path-shaped quoted-key continuation also makes the original select warning data, while the
closed-end edit happens to mask it. That wider example establishes neither an admitted/public defect
nor a general repair. The initial suspected defect is narrowed by the verified syntax fence.

An ordinary-manifest case independently distinguishes the upper bound. Its containing configuration
has both file spellings, so Cargo prints a warning ending with the preferred configuration path,
a closing backtick and a newline. A distinct output ancestor is named to match those exact bytes.
Its target layout contains a colon, producing the existing genuine Unix library-path joining refusal
immediately after that warning.

    warning: both ... exist. Using `<repo>/.cargo/config`
    error: failed to join paths from `$…` together

The known ancestor spelling is `<repo>/.cargo/config` followed by a backtick and newline.
The text match ends exactly where the real error starts. The original open upper bound retains that
error; the closed bound suppresses it and returns empty. These paths are distinct filesystem objects:
the collision involves path spelling plus Cargo's framing bytes. This uses ordinary manifest and
configuration text with inventory-shaped output/cwd/target paths; permanent inventory reproduction
is the next parent, so this assessment claims no actual public-gate run.

| Edit | Real producer witness | Next parent |
| --- | --- | --- |
| `L714.lit.253` | Quoted-key continuation distinguishes the direct helper; inventory syntax refuses the manifest | Failed-command helper parent with explicit syntax-refusal boundary |
| `L717.cmp.258` | Configuration-warning terminator matches an ancestor and edited bound suppresses following setup error | Inventory boundary/operational parent with success and recovery |

A lexical-normalization alias hypothesis is refuted. Cargo retains its raw `/./` target spelling;
both error-like path controls agree for original and both edits. These are negative controls.

All 1058 held regular inputs restore with modes and Python/system hashes; five false bindings refuse.
Code, operator, catalogue and existing reasons remain unchanged. The read-only existing-commit source
archive has 4639 entries, including 2906 regular files verified byte-for-byte. All 30 prior source
pins agree with their archive copies and system hashes. No other Git checkout, dependency build
or source amendment is made. The cache binds evidence; no complete producer grammar or whole-file
adequacy is claimed.

Fixture metadata stores repository-relative bases. Current file access derives the root at runtime;
the audit derives historical text prefixes from the retained raw Cargo records. A different root
spelling resolves the same files, and the metadata correction preserves every raw diagnostic byte.


### Final diagnostic regression parents

Regression slice `.14.3.2.2.2.2.6.2` (`2026-10-18`) makes both remaining producer controls
permanent, using actual [Cargo](ledger.md#rust-toolchain) and bounded isolated parents.

The quoted-key parent starts with an ordinary successful inventory. It then commits a Cargo-valid
quoted key whose decoded name contains a newline and `erroneous-warning`. The inventory must refuse
the manifest syntax before creating any program target directory, and write no inventory. A direct
Cargo build succeeds and emits the expected warning. After a real type error is introduced, the
command helper must retain the actual compiler error and structured E0308. Restoring ordinary
source/manifest bytes makes both the direct command and inventory succeed. This guards the direct
failed-command contract without admitting quoted keys to inventory.

The boundary parent begins and ends with successful ordinary inventories. Its alternate output
lies below a directory whose spelling matches the containing configuration warning's final path,
closing backtick and newline. A colon in the target layout produces the genuine operational failure.
The actual inventory must retain the Cargo path-joining error for both roots and write no inventory.
The parent verifies that the legacy configuration is a file and the matching ancestor is a distinct
directory. It explicitly authorizes only the existing fixture's containing configuration root.

| Exact edit | Actual finishing parent |
| --- | --- |
| `L714.lit.253` | `trust::tests::cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning` |
| `L717.cmp.258` | `trust::tests::cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary` |

Both unmodified parents pass. The actual guarded selected native replay reports **two killed,
strict exit 0**, with precisely those finishing names and no other outcome. All 1060 original and
current copied regular inputs restore with modes and Python/system hashes; seven altered receipts
refuse. The copy borrows index metadata and makes no copied-index/submodule-content claim.

Exact reconstruction removes only the added test functions to recover every prior source byte.
Production behavior, earlier tests, operators, catalogue and existing reasons remain unchanged.
Final held focused verification and book/staged checks bind publication. This resolves the final
individual survivor obligations; the complete current region replay and full integration checkpoint
remain separate committed slices. Historical results are not added together as a substitute for
that new measurement, and no whole-file adequacy follows yet.


### Complete diagnostic-region replay

Replay `.14.4.1` (`2026-10-18`) freshly judges every current `cargo_error_line`/`run_cargo` edit
against the controlling full parent population. This is an actual new run, rather than the sum of
earlier partial outcomes. The unmodified copied tree passes; original and restored copied test
lists select the same **153 tests**, with route tests skipped and no ignored parent attributes.

| Scope/result | Actual observation |
| --- | --- |
| Global production listing | 1526 edits listed |
| Complete diagnostic region | 51 edits attempted |
| Named finishing kills | 43 |
| Proven equivalents | 4 |
| Compiler refusals | 4 |
| Survivors, timeouts, unjudged | 0 each |
| Strict native exit | 0 |

This local native-host run uses the [pinned toolchain](ledger.md#rust-toolchain). Source identity is
`069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba`; the held Git activation tree is `6306e488b8c4e06ce5fc1b39acb53c74586b3028`.
Later completion documents receive separate book/staged checks. These identities do not claim that
the later document tree itself was the native input or that a full integration stamp was produced.

Each row below is freshly observed. One actual finishing parent is shown for each kill; the native
receipt retains every reported failing name. Compiler refusals execute no mutant tests and establish
no coverage. Equivalent rows retain their individual source proofs and revalidation conditions.

| Exact edit | Change | Outcome | One actual finishing parent or precise reason |
| --- | --- | --- | --- |
| `L676.eq.231` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key != "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_keep_an_independent_target_alias` |
| `L685.iter.232` | Visit first positions of `for path in paths` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L685.iter.233` | Visit last positions of `for path in paths` | killed | `trust::tests::cargo_failure_diagnostics_keep_cwd_beside_a_separate_target` |
| `L685.iter.234` | Visit first and last positions of `for path in paths` | equivalent | [At-most-two path vector](#finite-cargo-path-iterator-proofs) |
| `L687.iter.235` | Visit first positions of `for spelling in std::iter::once(path).chain(canonical)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.236` | Visit last positions of `for spelling in std::iter::once(path).chain(canonical)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.237` | Visit first and last positions of `for spelling in std::iter::once(path).chain(canonical)` | equivalent | [At-most-two spelling chain](#finite-cargo-path-iterator-proofs) |
| `L688.iter.238` | Visit first positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.239` | Visit last positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.240` | Visit first and last positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.241` | Take newline-prefix guard only at first ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.242` | Take newline-prefix guard only at last ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.243` | Take newline-prefix guard only at first and last ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.cond.244` | `if prefix.contains('\n') {` → `if true {` | equivalent | [Absolute no-newline prefix](#absolute-diagnostic-prefix-proofs) |
| `L690.cond.245` | `if prefix.contains('\n') {` → `if false {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L691.stmt.246` | `prefixes.insert(prefix.replace('\'', "'\\''"));` → `// prefixes.insert(prefix.replace('\'', "'\\''"));` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L692.stmt.247` | `prefixes.insert(prefix.into_owned());` → `// prefixes.insert(prefix.into_owned());` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L706.iter.248` | Visit first positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.249` | Visit last positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.250` | Visit first and last positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L708.stmt.251` | `offset += physical.len();` → `// offset += physical.len();` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.dir.252` | `if line.starts_with("error")` → `if line.contains("error")` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.lit.253` | `if line.starts_with("error")` → `if line.starts_with("erro")` | killed | `trust::tests::cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning` |
| `L714.lit.254` | `if line.starts_with("error")` → `if line.starts_with("rror")` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.logic.255` | `&& !path_spans` → `\|\| !path_spans` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.not.256` | `&& !path_spans` → `&& path_spans` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.cmp.257` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin < start && start < end)` | equivalent | [Absolute prefix cannot begin at error](#absolute-diagnostic-prefix-proofs) |
| `L717.cmp.258` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start && start <= end)` | killed | `trust::tests::cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary` |
| `L717.logic.259` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start \|\| start < end)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L732.stmt.260` | `c.args(args).current_dir(cwd).env_clear();` → `// c.args(args).current_dir(cwd).env_clear();` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.261` | Visit first positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.262` | Visit last positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.263` | Visit first and last positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L734.stmt.264` | `c.env(k, v);` → `// c.env(k, v);` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L738.stmt.265` | `c.env("RUSTC", "rustc");` → `// c.env("RUSTC", "rustc");` | killed | `trust::tests::cargo_diagnostic_flags_do_not_enter_unit_configurations` |
| `L742.cond.266` | `if !out.status.success() {` → `if true {` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L742.cond.267` | `if !out.status.success() {` → `if false {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L742.not.268` | `if !out.status.success() {` → `if out.status.success() {` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L676.chain.1145` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().take(1).rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L676.chain.1146` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().rev().take(1).rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L687.chain.1147` | `for spelling in std::iter::once(path).chain(canonical) {` → `for spelling in std::iter::once(path)/* chain dropped */ {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.contains.1148` | `if prefix.contains('\n') {` → `if ((prefix).first() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` has no first method |
| `L690.contains.1149` | `if prefix.contains('\n') {` → `if ((prefix).first().copied() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` has no first method |
| `L690.contains.1150` | `if prefix.contains('\n') {` → `if ((prefix).last() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` is not an iterator |
| `L690.contains.1151` | `if prefix.contains('\n') {` → `if ((prefix).last().copied() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` is not an iterator |
| `L698.chain.1152` | `.iter()` → `.iter().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L698.chain.1153` | `.iter()` → `.iter().rev().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1154` | `.iter()` → `.iter().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1155` | `.iter()` → `.iter().rev().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.clogic.1497` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\|  start < end)` | killed | `trust::tests::cargo_failure_diagnostics_keep_an_error_before_a_multiline_path` |
| `L717.clogic.1498` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start )` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |

The per-operator totals independently agree with every attempted edit:

| Operator | Attempted | Compiler-refused |
| --- | ---: | ---: |
| eq | 1 | 0 |
| iter | 15 | 0 |
| guard | 3 | 0 |
| cond | 4 | 0 |
| stmt | 6 | 0 |
| dir | 1 | 0 |
| lit | 2 | 0 |
| logic | 2 | 0 |
| not | 2 | 0 |
| cmp | 2 | 0 |
| chain | 7 | 0 |
| contains | 4 | 4 |
| clogic | 2 | 0 |

Only the four applicable reason records are judged here. The per-file record retains 26 entries;
this bounded replay does not validate the rest. All five newly added finishing parents remain named
kills in the full population. The source/caller/environment guards stay exact, all 30 pinned source
copies independently hash-check, and Cargo/rustc metadata agrees before and after the run. Metadata
is not a binary-digest attestation.

All 1061 original and current copied regular inputs restore with modes and Python/system hashes.
The restored copied tree is then listed again, proving the same parent population and unchanged
copied inputs. Copied index metadata is borrowed; the submodule supplies only opaque index identity.
Ten altered receipts refuse: wrong exit, missing/duplicate row, foreign ID, wrong replacement,
missing/foreign finishing name, unlisted equivalent, wrong compiler-refusal reason and wrong
operator total.

To repeat this dated selection, prepare the local shell stores and the evidenced installed-host
write guard from the earlier [preflight](#run_cargo-locality-preflight) first. Check the
source identity and re-discover/rebind IDs after source changes; do not read the global listing as
an attempted whole-file sweep. The actual full parent command and complete selection are:

```bash
/usr/bin/sandbox-exec -D WRITABLE_ROOT="$PWD" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
    --timeout 600 --filter=trust:: --filter=trust_generated:: \
    --filter=trust_gate:: --filter=--skip --filter=route_tests \
    --timeouts-fail --equivalents xtask/trust.equivalents \
  --only L676.eq.231 \
  --only L685.iter.232 \
  --only L685.iter.233 \
  --only L685.iter.234 \
  --only L687.iter.235 \
  --only L687.iter.236 \
  --only L687.iter.237 \
  --only L688.iter.238 \
  --only L688.iter.239 \
  --only L688.iter.240 \
  --only L688.guard.241 \
  --only L688.guard.242 \
  --only L688.guard.243 \
  --only L690.cond.244 \
  --only L690.cond.245 \
  --only L691.stmt.246 \
  --only L692.stmt.247 \
  --only L706.iter.248 \
  --only L706.iter.249 \
  --only L706.iter.250 \
  --only L708.stmt.251 \
  --only L714.dir.252 \
  --only L714.lit.253 \
  --only L714.lit.254 \
  --only L715.logic.255 \
  --only L715.not.256 \
  --only L717.cmp.257 \
  --only L717.cmp.258 \
  --only L717.logic.259 \
  --only L732.stmt.260 \
  --only L733.iter.261 \
  --only L733.iter.262 \
  --only L733.iter.263 \
  --only L734.stmt.264 \
  --only L738.stmt.265 \
  --only L742.cond.266 \
  --only L742.cond.267 \
  --only L742.not.268 \
  --only L676.chain.1145 \
  --only L676.chain.1146 \
  --only L687.chain.1147 \
  --only L690.contains.1148 \
  --only L690.contains.1149 \
  --only L690.contains.1150 \
  --only L690.contains.1151 \
  --only L698.chain.1152 \
  --only L698.chain.1153 \
  --only L716.chain.1154 \
  --only L716.chain.1155 \
  --only L717.clogic.1497 \
  --only L717.clogic.1498
```

The complete `trust.rs` sweep remains separate. Full integration and final bounded-parent closure
are next in `.14.4.2`; no remote CI, physical-board or whole-file adequacy claim follows here.
