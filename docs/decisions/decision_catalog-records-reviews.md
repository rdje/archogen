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

## Why

The decision record states the design as it stands. Its reviews are its history: every round appends a table,
and a table's answers cite sections by number. Kept in the decision, the history would grow with every round
until it crowded out the design it reviews. Kept here, each stays readable, and the decision keeps one line per
round.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in
  `decision_catalog-records.md`'s `## Review`.
- A finding is answered in the decision record first, and its row here names the sections that answer it.
