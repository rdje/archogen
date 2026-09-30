# Catalog records: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`). This is the review history of
  [[decision_catalog-records]], moved out of it verbatim on `2026-09-30`: round 5's table would have taken that
  record past the per-file ceiling `README-ROUTES` holds for `docs/decisions/` (`README_POLICY.md`).

## The fact / decision

`ROADMAP.md` §9: "Independent reviewers check preconditions and any interpretation on which correctness relies".
Each round below is one such review of `decision_catalog-records.md`, with every finding and the answer the record
gives it. The section numbers are that record's.

Each round was a new context, read-only, which had not written the record. Each was given `ROADMAP.md` §7.2–§7.5,
§9, §10.3, §14.4 and §15, the runtime variant's §1, `Bound`, the reader's forms and the profile registry, and was
barred from any other implementation.

**Round 1**, `2026-09-30`: 22 findings, 6 of them defects, and the verdict that the record "should not be
accepted as it stands". Acceptance sentence (2) failed by construction, and sentence (1) held only for what the
hash covered.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the record version was in the contract hash and moved with every facet, and dependents pinned it, so a timing change staled the contract, cascaded, and invalidated behavioral claims | the record's `version` is the contract's alone; the lock pins **own** hashes; dependencies take a caret requirement; costs name their targets and `measured-with` carries timing-only code (§2, §8, §9) |
| 2 | defect | a timing review was not tied to the code whose costs it states | the timing bound hash covers the implementations of the record, its dependency closure and its `measured-with` records with theirs (§3) |
| 3 | defect | only hand-named files were hashed | an entry inside a package stands for the package; reached sets; the compiler's dependency information decides completeness in the gate (§3) |
| 4 | defect | invalidation compared with the last-blessed lock, so it was empty at every commit | claims carry closures with statuses, reads and inputs, and invalidation compares with them (§10) |
| 5 | defect | citations were declared; timing-implies-implementation was a convention; `none` made hardware records unusable | lookups compute citations; the closure follows derived lines, so it is complete by construction (§7) |
| 6 | defect | a review had no verdict, and promotion was a file move | verdicts `production` and `rejected`; production needs a `production` status on all four facets (§5, §6) |
| 7 | gap | a `none` facet could not be reviewed | it is reviewed like any other (§5, §6) |
| 8 | gap | the evidence behind a cost was never hashed | `file` locators must be in the own set, and `ledger` sections are in the bound hash (§2, §3) |
| 9 | gap | target and binary were free strings; `basis` could be empty | targets resolve and are in bound hashes; `binary` has three forms, checked against the claim's engine-made image; `basis` is never empty (§2, §3, §7) |
| 10 | gap | a laundered version kept dependents' reviews | a dependent's bound contract hash covers the dependency's; the lock is append-only (§3, §9) |
| 11 | gap | `by ≠ maintainer` compared a person with a role; reviews could be edited | roles and task trees cannot coincide; `who` cannot be the maintainer; reviews are ledgered (§5, §9); §13 names the rest |
| 12 | gap | facts lived only in the behavioral model | §12 fixes each variant name's kind and facet, and condition 8 sits on each cost (§2, §12) |
| 13 | gap | not every variant input had an owner | §12 covers every row; composite inputs are the caller's until `M2.10` |
| 14 | gap | §9's per-catalog admission evidence was checked nowhere | §6's table; the rest is a review obligation, stated in §13 |
| 15 | gap | profiles and preconditions were never consulted | `unsupported-profile` at admission; preconditions become assumptions (§7) |
| 16 | gap | "source location" meant the catalog file's span | every known fact and cost carries a `locator` (§2) |
| 17 | ambiguity | the byte encoding was undecided | §3's normative grammar and a worked example |
| 18 | ambiguity | "tracked", symlinks, submodules, path spellings, stray files | §4; §1 |
| 19 | ambiguity | `value` raw or padded; `Bound` mapping; factor range; conversion | §2; §12 |
| 20 | ambiguity | the lock's version rules | §9 |
| 21 | nit | non-ASCII, bidi and zero-width characters | the whole file, and every decoded string, is printable ASCII (§1) |
| 22 | nit | summary and §5 disagreed; order unstated | aligned; §5 states the order |

