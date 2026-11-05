# The `archogen` command line

## The idea, in plain words

`archogen` is a program you run from a terminal, and everything the toolchain does starts there: `archogen check`
reads a description and says whether it is acceptable, `archogen build` generates a system from it. Each command
says honestly how finished it is — the help text marks a command that works only in an early, narrow form as
*experimental*, and one that does not exist yet as *unimplemented*, with the piece of work that will build it — so
you never mistake a placeholder for a feature. And when something goes wrong, the program's exit code tells a
script what kind of problem it was, using the same verdicts a person reads.

> **In one minute, for engineers.** The command surface is `ROADMAP.md` §10.2's interface target, declared once in
> `crates/archogen-cli/src/spec.rs`, from which the help text and the parser are derived, with a three-state
> maturity per command (built, experimental, unimplemented with its owning leaf) and, for the programmatic surface,
> whether it is offered as a tool. `check` is built; `build` is experimental, on the hosted playground; the rest name their
> owners. `archogen mcp`, §10.4's server, offers the tools to an agent over [MCP](ledger.md#mcp-specification). Exit codes map totally from the verdicts;
> diagnostics carry spans and repairs.

## How it works

`archogen` is the single entry point to the toolchain. Its command surface is fixed by the
interface target in `ROADMAP.md` §10.2 and is declared in one table
(`crates/archogen-cli/src/spec.rs`), from which both the help text and the parser are derived —
so a documented option is always an accepted option, and the reverse.

```console
$ archogen --help
archogen — generate a specialized operating system from a functional eADL description

USAGE:
    archogen <COMMAND> [OPTIONS]
    archogen help <COMMAND>

COMMANDS:
    check       elaborate and type-check a description against a profile
    resolve     resolve providers and resources into an independently checked build plan   [experimental — one system of a description, over its own blocks and platforms; completed by leaf M3.7]
    plan-check  check a resolved plan against its description, by the independent configuration checker
    build       generate, assemble and build a complete system and its simulator   [experimental — the hosted playground: the plan's releases run, and no job holds application code; completed by leaf M4.12]
    analyze     run an analysis for one named property over a build   [unimplemented — tracked by leaf M2.21.2]
    verify      run a verification tier over a build   [unimplemented — tracked by leaf M4.11]
    explain     explain how one requirement was realized, or why it could not be   [experimental — one requirement of one system, over a description's own blocks and platforms; completed by leaf M3.7]
    replay      replay a recorded failure manifest and check its identity   [unimplemented — tracked by leaf M4.7]
    mcp         serve the engine API to an agent over MCP, on standard input and output
```

## The precise rules

### The commands

