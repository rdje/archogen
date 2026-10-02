# Catalog records: what one holds, how it is hashed, and when it may back a claim

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the Rust toolchain](../../book/src/ledger.md#rust-toolchain) — `rustc`, `cargo` and
  `rustup`, whose version, scope and limits are in the ledger
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`), deciding what `ROADMAP.md` §9 requires of a catalog entry
  before any code for it is written: "an ID, semantic version, content hash, source/license metadata, maintainer,
  dependencies, supported profiles, preconditions, guarantees, implementation source, model source, cost evidence,
  and evidence status". It also covers that section's two rules. "Unknown or unreviewed data may exist in an
  experimental namespace but cannot silently satisfy a stronger production claim." And "Behavioral and timing models
  may have separate versions because their changes invalidate different claims." The reviews, and the answer to each
  of their findings, are the last section.

## The fact / decision

A catalog record is a tracked ASCII file under `catalog/`, written in the eADL reader's datum syntax and read by
that reader alone. A record has four **facets**: its contract, its implementation, its behavioral model and its
timing model. Each facet has two computed hashes, and neither is ever written:

- its **own** hash covers the facet's own forms and the files under the entries it names. It pins the facet's
  version in an append-only lock;
- its **bound** hash adds everything else the facet rests on (§3):
  - for a contract, its dependencies' contracts;
  - for an implementation, the files its packages reach: their dependencies, the workspace manifest and the
    toolchain files;
  - for a behavioral model, its record's contract, the code it describes, its dependencies' behavioral models, its
    targets (every target, for `any`), its ledger sources and the files its own packages reach;
  - for a timing model, its record's contract, the code whose costs it states, the timing models of its
    dependencies and of its `measured-with` records, the targets of its costs, its ledger sources and the files its
    own packages reach.

A facet's **evidence status is derived, never written**. It comes from reviews, whose verdict is `production` or
`rejected`:

- a production verdict holds only at the bound hash it names, so any edit to what it covered voids it. It also
  lapses while a rejection names that same hash, until a review at that hash answers the rejection;
- a rejection stands, whatever the content becomes, until a later review answers it by name. It binds the facet,
  every record that supersedes the rejected one, and any facet of the same kind that holds content it covered: the
  same fact or cost, the same source entry, a file with the same bytes, or the same forms;
- every review is kept in an append-only ledger with its facet, verdict and hash. What a rejection covered and
  what a review answered are read from the commits that ledgered them, and which record succeeded which from every
  earlier commit's records. None is written a second time, and none can be taken back, reordered or renamed away.

A claim's citations are **computed from what it read**. Its closure is exactly the facets whose hashes enter those
it read, with each of their records' contracts, and the result records each one's bound hash and status. That is how
a later change, rejection or demotion is traced to it.

A **production claim** is admitted only when every catalog input comes from a production record, the claim's
own engine-made image stands behind every image-dependent cost, and that image was built only from reviewed code
of production records, every package it compiled included.
The description's inputs are the description's own. Nothing may come from the caller, no cost it reads may be on
a target that is not a board, the image's own build must show it read only reviewed files, and the catalog it
reads must be a commit whose ledger holds every line its history and the published main line hold. A record may
live in the production namespace only while each of its four facets is `production`.

The behavioral and timing models are separate facets with separate versions, and neither's hash covers the
other. So a change to one invalidates only the claims that read it. Both rest on their record's contract, so a
change to the contract invalidates both.

What this defends against, and what it assumes, is §0. A bare section number is this record's own; one of
`ROADMAP.md`'s is written with its name.

### 0. What it defends against

The director ruled this model on `2026-09-30` (leaf `M2.7.1`), after six review rounds whose later findings mostly
assumed control of the build machine or the hosting.

- **In scope: every change that reaches the catalog through the repository.** Tracked files, the lock, commits,
  merges and pushes, made by anyone, an author who bypasses the local gate or edits the lock by hand included.
  Each such change is refused, or makes the affected reviews stale and the affected claims affected (§10). A
  construction of that kind that succeeds is a defect in this record.
- **Premises.** Each is named with the check the gate runs where a cheap one exists. A construction that needs a
  premise broken is outside the model: it is answered where the answer is cheap, and §13 lists what remains.
  1. **The toolchain is the pinned release, unmodified.** Checked: the pin is a release number, never a moving
     or nightly channel; the gate runs with `RUSTUP_TOOLCHAIN` set to it, so no rustup override applies, and
     refuses when `rustc -vV` names another release; an image's build record carries the same line (§3, §7). Not
     checked: that the installed files are the release's bytes.
  2. **Git's local state reports the repository as it is.** No replace object, no remote-tracking ref set by hand.
     Checked: every reader sets `GIT_NO_REPLACE_OBJECTS`, and the gate writes each file from its blob itself, so
     no filter, attribute or line-ending setting applies (§3).
  3. **The published main line is protected.** The hosting settings this means, which the director confirms:
     - changes reach `main` only through pull requests, administrators included;
     - the required checks run on the merge result, with branches up to date or a merge queue;
     - merge commits are the only merge method;
     - `main` is never force-pushed and never deleted;
     - the required check is a workflow the hosting's ruleset requires, pinned outside the pull request's tree,
       with its job name reserved, since every workflow a pull request runs reports from the same app;
     - **the check is protected from everything it judges.** No code a pull request controls runs in the check
       or reaches its credentials, no file of it lies on the path of anything the check builds from the base or
       runs, and nothing but the check can post its verdict. **The judged tree is data:** the check reads its
       catalog, history and files, builds its record packages only under §3's rules, after §3's configuration and
       manifest checks, into target directories of their own, and runs nothing it builds from it. The checker is
       built from the base commit, the one the pull request merges into. Its closure is the packages it is built
       from, as `cargo metadata` resolves them at the base, with the root manifest, `Cargo.lock`,
       `rust-toolchain.toml`, `.cargo/`, `.github/` and `scripts/`. Code owners other than the author review it,
       and the claim-side checks below compare that set: measured on `2026-09-30`, `xtask` reaches
       `archogen-api`, `eadl-front` and `eadl-model`, and the catalog crate and `archogen-evidence` join them with
       `M2.7.3`. One identity commits every commit here today (§13), so until the director names a second, the
       premise is unmet (findings §11).

     **How the check achieves that is `M2.7.6`'s design**, reviewed on its own and tested adversarially: every
     construction the catalog's reviews found against it is a test whose program must never run. They are build
     scripts and cargo configuration in a judged tree, where the base and judged trees sit, a target directory
     shared with the record builds, where the ownership file lives and what it means, rustup's toolchain file, the
     workflow's token, stale approvals, a merge queue, and shared runners and caches (rounds 12 to 15). A
     construction that gets past it is a failure of premise 3, and a defect of `M2.7.6`, not of this record.

     The premise holds from a named commit of `main`, the first after those settings were confirmed. The director's
     confirmation names it (findings §11), and the claim tooling holds it as a constant beside the canonical URL
     (§4), refusing one that is not on `origin/main`'s first-parent chain. Nothing before it is judged by it.

     Checked where it is cheap:
     - until that commit exists, a production claim is `not-established`, naming that premise 3 has no named
       commit (§7);
     - a production claim is `not-established` when a first-parent commit of `origin/main` after that commit is
       not a merge commit the hosting made: committed as the hosting and signed with one of its keys, which the
       claim tooling holds as constants beside the canonical URL, each with the first-parent range it signs. A key
       the hosting rotates in is added by a reviewed change (§7);
     - a production claim's tooling is built under §3's environment and configuration rule from a written tree of
       a first-parent commit of `origin/main` at or after the named commit, never from a working tree, and records
       that commit. The claim is `not-established` when the checker's closure changed along `origin/main`'s
       first-parent chain between that commit and the `origin/main` commit the claim records (§7);
     - the loader re-applies §5's ledger-time checks at every ledgering commit, so a review that CI failed to check
       is still refused (§5).

     A claim records the `main` commit it was checked against, so a reader sees what it rests on, and §10 flags a
     claim whose commit has left `main`'s history.
  4. **The machine running a check is not working against it.** Its sysroot, rustup's settings and its `PATH` are
     as installed. Premise 1's checks catch an honest mismatch, not a deliberate one.

### 1. Where a record lives and how it is written

- **One record per file**, `catalog/<namespace>/<id>.catalog`.
  - The namespace is the directory, `production` or `experimental` (§6), and no record restates it.
  - The file stem is the record's id, and ids are unique across both namespaces.
  - The only other file allowed under `catalog/` is `catalog/catalog.lock` (§9). Any other file or directory
    there is a refusal, so a misspelled extension cannot hide a record.
  - Every record is tracked (§4). An untracked one is a refusal.
- **An id** is dotted segments. Each segment is a lowercase ASCII letter, followed by lowercase letters and
  digits with single `-` between them: no leading, trailing or doubled `-`. For example,
  `rt.scheduler.fixed-priority`.
- **The syntax is the eADL reader's datum layer**: lists, symbols, integers, strings and `;` comments, read by
  `eadl_front::read` and nothing further. Decimals are refused in `/1`.
  - A record is **not** an eADL description: it is not type-checked, not a `.eadl` file, not in the conformance
    suite's population and not in the language baseline. `ROADMAP.md` §5.6 puts engine knowledge in "versioned
    engine catalogs or plugins … even when they are stored in a declarative engine-specific format".
  - Every reader diagnostic is a refusal.
- **ASCII by construction.**
  - Every byte of the file, comments included, is printable ASCII (`0x20`–`0x7E`, the space included) or a line
    feed.
  - Every string's **decoded** value is printable ASCII too, so an escape such as `\n` or `\u{202E}` is refused.
  - So nothing a reviewer reads can hide a control, bidirectional or zero-width character, and no hash input can
    hold one.
- **Exactly one top-level form**, headed `catalog-record`, then the id, then the fields of §2 in §2's order, each
  exactly once, then the reviews, which may repeat.
  - Inside a facet, a fact, a cost, a locator and a review, the subforms also come in §2's order.
  - A subform §2 marks optional may be absent. Anything unknown, missing, duplicated or out of order is a
    refusal; a code fact's `locator` alone may repeat, as §14.2 states.
- **Every field keeps its source location**, since the reader gives each form a span, and a diagnostic points at
  the field. Where a fact or cost came *from* is its `locator` (§2).

### 2. The fields

**The contract.** These fields are the record's contract facet, and its `version` is the record's semantic
version.

| `ROADMAP.md` §9 field | Form | Rule |
| --- | --- | --- |
| ID | the symbol after `catalog-record` | §1's grammar, equal to the file stem |
| semantic version | `(version "MAJOR.MINOR.PATCH")` | three decimal numbers, each `0` or without a leading zero, and no suffix. It is **the contract's version**. Each other facet has its own |
| — (which of `ROADMAP.md` §9's four catalogs) | `(catalog algorithms)`, `machine`, `devices` or `interfaces` | `ROADMAP.md` §9's table. It decides what production requires (§6) |
| source/license metadata | `(source (origin "…") (license "…"))` | both non-empty. `origin` says where the content came from, exactly enough to find it again: this repository and the leaf that wrote it, or an outside source with a ledger section (`SOURCE-LEDGER`) |
| maintainer | `(maintainer <tree-id>)` | the task tree that owns the record, such as `M2`: an uppercase letter, then uppercase letters and digits |
| dependencies | `(depends (<id> "<MAJOR.MINOR>") …)`, or `(depends)` | a requirement on the dependency's **contract** version, with Cargo's caret meaning. `"1.2"` is at least `1.2.0` and below `2.0.0`. With major `0`, the minor is the boundary: `"0.3"` is at least `0.3.0` and below `0.4.0`. The catalog holds one record per id, so there is at most one candidate and nothing to choose. An id appears once, must resolve and must match, and the graph has no cycle |
| — (lineage) | `(supersedes <id> …)`, or `(supersedes)` | ids this record replaces. Each must have ledger lines and no present record. Once a commit holds it, it is permanent, since §5 reads the lineage from every earlier commit's records: the record cannot drop it, and no record may take a superseded id again. A superseded id's unanswered rejections pass to every record that supersedes it, directly or through a chain (§5). A rename or a split cannot shed a verdict through lineage, and §5's items bind what moves without it |
| supported profiles | `(profiles <profile> …)` | at least one, each a profile the engine supports (`eadl_model::profile::supported`) |
| — (where the record applies) | `(targets any)`, or `(targets <target-id> …)` | a named target is a stem with both `targets/<t>.env` and `targets/<t>.eadl`, directly under `targets/`, and the stem is lowercase ASCII letters and digits with single `-` between them. The `.env` follows §3's grammar and gives `TARGET_KIND`, which is `emulator` or `board`, and `RUST_TARGET`. Every cost's target must be one this field admits. `any` admits every target, and binds the behavioral model to every target under `targets/` (§3), so a new target makes its reviews stale |
| preconditions | `(preconditions "…" …)`, or `(preconditions)` | each a non-empty sentence. An empty list states that there are none, and a reviewer checks that |
| guarantees | `(guarantees "…" …)` | at least one: a record that guarantees nothing is not an entry |

**The other three facets.** Each is one form, always present, with its own version. `none` states that the facet
does not exist, and why, and it is reviewed like any other statement (§5).

| `ROADMAP.md` §9 field | Form |
| --- | --- |
| implementation source | `(implementation (version "…") (sources "<package>" …))`, with at least one entry, each a package (§3), optionally followed by `(assembly <architecture> "<package>" …)` naming those of them that hold assembly (§14.2); or `(implementation (version "…") (none "why"))` |
| model source, behavioral | `(behavior-model (version "…") (sources "…" …) (describes <id> …) (facts <fact> …))`, or `(… (none "why"))` |
| model source, timing, and cost evidence | `(timing-model (version "…") (sources "…" …) (measured-with <id> …) (facts <fact> …) (costs <cost> …))`, or `(… (none "why"))` |

- **`describes`** names the records whose code the behavioral facts are about, beyond the record's own
  implementation. For example, an interface record whose code lives in another record's package. Each must
  resolve.
- **`measured-with`** names the records whose code is inside the timing model's costs, or whose timing figures a
  cost is derived from, beyond the record's dependency closure. Each must resolve.
- An empty list names none.
- **A code fact** is one that §12 lists as about code. Its locators are `code` locators into the code the fact is
  about, as the locator rule below admits it for the fact's facet: in a behavioral model, the record's own
  implementation or a `describes` record's; in a timing model, `eager-switching` included, also a record in its
  dependency closure or a `measured-with` record.

**A fact** is `(fact <name> yes|no (locator …) (basis "…"))`, or `(fact <name> (unknown "why"))`. A code fact may
carry several `(locator (code …))` subforms, one after another, no two the same, each admitted by the rule below
(§14.2).

**A cost** is `(cost <name> (target <t>) (unknown "why"))`, or a known value:

```text
(cost <name> (target <t>) (value <n>) (unit ns|us|ms) (scope "…") (holds-for (tasks <n>) (sources <n>))
      (holds-under-preemption yes|no) (binary …) (evidence …) (locator …) (basis "…"))
```

- **`target`** is a named target that the contract's `targets` admits, for known and unknown costs alike.
- **Integers.** `value` and every other integer in a cost is non-negative. The reader's integers are signed
  64-bit, so each lies in `0 … 2^63 − 1`.
- **`holds-for`** gives the largest number of tasks, and of declared interrupt sources other than the timer,
  that the value holds for. The timer service's cost, for instance, grows with the tasks it releases.
- **`holds-under-preemption`** is the runtime variant's condition 8 for this one value: whether it holds under
  any preemption pattern. The statement sits on the value it vouches for, so no other record's fact can vouch
  for it. For the costs the composition of the variant's inputs adds, it also says the value holds from any state
  the code before it leaves (§12).
- **`binary`** takes one of three forms, and §7 decides which of them may back a production claim:
  - `(binary "sha256:<hex>")`, the image the value was obtained on;
  - `(binary unbuilt)`, when there was no image;
  - `(binary independent "why")`, when the value depends on no image. It is admitted for exactly two names,
    `compare-rounding` and `delivery` (§12), and only on a target whose `TARGET_KIND` is `board`. Every other
    name, and every emulator, refuses it.
- **`evidence`** is one of `ROADMAP.md` §7.3's four categories:
  - `(evidence assumed)`;
  - `(evidence observed-maximum)`;
  - `(evidence observed-maximum (observed <n>) (safety-factor <numerator> <denominator>))`;
  - `(evidence externally-supplied)`;
  - `(evidence analytically-established)`.

  With a safety factor, `observed` is the raw observation, and `value` must equal ⌈`observed` × numerator /
  denominator⌉, computed exactly in 128 bits. The factor's terms are `u32` and must pad: the denominator is nonzero,
  the numerator is at least the denominator, and a term outside `u32` is refused the same way. As `ROADMAP.md` §7.3
  says, the factor changes nothing about the category.
- **`locator`** is optional for `assumed` and required otherwise.
- **A known cost becomes an `archogen_evidence::Bound`**, which must pass `Bound::validate`.
  - `quantity` is the name, and `value`, `unit`, `scope` and `target` map as named.
  - `binary` is `sha256:<hex>`, `unbuilt`, or `independent: <why>`.
  - The origin, with `L` as the rendered locator and `B` as the basis:

    | Category | Origin |
    | --- | --- |
    | `assumed` | `Assumed { rationale: B }`, or `"L: B"` when a locator is present |
    | `observed-maximum` | `ObservedMaximum { conditions: "L: B", safety_factor }` |
    | `externally-supplied` | `ExternallySupplied { source: L }` |
    | `analytically-established` | `AnalyticallyEstablished { argument: "L: B" }` |

  - `L` is `file:<path>`, `code:<id>:<path>` or `ledger:<anchor>: <detail>`.

**A locator** takes one of three forms:

- `(locator (file "<path>"))` names a file in the facet's own source set (§3), so the facet's own hash covers it;
- `(locator (code <id> "<path>"))` names a file in record `<id>`'s implementation own set, so the facet's bound
  hash covers it through `implementation <id>` (§3). In a behavioral model, `<id>` is the record itself or a
  `describes` record. In a timing model, it is the record, a record in its dependency closure, or a
  `measured-with` record. A code fact takes this form and no other;
- `(locator (ledger <anchor> "<revision>, <section>"))` names a section of `docs/book/src/ledger.md`. The section
  is the part from the heading `` ## `<anchor>` `` up to the next line beginning `## `, or the end of the file.
  It must exist and be the only heading with that anchor, and its bytes are in the facet's bound hash (§3).

A `basis` is never empty.

**Names.**

- Facts and costs share one name space per record.
- A name is lowercase letters, digits, `-` and `.`.
- A fact name appears once per record, and a cost name once per target.
- §12 fixes each runtime variant name's kind (fact or cost), its facet, whether a fact is about code, and whether
  a cost admits `independent`. The same name as the other kind, or in the other facet, is a refusal, so which hash
  covers it is not the author's choice.

**Reviews** come after every field, zero or more, each written as follows (§5):

```text
(review (facet <facet>) (hash "sha256:…") (verdict production|rejected) (answers "sha256:…" …)
        (by <role> "<who>") (date "YYYY-MM-DD") (basis "…"))
```

`answers` is optional, and allowed only on a `production` verdict.

**Nothing defaults.** A cost or a fact nobody knows is written `unknown`, with the reason. An analysis that needs
it refuses (§12).

### 3. Hashes: a normative byte grammar

This section is kept in [`decision_catalog-records-hashes.md`](decision_catalog-records-hashes.md). It moved there
when this record neared its size ceiling, with round 14's T1 and T15 edits and nothing else. It is part of this
record, normative and reviewed with it: the byte grammar every hash is computed over, what each facet's own and
bound hash covers, the package rules and the manifest dialect, what the gate builds and lets cargo read, and the
worked example that pins them.

### 4. Paths, "tracked", and what reads the files

- **A path** is relative to the repository root and written in normal form: segments of ASCII letters, digits,
  `.`, `_`, `+` and `-`, joined by single `/`, with no empty, `.` or `..` segment and no leading or trailing
  `/`.
  - Every tracked path in the repository satisfies this as of `2026-09-30`: `git ls-files -z` lists none that
    does not.
  - The rule applies to paths written in a record and to paths produced by expansion. A path outside it is
    refused.
  - A written path appears at most once in each list it is written in: a facet's `sources`, and an `assembly`
    declaration's packages. A declaration's packages are, byte for byte, entries of the facet's `sources` (§14.2).
    Two facts may locate one file, and so may two locators of one fact naming different records.
- **Tracked** means present in the git index with mode `100644` or `100755`, matched byte for byte, so case counts
  even on a case-insensitive file system. Refused:
  - a symbolic link (`120000`);
  - a submodule entry (`160000`);
  - an untracked path;
  - a directory with no tracked file under it.
- **The loader is I/O-free.** Its caller gives it the tracked set and each file's bytes, and a **history**: for
  each commit the loader asks about, its parents, its committer date, and its tracked set and bytes. Status reads
  the commits that ledgered each review (§5), and lineage reads every earlier commit's records, so every load has
  a history. The caller that runs git is the repository's own tooling under `xtask/`, outside `NO-SUBPROCESS`'s
  population, since no product crate spawns a process. How a product reads history is `M4`'s, and until then a
  production claim is made only by that tooling and by tests (§13). Tests give a history in memory.
  - **The gate's pending commit.** The gate reads the index, so the commit being read does not exist yet. Its
    parents are `HEAD`, and `MERGE_HEAD` too while a merge is in progress. Its committer date and time-zone offset
    are those `git var GIT_COMMITTER_IDENT` gives when the gate starts, run in the hook's own environment rather
    than the history readers' allowlist, since it reads no history and needs `TZ` and `GIT_COMMITTER_DATE`. So the
    gate reads a review's date as CI's replay will (§5). The gate runs from both `pre-commit` and
    `pre-merge-commit`, since a merge that git records without conflicts runs only the second, and §9 routes
    catalog changes to `main` by merges. `pre-merge-commit` may run before git writes `MERGE_HEAD`, and then the
    gate sees one parent; `M2.7.4` measures which. Until measured, a catalog merge is made with
    `git merge --no-commit` and then `git commit`, whose `pre-commit` sees both parents, and the gate's verdict from
    `pre-merge-commit` is advisory. Rebase, cherry-pick, `am` and revert make commits without running either hook.
    For them, as for a bypassed gate, premise 3's check decides. A pre-commit hook cannot tell an amend from a new
    commit. For an amend, the real parents are
    those of the commit it replaces, so the gate's verdict is advisory: it can pass a commit that its real parents
    make a ledgering commit that fails verification. `M2.7.4` runs the gate again after the commit, against the
    commit as made, and reports a mismatch at once; the repair is to reset to the commit the amend replaced. CI's
    replay, which judges each commit as it was made (§9), decides.
- **No caller reads a catalog input from the working tree.** Each takes the tracked set from `git ls-files -s -z`
  or `git ls-tree -r -z`, and each file's bytes from its blob, so line-ending conversion on checkout does not matter.
  The gate's working-tree reads are four, each named where it is made:
  - it compares the repository's own cargo configuration files with the index (§3);
  - it lists the cargo configuration files on each cargo command's directory path (§3);
  - it tests whether a configuration or toolchain file on a package's ancestor path is present but untracked (§3);
  - it lists the untracked files under `catalog/` (below).
  Tests give files in memory.
  - The gate and an exploratory claim read the **index**. The gate checks what is about to be committed.
  - A production claim reads a **commit**, `HEAD` unless one is named, and records it. Its ledger must hold every
    line of every ancestor commit's ledger and of the published main line's, `origin/main` as last fetched, with its
    ancestors (§9). The claim records that `main` commit too. So a line dropped with the gate bypassed is seen, and
    so is a rejection published after the commit's branch forked, as far as the clone has fetched (premise 3). A
    clone with no `origin/main` is refused. The claim also records `origin`'s URL, read with `git remote get-url
    origin`, and refuses one that is not the repository's canonical URL, which the claim tooling holds as a
    constant. URLs are compared as repositories, not as strings: `https` and `ssh` forms of the same host and path
    are equal, with or without a trailing `.git`. So a fork's `main`, which premise 3 does not protect, is not taken
    for this one. `origin`'s fetch refspec must map `refs/heads/main` to `refs/remotes/origin/main`, no other
    refspec, of `origin` or of any other remote, may map to it, no negative refspec may exclude it, and no
    `url.*.insteadOf` may rewrite `origin`'s URL. Otherwise another branch or another repository could stand behind
    a canonical-looking `origin/main`.
  - **Every reader reads history as it is** (premise 2, with its cheap checks). Each runs git with an environment
    allowlist, as the builds do: `PATH`, `HOME`, the `GIT_DIR`, `GIT_INDEX_FILE` and `GIT_WORK_TREE` a hook is
    given, and `GIT_NO_REPLACE_OBJECTS` set, and nothing else, so no `GIT_GRAFT_FILE`, `GIT_SHALLOW_FILE` or object
    directory override applies. It refuses a shallow repository (`git rev-parse --is-shallow-repository`) and a
    grafts file wherever git would read it (`git rev-parse --git-path info/grafts`), and reads parents with
    `core.commitGraph=false`. A shallow boundary would be a false ledgering commit for every line. CI checks out
    the full history.
  - The gate also lists the untracked files under `catalog/`, ignored ones included (`git ls-files --others`
    without `--exclude-standard`), and refuses them (`catalog-layout`), since no reader of the index can see them
    and a `.gitignore` could otherwise hide a stray record.
- **Without git.** A `ROADMAP.md` §10.3 package carries no index. Loading a catalog there needs the package's own
  manifest of files, which is `M4`'s. Until then, the catalog loads only from a repository.

### 5. Evidence status is derived, never written

A review names a facet, the facet's **bound** hash at the time, a verdict, and its reviewer. A production review
may also name, in `answers`, the ledger hashes of rejections it answers. Status is **order-free**, meaning the
position of reviews in the file decides nothing:

| Status | When |
| --- | --- |
| `rejected` | the facet inherits a rejection, at any hash, and no single production review of the facet answers it while covering every item of the rejection that the facet now holds |
| `production` | otherwise, when a production review names the facet's current bound hash `h`, and every rejection of the facet that names `h` is answered by a production review that also names `h` |
| `stale` | otherwise, when some review names the facet |
| `unreviewed` | otherwise |

**The commit that ledgered a review** is a commit whose lock holds the review's line and none of whose parents'
locks does. It is searched for in the ancestry of the commit being read and, for a production claim and for §10, in
the ancestry of the `origin/main` commit the reading records (§4) too, so a line joined from the published main
line has one. There may be more than one, if two branches ledgered it.

**Each ledgering commit is verified before anything is read from it.** There, the review's form in its record must
hash to the line's ledger hash, the line's facet and verdict must be the form's, and the facet's bound hash must
equal the one the line names. A ledgering commit that fails any of these is a lock edited by hand, and the catalog
does not load (`catalog-lock-review`), unless a waiver names the line (§9). Every ledgering commit therefore held
the same review and the same facet content. The review's form, its `answers` included, and what the facet held are
read from any of them, with the same result. **Status reads a review's facet, verdict and answers from its form**,
never from the line, whose copies are an index. **The loader also re-applies the ledger-time checks below at each
ledgering commit**, with that commit's record, parents' ledgers and committer date. So a review that a bypassed gate
and a neutered CI let through is refused at load (`catalog-review`), not only when it was ledgered, unless a waiver
names it.

**Each ledgering commit is verified under the rules it was ledgered under**, those its lock's first line names
(§9). `# archogen-catalog/1` names both §3's grammar and these ledger-time checks. A later version replaces that
line, with a migration note as `ROADMAP.md` §15 asks, in the first catalog change after a checker that knows both
versions has merged (premise 3). A line ledgered
under `/1` is verified under `/1` for good, so a rule tightened later never refuses history that was honest when it
was made. A corrected implementation of the same rule is not a tightening, and §9's waiver is its repair. **One
identifier names the grammar and the checks**, and every hash begins with it. So any later version, of either,
moves every hash and makes every review stale, and its migration note says how each earlier line is compared under
it. A version that changes the checks alone may first split the identifier in two. The change that bumps it
reviews again or demotes every production record, since every hash moves.

**The rejections a facet inherits** are read from the ledger and the commits that ledgered it:

1. its record's own rejections of that facet, at any hash;
2. that facet's rejections of every id its record supersedes, directly or through a chain. The **lineage** is every
   `supersedes` any ancestor commit's record held, so a lineage once committed cannot be dropped;
3. those whose **items** the facet holds. A rejection's items are read from the facet as the commit that ledgered
   it held it:
   - each fact's name; each cost's name and target, and the cost's name with its target's `TARGET_KIND` and
     `RUST_TARGET`, as that commit's `.env` gave them;
   - each source entry, as written;
   - the hash of each own-set file's bytes;
   - for a contract, each guarantee and each precondition, compared by `E` of the string, so by its decoded value
     and not by how an escape spells it;
   - the facet's forms hash: §3's hash over the identifier the ledgering commit's lock names, `archogen-catalog/1`
     in `/1`, `forms <facet>` and `E` of each of the facet's forms, with every `version` form removed, and for a
     contract its `source`, `maintainer` and `supersedes` forms too, which change with who copied it rather than
     with what it promises. The facet it is compared with is hashed under the same identifier, whatever version
     the catalog has reached since, and every later version keeps each earlier one's forms hash computable.

   A facet of the same kind, in any record, inherits the rejection when it supplies the same fact name, the same
   cost name and target or the same cost name on a target of the same kind and Rust target, names the same source
   entry, holds a file with the same bytes, states the same guarantee or precondition, or has forms that hash the
   same. So content that moves takes the
   rejection with it, whether its directory or its target was renamed or not, and so does a contract or a `none`
   statement copied under another id. Nothing about items is written, so nothing about them can be edited away.

Items over-approximate. A rejection of one cost in a package reaches every record that names the package; a
rejected cost reaches the same-named cost on every target of the same kind and Rust target; a file
whose bytes an unrelated facet also holds, an empty one for instance, carries the rejection there; and so do the
same forms, such as an empty behavioral model or a common `none` statement; and so does a common guarantee or
precondition sentence, such as "one processor", to every contract that states it. A reviewer answers it there. An
`answers` may name any rejection the facet inherits.

**An answer covers the items the answering review saw.** It lifts the rejection for the items of it that the facet
held at the hash the answering review names. If the facet later holds an item of the rejection that it did not
hold then, the rejection binds again, until a single review at a hash that holds every item of it the facet now
holds answers it. So an answer given
where a rejection reached a facet only through an empty file does not let the rejected cost itself move there
unseen.

**Order, weakest first:** `rejected`, `unreviewed`, `stale`, `production`. A record's status is its weakest
facet's.

- **A rejection stands until it is answered.** A version bump, or a comment edited in a covered file, moves the
  hash but leaves the facet `rejected`. Only a production review that names the rejection in `answers`, with a
  basis saying why, lifts it. Answering is a reviewer's act, recorded for good.
- **A production verdict does not outlive a rejection at its own hash.** Content rejected at `h`, changed,
  approved at the new hash with the rejection answered, and then changed back to `h`, is not `production`: the
  earlier production review at `h` counts again only when a review at `h` answers the rejection that names `h`.
- **There is no experimental verdict.** The experimental namespace needs no review. A reviewer who finds a facet
  fit for experiment but not for production writes `rejected`, with a basis that says so.

**The roles** are:

- `director`;
- `independent-context`: a context that did not write the facet, whose basis says what it was given and what it
  was barred from;
- `external`: a named outside party, whose basis locates its statement.

A maintainer is a task tree, and no role is a task tree. §13's first limit says what independence rests on
beyond that.

**A review is checked against the commit that ledgered it**, by bless, the gate, CI's replay and every load (§9),
and refused when:

- its facet is not one of the four;
- its hash is not written as §3 requires, or is not the facet's bound hash when it is ledgered. A review is of
  content that exists;
- its verdict or role is outside the closed sets;
- `answers` is on a rejection, or names a hash that is not a rejection the facet inherits. A rejection of the
  facet itself, or one it inherits through lineage, is answered only once it is in a parent's ledger, so a bless
  cannot ledger both it and its answer. A rejection a facet inherits only through items may be answered in the
  commit that ledgers it, so a production record that the items reach can stay loadable (§6) without being
  demoted. Answers are checked against the parents' ledgers and the rejections the same bless ledgers, so the order
  a bless processes reviews in decides nothing;
- its `who` is empty or equals the maintainer's tree id;
- its `basis` is empty;
- its date is not a real calendar date, or is later than the committer date of the commit that ledgers it, read
  in the time-zone offset that commit records. Date and offset are both in the commit, so a check that replays it
  later gets the same answer, and a reviewer east of UTC is not refused before UTC's midnight.

A ledgered review is not re-checked when fields around it change later.

**Reviews cannot be taken back.** Every review is in the lock's append-only ledger with its facet, verdict and
hash (§9), and a present record must contain every review the ledger holds for its id. A rejection therefore
survives the record being deleted and re-added. It survives a rename or a split through the lineage, and content
that moves without lineage through its items, both read from history. Editing a review counts as removing one, and is
refused the same way.

**A review covers everything the bound hash covers**, `none` statements included.
`(timing-model … (none "entry cost is inside rt.dispatch's service cost"))` decides whether a cost is charged at
all, and it is reviewed like a number.

⭐ **Why derived.** A written `(evidence-status reviewed)` is the one field an edit can make false with nobody
noticing. Binding a production verdict to the bound hash makes forgetting impossible. The edit changes the hash,
and the review no longer names it. A rejection, which the edit must not be able to escape, is bound to the
facet instead.

### 6. Namespaces

- **`experimental`** holds any record that passes §1–§5's structure and §9's lock checks. Unknowns are allowed,
  and so are unreviewed, stale and rejected facets.
- **`production`** additionally requires:
  1. each of the four facets has status `production`, so a reviewer, not a file move, decided each is fit for
     production;
  2. no fact and no cost is `unknown`;
  3. every dependency, `describes` record and `measured-with` record is itself in `production`;
  4. the facets its catalog requires are present and not empty: an implementation with a package, a behavioral
     model with at least one fact, a timing model with at least one cost:

     | Catalog | Present and not empty in production | `ROADMAP.md` §9's admission evidence, which the production reviews' basis must address |
     | --- | --- | --- |
     | `algorithms` | implementation, behavior-model, timing-model | contract, reviewed source basis, reference behavior, implementation tests, supported analysis model |
     | `machine` | behavior-model | primary specification references, exact revisions, reviewed extraction, executable checks where possible |
     | `devices` | behavior-model, implementation (the driver) | access schema, observable behavior, executable model, driver, independent conformance evidence |
     | `interfaces` | behavior-model | versioned API, model tests, valid and invalid examples |

     The first column is checked mechanically. That the reviews addressed the second is not, and §13 says so.

- **A record in `production` that fails any of these is refused at load**, and the catalog does not load. It is not
  demoted. A production record that is not reviewed is a defect in the catalog. Demoting it quietly is the silence
  `ROADMAP.md` §9 forbids, and it would let the next edit to a reviewed record pass unseen. The repair is visible
  either way: review it again, or move the file to `experimental`.
- **"In production"** everywhere in this record means the namespace. Status is always a facet's (§5).

### 7. Claims: computed citations, strength and the production rule

- **A claim reads, and its citations are what it read.** The catalog is consulted by `(facet, name)` lookups under
  a selection (§12). Each returns its fact or cost together with the `(id, facet)` it came from. A claim's
  citations are the union of what its lookups returned. No claimant writes a citation list, so none can leave out
  a facet it used.
- **The closure** of a citation `(R, F)` is:
  - `(R, F)` and `(R, contract)`;
  - for each derived line of `(R, F)` that names a facet `(X, G)`, the closure of `(X, G)`.

  So the closure holds exactly the facets whose bound hashes enter the cited one's, and each of their records'
  contracts. It is complete by construction. A `none` facet in it is part of it: a reviewed statement that
  nothing of that kind is there.
- **A claim states** its profile, its target when it has one, and its **strength**, `exploratory` or
  `production`. It also states the image it is about, when an engine-made build (`M4`) gives it one. A caller
  cannot supply the image.
- **A claim result carries:**
  - its strength, profile, target and image, the commit it read, and the `origin/main` commit it was checked
    against, with `origin`'s URL, and the commit its own tooling was built from;
  - its closure, as `(id, facet, version, bound hash, status)` lines, and its image's closure as the same lines
    when it has an image;
  - its reads, as `(facet, name) → id`, and the lookups that found nothing, each with its selection;
  - every input it took from outside the catalog and the description, named, with the caller or application as
    its source;
  - every precondition in the closure, as an assumption. `ROADMAP.md` §7.2 requires a claim to "list the unproved
    links", and a precondition is one.
- **Admission, strongest verdict first:**
  1. `unsupported-profile`: the claim's target is not a named target (§2); or a record in the closure, or in the
     image's closure below, omits the claim's profile, or its `targets` does not admit the claim's target. A claim
     with no target is admitted only when every record in its closure names `(targets any)`.
  2. `not-established`, for a **production** claim only, naming each cause:
     - a facet in the closure, or in the image's closure below, belongs to an `experimental` record (named, with
       its status);
     - an input came from the caller or the application. Before `M4` gives such inputs their own evidence rules,
       none can back a production claim. For the runtime variant that means `C_i`, `CS_i`, `J^release`, `J_s`,
       the task facts and the plan's inputs (§12);
     - a cost the claim **read** is on a target whose `TARGET_KIND` is not `board`. The test is closed: only a
       board's timing is target evidence (`decision_emulator-independence-retained.md`);
     - a cost the claim read has `binary` `unbuilt`, or an image other than the claim's. A claim with no image
       fails this for every cost that is not `independent`;
     - the closure holds an implementation that is not `none`, and the claim has no image, or an image not built
       from the reviewed code the way the gate builds it. The image's build record (`M4`) must name, for each
       package it compiled, the profile and target of the gate's matrix it was built in, with no other flag and no
       other `cfg`; the `rustc -vV` of its compiler, which must name the pinned release; the environment the build
       ran in, whose variables must be the gate's allowlist and nothing else, with `RUSTUP_TOOLCHAIN` the pin and
       `CARGO_HOME` an empty directory of the build's own, so `RUSTC_BOOTSTRAP` or a `CARGO_PROFILE_*` override is
       seen; and the compiler's dependency information of that build, each source path
       with the hash of the bytes compiled and each environment dependency held to the gate's rule (§3). **The
       image's closure** is the claim's closure joined with the implementation facet, and its closure, of every
       record whose package the image compiled, so a package the claim never read, such as a console driver, is held
       to the same rules: step 1, the first cause above, and the source paths below. Every source path must then be
       one of:
       - a file of the own or reached set of an implementation in the image's closure, at the bound hash the
         closure holds;
       - generated code, named by the plan (`ROADMAP.md` §7.5) that generated it, with its hash;
       - an application input of `ROADMAP.md` §10.3, named with its hash.

       Any other path refuses the claim. Generated code and application inputs are named in the claim, and until
       `M4` gives them their own evidence rules, either makes a production claim `not-established`, as a caller's
       input does. A cost measured on an image is that image's code, and a fact is reviewed at the closure's, so
       the two must be the same code, built the same way. A closure whose implementations are all `none` needs no
       image;
     - the catalog was not read from a commit whose ledger holds every line of every ancestor's and of the
       published main line's, or the clone has no `origin/main` (§4, §9);
     - a record whose implementation facet, not `none`, is in the claim's closure has a package the image did not
       compile. A fact about code must be about the code the image holds, so the closure is a
       subset of what the image compiled, as well as the image's sources being a subset of the image's closure's
       sets;
     - premise 3 has no named commit yet; a first-parent commit of `origin/main` after it is not a merge commit
       the hosting made; or the claim's tooling was not built from a written tree of a first-parent commit of
       `origin/main` at or after the named commit, or the checker's closure changed along `origin/main`'s
       first-parent chain between that commit and the `origin/main` commit the claim records (premise 3).

  As in the runtime variant's admission, the strongest verdict is reported with every reason that reaches it.
  What an analysis does with a value it could not read is the analysis's verdict, not admission's (§12).
