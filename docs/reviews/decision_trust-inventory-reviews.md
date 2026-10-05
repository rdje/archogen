# The trust-dependency inventory: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.1` (`docs/tasks/M3.md`). This is the review history of
  [[decision_trust-inventory]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M3.6.1`'s acceptance is a review, by a context that did not write the record, finding that the gate as specified
exercises each of `ROADMAP.md` §14.4's five cases, that no input a root's build reads can be shared without being
reported, and that the baseline cannot be accepted by its author. Each round below is one such review, with every
finding and the answer the record gives it.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `ROADMAP.md` §4.2, §4.4, §10.3 and §14.4;
- `docs/decisions/decision_zero-dependency-engine-core.md` and `decision_findings-for-director-review.md` §11;
- `xtask/src/catalog_build.rs`, which reads cargo's metadata and the compiler's dependency information today;
- the workspace's manifests, and cargo's and rustc's documentation for `cargo metadata` and dependency information.

It is barred from other implementation, and may check external documentation on the web.

## Rounds

**Round 1**, `2026-10-03`: the reviewer re-measured §1 with `cargo metadata` and the dependency information under
`target/`, confirmed every fact, found cases 1, plain-source 2 and 5 working, and found the volume case-insensitive.
There were 20 findings, 11 of them defects, and the verdict was "not acceptable as it stands". Most answers reuse what
the catalog's build checker already does — build from the commit's blobs, a cleared environment, configurations
above the repository refused, build scripts, procedural macros and link inputs refused — so the record takes rules
already reviewed rather than writing new ones. Measured while answering: dependency information names an included
file unnormalised (`crates/archogen-api/src/../../../docs/semantics/kinds/core.eadl`), and the five roots of the
answered §1 still share nothing.

| Finding | Defect | Answer |
| --- | --- | --- |
| A1 — the generator rooted at the `archogen-s0` library, not the `archogen` executable that runs `archogen build`, so the kind modules and `archogen-api` it compiles in were invisible | yes | §2: a root is the executable that runs a role, the library only where none runs it yet; §1: the generator is `archogen`, its reach and the kind modules measured |
| A2 — the reference model's counterpart, `rt-core`, not a root | yes | §2: for each reference model, the implementation it validates is a root; §1 lists `rt-core` |
| A3 — `trust/roots.eadl` the author's to change, and nothing checking a role is filled | yes | §2 and §5: the roots change only under §5's review; every workspace package is classified, `trust-unrooted-package` otherwise |
| A4 — "a review is accepted" not mechanical; `trust/` outside findings §11's code-owned paths | yes | §5: acceptance is the catalog's premise 3 — a hosting-signed merge on `origin/main`'s first-parent chain after the named commit — with the gate a required check; findings §11 amended to add `trust/` and the gate's code |
| A5 — build-script reads, procedural-macro reads and native link inputs outside rustc's dependency information | yes | §3: each refused in a root's closure, `trust-undeclared-input`, as the catalog refuses them, until a leaf designs its coverage |
| A6 — `RUSTFLAGS`, `[env]`, profile overrides, `RUSTC_WRAPPER`, configuration above the repository; a build-configuration file used as data hidden by its exclusion | yes | §3: the catalog's cleared environment and configuration refusals; each unit's `--cfg` and codegen arguments and `env-dep` values in an item's configuration; a configuration file read as data is an ordinary file |
| A7 — builds over the working tree, so an uncommitted revert masks a committed change | yes | §3: built from the commit's blobs, never the working tree |
| A8 — case 4 had no consumer: the gate builds its own inventory | yes | §6: judged at packaging; the package ships the trust build's artifacts, equality by construction, no reproducibility assumed; `trust-verify`; `M4.7`'s acceptance amended |
| A9 — one file under two paths: a symlink, a case alias | yes | §3: names resolved lexically, a symbolic link refused, case twins refused by the written tree |
| A10 — no feature set recorded; features from workspace-wide metadata would fabricate case 5 | yes | §3 and §6: configuration from each root's own units, never from workspace unification |
| A11 — a verbatim copy unreported | yes | §4: a non-empty file whose sha256 appears in both roots under different paths is a shared item; edited copies stated as unseen |
| A12 — whole-file hashing over-approximates | no | §4 declares it |
| A13 — case 5's sentence inexact | no | §6: the gate compares shared items only; the identity is recorded, not compared |
| A14 — `std` and the compiler shared by every root | no | §3: accepted infrastructure by toolchain identity, its bump reviewed where it is ledgered |
| A15 — a baseline form whose item is no longer shared | no | §5: refused until removed |
| A16 — the CI trigger | no | §7: any path in any root's inventory, `trust/` and the gate's code |
| A17 — the exclusions | no | sound once A1 is answered; §2 classifies `xtask` and the wasm module as not roots |
| A18 — `catalog_build::dep_info` drops `env-dep` values and scans `deps/` only | no | `M3.6.2`'s; build scripts are refused, so no `build/` unit |
| A19 — `/1`, a root reaching itself, "no feature" | no | §1 and §2: the root's package included; `--no-default-features` and no `--features` |
| A20 — the fixed runtime-data statement goes false at `M2.7.5` | no | §2 and §3: runtime data a per-root declaration |

**Round 2**, `2026-10-03`, a new context that had not read round 1: it re-measured §1 and every pairwise intersection,
found the copy rule silent on today's tree (no two tracked files under `crates/`, `xtask/` and the kind modules share a
sha256), confirmed each reused catalog rule is real, compiled a three-line `global_asm!` crate with the pinned rustc to
show that a `.incbin` file's bytes reach the rlib and not its `.d`, and read cargo's `compute_metadata` and GitHub's
rules for required checks. There were 18 findings, 12 of them defects, and the verdict was "not acceptable as it
stands". The sharpest was B1: a form proposed in a pull request could never pass that pull request's required check,
so F30 would have blocked for ever. The answer gives each item one of three outcomes — refused, pending, accepted —
so a review is its merge, and makes F30 `not-established` before the named commit, as the catalog's claims are.

| Finding | Defect | Answer |
| --- | --- | --- |
| B1 — a form or roots change could never pass its own pull request's required check: accepted only once on `main` | yes | §5 and §6: three outcomes; pending items are reported on a pull request and fail only the release's assurance step; the protected merge is the review |
| B2 — no bootstrap: the first `trust/roots.eadl` refused for ever or accepted by its author; new packages stalled unannounced | yes | §5: before the named commit everything is pending and F30 `not-established`; the first acceptance is the first protected pull request proposing the files whole; `M3.5` and `M4.2` say their programs are pending classifications |
| B3 — `global_asm!`'s `.incbin` reads a file no dependency information names (measured) | yes | §3: assembler directives refused in a root's closure, as the catalog refuses them |
| B4 — runtime data both roots read at one path not a shared item | yes | §4: a runtime-data file both roots declare, at one path or two |
| B5 — one executable running two roles makes every package shared, losing case 1 in noise | yes | §2: refused, `trust-shared-program`; `M2.21` filed, since `archogen analyze` would do it and its declared owner `M2.6` closed without it |
| B6 — programs are targets: `src/bin/*.rs` in a classified package arrives unrooted | yes | §2: every executable target classified; unclassified is pending |
| B7 — the reference model's root contradicted the executable rule; the differential harness and its adapter outside every inventory | yes | §2: the exception stated; §4: the comparison harness an item the pair shares |
| B8 — `-C metadata` and `cfg` differ by host, so every accepted item fires across machines | yes | §2 and §3: one host, the CI runner's, named by the baseline; elsewhere the gate reports and compares nothing; `-C metadata` and `extra-filename` left out, `env-dep` paths relative |
| B9 — an edition change reaches every shared package untouched | yes | §3: the whole rustc invocation, `--edition` included |
| B10 — case 5's example (adding a non-root package) failed the gate | yes | §6: a package with no executable target is case 5; one with an executable target a pending classification |
| B11 — case 4 bound the inventory to the shipped artifacts, not to the program that produced the results | yes | §6: each result names its producer's sha256; `trust-verify` refuses another producer and any result of a library-rooted role; `M4.7` carries it |
| B12 — `CARGO_HOME` only created, never emptied; build scripts not refused before building | yes | §3: `CARGO_HOME` made anew every run; build scripts and macros refused from `cargo metadata` before any build |
| B13 — the reuses need adapting | no | `M3.6.2`'s goal names each adaptation |
| B14 — the refusal stated as coverage owed | no | confirmed; unchanged |
| B15 — host-only builds leave target-gated code unseen | no | §3's outside list states it |
| B16 — acceptance trusts code ownership of `trust/` from the named commit | no | §5: trusted from the named commit on, `M2.7.6`'s design |
| B17 — a path-filtered required check is left pending | no | §7: CI unfiltered |
| B18 — wording: four roots and one empty role; the step-1 bullet | no | the step-1 bullet marked superseded |

**Round 3**, `2026-10-03`, a new context that had not read rounds 1 and 2, which compiled scratch crates under the
pinned toolchain — `naked_asm!` and `global_asm!` with `.incbin`, doc-attribute includes, `debugger_visualizer`,
`include!` of a generated path, `#[path]` through a macro — and built a two-crate workspace in two directories, under
two toolchains, as a library root and as a binary root. It confirmed §1, the reused rules and the configuration's
stability on one host. There were 22 findings, 16 of them defects, and the verdict was "not acceptable as it stands".
Defects per round: 11, 12, 16 — rising, and most of them in what the record had decided beyond its domain: what an
unreviewed item does to a claim, a package or a release, and how acceptance is recorded and protected. The knowledge
card's fifth habit applies: the record is narrowed to roots, the inventory, what is shared and the report; its gate
refuses or reports; acceptance and its costs go to the leaves that own them, each carrying its part.

| Finding | Defect | Answer |
| --- | --- | --- |
| C1 — `naked_asm!` with `.incbin` reads a file no dependency information names (measured), and was not refused | yes | §3, §6 and How to apply: `naked_asm!` refused with `global_asm!` and `asm!` |
| C2 — results of a library-rooted role refused, so the reference model's results never fit a package | yes | §5: binding each result to its producer is `M4.7`'s, the comparison harness a producer the trust build inventories; `M4.7`'s acceptance amended |
| C3 — pending items reached packages and claims | yes | §5: what an unreviewed item costs a claim is `M4.8`'s, `not-established` naming F30, in its acceptance |
| C4 — before the named commit the release step's outcome undefined | yes | §5 and §7: the trust step reports Unavailable until §5's leaves land, never Passed |
| C5 — off the named host, nothing compared and the release step passing | yes | §5: Unavailable on every host until then |
| C6 — `trust/roots.eadl` accepted whole by whichever merge last touched it | yes | §5: acceptance per form, `M2.7.6`'s and `M3.6.5`'s |
| C7 — code ownership of `trust/` carried by no leaf | yes | `M2.7.6.5`'s goal: `trust/` and the gate's code in `CODEOWNERS` |
| C8 — a fork's main | yes | §5: the catalog's checks of `origin`, `M2.7.6`'s |
| C9 — a stale form refused in §5, pending in §6 | yes | §6: `trust-baseline-stale`, refused |
| C10 — the comparison harness shared today, so the first baseline is not empty | yes | §1, `M3.6.3` and findings §11: today's one shared item stated |
| C11 — the toolchain in `--extern` hashes and the program path | yes | §3: stripped and dropped |
| C12 — lint levels from the root manifest in every invocation | yes | §3: lint levels left out; the edition and profiles are configuration |
| C13 — generated sources sharing a generator unseen and unstated | yes | §3: a stated gap until a leaf makes generated sources declare their generator and input |
| C14 — a `cdylib` unclassified; a classification never reopened as its closure grows | yes | §2: `cdylib`, `staticlib` and `dylib` classified, with the role libraries their closure reaches; reopened when that grows |
| C15 — `M3.6.3`'s leaf contradicted the record; no owner for `trust-verify`'s test | yes | `M3.6.3` rewritten: both commands, reported or refused, `trust-verify` tested |
| C16 — findings §11's paragraph described a gate that waits | yes | amended to the narrowed record |
| C17 — `write_tree`'s path grammar | no | §3 states it |
| C18 — the alias-only configuration rule | no | §3 states it |
| C19 — proposed or accepted roots | no | §2: the roots the commit proposes |
| C20 — `resolve` and `mcp` | no | `M3.4`'s goal carries `resolve`; `mcp` is the built binary's, which `M2.21`'s rule reaches |
| C21 — runtime data in `M2.7.5` | no | `M2.7.5`'s acceptance declares it |
| C22 — the harness identified and built | no | `M3.6.2`'s, by the record's §4 |

**Round 4**, `2026-10-03`, a new context that had not read rounds 1 to 3. It built every root clean under the pinned
toolchain from `git archive` copies, with a fresh `CARGO_HOME` and target directory, and built scratch crates: a
macro that makes two roots include one file, a renamed `global_asm!` with `.incbin`, a profile edit, per-root against
workspace-wide feature unification. It confirmed §1's counts and empty intersections, the normalised configuration
across two checkout directories and three toolchains, per-root features, `debugger_visualizer` in the dependency
information, and the harness's reuse of the roots' units. There were 20 findings, 11 of them defects, and the verdict
was "not acceptable as it stands". Defects per round: 11, 12, 16, 11. Seven were delegations the named leaf did not
carry, or carried weaker; four were holes in the inventory. The answer keeps the narrowing: the step's outcome leaves
the record for `M3.6.3` and `M3.6.5`, each delegation is written into an acceptance, and the four holes are closed by
rules that say what is built and what is shared.

| Finding | Defect | Answer |
| --- | --- | --- |
| D1 — `M2.7.6.5` had no acceptance, and "per form" was given to a code-owner rule, which approves by path | yes | `M2.7.6.5`'s acceptance written; §5: per form is `M3.6.5`'s alone |
| D2 — what an unclassified program, or an unaccepted root form or classification, costs carried by no leaf | yes | §5, `M4.8` and `M3.6.5`: established and Passed only with every form and classification accepted and no program unclassified |
| D3 — `M4.8`'s rule vacuous while nothing is shared | yes | `M4.8` stated positively: established only with an accepted inventoried root for each role and every shared item accepted |
| D4 — the comparison harness built and inventoried by no rule | yes | §2, §3, §4: the pair's form names the harness target; built with `cargo test --release --no-run`, its files, configuration and artifact recorded |
| D5 — `trust-shared-program` with no mechanism; "role libraries" undefined | yes | §2: each root's form names its role packages; refused when an executable root's build compiles another role's role package |
| D6 — a file a shared package's macro makes both roots include, outside that package's dependency information (measured) | yes | §4: a shared file is one both roots' compilations read that no shared package's own compilation reads |
| D7 — a profile edit changes every root (measured `-C panic=abort`) and was compared nowhere | yes | §3, §4: the build configuration — the toolchain's identity and the root manifest's tables that reach rustc — an item every pair shares |
| D8 — `M3.6.2` omitted `naked_asm!`, said "reported", and a renamed `global_asm` evades a macro scan (measured) | yes | §3 and `M3.6.2`: the catalog's identifier rules, `asm`, `global_asm`, `naked_asm` anywhere and `link` in an attribute; refused |
| D9 — generated sources handed to no leaf, though §14.4's case 3 names them | yes | `M3.6.6` filed |
| D10 — off the baseline's host, the report and a Passed step unstated | yes | §2: every refusal but a stale form applied, every shared item reported unreviewed; `M3.6.5`: never Passed off that host |
| D11 — the step's outcome decided here, and `Unavailable` not producible by the runner | yes | §0, §5, §7: the outcome is `M3.6.3`'s and `M3.6.5`'s; `M3.6.3`: `Action::NotBuilt` owned by `M3.6.5` |
| D12 — `--diagnostic-width` differs with the terminal; `--check-cfg` kept | no | §3: the output-format flags and `--check-cfg` left out |
| D13 — the summary listed two refusals of four | no | the opening lists all four and the grown closure |
| D14 — the toolchain "accepted as infrastructure" | no | §3: the build configuration, a shared item like any other |
| D15 — "repairs alone" for a fix that edits `trust/` | no | §6: repaired in the code or in `trust/`, which review accepts |
| D16 — `M3.6`'s verification and `PROGRAM.md`'s fixture map stale | no | `.1` to `.6`; F30's map names the leaves that carry acceptance and its costs |
| D17 — `M4.2` and `M3.5` tied to the named commit | no | unreviewed until accepted under `M3.6.5` |
| D18 — an absolute dependency name inside the tree refused | no | §3: the written tree's prefix stripped, then resolved |
| D19 — the ledger's scope claimed every file named | no | the ledger's Known limitations name `.incbin`; the refusal is lexical, so no premise is re-measured |
| D20 — the gate reads its forms with the generator's reader | no | §7 states it |

**Round 5**, `2026-10-03`, a new context that had not read rounds 1 to 4, which measured from a `git archive` copy
under the pinned toolchain: scratch packages putting an `.incbin` inside an `include!` target and a `#[path]` module,
a new direct edge to an already shared package, and the renamings of assembler macros. It confirmed §1, the
harness's units byte for byte against the roots', every renaming, raw identifier and `cfg_attr` form caught, today's
57 files clean under the scan, off-host behaviour consistent, and every delegation carried bar four. There were 16
findings, 10 of them defects, and the verdict was "not acceptable as it stands". Defects per round: 11, 12, 16, 11,
10. None reopened a delegation or asked the record to decide another leaf's part; each sharpened a rule.

| Finding | Defect | Answer |
| --- | --- | --- |
| E1 — the catalog's scan reads only `.rs` files under a package; an `.incbin` inside an `include!` target or a `#[path]` module passed (measured); an untokenizable file | yes | §3: the scan over every file the dependency information names, data included; a file the token rules cannot read refused |
| E2 — edges without the dependent: a new consumer of a shared package unreported (measured) | yes | §4: edges are (root, dependent package, edge kind) for every edge into the item |
| E3 — a held but unaccepted form hides its item, or case 5 is never silent | yes | §0, §6: the report's change part and its standing list; case 5 reads the change part |
| E4 — a result's producer never compared with the inventoried artifact | yes | §6 and `M4.7`: `trust-verify` refuses a result naming another program |
| E5 — `M4.8` asked only that no program be unclassified | yes | §5 and `M4.8`: every root form and classification accepted |
| E6 — runtime data handed to a root but undeclared passes | yes | §5, §6: `M4.7`'s manifest records every input handed; `trust-verify` refuses an undeclared one; `M3.4`, `M3.5`, `M4.2` declare theirs |
| E7 — acceptance tied to the form's author, not the change's | yes | §5 and `M3.6.5`: an identity that authored neither the form nor any commit that changed its item since its last accepted form |
| E8 — `Action::NotBuilt` runs nothing; the step's two possible builds | yes | §5 and `M3.6.3`: a new runner action, `Failed` on a refusal, otherwise not built, owned by `M3.6.5` |
| E9 — an inventory written on a refusal reaches the package | yes | §3: no inventory on any refusal; `M3.6.2`'s acceptance |
| E10 — "closure grew" beside "role packages grown" | yes | §0: role packages |
| E11 — the harness compared only within its pair | no | §4: paired with every other root as well |
| E12 — a classification whose program is gone | no | §6: stale, refused like a baseline form |
| E13 — a classified program's role packages unspecified | no | §2: `cargo metadata`'s graph, an example's development edges included |
| E14 — "outside the tree" against "not a blob" | no | §3: not a blob of the commit, and each file's bytes after the build its blob's |
| E15 — compiling a role package counted as running it | no | §2: counted; types or constants taken from a third package |
| E16 — `link` refused inside any macro call | no | benign and absent today; the catalog's rule kept |

**Round 6**, `2026-10-03`, a new context that had not read rounds 1 to 5. It built the five programs from a `git
archive` copy with a fresh `CARGO_HOME`, built scratch workspaces for a link-time symbol binding and for a new reader of
a shared package's file, and ran both a copy of the catalog's tokenizer limited to the record's subset and the
catalog's own `scan` over the real roots. It confirmed §1, the harness's units identical to the roots', the reused code
as the record describes it, and every delegation carried as worded bar one. There were 15 findings, 10 of them
defects, and the verdict was "not acceptable as it stands". Defects per round: 11, 12, 16, 11, 10, 10. One defect was
round 5's own answer, which asserted the roots use `include!` and `#[path]` without counting — the second habit of the
review-answers card, a justification resting on a charge never checked.

| Finding | Defect | Answer |
| --- | --- | --- |
| F1 — an `extern "Rust"` block in a reference package ran the implementation's `#[no_mangle]` function with no shared package, file or copy (measured) | yes | §3: link-time symbol binding refused, the catalog's `extern` rule in the subset; today's three `pub extern "C" fn` admitted |
| F2 — a file one root compiles and another is handed, at one path, no shared item | yes | §4: one file bullet, compiled or handed, at one path or two |
| F3 — a claim's subject, the description or the plan, cannot be a declared dependency | yes | §3, §6, `M4.7`, `M3.5`, `M4.2`: dependencies declared by path, subjects recorded with the claim |
| F4 — a non-shared package starting to read a shared package's file unreported (measured) | yes | §4: each file of a package item carries its readers, compared |
| F5 — the change part measured against forms the commit writes; root forms outside it | yes | the opening, §6: against the base commit's forms; a root form added, changed or removed in it |
| F6 — the first baseline in the blocked leaf, so case 5 is never silent before the director acts | yes | §5, `M3.6.3`: the proposed forms, the host and today's classifications committed unaccepted; `M3.6.5` accepts them |
| F7 — the catalog's tokenizer over data refuses the kind modules (measured); the roots use neither `include!` nor `#[path]` | yes | §3: `include` refused anywhere, `path` and `link` inside an attribute or one named by a metavariable — the catalog's macro-argument rule left out, since `eadl-front` and `rt-analysis` write `path` there; `.rs` files scanned, data hashed; round 5's premise corrected |
| F8 — no leaf fails packaging on `trust-verify`; what an unaccepted item costs a package decided nowhere | yes | §5, `M4.7`: packaging runs `trust-verify` and fails on a refusal; nothing beyond `M4.8`'s `not-established` |
| F9 — the report unbound to its build | yes | §6: the gate's report names the build identity and the inventory's and baseline's sha256; `trust-verify` refuses another's |
| F10 — authorship free text | yes | §5, `M3.6.5`: the hosting's authenticated identity; an unestablished author counts as everyone |
| F11 — the linker's version promised, not listed | no | §3: `cc --version`'s first line |
| F12 — the harness's edges and pairing | no | §4: a `development` edge; paired with every root other than its pair's |
| F13 — the step after `M3.6.5` when not Passed | no | `M3.6.5`: `Failed` on a refusal or anything unaccepted, `Unavailable` off the host |
| F14 — a non-root package's profile override trips every pair | no | §3: profile tables that apply to a root's units |
| F15 — "(§0)" dangling | no | "the opening" |

**Round 7**, `2026-10-05`, a new context that had not read rounds 1 to 6, which built a `git archive` copy and three
scratch workspaces and ran the catalog's tokenizer, limited to the record's subset, over the real roots. It confirmed
§1, that the subset spared today's roots, and every delegation carried bar two. There were 18 findings, 8 of them
defects, and the verdict was "not acceptable as it stands". Defects per round: 11, 12, 16, 11, 10, 10, 8. For the
fifth round running, a reader found a channel the hand-built subset of refusals admitted. That is the defect seen from
the right angle: an enumeration of forbidden constructs is never complete. The answer applies the catalog's rules
whole, default-deny, and admits by review only the sites today's roots need — 13, measured with the catalog's own
`scan` over every compiled `.rs` file.

| Finding | Defect | Answer |
| --- | --- | --- |
| G1 — `macro_rules!` assembling `#[path]` onto `evil.txt`, compiled as Rust and never scanned; an `extern` block and an `.incbin` inside (measured) | yes | §3: the catalog's rules whole, `macro_rules` and `macro` refused; default-deny with admissions |
| G2 — a reference package's `#[no_mangle] memcmp` taking over the implementation's comparison in the harness (measured) | yes | §3: `no_mangle` refused, the wasm module's three exports admitted by review |
| G3 — a procedural macro on the harness's development edge runs during the trust build (measured) | yes | §3, `M3.6.2`: every refusal over the harness's closure with development edges followed |
| G4 — a merged sharing without its form leaves every later commit to report it, against case 5 | yes | §6: `trust-form-missing`, refused on the host; `M3.6.3`'s unrelated-commit fixture |
| G5 — the baseline described as accepted forms while `M3.6.3` commits proposed ones | yes | the opening and §5: proposed or accepted, accepted only as `M3.6.5` reads it |
| G6 — the off-host outcome read two ways | yes | §5, `M3.6.5`: off the host, `Failed` on a refusal and `Unavailable` else |
| G7 — a first acceptance's window unanchored; the pusher's source unnamed | yes | §5, `M3.6.5`: from the commit that proposed the form; the hosting's record of the merging pull request |
| G8 — the host read from the commit's own baseline would let a commit turn its comparison off | yes | §2: the base commit's host; a host change in the change part; off-host "not compared" |
| G9 — §5 restates the leaves' acceptance, two places to keep in step | no | `PROGRAM.52.2` moves §5 into the hand-off ledger |
| G10 — file readers missing from a form's fields | no | §5: content, configuration, edges and file readers |
| G11 — which commit the gate is built from | no | §7: the base commit, as the catalog's checker is |
| G12 — `trust-shared-program` and the harness | no | §2: the harness apart |
| G13 — a root form whose target is gone | no | `trust-form-missing` and `trust-baseline-stale` together cover it |
| G14 — `M3.6.2`'s symbolic-link wording broader than §3's | no | `M3.6.2`: the rule is §3's, a name the dependency information holds |
| G15 — How to apply omitted `extern`, `include!`, `#[path]` | no | rewritten: any refused site, by admission |
| G16 — `M4.7` before `M2.21` | no | `M4.7`'s producer for the scheduling checker is the executable `M2.21` decides; ordered in `M4.7` |
| G17 — subject against dependency decided by the pipeline | no | a stated gap: §3 lists what a root reads at run time that the pipeline does not hand it |
| G18 — today's role packages unlisted | no | §2 lists them; `eadl-front` and `eadl-model` are no role's |
