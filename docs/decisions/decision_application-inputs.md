# The application inputs: what a description may not state of a task's own code

- **Type:** `decision`
- **Date:** `2026-11-07`
- **Status:** `active`
- **Owner / source:** leaf `M4.12`, from `ROADMAP.md` §10.3's "separately supplied application inputs", the runtime plan's `RTP-H5` and the release alarm's `RA-H7`. Review: leaf `M4.12.1`.

## The fact / decision

**In plain words.** A description says what a system is made of and what it must do, and never how a task's own code does it, so it cannot say how long that code takes, how much stack it needs or which calls it makes. Those figures belong to whoever wrote the code. A runtime plan today lists each of them as owed and says `(complete no)`. This is the one file that answers: `archogen-application/1`, written by a person or a tool, one per system, read by `crates/archogen-plan/src/application.rs`.

**What it holds.** The system and target it is for, one declaration of the whole application, the application's initialisation, and one clause per task:

```text
(format "archogen-application/1")
(system heartbeat)
(target hosted-playground)
(leaves-interrupt-hardware-alone yes (clause "…the words below, whole…") (basis "it touches no device"))
(initialisation
  (stack-bound (value 256) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured")))
(task beat
  (entry-point "app::beat")
  (execution-bound (value 200) (unit us) (scope "one job") (binary unbuilt)
                   (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound"))
  (os-calls (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (call mask 2) (call unmask 2))
  (node-calls (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (call console.write 1))
  (masked-runs (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (run tail (ending unmask) (value 5) (unit us) (scope "the tail's update") (binary unbuilt)
              (evidence analytically-established) (locator "docs/argument.md") (basis "four loads")))
  (stack-bound (value 512) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (suspends no) (locks-scheduler no) (shares-outside-sections no)
  (masks-only-through-api yes)
  (figures-under-preemption yes)
  (deadlines-within-reach))
```

**Every clause is said, and nothing is defaulted.** A clause left out is refused naming it, one written twice is refused, and one this format does not know is refused, a language version among them: silence has no value, as in the rest of the engine. A list with nothing in it says so, `(deadlines-within-reach)` and an empty `(node-calls …)` among them. A task named twice is refused too. A file holds at most `MOST_BYTES`, 16 MiB, since a person writes it, and what is past that is refused by its length. What does not read as forms is refused with what the reader said of it.

**Each bound and each list of counts carries `ROADMAP.md` §7.3's record**, as a fact of its own: its scope, the binary it is of or `unbuilt`, and its evidence, one of the four categories `assumed`, `observed-maximum`, `externally-supplied` and `analytically-established`. A bound also has its value and unit, and a count's unit is calls. All but an assumption says where it can be checked, by a `locator`, and each says why, by a `basis`. So an empty list still says what backs it, as the composite-inputs record's §6 has it. A bound's observed maximum may give what it observed and a safety factor, which is held to its arithmetic, must pad, is kept for what reads it, and changes no category: a padded observation is still an observation. A list says `observed-maximum` alone, each of its counts being what was seen.

**A masked run is one way of a run's ending.** Each `run` says `(ending unmask)` or `(ending completion)`, which the composition charges as `api.unmask` or `completion` (the composite-inputs record's §3), and a run that ends both ways is written once for each. Its bound has its own record, so it keeps its own category and any padding of its own, and the list's record is that of the counts inside. Its `call`s are the primitives it calls other than the `mask` and the `unmask` that delimit it, and its `node-call`s the nodes it calls.

**A node is written as the plan writes it,** its name or an instance's `(adapter …)` text, in `node-calls`, a run's `node-call` and `deadlines-within-reach`.

**What each text can hold.** A text says something, on one line, with no quote or backslash, since `/2` carries it on one line of a plan. `entry-point` is a path a generated job can call, names joined by `::`, such as `app::beat`, since `M4.13` writes that call.

**Times are a time and a stack is a size.** A time's unit is `s`, `ms`, `us` or `ns`, and a stack's `byte`, `KiB` or `MiB`, the language's, and each value is a whole number no less than nothing. A binary is `unbuilt` or a sha256.

**What it declares of the whole application is quoted, not named.** `leaves-interrupt-hardware-alone` is the fact every composite is gated on, and its name no longer says what it holds: since `M2.30` it holds the runtime's own data and code as well as the interrupt hardware. A plan's owed line shows the name alone. So this file quotes the whole clause, in the composite-inputs record's own words (its §2), which are these three parts, joined by ` … ` where that record goes on with what is no part of them:

> after the first enabling of interrupts, application code runs only as a declared task's job or as a service's application code; and no application code, a task's, a service's or application initialisation's, writes the timer's counter or compare, the interrupt controller's configuration, or the hart's interrupt or trap state other than through the runtime API's masking primitives (a job's only: the fault contract, Terms) or reads or writes the controller's claim and complete registers, a read of which claims, or a declared source's device registers; no service calls a runtime primitive; and no application code releases a task or calls a runtime function other than the runtime API record's primitives and its completion path. A runtime function is any function of a catalog record's implementation, a driver's included, called after the first enabling of interrupts; before it, application initialisation reaches that hardware only through catalog functions, which `runtime-discipline.<id>` covers.

> Application code calls no entry of the fault path — in initialisation, before the first enabling of interrupts, as in a job or a service's application code after it; a panic reaching the image's handler is no such call

