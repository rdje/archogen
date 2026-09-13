# The `osgen` command line

`osgen` is the single entry point to the toolchain. Its command surface is fixed by the
interface target in `ROADMAP.md` §10.2 and is declared in one table
(`crates/osgen-cli/src/spec.rs`), from which both the help text and the parser are derived —
so a documented option is always an accepted option, and the reverse.

```console
$ osgen --help
osgen — generate a specialized operating system from a functional eADL description

USAGE:
    osgen <COMMAND> [OPTIONS]
    osgen help <COMMAND>

COMMANDS:
    check    elaborate and type-check a description against a profile   [unimplemented — tracked by leaf M1.8]
    resolve  resolve providers and resources into an independently checked build plan   [unimplemented — tracked by leaf M3.4]
    build    generate, assemble and build a complete system and its simulator   [unimplemented — tracked by leaf M4.2]
    analyze  run an analysis for one named property over a build   [unimplemented — tracked by leaf M2.6]
    verify   run a verification tier over a build   [unimplemented — tracked by leaf PROGRAM.3]
    explain  explain how one requirement was realized, or why it could not be   [unimplemented — tracked by leaf M3.4]
    replay   replay a recorded failure manifest and check its identity   [unimplemented — tracked by leaf M4.7]
```

## The commands

| Command | Does | Built by |
| --- | --- | --- |
| `osgen check <DESCRIPTION> [--profile <PROFILE>]` | elaborate and type-check a description against a profile | leaf `M1.8` |
| `osgen resolve <DESCRIPTION> [--profile <P>] [--locked] --out <PLAN>` | resolve providers and resources into an independently checked build plan | leaf `M3.4` |
| `osgen build <DESCRIPTION> [--profile <P>] [--locked] --out <DIR>` | generate, assemble and build a complete system and its simulator | leaf `M4.2` |
| `osgen analyze <BUILD> --property <PROPERTY>` | run an analysis for one named property | leaf `M2.6` |
| `osgen verify <BUILD> --tier <TIER>` | run a verification tier | leaf `PROGRAM.3` |
| `osgen explain <BUILD> --requirement <REQUIREMENT>` | explain how one requirement was realized, or why it could not be | leaf `M3.4` |
| `osgen replay <MANIFEST>` | replay a recorded failure manifest and check its identity | leaf `M4.7` |

Every command is currently **unimplemented**, and says so precisely:

```console
$ osgen check examples/periodic-three/system.eadl --profile rt-static-up-v1
osgen: unimplemented: `osgen check` is not implemented yet
  hint: it is part of the interface target in ROADMAP.md §10.2; the work is tracked by task-tree leaf M1.8 (docs/TASK_TREE.md)
$ echo $?
20
```

An unbuilt command names the task-tree leaf that will build it. A gap in this toolchain is
tracked work, not an unknown.

## Exit codes

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
| `2` | `usage` | the command line was malformed |
| `20` | `unimplemented` | the command is part of the §10.2 target but is not built yet |

`tool-failure` sits with the diagnostic results because §5.5 defines it there, but it shares
its code with nothing: a toolchain that failed has not produced a valid system, and must
never be readable as one. `unimplemented` is temporary — it disappears command by command as
the owning leaves land.

## Diagnostics

Every refusal states what happened and what to do about it, because §5.5 requires a concrete
repair direction on every diagnostic:

```console
$ osgen resolve system.eadl
osgen: usage: `osgen resolve` requires `--out`
  hint: osgen resolve <DESCRIPTION> [--profile <PROFILE>] [--locked] --out <PLAN> — where to write the resolved plan

$ osgen check system.eadl --strict
osgen: usage: unknown option `--strict` for `osgen check`
  hint: `osgen check` accepts: --profile
```

Asking a command what it needs never requires satisfying it first — `osgen resolve --help`
and `osgen help resolve` both work with no other arguments.

## What it is built from

The CLI depends on the Rust standard library and nothing else. That is a deliberate,
recorded decision (`docs/decisions/decision_zero-dependency-engine-core.md`): §4.4 makes
every dependency shared between the generator and the checker a reviewable trust event, §10.3
requires locked offline builds, and §5.5 makes diagnostic wording part of the user contract.