**Round 2**, `2026-09-30`: 13 of round 1's answers were judged partial. 25 new findings, 10 of them defects, and
the verdict was "should not be accepted yet". All nine digests of the first worked example matched, but that
example exercised no `file`, dependency or target line.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| B1 | defect | invalidation ignored rejections and demotions | recorded statuses are compared at the recorded hash, and a production claim's record must still be in the namespace (§10) |
| B2 | defect | `J^release` and `J_s` include what the catalog cannot know; costs growing with task count had no checked scope | composite, the caller's until `M2.10`; `holds-for` on every known cost (§2, §12) |
| B3 | defect | `C_i`, `CS_i` under-charged; application declarations unowned | as B2; the application owns the task facts and condition 8 for its figures (§12) |
| B4 | defect | behavioral facts about code were never bound to it | the behavioral bound hash covers the record's implementation, `describes` records' and its targets (§3) |
| B5 | gap | a dependency's model change never staled a dependent's review | `behavior-model <D>` and `timing-model <D>` lines (§3) |
| B6 | defect | timing edits became contract edits; shared files coupled the models | costs name targets; `measured-with`; no file in a behavioral and a timing own set (§2, §8) |
| B7 | defect | `binary` was never compared | a production claim needs its own engine-made image behind every image-dependent cost (§7) |
| B8 | gap | image-independent costs had no encoding | `(binary independent "why")`, refused for image-specific names and on emulators (§2) |
| B9 | defect | a rejection could be laundered; append-only lived only in a skippable gate | the review ledger; CI's comparison; tracked records; every caller reads the index (§4, §5, §9) |
| B10 | gap | "latest-dated" could be gamed | status is order-free; a rejection stands until answered (§5) |
| B11 | defect | package expansion leaked | package rules, reached sets, and the compiler's dependency information over a matrix of features, profiles and targets (§3) |
| B12 | defect | decoded strings could hold LF or non-ASCII | refused (§1) |
| B13 | ambiguity | expanded paths unconstrained | one path grammar; `-z` (§4) |
| B14 | ambiguity | target lines as a set or per mention | a set; the worked example has target lines (§3) |
| B15 | ambiguity | unknown's verdict, and the enabled set's owner, disagreed with the variant | `analysis-inconclusive`; the variant's "How to apply" amended (§12) |
| B16 | gap | the facet a lookup returned was the author's choice | kind and facet fixed per name (§2, §12) |
| B17 | gap | an `external` locator was checked against nothing | `ledger` anchors must exist, and their sections are hashed (§2, §3) |
| B18 | gap | uncomputable hashes and new providers | both affected (§10) |
| B19 | ambiguity | which `unbuilt` costs block | the costs the claim read (§7) |
| B20 | nit | locator rendering, integers, unknown costs' target, subform order | §2; §1 |
| B21 | nit | comments could carry misleading code points | the whole file is printable ASCII (§1) |
| B22 | nit | a source under `catalog/` | refused (§3) |
| B23 | nit | vacuous machine row; empty timing model counted as present | "present and not empty" (§6) |
| B24 | gap | S0's pointer had no rule | `(id, contract version, record hash)`, recomputed by its checker (§7) |
| B25 | nit | the `0.x` caret rule | stated (§2) |

**Round 3**, `2026-09-30`: the three file hashes and all 18 digests of the two-record example matched, recomputed
by `shasum`, `sha256sum` and `openssl`. 11 of round 2's answers were judged partial. 24 new findings, 7 of them
defects, and the verdict was "cannot yet be accepted".

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| C1 | defect | review order decided a verdict and was not ledgered, so swapping two reviews undid a demotion | status is order-free: a production verdict holds at its hash, a rejection holds until answered, and there is no experimental verdict to order against (§5) |
| C2 | defect | renaming or splitting a record shed its rejections | `supersedes`; a retired id with an unanswered rejection must be superseded, and its successor inherits it (§2, §5, §9) |
| C3 | gap | "final for that hash" was escaped by a version bump or a comment edit | a rejection stands at any hash until a production review names it in `answers` (§5) |
| C4 | defect | the own hash covered dependency packages, the workspace manifest and toolchain files, so a workspace lint forced every version up | those are the reached set, in bound hashes only (§3) |
| C5 | defect | the gate refused only repository files outside the set | every dependency path outside the sets and the sysroot is refused (§3) |
| C6 | gap | the build matrix, linker inputs, model packages, index versus working tree and the environment were open | a checkout of the index; a cleared environment; features, profiles and targets enumerated; every package in every source set built; config files may hold only `[alias]` (§3); the rest is §13's |
| C7 | defect | caller-supplied inputs and the claim's binary escaped the production rule | caller and application inputs make a production claim `not-established`; the image comes only from an engine-made build (§7) |
| C8 | defect | timing facts vouched for costs their hash did not cover | condition 8 is each cost's `holds-under-preemption`; `eager-switching` must come from the record that supplies `switch` (§2, §12) |
| C9 | defect | facts about code the record does not implement were bound to nothing | `describes`, derived into the behavioral bound hash; code facts need an implementation or `describes` (§2, §3, §12) |
| C10 | gap | the closure missed facets its bound hashes cover; "in production" was undefined | the closure follows every derived line; "in production" is the namespace; statuses are recorded and compared for every claim (§6, §7, §10) |
| C11 | gap | `measured-with` was not transitive | its records' dependency closures are derived too (§3) |
| C12 | gap | the overlap rule was per record, and refused shared dependency packages | catalog-wide, over own sets only (§8) |
| C13 | gap | costs for a target the contract excluded were selected, then refused | refused at load; §8 says admitting a new target is a contract edit, and why (§2, §8) |
| C14 | gap | some catalog inputs are composite too | `C_s` needs `no-application-code.<source>`; image-specific names; what `γ`, `ρ` and `δ` must bound; the application's masking is a task fact (§12) |
| C15 | ambiguity | `holds-for`'s verdict and "other sources" | outside `holds-for` is undeclared, so `analysis-inconclusive`; "other" means declared sources but the timer (§2, §12) |
| C16 | gap | an emulator's timing setup was in no hash | a production claim never rests on an emulator's cost; the files its `.env` names are target files; the invocation scripts are §13's (§3, §7, §13) |
| C17 | gap | one image per target was unstated | §13 |
| C18 | gap | bless and the ledger could leave an id unloadable | bless recomputes lines not at `HEAD`, checks each review once when ledgering it (§5, §9) |
| C19 | ambiguity | `missing-fact` had no trigger | removed; an unreadable value is the analysis's `analysis-inconclusive` (§7, §12) |
| C20 | ambiguity | `<source>` was undefined | the source's id, verbatim; one outside the grammar reaches the variant undeclared (§12) |
| C21 | ambiguity | eleven details left to the implementation | each is decided: `Bound`'s binary and origins (§2); the ledger hash and a worked lock and review line (§3); every caller reads the index (§4); the date rule in UTC (§5); the targets for `any`, directory entries with packages and workspace discovery (§3); CI's base (§9); the integer range (§2); each name's kind (§12) |
| C22 | nit | `.env` edits void reviews across the target | §13 |
| C23 | gap | ledger sections were unhashed | `ledger` derived lines (§3) |
| C24 | nit | the variant still called some inputs the engine's or the platform's | marked in the variant's record |