| Command | Does | Built by |
| --- | --- | --- |
| `archogen check <DESCRIPTION> [--profile <PROFILE>]` | elaborate and type-check a description against a profile | **built** — a description is one file or a module tree; see [Checking a description](checking.md) and [Modules and composition](modules.md) |
| `archogen resolve <DESCRIPTION> [--profile <P>] [--locked] --out <PLAN> [--unchecked] [--system <NAME>]` | resolve providers and resources into an independently checked build plan | **experimental** — one system of a description, a module tree among them, over its own blocks and platforms, the plan checked by `archogen-plan-check`; `--locked` refused until `M4.7`; `M3.7` adds the catalog's providers, completing it |
| `archogen plan-check <DESCRIPTION> <PLAN>` | check a resolved plan against its description, by the independent configuration checker | **built** — runs the configuration checker's own executable, `archogen-plan-check`, from an install, its arguments handed over unparsed; see [Checking a plan](plan-check.md) |
| `archogen build <DESCRIPTION> [--profile <P>] --out <DIR> [--system <NAME>]` | generate, assemble and build a complete system and its simulator | **experimental** — the hosted playground, see [Building a system](building.md); leaf `M4.12` completes it |
| `archogen analyze <BUILD> --property <PROPERTY>` | run an analysis for one named property | leaf `M2.21.2` |
| `archogen verify <BUILD> --tier <TIER>` | run a verification tier | leaf `M4.11` |
| `archogen explain <DESCRIPTION> [--profile <P>] --requirement <FACT> [--requester <NAME>] [--task <NAME>] [--system <NAME>]` | explain how one requirement was realized, or why it could not be | **experimental** — one system of a description, over its own blocks and platforms, the search's own account, its plan unchecked (leaf `M3.4.13.2`); §10.2's form reads a build, which `M4.2` completes |
| `archogen replay <MANIFEST>` | replay a recorded failure manifest and check its identity | leaf `M4.7` |
| `archogen mcp` | serve the engine API to an agent over [MCP](ledger.md#mcp-specification), on standard input and output — §10.4's server, not one of §10.2's seven | **built** — leaf `API.6.4`; the tools it offers are this table's, `build` and `verify` excluded |

`check` is built, over one file or a module tree. A module file is **elaborated** from its module path
(`docs/semantics/reference.md` §6 rule 7), its names are resolved (§6 rules 9 and 10), and its
declarations are **type-checked** by every pass a single description gets. The summary said "elaborate
and type-check" from the day `spec.rs` was written (`2026-09-13`) while no command elaborated anything,
until leaf `M1.29.1`; then it stated the two halves apart while only one was true of a module tree
(`M1.29.2`); two legs in `crates/archogen-cli/tests/module_files.rs` couple each word to its capability,
so the sentence can neither run ahead of the code nor lag behind it.

`build` **runs, but short of its §10.2 contract**, and the
surface has a third state to say so rather than lying in either direction:

```console
$ archogen help build | tail -5
STATUS: EXPERIMENTAL
    This command runs, but only over the hosted playground: the plan's releases run, and no job holds application code.
    Its output carries no timing, assurance, or OS-completeness claim. Completing it
    to the ROADMAP.md §10.2 contract is task-tree leaf M4.12
    (see docs/TASK_TREE.md).
```

Two states would have forced a choice between two false statements. Marking it *built* promises
"a complete system and its simulator"; marking it *unimplemented* denies a command that works,
and a help text that lies about a gap teaches users to stop reading it.

The commands not built yet say so precisely:

```console
$ archogen replay artifacts/failure/replay.json
archogen: unimplemented: `archogen replay` is not implemented yet
  hint: it is part of the interface target in ROADMAP.md §10.2; the work is tracked by task-tree leaf M4.7 (docs/TASK_TREE.md)
$ echo $?
20
```

An unbuilt command names the task-tree leaf that will build it. A gap in this toolchain is
tracked work, not an unknown.

The `[unimplemented]` marker comes from the same table the parser validates against, so a
command becomes real and stops advertising itself as unbuilt in **one** change — a test asserts
that `check` no longer carries the marker, `replay` still does, and `resolve` and `explain`, built `2026-10-27`, say
they are experimental.

`archogen resolve` (leaf `M3.4.4`) chooses one system's providers and writes the plan, `archogen-assignment/2`
([Matching](matching.md#the-search)), then runs the configuration checker over it, so the plan it reports was judged
by a program that shares none of its code; with `--unchecked` it writes the plan alone and says so:

```console
$ archogen resolve system.eadl --out plan.json --unchecked
system.eadl: resolved system `heartbeat` against profile `rt-static-up-v1` into plan.json (1 binding(s))
  experimental: one system of a description, over its own blocks and platforms; completed by M3.7
archogen: unchecked: `--unchecked` — the plan was not checked by `archogen-plan-check`; nothing independent judged it
$ cat plan.json
(format "archogen-assignment/2")
(profile rt-static-up-v1)
(system heartbeat)
(program "69e3669f8a0565716da5d870bd4083f4a4de025c447c4697049e2ca2f2b1674b")
(adapters adapters/1)
(bind console.write observable-output console.uart)
```

That is `examples/s0-heartbeat/system.eadl`. A plan that binds an engine adapter holds only once the adapter's code
keeps what its kind promises, and the command says so under its first line. Each line names the instance, what it owes
— an `obligation` the realization must discharge, or an `interrupt` the scheduling analysis charges — its name, and what
it means (leaf `M3.2.6.1`, [The engine API](engine-api.md#the-second-operation-resolve-the-search-alone)). Over
`crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl`, a short counter extended by the engine:

```text
  conditional: it binds engine adapters, and holds once each obligation below is discharged and each cost charged
    (adapter epoch-extender timer.short) obligation `update`: the epoch is advanced by a wake programmed within the provider's unambiguous horizon
    (adapter epoch-extender timer.short) obligation `coherent`: a read joins epoch and counter so that no update is torn
    (adapter epoch-extender timer.short) interrupt `wake`: one interrupt at its provider per update period, the period the realization states and no longer than that horizon
```

`archogen explain` prints the same lines under the plan it explains. Without `--unchecked`, an install's
`archogen-plan-check` judges the plan,
its record written in `plan.json.check/`, and the command's status is the checker's verdict. A description the search
cannot resolve writes no plan and exits with the search's verdict — `periodic-three` is `missing-fact` today, its
timer offering a width and no modulus and its policy's constraints waiting on the catalog — and `--locked` is refused
until lock data exists (`M4.7`). `--system` names the system when a description declares several.

**An output replaces only what its own kind wrote.** Everything a run writes stays in a private directory, `PLAN.part`, where the
checker judges it, and it lands only on acceptance, or under `--unchecked`: the record in `PLAN.check/`, then the plan at `PLAN`. So what
stands at either name stands until there is a plan that may replace it, and a plan replaces only a plan or an empty file, a record only a
record or an empty file. A plan is known by the engine's own reader of the format, which takes only the text the search writes. So a
description, a module, a file of notes, a plan with anything written in it and a plan of another format are all no plan, however each is
named. `crates/archogen-cli/src/place.rs` is how this command and
`archogen build` read a place, and [Building a system](building.md) has `build`'s table. Each row below is a fixture of
`crates/archogen-cli/tests/resolve.rs`, which runs it; a row with no fixture, or a fixture with no row, fails. A route it does not name is
promised nothing and worth reporting. ⚠️ As for a build, the standard library has no rename that refuses to replace, so each path is held
again just before it is used, and in that instant what stands there is what is used.

<!-- machine-read: plan-routes -->
| Route | What stands there | Status | It says | What is left |
| --- | --- | --- | --- | --- |
| `no-name` | `--out .` | `usage` | `names no file of its own` | nothing made |
| `empty-name` | `--out ''` | `usage` | `names no file of its own` | nothing made |
| `a-trailing-dot` | `--out q/.` | `usage` | `names no file of its own` | nothing made |
| `a-trailing-separator` | `--out q/` | `usage` | `names no file of its own` | nothing made |
| `back-through` | `--out missing/../d` | `usage` | `goes back through a name` | nothing made |
| `a-directory` | a directory at `PLAN` | `usage` | `is a directory` | the directory |
| `a-link` | a link at `PLAN` | `usage` | `is a link` | the link, and what it points at |
| `a-pipe` | what is no file at `PLAN` | `usage` | `is no file` | it |
| `no-plan` | a file at `PLAN` that is no plan | `usage` | `is no plan` | the file |
| `a-plan-with-notes-in-it` | a plan with anything written in it, in any place, or of another format | `usage` | `is no plan` | the file, with its notes |
| `closed-to-reads` | a file at `PLAN` this user cannot read, so nothing can tell it is a plan | `usage` | `to tell it is a plan` | the file |
| `its-own-description` | the description itself at `PLAN`, with `--unchecked` or without it | `usage` | `is no plan` | the description |
| `its-own-description-by-a-descriptor` | the description at `PLAN`, itself given as `/dev/fd/3` | `usage` | `is no plan` | the description |
| `a-module-it-imports` | a module the description imports at `PLAN` | `usage` | `is no plan` | the module |
| `reads-once` | a description that is no regular file, such as a pipe or `<(…)`, which the checker would read again | `usage` | `no regular file` | any plan at `PLAN`, and its record |
| `reads-differently` | a description that reads differently the second time, such as `/dev/stdin` from a file | `usage` | `reads differently` | nothing made |
| `a-module-that-reads-once` | a module that is no regular file | `usage` | `no regular file` | nothing made |
| `part-stands` | anything at `PLAN.part`, which a run that stopped may have left, or a refused one for a report | `usage` | `a plan is written there first` | it |
| `check-stands` | a file at `PLAN.check`, where the checker's record goes | `usage` | `stands where the checker's record goes` | it, and any plan at `PLAN` |
| `check-a-link` | a link at `PLAN.check` | `usage` | `stands where the checker's record goes` | it, what it points at, and any plan at `PLAN` |
| `check-a-link-unchecked` | a link at `PLAN.check`, with `--unchecked`, which writes no record | `ok` | `resolved system` | the plan; the link, and all it points at |
| `record-no-record` | what is no record at `PLAN.check/plan-check.json` | `usage` | `is no record` | it |
| `record-no-record-unchecked` | what is no record at that name, with `--unchecked` | `ok` | `resolved system` | the plan, and it |
| `check-reused` | a directory at `PLAN.check` holding an earlier record | `ok` | `resolved system` | the plan; in that directory the record replaced, and all else as it was |
| `a-record-waiting` | a file at the name a record waits at on its way, which a run that stopped may have left | `usage` | `a record waits there` | it |
| `check-closed` | a directory at `PLAN.check` that takes no record | `tool-failure` | `cannot put the checker's record` | what stood at `PLAN`; nothing this run wrote |
| `check-unsearchable-unchecked` | a `PLAN.check` that cannot be searched, with `--unchecked`, so nothing can tell what record it holds | `usage` | `cannot tell whether an earlier record` | the plan and the record as they were |
| `record-unreadable` | a record that cannot be read at that name | `usage` | `cannot read` | the plan and the record as they were |
| `a-record-keeps-its-mode` | an earlier record at a mode of its own | `ok` | `resolved system` | the plan; the new record at that mode |
| `flagged-plan` | a flag on `PLAN`, found at the last step, where the platform has flags | `tool-failure` | `cannot write the plan` | the plan and the record as they were; nothing this run wrote |
| `flagged-plan-unchecked` | a flag on `PLAN`, with `--unchecked` and a file at the name a record waits at | `tool-failure` | `cannot write the plan` | the plan, and that file |
| `flagged-record` | a flag on the earlier record, where the platform has flags | `tool-failure` | `cannot move the earlier record` | the plan and the record as they were; nothing this run wrote |
| `came-in-stage` | while the checker runs, something comes in the stage beside the plan | `ok` | `holding what came in it` | the plan and its record; what came in `PLAN.part`, which the next run names |
| `stage-swapped` | while the checker runs, the stage is swapped for a link | `tool-failure` | `no longer the directory this run made` | what stood at `PLAN`; the link |
| `stage-replaced` | while the checker runs, another directory comes to stand at the stage's name | `tool-failure` | `wrote no record` | what stood at `PLAN`; that directory, with what it holds |
| `plan-edited-meanwhile` | while the checker runs, the plan it is judging is edited | `tool-failure` | `changed while the plan was judged` | what stood at `PLAN`; the stage, to be looked at |
| `check-stands-unchecked` | a file at `PLAN.check`, with `--unchecked` | `ok` | `resolved system` | the plan, and that file |
| `no-room-name` | a name with no room for what goes beside it | `usage` | `too long` | nothing made |
| `no-room-check` | a name with room for `.part` and none for `.check` | `usage` | `too long` | any plan at `PLAN` |
| `no-room-check-unchecked` | a name with no room for `.check`, with `--unchecked`, which needs none | `ok` | `resolved system` | the plan |
| `no-room-record` | a path with no room for what the checker writes, as it is handed the path | `usage` | `leaves no room in a path` | nothing made |
| `closed-parent` | a parent this user cannot write | `usage` | `reach and write` | the parent, empty |
| `file-on-the-way` | a file among `PLAN`'s parents | `usage` | `is no directory` | the file |
| `link-on-the-way` | a link to nothing among `PLAN`'s parents | `usage` | `a link to nothing` | the link |
| `link-to-a-directory-on-the-way` | a link to a directory among `PLAN`'s parents | `ok` | `resolved system` | the plan, beneath what the link points at |
| `link-loop-on-the-way` | a loop of links among `PLAN`'s parents | `usage` | `name another place` | the links |
| `absent` | nothing | `ok` | `resolved system` | the plan, its directories made, and its record in `PLAN.check` |
| `over-an-empty-file` | an empty file at `PLAN` | `ok` | `resolved system` | the plan |
| `over-a-plan` | a plan at `PLAN`, its mode one this user can read | `ok` | `resolved system` | the plan, at that file's mode and group |
| `unchecked-over-a-checked` | a checked plan at `PLAN`, with `--unchecked` | `ok` | `resolved system` | the plan; the earlier record gone, since it would read as this plan's |
| `refused` | the checker refuses the plan | `tool-failure` | `refuses the plan the search wrote` | what stood at `PLAN` and its record; the plan and this record in `PLAN.part`, for the report |
| `record-unsound` | the checker's record does not hold together with what it did | `tool-failure` | `does not hold together` | what stood at `PLAN`; the plan and the record in `PLAN.part`, for the report |
| `no-record` | the checker runs and writes no record | `tool-failure` | `wrote no record` | what stood at `PLAN` and its record; nothing this run wrote |
| `no-checker` | no install beside the command, so no checker can be run | `tool-failure` | `has no install manifest` | what stood at `PLAN`; nothing this run wrote |
| `closed-meanwhile` | the checker accepts, and `PLAN.check` can no longer take a record | `tool-failure` | `cannot put the checker's record` | what stood at `PLAN` and its record; nothing this run wrote |
| `a-flag-as-its-value` | `--out --unchecked`, the parser taking the next word as `--out`'s value | `ok` | `resolved system` | a plan named `--unchecked` |

`archogen explain` (leaf `M3.4.13.2`) says what the search did with one requirement: `--requirement` names the fact,
and `--requester` or `--task` narrows it to one requester's; without either, each requester of the system requiring
it is explained. For each it prints where the requirement is written, every provider of the system's hardware and
its answer, and then the provider the plan binds, with what the joint rules say of each other one in its place:

```console
$ archogen explain system.eadl --requirement observable-output
`observable-output` required by `console.write`
  written at system.eadl:32:20
  `console.uart` meets it
  `host.playground` leaves it undescribed
  bound to `console.uart` by the search's plan
system.eadl: the plan of system `heartbeat` is the search's, unchecked: `archogen resolve` runs the configuration checker over the plan it writes
  experimental: one requirement of one system, over a description's own blocks and platforms; completed by M3.7
```

When no plan is written it says why — `no provider meets it`, with the search's own diagnostic, or `met alone`, with
what stopped the search — on standard error as `archogen resolve` prints it, and exits with the same verdict. A fact
no requester of the system requires is `usage`, its hint listing the requirements the system writes. It runs no
checker and claims none: the plan it explains is the one `archogen resolve --unchecked` writes. Given a directory
`archogen build` wrote, it reads that build, §10.2's form ([Building a system](building.md)).

**The checkers are programs of their own** (ruled `2026-10-22`, findings §16 (a); built by `M3.5.1.4`, `M3.5.6` and
`M2.21`). The configuration checker is `archogen-plan-check` and the scheduling checker `archogen-analyze`,
executables [an install](#installing) places beside `archogen`. `archogen plan-check` runs the first as git runs its
sibling commands, handing over its arguments unparsed — `--help` among them, so `archogen help plan-check` is the
checker's own help — and relaying its output and exit code (leaf `M3.5.1.4.1`); `archogen analyze` will run the
second so, and `archogen resolve` searches, then runs `archogen-plan-check` over the plan it wrote — or, with
`--unchecked`, writes the plan alone, a downgrade you ask for by name (leaf `M3.4.4`). The front runs only a sibling the install vouches for: the install manifest beside it,
`archogen-install/1`, must name the directory it lies in and the running `archogen`'s own fingerprint, and the
sibling's bytes must be the fingerprint the manifest gives it — so an older checker left beside a newer front is
refused, not run. A copy of `archogen` that no install placed runs no checker, and says how to install one. Each
checker writes a result record, `archogen-result/1`, and exits with its status's code. When the front consumes a
result — `archogen resolve` does, before it reports a plan — it believes the record only when its status and the exit
code agree, it names as its producer the program the front verified, and its subject is the very program and plan,
under the very profile, the front computed; anything else is `tool-failure` naming what disagreed, and a checker that
finds the description itself wanting is the two roles disagreeing, both verdicts named (leaf `M3.5.1.4.3`). So no program runs two roles' logic, and the generator's MCP server
offers neither checker's command: each checker serves its own tools — `archogen-plan-check --mcp` serves `plan-check`
([Checking a plan](plan-check.md)).

### Exit codes

Exit codes are a stable part of the contract — CI tiers and scripts branch on them. Two
families share the space and are deliberately kept apart.

**Diagnostic results** say what the toolchain concluded *about the submitted system*. They
are the `ROADMAP.md` §5.5 vocabulary:

| Code | Result | Meaning |
| --- | --- | --- |
| `10` | `invalid-description` | malformed, contradictory, or ill-typed input |
| `11` | `missing-fact` | relevant contract information is unavailable |
| `12` | `unsupported-profile` | the request lies outside implemented semantics |
| `13` | `infeasible-configuration` | supported constraints have no satisfying assignment |
| `14` | `analysis-inconclusive` | a resource limit or unresolved bound prevented a conclusion |
| `15` | `not-established` | a sufficient analysis did not establish the requested property |
| `16` | `counterexample` | a validated witness violates a named property |
| `70` | `tool-failure` | internal failure or an unavailable required tool |

**Process-level statuses** say what happened to *the invocation*. They make no claim about
any description:

| Code | Status | Meaning |
| --- | --- | --- |
| `0` | `ok` | the command completed and its result was affirmative |
| `2` | `usage` | the command line was malformed, or names what the command cannot use |
| `3` | `stale-input` | the inputs do not belong together — a plan resolved from another description, a build made from another plan — so nothing was judged; resolve, or build, again |
| `20` | `unimplemented` | the command is part of the §10.2 target but is not built yet |

`tool-failure` sits with the diagnostic results because §5.5 defines it there, but it shares
its code with nothing: a toolchain that failed has not produced a valid system, and must
never be readable as one. A write to standard output that fails is one too, for every command, whatever
the verdict was and with what the command did left standing, and it says so on standard error; a standard error nobody reads changes
nothing ([Building a system](building.md) says what neither can see). `unimplemented` is temporary — it disappears command by command as
the owning leaves land. `stale-input` is permanent: a checker answers it when what it was handed was not made
together, and it says nothing about the system either.

The vocabulary has a version of its own, `1.1`, since a checker carries it without the engine API
([What is versioned](versions.md)). Within its major a status keeps its name, its code and its meaning, and a new
one moves the minor.

### Installing

An install puts the `archogen` program, and the checkers it runs, in one directory, with a small file beside them
that says which build they came from. The front reads that file before it runs a checker, so a checker left over
from another build is refused rather than trusted.

```console
$ cargo xtask install --to /opt/archogen/bin
install: placed archogen and 0 sibling(s) in /opt/archogen/bin, from commit …; manifest archogen-install.json
```

`cargo xtask install` builds the trust inventory of the commit (`HEAD` unless `--commit` names another) and places
that build's executables: the generator's, `archogen`, and exactly the roots its form in `trust/roots.eadl` says it
invokes — the configuration checker's, `archogen-plan-check`, since leaf `M3.5.1.4.4`, and the scheduling checker's
when `M2.21` adds it. Each file is hashed before it is
copied and where it lands, and refused unless both equal the inventory's sha256. The directory must be absent or
empty, since an install removes nothing, and the manifest, `archogen-install.json` in the format `archogen-install/1`
([What is versioned](versions.md)), is written last, so a failed install leaves none. It names the directory, the
build's commit, tree, compiler and host, the inventory's sha256, and each program's name, role and sha256.

⚠️ The digests catch a checker that is stale or from another build, never tampering: whoever can write the
directory can replace the front and the manifest together.

### Diagnostics

Every refusal states what happened and what to do about it, because §5.5 requires a concrete
repair direction on every diagnostic:

```console
$ archogen resolve system.eadl
archogen: usage: `archogen resolve` requires `--out`
  hint: archogen resolve <DESCRIPTION> [--profile <PROFILE>] [--locked] --out <PLAN> [--unchecked] [--system <NAME>] — where to write the resolved plan

$ archogen check system.eadl --strict
archogen: usage: unknown option `--strict` for `archogen check`
  hint: `archogen check` accepts: --profile
```

Asking a command what it needs never requires satisfying it first — `archogen resolve --help`
and `archogen help resolve` both work with no other arguments.

### What it is built from

The CLI depends on the Rust standard library and nothing else. That is a deliberate,
recorded decision (`docs/decisions/decision_zero-dependency-engine-core.md`): §4.4 makes
every dependency shared between the generator and the checker a reviewable trust event, §10.3
requires locked offline builds, and §5.5 makes diagnostic wording part of the user contract.
