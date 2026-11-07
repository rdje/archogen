# The application inputs: what a description may not state of a task's own code

- **Type:** `decision`
- **Date:** `2026-11-07`
- **Status:** `active`
- **Owner / source:** leaf `M4.12`, from `ROADMAP.md` §10.3's "separately supplied application inputs", the runtime plan's `RTP-H5` and the release alarm's `RA-H7`. Review: leaf `M4.12.1`.

## The fact / decision

**In plain words.** A description says what a system is made of and what it must do, and never how a task's own code does it, so it cannot say how long that code takes, how much stack it needs or which calls it makes. Those figures belong to whoever wrote the code. A runtime plan today lists each of them as owed and says `(complete no)`. This is the one file that answers: `archogen-application/1`, written by a person or a tool, one per system, read by `crates/archogen-plan/src/application.rs`.

**What it holds.** The system and target it is for, one declaration of the whole application, and one clause per task:

```text
(format "archogen-application/1")
(system heartbeat)
(target hosted-playground)
(leaves-interrupt-hardware-alone yes (clause "…the words below, whole…") (basis "why it holds"))
(task beat
  (entry-point "app::beat")
  (execution-bound (value 200) (unit us) (scope "one job") (binary unbuilt)
                   (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound"))
  (os-calls (evidence analytically-established) (locator "docs/argument.md") (basis "by reading") (call mask 2) (call unmask 2))
  (node-calls (evidence analytically-established) (locator "docs/argument.md") (basis "by reading") (call console.write 1))
  (masked-runs (evidence analytically-established)
    (run tail (value 5) (unit us) (scope "the tail's update") (binary unbuilt) (locator "docs/argument.md") (basis "four loads") (call unmask 1)))
  (stack-bound (value 512) (unit bytes) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (suspends no) (locks-scheduler no) (shares-outside-sections no)
  (masks-only-through-api yes)
  (figures-under-preemption yes)
  (deadlines-within-reach timer.short))
```

**Every clause is said, and nothing is defaulted.** A clause left out is refused naming it, one written twice is refused, and one this format does not know is refused: silence has no value, as in the rest of the engine. A task named twice is refused too.

**Each bound carries `ROADMAP.md` §7.3's record**, as a fact of its own: its value and unit, its scope, the binary it is of or `unbuilt`, and its evidence, one of the four categories `assumed`, `observed-maximum`, `externally-supplied` and `analytically-established`. All but an assumption says where it can be checked, by a `locator`, and each says why, by a `basis`. An observed maximum may give what it observed and a safety factor, which is held to its arithmetic, must pad and changes no category: a padded observation is still an observation.

**Times are a time and a stack is a size.** A time's unit is one the language reads, a stack's is `bytes` or `KiB`, and each value is a whole number no less than nothing. A binary is `unbuilt` or a sha256.

**What it declares of the whole application is quoted, not named.** `leaves-interrupt-hardware-alone` is the fact every composite is gated on, and its name no longer says what it holds: since `M2.30` it holds the runtime's own data and code as well as the interrupt hardware. A plan's owed line shows the name alone. So this file quotes the whole clause, which is `CLAUSE_PARTS`, in the composite-inputs record's own words (its §2), and `the_clause_a_file_quotes_is_the_record_s_own_words` reads each part back from there. What a person signs is the words. A file quoting others does not read, so a change to the clause makes every earlier declaration refuse until someone has read the new one.

**And it says what the declaration rests on**, which a yes or a no cannot carry: the clause counts an overrun of a task's own stack bound among what application code must not do. So the fact holds only as far as each task's stack bound holds, and `leaves_alone_rests_on` gives each with its category. A `yes` over an `assumed` stack bound is a `yes` resting on an assumption, and what reads it says so.

**What a task's clauses are for.** Each answers what `archogen-runtime-plan/1` owes `(by application)`:

| Fact a plan owes | Clause |
| --- | --- |
| `entry-point` | `entry-point` |
| `execution-bound`, `execution-category` | `execution-bound` and its evidence |
| `os-calls`, `os-calls-category` | `os-calls`: each call and the most one job makes, and their evidence |
| `masked-runs`, `masked-category` | `masked-runs`: each run, how long, and what it calls, and their evidence |
| `stack-bound`, `stack-category` | `stack-bound` and its evidence |
| `task-facts` | `suspends`, `locks-scheduler`, `shares-outside-sections` |
| `masks-only-through-api`, `figures-under-preemption` | those, each a yes or a no |
| `leaves-interrupt-hardware-alone`, of the system | the declaration above |

`node-calls` answers `RTP-H5`'s calls into nodes, and `deadlines-within-reach` `RA-H7`: a task writing an alarm on a time base that is the counter itself names that alarm, saying that each deadline it writes stands at or after its own release and within that alarm's `R`. Which alarms need it is known only once `/2` carries an alarm's time base, so this holds what a file says and `M4.1.7` asks it.

**Held against a plan.** `Application::unmet` says, in words, each thing a file leaves unmet of a plan: another system, a task the plan does not have, a task with nothing said of it, a call into a node neither its task line nor its own `bind` lines name, and a fact the plan owes that this format does not hold. That last is what keeps the two in step: a plan owing something new cannot pass unread. `every_fact_a_plan_owes_the_application_is_held` holds it over the three golden plans.

**What this does not do.** It reads the format and holds it against a plan. Carrying it into a run and into the plan's text is `archogen-runtime-plan/2`'s (`M4.1.7`), and judging its figures is the scheduling checker's (`M2.21`), so no figure here reaches a verdict yet, and a plan still says `(complete no)`. A call into a node past those its lines name goes through the code of one they name (`RTP-H5`), which is the realization's to show.

## Why

**One file, apart from the description**, since `ROADMAP.md` §10.3 keeps them apart: a description holds no implementation (`decision_eadl-engine-boundary.md`), and figures of a binary change with each build, where the description does not. **No default** for any clause, since a bound nobody stated reads the same as one somebody checked if the format supplies it. **The category beside each figure**, since §7.3 asks it and the composition reports each input by its category; a single category for a task would lose that its stack is assumed and its execution bound established.

**Quoting the clause and not a version of it.** A number or a digest would say which words were meant without showing them, and the point is that a person reads them. It costs a long line, which a tool can write.

**Rejected: a clause per fact in the description.** That is the first thing one would do, and it puts a binary's figures in a text the engine's digest is computed from, so rebuilding the code would move the plan.

## How to apply

- A plan's owed fact `(by application)` that this format does not hold fails `what_is_unmet_of_a_plan_is_said`'s path, so a change to `decision_runtime-plan.md` §2's list moves this record and the reader with it.
- Any change to what `/1` reads of this file's text moves `FORMAT` (`docs/book/src/versions.md`, `application-inputs-format`).
- `M4.1.7` carries it into `/2` and asks `RA-H7` of each task the time base requires. `M2.21` judges it. Related: `[[decision_runtime-plan]]`, `[[decision_release-alarm]]`, `[[decision_runtime-composite-inputs]]`.

## Review

Not yet reviewed: leaf `M4.12.1`, whose rounds will be in `docs/reviews/decision_application-inputs-reviews.md`.