**Round 4**, `2026-09-30`: all 22 values of the worked example matched again. The reviewer recomputed them with
`shasum`, `openssl` and `sha256sum`, from §3's grammar rather than from the printed lines. Of round 3's defects, C1
and C4 were judged closed, C5, C7, C8 and C9 partial, and C2 open. There were 14 new findings, 4 of them defects,
and the verdict was "not acceptable as it stands". The answers changed derived lines, so the example's model,
record and review hashes moved. An independent implementation of §3 reproduced every earlier digest before
computing the new ones, and each changed input was checked with three tools.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| C2 | defect, re-checked | three constructions still shed a rejection: a decoy that takes over the lineage, a stub that re-creates the retired id, and a split that keeps the original id | lineage is ledgered and permanent, and a superseded id is never used again. A rejection also binds its items, so what moves takes the rejection with it (§2, §5, §9) |
| C5 | partial | code under `cfg(not(feature = …))` was built by neither row of the matrix; the environment was a list to clear; native libraries are not in the dependency information | `/1` refuses features, so one configuration per profile and target is built; the environment is an allowlist; foreign blocks and `#[link` are refused (§3, §13) |
| C7 | partial | the image rule covered costs only: a claim about an image built from old code passed with facts reviewed on new code | the image's build record must name the closure's implementation bound hashes (§7). Features are refused in `/1` |
| C8 | partial | `no-application-code.<source>` had no record or facet, so a driver change elsewhere left it `production` | a behavioral code fact of the record that supplies `service.<source>`, like the source's other facts (§12) |
| C9 | partial | a record could state facts about another record's code without `describes`; a call resolved at link time reached code no Cargo edge names | a code fact takes a `code` locator into its record's or a `describes` record's code (§2); foreign blocks are refused (§3); the rest is review (§13) |
| D1 | defect | answering a rejection revived an older production review at the rejected hash, and a review could approve a hash the facet never took | a production review counts at `h` only when every rejection naming `h` is answered at `h`, and a review must name the current bound hash when it is ledgered (§5) |
| D2 | defect | a contract change never made the model reviews stale: a new profile, a dropped precondition or a weakened dependency kept them | both models' bound hashes carry `contract <R>` (§3, §8) |
| D3 | defect | `(targets any)` bound the behavioral model to no target, so a new two-hart target read a fact reviewed without it | `any` binds to every target under `targets/` (§2, §3) |
| D4 | defect | the append-only comparison belonged only to the gate and CI, so a staged lock without a rejection passed a claim | a production claim reads a commit and checks its lock against every ancestor's; CI covers every branch; unpublished history is §13's (§4, §9, §13) |
| D5 | gap | five couplings over-invalidate across the models | (b) and (c): no file is in an implementation's and a model's own sets, and code facts use `code` locators. (a), (d) and (e) are stated as over-approximations, which are sound (§7, §8, §13) |
| D6 | gap | a cost derived from another record's timing figure had no timing line to it | `measured-with` also gives `timing-model <X>` (§2, §3) |
| D7 | gap | the set of `TARGET_KIND` values was open, and the emulator test was fail-open | `emulator` or `board`; admission requires `board` (§2, §7) |
| D8 | ambiguity | "two cases admit" `independent` had no mechanical form | exactly `compare-rounding` and `delivery`, on a board. `preemption-delay` is image-specific (§2, §12) |
| D9 | gap | a host-only model package, a target without `RUST_TARGET` and a failed build were undecided | a model package builds for the host; a missing `RUST_TARGET` and a failed build are refused (§3, §11) |
| D10 | ambiguity | "closes a cycle" over which relation | a cycle in the graph of §3's derived facet lines, which is what hashing needs (§11) |
| D11 | gap | the summary overstated the production rule | restated. Where evidence was obtained is §13's |
| D12 | gap | the ledger's toolchain entry named neither rustup nor the claim the gate rests on | the entry names both, with the limits (`ledger.md#rust-toolchain`) |
| D13 | ambiguity | fourteen details left to the implementation | each decided: the manifest dialect; workspace membership; nested packages, refused; `.env` path values; the source facts' record; `eager-switching`; builds run in the package's directory; facts for a claim with no target; CI's base for any branch; untracked files, refused by the gate; `answers` names any inherited rejection; a duplicated ledger anchor, refused; a factor term outside `u32`, refused; the reader's language-version check can fire only on an `eadl-version` form, which §1 refuses anyway (§2–§5, §9, §11, §12) |
| D14 | nit | the review hash was shown abbreviated; the summary omitted the implementation's bound hash; "a space" repeated `0x20`; "not in the record" was vacuous | fixed |
| A1 | defect, found while answering | the lock held a review's ledger hash only, so a retired id's rejections could not be told from its production reviews: `catalog-lock-retired`, and C2's inheritance, were not computable | each review line carries its facet, verdict and hash, with answer, item and lineage lines, so status needs the ledger alone (§9) |
| A2 | defect, found while answering | §3 called any directory holding a `Cargo.toml` a package, so the root's workspace manifest made every crate an entry "inside a package", standing for the whole repository | a package is a manifest with a `[package]` table, and nested packages are refused (§3) |