> The runtime's own data is what this section's facts read in memory or in a register a port reserves: what the scheduler holds, its mask depth among it, what the timer service holds of the next release, the plan's tables, each saved context, and what a service, a record's or generated, holds of its source. Application code writes none of it, nor any other data or code of a catalog record's implementation or of the code the build generates, but through the calls `leaves-interrupt-hardware-alone` lets it make, or into what a call between it and that code hands it to fill, and it hands no call a place in any of it to write, but what that code handed it; in initialisation as in a job or a service's application code, and by no other means, a stray pointer or an overrun of its own stack bound among them (its own stack, within its bound, is its own, and a write a guard stops writes nothing, and is the fault contract's). For this clause a runtime function is also a function of the code the build generates, but for a node's code its task line's `uses` lists or its own `bind` lines name (the runtime plan's §2.2, `RTP-H5`, `RTP-H13`), which §7 says what charges; and a job's code calls the completion path only as its last act, the path not returning to it.

`the_clause_a_file_quotes_is_the_record_s_own_words` reads each part back from there, with what stands on each side of it, so a sentence added beside one moves it too. "This section" in the third part is that record's §2. What a person signs is the words. A file quoting others does not read, so a change to the clause makes every earlier declaration refuse until someone has read the new one.

**And it says what the declaration rests on**, which a yes or a no cannot carry: the clause counts an overrun of code's own stack bound among what application code must not do, "in initialisation as in a job or a service's application code". So the fact holds only as far as each of those bounds does, and `leaves_alone_rests_on` gives each with its category: the application's initialisation first, then each task. A `yes` over an `assumed` stack bound is a `yes` resting on an assumption, and the scheduling checker says so when it judges the set (`M2.21`).

**What a task's clauses are for.** Each answers what `archogen-runtime-plan/1` owes `(by application)`:

| Fact a plan owes | Clause |
| --- | --- |
| `entry-point` | `entry-point` |
| `execution-bound`, `execution-category` | `execution-bound` and its evidence |
| `os-calls`, `os-calls-category` | `os-calls`: each call and the most one job makes, and their evidence |
| `masked-runs`, `masked-category` | `masked-runs`: each run with each ending, how long, and what it calls, each with its evidence, and the list's |
| `stack-bound`, `stack-category` | `stack-bound` and its evidence |
| `task-facts` | `suspends`, `locks-scheduler`, `shares-outside-sections` |
| `masks-only-through-api`, `figures-under-preemption` | those, each a yes or a no |
| `leaves-interrupt-hardware-alone`, of the system | the declaration above |

`node-calls` answers `RTP-H5`'s calls into nodes, and `deadlines-within-reach` `RA-H7`: a task writing an alarm on a time base that is the counter itself names that alarm, saying that each deadline it writes stands at or after its own release and within that alarm's `R`. Which alarms need it is known only once `/2` carries an alarm's time base, so this holds what a file says, and `/2`'s lowering asks it (`decision_runtime-plan-format-2.md`). `initialisation`'s stack bound answers no line of `/1`, whose plan does not lay out memory: `/2`'s boot stack holds it.

**Held against a plan.** `Application::unmet` says, in words, each thing a file leaves unmet of a plan: another system, another target, which against `/1` is any but `hosted-playground`, a task the plan does not have, a task with nothing said of it, a call into a node neither its task line nor its own `bind` lines name, in `node-calls` or inside a masked run, and a fact the plan owes `(by application)` that this format does not hold of that subject. That last is what keeps the two in step: a plan owing something new cannot pass unread. `every_fact_a_plan_owes_the_application_is_held` holds it over the three golden plans, and `what_is_unmet_of_a_plan_is_said` holds each of these. A primitive's name in `os-calls` or a run's `call` is the catalog's, and the composition refuses one no record costs.

**What this does not do.** It reads the format and holds it against a plan. Carrying it into a run and into the plan's text is `archogen-runtime-plan/2`'s (`M4.1.7`), and judging its figures is the scheduling checker's (`M2.21`), so no figure here reaches a verdict yet, and a plan still says `(complete no)`. A call into a node past those its lines name goes through the code of one they name (`RTP-H5`), which is the realization's to show.

## Why

**One file, apart from the description**, since `ROADMAP.md` §10.3 keeps them apart: a description holds no implementation (`decision_eadl-engine-boundary.md`), and figures of a binary change with each build, where the description does not. **No default** for any clause, since a bound nobody stated reads the same as one somebody checked if the format supplies it. **The category beside each figure**, since §7.3 asks it and the composition reports each input by its category; a single category for a task would lose that its stack is assumed and its execution bound established.

**Quoting the clause and not a version of it.** A number or a digest would say which words were meant without showing them, and the point is that a person reads them. It costs a long line, which a tool can write.

**Rejected: a clause per fact in the description.** That is the first thing one would do, and it puts a binary's figures in a text the engine's digest is computed from, so rebuilding the code would move the plan.

## How to apply

- A plan's owed fact `(by application)` that this format does not hold fails `every_fact_a_plan_owes_the_application_is_held`, so a change to `decision_runtime-plan.md` §2's list moves this record and the reader with it.
- Any change to what `/1` reads of this file's text moves `FORMAT` (`docs/book/src/versions.md`, `application-inputs-format`).
- `M4.1.7` carries it into `/2`, asks `RA-H7` of each task the time base requires, and holds a file's `target` against the plan's. `M2.21` judges it. Related: `[[decision_runtime-plan]]`, `[[decision_release-alarm]]`, `[[decision_runtime-composite-inputs]]`.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 14 | 13 (AI1-1 to AI1-13), beside 1 of wording | `2026-11-07`, not acceptable — answered: the format now holds what the composition reads of a masked run, each list's §7.3 record and each run's own, a node as the plan writes it, and the application's initialisation; `unmet` holds what the record said it held; a person is sent to words that are there. No file of this format existed outside its tests, so its text moved without a new version. Its rounds are in `docs/reviews/decision_application-inputs-reviews.md`. |
