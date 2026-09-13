# Verifying the toolchain

The project's own checks are organised into the five tiers of `ROADMAP.md` §14.3, and each one
is a named command:

```console
$ cargo xtask verify --tier focused      # or: make focused
$ cargo xtask verify --tier integration  # or: make integration
$ cargo xtask verify --list              # or: make tiers
```

| Tier | When | What it covers |
| --- | --- | --- |
| `focused` | each edit loop, and every ordinary commit | format, lints, the whole contract suite |
| `integration` | before a push, and before closing a milestone | the above, plus the doctrine enforcer, the book build, and the pinned emulator |
| `extended` | scheduled, or when a change touches parsing, arithmetic or event ordering | fuzzing, mutation, Miri |
| `hardware` | a change to target support, and every release gate | board regressions and timing observations |
| `assurance` | every supported release | trust inventory, claim completeness, source and binary identity |

## A tier has three outcomes, not two

§14.3 ends with a sentence that is easy to agree with and hard to implement:

> A required tool skipped or unavailable is reported as such, **not a passed check**.

A runner that silently drops a step it cannot run converts an *absence of evidence* into an
*appearance of evidence* — and it does so exactly when the missing step is the one that mattered.
So the runner has a third verdict:

| Verdict | Exit | Means |
| --- | --- | --- |
| `passed` | `0` | every step ran and succeeded |
| `failed` | `1` | a step ran and failed |
| `incomplete` | `20` | nothing failed, and something could not be run |

`incomplete` is not a pass and not a failure. Nothing is broken; the evidence is simply not
there. And the two reasons a step cannot run are kept apart, because the response differs:

- **unavailable** — the step exists, and a *tool* is missing from this machine. Install it.
- **not built** — the step does not exist yet, and it names the **task-tree leaf** that owns
  building it, so a reader learns where the work is tracked rather than concluding the project
  forgot.

## What that looks like today

```console
$ cargo xtask verify --tier integration
tier: integration — before a push, and before closing a milestone
  ✅ fmt                  0.11s  every Rust source is in canonical format
  ✅ clippy               0.07s  no lint fires anywhere, including in tests and examples
  ✅ tests                2.27s  every contract test passes, F28 and the semantic corpus included
  ✅ doctrines            1.44s  every repository invariant holds on the working tree
  ✅ book                 0.06s  the mdBook builds — it is the director's window
  ⚠  emulator           UNAVAILABLE — `qemu-system-riscv64` is not on PATH
     §14.3 puts "selected emulator runs" in this tier. Without QEMU there is no independent
     execution of a target binary at all — see docs/targets/first-target.md
tier integration: incomplete — 5 passed, 0 failed, 1 unavailable, 0 not built
  ⚠  incomplete is NOT a pass.
$ echo $?
20
```

⭐ **Four of the five tiers are incomplete, and that is the runner's most useful output.** Before
it existed, the fuzz corpus, the mutation harness, the Miri wiring, the board and the whole
assurance story were not *reported as missing* — they were simply not mentioned, which reads
identically to being covered. Each gap now names its owner: `PROGRAM.9` for the extended tier's
three steps, `M5.1` for the board, `M3.6` / `M4.8` / `M4.7` for the assurance tier.

## Why `focused` runs the whole suite

§14.3 defines the focused tier as "format/type checks and **affected** contract tests", and
selecting affected tests needs change-impact machinery. Measured warm on this tree, the three
focused steps take **2.9 seconds** together. At that size the machinery would cost more than it
saves and would be one more thing to be wrong — so `focused` runs everything, and the measurement
is written into the runner's own source so the decision can be re-taken against a number rather
than re-argued from memory.