**Round 5**, `2026-09-30`: all 22 values of the worked example matched, by two routes: hand-written inputs
hashed by `shasum` and `openssl`, and a separate parser of the record's text hashed by `hashlib` and `sha256sum`.
Of the re-checks, C8, D1, D2, A1 and A2 were judged closed, and C2, C5, C7, C9, D3 and D4 partial, D4 with a
defect-level construction. There were 21 new findings, 2 of them defects, and the verdict was "cannot be accepted
as it stands". No answer changes a derived line, so the example's digests stand.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| C2 | partial, re-checked | a renamed package directory, and a contract or `none` statement copied under a new id, carried no item of the rejection | items now include each own-set file's bytes and the facet's forms without `version`, so moved content matches whatever its path (§5, §9) |
| C5 | partial | only the gate builds; the gate's build read unhashed cargo configuration; manifest flags and assembly bypassed it | E1, E5, E6 and E7 below |
| C7 | partial | the build record named source hashes and not how the image was built | E4 below |
| C9 | partial | `asm!` reached another record's code at link time with no foreign block and no Cargo edge | assembly macros are refused as tokens in `/1` (§3) |
| D3 | partial | a claim on an undescribed target read an `any` fact | a claim's target must be a named target, or it is `unsupported-profile` (§7) |
| D4 | partial, defect-level | a branch forked before a published rejection escaped it, and a shallow clone skipped the check | a production claim also takes `origin/main`'s ledger and its ancestors, and refuses a shallow history (§4, §9) |
| E1 | defect | a production claim never re-established completeness: a `#[path]` to an unhashed file, committed without the gate, reached a claimed image | the image's build record carries its compiler's dependency information, which must lie in the closure's sets (§7) |
| E2 | defect | `covers` and lineage lines were written by bless and checked by nobody, so a hand edit could strip them | the gate and CI recompute every line their base lacks and refuse any difference; a `supersedes` with no line is `catalog-lock-missing`; a retired id's rejection is discharged only by a successor's review (§9) |
| E3 | gap | items missed unchanged content | as C2 (§5) |
| E4 | gap | the build record attested sources, not configuration | the record names each package's matrix cell, with no other flag or `cfg` (§7) |
| E5 | gap | a checkout under `target/` let cargo read `target/.cargo` and the working tree's configuration | the gate lists every config file cargo would read and refuses any it does not hash, the repository's own tracked copies excepted when their bytes match the index; `CARGO_HOME` is an empty directory of the gate's; dependency paths are read relative to the checkout (§3, §13) |
| E6 | gap | `cargo-features` and `rustflags` were not refused | refused (§3) |
| E7 | gap | assembly reached link-time symbols and files no dependency information names | `asm!`, `global_asm!` and `naked_asm!` refused in `/1`; the slice needs none (§3) |
| E8 | ambiguity | the lexical rule's unit, and whether it refused `extern "C" fn` or `#[link_section]` | defined over tokens: a foreign block, an attribute whose path is exactly `link`, an assembly macro (§3) |
| E9 | ambiguity | the dialect read as two-segment headers excluded the root's `[workspace.lints.clippy]` | headers take any number of bare keys; a quoted `cfg(…)` key is outside the dialect (§3) |
| E10 | ambiguity | the `.env` grammar, duplicate keys, the target-id grammar, recursion under `targets/`, and `RUST_TARGET` | a line grammar with each key once, a stem grammar, directly under `targets/`, and `RUST_TARGET` required of every target (§2, §3) |
| E11 | ambiguity | where a claim's target comes from | as D3 (§7) |
| E12 | ambiguity | whether a production claim with no image could be admitted | only when every implementation in its closure is `none` (§7) |
| E13 | gap | two suppliers of a name, and a failed co-location, had no verdict | refused at load: `catalog-conflict` for every profile and target, `catalog-field` for co-location (§9, §12) |
| E14 | gap | three facts were not classified as about code | the code facts are listed; the three are among them (§12) |
| E15 | gap | the catalog's section fact and the task fact were not composed; ρ's refinement had no owner | the variant's condition is their conjunction (`M2.7.5`); ρ is the conservative worst case until `M2.10` (§12) |
| E16 | ambiguity | whether reached packages are held to the package rules | they are, and their workspace manifests are reached (§3) |
| E17 | gap | a merge's second parent was compared with nothing | the gate and CI compare with every parent (§9) |
| E18 | gap | profile definitions were in no hash | a profile's meaning is versioned by its id (§15); §13 says so |
| E19 | gap | a lookup that found nothing was not recorded | claims record failed lookups, which §10 checks (§7, §10) |
| E20 | ambiguity | whether a bless could answer a rejection it ledgers | no: a rejection is answerable once at `HEAD` (§5) |
| E21 | nit | §8's "as they were" against its own exemption; the summary's reached files; §5's "of that facet"; §13's "verdicts" and "unreviewed"; the ledger's "two" and "three" | each corrected |

