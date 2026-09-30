# Catalog records: what one holds, how it is hashed, and when it may back a claim

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the Rust toolchain](../book/src/ledger.md#rust-toolchain) — `rustc`, `cargo` and
  `rustup`, whose version, scope and limits are in the ledger
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`), deciding what `ROADMAP.md` §9 requires of a catalog entry
  before any code for it is written: "an ID, semantic version, content hash, source/license metadata, maintainer,
  dependencies, supported profiles, preconditions, guarantees, implementation source, model source, cost evidence,
  and evidence status". It also covers §9's two rules. "Unknown or unreviewed data may exist in an experimental
  namespace but cannot silently satisfy a stronger production claim." And "Behavioral and timing models may have
  separate versions because their changes invalidate different claims." The reviews, and the answer to each of
  their findings, are the last section.

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
    targets (every target, for `any`) and its ledger sources;
  - for a timing model, its record's contract, the code whose costs it states, the timing models of its
    dependencies and of its `measured-with` records, the targets of its costs and its ledger sources.

A facet's **evidence status is derived, never written**. It comes from reviews, whose verdict is `production` or
`rejected`:

- a production verdict holds only at the bound hash it names, so any edit to what it covered voids it. It also
  lapses while a rejection names that same hash, until a review at that hash answers the rejection;
- a rejection stands, whatever the content becomes, until a later review answers it by name. It binds the facet,
  every record that supersedes the rejected one, and any facet anywhere that supplies a fact, cost or source it
  covered;
- every review is kept in an append-only ledger with its facet, verdict and hash, and so are the lineage and the
  items each rejection covered. None can be taken back, reordered or renamed away.

A claim's citations are **computed from what it read**. Its closure is exactly the facets whose hashes enter
those it read, and the result records each one's bound hash and status. That is how a later change, rejection or
demotion is traced to it.

A **production claim** is admitted only when every catalog input comes from a production record, the claim's
own engine-made image stands behind every image-dependent cost, and that image was built from the reviewed code.
The description's inputs are the description's own. Nothing may come from the caller, no cost it reads may be on
a target that is not a board, and the catalog it reads must be a commit whose ledger holds all of its history. A
record may live in the production namespace only while each of its four facets is `production`.

The behavioral and timing models are separate facets with separate versions, and neither's hash covers the
other. So a change to one invalidates only the claims that read it. Both rest on their record's contract, so a
change to the contract invalidates both.

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
    suite's population and not in the language baseline. §5.6 puts engine knowledge in "versioned engine catalogs
    or plugins … even when they are stored in a declarative engine-specific format".
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
    refusal.
- **Every field keeps its source location**, since the reader gives each form a span, and a diagnostic points at
  the field. Where a fact or cost came *from* is its `locator` (§2).

### 2. The fields

**The contract.** These fields are the record's contract facet, and its `version` is the record's semantic
version.

| §9 field | Form | Rule |
| --- | --- | --- |
| ID | the symbol after `catalog-record` | §1's grammar, equal to the file stem |
| semantic version | `(version "MAJOR.MINOR.PATCH")` | three decimal numbers, each `0` or without a leading zero, and no suffix. It is **the contract's version**. Each other facet has its own |
| — (which of §9's four catalogs) | `(catalog algorithms)`, `machine`, `devices` or `interfaces` | §9's table. It decides what production requires (§6) |
| source/license metadata | `(source (origin "…") (license "…"))` | both non-empty. `origin` says where the content came from, exactly enough to find it again: this repository and the leaf that wrote it, or an outside source with a ledger section (`SOURCE-LEDGER`) |
| maintainer | `(maintainer <tree-id>)` | the task tree that owns the record, such as `M2`: an uppercase letter, then uppercase letters and digits |
| dependencies | `(depends (<id> "<MAJOR.MINOR>") …)`, or `(depends)` | a requirement on the dependency's **contract** version, with Cargo's caret meaning. `"1.2"` is at least `1.2.0` and below `2.0.0`. With major `0`, the minor is the boundary: `"0.3"` is at least `0.3.0` and below `0.4.0`. The catalog holds one record per id, so there is at most one candidate and nothing to choose. An id appears once, must resolve and must match, and the graph has no cycle |
| — (lineage) | `(supersedes <id> …)`, or `(supersedes)` | ids this record replaces. Each must have ledger lines and no present record. Blessing ledgers each as a lineage line (§9), and from then on it is permanent: the record cannot drop it, and no record may take a superseded id again. A superseded id's unanswered rejections pass to every record that supersedes it, directly or through a chain (§5). A rename or a split cannot shed a verdict through lineage, and §5's items bind what moves without it |
| supported profiles | `(profiles <profile> …)` | at least one, each a profile the engine supports (`eadl_model::profile::supported`) |
| — (where the record applies) | `(targets any)`, or `(targets <target-id> …)` | a named target is a stem with both `targets/<t>.env` and `targets/<t>.eadl`. The `.env` must give `TARGET_KIND`, and it is `emulator` or `board`. Every cost's target must be one this field admits. `any` admits every target, and binds the behavioral model to every target under `targets/` (§3), so a new target makes its reviews stale |
| preconditions | `(preconditions "…" …)`, or `(preconditions)` | each a non-empty sentence. An empty list states that there are none, and a reviewer checks that |
| guarantees | `(guarantees "…" …)` | at least one: a record that guarantees nothing is not an entry |

**The other three facets.** Each is one form, always present, with its own version. `none` states that the facet
does not exist, and why, and it is reviewed like any other statement (§5).

| §9 field | Form |
| --- | --- |
| implementation source | `(implementation (version "…") (sources "<package>" …))`, with at least one entry, each a package (§3); or `(implementation (version "…") (none "why"))` |
| model source, behavioral | `(behavior-model (version "…") (sources "…" …) (describes <id> …) (facts <fact> …))`, or `(… (none "why"))` |
| model source, timing, and cost evidence | `(timing-model (version "…") (sources "…" …) (measured-with <id> …) (facts <fact> …) (costs <cost> …))`, or `(… (none "why"))` |

- **`describes`** names the records whose code the behavioral facts are about, beyond the record's own
  implementation. For example, an interface record whose code lives in another record's package. Each must
  resolve.
- **`measured-with`** names the records whose code is inside the timing model's costs, or whose timing figures a
  cost is derived from, beyond the record's dependency closure. Each must resolve.
- An empty list names none.
- **A code fact** is one that §12 lists as about code. Its locator is a `code` locator into the code the fact is
  about: the record's own implementation, or a `describes` record's.

**A fact** is `(fact <name> yes|no (locator …) (basis "…"))`, or `(fact <name> (unknown "why"))`.

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
  for it.
- **`binary`** takes one of three forms, and §7 decides which of them may back a production claim:
  - `(binary "sha256:<hex>")`, the image the value was obtained on;
  - `(binary unbuilt)`, when there was no image;
  - `(binary independent "why")`, when the value depends on no image. It is admitted for exactly two names,
    `compare-rounding` and `delivery` (§12), and only on a target whose `TARGET_KIND` is `board`. Every other
    name, and every emulator, refuses it.
- **`evidence`** is one of §7.3's four categories:
  - `(evidence assumed)`;
  - `(evidence observed-maximum)`;
  - `(evidence observed-maximum (observed <n>) (safety-factor <numerator> <denominator>))`;
  - `(evidence externally-supplied)`;
  - `(evidence analytically-established)`.

  With a safety factor, `observed` is the raw observation, and `value` must equal ⌈`observed` × numerator /
  denominator⌉, computed exactly in 128 bits. The factor's terms are `u32` and must pad: the denominator is
  nonzero, the numerator is at least the denominator, and a term outside `u32` is refused the same way. As §7.3
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

Every hash is SHA-256 (FIPS 180-4), written `sha256:` followed by 64 lowercase hexadecimal digits. Every hash
input is ASCII text, one item per line. Every line ends in one line feed (`0x0A`), the last one included, and
there is nothing else.

**The encoding of a form, `E`**, is independent of any printer, so no change elsewhere moves a hash:

- a symbol is its text;
- an integer is its decimal value, with no leading zeros. A hexadecimal literal is encoded in decimal, so `0x28`
  is `40`;
- a string is `"`, then its decoded value with `\` written `\\` and `"` written `\"`, then `"`. §1 has made the
  decoded value printable ASCII;
- a list is `(`, then its items' encodings joined by one space, then `)`.

**The forms of each facet:**

- the contract's are the `version`, `catalog`, `source`, `maintainer`, `depends`, `supersedes`, `profiles`,
  `targets`, `preconditions` and `guarantees` forms, each whole, in record order;
- each other facet's is its one form, whole.

**Source sets.** Each entry in `sources` is a tracked file, a directory, or a package: a directory whose
`Cargo.toml` has a `[package]` table. A manifest with only a `[workspace]` table does not make a package.

- **An entry inside a package stands for the whole package**, so a named file or subdirectory cannot narrow what
  is covered.
- **A directory entry with a package below it is refused.** Name the package instead.
- **A package with another package below it is refused**, so an entry stands for exactly one package.
- **The facet's own set:**
  - a file entry is that file;
  - a directory entry is every tracked file under it;
  - a package entry is every tracked file under the package's directory.
- **The facet's reached set** is what a package brings in beyond its directory, and it enters the bound hash
  only, as `file` lines:
  - the packages its manifest names under `[dependencies]`, `[build-dependencies]` and every
    `[target.<cfg>.dependencies]`, transitively, each with every tracked file under its directory.
    `[dev-dependencies]` are not followed, because they do not enter the built code;
  - its workspace's manifest: the nearest `Cargo.toml` with a `[workspace]` table, in the package's directory
    or an ancestor. The package must be that manifest's own package, or match an entry of its `members` and no
    entry of its `exclude`. An entry is a relative path whose segments are literal or exactly `*`, which matches
    one segment. Anything else is refused. So is a package that workspace does not include, which Cargo would walk
    past to an outer workspace (the ledger's `rust-toolchain` scope): `/1` does not follow it;
  - from the package's directory and every ancestor up to the repository root, each of `.cargo/config.toml`,
    `.cargo/config`, `rust-toolchain.toml` and `rust-toolchain` that exists.

  A change to the workspace manifest or the toolchain therefore makes reviews stale, but moves no version.

Both sets are sorted bytewise by path, without duplicates, and a file in the own set is not repeated in the
reached set. A path under `catalog/` is refused, since a record could otherwise hash its own reviews.

**A package is refused (`catalog-source`), fail-closed,** when:

- a line of its manifest, or of its workspace manifest, is outside the dialect below;
- it has a build script: a `build.rs` at its root, or a `build` key other than `false`;
- it is a procedural macro (`proc-macro = true`), or has a `package.workspace` key;
- it declares a `[features]` table or an optional dependency. `/1` builds one configuration per profile and
  target, so what a review saw is that configuration;
- a dependency in any dependency table, `[dev-dependencies]` included, is not a path dependency, or is
  `workspace = true`;
- a `path` key of `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]` or `[[bench]]` leaves its directory;
- its workspace manifest has a `[patch]` or `[replace]` table;
- a config or toolchain file on its ancestor path is present but untracked, or a config file there holds any
  table but `[alias]`. Build flags, linkers and linker scripts are the build's, and `/1` admits none;
- a file of the package holds a foreign block, which is `extern` followed by an ABI string or by `{`, or a
  `#[link` attribute. Code that a symbol reaches only at link time, and a native library, are in no source set
  and in no dependency information, so `/1` admits neither.

**The manifest dialect** is the part of TOML these manifests use. A line is blank; a comment; a table header
`[a]`, `[a.b]` or `[[a]]` whose segments are bare keys (ASCII letters, digits, `-` and `_`); or `key = value`,
where the key is a bare key or bare keys joined by `.`. A value is, on the same line, a basic string with no
escape, an integer, `true` or `false`, an array of those, or an inline table `{ key = value, … }` of those. A
quoted key, a literal or multi-line string, a value continued on the next line, a float or a date is outside it.
Every manifest in the repository is inside it as of `2026-09-30`.

These rules are necessary, not sufficient. What the compiler reads is decided by `#[path]`, `cfg_attr`, renamed
include macros and `env!`, and no lexical rule catches them all. So **the gate (`M2.7.4`) builds every package
in every source set** and reads the compiler's dependency information:

- **What it builds from.** A checkout of the git index, in the repository's own `target/`. Each build runs in its
  package's own directory, so the config and toolchain files that cargo and rustup discover are the ones on the
  ancestor path this section hashes.
- **Its environment is an allowlist:** `PATH`, `HOME`, `CARGO_HOME`, `RUSTUP_HOME` and the gate's own
  `CARGO_TARGET_DIR`, and nothing else. A list of variables to clear would miss the next one that changes a build.
- **The build matrix.** The dev and release profiles. An implementation package is built for the host and for
  each target's `RUST_TARGET`, for every target the record names in its contract or costs, where `any` means
  every target under `targets/`. A target it must build for that gives no `RUST_TARGET` is refused. A package
  that is only in a model's sets runs on the host, and is built for the host alone. There is no feature axis,
  because `/1` refuses features.
- **What it refuses:**
  - a package that does not build under the matrix;
  - every path in the compiler's dependency information that is in neither set and not under the toolchain's
    sysroot (`rustc --print sysroot`);
  - every environment dependency except `CARGO_PKG_*`, `CARGO_CRATE_NAME`, `CARGO_MANIFEST_DIR` and
    `CARGO_MANIFEST_PATH`.

The compiler, not a scan, decides completeness.

**Own hash** of facet `F` of record `R`:

```text
archogen-catalog/1
own F R
E(form)                                          one line per form of the facet, in record order
file <path> sha256:<hash of the file's bytes>    one line per file of the own set, in its order
```

**Bound hash** of `F`:

```text
archogen-catalog/1
bound F R
own sha256:<own hash of F>
<derived lines: a set, without duplicates, sorted bytewise>
```

Each derived line takes one of four forms:

- `<facet> <id> sha256:<that facet's bound hash>`;
- `file <path> sha256:<file hash>`, for the reached set;
- `target <path> sha256:<file hash>`;
- `ledger <anchor> sha256:<hash of the section's bytes>`.

The **target files** of a target `t` are `targets/<t>.env`, `targets/<t>.eadl`, and every path the `.env` gives
as a value. A value that holds `/` is a path: it must be tracked and in §4's normal form, or the target is
refused. A value without `/`, such as a program's name or a triple, is not a path. The **dependency closure** of
a record is its dependencies, transitively.

| Facet | Derived lines |
| --- | --- |
| contract | `contract <D>` for each direct dependency `D`, so a dependency's changed guarantee makes its dependents' contract reviews stale, whatever its version says |
| implementation | `file` lines of its reached set |
| behavior-model | `contract <R>`, because the facts are stated under its profiles, targets, preconditions and dependencies; `implementation <R>` and `implementation <X>` for each `describes` record `X`, because a fact about code is voided when the code changes; `behavior-model <D>` for each direct dependency; `target` lines for the target files of each target the contract names, or of every target under `targets/` when it names `any`; `ledger` lines for its ledger locators; `file` lines of its reached set |
| timing-model | `contract <R>`, for the same reason; `implementation <X>` for `R`, every record in `R`'s dependency closure, and every `measured-with` record with its own dependency closure; `timing-model <D>` for each direct dependency and each `measured-with` record; `target` lines for the target files of each target its costs name; `ledger` lines; `file` lines of its reached set |

- **Independence.** No behavioral line names a timing model, and no timing line names a behavioral model. That is
  what keeps them independent (§8).
- **The record's hash**, §9's "content hash", is over:

  ```text
  archogen-catalog/1
  record R
  contract sha256:<bound>
  implementation sha256:<bound>
  behavior-model sha256:<bound>
  timing-model sha256:<bound>
  ```

- **A review's ledger hash** is over its three lines, `archogen-catalog/1`, `review R` and `E(review form)`.
- **Excluded from every facet hash** are the reviews, since a review cannot cover itself, and the namespace.
  Promotion changes no hash, and it needs production verdicts (§6).
- **A `none` facet is hashed like any other.** A `none` timing model still gets its `contract` and
  `implementation` lines, so going from `none` to present, or back, is a change.
- **Line 2 names the facet or review and the id**, so a review cannot be copied between records or facets whose
  text is the same.
- **`archogen-catalog/1` versions this grammar.** Changing any part of this section changes it.

**Worked example.** Two records and three files, given here rather than tracked. They exercise every kind of
line except `file` lines of a reached set and `ledger` lines, which need a package or a ledger. They also
exercise an escaped string, a hexadecimal integer, a review line and lock lines. The example's target gives no
`TARGET_KIND`, so it would not load. It fixes bytes, not a catalog. The digests below were computed by an
independent implementation of this grammar, which first reproduced every digest of the previous revision, and
each changed hash input was checked with `shasum -a 256`, `sha256sum` and `openssl dgst -sha256`. `M2.7.3` must
reproduce every one.

The three files, with their exact bytes (each line ends in a line feed):

| Path | Content |
| --- | --- |
| `docs/example/model.txt` | `model` |
| `targets/example-target.env` | `TARGET_ID=example-target` |
| `targets/example-target.eadl` | `(platform)` |

```text
(catalog-record example.base
  (version "0.1.0")
  (catalog machine)
  (source (origin "the worked example of decision_catalog-records.md") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends)
  (supersedes)
  (profiles rt-static-up-v1)
  (targets example-target)
  (preconditions "a \"quoted\" precondition")
  (guarantees "one processor")
  (implementation (version "0.1.0") (none "a machine has no code"))
  (behavior-model (version "0.1.0") (sources "docs/example/model.txt") (describes)
    (facts (fact one-processor yes (locator (file "docs/example/model.txt")) (basis "the model says so"))))
  (timing-model (version "0.1.0") (none "the costs of a machine are its devices'"))
  (review (facet behavior-model)
          (hash "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813")
          (verdict production) (by independent-context "the worked example") (date "2026-09-30")
          (basis "the model file says one processor")))

(catalog-record example.timed
  (version "1.0.0")
  (catalog algorithms)
  (source (origin "the worked example of decision_catalog-records.md") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends (example.base "0.1"))
  (supersedes)
  (profiles rt-static-up-v1)
  (targets any)
  (preconditions)
  (guarantees "a switch costs what it costs")
  (implementation (version "1.0.0") (none "the example has no code"))
  (behavior-model (version "1.0.0") (sources) (describes) (facts))
  (timing-model (version "1.0.0") (sources) (measured-with) (facts)
    (costs (cost switch (target example-target) (value 0x28) (unit ns) (scope "one switch")
                 (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt)
                 (evidence assumed) (basis "chosen for the example")))))
```

`E` renders the cost on one line, with `0x28` as `40`:

```text
(cost switch (target example-target) (value 40) (unit ns) (scope "one switch") (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt) (evidence assumed) (basis "chosen for the example"))
```

`example.timed` names `(targets any)`, so its behavioral model has `target` lines for every target under
`targets/`, which in this example is `example-target` alone. Its timing model has them for the target its cost
names. Both models of both records carry a `contract` line. The file hashes:

- `model.txt`: `sha256:98ad61a25e3683b6adf2474b01bbe1c27de6aad2ce3a80ff4140fe473c14e691`;
- `.env`: `sha256:0061dd5cabefd87f90429ee144bc8b8d0824108bb668f86fc7b655311ebe08c0`;
- `.eadl`: `sha256:16cc827ff198a7e91963562dce366b4ad1c56a2f4c3d4d81d5d7b3da7f05a2c0`.

| Hash | `example.base` | `example.timed` |
| --- | --- | --- |
| own contract | `sha256:7d0e0beeb4a60b63ec582923f85b705d783f6a23a9981e1376d31bbc7cdaafe0` | `sha256:27d97e2d076776372efefbd88b7e9da223cdc9a1dde381bb0471dc1654f2f6de` |
| bound contract | `sha256:067674b7244594b9bd0f363b8b1eda35e23efd2b968356a317c41f75f9c6bee7` | `sha256:d12c172c744fa512aea123b9b5bcbf872230f25f369a12bd493a4c609c4da6ab` |
| own implementation | `sha256:8b294520470fcc0abd962849a5ece4b7ae461f499a96dc7e47c40cf5d57cc025` | `sha256:55bb9d366caa730ac832bcaa77718d63e3e63c3d5cead08df507b04a3be7fa05` |
| bound implementation | `sha256:00e05895b46d115c43659f54d862f60eec69289643db3fa8e7ed54082b860cfe` | `sha256:ea8fa872738441b53d900655865fc27aa1464beb2c99f21ef949755aeb78f304` |
| own behavior-model | `sha256:65dc42732194435388d0d7b717a23ad13dc4cc17dbdb77fca6ff117e5f394c29` | `sha256:f0c1fb43fa476b20c71de169a42d4531180c44a947dceb1051f899881e48ead4` |
| bound behavior-model | `sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813` | `sha256:63f5b5125e0c2170973a7a979cab7d8b169840095ae47c48a834b03d12de7e6d` |
| own timing-model | `sha256:7a181dae6939f076f6a2fdf1f664517607b46b41d0b571f204d8542920d17867` | `sha256:b21769048b5a0a484669bde510a487f610013fb2aa2c0c039734875dd7133460` |
| bound timing-model | `sha256:56d69b33f75c5d7e9373004a69a7a382d5033c73b181ef4d52ef30d4c2a1da50` | `sha256:f674a199d995429c5c03f748b80eab021c8870e8d096709a9e05894ef1245e53` |
| record | `sha256:66ca4a999f157f4efde58048ee4bca9a0518ce571a3538cf87289e30b5fa17ab` | `sha256:48a323e7f59e248413585129801d7a6eb68bb68b78848175629f0f4f40213b6a` |

For instance, the input of `example.timed`'s bound timing model is:

```text
archogen-catalog/1
bound timing-model example.timed
own sha256:b21769048b5a0a484669bde510a487f610013fb2aa2c0c039734875dd7133460
contract example.timed sha256:d12c172c744fa512aea123b9b5bcbf872230f25f369a12bd493a4c609c4da6ab
implementation example.base sha256:00e05895b46d115c43659f54d862f60eec69289643db3fa8e7ed54082b860cfe
implementation example.timed sha256:ea8fa872738441b53d900655865fc27aa1464beb2c99f21ef949755aeb78f304
target targets/example-target.eadl sha256:16cc827ff198a7e91963562dce366b4ad1c56a2f4c3d4d81d5d7b3da7f05a2c0
target targets/example-target.env sha256:0061dd5cabefd87f90429ee144bc8b8d0824108bb668f86fc7b655311ebe08c0
timing-model example.base sha256:56d69b33f75c5d7e9373004a69a7a382d5033c73b181ef4d52ef30d4c2a1da50
```

`example.base`'s lock lines, its review's included, are:

```text
example.base contract 0.1.0 sha256:7d0e0beeb4a60b63ec582923f85b705d783f6a23a9981e1376d31bbc7cdaafe0
example.base implementation 0.1.0 sha256:8b294520470fcc0abd962849a5ece4b7ae461f499a96dc7e47c40cf5d57cc025
example.base behavior-model 0.1.0 sha256:65dc42732194435388d0d7b717a23ad13dc4cc17dbdb77fca6ff117e5f394c29
example.base timing-model 0.1.0 sha256:7a181dae6939f076f6a2fdf1f664517607b46b41d0b571f204d8542920d17867
example.base review sha256:dd436a7114e51f9b4327c42f0114240a4bafdcfaede3415420aff27fee85c603 behavior-model production sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813
```

The review's `E`, which follows `archogen-catalog/1` and `review example.base` in its ledger hash's input, is one
line, with the hash in full:

```text
(review (facet behavior-model) (hash "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813") (verdict production) (by independent-context "the worked example") (date "2026-09-30") (basis "the model file says one processor"))
```

### 4. Paths, "tracked", and what reads the files

- **A path** is relative to the repository root and written in normal form: segments of ASCII letters, digits,
  `.`, `_`, `+` and `-`, joined by single `/`, with no empty, `.` or `..` segment and no leading or trailing
  `/`.
  - Every tracked path in the repository satisfies this as of `2026-09-30`: `git ls-files -z` lists none that
    does not.
  - The rule applies to paths written in a record and to paths produced by expansion. A path outside it is
    refused.
  - A written path appears at most once in a facet.
- **Tracked** means present in the git index with mode `100644` or `100755`, matched byte for byte, so case counts
  even on a case-insensitive file system. Refused:
  - a symbolic link (`120000`);
  - a submodule entry (`160000`);
  - an untracked path;
  - a directory with no tracked file under it.
- **The loader is I/O-free.** Its caller gives it the tracked set and each file's bytes.
- **No caller reads the working tree.** Each takes the tracked set from `git ls-files -s -z` or
  `git ls-tree -r -z`, and each file's bytes from its blob, so line-ending conversion on checkout does not matter.
  Tests give files in memory.
  - The gate and an exploratory claim read the **index**. The gate checks what is about to be committed.
  - A production claim reads a **commit**, `HEAD` unless one is named, and records it. Its ledger must hold every
    line of every ancestor commit's ledger (§9), so a line dropped from history with the gate bypassed is seen.
  - The gate also lists the untracked files under `catalog/` and refuses them (`catalog-layout`), since no reader
    of the index can see them.
- **Without git.** A §10.3 package carries no index. Loading a catalog there needs the package's own manifest of
  files, which is `M4`'s. Until then, the catalog loads only from a repository.

### 5. Evidence status is derived, never written

A review names a facet, the facet's **bound** hash at the time, a verdict, and its reviewer. A production review
may also name, in `answers`, the ledger hashes of rejections it answers. Status is **order-free**, meaning the
position of reviews in the file decides nothing:

| Status | When |
| --- | --- |
| `rejected` | the facet inherits a rejection that no production review of the facet answers. The rejection may be at any hash |
| `production` | otherwise, when a production review names the facet's current bound hash `h`, and every rejection of the facet that names `h` is answered by a production review that also names `h` |
| `stale` | otherwise, when some review names the facet |
| `unreviewed` | otherwise |

**The rejections a facet inherits** are read from the ledger alone (§9):

1. its record's own rejections of that facet, at any hash;
2. those of every id its record supersedes, directly or through a chain of ledgered lineage;
3. those that cover an **item** the facet supplies. Blessing records a rejection's items when it ledgers it: each
   fact's name, each cost's name and target, and each source entry, a package by its directory and a file or a
   directory by its path. A facet of the same kind that supplies one of them, in any record, inherits the
   rejection. So a cost moved to another record takes the rejection with it, and so does a rejected package
   that another record's implementation names.

Items over-approximate: a rejection of one cost in a package reaches every record that names the package, and a
reviewer answers it there. An `answers` may name any rejection the facet inherits.

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

**A review is checked once, when it is ledgered** (§9), and refused when:

- its facet is not one of the four;
- its hash is not written as §3 requires, or is not the facet's bound hash when it is ledgered. A review is of
  content that exists;
- its verdict or role is outside the closed sets;
- `answers` is on a rejection, or names a hash that is not a rejection the facet inherits;
- its `who` is empty or equals the maintainer's tree id;
- its `basis` is empty;
- its date is not a real calendar date, or is later than the day it is ledgered, in UTC.

A ledgered review is not re-checked when fields around it change later.

**Reviews cannot be taken back.** Every review is in the lock's append-only ledger with its facet, verdict and
hash (§9), and a present record must contain every review the ledger holds for its id. A rejection therefore
survives the record being deleted and re-added. It survives a rename or a split through the ledgered lineage
(§2), and content that moves without lineage through its items. Editing a review counts as removing one, and is
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

     | Catalog | Present and not empty in production | §9's admission evidence, which the production reviews' basis must address |
     | --- | --- | --- |
     | `algorithms` | implementation, behavior-model, timing-model | contract, reviewed source basis, reference behavior, implementation tests, supported analysis model |
     | `machine` | behavior-model | primary specification references, exact revisions, reviewed extraction, executable checks where possible |
     | `devices` | behavior-model, implementation (the driver) | access schema, observable behavior, executable model, driver, independent conformance evidence |
     | `interfaces` | behavior-model | versioned API, model tests, valid and invalid examples |

     The first column is checked mechanically. That the reviews addressed the second is not, and §13 says so.

- **A record in `production` that fails any of these is refused at load**, and the catalog does not load. It is
  not demoted. A production record that is not reviewed is a defect in the catalog. Demoting it quietly is the
  silence §9 forbids, and it would let the next edit to a reviewed record pass unseen. The repair is visible
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
  - its strength, profile, target and image;
  - its closure, as `(id, facet, version, bound hash, status)` lines;
  - its reads, as `(facet, name) → id`;
  - every input it took from outside the catalog and the description, named, with the caller or application as
    its source;
  - every precondition in the closure, as an assumption. §7.2 requires a claim to "list the unproved links", and
    a precondition is one.
- **Admission, strongest verdict first:**
  1. `unsupported-profile`: a record in the closure omits the claim's profile, or its `targets` does not admit
     the claim's target.
  2. `not-established`, for a **production** claim only, naming each cause:
     - a facet in the closure belongs to an `experimental` record (named, with its status);
     - an input came from the caller or the application. Before `M4` gives such inputs their own evidence rules,
       none can back a production claim. For the runtime variant that means `C_i`, `CS_i`, `J^release`, `J_s`,
       the task facts and the plan's inputs (§12);
     - a cost the claim **read** is on a target whose `TARGET_KIND` is not `board`. The test is closed: only a
       board's timing is target evidence (`decision_emulator-independence-retained.md`);
     - a cost the claim read has `binary` `unbuilt`, or an image other than the claim's. A claim with no image
       fails this for every cost that is not `independent`;
     - the claim's image was not built from the reviewed code: its build record (`M4`) does not name, for each
       implementation facet in the closure, the bound hash the closure holds. A cost measured on an image is that
       image's code, and a fact is reviewed at the closure's, so the two must be the same code;
     - the catalog was not read from a commit whose ledger holds every line of every ancestor's (§4, §9).

  As in the runtime variant's admission, the strongest verdict is reported with every reason that reaches it.
  What an analysis does with a value it could not read is the analysis's verdict, not admission's (§12).
- **An exploratory claim** is admitted past the first step. Its result names every `experimental` record in its
  closure, with its status. That is how experimental data exists "but cannot **silently**" satisfy anything.
- **A production claim is never quietly treated as exploratory**, and its shortfall is a verdict, not an error.
- **Evidence strength is the analysis's to state.** An `assumed` or `observed-maximum` cost may back a production
  claim, since §7.3 allows both, and it is named as an assumption with its category. The production rule is about
  review, identity and provenance, not about evidence strength.
- **References outside claims.** An artifact that names a record without making a claim, such as S0's provenance
  file, names it by `(id, contract version, record hash)`. Whatever checks the artifact recomputes the record hash.
  For S0 that is `M2.7.4`'s gate. The record hash moves with any facet, which is what a reference to the whole
  record should see.

### 8. Behavioral and timing models are versioned independently

- **Separate facets.** Each has its own `version`, own hash, bound hash, reviews and lock lines.
- **No hash covers the other model.** §3's derived lines never cross between them, and the contract names only
  other contracts. The record's `version` is the contract's alone. Costs name their targets inside the timing
  model. So adding costs for a target the contract already admits, or a timing-only code dependency
  (`measured-with`), is not a contract edit. Admitting a new target is. Both models then apply there too, and
  their `contract` lines make both reviews stale.
- **Both models rest on the contract.** Each is stated under the record's profiles, targets, preconditions and
  dependencies, so a contract change voids both models' reviews. Independence is between the two models, and the
  contract is neither.
- **A change to the timing model** voids its production reviews and affects every claim that read it. It leaves
  the behavioral reviews, the contract review and every claim that did not read the timing model as they were.
  The converse holds too. This is §9's reason: "their changes invalidate different claims". `M2.7.3` tests both
  directions.
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
  derived lines those of both models of each record that describes or measures it. The code is what both models
  are about. This is §9's "Any bound tied to a binary is invalidated by an applicable code, toolchain, linker,
  feature, or target change". Code, toolchain files and targets are in the bound hashes. The claim's image covers
  the rest (§7), and §13 names what that leaves.

### 9. Versions, the lock and the review ledger

- **What `catalog/catalog.lock` holds** is everything status needs, so status is computable for a record that no
  longer exists:
  - one line per facet version ever blessed, `<id> <facet> <version> sha256:<own hash>`;
  - one line per review ever blessed, `<id> review sha256:<ledger hash> <facet> <verdict> sha256:<the bound hash
    it names>` (§3);
  - one line per answer, `<id> answers sha256:<answering review's ledger hash> sha256:<answered rejection's
    ledger hash>`;
  - one line per item of each rejection (§5): `<id> covers sha256:<rejection's ledger hash>`, then `fact <name>`,
    `cost <name> <target>` or `source <path>`;
  - one line per lineage, `<id> supersedes <old-id>`.

  Lines are sorted bytewise by id, then by kind in the order contract, implementation, behavior-model,
  timing-model, review, answers, covers, supersedes. Versions are in semantic-version order, and the other kinds
  are in bytewise order of their line.
- **It is append-only.**
  - A line, once committed, is never changed or removed.
  - The gate compares the lock with its version at `HEAD`. CI, for a push to any branch, compares it with the
    commit the push replaced on that branch, unless the branch is new, and with the merge base with `origin/main`.
  - A production claim compares the lock at the commit it reads with the lock at every ancestor (§4), so a line
    some ancestor held and the commit lacks is seen even when the gate was bypassed.
  - Either refuses a lock that dropped or altered a committed line.
  - A retired record's lines stay behind as history, so its versions cannot be reused and its reviews cannot be
    shed.
- **Checks at load:**

  | Code | When |
  | --- | --- |
  | `catalog-lock-missing` | a facet's current version, or a review in a record, has no line. The repair is to bless |
  | `catalog-lock-unbumped` | a line has the same id, facet and version as a current facet, but a different own hash: changed without a version bump |
  | `catalog-lock-downgrade` | a facet's current version is below a version the lock holds for it |
  | `catalog-lock-review` | a present record lacks a review the ledger holds for its id; a review's form disagrees with its line's facet, verdict or hash, or its `answers` with the answer lines |
  | `catalog-lock-retired` | a retired id has an unanswered rejection and no present record supersedes it; a present record takes a superseded id; a present record lacks a `supersedes` its lineage lines hold |

- **Own hashes, not bound ones.** The lock pins **own** hashes, so a facet's version moves when its own forms or
  files do, and nothing else forces an edit. Bound hashes move with dependencies, reached files, targets and
  ledger sections. They make the right reviews stale (§5), and no version cascade follows.
- **The lock keeps versions honest and reviews permanent.** The soundness of status and invalidation rests on
  bound hashes and the ledger, not on the version lines.
- **Blessing** (`ARCHOGEN_BLESS_CATALOG=1`):
  - recomputes every line that is not at `HEAD` from the current records, and keeps every line that is. A review
    added and then edited before a commit leaves no trace, and nothing committed can be undone;
  - checks each review it ledgers against §5, its hash against the facet's current bound hash included;
  - writes the answer, item and lineage lines of what it ledgers. A rejection's items are the facet's at the time,
    which is the content the review saw, since its hash is the current one;
  - refuses `catalog-lock-unbumped`, `catalog-lock-downgrade`, `catalog-lock-review` and
    `catalog-lock-retired`.

  So regenerating the lock cannot launder a change. §14.4 says the same of the trust baseline: "The author of the
  implementation change cannot satisfy the gate solely by regenerating the expected baseline."

### 10. Invalidation

§9: "Dependency-based invalidation must identify affected builds and analyses."

- **Against what a claim recorded, not against the lock.** A claim result carries its closure with each line's
  bound hash and status, its reads, and its strength (§7). A recorded closure line is **affected** when:
  - its record is gone, or cannot be read;
  - its facet is now `none`;
  - its current bound hash differs, or cannot be computed, because a dependency is missing, a cycle has appeared
    or a source is untracked;
  - its facet's status **at the recorded hash** differs from the recorded status. This covers a later rejection or
    an answer, for any claim, exploratory ones included. Reviews are permanent (§9), so the status can always be
    recomputed;
  - for a production claim, its record is no longer in the `production` namespace.

  A recorded read is **affected** when the same lookup, under the same selection, would now return another
  record, or a conflict (§12).
- **Over the recorded closure, not today's graph.** A dependency that has since left the graph is still checked.
- **It runs even when the catalog does not load.** Hashes are computed without the lock, and anything that cannot
  be read or computed counts as affected. The answer errs toward too many and never too few.
- **Builds.** When `M4`'s lock data names the closure lines a build used (§10.3: "eADL/module/catalog versions,
  source hashes"), the same function answers for the build.

### 11. Refusals

Every refusal names its record, its field and the field's source location, and has one code.

| Code | When |
| --- | --- |
| `catalog-read` | a reader diagnostic; a byte outside §1's set; a decoded string outside printable ASCII |
| `catalog-layout` | a file under `catalog/` that is neither a record in a namespace directory nor the lock; an untracked record or file under `catalog/` (the gate) |
| `catalog-shape` | not exactly one `catalog-record` form; a field or subform missing, unknown, duplicated or out of order; a repeated name; a decimal |
| `catalog-id` | the id breaks §1's grammar, differs from the file stem, or is used twice |
| `catalog-version` | a version or a requirement breaks §2's form |
| `catalog-field` | one of the following: <br>• an empty string where §2 requires text <br>• an unknown catalog, profile, target, unit, category, role, verdict or `TARGET_KIND` <br>• a target without `TARGET_KIND` <br>• a `.env` value holding `/` that is not a tracked path in normal form <br>• a target an implementation must build for that gives no `RUST_TARGET` <br>• no guarantees <br>• a malformed fact or cost <br>• a negative integer <br>• a safety factor that does not pad, has a term outside `u32`, or whose `value` is not the padded observation <br>• a known cost that `Bound::validate` refuses <br>• a cost on a target the contract does not admit <br>• `independent` outside its two names, or on a target that is not a board <br>• a variant name of the wrong kind or in the wrong facet |
| `catalog-locator` | a `file` locator outside the facet's own set; a `code` locator outside its record's implementation own set, or naming a record §2 does not allow; a code fact without a `code` locator; a `ledger` anchor the ledger does not hold, or holds twice; a missing locator where §2 requires one |
| `catalog-source` | one of the following: <br>• §3's package rules, manifest dialect and workspace rule, and §4's path rules <br>• a path under `catalog/` <br>• a directory entry with a package below it, or a package with a package below it <br>• a file in two own sets of different kinds (§8) <br>• a package that does not build under the gate's matrix <br>• a compiler-read path or environment dependency outside the sets (the gate) |
| `catalog-dependency` | a `depends`, `describes`, `measured-with` or `supersedes` id that is unresolved, unmatched by its one candidate's version, or listed twice; a cycle in the graph of §3's derived facet lines, which is the only acyclicity hashing needs; a `supersedes` id with a present record or no ledger lines |
| `catalog-review` | §5's review rules |
| `catalog-production` | §6 |
| `catalog-lock-missing`, `catalog-lock-unbumped`, `catalog-lock-downgrade`, `catalog-lock-review`, `catalog-lock-retired` | §9 |

**A refusal to load is a defect in the engine's own knowledge**, not a verdict on anyone's description. When a
command loads the catalog (`M3` onward), it exits `tool-failure`. A claim's outcome is separate, and §7 gives it.

### 12. What the runtime variant takes, and from whom

`decision_runtime-analysis-variant.md` §1 lists every input, and each has one owner here:

- **description:** the eADL description;
- **application:** the separately supplied inputs of §10.3 ("separately supplied application inputs");
- **the plan:** §7.5's resolved plan, which `M4` produces;
- **the caller:** stands in for the application and the plan until they exist. Everything the caller supplies is
  named in the conclusion, and none of it can back a production claim (§7).

| Variant input | Owner | Name, kind and facet |
| --- | --- | --- |
| `T_i`, `D_i`, `J_i^event`, priority, what releases the task, whether every arrival does | description | — |
| `T_s` | description | the minimum separation of a source's arrivals is a property of its environment |
| `C_i`, `CS_i` | **composite**: application, plus kernel code run for the job | the caller supplies each whole, as the variant defines it, until `M2.10` composes it from parts |
| `J_i^release`, `J_s` | **composite**: the catalog's part, the application's masked runs, and the plan's queued services | the caller supplies each whole until `M2.10` |
| task facts | application | whether a task suspends, locks the scheduler, shares data outside its sections, or masks other than through the runtime API, plus condition 8 for its own figures. The runtime API record's behavioral facts `no-suspension-primitive` and `no-scheduler-lock-primitive` support the first two for a task the application declares uses only that API |
| `C_rel` | catalog | `timer-service`, a timing cost |
| `C_s` | catalog, only with the code fact `no-application-code.<source>` `yes` from the same record | `service.<source>`, a timing cost. The record that supplies it supplies the source's behavioral code facts too: `no-application-code`, `acknowledge-at-entry` and `defers-nothing`, each suffixed `.<source>`. So a change to that record's code, which is what could add application code to the service, makes the fact's review stale. Without the fact `yes`, the service runs application code, so `C_s` is composite, and the caller's |
| acknowledge point, deferred work | catalog | behavioral code facts `acknowledge-at-entry.<source>` (`no` means at exit) and `defers-nothing.<source>`, from the record that supplies `service.<source>`. A task that runs deferred work is named by the description |
| interrupt priority, the enabled set | the plan | — |
| `S`, `W_wake`, `γ`, `ρ`, `δ` | catalog | timing costs `switch`, `wake`, `preemption-delay`, `compare-rounding` and `delivery` |
| the platform facts | catalog | behavioral facts, one per condition of the variant's `PlatformFacts` (listed below). **Code facts** carry a `code` locator into the code they are about (§2). **Hardware facts** are `one-processor` and `compare-level`. The timing fact `eager-switching` must come from the record that supplies `switch`: `yes` means the variant's condition holds for that `switch`, that switching is eager or that `S` includes every deferred save and restore, and its basis says which |

The behavioral code facts, one per condition of the variant's `PlatformFacts`:

- `preemptive-everywhere`
- `interrupts-do-not-nest`
- `sections-mask-every-interrupt`, which covers the kernel's sections; the application's are a task fact
- `services-preempt-every-task`
- `pending-taken-and-transitions-unmasked`
- `timer-event-driven`
- `compare-rounds-up`
- `due-check-matches-compare`
- `no-early-release`
- `raised-only-when-due`, which states both "raised only when a release is due" and "releases every due task",
  as the variant's field does
- `only-timer-releases-timer-tasks`

The variant's condition 8 for catalog costs is each cost's own `holds-under-preemption`.

- **Image-specific names.** `switch`, `wake`, `preemption-delay`, `timer-service` and every `service.<source>`
  include the image's code: generated code (§8.2) and build-selected instrumentation. `independent` is admitted
  only for `compare-rounding` and `delivery`, and only on a board (§2).
- **What each catalog value must bound, independently of any application:**
  - `preemption-delay`: the most one preemption adds to **any** preempted execution. A value measured on
    particular code is that code's, and its `scope` says so;
  - `compare-rounding`: the worst case over all release instants. A smaller value because the description's
    releases fall on ticks is the analysis's to derive, not the catalog's;
  - `delivery`: the delay for any interrupted code. On an emulator, delivery waits for the end of a translated
    block, so it is image-specific there, and §2 refuses `independent` for it.
- **Selection.** An analysis for profile `P` on target `X` draws on these records only:
  - for facts, records whose `profiles` include `P` and whose `targets` admit `X`;
  - for costs, costs whose `target` is `X`, in records whose `profiles` include `P`. The contract admits `X`,
    by §2.

  A claim with no target draws facts only from records with `(targets any)`, and reads no cost.

  Lookups are by `(facet, name)`, with the kind and facet this table fixes.
- **`<source>`** is the source's id as the description and the enabled set name it, verbatim. An id outside
  §2's name grammar cannot be supplied by any record, so its inputs reach the variant undeclared, with the reason
  named.
- **Each name comes from exactly one record.** Two are refused rather than reconciled, since §9 says
  "Cross-validation investigates source conflicts rather than averaging them".
- **What the variant does with a value it could not read.** Three cases reach the variant as an undeclared
  input: a name no record supplies, an `unknown`, and a cost outside its `holds-for` (more tasks, or more declared
  sources other than the timer). The variant refuses with its own verdict, `analysis-inconclusive` (the variant's
  §5), with the reason named. It never fills a value in. A cost with `holds-under-preemption` `no` fails condition
  8, which the variant refuses as outside the model.
- **Conversion.** A cost converts into the analysis's unit exactly toward a finer unit, and rounds up toward a
  coarser one, as the variant's §1 rounds every cost. An overflow is refused.

### 13. Limits

- **Reviewer independence is asserted, not verified.** Roles and task trees cannot coincide, and `who` cannot be
  the maintainer. That is all the structure guarantees. Git authorship cannot help, because one identity commits
  everything in this repository. What a later reader checks is the review's `basis`.
- **§9's admission evidence per catalog**, beyond which facets are present and not empty, is an obligation of the
  production review's basis: the tests, the reference behavior, the exact revisions and the conformance evidence.
  Nothing checks it mechanically.
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
  - a record moved to `experimental`, which affects every production claim that read it.
- **What no hash covers:**
  - the installed compiler's bits (the channel in `rust-toolchain.toml` is covered);
  - cargo configuration above the repository root and under `$CARGO_HOME`;
  - `--config` flags;
  - rustup's per-directory overrides;
  - `Cargo.lock`. With every dependency a path one, it adds nothing the sets lack, and §3 refuses the first
    non-path dependency;
  - the invocation scripts of an emulator target (the files its `.env` names are covered).

  A claim's image is what covers these for a built system. So §7 lets only `independent` costs, and costs from
  the claim's own engine-made image, back a production claim. Before `M4`, no claim has an image.
- **A rejection binds items, not meaning.** Content moved into another record without lineage, and changed so
  that no fact name, cost name and target, or source entry matches, is new content for review. The ledger is where
  a reviewer of related content looks, and nothing forces the look.
- **Unpublished history can be rewritten.** A rejection committed and never pushed disappears with a reset. The
  ledger's permanence is that of the history CI has seen.
- **The refusal of foreign blocks and `#[link` is lexical.** A macro that expands to one is not seen. For costs,
  the claim's image covers the code it links.
- **A code locator is necessary, not sufficient.** It proves that a fact points into code its bound hash covers.
  That the fact depends on no other code is the review's to check.
- **Where evidence was obtained is the review's to check.** Admission refuses costs on a target that is not a
  board. A fact, or a cost labelled with a board, whose evidence came from an emulator shows only in its basis.
- **One image per target at a time.** A cost name appears once per target. So only the image its costs came from
  can hold production claims on that target, and the next image's measurement replaces them. Keying costs by
  image is `M4`'s.
- **No surface makes a production claim yet.** Strength is a parameter of admission (`M2.7.3`), and the runtime
  variant passes it through (`M2.7.5`). The report that states a production claim to a user is `M4`'s. Until
  then the production rule is exercised by tests.
- **Review granularity is the facet.** One reviewed cost and one unreviewed cost in a single timing model make
  the facet unreviewed. Splitting the record is the way to review part of it, and `supersedes` carries its
  verdicts across.
- **Nothing stores a claim yet** besides tests. §10's answer is only as good as the closure a stored claim keeps.
- **`C_i`, `CS_i`, `J^release`, `J_s`, and `C_s` without its fact, are not the catalog's in `/1`** (§12). Until
  `M2.10` composes them, the variant's soundness for them rests on the caller's figures. No production claim can
  rest on those figures (§7).

## Why

- **§9 asks for a mechanism, not a label.** Every design that writes the evidence status into the record relies
  on each editor remembering to lower it.
  - A production verdict bound to a hash is voided by the edit itself.
  - A rejection bound to the facet cannot be edited away, only answered.
  - A ledger that keeps every review means no verdict can be shed, reordered or renamed away.
- **Two hashes per facet, because one does two jobs badly.**
  - The **own** hash keeps versions honest. It moves only when the facet's own forms or files do, so a change
    elsewhere forces no edit.
  - The **bound** hash keeps reviews and claims honest. It moves when anything they rest on does: a dependency, the
    code under a model, a workspace file, a target or a ledger section.

  With one hash per record, as the first draft had, a timing change forced a contract bump. That voided the
  behavioral reviews and made §9's "their changes invalidate different claims" false.
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
- **Refusing a stale production record at load, not demoting it.** A demotion would be the "silently" §9 forbids.
- **A requirement per dependency, not an exact pin.** A compatible change to a dependency edits no dependent, and
  the dependency's bound contract hash still makes its dependents' contract reviews stale. There is one candidate
  per id, so no resolver chooses anything a locked build would have to repeat (§10.3).
- **Rejections bind items as well as ids**, because an id is a name its author can change, and the content a
  rejection was about is what must not return unexamined.
- **A production claim reads a commit**, because the index is not history, and the ledger's permanence is a
  property of history.
- **Composite inputs left whole rather than composed badly.** A catalog figure that silently omits the
  application's share is the under-charge §7.4 exists to prevent (§12).
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
  ledgers it for good. Promotion to `production` needs a
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

## Review (`ROADMAP.md` §9: "Independent reviewers check preconditions and any interpretation on which correctness relies")

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
