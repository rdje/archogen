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
| the tier runner | which §14.3 tier passes here, and what is missing from the ones that do not? (exit 0 passed · 1 failed · **20 incomplete — never a pass**) | `make focused` · `make integration` · `make tiers` (= `cargo xtask verify …`) |
| the doctrine enforcer | which repository invariant is broken, and where? | `make gate` (= `scripts/check_doctrines.sh`) |
| the seal checker | has the frozen evaluation set been modified, extended, or named outside its directory? | `bash scripts/check_frozen_evaluation.sh` |
| the S0 retirement check | is every hard-coded S0 assumption still marked, listed and owned — and has the prototype acquired a consumer it should not have? (`--self-test` runs three RED arms) | `bash scripts/check_s0_retirement.sh` |
| the book-anchor check | does every book chapter point at code that exists, and does any chapter point at nothing? (`--self-test` runs three RED arms) | `bash scripts/check_book_anchors.sh` |
| the profile drift test | has the published profile page diverged from the profile data the engine consults? | `cargo test -p eadl-model` |
| the reach drift gate | does any **live** surface still publish a schema-reach figure the corpus no longer measures? A figure in prose is compared against the measurement; a figure that is neither the measurement nor marked past tense is a stale claim. 5 RED arms: stale current figure · historical figure with its past tense removed · a figure put back into the module header · the measuring test unnamed · a corpus that grew (`14 of 14` vs measured `13 of 13`). ⚠️ Its past-tense escape is `M1.25`'s: a marker scan is satisfiable by unrelated prose on the same line | `cargo test -p eadl-model --test kinds the_live_surfaces_publish_the_measured_reach` |
| the corpus-figure gate | does any **live** surface still publish a boundary-corpus figure the corpus does not measure — size, accept/reject split, or ambiguous count — and does the suite's own module header carry one at all? A header states the rule and names the measuring test; the book chapter and the corpus index may carry the figure *because* a gate reads them and compares. History is an explicit `HISTORICAL_FIGURES` list, not a tense scan — the tense scan was measured passing on its own defect. 7 RED arms, each fed the prose that was actually wrong and each pinning its violation **count**: stale book figure · stale index figure on a line that also says `before` · a figure put back into the header, correct or not · the measuring test unnamed · an index summary that disagrees · a chapter that publishes no size · an index with no summary line | `cargo test -p eadl-front --test corpus the_live_surfaces_publish_the_measured_corpus_size` |
| the emulator tool | what exactly does the pinned target run, and is its toolchain present? (exit 20 = required tool unavailable, never a skipped pass) | `scripts/target_emulator.sh --print <img>` / `--check` / `--dump-dtb <out>` |
| the code-path seam | is this staged file classified as a code change here? | `git diff --cached --name-only \| grep -Ef <(grep -vE '^\s*(#\|$)' .doctrine/code_paths.txt)` |
| the grammar conformance check | does the reader still implement the normative grammar — same language, same token boundaries — and does that grammar agree with the language reference on **every literal form the reference enumerates**, including the ones no shipped description contains? RED arms restore the two divergences the literal-space leg was written for, byte-for-byte from `git show` | `cargo test -p eadl-front --test conformance` |
| the reference value check | does the frontend still do what the language reference *says* — the exact value of every literal, the decoded value of every string, canonical text carrying no control character **and reading back to the same form**, the headers a comment block yields including the ones that yield none — and does the reference still say what the code does: a value for every literal the corpus ships, a row for every diagnostic code its four declared sources can emit, a repair direction in every one of those rows, and a citation that resolves? Read out of `docs/semantics/reference.md`, not written beside it; the code census runs in **both** directions, so a green run is evidence about the scanner and not only about the table. RED arms per leg, each pinning its violation count, two of which prove a header discriminator load-bearing by editing a **row** rather than the code | `cargo test -p eadl-front --test reference` |
| a re-measurement instrument | has a reported vendor defect been re-measured, and is it **gone** at the pin? (exit `0` gone · `1` still present · `2` could not run — the **opposite** polarity of the frozen `repro.sh` beside it, which can only report *change*) | `bash docs/feedback/linkedspec/issues/LS-005-guide-ordering/remeasure.sh vendor/linkedspec` — `--self-test` runs its RED arms |
| the LinkedSpec evaluation launcher | is the vendored recognizer prepared and built at the pin, which revision is the build named after, and where are its binaries? (exit `0` ok · `2` a prerequisite is missing · otherwise the underlying command's own status, never rewritten) | `scripts/linkedspec_eval.sh env` · `prepare` · `build [--network]` · `bins` · `reference` · `run <bin> …` |
| `diagnose` | does this eADL file read, WHERE does it stop, what did it parse to, and what headers does it carry? (exit 0 clean · 1 diagnostics · 2 usage/IO) | `cargo run -q -p eadl-front --example diagnose -- <file.eadl>` |
| `cargo test --all` | does any contract test fail, and with which assertion diff? | `make test` |

## Building a new one

A diagnostic tool is a first-class deliverable: it lands in `scripts/` or as a test, it is
named here, and it has a **RED arm** — a demonstration that it fails when the thing it checks
is broken. A check that has only ever been seen green has not been shown to check anything.
