# The hosted build: from a runtime plan to a generated system that runs on the playground

- **Type:** `decision`
- **Date:** `2026-11-04`
- **Status:** `active` — written by leaf `M4.2.1`; built ahead of its review as its executable form
  (`decision_executable-design-reviews.md`); under review, `## Review`
- **External sources:** none: this record decides how the engine's own parts fit together
- **Owner / source:** leaves `M4.2` and `M4.3` (`docs/tasks/M4.md`), carrying `RTP-H1`, `RTP-H2`, `RTP-H3` and `RTP-H9` of
  [[decision_runtime-plan]]; `ROADMAP.md` §6.3 and §10.1 steps 6 to 8.

## The fact / decision

`archogen build` goes from a description to a package that compiles and runs on the hosted playground, through the
checked runtime plan, with no hand edit and nothing fetched. It replaced the S0 path, deleted once its retained fixtures ran
through this one.

### 1. The generated package

The build writes one [Cargo](../book/src/ledger.md#rust-toolchain) workspace, whose root is the system's package:

```text
Cargo.toml                 the system's package, its dependencies by relative path, and `[workspace]`
src/main.rs                the tables specialized from the plan, and a call of the simulator
rt-core/                   crates/rt-core's sources, byte for byte, under a generated manifest
sim-playground/            crates/sim-playground's sources, byte for byte, under a generated manifest
plan.txt                   the runtime plan's canonical text
provenance.json            what the plan was lowered from, and what each table entry came from
description/                the description and every module it imports, under the names they import each other by
assignment.eadl            the assignment the search wrote, which the checker judged
plan-check.json            the checker's result record for it
```

No byte of the package depends on the directory its description lies in or on where it goes: a provenance entry names a source
by its place relative to the description, whose own file name is part of what is recorded. A build records what it was made from
so that a later command reads what it read: `archogen explain` of a build, by §10.2, holds its files to their digests, explains
that description, and holds the search's plan equal to the one the build holds and to what its checker's record accepted, or says
`stale-input`. `provenance.json` lists every other file with its length and its digest, and each table entry with its generated line and its
line in the plan, and for a task its form in the description, and holds its own digest over its text. A digest catches an edit
and a stale file, never a forger: whoever can write the directory can rewrite the list. A provenance that no longer reads, or names
another format, is no build this engine reads, so it is `usage` and not `stale-input`.

Behavior is copied and never generated: what `src/main.rs` adds is data. So the code that runs is code the engine compiles
and tests, as `ROADMAP.md` §1.1 admits ("copied or linked components") and as S0 did, and a test holds each copy byte for
byte to its source. A path dependency on the checkout would tie the output to the place it was built, and a registry
dependency would need the network, which §10.3 keeps out of a build.

**The emitter holds its own copies**, under `crates/archogen-emit/copies/`. A generator that read a file of `crates/rt-core` at
compile time would be, to the trust inventory, a program of two roles (`trust-shared-program`), since `rt-core` is the
implementation's role package, and it reads the compiler's own record of what a build read. So the sources are text in the
emitter's own package, each read by an admitted `include_str!`, never compiled by the generator and never run by it. What this
costs is a second tree a change of the runtime must refresh, and the test says how. ⚠️ What it does not change is the fact the
inventory's rule exists for: the generator holds the implementation's text, as data, and the admissions name that. The admissions
hash `COPIES`'s extent and not the copied bytes, so that test is the only thing binding them. And since the generator cannot compile
against the runtime, a table it writes is first compiled when the package is: tests build and run each shape of table.

### 2. Time is exact

A plan's durations are exact rationals, so the tables hold integer **ticks** of `1/L` s, `L` the least common multiple of the
denominators of what the run repeats: each timer-released task's period and each arriving source's separation. A deadline or a
jitter, which no table holds, has no say, and nor does the minimum separation of a task a source releases, which repeats only as
that source does. Nothing rounds. What would not fit `u64` ticks, and a hyperperiod of it that would not, is refused
`unsupported-profile`, naming what, rather than approximated.

### 3. The simulator

`crates/sim-playground` is the deterministic event harness of `ROADMAP.md` §6.3, over the shared `rt-core`. It does no I/O:
it hands each event to an observer, and the generated `main` prints them.

**The order of one instant** is its whole allowed-ordering policy, so that one input has one trace:
1. a job with nothing left that takes time finishes, so its completion is recorded before a release at that instant, as `ROADMAP.md`
   §13.4 requires ("At a coincident completion/release instant, record completion before processing the new release"). It stops at an
   unmask that lets a pending interrupt in, which is taken before the job's next instruction, its completion among them;
2. an interrupt becomes pending at its arrival, whether or not anything can take it;
3. while the hardware is unmasked, every pending interrupt is taken, one at a time and none nested: the routes in the plan's
   rank order, then the release timer, as `RTP-H9` writes it. Taking one runs entry, its releases and exit, and each
   release is decided by `rt-core`;
4. the scheduling decision follows, and its switch;
5. the running job's body advances, until the next arrival or its own next step.

A mask step masks the hardware before raising the depth, an unmask lowers the depth before unmasking it, so no service ever
runs with the depth raised (`releases-never-latched`). Initialisation runs first, in the plan's steps, interrupts masked,
and interrupts are enabled at time zero.

**A body is data**: a list of steps, *run for N ticks*, *mask*, *unmask*. `/1` writes every job's as empty, so a job starts
and completes at one instant and the release trace alone observes it, until `M4.12`'s inputs give it code. The harness runs
longer bodies all the same, so preemption, masking and overrun are exercised now, by its own tests.

**A run names its scenario.** The horizon is one hyperperiod of what the run repeats (§2). Each
source whose separation is stated arrives at exactly that separation from time zero, the densest the plan allows. One whose
separation is zero, `unknown` or `absent`, or that the plan does not enable, does not arrive, and the trace's header says so: a run
never claims what it did not exercise. **The trace** is `archogen-playground-trace/3`, one line an event, its time exact, its events on
the fault paths the ones [`rt-static-up-v1-faults-observation.md`](../profiles/rt-static-up-v1-faults-observation.md) §2 fixes: an
overrun one `fault` and no `release`, a task already faulted nothing, a timer-released release its nominal instant, and idle entry
stamped only on a transition into it. Each `mask` and `unmask` that runs is shown, one that raises an assertion failure among them, and
a halt's `fault` holds what `rt-core` keeps, its first report the only one. What its §4 still needs is `M4.9`'s.

**A run is bounded.** A hyperperiod that fits `u64` can still be a run nobody could read: `1 ms` against `2305843009213693951 ms`
never ends. So a plan whose one hyperperiod holds more than `2^20` timer releases and source arrivals is refused
`unsupported-profile`, naming the count, rather than cut short; a run of a part of a hyperperiod, named as such, is a scenario
`M4.3` may add.

### 4. The emitter

`crates/archogen-emit` does no I/O: it turns its inputs into a list of files. It takes the program, the assignment, the
result record and the plan's text, and **accepts the plan only by lowering it again from those inputs and comparing bytes**
(`RTP-H3`). What it then refuses:
- `(released-by (unknown CAUSE))`, first of all, since what a run would cost means nothing until it can run, naming the task:
  `missing-fact` or `unsupported-profile`, as `RTP-H1` gives each cause;
- what the run repeats, or a hyperperiod of it, past `u64` ticks (§2), naming that, and a run past its bound (§3), naming its count:
  `unsupported-profile`.

A plan whose acceptance was taken as text is built marked experimental, in `provenance.json` and in the trace's header. No
front reaches that today, since the only build is the command line's, which runs the checker (§5); the rule stands for the
next front that takes a result it did not run.

### 5. The command

`archogen build DESCRIPTION --out DIR [--system NAME] [--profile P]` checks the description, searches, runs the installed
configuration checker on the plan it wrote, lowers on its acceptance, emits, and writes the package. `--locked` stays refused
until `M4.7`. It says what it did not do: that no job holds application code, and that the plan is `(complete no)`.

**The command line alone.** `ROADMAP.md` §10.4 rules that system generation is outside the programmatic interface, so the
engine API, the wasm module and the [MCP](../book/src/ledger.md#mcp-specification) server offer no build. What the command
needs of the engine API is only what the search already holds: the elaborated program and the kinds, beside the plan
(`Resolution::elaborated`).

**Where the package lands.** The routes `--out` can take are the rows of the table in
[Building a system](../book/src/building.md#the-rules), and those of `archogen resolve`'s `--out` of
[`cli.md`](../book/src/cli.md)'s: a test runs each row, and a row with no fixture, or a fixture with no row, fails. Those tables are the
rule, and a route they do not name is promised nothing. Round 6 is why (`## Review`): a rule stated in several sentences drifted
between them, round after round, and a table a test runs cannot. The designs the rows come from:

- **Two renames**, the package written whole in `DIR.building` first, so `DIR` holds at every moment what stood there, nothing, or the
  whole package.
- **Removal by the list.** A build removes of an earlier build each file `provenance.json` lists, only if still the regular file it wrote,
  reached through real directories; `target`, `Cargo.lock` and `.DS_Store` at its top, which building and looking at a package leave, move
  to the new package, or stay aside if the stage came to hold that name.
- **A private stage**, mode `700`, with its files created and never written over or through, and the directory itself held by what it is
  until it lands, and on the way out: what has come to stand at its name is not this run's to remove. Whatever `DIR` lets others do, its mode closes it to them; a name a package writes that something else fills is refused,
  and the stage kept. The package takes the mode and the group of what it replaces last, read again just before the first rename; where
  nothing stood it takes what a new directory takes there, which alone needs a directory made at that mode, empty, for the instant before
  it is closed. An access control list a parent passes down is no mode, and applies to the stage.
- **An output replaces only what its own kind wrote.** A build replaces exactly a build or an empty directory, a plan only a plan or an empty file, and a record
  only a record or an empty file, the checker's `--result` among them, and what replaces a file keeps its mode. Asking "is this what it was
  read from" found a new loophole each round, `/dev/fd/N` the last; asking "is this ours" has no such door. A plan is known by the engine's
  own reader of the format, which takes only text in the canonical form the search writes, as a build is known by what it lists: so a comment
  in any place makes it none, as a file edited makes a build none. ⚠️ A plan is less held than a build, which lists a digest of every file: an edit
  that leaves the text canonical, one binding for another, is still a plan and the next run replaces it, since canonical text carries no digest
  of itself. A plan is the search's output, and what is chosen belongs in the description.
- **A plan judged before it lands.** `archogen resolve` writes all it writes in `PLAN.part`, a private directory where the checker judges it,
  and moves it out only on acceptance, or under `--unchecked`: the record first, taking its name with the earlier one held aside, as the steps
  that find a mode or a flag in the way, then the plan. Each step undoes only what the ones before it moved. So what stands at either name
  stands until there is a plan that may replace it, and a plan does not land to be refused after. Rounds 7, 8 and 9 are why.
- **What the checker reads again is what the front read.** It reads the description and every module for itself, so each must be a regular
  file with the same bytes, or the command refuses before it runs: a pipe would read empty or never end, and `/dev/stdin` reads empty the
  second time. The engine's own kind modules come from no file, and are passed over.
- **One way to read a place**, `crates/archogen-cli/src/place.rs`, for both commands. Only a name found absent is absent, and whatever stops
  a place being read or made is `usage`, nothing yet being written. Room in a path is judged as the checker is handed the path, its
  directory canonical, and after the parents stand, since nothing else names it so.

⚠️ **What it does not hold against.** `std` has no rename that refuses to replace and no directory-relative open, so each path is held
again just before it is used, and in that instant what stands there is what is used. It reads no access control list, owner or flag, so a
package keeps none of them. No sync call is made, so nothing here claims anything across a power loss.

**The two standard streams.** `Watched` (`crates/archogen-cli/src/lib.rs`) remembers any write to standard output that failed, and the
executable reads it, so every command's lost report has one route: it says so, and exits `tool-failure`, with what the command did left
standing. A sibling's output goes on stream by stream, one failing not stopping the other, and neither decides anything, the program having
judged. A standard error nobody reads changes nothing, there being nowhere left to say it. ⚠️ The Rust runtime gives a stream closed before the
run `/dev/null`, and takes a stream open only for reading as one that wrote, so for those no failure exists to see.

### 6. What S0 keeps, and when it goes

S0's frozen expected observations are functional fixtures of the system's behavior, and §12 M4 asks that they run through the
supported path. So a test holds each one's releases equal to the new trace's. The one fixture S0 refused, a task with a
minimum separation and no source, stays refused, as `missing-fact` now that the plan can say why, where S0 could only call the
capability unsupported. Then the S0 crate was deleted (`M4.2.4`), and the retirement record says what became of each assumption.

## Why

- **Copies over dependencies.** The output is a self-contained tree that builds anywhere, and `rt-core` is the same bytes the
  engine tests. A generated scheduler would be a second scheduler nobody tested.
- **Integer ticks over rationals in the run.** The generated system and the harness then need no rational arithmetic, and a
  target's timer, which counts, reads the same tables. Exactness is kept where it matters, in the plan, and the tick is
  derived from it.
- **One order for one instant.** §6.3 asks which orderings a model allows. This harness allows one, so a trace can be compared
  byte for byte, and a later model that allows more states its own policy.
- **A body as data, before any application exists.** A simulator exercised only on empty jobs would not be exercised at all.

## How to apply

- `M4.3.1` builds the simulator; `M4.2.2` the emitter; `M4.2.3.1` the command, S0's fixtures through it and the book;
  `M4.2.3.2` `archogen explain` on a build; `M4.2.3.3` the review; `M4.2.4` S0's retirement.
- A change that moves a generated file for the same inputs moves `archogen-playground-trace` or its emitter's identity, as
  `docs/book/src/versions.md` registers them.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 15 | 10 (H1-1 to H1-10), beside 4 gaps and 1 of wording | `2026-11-04`, not acceptable — answered: a build deletes nothing it did not write, by a list of what it wrote and a stage only this run makes; two renames; every disagreement keeps its record; `explain` holds a build to its digests and a request that misses is `usage`; a zero separation named; a run bounded |
| 2 | 12 | 4 (H2-1 to H2-4), beside 7 gaps and 1 of wording | `2026-11-04`, not acceptable — answered: removal by the list, what cargo leaves carried, an empty directory no build's; the unknown cause first; provenance's own digest; `explain` asks what `build` asks; the tick and the hyperperiod of what the run repeats |
| 3 | 9 | 1 (R3-1), beside 5 gaps and 3 of wording | `2026-11-04`, not acceptable — answered: removal holds each path as the judgement does; a stage private from the first; what comes meanwhile refused; a linked description's imports found where the front finds them; what `std` cannot hold, stated |
| 4 | 5 | 2 (R4-1, R4-2), beside 3 of wording | `2026-11-05`, not acceptable — answered: a directory closed to its owner's writes takes a package, the exact mode last; only `NotFound` is absence, and a failed stage takes its parents with it; each refusal its own repair; what the guarantee excludes, said where it is stated |
| 5 | 11 | 4 (R5-1, R5-2, R5-3, R5-5), beside 4 gaps and 3 of wording | `2026-11-05`, not acceptable — answered: a stream that remembers any failed write, every route one; what a stream closed before the run cannot show, stated; whatever stops the place opening is `usage`, by its cause; a carried name no bar to what came; the stage's mode from the call that makes it, and held again at landing; a directory that cannot be read refused |
| 6 | 17 | 5 (R6-1 to R6-5), beside 5 gaps and 7 of wording | `2026-11-05`, not acceptable — answered by method as well as finding by finding: each route of `--out` a row a test runs, so no sentence states one; one way to read a place for both commands; a stage private from the first, whose files are created and never written over; a sibling's streams relayed apart, and a standard error nobody reads changing nothing |
| 7 | 22 | 8 (R7-1 to R7-8), beside 9 routes no row named, 2 gaps and 3 of wording | `2026-11-05`, not acceptable — answered: `resolve` judges at `PLAN.part` and lands only on acceptance, and refuses to replace what its description was read from; a description the checker cannot read again refused; the stage held by what it is; the checker's own `.part` created, never written through; each named route a row |
| 8 | 19 | 9 (R8-1 to R8-9), beside 2 routes no row named, 3 gaps and 5 of wording | `2026-11-05`, not acceptable — answered by one principle: an output replaces only what its own kind wrote, for a plan, a record and the checker's `--result`; everything a run writes kept in one private stage until it lands, the record moved first so that a plan cannot land to be refused after |
| 9 | 16 | 6 (R9-1 to R9-6), beside 5 gaps, 3 routes no row named and 2 of wording | `2026-11-05`, not acceptable — answered: a plan known by how it reads, not by how it begins; a landing whose each step undoes only what it moved, the record taking its name before the plan moves; a record the command cannot read refused, not taken as none; every source held to reading the same again |
| 10 | 13 | 2 (R10-1, R10-2), beside 7 gaps, 2 routes no row named and 2 of wording | `2026-11-05`, not acceptable — answered: a plan known by the engine's own reader of the format, which takes only what the search writes; a stage held by what it is on the way out too; a question of a module's room, of a closed record and of a name with no room each answered as it is |
| 11 | 9 | 4 (R11-1 to R11-4), beside 3 gaps, 1 route no row named and 1 of wording | `2026-11-05`, not acceptable — answered: what a canonical plan cannot show, stated and held by a row; no bound claimed that an engine's own output can pass, format `/5` giving each listed file's length so that another length is changed unread and the same length is digested as it is read; the checker's arguments read as the front's are |
| 12 | 7 | 1 (R12-1), beside 3 gaps, 1 route no row named and 2 of wording | `2026-11-05`, not acceptable — answered: what cannot be read is not called a change, so a closed file of an intact build is `usage` with a repair that works, and a changed one is told a new directory; each repair followed to its end; a provenance past its bound told by its length, unread |

The rounds, each finding and its answer, are in
[`docs/reviews/decision_hosted-build-reviews.md`](../reviews/decision_hosted-build-reviews.md).