**Round 6**, `2026-09-30`: all 22 values matched by two routes, and the reviewer re-measured the record's facts
about the tree: every tracked path in normal form, every manifest inside the dialect, `.cargo/config.toml` holding
only `[alias]`, the target's `.env` inside its grammar. C2 and D3 were judged closed; C5, C7, C9, D4, E1 and E2
partial, the last three with defect-level constructions. There were 16 new findings, 5 of them defects, and the
verdict was "cannot be accepted as it stands". Most of the defects assumed control of the build machine or the
hosting, and the defect count over six rounds (6, 10, 7, 4, 2, 5) was not falling. So the director was asked, and
ruled that the record states a threat model: it is §0 now, and later rounds are judged against it. No answer
changes a derived line, so the example's digests stand; a worked forms digest is added.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| C5, C7, C9 | partial | through F4, F5, F6, F8 and F13 | as those rows |
| D4 | partial, defect-level | a stale, rewritten or absent `origin/main` | as F2 |
| E1 | partial, defect-level | the sysroot, `.incbin` and smudge filters | as F4, F5 and F6 |
| E2 | partial, defect-level | nothing checked `covers` lines at load | as F1 |
| F1 | defect | a bypassed commit, left on `main` though CI failed, ledgered a rejection with no items, which nothing recomputed | in scope, and removed at the root: items, answers and lineage are no longer written. They are read from the commit that ledgered each review, so there is nothing to strip. That CI blocks `main` is premise 3 (§0, §5, §9) |
| F2 | defect | `origin/main` stale, rewritten by hand or by a force-push, or absent | a clone with no `origin/main` is refused; the claim records the `main` commit it was checked against, and §10 flags a claim whose commit has left `main`'s history. A ref set by hand is premise 2, a force-push premise 3 (§0, §4, §7, §10, §13) |
| F3 | ambiguity | recomputing a push as one state refused the normal reject-then-fix workflow | the gate and CI replay every commit from the base, each against its own parents (§9) |
| F4 | defect | a `#[path]` into the writable sysroot | the `path` attribute is refused, and so is every source path outside the written index, the sysroot's included; measured, no workspace crate's dependency information names one. Editing an installed toolchain file is premise 1 (§3) |
| F5 | defect | a renamed `global_asm` import, and a module read from a non-`.rs` file | the identifiers `asm`, `global_asm` and `naked_asm` are refused anywhere, as are `#[path]` and `include!`, so every Rust source is a tracked `*.rs` file (§3) |
| F6 | defect | a smudge filter or line-ending setting changed the bytes the gate compiled | the gate writes each file from its blob, with no checkout, and compares each source's bytes with its blob after the build; case-colliding paths are refused. Git's local state is premise 2 (§0, §3) |
| F7 | gap | which paths an image may read beyond the closure | the image's closure joins every record whose package it compiled; generated code and application inputs are named with their hashes, and make a production claim `not-established` until `M4` rules on them; any other path refuses the claim (§7) |
| F8 | gap | the compiler was not checked against the pin; rustup overrides | `RUSTUP_TOOLCHAIN` is set to the pinned channel, `rustc -vV` must name it, the build record carries it, and an index without the toolchain file is refused (§0, §3, §7) |
| F9 | ambiguity | a config file's tables were not held to a dialect | config files are held to the manifest dialect, every key path beginning `alias` (§3) |
| F10 | ambiguity | `exclude`'s meaning, and an outer workspace cargo could reach | `exclude` is a path prefix, as cargo's is, and `cargo metadata` must report the written index's root as the workspace root (§3) |
| F11 | gap | `.env` values a shell reads differently, and a file named without `/` | values are restricted to characters with no shell meaning, and any value naming a tracked file is a path (§3) |
| F12 | gap | §10 named no tree | the commit read and `origin/main`, both recorded (§10) |
| F13 | gap | a macro-assembled foreign block, symbol interposition, global hooks | `extern` is refused except in `extern crate` and `extern "…" fn`, and `no_mangle`, `export_name`, `link_section`, `used`, `panic_handler`, `global_allocator` and `alloc_error_handler` are refused; `rt-core` passes (§3) |
| F14 | gap | a cost moved with its target renamed | cost items also match a cost's name on a target whose files hash the same (§5) |
| F15 | ambiguity | seven details | (a) timing code facts follow the timing locator rule (§2); (b) an `unknown` supplies its name (§12); (c) a claim with no target needs every closure record to be `any` (§7); (d) dependency paths are resolved lexically, then held to §4 (§3); (e) a review's date is checked against its ledgering commit's committer date (§5); (f) no sysroot path is admitted (§3); (g) premise 3 (§0) |
| F16 | nit | the summary's closure; identical forms not listed as an over-approximation; no worked forms digest; the variant's note; a refusal that could not fire | each fixed; the forms digest is in §3's example, and the variant's ownership note names the fourth task fact |

