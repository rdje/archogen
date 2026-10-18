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
