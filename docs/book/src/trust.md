# Checks that must not share a mistake: the trust gate

## The idea, in plain words

archogen does not ask you to trust one program. The **generator** turns a description into a system; separate
programs then check its work — a **configuration checker** that the plan is valid, a **scheduling checker** that the
deadlines hold — and a **reference model**, written apart from the runtime, says what the runtime should have done.
Each check is worth something only because it was reached another way. Two checks built from the same code can share a
mistake, and then they agree on a wrong answer.

So sharing has to be **visible**. Programs share a great deal, and most of it is harmless: a shared way to allocate
memory cannot make two programs agree on a wrong answer. A shared way to evaluate a constraint can. The trust gate
does not forbid sharing. It lists what each checked program is built from, finds everything two of them have in
common, and makes every such item wait for a person to look at it — and it says plainly when something new is shared,
or something already shared has changed behind an unchanged name.

An everyday comparison: two accountants auditing the same books are worth two opinions only if they did not both copy
their totals from the same spreadsheet. Nobody minds them using the same calculator. The gate is the list of
everything they used, kept current, with each shared spreadsheet flagged for review.

> **In one minute, for engineers.** `trust/roots.eadl` names each **root** — the program that runs a role — and
> classifies every other program target of the workspace. `cargo xtask trust-inventory` builds each root clean from
> the commit's blobs, reads what [rustc](ledger.md#rust-toolchain) read, and lists every **shared item** two roots have in common: a package, a
> file, a byte-for-byte copy, the comparison harness, the build configuration. `trust/baseline.eadl` holds one
> **form** per shared item, proposed by the tool and accepted only by a protected review (`M3.6.5`).
> `cargo xtask trust-gate` compares a commit's inventory with its base commit's forms and writes a report in two
> parts, the change and the standing list; `cargo xtask trust-verify` checks a package that carries an inventory. It
> enforces disclosure and change control. It does not prove independence.

## How it works

### The roots

A **root** is the program that runs one role: the `archogen` executable for the generator, the scheduling checker's
library until an executable runs it, and, for the reference model and the implementation it validates, their
libraries, since a test harness runs them. Each is a form in `trust/roots.eadl`, naming its role, its package and
target, the **role packages** holding its role's own logic, and any fixed data it is handed at run time:

```text
(defroot generator
  (role generator)
  (package "crates/archogen-cli")
  (target bin archogen)
  (role-packages "crates/archogen-cli" "crates/archogen-api" "crates/archogen-s0"))
```

A program that would run two roles is refused: one program cannot be independent of itself. Every other program the
workspace can build — an example, a diagnostic tool, the browser module — is classified as not a root, with a reason
and the role packages it compiles, so no one can add a second checker beside the generator unseen.

### The inventory

`cargo xtask trust-inventory` writes the commit's tree from git's blobs, builds each root in the release profile,
clean, and reads the compiler's own record of every file it read. Every source a root compiles is held to the
catalog's rules: a construct that could read something the commit does not track — an `include!` of an untracked
file, an environment variable, assembly pulling in bytes — is refused unless a reviewed **admission** names it.

A file committed as a generator's output — a table a script wrote from a data file — is declared in the same roots
file, by a `defgenerated` form naming the file, the generator files that wrote it, the inputs they read, the command
that ran and why:

```text
(defgenerated "crates/x/src/table.rs"
  (generator "scripts/gen_table.sh")
  (inputs "docs/data/table.csv")
  (command "bash scripts/gen_table.sh docs/data/table.csv")
  (reason "the instruction table, generated from the vendor's CSV"))
```

The inventory reads these forms before it writes or builds anything, and refuses one that departs from that shape — a
clause the form does not take, no generator, a command that is not one string — or two forms declaring one file.

Once a program reads a declared file, the form's generator files and inputs become that program's **provenance**, and
two programs whose provenance meets share a **generated-provenance** item the gate reports like any other: one input
under two generators, a generator both use, a data file one reads that the other's source was made from, or a copy of
an input under another name. Two generated tables that differ in every byte, written by one script from one data file,
are seen to share both. A generator or input that is itself generated — a chain — or that is no blob of the commit
is refused.

A generator must lie outside the reading program's other roles: a table the generator's own executable wrote, read by
a checker, is refused, since the checker would then trust the very logic it checks. A generator that is a program —
an executable or an example — is built as a root is, its sources held to the same rules and its admitted sites counted
with the roots', unless it is a root, whose build it already is; a script is judged by the package that holds it; a
Rust file that is no program's entry point is refused, since only a program's build can be computed. How an undeclared
generated file is recognised is being built (`M3.6.6.2`).