- **An exploratory claim** is admitted past the first step. Its result names every `experimental` record in its
  closure, with its status. That is how experimental data exists "but cannot **silently**" satisfy anything.
- **A production claim is never quietly treated as exploratory**, and its shortfall is a verdict, not an error.
- **Evidence strength is the analysis's to state.** An `assumed` or `observed-maximum` cost may back a production
  claim, since `ROADMAP.md` §7.3 allows both, and it is named as an assumption with its category. The production
  rule is about review, identity and provenance, not about evidence strength.
- **References outside claims.** An artifact that names a record without making a claim, such as S0's provenance
  file, names it by `(id, contract version, record hash)`. Whatever checks the artifact recomputes the record hash.
  For S0 that is `M2.7.4`'s gate. The record hash moves with any facet, which is what a reference to the whole
  record should see.

### 8. Behavioral and timing models are versioned independently

- **Separate facets.** Each has its own `version`, own hash, bound hash, reviews and lock lines.
- **No hash covers the other model.** §3's derived lines never cross between them, and the contract names only
  other contracts. The record's `version` is the contract's alone. Costs name their targets inside the timing
  model. So adding costs for a target the contract already admits, or a timing-only code dependency
  (`measured-with`), is not a contract edit. For a record that names its targets, admitting a new one is: both
  models then apply there too, and their `contract` lines make both reviews stale. For a `(targets any)` record, a
  new target under `targets/` edits nothing in the record. It makes only the behavioral review stale, since that
  model's bound hash holds every target's files (§3), and a cost on the new target is a timing-model edit.
