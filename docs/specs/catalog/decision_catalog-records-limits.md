# Catalog records: the limits

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the Rust toolchain](../../book/src/ledger.md#rust-toolchain) — `rustc`, `cargo` and
  `rustup`, whose version, scope and limits are in the ledger
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`). This is §13 of [[decision_catalog-records]], moved out of it
  in the change that answered its eleventh review, which took that record to 1 182 of the 1 200 lines
  `README-ROUTES` allows a file. It is part of that record: normative, numbered as its §13, and reviewed with it. That
  change's edits to §13 are in the review history.

## The fact / decision

### 13. Limits

- **Reviewer independence is asserted, not verified.** Roles and task trees cannot coincide, and `who` cannot be
  the maintainer. That is all the structure guarantees. Git authorship cannot help, because one identity commits
  every commit in this repository but the template's initial one. What a later reader checks is the review's `basis`.
- **`ROADMAP.md` §9's admission evidence per catalog**, beyond which facets are present and not empty, is an
  obligation of the production review's basis: the tests, the reference behavior, the exact revisions and the
  conformance evidence. Nothing checks it mechanically.
- **Bump size is not checked.** A patch-level bump over a breaking change passes. The lock checks that a version
  moved, not by how much.
- **Hashes over-approximate.** Any of these voids reviews that its bound hashes reach, which is sound and costs a
  review:
  - a comment change in a package;
  - a workspace lint;
  - any edit to a target's `.env`, including `TARGET_VERIFIED_BY`, which voids every review on that target;
  - a ledger section's wording;
  - a contract edit, which voids both models' reviews;
  - a new target, which voids the behavioral reviews of every `any` record;
  - a model package that depends on another model's package, which moves both bound hashes;
  - a record moved to `experimental`, which affects every production claim that read it;
  - a file whose bytes an unrelated facet also holds, which carries a rejection to it (§5);
  - a rejected cost, which reaches the same-named cost on every target of the same kind and Rust target (§5);
  - a rejected contract, which reaches every contract that states one of its guarantees or preconditions (§5).
- **What no hash covers:**
  - the installed compiler's bits (the channel in `rust-toolchain.toml` is covered);
  - cargo configuration outside the repository. The gate refuses any it would read and gives cargo an empty
    `CARGO_HOME` (§3); the build of an image is `M4`'s to hold to the same rule, which §7's build record shows;
  - the installed toolchain's files, git's local configuration and refs, and the hosting's protection of `main`.
    These are §0's premises, each with what is checked of it;
  - the meaning of a profile. A record names a profile by its id, and a changed meaning moves the id (`ROADMAP.md`
    §15), which the version register records;
  - `--config` flags;
  - rustup's per-directory overrides;
  - `Cargo.lock`. With every dependency a path one, it adds nothing the sets lack, and §3 refuses the first
    non-path dependency;
  - the invocation scripts of an emulator target (the files its `.env` names are covered).

  A claim's image is what covers these for a built system. So §7 lets only `independent` costs, and costs from
  the claim's own engine-made image, back a production claim. Before `M4`, no claim has an image.
- **A rejection binds items, not meaning.** Content moved into another record without lineage, and changed so
  that no item of §5 matches, whether a name, an entry, a file's bytes or the forms, is new content for review. The
  ledger is where a reviewer of related content looks, and nothing forces the look.
- **The twelve port facts §14.2 names are known only with a locator into declared assembly.** Until `M2.12` they were `unknown`
  by name, since §3 refused assembly. §14.2 admits the port's assembly in a package an `assembly` declaration names,
  and restates the rule by what a locator reaches: each of the twelve facts §14.2 names — the `switch` group's eight
  of §12, and the port's half of `preemptive-everywhere` (the trap exit that performs a decided switch),
  `sections-mask-every-interrupt`, `releases-never-latched` and `primitives-out-of-line` — is known only as §14.2
  states, and refused known otherwise (`catalog-field`); §14.4's port statement, also in the `switch` group, is not
  held to it. A fact whose basis rests on the port's code too, as
  `acknowledge-at-entry.<source>`, `one-request-per-arrival.<source>` and `raised-only-when-due`'s may, should name
  the port's record in `describes` and carry a locator into it; the loader checks nothing more there, and that its
  locators reach all the code its basis rests on is the review's. Until a record with such a declaration exists, every analysis of the runtime variant over the
  catalog is still `analysis-inconclusive`, naming these facts.
- **A bad line on `main` is repaired only by a waiver.** The lock is append-only and `main` is never rewritten, so a
  line that fails verification would stop the catalog loading for good in every clone that fetches it. It has two
  causes. One is a failure of premise 3. The other is a defect in the `/1` verifier: its fix refuses every line the
  faulty one passed, lines landed before premise 3's named commit included, and "a rule tightened later" (§5) does
  not cover a corrected implementation of the same rule. The repair is §9's waiver, which lets the catalog load and
  can only lower a status. Premise 3 and its checks keep the first cause away, and the verifier's tests (`M2.7.3`,
  `M2.7.4`) the second.
- **The checker is the repository's own code.** The loader, the gate, CI and what they are built with judge every
  catalog change. A pull request cannot change the checker that judges it, since the check builds the checker from
  the base (premise 3), and the checker's closure is under its code owners' review. A checker change reviewed in
  error judges every catalog change after it: the protection reaches as far as that review, and no further.
- **A rejection binds for good once it is in `main`'s history.** Until then it binds the branch it is on, and is
  lost with it: a reset of an unpushed branch, a pushed branch deleted and pushed again as new, or one
  force-pushed without it and pushed again. So a reviewer's
  rejection is merged to `main` in its own commit before the facet it names changes (How to apply). One that
  reached `origin/main` binds every production claim made from a clone that has fetched it, whatever branch the
  claim reads (§4), and while `main` is protected (premise 3).
- **The token refusals are of what is written, and what is written is what is compiled.** No package in the build
  defines a macro: `macro_rules` and `macro` are refused as words, and a procedural macro by its manifest and by
  what `cargo metadata` resolves. A macro invocation's arguments hold none of the refused words, and the scan
  covers every file the compiler's dependency information lists (§3). What they rest on
  beyond that is premise 1: the toolchain's own macros and attributes are the pinned release's. For an image, the
  dependency information in its build record covers every Rust source it compiled, each with its hash.
- **A review's date is checked against a date its author sets.** A committer date is the committer's to choose, so
  the check (§5) catches a date that is malformed or later than its commit, not a commit dated falsely.
- **A code locator is necessary, not sufficient.** It proves that a fact points into code its bound hash covers.
  That the fact depends on no other code is the review's to check.
- **Where evidence was obtained is the review's to check.** Admission refuses costs on a target that is not a
  board. A fact, or a cost labelled with a board, whose evidence came from an emulator shows only in its basis.
- **One image per target at a time.** A cost name appears once per target. So only the image its costs came from
  can hold production claims on that target, and the next image's measurement replaces them. Keying costs by
  image is `M4`'s.
- **No surface makes a production claim yet.** Strength is a parameter of admission (`M2.7.3`), and the runtime
  variant passes it through (`M2.7.5`). The report that states a production claim to a user is `M4`'s, and so is
  how a product reads history (§4). Until then the production rule is exercised by tests and the repository's
  tooling.
- **Review granularity is the facet.** One reviewed cost and one unreviewed cost in a single timing model leave
  the facet without a production review at its hash. Splitting the record is the way to review part of it, and
  `supersedes` carries its rejections across; a production verdict never crosses, since a review names its id.
- **Nothing stores a claim yet** besides tests. §10's answer is only as good as the closure a stored claim keeps.
- **`C_i`, `CS_i`, `J^release`, `J_s`, and `C_s` without its fact, are not the catalog's in `/1`** (§12). The
  first four are composed from the parts §12 names (`decision_runtime-composite-inputs.md`). Until `M2.10.2`
  implements that, and for `C_s` without its fact in any case, the variant's soundness for them rests on the
  caller's figures. No production claim can
  rest on those figures (§7).

## Why

A design's limits are what it states it does not do. They grow with each review round that finds a residue worth
naming, so they are kept apart, like §12, and a reviewer of the record is given both.

## How to apply

- A bare section number here is the catalog record's. One that is `ROADMAP.md`'s says so.
- A limit that a later leaf lifts names that leaf, and leaves this section when the leaf closes.