For each pair of roots it then lists what they share, each item with the sha256 of its **content**, its
**configuration** (features, `cfg`, edition, as each root's own build sets them), its **edges** (who depends on it)
and, for a package's files, its **readers**. A new consumer of a package already shared is a change, though nothing in
the package moved.

### The baseline and its forms

`trust/baseline.eadl` holds one form per shared item: the item, the digests the inventory measured, and the review's
part — its classification, the property it can affect, its residual risk, the controls that remain:

| Classification | Can a shared item carry a shared mistake? |
| --- | --- |
| infrastructure | no — a shared allocator cannot make two programs agree on a wrong answer |
| interpretation / normalization | yes — it shapes what both sides *see* |
| semantic analysis | yes — it shapes what both sides *conclude* |
| authoritative data | yes |
| reference derivation | yes |

`cargo xtask trust-baseline --propose` writes the forms with the review's part `unstated`. Nothing in the file can say
a form is accepted: acceptance is read where reviews are protected, never from a form's presence.

### The gate

`cargo xtask trust-gate` builds the commit's inventory and compares it with the forms its **base** commit holds. Its
report, `target/trust/report.txt`, names the build it describes and comes in two parts — here as it would read had the
scheduling checker come to compile the language's model, which is no role's own:

```text
trust-gate report — docs/specs/trust/decision_trust-inventory.md §6
commit: …
host: x86_64-unknown-linux-gnu
inventory: sha256 …
baseline: sha256 … — trust/baseline.eadl of the base commit …
verdict: passed

== the change, against the base commit's forms ==
trust-new-shared: `generator+scheduling-checker package crates/eadl-model`

== the standing list: nothing is accepted before M3.6.5, so every entry is unreviewed ==
shared item `generator+scheduling-checker build-configuration`: proposed, classification unstated
…
```

The **change** says what this commit shares that its base did not, or shares differently, which root forms it
added, changed or removed against its base's — a comment is no change — and which program targets are not classified
or have outgrown their classification; or *unchanged*. The
**standing list** names everything still awaiting review. A commit that shares something new must carry the form the
tool proposes for it, so the sharing is reported once, by the commit that makes it, and the next unrelated commit
reads *unchanged*.

### The package verifier

A report that a property holds rests on results produced by particular programs. `cargo xtask trust-verify <package>`
checks a package that carries the trust build's inventory, report and programs: that they are one commit's and one
toolchain's, that each result was produced by the program its role names, and that every file handed to a program at
run time was declared.

## The precise rules

The design is `docs/specs/trust/decision_trust-inventory.md`, reviewed round by round until a round found no defect;
its section numbers are used below. The code is `xtask/src/trust.rs` (the inventory), `xtask/src/trust_gate.rs` (the baseline and the
gate) and `xtask/src/trust_verify.rs` (the verifier); committed generated sources are
`docs/specs/trust/decision_trust-generated-sources.md` and `xtask/src/trust_generated.rs`.

### The run's order

The inventory decides in steps, each running only when those before it refused nothing (the generated-sources record
§2): first the forms' text, nothing built; then the programs' builds and the data their forms hand them, with every
refusal the rules below make before or in those builds. A step that refuses ends the run and leaves no inventory — not
even one an earlier run wrote, which would read as this commit's. A step in which a build fails still runs to its end,
every build in it made, and then ends unable to judge (exit 2), the refusals it found printed beside the failure. A
form of `trust/roots.eadl` other than a `defgenerated` one that departs from its shape also leaves the gate unable to
judge, the `defgenerated` refusals read before it printed beside it. The third step judges every form a program
reads: a generator or input that is a symbolic link or no blob of the commit is refused, and so is one a form declares
or whose header marks it generated, a chain refused until `M3.6.6.4` decides one; a form no program reads is held to its
text alone. The fourth judges each such form's generator files against each program that reads it: a program
target's entry point whose role packages, or the union of them when it is the entry point of more than one target, hold
another role's is refused and not built, and is otherwise built and checked as a root is, the manifest rules first;
a `.rs` file that is no program target's entry point is refused; a script lying in another role's package is refused.
A build that fails there ends the run unable to judge, as in the second step. One more step — an undeclared generated
file — is `M3.6.6.2`'s, under way.

### The codes

| Code | Outcome | When |
| --- | --- | --- |
| `trust-new-shared` | reported | an item the base commit's baseline does not hold, a generated provenance among them |
| `trust-shared-changed` | reported | an item whose content, configuration, edges or readers differ from the base commit's form, or a generated provenance whose content, roles or declared files do |
| `trust-unclassified-program` | reported | a program target neither a root nor classified, or whose classification's role packages grew |
| `trust-baseline-stale` | refused, on the baseline's host | a form whose item is no longer shared, a classification whose target is gone, an admission no site uses; and, wherever the gate runs, a form naming a package or target the commit no longer has |
| `trust-undeclared-input` | refused | an input the commit does not hold, or a construct the catalog's rules refuse, with no admission; a `defgenerated` form departing from its shape, or two forms declaring one file; a generator or input of a form a program reads that is no blob, a symbolic link, or itself generated; a `.rs` generator file that is no program target's entry point |
| `trust-form-missing` | refused, on the baseline's host | a shared item with no form in the commit's own baseline, or with one proposing other digests than the commit measures; a program target with no classification |
| `trust-shared-program` | refused | a root compiling another role's role package; a generator of a file a program reads that holds or lies in a role package of another role |
| `trust-inventory-stale` | refused, by `trust-verify` | a package's inventory missing or of another build, its report of another inventory, an artifact or a result's producer not the inventory's, a handed dependency undeclared |

**Reported** passes and lists the item unreviewed; **refused** fails the gate wherever it runs, and is repaired by a
change to the code or to `trust/`.

### One host

A form's digests hold only on the host they were taken on: the build configuration every pair shares holds the
toolchain's identity, and `rustc -vV` names its host. The baseline names its host, the CI runner's
(`x86_64-unknown-linux-gnu`). On another host the gate builds and applies every refusal but `trust-baseline-stale` and
`trust-form-missing`, lists every item unreviewed, and its change part says *not compared*, never *unchanged*.

### §14.4's cases

Each is a test of the gate in a scratch workspace, judged commit by commit by the real instrument (`M3.6.3.6`):

| Case | What changes | What the gate says |
| --- | --- | --- |
| 1 | a package both roots come to compile | `trust-new-shared`; `trust-form-missing` until its form is committed |
| 2 | a shared package's source edited, a feature of it activated | `trust-shared-changed`, naming the aspect |
| 3 | a data file both roots are handed; an input declared but absent | `trust-new-shared`; `trust-undeclared-input` |
| 4 | a package carrying an earlier commit's inventory | `trust-inventory-stale` |
| 5 | a README, a development profile, an override for a package no root compiles, a comment in the pin | *unchanged*, nothing refused |

Every code is removed in turn by a catalogued mutation the tests must kill, and a test holds the catalogue to the
record's table, so a code the record gains with nothing removing it fails.

### Where it runs

- In CI, `.github/workflows/trust-gate.yml`, on every pull request, the merge queue and every push to `main`, with no
  path filter. The gate is built from the base commit by the catalog's harness, so a change to the gate is judged by
  the gate it changes nothing of ([Verifying the toolchain](verification.md)).
- In the `assurance` tier's `trust-inventory` step, which fails on a refusal and is otherwise *not built*: a pass
  would claim forms accepted, and none can be before `M3.6.5`.
- In packaging, `trust-verify`, once `M4.7` writes packages.

### What it is not

A proof of semantic independence. Different crate names do not establish independent derivation; a copy edited after
copying, or a formula re-derived by hand, is not seen. The gate reads `trust/` with the generator's own reader, so a
fault there that misreads a form is shared with the generator, which is why a form's review reads its text.

## Today and ahead

Measured `2026-10-06`: the roots are the generator, the scheduling checker, the reference model and the
implementation, with the comparison harness beside them, and what they share is every pair's build configuration and
the harness the reference model and the implementation share. The configuration checker has no root yet (`M3.5`). The
gate passes with *not compared*, since no baseline is committed yet. The runner's baseline is proposed by the `trust-gate`
workflow's first run, and committed then (`M3.6.3.2.1`). Acceptance — who may accept a form, read on the protected
main line — waits on the director's protection of `main` and a second reviewer (`M3.6.5`). A committed generated
source, which shares its generator's mistakes with whatever reads it, is `M3.6.6`'s: its form is read today
(`M3.6.6.2.1`), what it shares is recorded (`M3.6.6.2.2`), what a generator may be is judged (`M3.6.6.2.3`), an undeclared one's refusal is next (`M3.6.6.2.4`), and this
chapter's whole account of it is `M3.6.6.3`'s. The package the verifier reads is
fixed provisionally until `M4.7` writes real ones.