- **Both models rest on the contract.** Each is stated under the record's profiles, targets, preconditions and
  dependencies, so a contract change voids both models' reviews. Independence is between the two models, and the
  contract is neither.
- **A change to the timing model** voids its production reviews and affects every claim that read it. It leaves
  the behavioral reviews, the contract review and every claim that did not read the timing model as they were,
  unless the change is to a file both models reach, which §13 lists as an over-approximation. The converse holds
  too. This is `ROADMAP.md` §9's reason: "their changes invalidate different claims". `M2.7.3` tests both
  directions.
- **For a production record, that holds when the change lands with its reviews.** A change moves the bound hash of
  every facet one of whose bound-hash inputs it changes, directly or through derived lines, transitively. For a
  timing model, that is every dependent's timing model and every record that names it in `measured-with`. For code,
  it is the behavioral model of each record that describes it, the timing model of each that measures it, both
  models of each that depends on it, and, through reached sets, every facet whose packages reach the changed
  package, a catalog `depends` or not. Each moved facet of a production
  record leaves that record failing §6, and the catalog does not load. So the change lands in one of two ways:
  - with a production review of every moved facet in the same commit. The gate lists them. Every claim whose
    closure holds none of them stands;
  - or with each record whose facet moved, and did not get its review, moved to `experimental`. Every production
    claim that read any facet of it is then affected (§10).

  The same holds for an edit to a `.env`, the workspace manifest or a toolchain file that a production record's
  bound hashes reach.
