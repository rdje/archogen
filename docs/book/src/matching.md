# Matching an offer to a requirement

## The idea, in plain words

A description says what each part of a system **offers** and what each part **requires**. A timer block offers a
counter that is 32 bits wide and ticks ten million times a second; a timekeeping service requires a counter whose
readings stay unambiguous for at least a minute. *Matching* answers the question in between: does this offer satisfy
that requirement? It answers one fact at a time, and only for facts the engine has a rule for.

Two things make the question harder than it looks. First, **more is not always better**. A wider counter is better
for a service that needs *at least* 32 bits, but not for a driver that reads exactly one 32-bit word and relies on
what it read. A faster counter is not better either: it wraps around sooner, so the span over which two readings can
be told apart gets shorter. Second, **what a part accepts of its caller is not something more of which is better**.
A function you can only reach from the processor's most privileged mode does not serve a caller in the mode below,
however "strong" that sounds. Matching treats such things as a list of what is accepted, and asks whether the
caller's case is on the list — never whether one level outranks another.

An everyday comparison: a fuse, a plug and a key. For a fuse, a higher rating is fine; for a plug, the pins must be
exactly the right shape; for a key, it opens your door or it does not, whatever else it opens. Each fact the engine
judges is marked, in one shared list called the **capability vocabulary**, with which kind of fact it is.

> **In one minute, for engineers.** The vocabulary `/1` (`docs/semantics/vocabulary/vocabulary.eadl`, written with
> the kind `deffact`) gives every judged fact a domain, a role and a direction: `at-least`, `at-most`, `exact`,
> `includes` (sets) or `within` (intervals); `exactly` is the one direction either side may always write. Values are
> exact rationals over `i128`, and an overflow is `unsupported-profile`, never a verdict. A fact has one outcome at a
> provider — valued, absent, unknown, derived or undescribed — and `unambiguous-horizon` is derived as
> `(modulus − 1) / rate`, inclusive. Preconditions are offered sets judged by inclusion, so no stronger precondition
> passes as a stronger capability. The relation is a predicate per (requirement, provider) and a preorder per fact;
> no lattice is claimed. It lives in `crates/eadl-resolve`, is held to the design's executable model over the model
> checker's universe, and `archogen check` does not call it yet.

## How it works

### The vocabulary

Every fact a requirement may constrain is one entry of the vocabulary. The entry says what kind of value the fact
has, whose word it is, and which way an offered value must lie against a required one:

```text
(deffact unambiguous-horizon
  (doc "the longest interval whose elapsed ticks two reads of the counter determine without wrap ambiguity: its modulus less one, over its rate")
  (domain quantity time)
  (role guarantee)
  (direction at-least)
  (derived-from counter-modulus tick-rate)
  (reads wrap-behavior)
  (rule horizon-from-modulus-and-rate))
```

The direction belongs to the fact, so it is checked wherever the fact is written:

| Kind of value | Example fact | Direction | Satisfied when |
| --- | --- | --- | --- |
| boolean | `observation-coherent` | `exact` | the values are equal |
| count | `queue-capacity` | `at-least` | the offered count is at least the required one |
| quantity | `delivery-bound` | `at-most` | the offered time is at most the required one |
| interval | `frequency` | `within` | the required range lies inside the offered one |
| enumeration | `tick-unit` | `exact` | the same alternative |
| set | `reachable-at-privilege` | `includes` | every required member is offered |
| group | `absolute-deadline` | `exact` head | the head holds and every sub-fact's requirement does |

A fact the vocabulary does not declare may still be offered, declared absent or needed — presence judges that — but a
constraint on it is `unsupported-profile`: no rule decides a value without a documented direction.

### Read or taken: consumed facts and channels

Some functions are read, and any number of users can read them at once: two services can both read one counter.
Others are *programmed*, and each user needs a piece of hardware of its own: two services cannot both own one alarm
comparator. The vocabulary says which is which. An entry may write `(use consumed)`; without it, a fact is `shared`.
`/1` marks two facts consumed, both alarms:

```text
(deffact relative-delay
  (doc "a delay programmed relative to the time it is set")
  (domain boolean)
  (role guarantee)
  (direction exact)
  (use consumed))
```

`absolute-deadline` is the other; every other entry is shared. A provider then says how many users it can serve at
once through its consumed facts with one more entry, `channels`, a count where more satisfies more:

