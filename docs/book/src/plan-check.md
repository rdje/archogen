# Checking a plan

## The idea, in plain words

Before the engine builds anything, it decides which part of the platform serves each need of the application: the
service that keeps time reads *this* counter, the console writes through *that* serial port. That decision is the
**plan**, and the part of it that says who serves whom is the **assignment**. The program that searches for an
assignment is large and clever, and clever programs make mistakes. So a second, much smaller program checks every
plan before it is used: the **configuration checker**. It was written apart from the search, from the rules alone,
so that a mistake in one is unlikely to be repeated by the other.

The checker reads two things: the description, and the **assignment record** the search wrote. It accepts the plan
only when every need is served, every pairing is one the description allows, and every part chosen really offers what
it is chosen for. Otherwise it refuses the plan and points at the need in the description and the line in the record
that disagree.

An everyday comparison: a seating plan for a dinner. The host draws it up; someone else checks it against the guest
list and the dietary notes. The checker does not care how the host chose. It asks whether every guest has a seat,
whether every seat is given to a real guest, and whether the vegetarian is sitting in front of the vegetarian meal.
And if the guest list changed after the plan was drawn, the checker does not try to patch the plan: it asks for a new
one.

> **In one minute, for engineers.** The record is `archogen-assignment/1`, an eADL document with a canonical form:
> the profile, the system, the sha256 digest of the elaborated program, the placement and one `bind` clause per
> binding — requester, fact, provider. The checks run in order — (1) the record reads and its digest is the program's, else
> `stale-input`; (2) the description is re-checked, by the language's passes and the provider-level rules; (3) each
> requirement that binds is bound once, each binding is a requirement's, of a requester the system serves, to a
> declared block or platform; (4) each binding is satisfied by the model's judgement — the reference model of
> `docs/semantics/model.md` §7, never the generator's code; (5) the bindings hold together by §8's joint rules, today
> rule 7's hardware, rules 2–4's channels, rule 5's ownership, rule 10's placement, rule 9's privilege at the
> profile's level and rule 13's one provider per function and qualifiers. A corrupted plan is `tool-failure`. The
> library is `crates/archogen-plan-check`; the executable `archogen-plan-check` runs it on files, and serves it to an
> agent as the [MCP](ledger.md#mcp-specification) tool `plan-check`, which takes the texts.

## How it works

### The record

The search writes its answer as a short eADL document, one clause per line, always in the same order. For the
heartbeat example, whose console service needs observable output and finds it at the serial port:

```text
(format "archogen-assignment/1")
(profile rt-static-up-v1)
(system heartbeat)
(program "…64 hexadecimal digits…")
(bind console.write observable-output console.uart)
```

The `program` line is a fingerprint of the whole description as the engine read it — every module, its version and
what it was given — so a plan can always be traced to exactly the description it was made for. A task binds under its
system's name, `(task heartbeat beat)`, so no declaration can be mistaken for a task.

### The checks, in order

1. **The record reads, and belongs to this description.** A record in another format, out of order, binding one need
   twice, or naming a profile or system that is not this one, is refused. A fingerprint that is not this
   description's is a **stale plan**: the description changed after the plan was made. That is not a mistake of the
   toolchain, so it gets a status of its own, `stale-input`, and the advice is to resolve again.
2. **The description is checked again.** The checker runs the language's own checks over the description, with the
   language definition compiled into the checker, and adds a few rules about providers that only a planner meets: a
   timer offering a channel-taking function while declaring no channel, a memory region that does not read, memory
   for the program overlapped by a device, or a region with no size where its size alone would say whether it meets
   that memory — a reserved range among them, since the plan must know the extent of what it avoids. A description
   the toolchain refuses never has a plan accepted.
3. **Every need is bound, once, to a real provider.** The checker works out, from the description alone, who asks
   for something: the system, each of its tasks, and every service or part they use. It finds each need where the
   description writes it. Then every need on a function of a device must have exactly one binding, every binding
   must be such a need, and every provider named must be a block or platform the description declares.
4. **Every binding holds.** Each need is judged against the provider it is bound to, by the same reference model the
   rules were written from. A counter of 16 bits does not serve a need for at least 32, and a part that does not
   describe a fact does not serve a need for it.
5. **The bindings hold together.** Some mistakes only show when the bindings are read side by side. Today the checker
   asks six such questions. First, is every provider part of the system's own **hardware** — the platform its
   `platform` clause names, the blocks and platforms the system and its tasks name, and everything those compose? A
   serial port on the board that the system's platform never composes serves nobody, however well it would match.
   Only blocks and platforms compose: a service that names a block does not bring it into the system, and a platform
   that refines another keeps that platform's promises, not its parts. Second, does every timer have enough
   **channels**? A need that programs a timer — a delay, a deadline — takes one of its channels, while a need that only
   reads it takes none. A timer with two channels serves two such needs, not three; a service several tasks share
   counts once, since it programs the timer for all of them; and where a timer does not say how many channels it has,
   two needs on it cannot be known to fit. Third, does any address belong to two parts at once? Two parts the plan uses
   whose registers share an address would fight over it — unless one is built from the other, in which case they are
   one part; only blocks and platforms build, so a part reached through a service is still a part of its own. Memory
   for the program belongs to no part, and a part the plan does not use claims nothing. Fourth, does
   the plan say where the program goes, and is that memory the program can run from, on the system's own hardware?
   Fifth, can each caller reach what it is bound to? The profile runs the runtime, the tasks and the services at one
   processor level — `machine` under `rt-static-up-v1` — and every part a task or service is bound to must be reachable
   from it; a part that does not say where it can be reached from cannot be known to be. The level comes from the
   profile, never from the plan, so a plan cannot pass by claiming another. A service that says it calls from a level
   the profile does not run anything at describes something this profile cannot build: that is the description's
   `unsupported-profile`, said before anything else of the plan. Sixth, is each device used whole? A service that
   reads time reads one counter: its width, rate and wrap come from the same part, or the readings mean nothing
   together. So one requester's needs on one function — a counter, a clock, a queue — are bound to one provider. And a
   need that qualifies everything its requester relies on, such as being available while the processor idles, is
   checked against every part the requester is bound to through its functions, and every part the services it uses are
   bound to.

The checker re-reads the needs every time. Changing a requirement and recomputing the fingerprint by hand does not get
an old plan through: check 4 judges the new requirement and refuses.

### Running the checker

The checks run as a program of their own, `archogen-plan-check`, which an install places beside `archogen`, and
which `archogen plan-check` runs, handing over its arguments as they are:

```text
archogen plan-check examples/periodic-three/system.eadl build/plan.json
archogen-plan-check [--profile ID] [--result PATH] [--] DESCRIPTION PLAN
```

It reads the description — a module, whose imports it finds in the module's directory, or a plain description — and
the plan; runs every check in order; writes its **result record**, `archogen-result/1`, to `PATH`, or to standard
output when no path is given; and exits with the result's status code. `--` before the two paths lets one begin with
a dash. The record names the program that ran, by its name and the fingerprint of its own file, and what
it judged: the fingerprint of the program the description became and of the plan. So whoever started it — `archogen`,
a script, a CI job — accepts a result only when the record and the exit code agree. A call it cannot take, or a file it
cannot read, writes no record and exits `2`: nothing was judged.

A plain description, a file of declarations with no `defmodule`, is read as one module of its own for the plan's
fingerprint; so a plan names it as surely as a module.

### As an MCP tool

An agent — a model, a script, an orchestrator — reaches the same check without files. `archogen-plan-check --mcp`
serves one tool, `plan-check`, over MCP, one message per line on standard input and output until the input ends, as
`archogen mcp` serves the generator's: both run over one framing, `crates/archogen-transport`, which belongs to no
role.

```text
archogen-plan-check --mcp
```

The tool takes **texts, never paths**: `description` and `plan`, and optionally `name` (how findings name the
description, `description.eadl` when absent), `profile`, and `modules`, each module the description imports mapped to
its text. Findings name the plan `plan.json`. It answers the **result record** itself: the record's bytes as the one
text block, the same JSON as the structured content, and `isError` unless the status is `ok` — so a client that reads
nothing but `isError` never takes a refusal for a pass. It is the record the executable writes for files holding the
same texts, byte for byte, naming the same program, the file that runs.

Like a call the executable cannot take, an argument the tool cannot use is answered with what to change and judges
nothing: an unknown argument, a missing text, a profile the checker does not have, or texts past its budget of
4 MiB, the description, the modules and the plan counted together. A line longer than six times that budget, and a
fixed allowance for the envelope, is refused unread. The tool is read-only: it touches no file and starts no program.

`--mcp` stands alone; through `archogen plan-check`, which gives the checker no input, the server ends at once.

### In a browser

A page can judge a plan too, with no program installed: the checker compiles to a WebAssembly module of its own,
`crates/archogen-plan-check-wasm`, which shares nothing with the generator's module — a page judging a plan runs no
code of the search that wrote it. The module imports nothing, so a page that loads it gives it no way to reach
anything; it is driven through three functions, `plan_check_input`, `plan_check` and `plan_check_output`, as the
generator's module is through its own.

The page hands it the same texts the MCP tool takes — the description, its modules, the profile, the plan — and the
module's own fingerprint, which the page computes from the bytes it loaded, since a module cannot read its own file.
It answers the **result record**, the one the executable and the tool write for the same texts, naming itself
`archogen-plan-check-wasm` and that fingerprint: a record from a page is never taken for the installed checker's. A
request it cannot judge — framed wrongly, a fingerprint misspelt, another profile, texts past the 4 MiB budget — gets
a short refusal of its own format, `archogen-plan-check-wasm-refusal/1`, saying what to change, and no record.

The request's layout, field by field, is `docs/decisions/decision_wasm-binding.md` §11. The loader a page uses, and
the checks that the compiled module answers as the host build does, are leaf `M3.5.7.2`'s.

### What binds, and what does not

Not everything a description requires is served by one provider:

- a need on a fact that belongs to a device's function — a counter's width, a timer's channel, an output — binds;
- a **statement** is the requirer's word about itself and binds nothing;
- `reachable-at-privilege` is judged against the level the profile gives the system, not by a binding;
- a **qualifier**, such as `available-in-state`, describes every function its requirer relies on. Where the requirer,
  or anything it uses, binds a function, the qualifier is judged at those providers and binds nothing itself; where
  nothing does, it binds like any other need.

## The precise rules

### Statuses

| Outcome | Status | Exit |
| --- | --- | --- |
| every check passes | `ok` | `0` |
| the plan was made for another description | `stale-input` | the status vocabulary's |
| the description is refused, by the language or by the checker | the description's own: `invalid-description`, `missing-fact`, `unsupported-profile`, … | the description's |
| the record is corrupted or does not match | `tool-failure` | the status vocabulary's |

A corrupted plan is the toolchain's fault — "never reported as a valid system" — so it is `tool-failure`, whatever was
wrong with it. A stale plan is the user's input out of date, and is refused before any other check.

### Codes

Each check reports under a code of its own. The codes are stated, with what fires them and an input that does, in
`docs/semantics/reference.md` §4; `crates/archogen-plan-check/tests/fires_on.rs` runs each input through the
`archogen-plan-check` program:

| Code | Check | Fires when |
| --- | --- | --- |
| `plan-record` | 1 | the record does not read, or names another profile or a system the description does not declare |
| `plan-stale` | 1 | the record's digest is not the description's |
| `plan-description` | 2 | the checker's own rules refuse a description the language's passes accept |
| `plan-binding` | 3 | a need left unbound, a binding of no need or of a requester the system does not serve, a provider not declared |
| `plan-unsatisfied` | 4 | a binding the reference model does not satisfy |
| `plan-joint` | 5 | the bindings do not hold together: a provider outside the system's hardware, more needs taking a channel than a provider has or two where its channels are unknown, two bound parts owning one address, the image placed nowhere or outside placement memory, a caller's provider not reachable at the profile's level, one requester's function split across providers, or a qualifier unmet where its requester relies |

Each finding names the requester and the fact, and points at the requirement in the description, the binding's line in
the record, or both — for check 4, also at what the provider states. The language's own diagnostics keep their codes.

### Where it lives

- `crates/archogen-plan-check/src/assignment.rs` — the record, its canonical writer and strict reader, and the
  program's digest;
- `crates/archogen-plan-check/src/check.rs` — the checks, in order, and their codes;
- `crates/archogen-plan-check/src/requesters.rs` — who requires, and where: the requesters a system serves, each
  requirement located by the checker and held by a test to the reference model's own reading;
- `crates/archogen-plan-check/src/bindings.rs` — checks 3 and 4;
- `crates/archogen-plan-check/src/providers.rs` — check 2's provider-level rules;
- `crates/archogen-plan-check/src/judgement.rs` — one judgement made the result record, which the executable writes
  and the tool answers;
- `crates/archogen-plan-check/src/mcp.rs` — the tool `plan-check`, over `crates/archogen-transport`;
- `crates/archogen-plan-check-wasm/src/lib.rs` — the browser module: its answer and its three exports, over
  `crates/archogen-plan-check-wasm/src/request.rs`, the request's framing;
- `crates/archogen-plan-check/src/joint.rs` — check 5, the joint rules, written from `docs/semantics/model.md` §8
  alone, never from the search's own version of them. One reading of it is public, `joint::system`: a system's
  hardware, the declarations it reaches and the requesters it serves, computed by the code the rules judge with, so
  that another crate can compare it with the search's;
- `crates/archogen-plan-check/tests/plan_f12.rs` — the corruptions of fixture F12, each refused under its code;
- `crates/archogen-plan-check/tests/plan_joint_fenced.rs` — each of check 5's rules, accepted and corrupted plans run
  through checks 1 to 4 and then check 5, written with it by the fenced context;
- `crates/archogen-plan-check/tests/system_reading.rs` — `joint::system` on systems whose hardware lies at depth,
  behind a service or behind a module's import, written by a fenced context too;
- `crates/archogen-plan-check/tests/mcp.rs` — the tool: its listing, the arguments it refuses, its budget, and its
  record read back and held equal to the executable's for the same texts;
- `crates/archogen-plan-check-wasm/tests/module.rs` — the browser module on the host: its record the tool's for the
  same texts, its refusals, its exports, and no generator package in what it compiles.

## Today and ahead

The checks above run as a library, and as the `archogen-plan-check` program; each code's input runs through the
program. Check 5's rules were written, like the rest, by a context that never read the search's own version of them.
Where the two meet, a test compares them: `crates/eadl-resolve/tests/system_reading.rs` holds the search's reading of
a system — its hardware, what it reaches, whom it serves — equal to check 5's over every tracked system and over
thousands of generated ones, which name each other at every position a description can; the trust gate lists it as a
derivation the two roles share (leaf `M3.5.5.7.2`). The program is the configuration checker's root in the trust
gate, and `archogen plan-check` runs it from the front (leaf `M3.5.1.4.1`). The front's half of a checked plan is
built too — it runs the checker over a plan of its own and believes the result only when the record, the exit code
and its own reckoning agree (`M3.5.1.4.3`); `archogen resolve`, which calls it after its search writes the plan, is
`M3.4`'s. The checker serves its own MCP tool (leaf `M3.5.1.6.4`) and has a browser module of its own (`M3.5.7.1`), whose
loader and artifact checks are `M3.5.7.2`'s.