- **No file is in two own sets of different kinds anywhere in the catalog**: an implementation's and a model's,
  or a behavioral model's and a timing model's. A file that held code and a model's statement, or a fact and a
  measurement, would move two own hashes at once, so it is refused. A model states facts about code through
  `code` locators (§2), which reach the code through bound hashes and never put it in the model's own set. Reached
  sets are exempt: a shared dependency package or workspace file moves bound hashes, not versions.
- **What a claim reads decides what invalidates it.** A timing claim that reads a behavioral fact, as the runtime
  variant reads the timer's compare semantics, cites that behavioral model through §7's lookups. A change to the
  fact affects it, as it should. Independence is between the models. It is not a promise that timing claims never
  depend on behavior.
- **The code is shared.** A change to an implementation's sources voids its production reviews, and through §3's
  derived lines those of both models of each record that describes or measures it. The code is what both models are
  about. This is `ROADMAP.md` §9's "Any bound tied to a binary is invalidated by an applicable code, toolchain,
  linker, feature, or target change". Code, toolchain files and targets are in the bound hashes. The claim's image
  covers the rest (§7), and §13 names what that leaves.

### 9. Versions, the lock and the review ledger

- **What `catalog/catalog.lock` holds:**
  - **its first line, exactly one, naming the rules version**, `# archogen-catalog/1` (§5). Bless writes it when it
    creates the lock, and it stands outside the sorted lines below. It changes only to a version the loader knows
    and that is later. A lock whose first line is missing, malformed, or names a version the loader does not know
    or one lower than a parent's lock names, is refused (`catalog-lock-review`): nothing is verified under no
    version. The append-only comparison of lines leaves the first line out, and holds it only to never decreasing.
    Every replay, premise 3's check included, refuses a line new in a replayed commit whose lock names a version
    lower than the base's, so a branch forked before a bump cannot go on ledgering under the older rules;
  - one line per facet version ever blessed, `<id> <facet> <version> sha256:<own hash>`;
  - one line per review ever blessed, `<id> review sha256:<ledger hash> <facet> <verdict> sha256:<the bound hash
    it names>` (§3);
  - one line per waiver, `<id> waiver sha256:<ledger hash> <commit>`, naming a review line and the commit that
    ledgered it by its full object name in lowercase hex (below).

  With the history of the commit being read, and of the published main line's for a production claim (§5), that is
  everything status needs, for a record that no longer exists too: each review's form, and the facet it saw, are in
  the commit that ledgered it (§5). Lines are sorted bytewise by id, then by kind in the order contract,
  implementation, behavior-model, timing-model, review, waiver. Versions are in semantic-version order, and
  reviews and waivers in bytewise order of their line.