```text
(defblock timer.compare
  (offers (absolute-deadline true) (channels 1 tick)))
```

The joint check counts them (below): a requester bound to a consumed fact at the value `true` takes one channel, and a
provider with more requesters than channels is refused. `use consumed` is written only on a boolean fact or a group's
head, whose bare offer is `true`; anywhere else, or with any word but `shared` and `consumed`, the vocabulary is
refused.

### One device's facts: functions

A service that reads time reads one counter: its width, its rate, how it wraps and whether a read can tear are all that
counter's. Bound to two counters — the horizon met by one, the coherent reads by another — it would read a time no
device keeps. So every entry names the **function** it describes, the operations of one device that are only valid
together, and the joint check keeps one requester's facts of one function at one provider (`M3.3.6`):

```text
(deffact observation-coherent
  (doc "a read of the counter's value never observes a torn update")
  (domain boolean)
  (role guarantee)
  (direction exact)
  (function counter))
```

`/1`'s functions: `counter` — the width, modulus, rate, unit, wrap behaviour, horizon, coherent reads and low-power
counting, and the two alarms with `channels`, since an absolute deadline is a time in the counter's time base and a
relative delay a comparator on it; `interrupt` — a source and its arrival separation; `arrival` — `bounded-arrival`,
which the realization gives; `output`, `queue`, `transfer`, `processor`, `scheduler`, `clock`, `release`, `debug`,
`mutex` and `bus`. Two facts are **qualifiers**, `(function each)`: `available-in-state` and `reachable-at-privilege`
describe every function their requirer relies on, as an operating condition must hold of all of it.
`or-through-mediation`, the requirer's word about itself, names none. A `requires` clause is not the unit: a description
may write one counter's facts across a clause and a `needs` of its own, as `examples/bounded-queue/system.eadl` does.

### What a provider offers

A **provider** is a block or a platform. It offers a fact bare, with a value, or, on an abstract platform, with a
bound a refinement must keep:

```text
(defblock timer.counter
  (offers (counter-modulus 4294967296) (tick-rate 10 MHz) uart))
(defplatform soc.abstract
  (offers (counter-width (at-least 32 bit))))
```

`(tick-rate (exactly 10 MHz))` in an offer is simply the value `10 MHz`. An abstract bound is not a value: it meets a
`needs`, and it leaves a valued requirement unknown. A provider that says two things of one fact — two values, a
value and a bound, an offer and an absence — is refused, since only its author knows which was meant.

### What a side requires

A requirement constrains one fact: a bound in the fact's direction, a bare value read in that direction, or a value
under `exactly`. Presence is written `(needs f)`. A group requires its head and each sub-fact:

```text
(defservice time.monotonic
  (requires (unambiguous-horizon (at-least 60 s)) (tick-unit ns)))
(defservice deadline.absolute
  (requires (absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))))
```

Everything one declaration requires is read together, wherever it is written, so two clauses that cannot both hold
are refused exactly as two constraints in one clause would be.

### One outcome per fact, then a verdict

At one provider, each fact has exactly one outcome, in this order: the **value** it offers; **absent**; **unknown**
(offered without a value); **derived** by the fact's rule; or **undescribed**. The requirement is then judged against
it: satisfied, refused, or one of absent, unknown and undescribed when there is no value to compare.

The horizon shows why derivation matters. A counter wraps at its modulus; between two readings a time `Δ` apart it
advances about `Δ × rate` ticks, and the two readings tell that advance apart only while it stays below the modulus.
So the unambiguous span is `(modulus − 1) / rate`:

| Counter | Horizon | Against `(at-least 60 s)` |
| --- | --- | --- |
| `(counter-modulus 4294967296)` at `10 MHz` | `429.4967295 s` | satisfied |
| `(counter-modulus 65536)` at `10 MHz` | `6.5535 ms` | refused |
| `(counter-modulus 600000000)` at `10 MHz` | `59.9999999 s` | refused — `modulus / rate` would have said exactly 60 s |

The same counter at `1 MHz` has ten times the horizon: slower is better here, because the direction belongs to the
horizon, not to anyone's sense of which counter is faster.

### Sets: what a part accepts