**Round 7**, `2026-09-30`, the first judged against §0: all 22 values of the worked example matched again by two
routes, hand-typed inputs through `shasum` and `openssl`, and a separate parser of the record's text through
`hashlib` and `sha256sum`. D4, F2, F3, F4 and F6 were judged closed, and F8 closed under premise 1. C5, C7, C9, E1,
E2, F1, F5 and F7 were judged partial or open. There were 15 new findings: 3 defects (G1, G2, G4), G5, a defect
that bites only once `M4` gives images, and G3, an ambiguity at defect level. None of them needs a §0 premise
broken. The verdict was "cannot be accepted as it stands". Answering G9 found one more: F14's answer matched a
cost moved with its target renamed by a hash of the target's files, but a target's own id is in its `.env` and
its `.eadl`, so no rename could ever match. No answer changes a hash input of the worked example, whose target's
one key keeps to the new key rule, so its digests stand.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| C5, E1, F5 | partial, defect-level | through G1 and G3 | as those rows |
| C7, F7 | partial | through G5 | as G5 |
| C9, F13 | partial, defect-level | through G2 | as G2 |
| E2, F1 | partial, defect-level | through G4 | as G4 |
| G1 | defect | a `path` attribute written inside `cfg_attr(…)`, or produced by a local macro, loads a module from a non-`.rs` file that is never scanned, so an assembler include inside it reads bytes no hash covers | the scan covers every source path the compiler's dependency information lists, each of which must end in `.rs`; `path`, `link` and `used` are refused inside an attribute at any depth, within `cfg_attr` and `unsafe` too; `include_str!` and `include_bytes!` are refused with `include!`. Measured: a plain build of `rt-core` lists only its three `.rs` sources (§3) |
| G2 | defect | a local macro takes `no_mangle` or a whole `extern "…" fn` as an argument and emits the attribute or a foreign block | a catalog package defines no macro: `macro_rules` and `macro` are refused. The attribute words are refused inside any macro invocation's arguments, and so is `extern`, whose exemption holds only outside them. The words that name importable attribute macros are refused anywhere, so a renamed import is caught too; `rt-core` passes (§3, §13) |
| G3 | ambiguity, defect-level | whether `r#asm` is `asm` | an identifier is compared by its name, without `r#`, and a non-ASCII identifier is refused, since Rust's NFC normalization could turn one into a refused name (§3) |
| G4 | defect | on an unpublished branch, a hand-copied lock line makes a branch commit the only ledgering commit in the claim's ancestry, so a rejection's items are read from substitute content | ledgering commits are searched in `origin/main`'s ancestry too, for a production claim and for §10, and each is verified before anything is read from it: the review's form must hash to the line's ledger hash, and the facet's bound hash must be the one the line names, or the catalog does not load. So every ledgering commit holds the same content (§5, §9, §11) |
| G5 | defect, latent until `M4` | the image's closure was held only to the source-path rule, and the claim result did not record it | every not-established cause applies to the image's closure, which the claim result records and §10 checks (§7, §10) |
| G6 | gap | nothing refused a nightly or moving channel in `rust-toolchain.toml` | the pin must be a release number, `MAJOR.MINOR.PATCH`; the repository's is `1.95.0` (§0, §3) |
| G7 | gap | the loader's inputs lacked history; the gate's pending commit and its date were undefined; how a product reads history was undecided | the loader is given a history, from the repository's tooling under `xtask/`, outside `NO-SUBPROCESS`'s population; how a product reads history is `M4`'s. The gate's pending commit has `HEAD`, and `MERGE_HEAD` in a merge, as parents, and the gate's start time as its date (§4, §13) |
| G8 | ambiguity | at a merge, whether a line is new against the first parent or all of them | against every parent, in the replay and in blessing, as the ledgering commit is defined (§9) |
| G9 | gap | no byte grammar for the hash of a target's files, which cost items used | the item is replaced: a cost's name with its target's `TARGET_KIND` and `RUST_TARGET`, which a rename keeps. This also answers the defect found while answering (§5, §13) |
| found while answering | defect | F14's hash of a target's files holds the target's own id, so a renamed target never matched | as G9 |
| G10 | ambiguity | §12's γ said "one preemption", the variant's says "one preemption or service" | aligned with the variant (§12) |
| G11 | gap | a timing change under a production record fails §6, so §8's independence holds only when the change lands with its review | stated, with the demotion route and its effect on claims, and for `.env`, workspace-manifest and toolchain edits too (§8) |
| G12 | nit | §2 named a lineage line the lock does not hold; the summary read lineage from ledgering commits | both corrected: lineage is read from every earlier commit's records (§2, summary) |
| G13 | nit | "every package in the repository passes" was false: the spike declares `[features]` | the rules bind only packages a record reaches, and the measurement is of `rt-core`, the one the slice names (§3) |
| G14 | nit | a new target is not a contract edit for a `(targets any)` record | stated: only the behavioral review goes stale, and a cost on the new target is a timing edit (§8) |
| G15 | nits | "checkout" for the written index; shell-special `.env` keys; where a `/` value resolves; a committer date the author controls | "written index" throughout; keys take one of six prefixes no shell variable has; a `/` value resolves from the root only; the date check's reach is a limit (§3, §13) |

**Round 8**, `2026-09-30`: all 23 values matched by two routes. Route A was hand-typed inputs through `shasum`, then a
second full run through `openssl`. Route B was a separate parser of the example's text through `hashlib`. The
reviewer re-measured the record's claims about the tree and found them all true: `rt-core`'s attributes and its
plain build's dependency information, the 538 tracked paths, the 14 manifests, and the target's keys. Round 7's G1,
G3 and G4, the row found while answering, and E2, F1 and F5 were judged closed. G2, C5, C9 and E1 were judged
partial at defect level through H1, and G5, C7 and F7 partial and latent. There were 19 new findings. One was a
defect, H1: a procedural macro under a spelling the record did not refuse. H2, H4 and H6 were at defect level. None
needs a §0 premise broken. The verdict was "cannot be accepted as it stands". No answer changes a hash input, so the
example's digests stand. The facts the answers rest on were measured: `cargo metadata --offline --locked` sees 11
packages, none with a `proc-macro` or `custom-build` target and none with a non-path source, and `rt-core`'s
graph of normal and build dependencies reaches no other package; `rt-core` holds no `debugger_visualizer`; the
repository is not shallow and has no `info/grafts`; none of the target's 19 slash-free values names a tracked file.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G2, C5, C9, E1 | partial, defect-level | through H1 | as H1 |
| G5, C7, F7 | partial, latent | through H7, H8 and H19 | as those rows |
| H1 | defect | `crate-type = ["proc-macro"]` or `proc_macro = true` makes a procedural macro that §3 did not refuse, which can emit refused forms from string literals and read files no hash covers | the keys `proc-macro`, `proc_macro`, `crate-type` and `crate_type` are refused in any table, whatever their value. Before anything is built, `cargo metadata --offline --locked` must show no `proc-macro` or `custom-build` target, no non-path source, and the same packages §3's reading reached, both ways. Builds run offline and locked (§3, §11, §13) |
| H2 | gap, defect-level | a squash or rebase re-ledgers a rejection where its hash is stale, and the only way through the gate was to delete it | catalog changes reach `main` by merge commits, and the replay's refusal of a re-ledgered stale rejection is the rule's mechanism. A rejection binds for good once in `main`'s history, and is merged there in its own commit before its facet changes (§9, §13, How to apply) |
| H3 | gap | a rejection that reaches a production record through items could land only by demoting it | a rejection a facet inherits only through items may be answered in the commit that ledgers it. The facet's own rejections, and those it inherits through lineage, still need a parent's ledger (§5) |
| H4 | ambiguity, defect-level | dependency tables named only as headers; dotted, nested and underscore spellings were missed | dependency tables are read by TOML's meaning, Cargo's underscore aliases included, and `cargo metadata`'s graph checks the reading both ways (§3) |
| H5 | gap | an amend passes the gate against the wrong parents, then fails verification for good | stated: the gate's verdict on an amend is advisory, `M2.7.4` re-runs it after the commit against the commit as made and reports a mismatch at once, and CI's replay decides. "Can only refuse more" was wrong and is withdrawn (§4) |
| H6 | ambiguity, defect-level | a line's facet and verdict were never checked once the record was gone | verification also checks the line's facet and verdict against the form, and status reads facet, verdict and answers from the form only (§5, §11) |
| H7 | gap, latent | the image's closure skipped step 1 | step 1 applies to the image's closure (§7) |
| H8 | gap, latent | the image's build record left out the environment and environment dependencies | it carries the environment, which must be the gate's allowlist with its values, and each environment dependency, held to the gate's rule (§7) |
| H9 | ambiguity | who runs §3's build checks, and over which units | the gate; CI on every replayed commit that changes a file those checks read; a production claim on the commit it reads; each over every unit the build compiles (§3) |
| H10 | ambiguity | a `.env` value under `catalog/` would make the lock a target file | refused (§3, §11) |
| H11 | gap | a contract's only item was its forms hash, which a copy with a new `origin` escaped | a contract's guarantees and preconditions are items, and its forms item leaves out `source`, `maintainer` and `supersedes` (§5) |
| H12 | ambiguity | §8 understated what a change to a production record moves | it names the whole set, every facet whose derived lines reach the edited one, each reviewed in the same commit or its record demoted (§8) |
| H13 | gap | shallow history, grafts and the commit-graph file | every reader refuses a shallow repository and an `info/grafts` file, and reads parents with `core.commitGraph=false`; CI checks out the full history (§4, §11) |
| H14 | gap | the claim did not record which repository `origin` is | it records `origin`'s URL and refuses one other than the canonical URL its tooling holds (§4, §7) |
| H15 | nit | `debugger_visualizer` also reads a file named in an attribute | refused as a word; the sentence now covers any file an attribute or macro made the compiler read (§3) |
| H16 | nit | "in the ledger at `HEAD`" does not fit a merge or the replay | "a parent's ledger" (§5) |
| H17 | nit | a UTC date refused honest reviewers east of UTC | the date is compared in the offset the ledgering commit records (§5) |
| H18 | nit | a slash-free value naming any tracked file turned it into a target file | such a value is refused; a file a target reads is named with its path (§3, §11) |
| H19 | nits | "no caller reads the working tree"; whether reached-set files are "a file of an implementation"; a lone `.env`; the closure's definition inside the `.env` paragraph; a legacy `rust-toolchain` file | the one working-tree read is named; own or reached set; a lone `.env` or `.eadl` is refused; the definition has its own paragraph; the legacy file is refused (§3, §4, §7, §11) |