- **It is append-only.**
  - A line, once committed, is never changed or removed.
  - The gate compares the lock with its version at every parent of the commit being made, both of a merge's. CI,
    for a push to any branch, compares it with the commit the push replaced on that branch, unless the branch is
    new, with the merge base with `origin/main`, and with each parent of each merge it receives. That push-time
    comparison runs the pushed tree's own scripts, so it is advisory and binds nothing: premise 3's check is the
    protected one.
  - **Catalog changes reach `main` by merge commits.** A squash or a rebase re-ledgers each review line in a new
    commit, where a rejection's hash is generally no longer the facet's current one, so the replay refuses it. The
    refusal is the rule's mechanism. The way through is a merge, which keeps the original ledgering commit in
    `main`'s history. Deleting the rejection on the same branch is refused against the bases it replays, and the
    push-time comparison warns of it; a branch force-pushed without it and pushed again loses it. A new branch
    forked from `main`, or a hosting squash that drops the line, is compared with bases that never held it, which is
    §13's limit on a rejection not yet in `main`.
  - A production claim compares the lock at the commit it reads with the lock at every ancestor and at
    `origin/main` and its ancestors (§4), so a line that history or the published main line holds, and the commit
    lacks, is seen even when the gate was bypassed.
  - Each refuses a lock that dropped or altered a line its base holds.
  - **Every new line is recomputed, commit by commit.** The gate, and CI against each of its bases, replay every
    commit between the base and the head, oldest first: each commit's new lines, those none of its parents' locks
    holds, are computed from that commit's own tree, as blessing would have written them, and a line that differs,
    is missing, or is one blessing would not write is refused. So a lock edited by hand is caught whether the edit
    drops, alters or adds, and a rejection fixed or answered in a later commit of the same push is judged in the
    commit that made it.
  - A retired record's lines stay behind as history, so its versions cannot be reused and its reviews cannot be
    shed.
