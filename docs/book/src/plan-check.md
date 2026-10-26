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
> `docs/semantics/model.md` §7, never the generator's code. A corrupted plan is `tool-failure`. The library is
> `crates/archogen-plan-check`; the joint rules (check 5) and the executable `archogen-plan-check` are still to come.

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
   timer offering a channel-taking function while declaring no channel, a memory region that does not read, or
   memory for the program overlapped by a device. A description the toolchain refuses never has a plan accepted.
3. **Every need is bound, once, to a real provider.** The checker works out, from the description alone, who asks
   for something: the system, each of its tasks, and every service or part they use. It finds each need where the
   description writes it. Then every need on a function of a device must have exactly one binding, every binding
   must be such a need, and every provider named must be a block or platform the description declares.
4. **Every binding holds.** Each need is judged against the provider it is bound to, by the same reference model the
   rules were written from. A counter of 16 bits does not serve a need for at least 32, and a part that does not
   describe a fact does not serve a need for it.

The checker re-reads the needs every time. Changing a requirement and recomputing the fingerprint by hand does not get
an old plan through: check 4 judges the new requirement and refuses.

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
`docs/semantics/reference.md` §4; `crates/archogen-cli/tests/fires_on.rs` runs each input through the checker:

| Code | Check | Fires when |
| --- | --- | --- |
| `plan-record` | 1 | the record does not read, or names another profile or a system the description does not declare |
| `plan-stale` | 1 | the record's digest is not the description's |
| `plan-description` | 2 | the checker's own rules refuse a description the language's passes accept |
| `plan-binding` | 3 | a need left unbound, a binding of no need or of a requester the system does not serve, a provider not declared |
| `plan-unsatisfied` | 4 | a binding the reference model does not satisfy |

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
- `crates/archogen-plan-check/tests/plan_f12.rs` — the corruptions of fixture F12, each refused under its code.

## Today and ahead

The checks above run as a library, and each code's input runs through it. Still to come: the joint rules — channels,
ownership, privilege, placement and one provider per function — as check 5, written by a context that never reads the
search's own version of them; and the `archogen-plan-check` program itself, which `archogen plan-check` and `archogen
resolve` will run. The search that writes the record is `M3.4`'s.
