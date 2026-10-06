# What a report may claim

## The idea, in plain words

A medical check-up does not end with "you are healthy". It ends with separate results — blood pressure, this
blood test, that scan — each saying what was measured, how, and what it can and cannot tell. archogen's reports work
the same way. A system has several properties worth knowing: whether its configuration is valid, whether it behaves
correctly, whether it meets its deadlines, whether it fits in memory, whether it starts up properly. The report
gives each one its own answer, and each answer says what kind of evidence it rests on: a check of the description,
an analysis under stated assumptions, tests run so far, a proof, a measurement on one particular board.

So there is no single word such as "verified" that covers everything, and a property nobody has looked at is never
silently counted as fine.

> **In one minute, for engineers.** `ROADMAP.md` §7.1 prohibits a global "verified" flag, and
> `crates/archogen-evidence/` encodes it three ways: no method aggregates a report's claims, a report with a property
> left unclaimed refuses to render, and every positive conclusion is a typed claim that cannot be built without its
> qualifier — the named constraints, the listed assumptions, the recorded coverage, the model, the refinement or the
> target. Bounds carry their provenance, trust dependencies are declared, and artefacts are named by content hash.

## How it works

archogen reports do not say "verified". They cannot: there is no such word in the vocabulary,
and the absence is enforced by the code that builds them.

`ROADMAP.md` §7.1 opens with a prohibition rather than a feature:

> The report contains separate statuses for configuration validity, runtime functional
> behavior, timing, memory bounds, startup behavior, and any future isolation property.
> **One global "verified" flag is prohibited.**

A prohibition written in a document is one somebody violates under deadline. This one is
encoded three ways (`crates/archogen-evidence/`).

### 1. There is no aggregate verdict

A report is a set of per-property claims. It offers no method that collapses them, so you
cannot ask whether "it" passed — the question has no referent.

### 2. Silence about a property is not a pass

A report refuses to render while any property lacks a claim:

```text
report is incomplete: no claim for configuration-validity, runtime-functional-behavior,
memory-bounds, startup-behavior — a property with no claim is not a pass
```

The only way to say nothing about a property is `not applicable`, which must still say *why*.

### 3. Every positive conclusion carries its qualifier

There is no `Verified`, no `Passed`, no bare `Holds`. Each conclusion names what makes it
true, and cannot be constructed without it:

| Evidence category | What it may conclude |
| --- | --- |
| structural check | "the checked configuration satisfies **these named constraints**" |
| conditional analysis | "holds in **model M** under **these listed assumptions**" — an empty list is refused |
| tested conformance | "no violation was observed within **this recorded coverage**" |
| model proof | "**invariant I** holds for the stated formal **model M**" |
| implementation refinement | "transfers through **refinement R** and **its assumptions**" — an empty list is refused |
| target evidence | "applies only to **target T**, **binary B**" |

A conditional analysis with an empty assumption list renders as "holds", full stop — the same
overstatement wearing a different hat. It is refused:

```text
claim for `timing` is malformed: a conditional analysis with no listed assumptions is an
unconditional claim
```

Three sentences from §7.1 shape the whole table:

> Testing does not become proof through repetition. A published algorithm proof does not
> automatically verify its Rust implementation. A declared capability does not constitute
> hardware evidence.

So `tested conformance` has no route to a stronger conclusion, no matter how many runs.

## The precise rules

### Bounds remember where they came from

Every numerical bound records its units, scope, target, **binary identity**, and origin.

| Origin | Established bound? |
| --- | --- |
| assumed | no |
| observed maximum | no |
| externally supplied | yes, within the source's own scope |
| analytically established | yes |

§7.3: *"An observed maximum with a safety multiplier remains an empirical assumption unless a
valid argument establishes a bound."* So the safety factor lives **inside** the observation,
and cannot promote it — a factor of 1000 leaves it an observation. Multiplying a measurement
by 1.5 and calling the result a bound is the most common way a timing claim becomes untrue,
and it is untrue in a way that looks like diligence.

Safety factors are exact rationals (`3/2`), not floats: §7.4 requires exact integer or checked
rational arithmetic, and a binary float would introduce a rounding question in the one place
nobody would look for one.

A bound with no binary identity is refused outright:

```text
`binary`: a bound not tied to a binary is invalidated by any rebuild and nobody can tell
```

That is §15's point — a compiler flag change can invalidate a timing bound while the eADL
description is byte-identical.

### Trust dependencies

The generator and the independent checker are meant to reach their verdicts separately. §4.4's
position is not that sharing is forbidden — it is that sharing must be **visible**.

Each inventoried item records its identity, version, **content hash**, role, and which roots
reach it. The hash is what catches a change behind an unchanged name and version. The role is
what keeps the gate meaningful:

