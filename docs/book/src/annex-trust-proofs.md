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

The complete `trust.rs` sweep remains separate. The following checkpoint closes full local integration
and the diagnostic cohort; no remote CI, physical-board or whole-file adequacy claim follows here.


### Diagnostic cohort integration checkpoint

Checkpoint `.14.4.2` (`2026-10-18`) closes the diagnostic helper/caller cohort after the
[complete region replay](#complete-diagnostic-region-replay). The actual final source retains that
51-edit result: 43 named kills, four exact supported-domain equivalents and four compiler refusals,
with strict exit 0 and the full 153-test parent population. This integration run does not replace or
add mutation outcomes.

| Local integration step | Result |
| --- | --- |
| format | passed |
| strict lint | passed |
| complete contract suite | passed |
| doctrines | passed |
| gate self-tests | passed |
| pinned [mdBook](ledger.md#mdbook) | passed |
| no_std build | passed |
| pinned toolchain premises | passed |
| wasm build | passed |
| wasm binding | passed |
| emulator configuration | passed |
| target spike | passed |

```text
12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
activation tree: 5fc854ec08a322af168a2615934a988a6e9e1b47
```

The actual log and exit, official tier stamp and native Git identity agree. The maintained input
census restores all 1062 held regular inputs, including modes and Python/system hashes; the gitlink
is an opaque index identity. Five altered bindings refuse: nonzero exit, missing step, false summary,
wrong stamp and missing restoration. Final source/operator/catalogue/reason bytes remain exact,
the shared environment constructor and three-name compiled-input allowlist remain unchanged, and all
30 pinned source copies and tool metadata revalidate. The permanent operator suite has 34 controls.
Tool metadata is not executable-digest evidence.

Completion prose is a later document epoch with separate pinned-book and staged doctrine checks.
The integration stamp names the activation above; it does not claim this later prose was itself
the full tier's input. This closes the bounded diagnostic cohort, including its repaired caller and
framing defect, exact reasons and regression gaps. The 26-record equivalent file has only the four
applicable diagnostic reasons judged by this region run. Full-file adequacy, remote CI and physical-board
execution remain separate.

The next action is a survivor-ownership reconciliation, rather than the final whole-file sweep.
The `2026-10-18` task-tree census finds 463 historical rows still assigned to the general assessment
parent. This is an ownership-table observation, not a fresh count of current survivors: later repairs
may have changed or already covered some rows. Task `.15.1` binds every historical row to current
raw edits, retired edits or completed evidence, then schedules the next bounded assessment. The
controlling final-sweep prerequisite remains all required assessments and repairs complete.


### Survivor ownership reconciliation

The `2026-10-18` census (`.15.1`) reconciles every survivor from the initial complete
`trust.rs` measurement at `36154d6`. Native history remains 802 killed, 540 survived and
78 compiler refusals, with strict exit 1. The current production listing contains 1526 candidates.
Mapping old text to current text supplies no new mutation verdict or equivalence.

| Source-identity observation | Initial survivor rows |
| --- | ---: |
| Complete unchanged source span and exact edit retained | 512 |
| No current exact edit | 28 |
| Ambiguous mapping | 0 |
| Bound to a completed actual measurement by current raw edit | 70 |

The fresh-name portion of a generated iterator identifier contains its source line. The constructor
is unchanged and that prefix is absent from both complete sources; only its line number is normalized
when comparing replacements. The original fragments and every other replacement byte remain exact.
All retained rows also lie in complete unchanged source blocks. A missing exact edit means its old
text disappeared, not that its replacement behavior or new candidates pass.

For example, the old inline command framing in `compile` moved to the repaired `cargo_commands`
and `dependency_files` path. Some old edits therefore disappear, while retained ownership/configuration
edits still require assessment. Their replacement helpers have their own current mutation shapes.
The old `run_cargo` selector edits are covered by its separately documented repair and fresh complete
region replay; the initial raw outcomes remain historical.

The census finds 463 rows still assigned to the general parent before this update. They now all have
the concrete lanes below. Of these, 442 retain an exact current edit and 21 lack an exact current edit;
neither category is declared a current survivor or a pass. The completed 77-row families retain their
previous owners; 70 have a retained raw edit and seven lack one.

| Next assessment lane | Source scope | Historical rows assigned | Current candidates in the full lane |
| --- | --- | ---: | ---: |
| `.15.2` — Compiler-input path | `compile` plus `cargo_commands` and `dependency_files` | 48 | 159 |
| `.15.3` — Build artifacts | `build` | 21 | 34 |
| `.15.4` — Source extents | `refused_sites`, `site_extent` | 24 | 90 |
| `.15.5` — Compiled rule names | `inadmissible`, `compiled_names` | 18 | 42 |
| `.15.6` — Tool identity output | `tool_line` | 2 | 5 |
| `.15.7` — Target and program closure | `target_closure`, `classify_programs` | 30 | 90 |
| `.15.8` — Refusal and admission state | `refuse`, `Rules` | 60 | 135 |
| `.15.9` — Inventory orchestration | `inventory_with` | 237 | 688 |
| `.15.10` — Trust command | `run` | 15 | 15 |
| `.15.11` — Harness-relative units | `beside` | 8 | 15 |

The following measurement covers the complete current `compile` function (`.15.2.1.1`):
83 candidates against the full parent command, with strict timeout/unjudged rules and whole-input
restoration. Its command-record and dependency helpers follow as separate bounded children; actual
survivors gain controls, owned repairs or exact proofs before final complete replay. The remaining
lanes follow the same discipline. All required assessments precede the final whole-file sweep.

The maintained historical native auditor and actual current native listing agree with the census.
Independent old/current source-span and complete replacement checks cover every row; six altered
censuses refuse (missing/duplicate row, foreign current ID, wrong replacement, false retirement and
wrong ownership count). All 1062 held regular inputs restore before these later completion documents.
No Rust, operator, catalogue or equivalent reason changes. The closed diagnostic task subtree is
sealed with exact reconstruction and all previous archives preserved; the book remains unbounded.

The following tables expose every initial row, its current raw identity or its absence, the bound
historical verdict when one is available, and its assessment owner. Owner suffixes are under
`M3.6.9.6.3.2.2`; older families keep their previously completed records. “No bound result here”
does not erase earlier evidence or infer a failure: the census only binds the explicitly audited
receipt set. Every final current outcome still requires its actual native run.

#### Initial `read_roots` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L185.cond.2` | `L185.cond.2` | killed (historical) | `.1` |

#### Initial `read_form` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L235.iter.25` | `L238.iter.25` | killed (historical) | `.2` |
| `L235.iter.26` | `L238.iter.26` | killed (historical) | `.2` |
| `L235.guard.28` | `L238.guard.28` | killed (historical) | `.2` |
| `L235.guard.29` | `L238.guard.29` | killed (historical) | `.2` |
| `L235.guard.30` | `L238.guard.30` | killed (historical) | `.2` |
| `L235.guard.31` | `L238.guard.31` | killed (historical) | `.2` |
| `L235.guard.32` | `L238.guard.32` | killed (historical) | `.2` |
| `L264.stmt.42` | `L267.stmt.42` | killed (historical) | `.3` |
| `L317.stmt.47` | `L320.stmt.47` | killed (historical) | `.4` |
| `L263.chain.1020` | `L266.chain.1104` | killed (historical) | `.3` |
| `L312.chain.1029` | `L315.chain.1113` | killed (historical) | `.3` |
| `L312.chain.1030` | `L315.chain.1114` | killed (historical) | `.3` |

#### Initial `check_pairs` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L334.iter.51` | `L340.iter.52` | killed (historical) | `.7` |
| `L334.iter.52` | `L340.iter.53` | killed (historical) | `.7` |
| `L334.guard.53` | `L340.guard.54` | killed (historical) | `.7` |
| `L334.guard.54` | `L340.guard.55` | killed (historical) | `.7` |
| `L334.guard.55` | `L340.guard.56` | killed (historical) | `.7` |
| `L336.iter.58` | `L342.iter.59` | equivalent (historical) | `.8.2` |
| `L348.cond.64` | `L354.cond.65` | killed (historical) | `.7` |
| `L340.clogic.1392` | `L346.clogic.1496` | killed (historical) | `.7` |

#### Initial `unit_configuration` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L376.iter.65` | no current exact edit | no bound result here | `.9.1` |
| `L376.iter.66` | no current exact edit | no bound result here | `.9.1` |
| `L376.iter.67` | no current exact edit | no bound result here | `.9.1` |
| `L377.stmt.68` | no current exact edit | no bound result here | `.9.1` |

#### Initial `split_command` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L388.bool.71` | `L393.bool.67` | equivalent (historical) | `.10.3.4` |
| `L392.not.78` | `L399.not.69` | killed (historical) | `.10.3.4` |
| `L392.stmt.79` | `L399.stmt.70` | killed (historical) | `.10.3.4` |
| `L393.bool.80` | `L400.bool.71` | killed (historical) | `.10.3.4` |
| `L393.stmt.81` | `L400.stmt.72` | killed (historical) | `.10.3.4` |
| `L396.logic.84` | `L403.logic.75` | killed (historical) | `.10.3.4` |
| `L396.cond.86` | `L403.cond.77` | equivalent (historical) | `.10.3.4` |
| `L399.bool.90` | `L406.bool.81` | equivalent (historical) | `.10.3.4` |
| `L399.stmt.91` | `L406.stmt.82` | equivalent (historical) | `.10.3.4` |
| `L404.logic.92` | `L411.logic.83` | killed (historical) | `.10.3.4` |
| `L404.logic.93` | `L411.logic.84` | killed (historical) | `.10.3.4` |
| `L404.logic.94` | `L411.logic.85` | killed (historical) | `.10.3.4` |
| `L404.cond.95` | `L411.cond.86` | equivalent (historical) | `.10.3.4` |
| `L404.cond.96` | `L411.cond.87` | killed (historical) | `.10.3.4` |
| `L404.not.97` | `L411.not.88` | killed (historical) | `.10.3.4` |
| `L405.stmt.98` | `L412.stmt.89` | killed (historical) | `.10.3.4` |

#### Initial `DROPPED` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L416.bool.101` | `L554.bool.158` | killed (historical) | `.11.2.2` |
| `L417.bool.102` | `L555.bool.159` | equivalent (historical) | `.11.2.4.2` |
| `L418.bool.103` | `L556.bool.160` | equivalent (historical) | `.11.2.4.1` |
| `L419.bool.104` | `L557.bool.161` | equivalent (historical) | `.11.2.4.1` |
| `L420.bool.105` | `L558.bool.162` | equivalent (historical) | `.11.2.4.1` |
| `L421.bool.106` | `L559.bool.163` | equivalent (historical) | `.11.2.4.1` |
| `L422.bool.107` | `L560.bool.164` | equivalent (historical) | `.11.2.4.1` |
| `L423.bool.108` | `L561.bool.165` | equivalent (historical) | `.11.2.4.1` |
| `L424.bool.109` | `L562.bool.166` | equivalent (historical) | `.11.2.4.1` |
| `L425.bool.110` | `L563.bool.167` | equivalent (historical) | `.11.2.4.1` |
| `L426.bool.111` | `L564.bool.168` | equivalent (historical) | `.11.2.4.3.2` |

#### Initial `normalise` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L452.cond.120` | `L590.cond.177` | killed (historical) | `.13.1` |
| `L452.dir.121` | `L590.dir.178` | killed (historical) | `.13.2.2` |
| `L453.int.122` | `L591.int.179` | killed (historical) | `.13.1` |
| `L454.stmt.125` | `L592.stmt.182` | killed (historical) | `.13.1` |
| `L457.int.129` | `L595.int.186` | killed (historical) | `.13.1` |
| `L457.int.130` | `L595.int.187` | killed (historical) | `.13.2.3` |
| `L459.dir.131` | `L597.dir.188` | equivalent/survived (historical) | `.13.2.4.2.1` |
| `L459.lit.132` | `L597.lit.189` | equivalent/survived (historical) | `.13.2.4.2.1` |
| `L460.dir.135` | `L598.dir.192` | equivalent/survived (historical) | `.13.2.4.2.2` |
| `L460.lit.136` | `L598.lit.193` | equivalent/survived (historical) | `.13.2.4.2.2` |
| `L461.dir.139` | `L599.dir.196` | equivalent/survived (historical) | `.13.2.4.2.3` |
| `L461.lit.140` | `L599.lit.197` | equivalent/survived (historical) | `.13.2.4.2.3` |
| `L466.stmt.146` | `L604.stmt.203` | killed (historical) | `.13.2.3` |
| `L467.int.148` | `L605.int.205` | killed (historical) | `.13.2.3` |
| `L472.int.155` | `L610.int.212` | killed (historical) | `.13.1` |
| `L473.dir.156` | `L611.dir.213` | killed (historical) | `.13.2.2` |
| `L474.dir.157` | `L612.dir.214` | killed (historical) | `.13.1` |
| `L476.cmp.158` | `L614.cmp.215` | equivalent (historical) | `.13.2.5` |
| `L481.stmt.159` | `L619.stmt.216` | killed (historical) | `.13.1` |
| `L452.chain.1035` | `L590.chain.1129` | killed (historical) | `.13.1` |
| `L452.chain.1036` | `L590.chain.1130` | killed (historical) | `.13.1` |

#### Initial `run_cargo` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L541.iter.177` | `L733.iter.263` | killed (historical) | `.14` |
| `L550.dir.182` | no current exact edit | no bound result here | `.14` |
| `L550.lit.183` | no current exact edit | no bound result here | `.14` |
| `L550.lit.184` | no current exact edit | no bound result here | `.14` |

#### Initial `compile` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L606.guard.189` | no current exact edit | no bound result here | `.15.2` |
| `L606.guard.190` | no current exact edit | no bound result here | `.15.2` |
| `L606.guard.191` | no current exact edit | no bound result here | `.15.2` |
| `L610.int.193` | no current exact edit | no bound result here | `.15.2` |
| `L610.int.194` | no current exact edit | no bound result here | `.15.2` |
| `L612.cond.196` | `L799.cond.277` | no bound result here | `.15.2` |
| `L627.dir.203` | `L814.dir.284` | no bound result here | `.15.2` |
| `L627.lit.204` | `L814.lit.285` | no bound result here | `.15.2` |
| `L655.iter.210` | `L841.iter.290` | no bound result here | `.15.2` |
| `L655.guard.212` | `L841.guard.292` | no bound result here | `.15.2` |
| `L655.guard.213` | `L841.guard.293` | no bound result here | `.15.2` |
| `L655.guard.214` | no current exact edit | no bound result here | `.15.2` |
| `L655.guard.215` | no current exact edit | no bound result here | `.15.2` |
| `L655.guard.216` | no current exact edit | no bound result here | `.15.2` |
| `L659.dir.224` | `L845.dir.301` | no bound result here | `.15.2` |
| `L659.lit.225` | `L845.lit.302` | no bound result here | `.15.2` |
| `L667.stmt.228` | `L857.stmt.304` | no bound result here | `.15.2` |
| `L669.logic.229` | no current exact edit | no bound result here | `.15.2` |
| `L669.logic.230` | no current exact edit | no bound result here | `.15.2` |
| `L669.logic.231` | no current exact edit | no bound result here | `.15.2` |
| `L669.cond.233` | no current exact edit | no bound result here | `.15.2` |
| `L670.stmt.235` | `L871.stmt.314` | no bound result here | `.15.2` |
| `L672.bool.236` | no current exact edit | no bound result here | `.15.2` |
| `L672.stmt.237` | no current exact edit | no bound result here | `.15.2` |
| `L673.dir.238` | no current exact edit | no bound result here | `.15.2` |
| `L676.dir.241` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.245` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.246` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.247` | no current exact edit | no bound result here | `.15.2` |
| `L683.cond.249` | no current exact edit | no bound result here | `.15.2` |
| `L703.iter.253` | `L885.iter.316` | no bound result here | `.15.2` |
| `L703.iter.254` | `L885.iter.317` | no bound result here | `.15.2` |
| `L703.iter.255` | `L885.iter.318` | no bound result here | `.15.2` |
| `L703.guard.256` | `L885.guard.319` | no bound result here | `.15.2` |
| `L703.guard.257` | `L885.guard.320` | no bound result here | `.15.2` |
| `L703.guard.258` | `L885.guard.321` | no bound result here | `.15.2` |
| `L704.logic.259` | `L886.logic.322` | no bound result here | `.15.2` |
| `L704.logic.260` | `L886.logic.323` | no bound result here | `.15.2` |
| `L704.logic.261` | `L886.logic.324` | no bound result here | `.15.2` |
| `L704.cond.263` | `L886.cond.326` | no bound result here | `.15.2` |
| `L729.cond.269` | `L911.cond.332` | no bound result here | `.15.2` |
| `L639.chain.1057` | `L826.chain.1162` | no bound result here | `.15.2` |
| `L639.chain.1058` | `L826.chain.1163` | no bound result here | `.15.2` |
| `L659.contains.1059` | `L845.contains.1164` | no bound result here | `.15.2` |
| `L659.contains.1060` | `L845.contains.1165` | no bound result here | `.15.2` |
| `L659.contains.1061` | `L845.contains.1166` | no bound result here | `.15.2` |
| `L659.contains.1062` | `L845.contains.1167` | no bound result here | `.15.2` |
| `L676.chain.1065` | no current exact edit | no bound result here | `.15.2` |

#### Initial `build` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L768.guard.274` | `L950.guard.337` | no bound result here | `.15.3` |
| `L768.guard.275` | `L950.guard.338` | no bound result here | `.15.3` |
| `L768.guard.276` | `L950.guard.339` | no bound result here | `.15.3` |
| `L768.guard.277` | `L950.guard.340` | no bound result here | `.15.3` |
| `L768.guard.278` | `L950.guard.341` | no bound result here | `.15.3` |
| `L768.guard.279` | `L950.guard.342` | no bound result here | `.15.3` |
| `L770.cond.282` | `L952.cond.345` | no bound result here | `.15.3` |
| `L771.stmt.283` | `L953.stmt.346` | no bound result here | `.15.3` |
| `L777.cond.286` | `L959.cond.349` | no bound result here | `.15.3` |
| `L778.stmt.287` | `L960.stmt.350` | no bound result here | `.15.3` |
| `L795.logic.290` | `L977.logic.353` | no bound result here | `.15.3` |
| `L798.logic.293` | `L980.logic.356` | no bound result here | `.15.3` |
| `L789.chain.1072` | `L971.chain.1174` | no bound result here | `.15.3` |
| `L789.chain.1073` | `L971.chain.1175` | no bound result here | `.15.3` |
| `L795.chain.1074` | `L977.chain.1176` | no bound result here | `.15.3` |
| `L795.chain.1075` | `L977.chain.1177` | no bound result here | `.15.3` |
| `L798.chain.1076` | `L980.chain.1178` | no bound result here | `.15.3` |
| `L798.chain.1077` | `L980.chain.1179` | no bound result here | `.15.3` |
| `L801.chain.1078` | `L983.chain.1180` | no bound result here | `.15.3` |
| `L801.chain.1079` | `L983.chain.1181` | no bound result here | `.15.3` |
| `L805.chain.1080` | `L987.chain.1182` | no bound result here | `.15.3` |

#### Initial `refused_sites` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L835.guard.298` | `L1017.guard.361` | no bound result here | `.15.4` |
| `L835.guard.299` | `L1017.guard.362` | no bound result here | `.15.4` |
| `L835.guard.300` | `L1017.guard.363` | no bound result here | `.15.4` |
| `L848.int.305` | `L1030.int.368` | no bound result here | `.15.4` |
| `L852.cmp.307` | `L1034.cmp.370` | no bound result here | `.15.4` |
| `L853.eq.308` | `L1035.eq.371` | no bound result here | `.15.4` |
| `L855.int.311` | `L1037.int.374` | no bound result here | `.15.4` |
| `L857.cond.316` | `L1039.cond.379` | no bound result here | `.15.4` |
| `L860.iter.317` | `L1042.iter.380` | no bound result here | `.15.4` |
| `L860.iter.318` | `L1042.iter.381` | no bound result here | `.15.4` |
| `L860.iter.319` | `L1042.iter.382` | no bound result here | `.15.4` |

#### Initial `site_extent` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L911.cond.340` | `L1093.cond.403` | no bound result here | `.15.4` |
| `L924.int.345` | `L1106.int.408` | no bound result here | `.15.4` |
| `L931.cmp.347` | `L1113.cmp.410` | no bound result here | `.15.4` |
| `L933.logic.351` | `L1115.logic.414` | no bound result here | `.15.4` |
| `L951.cmp.361` | `L1133.cmp.424` | no bound result here | `.15.4` |
| `L954.stmt.363` | `L1136.stmt.426` | no bound result here | `.15.4` |
| `L955.int.364` | `L1137.int.427` | no bound result here | `.15.4` |
| `L955.int.365` | `L1137.int.428` | no bound result here | `.15.4` |
| `L962.stmt.369` | `L1144.stmt.432` | no bound result here | `.15.4` |
| `L963.int.371` | `L1145.int.434` | no bound result here | `.15.4` |
| `L970.stmt.375` | `L1152.stmt.438` | no bound result here | `.15.4` |
| `L917.chain.1086` | `L1099.chain.1188` | no bound result here | `.15.4` |
| `L917.chain.1087` | `L1099.chain.1189` | no bound result here | `.15.4` |

#### Initial `inadmissible` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L996.bool.382` | `L1178.bool.445` | no bound result here | `.15.5` |
| `L999.eq.383` | `L1181.eq.446` | no bound result here | `.15.5` |
| `L999.logic.385` | `L1181.logic.448` | no bound result here | `.15.5` |
| `L994.chain.1088` | `L1176.chain.1190` | no bound result here | `.15.5` |
| `L998.chain.1091` | `L1180.chain.1193` | no bound result here | `.15.5` |
| `L989.clogic.1393` | `L1171.clogic.1499` | no bound result here | `.15.5` |
| `L995.clogic.1396` | `L1177.clogic.1502` | no bound result here | `.15.5` |
| `L999.clogic.1397` | `L1181.clogic.1503` | no bound result here | `.15.5` |
| `L999.clogic.1398` | `L1181.clogic.1504` | no bound result here | `.15.5` |

#### Initial `compiled_names` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1014.cmp.387` | `L1196.cmp.450` | no bound result here | `.15.5` |
| `L1019.guard.393` | `L1201.guard.456` | no bound result here | `.15.5` |
| `L1019.guard.394` | `L1201.guard.457` | no bound result here | `.15.5` |
| `L1019.guard.395` | `L1201.guard.458` | no bound result here | `.15.5` |
| `L1020.cond.398` | `L1202.cond.461` | no bound result here | `.15.5` |
| `L1021.stmt.400` | `L1203.stmt.463` | no bound result here | `.15.5` |
| `L1026.eq.401` | `L1208.eq.464` | no bound result here | `.15.5` |
| `L1028.int.402` | `L1210.int.465` | no bound result here | `.15.5` |
| `L1028.int.403` | `L1210.int.466` | no bound result here | `.15.5` |

#### Initial `tool_line` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1073.iter.410` | `L1255.iter.473` | no bound result here | `.15.6` |
| `L1073.iter.412` | `L1255.iter.475` | no bound result here | `.15.6` |

#### Initial `target_closure` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1098.stmt.417` | `L1280.stmt.480` | no bound result here | `.15.7` |
| `L1100.iter.419` | `L1282.iter.482` | no bound result here | `.15.7` |
| `L1100.iter.420` | `L1282.iter.483` | no bound result here | `.15.7` |
| `L1100.guard.422` | `L1282.guard.485` | no bound result here | `.15.7` |
| `L1100.guard.423` | `L1282.guard.486` | no bound result here | `.15.7` |
| `L1103.logic.435` | `L1285.logic.498` | no bound result here | `.15.7` |
| `L1103.logic.437` | `L1285.logic.500` | no bound result here | `.15.7` |
| `L1103.contains.1098` | `L1285.contains.1200` | no bound result here | `.15.7` |
| `L1103.contains.1100` | `L1285.contains.1202` | no bound result here | `.15.7` |

#### Initial `classify_programs` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1128.logic.453` | `L1310.logic.516` | no bound result here | `.15.7` |
| `L1129.logic.455` | `L1311.logic.518` | no bound result here | `.15.7` |
| `L1139.logic.460` | `L1321.logic.523` | no bound result here | `.15.7` |
| `L1118.chain.1102` | `L1300.chain.1204` | no bound result here | `.15.7` |
| `L1119.chain.1104` | `L1301.chain.1206` | no bound result here | `.15.7` |
| `L1119.chain.1106` | `L1301.chain.1208` | no bound result here | `.15.7` |
| `L1120.chain.1107` | `L1302.chain.1209` | no bound result here | `.15.7` |
| `L1120.chain.1108` | `L1302.chain.1210` | no bound result here | `.15.7` |
| `L1125.chain.1109` | `L1307.chain.1211` | no bound result here | `.15.7` |
| `L1133.chain.1111` | `L1315.chain.1213` | no bound result here | `.15.7` |
| `L1134.chain.1113` | `L1316.chain.1215` | no bound result here | `.15.7` |
| `L1134.chain.1114` | `L1316.chain.1216` | no bound result here | `.15.7` |
| `L1134.contains.1117` | `L1316.contains.1219` | no bound result here | `.15.7` |
| `L1151.chain.1122` | `L1333.chain.1224` | no bound result here | `.15.7` |
| `L1151.chain.1123` | `L1333.chain.1225` | no bound result here | `.15.7` |
| `L1169.chain.1125` | `L1351.chain.1227` | no bound result here | `.15.7` |
| `L1170.chain.1126` | `L1352.chain.1228` | no bound result here | `.15.7` |
| `L1170.chain.1127` | `L1352.chain.1229` | no bound result here | `.15.7` |
| `L1139.clogic.1399` | `L1321.clogic.1505` | no bound result here | `.15.7` |
| `L1139.clogic.1400` | `L1321.clogic.1506` | no bound result here | `.15.7` |
| `L1139.clogic.1401` | `L1321.clogic.1507` | no bound result here | `.15.7` |

#### Initial `refuse` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1192.stmt.466` | `L1374.stmt.529` | no bound result here | `.15.8` |
| `L1193.stmt.467` | `L1375.stmt.530` | no bound result here | `.15.8` |

#### Initial `Rules` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1224.int.468` | `L1408.int.531` | no bound result here | `.15.8` |
| `L1224.int.469` | `L1408.int.532` | no bound result here | `.15.8` |
| `L1224.stmt.470` | `L1408.stmt.533` | no bound result here | `.15.8` |
| `L1230.cond.472` | `L1414.cond.535` | no bound result here | `.15.8` |
| `L1231.int.473` | `L1415.int.536` | no bound result here | `.15.8` |
| `L1231.int.474` | `L1415.int.537` | no bound result here | `.15.8` |
| `L1231.stmt.475` | `L1415.stmt.538` | no bound result here | `.15.8` |
| `L1249.guard.480` | `L1433.guard.543` | no bound result here | `.15.8` |
| `L1249.guard.481` | `L1433.guard.544` | no bound result here | `.15.8` |
| `L1249.guard.482` | `L1433.guard.545` | no bound result here | `.15.8` |
| `L1253.iter.486` | `L1437.iter.549` | no bound result here | `.15.8` |
| `L1253.iter.487` | `L1437.iter.550` | no bound result here | `.15.8` |
| `L1253.iter.488` | `L1437.iter.551` | no bound result here | `.15.8` |
| `L1261.cond.492` | `L1445.cond.555` | no bound result here | `.15.8` |
| `L1262.stmt.494` | `L1446.stmt.557` | no bound result here | `.15.8` |
| `L1278.int.495` | `L1472.int.574` | no bound result here | `.15.8` |
| `L1278.int.496` | `L1472.int.575` | no bound result here | `.15.8` |
| `L1278.stmt.497` | `L1472.stmt.576` | no bound result here | `.15.8` |
| `L1286.int.500` | `L1480.int.579` | no bound result here | `.15.8` |
| `L1308.guard.508` | `L1502.guard.587` | no bound result here | `.15.8` |
| `L1308.guard.509` | `L1502.guard.588` | no bound result here | `.15.8` |
| `L1308.guard.510` | `L1502.guard.589` | no bound result here | `.15.8` |
| `L1308.guard.511` | `L1502.guard.590` | no bound result here | `.15.8` |
| `L1308.guard.512` | `L1502.guard.591` | no bound result here | `.15.8` |
| `L1308.guard.513` | `L1502.guard.592` | no bound result here | `.15.8` |
| `L1308.guard.514` | `L1502.guard.593` | no bound result here | `.15.8` |
| `L1308.guard.515` | `L1502.guard.594` | no bound result here | `.15.8` |
| `L1313.cond.520` | `L1507.cond.599` | no bound result here | `.15.8` |
| `L1317.stmt.521` | `L1511.stmt.600` | no bound result here | `.15.8` |
| `L1325.cond.525` | `L1519.cond.604` | no bound result here | `.15.8` |
| `L1338.iter.529` | `L1532.iter.608` | no bound result here | `.15.8` |
| `L1363.guard.536` | `L1557.guard.615` | no bound result here | `.15.8` |
| `L1363.guard.537` | `L1557.guard.616` | no bound result here | `.15.8` |
| `L1363.guard.538` | `L1557.guard.617` | no bound result here | `.15.8` |
| `L1367.iter.542` | `L1561.iter.621` | no bound result here | `.15.8` |
| `L1367.iter.543` | `L1561.iter.622` | no bound result here | `.15.8` |
| `L1367.iter.544` | `L1561.iter.623` | no bound result here | `.15.8` |
| `L1373.eq.546` | `L1567.eq.625` | no bound result here | `.15.8` |
| `L1373.logic.549` | `L1567.logic.628` | no bound result here | `.15.8` |
| `L1373.cond.551` | `L1567.cond.630` | no bound result here | `.15.8` |
| `L1374.iter.553` | `L1568.iter.632` | no bound result here | `.15.8` |
| `L1374.iter.554` | `L1568.iter.633` | no bound result here | `.15.8` |
| `L1374.iter.555` | `L1568.iter.634` | no bound result here | `.15.8` |
| `L1375.iter.556` | `L1569.iter.635` | no bound result here | `.15.8` |
| `L1375.iter.557` | `L1569.iter.636` | no bound result here | `.15.8` |
| `L1375.iter.558` | `L1569.iter.637` | no bound result here | `.15.8` |
| `L1382.iter.564` | `L1576.iter.643` | no bound result here | `.15.8` |
| `L1382.iter.565` | `L1576.iter.644` | no bound result here | `.15.8` |
| `L1382.guard.566` | `L1576.guard.645` | no bound result here | `.15.8` |
| `L1382.guard.567` | `L1576.guard.646` | no bound result here | `.15.8` |
| `L1382.guard.568` | `L1576.guard.647` | no bound result here | `.15.8` |
| `L1383.cond.570` | `L1577.cond.649` | no bound result here | `.15.8` |
| `L1253.chain.1133` | `L1437.chain.1235` | no bound result here | `.15.8` |
| `L1253.chain.1134` | `L1437.chain.1236` | no bound result here | `.15.8` |
| `L1312.chain.1135` | `L1506.chain.1239` | no bound result here | `.15.8` |
| `L1312.chain.1136` | `L1506.chain.1240` | no bound result here | `.15.8` |
| `L1313.contains.1137` | `L1507.contains.1241` | no bound result here | `.15.8` |
| `L1313.contains.1139` | `L1507.contains.1243` | no bound result here | `.15.8` |

#### Initial `inventory_with` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1435.eq.577` | `L1629.eq.657` | no bound result here | `.15.9` |
| `L1459.iter.584` | `L1653.iter.664` | no bound result here | `.15.9` |
| `L1459.iter.586` | `L1653.iter.666` | no bound result here | `.15.9` |
| `L1460.iter.587` | `L1654.iter.667` | no bound result here | `.15.9` |
| `L1460.iter.588` | `L1654.iter.668` | no bound result here | `.15.9` |
| `L1460.iter.589` | `L1654.iter.669` | no bound result here | `.15.9` |
| `L1460.guard.590` | `L1654.guard.670` | no bound result here | `.15.9` |
| `L1460.guard.591` | `L1654.guard.671` | no bound result here | `.15.9` |
| `L1460.guard.592` | `L1654.guard.672` | no bound result here | `.15.9` |
| `L1461.logic.594` | `L1655.logic.674` | no bound result here | `.15.9` |
| `L1494.cond.600` | `L1688.cond.680` | no bound result here | `.15.9` |
| `L1598.iter.613` | `L1792.iter.693` | no bound result here | `.15.9` |
| `L1598.iter.614` | `L1792.iter.694` | no bound result here | `.15.9` |
| `L1599.iter.617` | `L1793.iter.697` | no bound result here | `.15.9` |
| `L1599.guard.620` | `L1793.guard.700` | no bound result here | `.15.9` |
| `L1607.iter.624` | `L1801.iter.704` | no bound result here | `.15.9` |
| `L1607.iter.626` | `L1801.iter.706` | no bound result here | `.15.9` |
| `L1607.guard.627` | `L1801.guard.707` | no bound result here | `.15.9` |
| `L1607.guard.629` | `L1801.guard.709` | no bound result here | `.15.9` |
| `L1619.eq.632` | `L1813.eq.712` | no bound result here | `.15.9` |
| `L1619.logic.634` | `L1813.logic.714` | no bound result here | `.15.9` |
| `L1647.iter.645` | `L1841.iter.725` | no bound result here | `.15.9` |
| `L1652.iter.646` | `L1846.iter.726` | no bound result here | `.15.9` |
| `L1652.iter.647` | `L1846.iter.727` | no bound result here | `.15.9` |
| `L1652.iter.648` | `L1846.iter.728` | no bound result here | `.15.9` |
| `L1682.iter.654` | `L1877.iter.734` | no bound result here | `.15.9` |
| `L1687.stmt.658` | `L1882.stmt.738` | no bound result here | `.15.9` |
| `L1689.iter.659` | `L1884.iter.739` | no bound result here | `.15.9` |
| `L1689.iter.661` | `L1884.iter.741` | no bound result here | `.15.9` |
| `L1689.guard.662` | `L1884.guard.742` | no bound result here | `.15.9` |
| `L1689.guard.664` | `L1884.guard.744` | no bound result here | `.15.9` |
| `L1690.eq.666` | `L1885.eq.746` | no bound result here | `.15.9` |
| `L1690.logic.669` | `L1885.logic.749` | no bound result here | `.15.9` |
| `L1691.logic.671` | `L1886.logic.751` | no bound result here | `.15.9` |
| `L1691.logic.672` | `L1886.logic.752` | no bound result here | `.15.9` |
| `L1691.logic.673` | `L1886.logic.753` | no bound result here | `.15.9` |
| `L1691.logic.674` | `L1886.logic.754` | no bound result here | `.15.9` |
| `L1691.logic.675` | `L1886.logic.755` | no bound result here | `.15.9` |
| `L1691.cond.676` | `L1886.cond.756` | no bound result here | `.15.9` |
| `L1731.iter.693` | `L1926.iter.774` | no bound result here | `.15.9` |
| `L1732.iter.696` | `L1927.iter.777` | no bound result here | `.15.9` |
| `L1759.iter.710` | `L1954.iter.791` | no bound result here | `.15.9` |
| `L1759.iter.711` | `L1954.iter.792` | no bound result here | `.15.9` |
| `L1759.iter.712` | `L1954.iter.793` | no bound result here | `.15.9` |
| `L1759.guard.713` | `L1954.guard.794` | no bound result here | `.15.9` |
| `L1759.guard.714` | `L1954.guard.795` | no bound result here | `.15.9` |
| `L1759.guard.715` | `L1954.guard.796` | no bound result here | `.15.9` |
| `L1826.iter.729` | `L2021.iter.810` | no bound result here | `.15.9` |
| `L1827.iter.730` | `L2022.iter.811` | no bound result here | `.15.9` |
| `L1827.iter.731` | `L2022.iter.812` | no bound result here | `.15.9` |
| `L1827.iter.732` | `L2022.iter.813` | no bound result here | `.15.9` |
| `L1845.iter.739` | `L2040.iter.820` | no bound result here | `.15.9` |
| `L1857.iter.750` | `L2052.iter.831` | no bound result here | `.15.9` |
| `L1857.iter.751` | `L2052.iter.832` | no bound result here | `.15.9` |
| `L1858.iter.752` | `L2053.iter.833` | no bound result here | `.15.9` |
| `L1858.iter.753` | `L2053.iter.834` | no bound result here | `.15.9` |
| `L1858.iter.754` | `L2053.iter.835` | no bound result here | `.15.9` |
| `L1858.guard.755` | `L2053.guard.836` | no bound result here | `.15.9` |
| `L1858.guard.756` | `L2053.guard.837` | no bound result here | `.15.9` |
| `L1858.guard.757` | `L2053.guard.838` | no bound result here | `.15.9` |
| `L1890.iter.769` | `L2085.iter.850` | no bound result here | `.15.9` |
| `L1890.iter.770` | `L2085.iter.851` | no bound result here | `.15.9` |
| `L1890.iter.771` | `L2085.iter.852` | no bound result here | `.15.9` |
| `L1890.guard.772` | `L2085.guard.853` | no bound result here | `.15.9` |
| `L1890.guard.773` | `L2085.guard.854` | no bound result here | `.15.9` |
| `L1890.guard.774` | `L2085.guard.855` | no bound result here | `.15.9` |
| `L1890.guard.775` | `L2085.guard.856` | no bound result here | `.15.9` |
| `L1890.guard.776` | `L2085.guard.857` | no bound result here | `.15.9` |
| `L1890.guard.777` | `L2085.guard.858` | no bound result here | `.15.9` |
| `L1896.logic.781` | `L2091.logic.862` | no bound result here | `.15.9` |
| `L1897.eq.782` | `L2092.eq.863` | no bound result here | `.15.9` |
| `L1897.eq.783` | `L2092.eq.864` | no bound result here | `.15.9` |
| `L1897.eq.784` | `L2092.eq.865` | no bound result here | `.15.9` |
| `L1897.logic.786` | `L2092.logic.867` | no bound result here | `.15.9` |
| `L1897.logic.787` | `L2092.logic.868` | no bound result here | `.15.9` |
| `L1900.iter.788` | `L2095.iter.869` | no bound result here | `.15.9` |
| `L1900.iter.789` | `L2095.iter.870` | no bound result here | `.15.9` |
| `L1900.iter.790` | `L2095.iter.871` | no bound result here | `.15.9` |
| `L1901.iter.791` | `L2096.iter.872` | no bound result here | `.15.9` |
| `L1901.iter.793` | `L2096.iter.874` | no bound result here | `.15.9` |
| `L1910.cond.796` | `L2105.cond.877` | no bound result here | `.15.9` |
| `L1911.stmt.798` | `L2106.stmt.879` | no bound result here | `.15.9` |
| `L1941.iter.806` | `L2136.iter.888` | no bound result here | `.15.9` |
| `L1941.iter.808` | `L2136.iter.890` | no bound result here | `.15.9` |
| `L2021.iter.834` | `L2216.iter.916` | no bound result here | `.15.9` |
| `L2022.iter.837` | `L2217.iter.919` | no bound result here | `.15.9` |
| `L2022.guard.840` | `L2217.guard.922` | no bound result here | `.15.9` |
| `L2045.lit.853` | `L2240.lit.935` | no bound result here | `.15.9` |
| `L2062.iter.856` | `L2257.iter.938` | no bound result here | `.15.9` |
| `L2062.iter.857` | `L2257.iter.939` | no bound result here | `.15.9` |
| `L2062.guard.859` | `L2257.guard.941` | no bound result here | `.15.9` |
| `L2062.guard.860` | `L2257.guard.942` | no bound result here | `.15.9` |
| `L2065.bool.865` | `L2260.bool.947` | no bound result here | `.15.9` |
| `L2067.dir.867` | `L2262.dir.949` | no bound result here | `.15.9` |
| `L2079.stmt.876` | `L2274.stmt.958` | no bound result here | `.15.9` |
| `L2091.iter.880` | `L2286.iter.962` | no bound result here | `.15.9` |
| `L2108.logic.882` | `L2303.logic.964` | no bound result here | `.15.9` |
| `L2109.eq.884` | `L2304.eq.966` | no bound result here | `.15.9` |
| `L2109.eq.885` | `L2304.eq.967` | no bound result here | `.15.9` |
| `L2109.logic.886` | `L2304.logic.968` | no bound result here | `.15.9` |
| `L2109.logic.888` | `L2304.logic.970` | no bound result here | `.15.9` |
| `L2182.iter.897` | `L2377.iter.979` | no bound result here | `.15.9` |
| `L2182.guard.899` | `L2377.guard.981` | no bound result here | `.15.9` |
| `L2182.guard.900` | `L2377.guard.982` | no bound result here | `.15.9` |
| `L2190.logic.905` | `L2385.logic.987` | no bound result here | `.15.9` |
| `L2204.eq.912` | `L2399.eq.994` | no bound result here | `.15.9` |
| `L2204.eq.913` | `L2399.eq.995` | no bound result here | `.15.9` |
| `L2204.logic.914` | `L2399.logic.996` | no bound result here | `.15.9` |
| `L2204.logic.916` | `L2399.logic.998` | no bound result here | `.15.9` |
| `L2212.iter.920` | `L2407.iter.1002` | no bound result here | `.15.9` |
| `L2218.logic.922` | `L2413.logic.1004` | no bound result here | `.15.9` |
| `L2220.eq.925` | `L2415.eq.1007` | no bound result here | `.15.9` |
| `L2220.eq.926` | `L2415.eq.1008` | no bound result here | `.15.9` |
| `L2220.logic.928` | `L2415.logic.1010` | no bound result here | `.15.9` |
| `L2296.iter.936` | `L2491.iter.1018` | no bound result here | `.15.9` |
| `L2296.iter.937` | `L2491.iter.1019` | no bound result here | `.15.9` |
| `L2296.guard.939` | `L2491.guard.1021` | no bound result here | `.15.9` |
| `L2296.guard.940` | `L2491.guard.1022` | no bound result here | `.15.9` |
| `L2308.iter.947` | `L2503.iter.1029` | no bound result here | `.15.9` |
| `L2308.iter.948` | `L2503.iter.1030` | no bound result here | `.15.9` |
| `L2308.guard.950` | `L2503.guard.1032` | no bound result here | `.15.9` |
| `L2308.guard.951` | `L2503.guard.1033` | no bound result here | `.15.9` |
| `L2309.cond.952` | `L2504.cond.1034` | no bound result here | `.15.9` |
| `L2314.cond.962` | `L2509.cond.1044` | no bound result here | `.15.9` |
| `L2375.iter.986` | `L2570.iter.1068` | no bound result here | `.15.9` |
| `L2375.iter.987` | `L2570.iter.1069` | no bound result here | `.15.9` |
| `L2439.stmt.992` | `L2634.stmt.1074` | no bound result here | `.15.9` |
| `L1447.chain.1145` | `L1641.chain.1249` | no bound result here | `.15.9` |
| `L1447.chain.1146` | `L1641.chain.1250` | no bound result here | `.15.9` |
| `L1461.contains.1148` | `L1655.contains.1252` | no bound result here | `.15.9` |
| `L1461.contains.1150` | `L1655.contains.1254` | no bound result here | `.15.9` |
| `L1491.chain.1152` | `L1685.chain.1256` | no bound result here | `.15.9` |
| `L1553.chain.1154` | `L1747.chain.1258` | no bound result here | `.15.9` |
| `L1553.chain.1155` | `L1747.chain.1259` | no bound result here | `.15.9` |
| `L1557.contains.1158` | `L1751.contains.1262` | no bound result here | `.15.9` |
| `L1557.contains.1159` | `L1751.contains.1263` | no bound result here | `.15.9` |
| `L1557.contains.1160` | `L1751.contains.1264` | no bound result here | `.15.9` |
| `L1557.contains.1161` | `L1751.contains.1265` | no bound result here | `.15.9` |
| `L1566.chain.1162` | `L1760.chain.1266` | no bound result here | `.15.9` |
| `L1566.chain.1163` | `L1760.chain.1267` | no bound result here | `.15.9` |
| `L1578.chain.1165` | `L1772.chain.1269` | no bound result here | `.15.9` |
| `L1583.chain.1168` | `L1777.chain.1272` | no bound result here | `.15.9` |
| `L1587.chain.1170` | `L1781.chain.1274` | no bound result here | `.15.9` |
| `L1587.chain.1171` | `L1781.chain.1275` | no bound result here | `.15.9` |
| `L1591.chain.1172` | `L1785.chain.1276` | no bound result here | `.15.9` |
| `L1591.chain.1173` | `L1785.chain.1277` | no bound result here | `.15.9` |
| `L1591.chain.1174` | `L1785.chain.1278` | no bound result here | `.15.9` |
| `L1618.chain.1177` | `L1812.chain.1281` | no bound result here | `.15.9` |
| `L1618.chain.1178` | `L1812.chain.1282` | no bound result here | `.15.9` |
| `L1691.contains.1179` | `L1886.contains.1283` | no bound result here | `.15.9` |
| `L1691.contains.1181` | `L1886.contains.1285` | no bound result here | `.15.9` |
| `L1826.chain.1211` | `L2021.chain.1315` | no bound result here | `.15.9` |
| `L1858.chain.1226` | `L2053.chain.1330` | no bound result here | `.15.9` |
| `L1858.chain.1227` | `L2053.chain.1331` | no bound result here | `.15.9` |
| `L1875.chain.1235` | `L2070.chain.1339` | no bound result here | `.15.9` |
| `L1877.chain.1237` | `L2072.chain.1341` | no bound result here | `.15.9` |
| `L1877.chain.1238` | `L2072.chain.1342` | no bound result here | `.15.9` |
| `L1893.chain.1243` | `L2088.chain.1347` | no bound result here | `.15.9` |
| `L1981.chain.1247` | `L2176.chain.1351` | no bound result here | `.15.9` |
| `L1981.chain.1248` | `L2176.chain.1352` | no bound result here | `.15.9` |
| `L2023.contains.1256` | `L2218.contains.1360` | no bound result here | `.15.9` |
| `L2023.contains.1258` | `L2218.contains.1362` | no bound result here | `.15.9` |
| `L2054.chain.1259` | `L2249.chain.1363` | no bound result here | `.15.9` |
| `L2054.chain.1260` | `L2249.chain.1364` | no bound result here | `.15.9` |
| `L2068.chain.1262` | `L2263.chain.1366` | no bound result here | `.15.9` |
| `L2097.chain.1266` | `L2292.chain.1370` | no bound result here | `.15.9` |
| `L2105.chain.1267` | `L2300.chain.1371` | no bound result here | `.15.9` |
| `L2105.chain.1268` | `L2300.chain.1372` | no bound result here | `.15.9` |
| `L2106.chain.1269` | `L2301.chain.1373` | no bound result here | `.15.9` |
| `L2106.chain.1270` | `L2301.chain.1374` | no bound result here | `.15.9` |
| `L2106.chain.1271` | `L2301.chain.1375` | no bound result here | `.15.9` |
| `L2108.contains.1272` | `L2303.contains.1376` | no bound result here | `.15.9` |
| `L2127.chain.1276` | `L2322.chain.1380` | no bound result here | `.15.9` |
| `L2127.chain.1277` | `L2322.chain.1381` | no bound result here | `.15.9` |
| `L2145.chain.1278` | `L2340.chain.1382` | no bound result here | `.15.9` |
| `L2145.chain.1279` | `L2340.chain.1383` | no bound result here | `.15.9` |
| `L2154.chain.1280` | `L2349.chain.1384` | no bound result here | `.15.9` |
| `L2154.chain.1281` | `L2349.chain.1385` | no bound result here | `.15.9` |
| `L2165.chain.1282` | `L2360.chain.1386` | no bound result here | `.15.9` |
| `L2165.chain.1283` | `L2360.chain.1387` | no bound result here | `.15.9` |
| `L2202.chain.1293` | `L2397.chain.1397` | no bound result here | `.15.9` |
| `L2216.chain.1296` | `L2411.chain.1400` | no bound result here | `.15.9` |
| `L2218.contains.1299` | `L2413.contains.1403` | no bound result here | `.15.9` |
| `L2229.chain.1305` | `L2424.chain.1409` | no bound result here | `.15.9` |
| `L2229.chain.1306` | `L2424.chain.1410` | no bound result here | `.15.9` |
| `L2230.chain.1309` | `L2425.chain.1413` | no bound result here | `.15.9` |
| `L2233.chain.1310` | `L2428.chain.1414` | no bound result here | `.15.9` |
| `L2236.chain.1313` | `L2431.chain.1417` | no bound result here | `.15.9` |
| `L2237.chain.1314` | `L2432.chain.1418` | no bound result here | `.15.9` |
| `L2237.chain.1315` | `L2432.chain.1419` | no bound result here | `.15.9` |
| `L2237.contains.1319` | `L2432.contains.1423` | no bound result here | `.15.9` |
| `L2237.contains.1320` | `L2432.contains.1424` | no bound result here | `.15.9` |
| `L2249.chain.1321` | `L2444.chain.1425` | no bound result here | `.15.9` |
| `L2249.chain.1322` | `L2444.chain.1426` | no bound result here | `.15.9` |
| `L2250.chain.1323` | `L2445.chain.1427` | no bound result here | `.15.9` |
| `L2250.chain.1324` | `L2445.chain.1428` | no bound result here | `.15.9` |
| `L2250.chain.1325` | `L2445.chain.1429` | no bound result here | `.15.9` |
| `L2251.chain.1326` | `L2446.chain.1430` | no bound result here | `.15.9` |
| `L2251.chain.1327` | `L2446.chain.1431` | no bound result here | `.15.9` |
| `L2252.chain.1329` | `L2447.chain.1433` | no bound result here | `.15.9` |
| `L2252.chain.1330` | `L2447.chain.1434` | no bound result here | `.15.9` |
| `L2257.chain.1333` | `L2452.chain.1437` | no bound result here | `.15.9` |
| `L2257.chain.1334` | `L2452.chain.1438` | no bound result here | `.15.9` |
| `L2283.chain.1340` | `L2478.chain.1444` | no bound result here | `.15.9` |
| `L2283.chain.1341` | `L2478.chain.1445` | no bound result here | `.15.9` |
| `L2284.chain.1342` | `L2479.chain.1446` | no bound result here | `.15.9` |
| `L2284.chain.1343` | `L2479.chain.1447` | no bound result here | `.15.9` |
| `L2285.chain.1346` | `L2480.chain.1450` | no bound result here | `.15.9` |
| `L2291.chain.1347` | `L2486.chain.1451` | no bound result here | `.15.9` |
| `L2291.chain.1348` | `L2486.chain.1452` | no bound result here | `.15.9` |
| `L2292.chain.1349` | `L2487.chain.1453` | no bound result here | `.15.9` |
| `L2292.chain.1350` | `L2487.chain.1454` | no bound result here | `.15.9` |
| `L2292.chain.1351` | `L2487.chain.1455` | no bound result here | `.15.9` |
| `L2293.chain.1354` | `L2488.chain.1458` | no bound result here | `.15.9` |
| `L2294.chain.1359` | `L2489.chain.1463` | no bound result here | `.15.9` |
| `L2294.chain.1360` | `L2489.chain.1464` | no bound result here | `.15.9` |
| `L2344.chain.1365` | `L2539.chain.1469` | no bound result here | `.15.9` |
| `L2345.chain.1367` | `L2540.chain.1471` | no bound result here | `.15.9` |
| `L2345.chain.1368` | `L2540.chain.1472` | no bound result here | `.15.9` |
| `L2353.chain.1369` | `L2548.chain.1473` | no bound result here | `.15.9` |
| `L2353.chain.1370` | `L2548.chain.1474` | no bound result here | `.15.9` |
| `L2373.chain.1371` | `L2568.chain.1475` | no bound result here | `.15.9` |
| `L2373.chain.1372` | `L2568.chain.1476` | no bound result here | `.15.9` |
| `L2373.chain.1374` | `L2568.chain.1478` | no bound result here | `.15.9` |
| `L2374.chain.1375` | `L2569.chain.1479` | no bound result here | `.15.9` |
| `L2374.chain.1376` | `L2569.chain.1480` | no bound result here | `.15.9` |
| `L2374.chain.1377` | `L2569.chain.1481` | no bound result here | `.15.9` |
| `L2374.chain.1378` | `L2569.chain.1482` | no bound result here | `.15.9` |
| `L2374.chain.1379` | `L2569.chain.1483` | no bound result here | `.15.9` |
| `L2388.chain.1380` | `L2583.chain.1484` | no bound result here | `.15.9` |
| `L2388.chain.1381` | `L2583.chain.1485` | no bound result here | `.15.9` |
| `L2430.chain.1383` | `L2625.chain.1487` | no bound result here | `.15.9` |
| `L2431.chain.1384` | `L2626.chain.1488` | no bound result here | `.15.9` |
| `L2431.chain.1385` | `L2626.chain.1489` | no bound result here | `.15.9` |
| `L1435.clogic.1403` | `L1629.clogic.1509` | no bound result here | `.15.9` |
| `L1619.clogic.1405` | `L1813.clogic.1511` | no bound result here | `.15.9` |
| `L1750.clogic.1406` | `L1945.clogic.1512` | no bound result here | `.15.9` |

#### Initial `run` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L2449.cmp.994` | `L2644.cmp.1076` | no bound result here | `.15.10` |
| `L2449.cond.995` | `L2644.cond.1077` | no bound result here | `.15.10` |
| `L2450.int.996` | `L2645.int.1078` | no bound result here | `.15.10` |
| `L2450.int.997` | `L2645.int.1079` | no bound result here | `.15.10` |
| `L2454.stmt.998` | `L2649.stmt.1080` | no bound result here | `.15.10` |
| `L2458.int.999` | `L2653.int.1081` | no bound result here | `.15.10` |
| `L2458.int.1000` | `L2653.int.1082` | no bound result here | `.15.10` |
| `L2458.stmt.1001` | `L2653.stmt.1083` | no bound result here | `.15.10` |
| `L2478.iter.1002` | `L2673.iter.1084` | no bound result here | `.15.10` |
| `L2478.iter.1003` | `L2673.iter.1085` | no bound result here | `.15.10` |
| `L2478.iter.1004` | `L2673.iter.1086` | no bound result here | `.15.10` |
| `L2479.stmt.1005` | `L2674.stmt.1087` | no bound result here | `.15.10` |
| `L2488.stmt.1006` | `L2683.stmt.1088` | no bound result here | `.15.10` |
| `L2464.chain.1389` | `L2659.chain.1493` | no bound result here | `.15.10` |
| `L2464.chain.1390` | `L2659.chain.1494` | no bound result here | `.15.10` |

#### Initial `beside` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L526.chain.1037` | `L664.chain.1131` | no bound result here | `.15.11` |
| `L526.chain.1038` | `L664.chain.1132` | no bound result here | `.15.11` |
| `L526.chain.1039` | `L664.chain.1133` | no bound result here | `.15.11` |
| `L526.chain.1040` | `L664.chain.1134` | no bound result here | `.15.11` |
| `L526.chain.1041` | `L664.chain.1135` | no bound result here | `.15.11` |
| `L528.chain.1042` | `L666.chain.1136` | no bound result here | `.15.11` |
| `L529.chain.1044` | `L667.chain.1138` | no bound result here | `.15.11` |
| `L529.contains.1049` | `L667.contains.1143` | no bound result here | `.15.11` |


### Complete compiler-function measurement

Measurement `.15.2.1.1` (`2026-10-18`) actually attempts every current `compile`
candidate, using the complete controlling parent population. This function associates compiled
units with package targets, records dependency/environment inputs, normalizes command configuration,
and refuses unheld paths or a build that produced nothing of its own package.

| Observation | Actual result |
| --- | --- |
| Global production listing | 1526 candidates listed |
| Complete compiler-function selection | 83 attempted |
| Full parent population, original and restored copy | 153 tests, none ignored |
| Named finishing kills | 49 |
| Survivors requiring assessment | 28 |
| Compiler refusals | 6 |
| Equivalents / timeouts / unjudged | 0 / 0 / 0 |
| Native strict exit | 1 — criterion unmet |
| Unmutated copied baseline | passed |

The controller's exit 0 means the complete native result was audited. Native exit 1 remains an
unmet adequacy criterion. No survivor is classified as equivalent, and the six typed refusals
establish no coverage. No equivalent record applies to this selection; the file still contains
26 records whose unrelated validity is outside this run.

Every actual ID/raw replacement and ordered operator total binds the source
`069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba`.
The installed same-volume write guard passes its positive control and refuses all five forbidden
write operations before the run; the unmodified copied parent command passes. All 1063 held
regular inputs restore in the original and copied tree, with modes and Python/system hashes.
The restored copy retains the same full parent names; its borrowed index metadata does not prove
a copied Git index or submodule contents. The gitlink remains an opaque identity.

The native held activation is `21450e7ebb479e57f83ea79f013c4ab867397a9a`. This is not an
integration stamp or the later completion-document tree. Final source/operator/catalogue/reason
bytes, all 30 pinned source copies and Cargo/rustc metadata remain exact. Seven altered receipts
refuse: missing/duplicate row, foreign ID, wrong replacement or strict exit, and missing/foreign
finishing test name. The completion book/staged checks belong to a later document epoch.

| Operator | Attempted | Compiler-refused |
| --- | ---: | ---: |
| `stmt` | 5 | 2 |
| `iter` | 12 | 0 |
| `guard` | 12 | 0 |
| `cond` | 10 | 0 |
| `not` | 3 | 0 |
| `eq` | 3 | 0 |
| `int` | 2 | 0 |
| `logic` | 5 | 0 |
| `dir` | 5 | 0 |
| `lit` | 8 | 0 |
| `chain` | 10 | 0 |
| `contains` | 8 | 4 |

Each killed row below gives one actual finishing parent name; the native receipt retains the
complete name set. Survivor rows name their owned follow-up. Compiler-refused rows retain their
actual type diagnostic. Every selected candidate appears exactly once.

| Current ID | Operator | Actual outcome | Finishing parent, owned assessment or compiler diagnostic |
| --- | --- | --- | --- |
| `L783.stmt.269` | `stmt` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.270` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.271` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.272` | `iter` | killed | `trust::tests::a_harness_or_a_library_root_compiling_another_role_s_package_is_refused` |
| `L797.guard.273` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L797.guard.274` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L797.guard.275` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L799.cond.276` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L799.cond.277` | `cond` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L799.not.278` | `not` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L805.eq.279` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L806.int.280` | `int` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L806.int.281` | `int` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.eq.282` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.logic.283` | `logic` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.dir.284` | `dir` | survived | `.15.2.1.4` — pending assessment |
| `L814.lit.285` | `lit` | survived | `.15.2.1.4` — pending assessment |
| `L814.lit.286` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L834.stmt.287` | `stmt` | unviable | error[E0308]: match arms have incompatible types |
| `L841.iter.288` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.iter.289` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.iter.290` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.guard.291` | `guard` | killed | `trust::tests::a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused` |
| `L841.guard.292` | `guard` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L841.guard.293` | `guard` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L842.lit.294` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L842.lit.295` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L844.dir.296` | `dir` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.logic.297` | `logic` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.cond.298` | `cond` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.cond.299` | `cond` | killed | `trust::tests::a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused` |
| `L845.not.300` | `not` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.dir.301` | `dir` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L845.lit.302` | `lit` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L845.lit.303` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L857.stmt.304` | `stmt` | survived | `.15.2.1.5` — pending assessment |
| `L860.iter.305` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L860.iter.306` | `iter` | killed | `trust::tests::a_macro_assembling_a_path_attribute_is_refused` |
| `L860.iter.307` | `iter` | killed | `trust_generated::generator_tests::a_generator_build_failing_leaves_step_4_unable_to_judge_with_its_refusals_beside_it` |
| `L860.guard.308` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L860.guard.309` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L860.guard.310` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L866.cond.311` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L866.cond.312` | `cond` | survived | `.15.2.1.6` — pending assessment |
| `L866.dir.313` | `dir` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L871.stmt.314` | `stmt` | unviable | error[E0308]: if and else have incompatible types |
| `L877.stmt.315` | `stmt` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L885.iter.316` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.iter.317` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.iter.318` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.319` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.320` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.321` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.322` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.323` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.324` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.cond.325` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.cond.326` | `cond` | survived | `.15.2.1.7` — pending assessment |
| `L886.dir.327` | `dir` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.lit.328` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.lit.329` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.eq.330` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.cond.331` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.cond.332` | `cond` | survived | `.15.2.1.8` — pending assessment |
| `L911.not.333` | `not` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L784.chain.1156` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L784.chain.1157` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L804.chain.1158` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L804.chain.1159` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L822.chain.1160` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L822.chain.1161` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L826.chain.1162` | `chain` | survived | `.15.2.1.9` — pending assessment |
| `L826.chain.1163` | `chain` | survived | `.15.2.1.9` — pending assessment |
| `L845.contains.1164` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1165` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1166` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1167` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L886.contains.1168` | `contains` | unviable | error[E0599]: no method named first found for reference &String in the current scope |
| `L886.contains.1169` | `contains` | unviable | error[E0599]: no method named first found for reference &String in the current scope |
| `L886.contains.1170` | `contains` | unviable | error[E0599]: &String is not an iterator |
| `L886.contains.1171` | `contains` | unviable | error[E0599]: &String is not an iterator |
| `L911.chain.1172` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.chain.1173` | `chain` | killed | `trust::adequacy_tests::reader_role_tests::every_compiled_reader_is_judged_for_scripts_and_program_targets` |


All surviving obligations are scheduled, without a presumed cause:

| Assessment | Exact current survivors | What it must establish |
| --- | --- | --- |
| `.15.2.1.2` — Environment admission | `L841.guard.292`, `L841.guard.293`, `L845.dir.301`, `L845.lit.302` | foreign environment record positions and exact CARGO_PKG_ namespace |
| `.15.2.1.3` — Compiler-command filtering | `L797.guard.273`, `L797.guard.274`, `L797.guard.275`, `L799.cond.277` | non-rustc command reachability, actual producer/caller domain and command positions |
| `.15.2.1.4` — Artifact suffix extraction | `L814.dir.284`, `L814.lit.285` | exact extra-filename option and prefix/substring boundaries |
| `.15.2.1.5` — Environment-loop tail | `L857.stmt.304` | final continue control flow and complete remaining loop body/drop order |
| `.15.2.1.6` — Dependency absolute paths | `L860.guard.308`, `L860.guard.309`, `L860.guard.310`, `L866.cond.312` | absolute dependency refusals at every position and supported raw/canonical path geometry |
| `.15.2.1.7` — Configuration absolute paths | `L885.iter.316`, `L885.iter.317`, `L885.iter.318`, `L885.guard.319`, `L885.guard.320`, `L885.guard.321`, `L886.logic.322`, `L886.logic.323`, `L886.logic.324`, `L886.cond.326` | retained absolute configuration reachability, both rejection forms and every position |
| `.15.2.1.8` — Own-package completion | `L911.cond.332` | successful-build own-package completion and actual target/caller reachability |
| `.15.2.1.9` — Unit owner cardinality | `L826.chain.1162`, `L826.chain.1163` | target metadata lookup, duplicate owners and exact reachable collection cardinality |

The environment parents below resolve those four admission obligations. Command filtering is
the next assessment. Each follow-up uses actual supported callers and independent expectations,
then add finishing parent controls, repair an original defect if found, or establish a precise
source/domain proof. Finite examples alone do not establish equivalence. Each resulting slice is
committed separately before complete current-region replay; other helper/file scopes stay separate.

Reproduce the full parent population with:

```bash
cargo test -q -p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests
```

The exact selected native command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail --equivalents xtask/trust.equivalents \
  --only L783.stmt.269 \
  --only L797.iter.270 \
  --only L797.iter.271 \
  --only L797.iter.272 \
  --only L797.guard.273 \
  --only L797.guard.274 \
  --only L797.guard.275 \
  --only L799.cond.276 \
  --only L799.cond.277 \
  --only L799.not.278 \
  --only L805.eq.279 \
  --only L806.int.280 \
  --only L806.int.281 \
  --only L814.eq.282 \
  --only L814.logic.283 \
  --only L814.dir.284 \
  --only L814.lit.285 \
  --only L814.lit.286 \
  --only L834.stmt.287 \
  --only L841.iter.288 \
  --only L841.iter.289 \
  --only L841.iter.290 \
  --only L841.guard.291 \
  --only L841.guard.292 \
  --only L841.guard.293 \
  --only L842.lit.294 \
  --only L842.lit.295 \
  --only L844.dir.296 \
  --only L845.logic.297 \
  --only L845.cond.298 \
  --only L845.cond.299 \
  --only L845.not.300 \
  --only L845.dir.301 \
  --only L845.lit.302 \
  --only L845.lit.303 \
  --only L857.stmt.304 \
  --only L860.iter.305 \
  --only L860.iter.306 \
  --only L860.iter.307 \
  --only L860.guard.308 \
  --only L860.guard.309 \
  --only L860.guard.310 \
  --only L866.cond.311 \
  --only L866.cond.312 \
  --only L866.dir.313 \
  --only L871.stmt.314 \
  --only L877.stmt.315 \
  --only L885.iter.316 \
  --only L885.iter.317 \
  --only L885.iter.318 \
  --only L885.guard.319 \
  --only L885.guard.320 \
  --only L885.guard.321 \
  --only L886.logic.322 \
  --only L886.logic.323 \
  --only L886.logic.324 \
  --only L886.cond.325 \
  --only L886.cond.326 \
  --only L886.dir.327 \
  --only L886.lit.328 \
  --only L886.lit.329 \
  --only L911.eq.330 \
  --only L911.cond.331 \
  --only L911.cond.332 \
  --only L911.not.333 \
  --only L784.chain.1156 \
  --only L784.chain.1157 \
  --only L804.chain.1158 \
  --only L804.chain.1159 \
  --only L822.chain.1160 \
  --only L822.chain.1161 \
  --only L826.chain.1162 \
  --only L826.chain.1163 \
  --only L845.contains.1164 \
  --only L845.contains.1165 \
  --only L845.contains.1166 \
  --only L845.contains.1167 \
  --only L886.contains.1168 \
  --only L886.contains.1169 \
  --only L886.contains.1170 \
  --only L886.contains.1171 \
  --only L911.chain.1172 \
  --only L911.chain.1173
```

The guard and source activation described above remain part of the recorded execution. These
results establish neither final regional adequacy nor whole-file adequacy, remote CI or physical-board
execution. The complete measured result is committed before any subsequent assessment changes.


### Compiler environment admission regressions

The `2026-10-18` regression slice (`.15.2.1.2.1`) resolves the four measured environment
survivors with two permanent parents. Production already refuses these inputs; the correction is
to the test coverage, with production, earlier tests, mutation operators and equivalent reasons
unchanged.

A compiled environment record must use the exact `CARGO_PKG_` prefix or one of the explicit
allowed names, and a foreign record is refused at any position. The old single-`HOME` fixture's
foreign record is the final physical dependency line. A check narrowed to the last line or the two
ends therefore still passes that fixture. The new position parent holds a real foreign record
strictly inside the emitted dependency file, before an allowed package record.

| Measured survivor | Required finishing parent | Actual new verdict |
| --- | --- | --- |
| `L841.guard.292` | `trust::tests::compiled_environment_refusals_cover_nonterminal_records` | killed |
| `L841.guard.293` | `trust::tests::compiled_environment_refusals_cover_nonterminal_records` | killed |
| `L845.dir.301` | `trust::tests::compiled_environment_refuses_package_namespace_lookalikes` | killed |
| `L845.lit.302` | `trust::tests::compiled_environment_refuses_package_namespace_lookalikes` | killed |

The namespace parent uses two foreign names: `CARGO_PKGNAME`, which lacks the required underscore,
and `PREFIX_CARGO_PKG_NAME`, which merely contains the prefix. The expected refusal names the
literal foreign variable. Proper package variables remain accepted, including the recorded absence
of `CARGO_PKG_NOT_DEFINED`. For example, these are valid Rust reads but remain foreign to the
inventory's allowed namespace:

```rust
pub const MISSING_SEPARATOR: Option<&str> = option_env!("CARGO_PKGNAME");
pub const EMBEDDED_PREFIX: Option<&str> = option_env!("PREFIX_CARGO_PKG_NAME");
```

Both parents use actual inventory builds. Allowed controls write an inventory; foreign reads
produce the intended refusal and leave no `trust-dependencies.json`; restoring the allowed source
writes successfully again. The position control independently reads Cargo/rustc's actual dependency
file and asserts the literal records' physical ordering. The namespace control checks the recorded
allowed value and absence, rather than only accepting a successful command.

The selected native command uses the full **155 tests**, not just the new parents. Its unmodified
copied baseline passes, and all four exact edits have their required finishing names with strict
exit 0. Every original 153 parent remains present. All 1063 held original/copied regular inputs
restore, including modes and Python/system hashes; restored copied names agree. Seven false
bindings refuse, including substitution of another valid but unintended parent name.

Only the two Rust test functions are added. Removing them reconstructs every previous Rust byte;
the complete production listing remains 1526 with identical raw edits. The equivalent file still
has 26 records, none applicable here; all 30 pinned source copies and tool metadata remain exact.
The held native activation is `b423bf43fe5b28c48c01ea3101260108acd07df5`; later book/staged
completion checks and the final focused verification belong to their own epoch. Copied index
metadata is borrowed, the gitlink is opaque, and no executable-digest or new integration claim follows.

This resolves four particular obligations from the earlier 83-edit measurement. The other 24
survivors remain owned at that earlier measurement's epoch; no fresh whole-region count is inferred.
Command-filtering assessment `.15.2.1.3` is next, before complete current compiler replay and
the remaining helpers/files.


### Compiler command provenance assessment

Assessment `.15.2.1.3.1` (`2026-10-18`) finds a supported source of non-compiler
records and an original phantom-unit refusal. The four command-filter survivors remain
unclassified; JSON-message mode does not establish their equivalence.

The current callers use `build` or `test --no-run --test`, the fixed
`--message-format=json` and single `-v`, a cleared allowlisted environment, and prebuild
target-kind admission. A first real deprecated-note probe puts the warning's forged
`Running` line on stdout in JSON mode. The wider plain-mode control puts it on stderr,
where the compiled current parser reads it as an extra command. That first result cannot
be generalized to every compiler diagnostic.

Actual multiline assembler warnings provide the counterexample. The pinned compiler can emit
raw diagnostic text on stderr even in the fixed JSON mode. Cargo's pinned
`on_stderr_line_inner` forwards a non-JSON line unchanged; the current `cargo_commands`
then reads an embedded `Running` line as a command. The source census records ten matching
call sites across seven Cargo production modules for its two exact patterns. Those real
command writers do not account for every parsed record: diagnostic data is another route.
Seven newly copied source inputs verify against the official pinned archive and existing cache;
all 37 source identities and independent system hashes agree.

| Actual producer probe | Build/parser outcome |
| --- | --- |
| Deprecated note, fixed JSON mode | success; warning on stdout, only the real rustc record |
| Same note, wider plain mode | success; an extra non-compiler record parsed from stderr |
| Assembler warning with literal backslash-n | success; no extra parsed command |
| Assembler print directive | success; its output is on stdout |
| Assembler warning with a physical line break | success; two extra non-compiler records |
| Same physical warning, text naming rustc | success; two extra records naming rustc |

All these rows describe the installed pinned toolchain. The inventory boundary is checked
separately with a maintained two-root fixture whose roots compile a common dependency.
Ordinary source writes an inventory. The warning macro is refused by the catalog unless its
exact source extent has a reviewed admission; the following comparison includes that admission.

| Admitted warning payload | Actual inventory |
| --- | --- |
| `Running not-a-compiler`, with the command delimiters | written |
| `Running rustc`, with the same delimiters | refused for empty-name phantom units in both gen and chk |

The latter is an **owned P2 defect**, not an unadmitted assembly-site refusal. Real compilation
succeeds and the exact macro extent is admitted, yet warning text is interpreted as a compilation
unit. The false diagnostic says that each root compiles an empty-named crate with no target owner.
No inventory is left after refusal. The admitted non-compiler case is also a concrete supported
witness for testing the filter; it is not evidence for equivalence.

The compiler-warning source used by the second inventory case is:

```rust
core::arch::global_asm!(r#".warning "note
Running `rustc`""#);
pub fn f() {}
```

Its exact macro-extent admission is:

```lisp
(defadmit (file "crates/common/src/lib.rs") (rule "global_asm")
  (sha256 "8fe9c284e0ccf464eee42de4f7d905be99f720fff309738ff5e35117168164ae") (reason "a fixture"))
```

Changing the payload to `not-a-compiler` also requires updating the extent hash. Both cases
use the same roots, package graph, allowed configuration and JSON-mode build path. The observation
parent temporarily extends only the maintained restored sweep copy and uses its normal exact
outer-root configuration seam; no original Rust source changes. The copied Rust file restores
byte-for-byte, and all 1063 original regular inputs restore with modes and Python/system hashes.
Six altered observations refuse, including a false successful inventory and a wrong refusal reason.

The following slice adds permanent finishing controls for the four measured filter edits
(`.15.2.1.3.2`). The phantom-unit defect remains scheduled immediately afterward under
`.15.2.1.3.3`: define and verify compiler-invocation provenance that keeps warning data separate,
preserve admitted-source behavior, implement the repair, then rebind mutation/proof scopes and
run the full milestone gate. The command-filter cohort remains open until both tracks finish.
No new parent or equivalent entry is added by this assessment, and no fresh whole-region result,
remote CI or universal-host behavior is claimed.


### Admitted nonterminal warning regression parent

The `2026-10-18` slice `.15.2.1.3.2` adds one permanent parent:
`trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records`.
Two ordinary roots compile a common dependency. An ordinary control writes an inventory;
then the dependency gains a reviewed exact-extent assembly macro with two explicit warning
records, naming `not-a-compiler-first` and `not-a-compiler-last`. The unchanged production
filter ignores this warning data, and the inventory still writes. Restoring ordinary source
and its roots removes the admission and writes again.

The exact warning-source macro is:

```rust
core::arch::global_asm!(r#".warning "first
Running `not-a-compiler-first`"
.warning "last
Running `not-a-compiler-last`""#);
```

The producer control uses an actual cold Cargo build with the current environment constructor
and the fixed JSON/single-verbose arguments. It independently observes a genuine common-crate
compilation before the first warning and a later warning after it. One warning is therefore
nonterminal without depending on when the consumer crate starts compiling. The initial stricter
fixture required the dependency warning before that consumer's command. Actual output put the
consumer command first: pipelined dependency metadata can enable consumer work before final
assembler diagnostics. That experimental assertion failed and was corrected before verification;
the original failed receipt remains retained.

| Exact edit | Required actual finishing parent | New native outcome |
| --- | --- | --- |
| `L797.guard.273` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L797.guard.274` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L797.guard.275` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L799.cond.277` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |

The native command retains the full **156 tests**, including all earlier 155 parent names,
and freshly kills all four exact edits with the intended new parent and strict exit 0.
All 1066 held regular inputs restore in the original and copied tree, with modes and
Python/system hashes; restored copied names agree. Seven falsified receipts refuse, including
another valid but unintended finishing name.

Removing only this new Rust function reconstructs every earlier Rust byte. Production,
the 1526-candidate listing/raw edits, mutation operators, catalogue and 26 equivalent records
remain unchanged. All 37 pinned source copies and tool metadata revalidate. Native held
activation `ff8c42813c93968b07c03684f11f6713a3bf0f79` is distinct from later completion
documents and final staged focused verification; copied index metadata is borrowed and the
gitlink remains opaque.

These four particular filter obligations are resolved. Together with the earlier four environment
controls, eight individual obligations from the initial compiler measurement now have finishing
parents. The other 20 initial survivors remain owned at that measurement's epoch; this is not
a newly measured full-region result.

At this parent slice's close, the P2 phantom-unit defect remained open.
Its next action was the concrete producer/consumer contract and prototype, `.15.2.1.3.3.1`,
followed by bounded repair and complete current source/premise/mutation/integration verification.
The command-filter cohort stays open until that defect is fixed.


### Compiler provenance contract and prototype

The `2026-10-18` slice `.15.2.1.3.3.1` verifies a repair mechanism for the admitted-warning
false refusal above. **At this prototype epoch, production inventory was unchanged.** The prototype records compiler
invocations separately, through Cargo's published
[`RUSTC_WRAPPER` interface](https://doc.rust-lang.org/cargo/reference/environment-variables.html),
whose first argument is the compiler followed by its arguments. The global wrapper observes dependency
compilations too. The workspace-only wrapper is not used. Both interfaces and the installed public
compiler documentation are tied to the [pinned toolchain](ledger.md#rust-toolchain).

A compiler warning can contain any of the following data without becoming a compilation unit:

```text
Running `not-a-compiler`
Running `rustc`
Running `rustc --crate-name phantom phantom.rs --out-dir bogus`
```

The observer receives arguments from Cargo's process invocation, independently of those strings.
It forwards the arguments to the real compiler, with stdout and stderr inherited, and writes its own
completed record into the build's private directory. The consumer reads that directory only after
Cargo succeeds. It does not recover units from warning text if provenance is missing or malformed.

The proposed production boundary is:

| Step | Contract |
| --- | --- |
| Prepare the instrument | Compile the tracked embedded std-only observer with the pinned compiler once for this inventory invocation; verify its installed bytes in each role's target directory. |
| Begin a role build | Create a fresh same-volume session for the root, harness or generator. Preserve the existing cleared base environment and fixed compiler; add explicit instrument settings only. |
| Run the compiler | Reserve a partial record with exclusive creation before execution. Forward native argv and working directory, remove the wrapper and private settings from the compiler environment, and preserve diagnostic streams. |
| Complete the record | Retain session, role, invocation identity, working directory, complete argv bytes and exit/signal outcome. Publish without replacing another record. A record failure fails the build. |
| Read provenance | Require the exact schema and current identity, regular non-symlink files, complete byte framing and successful status. Empty, stale, partial, foreign-role or unknown records refuse. |
| Separate queries | Recognize the pinned version and target-information queries; they yield no units. Unknown shapes refuse. Each real compilation remains subject to metadata ownership, dependency files, source admission, environment and configuration checks. |
| Select the artifact | Bind the role target to an observed compilation. Ask pinned rustc for link-output names using empty input and observed naming options, then check basenames, target containment, regular output files and hashes. Arbitrary Cargo stdout does not select the artifact. |
| Finish or fail | Sort units as before. Keep instrument/session paths out of configuration and shared-item identities. On failure leave no inventory, retain useful diagnostics and partial evidence, wait for children, and use a new session next time. |

The [canonical inventory record](../../../docs/specs/trust/decision_trust-inventory.md#compiler-provenance-repair-contract-and-prototype)
keeps the same contract. Implementation remains divided into producer/unit collection, artifact
binding, compatibility/refusal controls and complete current mutation/premise/integration verification.
The first implementation slice must include a real public inventory red/green case for the P2 defect.
Independent review and final F30 adequacy remain separate obligations.

#### Replaying the prototype

The instrument and both native source fixtures are tracked:

```bash
python3 -I -B scripts/trust_compiler_provenance_probe.py \
  --out target/compiler-provenance-probe
```

Choose a new output directory on each run. It must be relative to the repository under `target/` and
resolve on the repository volume. The instrument derives the root from its own location, prepares
project-local temporary/cache directories, compiles the actual current environment constructor and
uses the pinned toolchain. The OS C compiler and installed toolchain/public documentation are read-only
external dependencies. No submodule implementation, other checkout or old evaluation case is used.
All build workspaces, Cargo homes, traces, binaries and receipts stay under the chosen output.

The prototype format is `archogen-compiler-provenance-prototype/1`. It encodes native argument and
working-directory bytes as hexadecimal; a record includes the session, role, PID, compiler exit code
and signal. Raw observations can therefore retain the actual runtime spelling without becoming
project lookup paths. Production configuration remains repository-relative after normalization.
This is an internal prototype format, not a new public inventory schema or a production repair.

#### Actual role and payload matrix

The dated run completes **20 actual Cargo builds**, all with release/locked/offline/no-default-features,
JSON messages and single verbosity, using the actual base environment plus explicit instrument settings.
Each cell gives the independently expected real compilation units; every build succeeds.

| Common-dependency source | Binary root | Library root | Comparison harness | Generator |
| --- | --- | --- | --- | --- |
| Ordinary | common, app | common, api | common, api, adapter | common, generator |
| Warning naming not-a-compiler | common, app | common, api | common, api, adapter | common, generator |
| Warning naming rustc | common, app | common, api | common, api, adapter | common, generator |
| Full command-shaped warning | common, app | common, api | common, api, adapter | common, generator |
| Escaped JSON-looking stdout | common, app | common, api | common, api, adapter | common, generator |

Each build also observes the pinned producer's three query invocations: version twice and target
information once. They produce no units. Every real unit agrees with a Cargo artifact report.
These direct Cargo fixtures exercise the mechanism; they do not replace the separately reproduced
public inventory admission/refusal case above or establish repaired production behavior.

The stdout probe uses an assembler print directive. Its output retains backslashes before quotes,
so it is JSON-looking text rather than a valid artifact object. Attempts using adjacent or single
quotes fail in the assembler. No artifact-report exploit was demonstrated. The contract binds
artifacts to real observed outputs so their selection does not depend on guessing which diagnostic
text can resemble a Cargo record.

For every observed compilation, a separate empty-input naming query selects only observed crate
name/type, test mode, target if present and extra-filename. All **45 naming queries** agree with actual
Cargo artifact bytes. A complete before/after file-hash census of each build directory is unchanged.
This uses rustc's documented
[`file-names` print option](https://doc.rust-lang.org/rustc/command-line-arguments/print-options.html#file-names),
checked against the installed pinned documentation and actual outputs. No source macro is evaluated
by these empty-input queries. The future artifact slice must preserve containment, role ownership and
existing supported path geometry, including cases beyond this finite fixture matrix.

#### Transport, failure and concurrency controls

An independently compiled C receiver writes its actual native argv as NUL-separated bytes. The
observer's decoded record matches it exactly for empty arguments, spaces, apostrophes, double quotes,
LF/CR, backslashes, Unicode and non-UTF8 Unix bytes. The receiver also checks the private fields and
wrapper setting are absent. Its exit7 and exact stdout/stderr are preserved. A signal15 case exits143
and records the signal; a compiler spawn failure exits2 and leaves a refused partial record. A missing
trace-directory setup exits2 before the requested receiver can execute.

Eight concurrent native writers retain distinct, complete records with exact independent argv
receipts. No record is replaced or left partial in that successful concurrency case. The consumer's
following controls each alter a separate copy of valid primary records and all refuse:

| Alteration | Required rejection |
| --- | --- |
| Missing record population | missing provenance |
| Partial file | incomplete/unexpected record |
| Old session | session identity |
| Other role | role identity |
| Other schema | schema identity |
| Extra field | record schema |
| Duplicate JSON field | duplicate JSON field |
| Invalid hexadecimal | byte framing/alphabet |
| Filename/PID disagreement | invocation identity |
| Failed compiler status | compiler status |
| Other compiler | compiler selection |
| Other working directory | compiler cwd |
| Unknown query | unknown query shape |
| Symbolic-link record | incomplete/unexpected record |

These **14 rejecting controls** and the native error cases exercise this prototype, not all future
production failure branches. A trusted host/toolchain and preflight's prohibition on executing build
scripts/procedural macros remain premises; the record is not a security boundary against a host that
can rewrite both instrument and evidence. Source admissions and all input/file/environment rules
remain in force.

#### Evidence and remaining work

The verified receipt is `target/compiler-provenance-prototype/verified/prototype-results.json`,
SHA-256 `d247612c4726520bed465797a60d3eb2728a9538025ad5934f9aa552eca2937f`.
The producer is durable: the replay command above builds the tracked instrument and fixtures.
The receipt binds their hashes, current production sources, native tool identities and installed
public documentation. The tracked supplemental auditor rechecks raw records, literal role sets and
actual artifact hashes, and rejects altered summaries:

```bash
python3 -I -B scripts/fixtures/compiler_provenance_audit.py \
  --input target/compiler-provenance-probe/prototype-results.json
```

The private runtime paths in raw argv are observations, not relocation inputs.

All 1067 original regular inputs restore at the activation epoch, including modes and independent
Python/system hashes. Later completion documents are a separate epoch. The original trust production
and tests, current mutation operators/catalogue and equivalent-reason file remain unchanged.
Exploratory failures are retained: an overly narrow emit-mode expectation omitted binary compilation;
an assembler template needed brace escaping; escaped stdout was initially mistaken for valid JSON.
The durable run uses the corrected expectations and records the narrower actual conclusions.

Next, `.15.2.1.3.3.2.1` implements actual invocation collection with an admitted-warning public
red/green regression. `.2.2` binds role artifacts to observed output; `.2.3` holds compatible paths,
roles and refusal/recovery behavior. Affected old helpers and equivalent premises must be explicitly
retired or revalidated as production changes, with their historical outcomes preserved.
`.3.3.3` then rebinds all affected sources, current raw mutations, callers and producer premises,
resolves new survivors and runs full milestone integration. The command-filter cohort and F30 stay
open until the repair and those gates finish.


### Production compiler invocation collection repair

The `2026-10-18` slice `.15.2.1.3.3.2.1` replaces production command collection with the
separate observer protocol verified above. The admitted warning that previously invented empty-name
units now produces a valid inventory. Root and dependency units come from actual compiler process
arguments. Diagnostic text cannot add a unit to this collection.

The regression uses two roots, each depending on `common`. An ordinary inventory establishes the
configuration. The common source then receives an exact-extent admission for each warning payload:

```text
Running `not-a-compiler`
Running `rustc`
Running `rustc --crate-name phantom phantom.rs --out-dir bogus`
```

Against the original production code, the bare `rustc` payload fails with empty-name phantom-unit
refusals for both roles. After the repair, every payload writes an inventory. Each root has exactly
its own package and the fixture package value `"crates/common"`, and its complete unit configurations agree with the ordinary
control. Restoring ordinary source and removing the admission succeeds again. Source-content hashes
still reflect the source that was actually compiled; the configuration comparison does not erase them.

#### What the production observer does

The inventory builds its tracked, embedded, std-only observer with the pinned compiler once per
inventory operation. For each root, comparison harness or generator compilation, it installs and
byte-verifies a copy under that role's Cargo target directory and creates a fresh records directory.
The base cleared environment is preserved. Only these explicit instrument settings are added:

```text
RUSTC_WRAPPER
ARCHOGEN_COMPILER_RECORDS
ARCHOGEN_COMPILER_SESSION
ARCHOGEN_COMPILER_ROLE
```

Cargo's published wrapper interface passes the compiler and its actual native arguments to this
observer. The compiler selection remains the pinned literal `rustc`. The observer reserves an
exclusive partial file, forwards the immutable argument tail to the real compiler, inherits its
working directory and diagnostic streams, and removes those private settings from the compiler's
environment. After the compiler finishes, it syncs the record, publishes it without replacement,
and removes the partial name. An observer failure makes the build fail.

The internal record schema is `archogen-compiler-provenance/1`. The consumer requires canonical JSON,
which also rejects duplicate fields, and validates every field:

| Field | Meaning and check |
| --- | --- |
| `schema` | Exact protocol version. |
| `session_hex` | The fresh current session identity; historical records cannot substitute. |
| `role_hex` | The current root, harness or generator build identity. |
| `cwd_hex` | The expected canonical working directory. |
| `pid` | A positive invocation identity agreeing with the record filename. |
| `argv_hex` | Complete native argument bytes, including the fixed compiler program. |
| `exit` | Exactly successful compiler status for an accepted record. |
| `signal` | No signal for an accepted record. |

The hexadecimal wire representation preserves bytes. The current JSON inventory requires interpreted
arguments and paths to be valid UTF-8; unsupported bytes produce an explicit inability to judge,
without lossy argument conversion. Raw runtime paths in trace evidence are observations, not project
lookup paths or persisted configuration. Session and observer paths do not enter unit configuration
or shared-item identity.

The consumer rejects missing, partial, linked, stale, foreign-role, malformed, failed or unknown
records. Version and target-information queries from the pinned producer yield no units. Real
compilations retain complete arguments and still pass the existing metadata ownership, dependency
file, source admission, environment and configuration checks. There is no diagnostic-parser fallback.
The unit sequence retains its deterministic sort.

This slice repairs **unit collection**. Role artifacts still use Cargo's artifact report; direct
observed-output binding is `.3.3.2.2`. The complete compatibility/failure matrix and whole-source
mutation/integration checkpoint remain separately owned. No full F30 adequacy or independent
review closure follows from this bounded repair.

#### Independent forwarding and failure controls

The production observer itself is checked with an independently compiled C receiver. That receiver
writes NUL-separated native argv. Empty arguments, spaces, quotes, LF/CR, backslashes, Unicode,
non-UTF8 bytes and flag-shaped arguments all arrive byte-for-byte, and the record agrees with the
receiver. Exit7 and exact stdout/stderr are preserved. The receiver verifies that the private fields
and wrapper setting are absent.

A real compiler source also asserts that every private setting is absent. Its successful compile
has the exact expected argument vector. A genuine compiler failure retains its diagnostic and is
rejected by the record consumer; a missing compiler leaves a refused partial record; a fresh session
recovers. A standalone observer test checks literal byte encodings and runs through the production
parent. Separate actual query records exercise missing/partial/session/role/cwd/extra-field/duplicate/
status/identity/hex/compiler/query/link/type/object refusals. Every valid copied record is accepted
before its individual fault is applied.

An initial identity test used PID zero while expecting a filename-mismatch diagnostic. Zero is
already an invalid identity, so the fixture was corrected to a different positive PID. The initial
failure remains in the evidence. This corrects the test's branch selection, not production behavior.

#### Bounded diagnostic fixtures and baseline evidence

Observer compilation increased the cost of each inventory operation. The old output-path diagnostic
parent ran its entire matrix under one 30-second bound and failed in the unmodified sweep copy.
An exact copied replay reproduced the timeout: the overall suite ended with 158 passes and one
failure. That is a failed baseline, not a mutation kill or an adequacy result.

The fixture now has three independently bounded parents, retaining the original first name:

| Parent suffix | Path cases |
| --- | --- |
| `cargo_failure_diagnostics_ignore_multiline_output_paths` | ordinary; newline plus erroneous-path; newline plus error-path |
| `_messages` | newline error decoy; newline Cargo-looking error; carriage-return/newline decoy |
| `_quoted_and_unicode` | apostrophe/backslash/newline path; Unicode/newline path |

All original path bytes and their order are retained. Each group uses the same success,
compiler-failure/no-inventory and recovery phases. An audit checks the original assertion body,
allowing only the fixture-name parameter and its exact formatting reflow. The bound stays at
30 seconds. The direct split run passes with observed parent times about 6.6, 9.7 and 9.7 seconds;
these timings describe that run, not every host or future load.

The failed sweep also exposed an owned tool-evidence gap: its baseline diagnostic omitted the raw
streams and finishing name. The exact copied test command supplied the diagnosis here.
`.3.3.3.0` must preserve native failed-baseline evidence before the full source sweeps. No automatic
retry or changed passing classification is used to hide the failed attempt.

#### Source, catalogue and reason continuity

The old command splitter and framer now exist only in test fixtures that observe historical verbose
output. Production never calls them. Five catalogue mutations targeting those removed production
paths are explicitly retired. The fixed-compiler mutation remains, with its reason updated for the
new program-identity check. The catalogue's exact-source uniqueness control passes.

Four delimiter reason records move to `xtask/trust.retired-equivalents`, a historical test fixture.
Compiled controls still exercise their exact five old helper edits and wider-domain counterexamples,
and require those keys to be absent from the active file. Historical native outcomes remain dated
records; they are not relabelled as new kills or equivalents.

The existing normalization and Cargo diagnostic helper bodies remain byte-for-byte unchanged.
Three iterator replacement counter names are rebound to current source lines. The producer-dependent
normalization premises are explicitly revalidated: the observer forwards actual immutable argv,
adds no compiler flags, and removes only instrument settings. The existing constructor, fixed
compiler, configuration/preflight and argument consumers retain their contracts. New source guards
include the observer, controller and base environment, with altered-premise rejections. All 34
operator and compiled proof controls pass on this source.

The fresh native epoch holds **162 tests**, including every previous parent plus the new
protocol and split-fixture parents. Its complete source listings contain 1433 trust, 124 controller
and 16 observer candidates; the bounded commands select only the following edits. Source listings
are discovery counts, not attempted coverage. Each selected protocol edit is killed by the intended
`trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs`
parent; other finishing names are retained in the raw results. All active trust reason records are
included in their separate replay.

| Source | Exact current edit | Native result |
| --- | --- | --- |
| `trust_compiler.rs` | `L216.cond.64` | killed |
| `trust_compiler.rs` | `L273.cond.97` | killed |
| `trust_compiler_observer.rs` | `L60.stmt.7` | killed |
| `trust_compiler_observer.rs` | `L64.stmt.11` | killed |
| `trust.rs` | `L345.iter.59` | equivalent |
| `trust.rs` | `L455.bool.83` | equivalent |
| `trust.rs` | `L456.bool.84` | equivalent |
| `trust.rs` | `L457.bool.85` | equivalent |
| `trust.rs` | `L458.bool.86` | equivalent |
| `trust.rs` | `L459.bool.87` | equivalent |
| `trust.rs` | `L460.bool.88` | equivalent |
| `trust.rs` | `L461.bool.89` | equivalent |
| `trust.rs` | `L462.bool.90` | equivalent |
| `trust.rs` | `L463.bool.91` | equivalent |
| `trust.rs` | `L464.bool.92` | equivalent |
| `trust.rs` | `L497.dir.112` | equivalent |
| `trust.rs` | `L497.lit.113` | equivalent |
| `trust.rs` | `L498.dir.116` | equivalent |
| `trust.rs` | `L498.lit.117` | equivalent |
| `trust.rs` | `L499.dir.120` | equivalent |
| `trust.rs` | `L499.lit.121` | equivalent |
| `trust.rs` | `L514.cmp.139` | equivalent |
| `trust.rs` | `L585.iter.158` | equivalent |
| `trust.rs` | `L587.iter.161` | equivalent |
| `trust.rs` | `L590.cond.168` | equivalent |
| `trust.rs` | `L617.cmp.181` | equivalent |

The protocol groups each finish with two kills and strict exit 0. The reason group finishes with
22 scoped equivalents and strict exit 0. There are no compiler refusals, timeouts or unjudged
results in these selected runs. Every group uses the complete parent command, not a finishing-only
subset, and restores all 1077 held regular inputs in the original and copied tree with modes and
independent Python/system hashes. Copied parent names agree exactly. The gitlink is metadata only.
All 37 retained pinned source identities and tool metadata remain bound.

The maintained compiler-selection catalogue entry is also replayed in the restored copy. It is killed
by `compiler_installation_paths_preserve_inventory_units_and_configuration`, as expected, and both
original/copy inputs restore again. A separate raw-receipt audit rejects altered scope, missing result,
replacement, finishing name, count, exit, filename and outcome summaries. The summary adapters are
ignored supplementary evidence; the native CLI, compiled controls and exact commands below are the
tracked/re-runnable producers.

All commands use this complete filter/strict tail:

```bash
--package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail
```

The consumer command is `bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs` with
`--only L216.cond.64 --only L273.cond.97`. The observer command is
`bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs` with
`--only L60.stmt.7 --only L64.stmt.11`. The trust command is
`bash scripts/mutation_sweep.sh xtask/src/trust.rs` with
`--equivalents xtask/trust.equivalents` and one `--only` for every trust edit in the table.
Run these commands sequentially: they share the maintained sweep workspace.

Held native tree `9abd2a4f22ae17b3eba7cac0d030d787427c7aa6` names this measured epoch.
The receipts are under `target/compiler-provenance-repair/current/`; the supplementary audit is
SHA-256 `8fa90c2092f898b465e942601d5221c56d2529b6bacea595ddbe57211dfbd51e`.


The held native epoch and final completion-document/focused epoch are separate. New judgment sources
`xtask/src/trust_compiler.rs` and `trust_compiler_observer.rs` join the complete later mutation scope.
Artifact binding, remaining compatibility controls, the baseline-evidence repair and full current
source/premise/integration verification still precede F30 signoff.

### Observed compiler artifact binding

The `2026-10-18` slice `.15.2.1.3.3.2.2` completes production role-artifact binding.
The inventory identifies an executable, a library's rlib or a comparison harness by the
compilation actually observed for that role. Cargo's stdout messages no longer select that file.
The earlier escaped JSON-looking warning experiment did not establish an artifact-report exploit;
this change implements the specified provenance contract without claiming one.

`xtask/src/trust_artifact.rs` owns selection and validation. The metadata pass retains each
target's source by package directory, kind and original target name, separately from the existing
unit-owner and target-classification maps. A binary or test selects that precise metadata target;
a library selects its unique library source. Selection requires exactly one recorded compilation
owned by the package and naming that source, whether relative to the written tree or under its
ordinary/canonical prefix. Missing or ambiguous metadata and invocations stop the inventory.
The generator-entrypoint caller still receives all verified units and ignores the additional
invocation return value; its compiled-input checks remain in force.

Source identity matters for test harnesses. A package may have a binary and integration test
with the same target name. A custom `[[test]]` with `harness = false` also compiles without
the normal `--test` switch. The permanent public-inventory regression builds that combination
and runs the selected artifact: it prints `custom-harness`, while its companion binary would
print `companion-bin`. The standard test-harness phase independently lists `same: test`.
The selector therefore uses metadata ownership and source, rather than treating the presence of
`--test` as target identity.

#### A naming query, with no source evaluation

For the selected invocation the instrument runs the pinned compiler in the same written tree,
with the unchanged base environment and null stdin:

```text
rustc --print=file-names - --crate-name <observed-name> \
  [--crate-type <observed-type>]... [--target <observed-target>] \
  [--test] [-C extra-filename=<observed-suffix>]
```

Only observed naming options are forwarded. The actual source path, code generation options,
feature cfgs, extern libraries and search paths are excluded. The observed `--out-dir` is retained
for locating outputs, not passed to the query. Repeated crate types are preserved; duplicate
singleton options, missing values and unsupported joined naming spellings refuse. An absent
extra-filename option is not invented. The query uses the fixed `rustc` entrypoint and pinned
toolchain environment, without the private observer fields. It creates no additional provenance
unit or session record.

A permanent actual-compiler control first compiles a real executable, then replaces its source
with `compile_error!`. Selection still succeeds and the executable prints `query-artifact`.
The source bytes, output file population and executable bytes stay identical across the query.
This is an observable check that naming does not re-evaluate that source; the separate prototype's
45 queries remain dated corroborating evidence, not a projection of all possible inputs.

#### File validation and inventory paths

The complete query output must be UTF-8, newline terminated and a nonempty sequence of unique
basenames. Empty names, dot/dot-dot, separators, CR, NUL and duplicate names refuse. The canonical
observed output directory must be within the canonical target directory for this role. Every
reported output must exist as a regular non-symlink file and is read. The selected artifact is
hashed: an executable requires exactly one output, and a library exactly one rlib among the
validated outputs.
Missing, foreign, linked, directory-valued and ambiguous outputs refuse rather than choosing an
arbitrary last report.

The recorded path is relative to the role's target directory and now names the compiler's direct
output. For example, an actual binary fixture records
`release/deps/a-ea31e1b4fa6d99dc`, while its library records
`release/deps/libb-20572edba3a82d76.rlib`. The suffixes are compiler/build identities, not names
a package consumer should hardcode. Previous Cargo root aliases such as `release/a` or
`release/libb.rlib` are no longer selected. The inventory schema still carries `artifact.path`
and `artifact.sha256`; packaging and verification use those fields and actual bytes.
A clean rebuild under another output directory must retain the established build/configuration
identity contracts; the broad trust parents continue to check that behavior.

The permanent binary/library parent runs the selected binary, checks the library archive header,
requires direct-output paths and compares both recorded hashes with the independent system
SHA-256 implementation. The harness parent does the same hash/path validation before running
both standard and custom harnesses. The output-refusal parent uses a literal independent digest
and checks valid behavior before each malformed-output case. The actual-query parent also tests
missing/wrong/ambiguous metadata, foreign/missing/duplicate invocations and an actual failing
target-information query, followed by fresh successful selection.

#### Replay commands and scope

All shell scratch/cache settings are those in the toolbox. The focused artifact command is:

```bash
cargo test -p xtask artifact -- --nocapture
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
  IteratorTests.test_cargo_absolute_prefix_edits_preserve_line_selection -v
```

The proof-premise guard initially rejects the changed production source. Revalidation confirms
the unchanged normalization and diagnostic-selection bodies, base environment, observer,
controller, CLI/gate, catalogue and active/historical reason files. The new artifact query uses
the same absolute tree and no Cargo producer flags. The guard now also binds artifact production
and rejects a deliberately changed query cwd. An initial isolated Python module-style command
could not import the test harness; the recorded replay uses the harness's documented file/root
arguments. That invocation error was not a proof result.

The native copied-tree command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail \
  --only L84.cond.58 --only L156.logic.98 --only L154.chain.132
```

It selects three actual discovery edits: omit the containment refusal, weaken package/source
conjunction to disjunction, and truncate the matching invocation population before uniqueness
checking. This is a bounded contract smoke measurement. The full new source and all affected
current sources/reasons remain owned by `.3.3.3`, after compatibility and the separately owned
baseline-evidence repair. No whole-source adequacy or F30 closure follows from these three edits.

The dated native result is 3 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Original and restored-copy parent CLI lists agree at 167 names; all 1079
held regular inputs restore with modes and independent Python/system hashes. The gitlink is
metadata only. All 37 retained producer-source pins remain bound. The receipt audit rejects nine
false summaries: scope, omitted result, replacement, intended name, total, exit, filename, source
identity and outcome. It is supplemental to the maintained native producer and tracked controls.
Held native tree `0e213978519fa101cc75e9f6eac1ed30249fd408` precedes completion-doc/focused staging.
Receipts are under `target/compiler-artifact-binding/`; supplemental audit SHA-256 is
`74fcaf11ad73522da648ecd14031568738f8893861dbe07302dc4721a7e0910a`.

The prior completed collector leaf is sealed without rewriting its bytes. Independent committed-core,
index and full pre-seal tree reconstruction agree, and all earlier archives remain identical.
Final focused/book/doctrine checks close this bounded slice. Compatibility, the owned baseline
evidence repair and complete current-source/integration review remain ahead; F30 remains open.

### Compiler warning data across harness and generator roles

The `2026-10-18` slice `.15.2.1.3.3.2.3.1` adds permanent actual-inventory controls for
the two remaining caller roles. The earlier root/library parent stays unchanged. Production
collector, observer, artifact selection, normalization, base environment, gate/CLI, catalogue,
proof controls and reason bytes stay identical to the preceding artifact-binding commit.

Both new parents use a committed fixture and real Cargo/compiler builds under the pinned toolchain.
The warning is an exact reviewed `global_asm!` site emitting a multiline `.warning` whose text
contains a `Running` line. The complete command-shaped payload is
`rustc --crate-name phantom phantom.rs --out-dir bogus`. Those tokens are warning data, never
a compiler invocation.

| Phase | Comparison harness | Separate generator entrypoint |
| --- | --- | --- |
| Ordinary source | Exact literal unit names, owners and source files; complete configurations retained as the baseline; selected test artifact hashed independently and run | Declared generator/input provenance paths, roles and hashes; separate generator target build required |
| Admitted `not-a-compiler` warning | Same literal units/files and complete configurations; artifact still lists the actual test | Exact source/input hashes and roles; source admission is used |
| Admitted `rustc` warning | Same checks | Same checks |
| Admitted full command-shaped warning | Same checks; no phantom unit | Same checks; no diagnostic-driven build refusal |
| Identical warning without its admission | Source-rule refusal, no inventory | Source-rule refusal, no inventory |
| Genuine `compile_error!` | Public Cargo failure summary and actual recorded compiler exit 1, no inventory | Public Cargo failure summary and actual recorded compiler exit 1, no inventory |
| Restored ordinary source | Original literal expectations/configurations and actual test executable recover | Original declared provenance/hash expectations recover |

Package/path strings below belong to the scratch fixtures. The harness compiles exactly `d` and `i` from `"crates/i"`, and `r` from `"crates/r"`.
Its test unit must read both `"crates/i/tests/d.rs"` and
`"crates/i/tests/support/mod.rs"`; the other units read their literal library roots.
The selected executable lists `same: test` and its inventoried hash agrees with the system
SHA-256 implementation. Complete configuration comparisons use the same fixture/output root,
while source-file population expectations are literal and independent of the collector.

The generator is a separate `"crates/g"` binary that reads `src/helper.rs`.
Its live form explicitly declares that helper as an input. The checker's provenance must
therefore contain precisely `"crates/g/src/main.rs"` as `generator` and
`"crates/g/src/helper.rs"` as `input`, with the exact committed source bytes hashed.
Every admitted warning must consume its source admission; no unused admission can hide a
skipped generator-file check.

An initial oracle expected an undeclared helper to appear automatically in provenance.
The [canonical generator limitation](trust.md#what-it-is-not) and
`docs/specs/trust/decision_trust-generated-sources.md` §8 explicitly distinguish a
generator's compiled-input checks from its declared provenance. Native dep-info did contain
the helper, while the serialized provenance correctly held only the declared entrypoint.
The fixture now declares the helper input rather than changing that contract. A separate
initial failure-message oracle expected the source diagnostic text; Cargo emits that on
its JSON stdout channel, while the public error selects Cargo's stderr summary. The corrected
control independently requires one failed compiler record with exit 1, null signal and the
expected crate-name argument. These were test-oracle corrections, not production repairs.

The tracked parents are:

```bash
cargo test -p xtask compiler_provenance_preserves_admitted -- --nocapture
```

Their exact names are
`trust::tests::compiler_provenance_preserves_admitted_harness_warning_records_and_recovery`
and
`trust_generated::generator_tests::compiler_provenance_preserves_admitted_generator_warning_records_and_recovery`.
The unchanged child-process protocol retains its 30-second limit and requires a finishing
named result. The local dated run passes both parents, with about 17.5 seconds for the harness
and 13.9 for the generator. These timings describe that run; they are not a new timing bound.

The native copied-tree smoke selects actual file-population edits, retaining only the first
or only the last dependency-file entry:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail \
  --only L775.iter.222 --only L775.iter.223
```

The scope is file-consumer integration across the real caller routes, not a reconstruction of
the retired warning parser. No whole-source adequacy result follows. Remaining path/feature/
copied-execution and lifecycle/concurrency controls stay separately owned before the complete
source/premise/integration milestone and F30 review.

The dated native result is 2 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Both new parents kill the first-file truncation; the harness also kills the
last-file truncation. Full original/restored-copy parent CLI lists agree at 169 names. All 1079
regular inputs restore with modes and Python/system hashes; the gitlink is metadata only. All 37
retained producer pins bind. Nine deliberately false receipt summaries reject; the supplementary
audit is SHA-256 `c88ab259e9a0c5336a3e2a4bcf9b32f9e293bf51eb1f36c48fce38ae0172e85e`.
Receipts are under `target/compiler-role-warnings/`; held native tree
`ade5532820ac0f42eeaeed5ad2e60db124b69b49` precedes completion-doc/focused staging.

The source diff is insertion-only inside test regions, with production/caller/reason identity
checks retained in the receipt. The final staged focused/book/doctrine workflow is required before
commit. Path/feature/copy and lifecycle/concurrent-writer slices remain active obligations;
complete source/premise/integration verification still precedes F30 signoff.

The previous committed artifact-binding leaf is sealed during completion bookkeeping. Its
committed core, archive/index bytes and full raw pre-seal reconstruction agree; all 275 earlier
archives remain identical. This archival epoch follows the native input-restoration epoch.

### Compiler artifact path and feature compatibility

The `2026-10-18` slice `.15.2.1.3.3.2.3.2` holds the observed-artifact contract across
actual harness and separate generator builds, including admitted target features. Earlier
quoted/multiline/source-environment/canonical path and feature-isolation parents remain in place.
Production and previous controls remain unchanged; the maintained mutation catalogue gains one
default-feature omission control.

The new parent is
`trust::tests::observed_role_artifacts_preserve_paths_and_disable_target_default_features`.
It builds a comparison harness that reads a declared generated support module, plus a separate
generator binary that reads its declared helper input. The library target and generator each
declare this reviewed manifest feature table:

```toml
[features]
default = ["tripwire"]
tripwire = []
```

Both source files contain a compile-error guard under `cfg(feature = "tripwire")`.
The manifests need exact whole-file admissions for the feature rule. Successful inventory builds
therefore exercise the fixed `--no-default-features` argument for the actual library, test and
generator target routes. A separate real Cargo build of the generator with defaults enabled must
fail with the generator tripwire diagnostic, establishing that its guard is operative.

The output cases are these fixture-relative path strings:

```text
plain
space'quote\slash\nRunning `rustc`\rµ
alias/output -> physical'\nµ/output
```

Here `\n` and `\r` describe actual LF and CR bytes in the fixture names; the mixed path also
contains a space, apostrophe, backslash, backticks and Unicode. The alias is a real symbolic link
in the scratch fixture. It resolves to a physical parent whose name itself contains a quote,
line break and Unicode. The artifact files themselves must still be regular non-symlink files
within the canonical target directory.

For every output, the parent requires:

- Complete package, unit and provenance records, and shared items, equal to the plain baseline.
- Literal unit counts and source-file/hash maps for every root and the comparison harness,
  including the harness's generated support module.
- Direct `release/deps/` artifact paths, existing regular files and actual bytes whose hashes
  agree with the independent system SHA-256 implementation.
- A selected harness executable that lists `same: test`.
- Precisely the declared generator/helper provenance paths and source hashes, plus the separate
  generator build directory.

The independent hash command receives artifact bytes on stdin. Its filename-output format
escapes paths containing line breaks or backslashes, so parsing that format as an unescaped digest
would be an incorrect oracle. No equality of rlib bytes across different output directories is
assumed: the inventory records each actual artifact and its digest at that output.

The direct command is:

```bash
cargo test -p xtask \
  observed_role_artifacts_preserve_paths_and_disable_target_default_features -- --nocapture
```

The dated run passes within the unchanged 30-second parent limit, at about 7.5 seconds.
The copied whole-trust baseline includes this parent. A native file-population smoke also
requires its finishing name:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail --only L775.iter.222
```

The maintained catalogue control `trust-cargo-target-default-features-enabled` removes only
the observed compile argument `--no-default-features`. Its exact source block must occur once,
and `cargo xtask mutate` must report a finishing kill by the new parent, then restore the
original source. It is run in the already verified scratch copy:

```bash
# Working directory: target/mutation-sweep/tree
cargo xtask mutate --timeouts-fail --only trust-cargo-target-default-features-enabled
```

This omission reaches the library's default-feature tripwire before generator checking. The
separate direct-Cargo failure above independently holds the generator guard. The control is a
regression against a missing argument, not a claim that current production enabled defaults.
The native smoke and catalogue replay are bounded evidence, not a whole-source adequacy result.

#### Owned native selection defect

Preparing the copied run uncovered a separate P2 tool defect. The native sweep's `--only`
filter does not validate requested IDs. Through its existing explicit Git/Cargo CLI fixture,
requesting `L999999.not-a-current-id.0` runs one baseline, attempts zero mutations, and exits 0
with `none attempted`. A contrasting valid control attempts its one mutation and reports an
equivalent. The actual script is unchanged in this reproduction; the fixture tests tool argument
selection and makes no claim about a Rust compiler.

Source review identifies the skip loop and empty-result success path as the root cause.
Task `M3.6.9.6.3.2.2.15.2.1.3.3.3.0.2` owns rejecting unknown, mixed-valid/unknown and stale
selections before commands, with permanent red/green tests and unchanged valid-selection/
restoration semantics. It is scheduled after failed-baseline evidence hardening and before
full source sweeps. Current measurements additionally bind exact selected/result populations;
the receipt audits reject missing results and incorrect scopes. This defect is open until its
owned repair passes; a successful zero-attempt selection is not mutation evidence.

The dated native result is one named file-population kill and one maintained default-feature
catalogue kill, both strict exit 0. The full original/restored-copy CLI lists agree at 170 parent
names. All 1081 held regular inputs restore after both runs, with modes and Python/system hashes;
the gitlink is metadata only. All 37 producer-source pins remain bound. Eleven false receipt
summaries reject. The catalogue preserves its entire prior prefix and appends only the new control.
Held native tree `2d57037b0d18086c94dcd668d65776f62f51180a` precedes completion-doc/focused staging.
Receipts are under `target/compiler-path-feature/`; supplemental audit SHA-256 is
`7ba3713e1e98903138857d1e19642b581242a631ca0c9dcb1fd920457748e14a`.

The prior committed warning-role leaf is sealed byte-exact. Its committed core, archive/index
and full saved raw reconstruction agree; all 276 older archives remain identical. The final
staged focused/book/doctrine workflow closes this bounded slice. Lifecycle/concurrent-writer
controls, both native-tool hardening children and full current-source/integration review remain
before F30 signoff.


### Compiler record boundary controls

The `2026-10-18` slice `.15.2.1.3.3.2.3.3.1` fills the remaining record-consumer
boundary controls. Production, existing tests, observer, artifact selector, constructors, CLI,
catalogue and reasons remain unchanged. This is one bounded part of the lifecycle matrix;
writer/setup/publication/signal and concurrent-writer closure remain separate children.

The new parent is
`trust::compiler::tests::actual_records_refuse_remaining_field_range_byte_and_shape_boundaries`.
It compiles the embedded observer with the pinned compiler, then uses that observer to run a
real successful Rust compilation with actual source, dep-info/link output and complete argv.
The consumer must return the literal original argument vector. Its completed record becomes
the seed for isolated corruption cases, each first checked as valid with that same result.

| Boundary | Explicit fault cases |
| --- | --- |
| Schema and decoded identity field types | `schema-version`, `schema-type`, `session-type`, `role-type`, `cwd-type`, `missing-key` |
| PID representation/range | `pid-zero`, `pid-negative`, `pid-overflow`, `pid-fraction`, `pid-type` |
| Exit and signal types | `exit-type`, `signal-number` |
| Argument vector | `argv-type`, `argv-empty` |
| Argument byte encoding | `hex-odd`, `hex-uppercase`, `hex-nul`, `hex-non-utf8` |
| Compilation/query shape | `query-tail`, `compilation-no-output`, `compilation-no-name`, `compilation-no-emit` |
| Canonical record framing | `no-newline`, `extra-newline` |
| Raw JSON/text | `invalid-json`, `invalid-text` |
| Record kind/extension | `record-directory`, `record-extension` |

The dated source-derived census contains 29 cases, each with a valid-before-fault check.
The earlier parent independently retains its missing/partial/stale-session/wrong-role/cwd,
extra/duplicate fields, failed status, PID/filename mismatch, invalid hex/compiler/query,
linked record, non-string argument and non-object controls. The actual compilation/forwarding
and native byte receiver parents remain intact.

Numeric PID faults use matching record filenames. In particular, PID 0 is paired with
`0.json`, so the positive-PID guard must reject it even though the filename agrees.
Byte faults append an extra encoded argument to an otherwise complete compilation record.
Omitting the NUL guard would therefore admit the damaged compilation, rather than merely
changing an unknown-query diagnostic. Odd-length and uppercase hex, embedded NUL and invalid
UTF-8 remain explicit refusal boundaries.

Missing/non-string identity fields, invalid PID type/range, malformed argv and status values,
partial invocation shapes, missing/extra record newlines, invalid JSON/text and directory/
extension substitutions must all refuse. Expected instrument-owned error categories are held;
raw JSON/UTF-8 reader failures must produce a nonempty error without overconstraining the
standard-library diagnostic. A fresh independently created session then runs a real
`rustc -vV` query successfully and yields no compilation units.

The direct command is:

```bash
cargo test -p xtask   actual_records_refuse_remaining_field_range_byte_and_shape_boundaries -- --nocapture
```

The final dated parent run passes within the existing 30-second bound, at about 0.65 seconds.
An earlier successful query-seed version was strengthened before measurement to require
otherwise valid compilations for byte faults and matching filenames for numeric PID faults.
That strengthens the acceptance oracle without changing production.

The native copied-tree command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs   --package xtask --timeout 600   --filter=trust:: --filter=trust_generated:: --filter=trust_gate::   --filter=--skip --filter=route_tests --timeouts-fail   --only L168.cond.31 --only L233.cmp.72
```

These actual discovery edits omit the NUL refusal and change the PID predicate from
`> 0` to `>= 0`. The new parent must finish and kill each. Complete current-source
assessment, all new judgment-source obligations and integration still belong to `.3`;
neither these samples nor the corruption matrix establishes whole-source mutation adequacy.
The separately owned baseline-evidence and unknown-ID selection repairs precede those full sweeps.

The dated native result is 2 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Both kills carry the new parent name. Original/restored-copy complete CLI
lists agree at 171 parents, and all 1082 held regular inputs restore with modes and independent
Python/system hashes. The gitlink is metadata only; all 37 retained producer-source pins bind.
Nine deliberately false receipt summaries reject. Held native tree
`1d6fca0e3c4861f50ce0bd33e241eff6c8184123` precedes completion-doc/focused staging.
Receipts are under `target/compiler-record-boundaries/`; supplemental audit SHA-256 is
`4c28247a53873e7d2ed3983bc3f334a08f3ec2b33addcab9ff9f6ee414c470e6`.

The prior committed path/feature leaf seals with exact committed core, archive/index and full
saved raw reconstruction; all 277 older archives remain identical. Final staged focused/book/
doctrine checks close this bounded consumer slice; writer/publication/signal and concurrency
children remain open before the compatibility checkpoint and native-tool/source milestone.