- **A waiver repairs a review line that fails verification**, which would otherwise stop the catalog loading for
  good (§13). Bless never writes one. It is added by hand on the director's ruling, recorded in the findings
  record, and reaches `main` by a merge commit. Its commit holds the waiver lines and may do two things more, and
  nothing else under `catalog/`, which `M2.7.4`'s replay checks: move to `experimental` every production record
  that the catalog, with the waiver applied, leaves short of §6, whatever the cause, and remove the waived reviews'
  forms from their records. The replay accepts a waiver, as the one line blessing would not
  write, only when the review line it names is in a parent's lock and fails §5's verification at the commit it
  names, which ledgered that line. At load, a waived line is exempt from every check that ties a line to its form:
  it is not verified, its record need not hold its form, and a form it still holds is not compared with it. It
  can only lower a status:
  - a production review it names establishes nothing, and its `answers` answer nothing;
  - when its line or its form at that commit says `rejected`, it binds as an unwaived rejection does, each facet
    either names: that facet, through lineage, and through the items the facet held at every commit that ledgered
    the line, until a review answers it by its ledger hash.

  So a waiver never lifts a rejection or grants a verdict. A facet that loses a production review to one is
  reviewed again. Like every line, a waiver is never removed. **For a verifier defect the order is fixed:** the
  fix merges first, judged by the checker it replaces (premise 3), and the waiver is the next catalog change,
  judged by the fixed one.