`reachable-at-privilege` lists the levels a function can be called from. A supervisor caller is served by an offer
of `supervisor machine` and not by `machine` alone, nor by `user`. `available-in-state` works the same way, with one
member always implied: a requirement for `idle` asks for `idle` and `run`, the state every use runs in.

### Several bindings at once: channels, owned regions, connectivity and routes

The relation answers for one requirement at one provider. A system makes many requirements at once, and some answers
that are each fine alone are wrong together. The joint check (`crates/eadl-resolve/src/joint.rs`) judges one complete
**assignment** — every requirement bound to one provider that satisfies it — for such mistakes, and reports what the
assignment leaves for the plan to do.

**Too many requesters for a provider's channels** (`ROADMAP.md`'s case F08). Two services each program an absolute
deadline, and the platform has one comparator with one channel:

```text
(defblock timer.compare
  (offers (absolute-deadline true) (channels 1 tick)))
(defservice alarm.engine   (requires (needs absolute-deadline)))
(defservice alarm.watchdog (requires (needs absolute-deadline)))
```

Each service alone is satisfied by `timer.compare`. Bound together, they need two channels, and the assignment is
refused, `infeasible-configuration`, naming both: *`timer.compare` serves 1 requester(s) at once through its consumed
facts, and 2 take one*. With `(channels 2 tick)` it holds; with one service it holds, since offering the function at
all serves one; with no `channels` written it is `missing-fact`, never a quiet yes. A requester is a task, a service,
or another declaration's own level, so two tasks that each write `(needs absolute-deadline)` are two requesters. A
service two tasks *use* is one requester: its realization serves both through one channel, so it is the multiplexer,
and the check reports that it owes them the shared semantics and a bounded overhead.

**Two providers owning the same hardware** (case F09). The platform offers a relative delay by programming its
comparator directly, and an absolute deadline through firmware that programs the same comparator:

```text
(defblock timer.direct
  (offers (relative-delay true)
    (region clint.mtimecmp (base 0x0200_4000) (size 8 byte))))
(defblock timer.firmware
  (offers (absolute-deadline true)
    (region sbi.timer-window (base 0x0200_0000) (size 64 KiB))))
```

A service needing each is satisfied alone; bound together, the two providers own overlapping bytes — the firmware's
window holds the comparator's registers — and the assignment is refused. A provider nothing is bound to owns
nothing, so an unused device costs nothing; and a provider never conflicts with one it composes through `uses`, as
a platform's window holds its own blocks.

A refused assignment is not yet a refused description: another assignment may hold. `M3.4`'s search tries them, and
the description is `infeasible-configuration` only when every assignment is refused. Both cases are descriptions
under `crates/eadl-resolve/tests/joint/`, and `crates/eadl-resolve/tests/joint.rs` refuses every assignment of each,
beside controls that hold.

**A device the system does not have.** An assignment is one system's, and a block or platform it binds is part of
that system's **hardware**: what its `platform` clause names, what its own and its tasks' `uses` and `needs` name, and
what those name in turn, at any depth. A block that offers the right fact but sits on no platform the system uses
cannot serve it:

```text
(defplatform bus.main (uses bus.timer) (offers (bus-width 32 bit)))
(defplatform bus.fast (refines bus.main) (needs bus.clock) (offers (bus-width 32 bit)))
(defblock bus.timer   (offers (counter-width 32 bit)))
(defblock bus.clock   (offers (clock-rate 10 MHz)))
(defblock stray.clock (offers (clock-rate 10 MHz)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs clock-rate))
  (platform (uses bus.fast)))
```

`s`'s hardware is `bus.fast` and `bus.clock`, which `bus.fast` names through `needs`. Binding the task's `clock-rate`
to `bus.clock` holds; binding it to `stray.clock` is refused, `infeasible-configuration`: *`s/t` is bound to
`stray.clock`, which is not part of `s`'s hardware*, with the repair *compose `stray.clock` into `s`'s platform, or
bind `s/t` to a provider its platform composes*. `bus.timer` is not `s`'s either: `bus.fast` *refines* `bus.main`, and a
refinement keeps the abstract platform's obligations, not its blocks. A system with no platform has no hardware, so
every binding of its requesters to a block is refused. Two systems in one description are judged apart, each against
its own board, so two boards that each put a timer at the same address never meet. A requester no system reaches is
judged for channels alone. What a description's code becomes when only unconnected providers offer a fact is decided
with the search (`M3.4`).