| Role | Shared with the generator costs independence? |
| --- | --- |
| infrastructure | no — a shared allocator cannot make two implementations agree on a wrong answer |
| interpretation / normalization | yes — it shapes what both sides *see* |
| semantic analysis | yes — it shapes what both sides *conclude* |
| authoritative data | yes |
| reference derivation | yes |

An unrelated change produces **no** warning. §14.4 requires that too, and a gate that cries
wolf is a gate that gets disabled.

The design is `docs/specs/trust/decision_trust-inventory.md`, and its measuring instrument is built:
`cargo xtask trust-inventory` (`xtask/src/trust.rs`) builds each **root** — the program that runs a role, such as
the `archogen` executable for the generator — clean from the commit's own files, reads what the compiler read for
it, and writes `target/trust/trust-dependencies.json` with every item two roots share. The roots are declared in
`trust/roots.eadl`, and so is every other **program target** of the workspace — an executable, an example, the
browser module — as not a root, with a reason and the role packages its build compiles. A program target the file
does not classify, or one that has started compiling another role's package, is reported for review
(`trust-unclassified-program`), so no one can add a checker beside the generator unseen (leaf `M3.6.3.1`).
`cargo xtask trust-baseline --propose` writes the **baseline** a review starts from, `trust/baseline.eadl`: one form
per shared item with the digests the inventory measured, and the review's part — the item's classification, the
property it can affect, its residual risk, the controls that remain — left `unstated`; nothing in the file can say a
form is accepted (leaf `M3.6.3.2`). Its digests hold only on the host they were taken on, and the baseline the gate
compares against is the CI runner's, so it is proposed there.
`cargo xtask trust-gate` judges a commit against its base (leaf `M3.6.3.3`): it builds the inventory and writes
`target/trust/report.txt` in two parts. The **change** names each shared item new since the base commit's baseline
(`trust-new-shared`) or measured otherwise (`trust-shared-changed`), or says *unchanged*; the **standing list** names
every shared item, root, classification and admission, none accepted until reviews are read where they are protected
(`M3.6.5`). On the baseline's host the gate refuses a commit whose own `trust/` holds no form for something it shares
or builds (`trust-form-missing`), or keeps one for something gone (`trust-baseline-stale`); off it, it compares
nothing and says so. A form naming a package the commit no longer has is refused everywhere, since nothing can be
built without it. In CI the gate is built from the base commit and runs on every pull request and every push to
`main` ([Verifying the toolchain](verification.md)); in the `assurance` tier its step fails on a refusal and is
otherwise *not built*, never a pass, until forms can be accepted (`M3.6.5`).
`cargo xtask trust-verify <package>` checks a package that carries an inventory, where a claim is consumed (leaf
`M3.6.3.5`): it refuses one whose inventory is missing or of another commit or toolchain, whose report names another
inventory, whose artifacts are not the inventory's, in which a result was produced by anything but its role's
inventoried program — or, for the reference model, its pair's comparison harness — or which hands a root a
dependency its form does not declare (`trust-inventory-stale`). The package's layout is fixed there provisionally,
since the package is `M4.7`'s to write.

⚠️ The honest limit, from §4.4 itself: *"This check enforces disclosure and change control; it
does not prove semantic independence."* Two separately written implementations of the same
misread specification share nothing any inventory can see.

### Content hashes

A binary identity and a trust item's content hash are both SHA-256 digests, written `sha256:` followed by the
digest in lowercase hexadecimal. The catalog of engine knowledge will identify each record the same way, and a
review of a record will hold only at the digest it names. So the digest decides whether a review still holds, and
it is computed inside the engine rather than by the system's `shasum`, because the engine runs no other program.

The function is `archogen_evidence::sha256` (`crates/archogen-evidence/src/sha256.rs`). Three checks stand
outside it:

- its constants are derived again, in exact integer arithmetic, from their definition in the standard: the
  fractional parts of the square and cube roots of the first primes;
- it reproduces the standard's published examples, the one-million-character message included;
- for every message length that puts the padding at a different place in one block or across two, its digest
  equals the one the system's own tool computed (`crates/archogen-evidence/tests/fixtures/sha256_boundaries.txt`).

The last check is the one that matters most. The standard's published messages all happen to leave the padding
somewhere harmless, so a hand-written SHA-256 that pads one byte too early passes every one of them. It fails the
boundary fixture, and that mistake is kept in the mutation catalogue to prove it stays caught.

```console
$ cargo test -p archogen-evidence --test sha256    # the published examples and every boundary length
$ bash scripts/sha256_fixture.sh --check           # the fixture against a fresh run of the system's tool
```