- **Checks at load:**

  | Code | When |
  | --- | --- |
  | `catalog-lock-missing` | a facet's current version, or a review in a record, has no line. The repair is to bless |
  | `catalog-lock-unbumped` | a line has the same id, facet and version as a current facet, but a different own hash: changed without a version bump |
  | `catalog-lock-downgrade` | a facet's current version is below a version the lock holds for it |
  | `catalog-lock-review` | the lock's first line missing, malformed, naming an unknown version or one lower than a parent's; a line that matches none of the lock's forms, or out of order; a waiver naming a line that verifies, or a commit that did not ledger it; a present record lacks a review the ledger holds for its id; a review's form disagrees with its line's facet, verdict or hash, or with its form in the commit that ledgered it; at a ledgering commit, the review's form does not hash to the line's ledger hash, the line's facet or verdict is not the form's, or the facet's bound hash is not the one the line names (§5) |
  | `catalog-lock-retired` | a retired id has a rejection that no production review of a record superseding it answers, and no present record supersedes it; a present record takes a superseded id; a present record lacks a `supersedes` the lineage holds for it |
  | `catalog-conflict` | for some profile and target the catalog names, or for a claim with no target, two records supply the same name under §12's selection |

- **Own hashes, not bound ones.** The lock pins **own** hashes, so a facet's version moves when its own forms or
  files do, and nothing else forces an edit. Bound hashes move with dependencies, reached files, targets and
  ledger sections. They make the right reviews stale (§5), and no version cascade follows.
- **The lock keeps versions honest and reviews permanent.** The soundness of status and invalidation rests on
  bound hashes and the ledger, not on the version lines.
- **Blessing** (`ARCHOGEN_BLESS_CATALOG=1`):
  - recomputes every line that no parent of the commit being made holds, from the current records, and keeps
    every line a parent holds. The parents are `HEAD`, and `MERGE_HEAD` during a merge (§4), so a review merged in
    from a branch where it was ledgered is kept, not recomputed against the merge's tree. A review
    added and then edited before a commit leaves no trace, and nothing committed can be undone;
  - checks each review it ledgers against §5, its hash against the facet's current bound hash included;
  - refuses `catalog-lock-unbumped`, `catalog-lock-downgrade`, `catalog-lock-review` and
    `catalog-lock-retired`.

  So regenerating the lock cannot launder a change. `ROADMAP.md` §14.4 says the same of the trust baseline: "The
  author of the implementation change cannot satisfy the gate solely by regenerating the expected baseline."

### 10. Invalidation

`ROADMAP.md` §9: "Dependency-based invalidation must identify affected builds and analyses."

- **Against what a claim recorded, not against the lock.** A claim result carries its closure, and its image's
  closure, with each line's bound hash and status, its reads, and its strength (§7). A recorded closure line is
  **affected** when:
  - its record is gone, or cannot be read;
  - its facet is now `none`;
  - its current bound hash differs, or cannot be computed, because a dependency is missing, a cycle has appeared
    or a source is untracked;
  - its facet's status **at the recorded hash** differs from the recorded status. This covers a later rejection or
    an answer, for any claim, exploratory ones included. Reviews are permanent (§9), so the status can always be
    recomputed;
  - for a production claim, its record is no longer in the `production` namespace.

  A recorded read is **affected** when the same lookup, under the same selection, would now return another
  record, or the catalog no longer loads. A recorded lookup that found nothing is affected when it now finds a
  record, since the claim took that input from elsewhere, or went without it.
- **What "now" is.** The check reads a commit, `HEAD` unless one is named, with its ledger joined with
  `origin/main`'s as admission does (§4), and records both commits. A production claim is also affected when the
  `main` commit it recorded is no longer an ancestor of `origin/main`: the history it was checked against has been
  rewritten (premise 3).
- **Over the recorded closure, not today's graph.** A dependency that has since left the graph is still checked.
- **It runs even when the catalog does not load.** Hashes are computed without the lock, and anything that cannot
  be read or computed counts as affected. The answer errs toward too many and never too few.
- **Builds.** When `M4`'s lock data names the closure lines a build used (`ROADMAP.md` §10.3: "eADL/module/catalog
  versions, source hashes"), the same function answers for the build.

### 11. Refusals

Every refusal names its record, its field and the field's source location, and has one code.

| Code | When |
| --- | --- |
| `catalog-read` | a reader diagnostic; a byte outside §1's set; a decoded string outside printable ASCII |
| `catalog-layout` | a file under `catalog/` that is neither a record in a namespace directory nor the lock; an untracked record or file under `catalog/` (the gate); a shallow repository or an `info/grafts` file (§4) |
| `catalog-shape` | not exactly one `catalog-record` form; a field or subform missing, unknown, duplicated or out of order, a code fact's locators apart, which §14.2 files; two identical locators of one fact; a repeated name; a decimal |
| `catalog-id` | the id breaks §1's grammar, differs from the file stem, or is used twice |
| `catalog-version` | a version or a requirement breaks §2's form |
| `catalog-field` | one of the following: <br>• an empty string where §2 requires text <br>• an unknown catalog, profile, target, unit, category, role, verdict or `TARGET_KIND` <br>• a target without `TARGET_KIND` or `RUST_TARGET`, whose stem breaks §2's grammar, or whose `.env` breaks §3's grammar <br>• a `.env` value holding `/` that is not a tracked path in normal form, one without `/` that names a tracked file, or a target file under `catalog/` <br>• a `.env` or `.eadl` directly under `targets/` without its pair <br>• a group name supplied under some selection by a record other than the one that supplies the group's anchor there (§12); a fact in a facet the facts table does not give it; a cost named `api.completion`; a self-statement named with another record's id <br>• no guarantees <br>• a malformed fact or cost <br>• a negative integer <br>• a safety factor that does not pad, has a term outside `u32`, or whose `value` is not the padded observation <br>• a known cost that `Bound::validate` refuses <br>• a cost on a target the contract does not admit <br>• `independent` outside its two names, or on a target that is not a board <br>• a variant name of the wrong kind or in the wrong facet <br>• a case §14.2 files under `catalog-field` |
| `catalog-locator` | a `file` locator outside the facet's own set; a `code` locator outside its record's implementation own set, or naming a record §2 does not allow; a code fact without a `code` locator, or with a locator that is not one (§14.2); a `ledger` anchor the ledger does not hold, or holds twice; a missing locator where §2 requires one |
| `catalog-source` | one of the following: <br>• §3's package rules, manifest dialect and workspace rule, and §4's path rules <br>• a `proc-macro`, `proc_macro`, `crate-type` or `crate_type` key; a legacy `rust-toolchain` file <br>• in what `cargo metadata` resolves: a procedural-macro or build-script target, a source that is not a path, or a package §3's reading did not reach, or reached and cargo does not hold <br>• a `cargo-features` or `rustflags` key; a refused identifier, keyword or attribute word, as tokens; assembly outside §14.2 and §14.3; a non-ASCII identifier; a compiler-read source path that does not end in `.rs` <br>• a cargo configuration file the gate may not let cargo read, or one outside the dialect <br>• two tracked paths that differ only in case; a pin that is not a release number; a compiler that is not the pinned release; a workspace root cargo reports elsewhere <br>• a source path outside the written index, or whose bytes differ from its blob <br>• a path under `catalog/` <br>• a directory entry with a package below it, or a package with a package below it <br>• a file in two own sets of different kinds (§8) <br>• a package that does not build under the gate's matrix <br>• a compiler-read path or environment dependency outside the sets (the gate) |
| `catalog-dependency` | a `depends`, `describes`, `measured-with` or `supersedes` id that is unresolved, unmatched by its one candidate's version, or listed twice; a cycle in the graph of §3's derived facet lines, which is the only acyclicity hashing needs; a `supersedes` id with a present record or no ledger lines |
| `catalog-review` | §5's review rules |
| `catalog-production` | §6 |
| `catalog-lock-missing`, `catalog-lock-unbumped`, `catalog-lock-downgrade`, `catalog-lock-review`, `catalog-lock-retired`, `catalog-conflict` | §9 |

**A refusal to load is a defect in the engine's own knowledge**, not a verdict on anyone's description. When a
command loads the catalog (`M3` onward), it exits `tool-failure`. A claim's outcome is separate, and §7 gives it.

### 12. What the runtime variant takes, and from whom

This section is kept in [`decision_catalog-records-variant-inputs.md`](decision_catalog-records-variant-inputs.md).
It moved there verbatim when this record neared its size ceiling, and was extended there by `M2.10.1`. It is part of
this record, normative and reviewed with it: every input of the runtime variant with its one owner, the names and
facets the catalog supplies them under, the facts the composition of the variant's composite inputs needs, and how a
lookup selects them.

### 13. Limits