**Round 9**, `2026-09-30`: all 23 values matched by two routes. Route A was hand-typed inputs through `shasum`. Route B
was a separate parser of the example's text through `hashlib`, which also re-derived the printed hash inputs and lock
lines. The reviewer re-measured every claim the record makes about the tree:
- 542 tracked paths in normal form, with no case collisions;
- every manifest inside the dialect;
- `rt-core`'s tokens, attributes and plain-build dependency information;
- `cargo metadata`'s 11 packages;
- the history neither shallow nor grafted;
- no slash-free `.env` value naming a file.

All were true except the §12 pointer's "verbatim", which I15 corrects. Round 8's defect-level answers, H1, H2, H4
and H6, were closed, and so were G2, C5, C9 and E1. G5, C7 and F7 were closed, latent until `M4`. There were 20 new
findings, none of them a defect. Four ambiguities and gaps at defect level, I1–I4, were all in §12's new composition
names, and none needs a premise broken. The verdict was "cannot be accepted as it stands", with "§3–§9 held". No
answer changes a hash input, so the example's digests stand. The answers to I1–I4 regroup §12's composition names,
and the composition record changes with them (`M2.10.1`).

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| H8, H12, H17, H19 | partial, nits | through I19, I10, I9 and I13 | as those rows |
| H9 | partial, gap | through I6 | as I6 |
| H13, H14 | partial, premise 2 | through I8 | as I8 |
| I1 | ambiguity, defect-level | §12 fixed no facet for the composition's seven facts, so a hardware fact in a timing model escaped the target's files | one table in §12 lists every fact the variant and the composition read, with its facet (behavioral, apart from `eager-switching`), what it is about and its group; a fact in another facet is refused (§12) |
| I2 | ambiguity, defect-level | co-location was checked per record, not per selection, so a fact from one record could meet a cost from another | groups, each with an anchor, checked under every selection; the analysis reads a group's names only from the anchor's record (§12) |
| I3 | gap, defect-level | "the record that supplies the runtime API" was undefined, several names had no pair, and a fact–fact pair was not enforced | the runtime API record is the one that supplies `completion`, and its group holds every `api.<p>` and `masked.<p>`, `masked.completion` and the four API facts; `pending-taken-after-unmask` has its own group (§12) |
| I4 | ambiguity, defect-level | `every-source-external` quantified over a description's sources, which no catalog reviewer sees | replaced by `external.<source>`, a hardware fact per source in its `service.<source>` group; a source with `no` is outside the composition (§12; the composition record's §2) |
| I5 | gap | an answer lifted a rejection for good, whatever items the facet held later | an answer covers the items the facet held at the answering review's hash; a new item binds the rejection again (§5) |
| I6 | gap | CI's build trigger missed record, target and `Cargo.lock` changes | CI builds every replayed commit that changes anything under `catalog/`, `Cargo.lock`, or a file the checks read (§3) |
| I7 | gap | a registry dependency in a member no record reaches blocks every gate run | the whole workspace is path-only, and a member that is not is refused by name (§3) |
| I8 | gap, premise 2 | `GIT_GRAFT_FILE`, a fetch refspec, `insteadOf`, and URL forms | git readers run with an environment allowlist; grafts are found with `git rev-parse --git-path`; the refspec and `insteadOf` are checked; URLs are compared as repositories (§4) |
| I9 | nit | the gate dated its pending commit in UTC | date and offset from `git var GIT_COMMITTER_IDENT` (§4) |
| I10 | nit | §8 left out couplings through reached sets | every facet one of whose bound-hash inputs moved (§8) |
| I11 | nit | "deleting the rejection is not a way through" held only on one branch; merge-only was not a premise | reworded, citing §13; premise 3 names merge commits (§0, §9) |
| I12 | nit | a clean merge runs `pre-merge-commit`, not `pre-commit` | the gate runs from both (§4) |
| I13 | nit | more than one working-tree read | the four are named (§4) |
| I14 | nit | "the environment below"; one `workspace_root` | corrected (§3) |
| I15 | nit | §12's pointer said "verbatim", and "Why" still said the composites were left whole | both corrected (§12, Why) |
| I16 | nit | a primitive named `completion` would collide with `masked.completion` | the name is reserved (§12) |
| I17 | nit | guarantee items compared as written; the over-approximation unlisted | compared by `E` of the string; listed in §5 and §13 |
| I18 | nit | §2 did not point to §12's extension of `holds-under-preemption` | it does (§2) |
| I19 | nit | "the gate's values" cannot hold for the build's own directories | the variables must be the allowlist, with the pin and an empty `CARGO_HOME` (§7) |
| I20 | nit | "names a tracked file" should exclude a directory | said so (§3) |

