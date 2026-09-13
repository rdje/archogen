# TOOLBOX.md — the tools-first diagnostic doctrine

⛔ **TOOLS-FIRST.** For ANY unknown — a failure, a crash, a hang, a surprising result, a
"why isn't this working" — reach for a diagnostic tool FIRST. Never eyeball the code and
guess a root cause.

## The rule

- A code change cannot land without **tool-backed WHY + WHERE** and a **measured
  before→after** recorded in its task-tree leaf (see the acceptance checklist in
  `DOCTRINE_ENFORCEMENT.md`).
- If no existing tool shows WHY+WHERE, **build one** — a probe, a tracer, a counter, a
  minimal reproduction harness. The diagnostic tool is a first-class deliverable, kept in
  the repo, not a throwaway.
- **ANTI-SPIN TRIPWIRE:** if you have analyzed for ~2 turns without producing NEW tool
  output that pinpoints WHY+WHERE, STOP — run a tool, build one, or escalate. Never loop
  on analysis.

## The 3-step UNKNOWN protocol (adapt the specific tools to your domain)

1. **WIDEN** — dump the full picture: enumerate all cases/states, the broadest inventory,
   so the failing one is visible in context.
2. **NARROW** — probe the specific failing case for its exact verdict + position/state.
3. **PINPOINT** — a scoped trace that names the exact function/rule/line that fails and why.

The point is to convert "it's broken somewhere" into "line X of function Y rejects input Z
because predicate P is false" before writing a single line of fix.

## This project's toolbox

Each row: what question the tool answers, and how to invoke it. Reach for one of these
before forming a theory about any failure.

| Tool | Answers | How to invoke |
| --- | --- | --- |
| `archogen <cmd>` exit code | what did the toolchain conclude, and about what? Diagnostic results (10–16, 70) are verdicts about the submitted system; process statuses (0, 2, 20) are verdicts about the invocation | `archogen <cmd> …; echo $?` — the table is in `archogen --help` |
| `archogen help <cmd>` | what does this command accept, and which leaf owns building it? | `archogen help build` |
| the doctrine enforcer | which repository invariant is broken, and where? | `make gate` (= `scripts/check_doctrines.sh`) |
| the seal checker | has the frozen evaluation set been modified, extended, or named outside its directory? | `bash scripts/check_frozen_evaluation.sh` |
| the profile drift test | has the published profile page diverged from the profile data the engine consults? | `cargo test -p eadl-model` |
| the emulator tool | what exactly does the pinned target run, and is its toolchain present? (exit 20 = required tool unavailable, never a skipped pass) | `scripts/target_emulator.sh --print <img>` / `--check` / `--dump-dtb <out>` |
| the code-path seam | is this staged file classified as a code change here? | `git diff --cached --name-only \| grep -Ef <(grep -vE '^\s*(#\|$)' .doctrine/code_paths.txt)` |
| `diagnose` | does this eADL file read, WHERE does it stop, what did it parse to, and what headers does it carry? (exit 0 clean · 1 diagnostics · 2 usage/IO) | `cargo run -q -p eadl-front --example diagnose -- <file.eadl>` |
| `cargo test --all` | does any contract test fail, and with which assertion diff? | `make test` |

## Building a new one

A diagnostic tool is a first-class deliverable: it lands in `scripts/` or as a test, it is
named here, and it has a **RED arm** — a demonstration that it fails when the thing it checks
is broken. A check that has only ever been seen green has not been shown to check anything.