This section is kept in [`decision_catalog-records-limits.md`](decision_catalog-records-limits.md). It is part of this
record, normative and reviewed with it: what the structure guarantees and what it leaves to review, what no hash
covers, where a rejection binds, and the limits of `/1` that later leaves lift.

### 14. The port's assembly

This section is kept in [`decision_catalog-records-port.md`](decision_catalog-records-port.md). It is part of this
record, normative and reviewed with it: what the port's assembly needs and what the pinned toolchain does with it,
measured; the declaration, invocations and templates that admit it in a record's package; the dialect per
architecture; and the port's facts restated by what a locator reaches (`M2.12`).

## Why

- **`ROADMAP.md` §9 asks for a mechanism, not a label.** Every design that writes the evidence status into the
  record relies on each editor remembering to lower it.
  - A production verdict bound to a hash is voided by the edit itself.
  - A rejection bound to the facet cannot be edited away, only answered.
  - A ledger that keeps every review means no verdict can be shed, reordered or renamed away.
- **Two hashes per facet, because one does two jobs badly.**
  - The **own** hash keeps versions honest. It moves only when the facet's own forms or files do, so a change
    elsewhere forces no edit.
  - The **bound** hash keeps reviews and claims honest. It moves when anything they rest on does: a dependency, the
    code under a model, a workspace file, a target or a ledger section.

  With one hash per record, as the first draft had, a timing change forced a contract bump. That voided the
  behavioral reviews and made `ROADMAP.md` §9's "their changes invalidate different claims" false.
- **Computed citations, and closures that follow the derived lines**, because a declared list, or a hand-written
  closure rule, is one someone can leave a facet out of.
- **Invalidation against recorded closures, reads and statuses**, because a lock is rewritten by every bless, and
  a comparison with it is empty at every commit.
- **The compiler decides a package's completeness**, because no lexical rule sees everything that `#[path]`,
  `cfg_attr` or a renamed include macro makes it read.
- **Production claims rest on the catalog, the description and an engine-made image, and nothing else.** A caller's
  figure, an emulator's timing, or a cost from another image each carries evidence the catalog's reviews never
  saw.
- **The eADL reader, not a new format.** The workspace depends on nothing outside `std`
  (`decision_zero-dependency-engine-core.md`), so TOML or JSON would mean a new parser. The flat `KEY=VALUE` of
  `targets/*.env` cannot nest a cost's fields. The reader already exists, is bounded and fuzzed, and gives every
  form a span. It reads only the datum layer, so a record is never mistaken for a description.
- **§3's own encoding**, not `Form::to_canonical`, keeps hashes stable under any change to the language's printer.
- **Refusing a stale production record at load, not demoting it.** A demotion would be the "silently" `ROADMAP.md`
  §9 forbids.
- **A requirement per dependency, not an exact pin.** A compatible change to a dependency edits no dependent, and
  the dependency's bound contract hash still makes its dependents' contract reviews stale. There is one candidate
  per id, so no resolver chooses anything a locked build would have to repeat (`ROADMAP.md` §10.3).
- **Rejections bind items as well as ids**, because an id is a name its author can change, and the content a
  rejection was about is what must not return unexamined.
- **A production claim reads a commit**, because the index is not history, and the ledger's permanence is a
  property of history.
- **Composite inputs composed from owned parts, never taken whole from one owner.** A catalog figure that silently
  omits the application's share is the under-charge `ROADMAP.md` §7.4 exists to prevent. So §12 names the catalog's
  parts, and `decision_runtime-composite-inputs.md` adds them to the application's.
- **Rejected: the hash written inside the record.** A record that contains its own hash needs a rule for the
  bytes it skips, and invites a hand-edited hash.

## How to apply

- **Writing a record:** start in `catalog/experimental/`.
  - Write every unknown as `unknown`, with the reason.
  - Point implementations at packages, and models at the files, directories or packages they rest on.
  - Name what the facts `describe` and what the costs were `measured-with`.
  - Then bless the lock (`ARCHOGEN_BLESS_CATALOG=1`).
- **Changing a record, or a file one covers:** move the version of each facet whose **own** hash moved, then
  bless. Every production review whose bound hash moved is now stale, which is correct.
- **Retiring, renaming or splitting a record:** the successor names it in `supersedes`, and answers any rejection
  it inherits. Lineage is permanent once blessed, and a superseded id is never used again.
- **Reviewing:** a reviewer outside the maintaining lane appends a `review`. It names the facet's current bound
  hash, which the catalog tool prints (`M2.7.3`) and which must still be current when it is blessed, and gives a
  verdict, any rejections it answers, and a `basis` saying what was checked against what. Then bless, which
  ledgers it. A rejection is merged to `main` in its own commit, by a merge commit, before the facet it names
  changes, and binds for good from then (§9, §13). Promotion to `production` needs a
  `production` status on all four facets.
- **Claiming:** read through the catalog's lookups, keep the result's closure, reads and inputs, and choose the
  strength. A production claim that comes back `not-established` names every cause.
- Related:
  - [[decision_runtime-analysis-variant]]: its §1 inputs are owned in §12 here. This leaf amended its "How to
    apply" and marked where it calls an input the engine's or the platform's;
  - [[decision_emulator-independence-retained]]: no timing claim rests on the emulator, which §7 enforces;
  - [[decision_target-platform-description]]: the targets the records name;
  - [[decision_zero-dependency-engine-core]];
  - [[decision_s0-retirement]]: the S0 stub this retires.

## Review (`ROADMAP.md` §9: "Independent reviewers check preconditions and any interpretation on which correctness
relies")

Every round was a new read-only context that had not written the record. The findings, and the answer to each, are
kept in [`decision_catalog-records-reviews.md`](../../reviews/decision_catalog-records-reviews.md). This record states
the design as it stands, and that one keeps how it got here.

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 22 | 6 | "should not be accepted as it stands" |
| 2 | 25 | 10 | "should not be accepted yet" |
| 3 | 24 | 7 | "cannot yet be accepted" |
| 4 | 14, and 2 found while answering | 4, C2 still open, and the 2 found while answering | "not acceptable as it stands" |
| 5 | 21 | 2, and a defect-level construction against D4's answer | "cannot be accepted as it stands" |
| 6 | 16 | 5, most needing control of the machine or the hosting; the director then ruled §0's threat model | "cannot be accepted as it stands" |
| 7, the first judged against §0 | 15, and 1 found while answering | 3 (G1, G2, G4), a latent one (G5), one at defect level (G3), and the one found while answering; none needs a premise broken | "cannot be accepted as it stands" |
| 8 | 19 | 1 (H1, a procedural macro under another spelling), and H2, H4 and H6 at defect level; none needs a premise broken | "cannot be accepted as it stands" |
| 9 | 20 | none; 4 at defect level (I1–I4), all in §12's new composition names; §3–§9 held | "cannot be accepted as it stands" |
| 10 | 18 | 1 (J1, code facts ungrouped from the costs whose code they state, and an image that need not compile the closure); J2–J4 near it | "cannot be accepted as it stands" |
| 11 | 14 | 2 latent (Q1, `compare-rounding` not tied to the timer service's code; Q2, releases by code no statement covered), with Q3 near them; §3–§9 held | "cannot be accepted as it stands"; all three latent until a board or `M4` |
| 12, the first under the closure rule | 12 | 1 live (R1, a pull request changing the checker that judges it), 1 latent (R3, forms items across rules versions); §3–§9 held | "does not yet meet the closure rule"; nothing else live at defect level once R1 is fixed and R2, R4 and R5 are answered |
| 13 | 14 | 1 live (S1, the checker built through packages premise 3 did not list); S2 and S3, routes the answers left impossible, fail closed | "does not yet meet the closure rule"; nothing else live at defect level once S1 is fixed, S2 and S3 answered and S4's sentence added |
| 14 | 15 | none by construction; 2 ambiguities at defect level, live (T1, cargo configuration read before §3's refusals; T15, where the two trees sit) | "does not yet meet the closure rule"; "one short revision from acceptable" |
| 15 | 14 | none by construction; 3 ambiguities at defect level, live, all in the check's protection (U1, U2, U5); the catalog's mechanics held | "does not meet the closure rule"; the check's mechanism then made `M2.7.6`'s, with premise 3 stating the property |
| 16 | 10, and 3 for `M2.7.6` | none, and none at defect level; the catalog's mechanics held | "The record meets the closure rule"; accepted in the change that answered it, which closed `M2.7.1` |