**Round 10**, `2026-09-30`: all 23 values matched by two routes, the second a separate parser of the example with its
own reader and encoder. Every measured claim about the tree was true, and the reviewer added: `origin` and its
refspec as the record requires, and no `insteadOf`. I1, I3–I8, I10, I11, I13–I15 and I17–I20 were closed; I2,
I9, I12 and I16 partial. There were 18 findings. One was a defect, J1: the eleven platform code facts were
ungrouped, so two honestly reviewed records could put one record's timer facts beside another's costs. And from
`M4`, an image need not compile the implementations the facts were about. The verdict was "cannot be accepted as
it stands". J7 and J8 need premise 3 broken, and each came with a missing cheap check. The answering context
measured `main`'s history for J7 (`origin/main` is the initial commit alone, and local `main` is 232 commits ahead
with no merge) and put premise 3's hosting settings to the director (findings §11). No answer changes a hash input.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| I2 | partial, defect-level | through J1 and J2 | as those rows |
| I9, H17 | partial, nit | through J6 | as J6 |
| I12 | partial, gap | through J5 | as J5 |
| I16 | partial, gap | through J12 | as J12 |
| J1 | defect | platform code facts ungrouped from the costs whose code they state; an image need not compile the closure | the six timer facts go with `timer-service`, the scheduler and sections facts with `completion`, and the trap and transition facts with `switch`; a claim is `not-established` when a closure implementation is not compiled into the image (§7, §12) |
| J2 | ambiguity, fails closed | the group rule under the no-target selection and under targets with no anchor made natural catalogs unloadable | the rule is checked only under target selections where the anchor is supplied, over every named target, `any` ones included; with no anchor the names are unread (§12) |
| J3 | gap | nothing stated that all sources share one controller | the target-level hardware fact `one-external-controller` (§12; the composition record's §2) |
| J4 | gap | §12 held only the task half of `leaves-interrupt-hardware-alone` | the full wording and the caller's declaration (§12) |
| J5 | gap, fails closed | `pre-merge-commit` may run before `MERGE_HEAD` exists | `M2.7.4` measures it; until then catalog merges use `--no-commit` then `git commit`, and the `pre-merge-commit` verdict is advisory (§4) |
| J6 | ambiguity | `git var` under the history allowlist loses `TZ` and `GIT_COMMITTER_DATE` | it runs in the hook's own environment (§4) |
| J7 | gap, premise 3 | the premise named properties, not settings; `main` has never been merged into | the settings named; a claim refuses a non-merge catalog commit on `origin/main` (§0, §7); the settings put to the director (findings §11) |
| J8 | gap, premise 3 | §5's ledger-time checks ran only in bless, the gate and CI | the loader re-applies them at every ledgering commit (§0, §5) |
| J9 | ambiguity | one review or several to cover a rejection's items | a single covering review (§5) |
| J10 | ambiguity | `external-before-timer`'s `no` against the `no` rule | exempted in the composition record (§2) |
| J11 | nit | §11's co-location wording predates groups | reworded by groups, facets, the reserved name and self-statements (§11) |
| J12 | gap | "no primitive named `completion`" enforced nowhere | a cost `api.completion` is refused, and the composition refuses such a primitive (§11, §12) |
| J13 | nit | §8's moved set overstated | each describer's behavioral model, each measurer's timing model, both of each dependent (§8) |
| J14 | nit | the path-only rule is a tightening, not a restatement | said so, and recorded in `decision_zero-dependency-engine-core.md` (§3) |
| J15 | nit | ignored files under `catalog/` | listed with `git ls-files --others`, no exclusions (§4) |
| J16 | nit, premise 2 | other refspecs mapping to `origin/main`, negative refspecs | refused (§4) |
| J17 | nit | rebase, cherry-pick, `am` and revert run no hook | said so; CI's replay decides (§4) |
| J18 | nit | a caller's `C_s` against the grouped source facts | every declared source needs a record anchoring its service (§12; the composition record's §6) |

## Why

The decision record states the design as it stands. Its reviews are its history: every round appends a table,
and a table's answers cite sections by number. Kept in the decision, the history would grow with every round
until it crowded out the design it reviews. Kept here, each stays readable, and the decision keeps one line per
round.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in
  `decision_catalog-records.md`'s `## Review`.
- A finding is answered in the decision record first, and its row here names the sections that answer it.