**The interrupts an assignment relies on.** Under one active core, every interrupt a plan relies on must be routed
from its source to the core, and the analysis requires the interrupts the build enables to equal its declared sources
plus the timer. The description says which source is part of the system; the controller between them, its line and
the latency are the catalog's. So the check reports a **route obligation** for each provider an assignment relies on
for an interrupt — never a refusal. In the sporadic-release case,

```text
(defblock event.line (offers interrupt-source min-arrival-separation))
(defplatform soc.p (offers (core-count 1 tick)) (requires (uses event.line)))
```

the service released by `event.line` gives one *source* route, carrying what `event.line` states of its own
`min-arrival-separation` — here offered without a value — for the analysis to read; and in case F09 each of the two
providers of the one comparator, direct and through firmware, gives a *timer* route of its own. The plan enables each, and no other interrupt.

**Privilege.** A plan runs one system's callers — its own level, its tasks, the services and policies it uses — at one
privilege level, and every provider a caller is bound to must be reachable from it and say so. Given `machine`:

```text
(defblock timer.m (offers (counter-width 32 bit) (reachable-at-privilege machine)))
(defblock timer.s (offers (counter-width 32 bit) (reachable-at-privilege supervisor)))
```

a task needing a counter is served by `timer.m`; bound to `timer.s` it is refused, `infeasible-configuration`: *`s/t`
runs at `machine`, and `timer.s` is reachable from `supervisor` alone*. A provider that does not say which levels it is
reachable from is `missing-fact`, and one declared reachable from none `infeasible-configuration`. A service that states
it calls from `supervisor` — `(requires (reachable-at-privilege supervisor))` — is `unsupported-profile` first, whatever
it is bound to, since this profile runs no caller there. A block needing a sibling's fact is no caller and has no level,
and a route carries the level its interrupt is delivered at. The level is the profile's: `rt-static-up-v1` runs the
runtime, its tasks and its services at `machine`, the level a RISC-V hart starts in at reset
([privileged specification](ledger.md#riscv-privileged) §2.1.4), and the hosted playground models it
(`docs/profiles/rt-static-up-v1.md`). `M3.4` gives it to every system it resolves; with none given the rule says nothing.
Every block and platform in a suite system's hardware states `(reachable-at-privilege machine)`, held by a census
over the whole suite that excludes no description, uc3's `examples/alternative-timer` among them
(`crates/eadl-resolve/tests/privilege_census.rs`).

**Where the image goes.** A region a platform offers with `(executable true)` is memory a plan may place code in, and
`(reserved true)` marks a range nothing may be placed in. Judged once per system:

```text
(defblock fw  (offers (region fw.range (base 0x8000_0000) (size 2 MiB) (reserved true))))
(defblock dev (offers (region dev.regs (base 0x1000_0000) (size 4 KiB))))
(defplatform soc (offers (region ram (base 0x8000_0000) (size 128 MiB) (executable true))) (requires (uses dev fw)))
```

`ram` is placement memory, and the firmware's reserved range inside it is a **hole**: the check reports a placement
obligation — `ram`, with the hole `fw.range` — which the plan lays the image out around and the build verifies. A
device's registers inside `ram`, or a second placement region overlapping it, are two claims on the same bytes,
`invalid-description`: *`dev`'s region `dev.regs` lies in `soc`'s placement memory `ram`*, with the repair *mark the
range `(reserved true)`, or move it out of the memory*. A region that is both executable and reserved is no placement
memory, and a region's sub-forms are written once each, a flag `true` or `false`. The plan chooses the region the image
goes in — a choice of anything but placement memory is refused — and lays out the offsets later (`M4.1`). The hosted
playground offers no such region and needs none: its host provides memory.

A required **ordering** between two effects — `(ordering (before (write device.control)) (after (write
memory.buffer)))` — is `unsupported-profile` under this profile: ordering matters where devices move data without the
processor, which the profile's `dma` exclusion leaves to a later profile, and its vocabulary entry waits for that one.

**What starts first.** A description never writes a boot sequence: it states what each part needs, and the check
derives the **initialization graph** from the assignment for the plan. A provider comes before each requester bound to
it — the counter before the clock service reading it — and a service before each service that `uses` or `needs` it;
a block a platform composes is part of it, so no edge runs between them, and the system's own level and its tasks,
which start after everything is initialised, are no nodes. Nothing is refused: a log that timestamps through a clock
that logs its wrap forms one **component**, which the engine initialises as one unit, since initialisation runs with
interrupts masked, before any task is released.

**One counter, read whole.** A requester's facts of one function are met by one provider. With the width offered by
one timer and the rate by another, neither offering both, the assignment is refused:

```text
(defblock timer.a (offers (counter-width 32 bit) (tick-rate 1 MHz)))
(defblock timer.b (offers (counter-width 8 bit) (tick-rate 10 MHz)))
(defservice time.monotonic (requires (counter-width (at-least 16 bit)) (tick-rate 10 MHz)))
```

*`time.monotonic`'s `counter` facts are bound to 2 providers: `counter-width` at `timer.a`; `tick-rate` at `timer.b`*,
`infeasible-configuration`, with the repair *bind `time.monotonic`'s `counter` to one provider that offers it whole, or
to an adapter that does*. Where a timer would be whole once a fact it leaves undescribed were described — the horizon
derived at one, the coherent reads offered at the other — the finding is `missing-fact`, naming that timer and the fact
to describe. A delay is its counter's comparator, so a 16-bit delay beside another block's 32-bit width is refused the
same way. Two services each reading a counter of its own hold: the rule is one requester's. A **qualifier** holds
everywhere its requester relies on: `(requires (available-in-state idle))` on a service that `needs` the clock service
is judged at the counter that service reads — *`time.wakeup` requires `available-in-state` of every function it relies
on, and `timer.t` does not meet it* — and the privilege level stays rule 9's. A census over the suite finds no
requester whose function its hardware offers only across providers, and lists the five counters whose horizon or wrap
behaviour no block states yet, with their only routes (`crates/eadl-resolve/tests/function_census.rs`).

### The list a resolver chooses from

Matching does not choose. For one requirement it lists every provider in the order given, with its verdict, the
value it found, the derivation it used and the direction it judged in; for a `requires` clause, whether each provider
satisfies all of it. Choosing among them, combining them and checking capacity and ownership is the resolver's work
in later leaves.

## The precise rules

The design is `docs/decisions/decision_substitutability-relation.md`; `docs/semantics/model.md` §7 is normative over
the code. Section numbers below are the record's.

### The vocabulary is a typed table

`crates/eadl-resolve/src/vocabulary.rs` reads the vocabulary against `docs/semantics/kinds/deffact.eadl`, holds each
entry's frame to the kind with the validator every declaration meets, and then refuses what a frame cannot say: a
direction its domain does not admit (§2's table), a derivation without its rule or with inputs that rule does not
read, a cycle among `derived-from` and `reads`, `implies` off a set fact, `use consumed` off a boolean fact or a
group's head, a guarantee naming no `function` or a statement naming one, `each` on a consumed, grouped or derived
fact, a group's sub-fact or a derivation's input of another function, a name twice, and a fact named like a clause word of the description kinds, since an offer on it would
read as the clause (§1.1). The vocabulary is part
of `eadl/1`: it is frozen with the language, and `archogen check` answers a file that writes a `deffact` as the
language's own definition, exit `20` ([Checking a description](checking.md)).

### Values and arithmetic

`crates/eadl-resolve/src/value.rs` (§2). A count is a non-negative integer, with an optional dimensionless unit, or
`(pow2 N)` for `N` from 0 to 126. Two amounts in one unit compare by their written numbers; in two units, in the base
unit. Information is a positive whole number of bits; a counter's modulus is positive. A comparison whose exact
arithmetic overflows decides nothing and is `unsupported-profile` — the value domain's limit, which a wider domain
would lift under §15.

### Offers

`crates/eadl-resolve/src/offer.rs` (§1, §2, §4, §5, §8). Refused, `invalid-description`: an item of `offers` naming
nothing or holding a clause; a list inside `absent`; an offer or an absence of a statement fact; a value outside its
domain; a direction's name, `includes`, `within` or `exact`, used as a wrapper; a boolean offered with a bound other
than `exactly`; a bound against the fact's direction; one fact offered and absent, with two values, or with a bound
beside a value; a derived fact offered or declared absent beside a fact its rule reads; a modulus above `2^width`,
or beside `(wrap-behavior saturating)`. One value written in several spellings is one offer, kept in every spelling,
and a comparison is decided by any spelling whose arithmetic does not overflow (§5).

### Requirements and sides

`crates/eadl-resolve/src/requirement.rs` (§1, §3 rules 5 and 6). A side holds every `requires`, `needs` and `uses`
written at any depth of a clause but `offers`, `absent` and `refines` — the positions presence reads. A `platform`
clause holds `needs`, `uses` and `requires` alone; `requires` holds constraints, `needs`, `uses` and nested
`requires`, each a clause of its own; `needs` and `uses` hold names. A `uses` naming a fact, a `needs` of a
statement, a bare name or `(f)` inside `requires` are refused. On one fact, two equalities that differ, or an
equality another constraint refuses, are a contradiction; what the arithmetic cannot decide is
`unsupported-profile`. A declaration whose local name is a fact is refused, at the root and inside every imported
module, where its qualified name would hide it (§1.1).

### Outcomes and verdicts

`crates/eadl-resolve/src/relation.rs` (§3, §4). Rule 1's order is value, absent — declared, or a required input of
the derivation declared absent — unknown — offered without a value, or an input the rule reads so offered — derived,
undescribed; an optional input declared absent derives nothing. A boolean named in `needs` is `(f true)`, so `false`
meets no presence; any other offer meets presence. A group's verdict is the first of refused, absent, unsupported,
unknown and undescribed among its head and its parts, whatever order they are written in. A statement, such as
`or-through-mediation`, is the requirer's own word: checked for its domain, judged against no provider.

### Codes

Every refusal is reported under the record's two codes, rows of `model.md` §6: `invalid-description` for what a
description writes that cannot mean anything, and `unsupported-profile` for a fact no rule decides or a comparison
past the arithmetic. The diagnostic names the fact and what is wrong and says what to do; the library keeps each
rule a typed cause (`crates/eadl-resolve/src/refusal.rs`). A code of its own per cause waits until a command can reach
the relation (`model.md` §7 rule 6).

### Joint rules

`docs/semantics/model.md` §8 is normative over `crates/eadl-resolve/src/joint.rs`: who a requester is (rule 1), when
a binding takes a channel (rule 2), the channel count and its codes (rule 3), the service that multiplexes (rule 4),
region ownership and its codes (rule 5), and that one assignment's findings decide nothing about the description until
the search has tried every assignment (rule 6); then one system's assignment, its hardware and connectivity (rule 7),
the route obligations (rule 8), privilege over a system's callers, given its level (rule 9), placement, once per
system (rule 10), ordering, unsupported under this profile (rule 11), the initialization graph (rule 12), and one
function at one provider, with the qualifiers (rule 13).

### What is not claimed

The relation is a predicate on one requirement and one provider. Per fact its direction induces a preorder and
nothing more: no join, no meet, no best offer, and no lattice (§7).

## Today and ahead

The relation is a library, complete for `M3.1`, and held to the design's executable model
(`crates/archogen-plan-check/src/model/`) over the model checker's whole universe by `crates/eadl-resolve/tests/production.rs`.
Measured `2026-10-06`: 154 381 provider-and-requirement pairs judged alike, 72 234 provider declarations read alike,
and the horizon agreeing with a simulation of the counter's reads on every case of the checker's grid. Each rule
has a catalogued mutation that its test must kill (`xtask/mutations.txt`).

`archogen check` does not call it yet, so no verdict a description gets today comes from it. Leaf `M3.4` wires it in,
over the providers of each system's hardware — a block no system uses decides nothing, as an unused device costs
nothing — deciding a description's `missing-fact` or `infeasible-configuration` from the
enumeration and keeping or migrating every verdict it moves. `M3.2` adds adapters — a provider in its own right, such
as an epoch extender — offered only where a requirer allows mediation; `M3.3` judges what one provider cannot: its
channels and owned regions (`M3.3.2`), connectivity and routes (`M3.3.3`) and privilege given a level (`M3.3.4.1`) are
judged today, over an assignment a test proposes, and placement over a system's map (`M3.3.5.1`); ordering is
unsupported under this profile (`M3.3.5.2`), the initialization graph derived (`M3.3.7`), and one function held at
one provider (`M3.3.6`); `M3.7` makes a catalog
record a provider.
