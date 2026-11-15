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
relative delay a comparator on it, and the five that describe its compare, how it is wired, its kind, its step, its
window and whether it reads whole numbers, which format `/2` reads; `interrupt` — a source and its arrival separation; `arrival` — `bounded-arrival`,
which the realization gives; `output`, `queue`, `transfer`, `processor`, its cores, and for `/2` how many interrupt
sources no mask stops and how many traps firmware beneath the image takes; `scheduler`, `clock`, `release`, `debug`,
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
or another declaration's own level — a block's or a platform's included, whose `requires` is bound like any other while
its `uses` stays composition (`M4.1.14`) — so two tasks that each write `(needs absolute-deadline)` are two requesters. A
service two tasks *use* is one requester: its realization serves both through one channel, so it is the multiplexer,
and the check reports that it owes them the shared semantics and a bounded overhead. Its users are the code that can
call it at run time (model §8 rule 4, `M4.1.2.4`): the jobs of each of the system's tasks that reaches it, and each
interrupt whose routine runs it — a source's handler that uses it, say. The system's own `uses` is none, nor a task of
another system, nor the service naming itself; and its own timer route, which only delivers its alarm back to it, is
none unless another part that route runs calls it there.

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
a platform's window holds its own blocks. Only blocks and platforms compose: a service's `uses` is a need of its own,
so a block reached through a service is still a part of its own.

A refused assignment is not yet a refused description: another assignment may hold. `M3.4`'s search tries them, and
the description is `infeasible-configuration` only when every assignment is refused. Both cases are descriptions
under `crates/eadl-resolve/tests/joint/`, and `crates/eadl-resolve/tests/joint.rs` refuses every assignment of each,
beside controls that hold.

**A device the system does not have.** An assignment is one system's, and a block or platform it binds is part of
that system's **hardware**: the blocks and platforms its `platform` clause names, or its own and its tasks' `uses` and
`needs` name, and the blocks and platforms those name in turn, at any depth — a service between them ends the path, so a
block that only a service names is not hardware, so a service naming one is `infeasible-configuration` for each system it leaves that block outside of, naming the service, the block and the system, with the repair *compose the block into the system's platform, or name hardware the platform composes*; a system whose platform composes the block is not refused for it (`M1.49`). A block that offers the right fact but sits on no platform the system uses
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
to `bus.clock` holds; binding it to `stray.clock` is refused, `infeasible-configuration`: *`(task s t)` is bound to
`stray.clock`, which is not part of `s`'s hardware*, with the repair *compose `stray.clock` into `s`'s platform, or
bind `(task s t)` to a provider its platform composes*. `bus.timer` is not `s`'s either: `bus.fast` *refines* `bus.main`, and a
refinement keeps the abstract platform's obligations, not its blocks. A system with no platform has no hardware, so
every binding of its requesters to a block is refused. Two systems in one description are judged apart, each against
its own board, so two boards that each put a timer at the same address never meet. A requester no system reaches is
judged for channels alone. What a description's code becomes when only unconnected providers offer a fact is decided
with the search (`M3.4`).

**The interrupts an assignment relies on.** Under one active core, every interrupt a plan relies on must be routed
from its source to the core, and the analysis requires the interrupts the build enables to equal its declared sources
plus the timer. The description says which source is part of the system; the controller between them, its line and
the latency are the catalog's, and whether a source reaches the core through that controller is stated with its
interrupt service, by whoever supplies that service: the generated code, where the plan generates it. So the check reports a **route obligation** for each provider an assignment relies on
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

a task needing a counter is served by `timer.m`; bound to `timer.s` it is refused, `infeasible-configuration`: *`(task s t)`
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
memory, and a region's sub-forms are written once each, a flag `true` or `false`. A bare `region` in an `offers`, with
no name and no bytes, says nothing of where anything is, so it is `invalid-description` wherever it is written, whether
or not anything uses the block. The plan chooses the region the image
goes in — leaving none chosen where there is placement memory, or choosing anything else, is refused — and lays out the
offsets later (`M4.1`). The hosted playground offers no such region and needs none: its host provides memory.

**A region with no size** (model §8 rule 10, `M4.1.2.5`). A region may leave out its `(size …)`, and then only its base
byte is known. What that means depends on what it is:

| The region with no size | Where it starts | What is concluded |
| --- | --- | --- |
| placement memory | anywhere | `missing-fact`, naming it: nothing can be laid out in it |
| a device's registers | below placement memory | `missing-fact`: only its size says whether it runs into the memory |
| a device's registers | inside placement memory | `invalid-description`: its base byte is already a second claim |
| a reserved range | below placement memory | `missing-fact`: only its size says whether it is a hole |
| a reserved range | inside placement memory | a hole for certain, taken to reach the region's end |
| a device's registers or a reserved range | above placement memory | nothing: a region grows upward, so it cannot reach down |

The hole is the case worth an example. It is certainly there, so the engine does not ask about it at once. It assumes
the largest extent the region allows, the end of the region, and carries on with whatever is left below it:

```text
(defblock fw (offers (region fw.range (base 0x8000_0800) (reserved true))))
(defplatform soc (offers (region ram (base 0x8000_0000) (size 4 KiB) (executable true))) (requires (uses fw)))
```

The first 2 KiB of `ram` stay free, so this holds, and `fw.range` is reported as a hole of `ram`. Move its base down to
`0x8000_0000` and nothing is left: the description is `missing-fact`, *`fw`'s reserved range `fw.range` has no size, and
with each such range taken to reach the end of `soc`'s placement memory `ram`, `s` has no free byte to place in*, with
the repair *write the reserved range's `(size …)`*. Every hole of no size in that region is named at once, so that one
pass repairs the description.

A size cannot repair everything. A sized range's bytes and the base byte of a range with no size are reserved whatever
any size says, so where those alone fill the region it is `invalid-description`, as for any memory reserved in every
byte, and no hole is named. The search, the configuration checker and, later, the layout (`M4.1.7`) all read a hole of
no size this same way.

A required **ordering** between two effects — `(ordering (before (write device.control)) (after (write
memory.buffer)))` — is `unsupported-profile` under this profile: ordering matters where devices move data without the
processor, which the profile's `dma` exclusion leaves to a later profile, and its vocabulary entry waits for that one.

**What starts first.** A description never writes a boot sequence: it states what each part needs, and the check derives
the **initialization graph** from the assignment for the plan. A provider comes before each requester bound to it — the
counter before the clock service reading it — and whatever a service or a policy names by `uses` or `needs`, a block
included, before it; a block a platform composes is part of it, so composition alone puts no edge between them, while a
binding between them does, as any binding does — a board whose platform and timer each need the other's facts is one
component — and the system's own level and its tasks, which start after everything is initialised, are no nodes. Nothing
is refused: a log that timestamps through a clock that logs its wrap forms one **component**, which the engine
initialises as one unit, since initialisation runs with interrupts masked, before any task is released. It does so in
one pass: the component's members one after another, in the order of their names, so none of them may call another's
code while it initialises.

**A task its own source releases.** A sporadic task that needs `interrupt-source` itself is released by every arrival
of the source it is bound to, so that source must not fire more often than the task's `min-separation` allows. Where
the description offers a source at least as far apart, the search binds that one; where it offers only faster ones,
or ones declaring their separation absent, the description is `unsupported-profile`, since this profile cannot release
a task on some arrivals only; and where the source states no separation, a bound among them, `missing-fact`.

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
on, and `timer.t` does not meet it* — and the privilege level stays rule 9's. There it binds nothing of its own: the
search writes no binding for it, so a provider failing it is passed over for one meeting it, and the configuration
checker refuses a plan that binds it (`M4.1.8`). A requester relying on no function binds its qualifier as any
requirement. `reachable-at-privilege` binds nothing at all: rule 9 judges it against the level the profile gives the
system (`M4.1.9`). A census over the suite finds no
requester whose function its hardware offers only across providers, and lists the five counters whose horizon or wrap
behaviour no block states yet, with their only routes (`crates/eadl-resolve/tests/function_census.rs`).

### The list a resolver chooses from

Matching does not choose. For one requirement it lists every provider in the order given, with its verdict, the
value it found, the derivation it used and the direction it judged in; for a `requires` clause, whether each provider
satisfies all of it. Choosing among them, combining them and checking capacity and ownership is the search's.

### The search

The search (`crates/eadl-resolve/src/search.rs`, leaf `M3.4.2`) chooses, for one system, one provider for each thing
each of its users requires: each service, policy, task and the system itself, one provider per fact it requires. It
looks only among the system's own hardware, in a fixed order — the engine's preferred providers first, then the
description's order — and tries whole assignments one after another until the joint rules accept one. The same
description always gets the same plan. It has a budget: when that runs out first, the answer is
`analysis-inconclusive` and no plan, never "impossible".

When it fails, it says why in the terms of `ROADMAP.md` §5.5. A requirement nothing in the hardware meets is
`missing-fact` when nobody states the fact or states it without a value, `infeasible-configuration` when every one that
states it refuses it or declares it absent, and `unsupported-profile` when a value lies past the exact arithmetic. When
every assignment breaks a joint rule, the conflicts of the first are the reason; when one breaks none but leaves a fact
unknown, that is `missing-fact`. A requirement only the realization can meet — a scheduling policy's `priorities`, a
release's accuracy — finds no provider until the catalog's records become providers (`M3.7`), as the relation's record
says (§9).

Where the description's own blocks and platforms hold no assignment, the search tries again with the engine's
adapters (`M3.2.3.2`): a deadline programmed through a relative timer, a counter extended to 64 bits
([below](#the-searchs-rules)). Only where neither finds one does it refuse, and the refusal says what it looked at — the
description's providers and the engine's adapters — and that the engine knows no other path: **missing engine support,
never a proof that no realization exists**. Where a device states part of what an adapter needs and leaves the rest
undescribed, that is a missing fact, and the message says which. `ROADMAP.md`'s case F10,
`crates/eadl-resolve/tests/search/f10-unsupported-implementation-path.eadl`. And a budget spent says it concluded
nothing — case F11, `crates/eadl-resolve/tests/search/f11-solver-resource-limit.eadl`, where every assignment is
refused and the budget runs out before the search could show it: `analysis-inconclusive`, and no plan. Every plan the
search writes for a tracked description is one the configuration checker accepts, which a test holds over all of them
(`crates/eadl-resolve/tests/plan.rs`).

The search can also say what it did with one requirement (`eadl_resolve::search::explain`, leaf `M3.4.13.1`), the
account `archogen explain` prints. It runs the same search and, for the requirement asked, lists each provider of the
hardware with its answer — meets it, offers a value it refuses, declares it absent, leaves it undescribed. Then it
names the provider the plan binds and, for each other one that meets it, what the joint rules say with that one in
its place — for one the search tried first, the reason it moved on. Or, with no plan, it says why: this requirement's own
refusal, or another's that stopped the search while this one was met. It reads nothing the search does not, so the
account and the plan cannot disagree.

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
a binding takes a channel (rule 2), the channel count and its codes (rule 3), the node that multiplexes (rule 4),
region ownership and its codes (rule 5), and that one assignment's findings decide nothing about the description until
the search has tried every assignment (rule 6); then one system's assignment, its hardware and connectivity (rule 7),
the route obligations (rule 8), privilege over a system's callers, given its level (rule 9), placement, once per
system (rule 10), ordering, unsupported under this profile (rule 11), the initialization graph (rule 12), one
function at one provider, with the qualifiers (rule 13), one time base (rule 14), an adapter's instance judged as one
provider (rule 15), and a sporadic task's own source slow enough to release it on every arrival (rule 16).

### The search's rules

`crates/eadl-resolve/src/search.rs`, over one description's declarations — a module tree's as the language checks it,
each renamed to its instance's path and each name it writes resolved where it was written (reference §6 rules 9 and 10;
`eadl_model::check::resolved_declarations`, leaf `M3.4.10`). A *pair* is one requester's requirements on one
fact, statements apart; its candidates are the system's hardware (rule 7) at which the relation satisfies every one of
them, in the policy's order. Complete assignments are judged in lexicographic order of the pairs' choices, the first
pair varying slowest, and none partially: rule 13 classes a function split as a conflict or an unknown by the
requester's other requirements of the function, so a partial assignment's conflict is no proof. The system's first
placement region (rule 10) is chosen, and its callers bound at the profile's level (rule 9). A pair with no candidate
takes `SR-H18`'s code over the providers that take part — those stating a fact it constrains, a group's sub-facts and a
derivation's inputs among them; when every assignment fails, the first free of conflicts gives its unknowns, else the
first one's conflicts; the verdict is the highest by precedence. A group is never met by different providers, nested
or flat: one side's facts of one function are met at one provider (rule 13), its facts of different functions may be
met apart, and two requesters — a system's own level and the service its task uses — are bound each on its own
(`SR-H17`, leaf `M3.4.9`).

What it chose is written by `crates/eadl-resolve/src/plan.rs` (`M3.4.3`) as `archogen-assignment/2` (`M3.2.5`), the record the
configuration checker judges: the generator's own writer and its own digest of the elaborated program, sharing no code
with the checker's reader, both held to one golden and to each other by `crates/eadl-resolve/tests/plan.rs`. A
requester is a declaration or a task of a system, written `(task SYSTEM TASK)`, so a task never reads as a service.

The generator reads the record back with a reader of its own, `plan::read`, which accepts only the text its writer
writes, and `plan::rebuild` gives the joint rules the record's bindings over the description as the search held them
(`M4.1.3`). A record that binds what the description does not require, leaves a requirement unbound, binds a requester to
an exclusive instance serving another, names an instance no requester of the system is bound to (`M4.1.10`), or binds
a requirement to a provider the relation does not find meeting it (`M4.1.12`) does not rebuild. So the runtime plan is lowered from the record the checker accepted, never from the search's memory
(`docs/decisions/decision_runtime-plan.md` §1). Over every description the search resolves, the record reads back to
the same bytes and its rebuilt bindings give the joint rules the search's own obligations.

The runtime plan is lowered from that record by `crates/archogen-plan/src/lib.rs` (`M4.1.4`), built ahead of its
design's acceptance as the design's executable form (`docs/decisions/decision_executable-design-reviews.md`). A plan is
lowered only on the configuration checker's acceptance: the production entry, `lower_checked`, takes the checker's result
record as well as the assignment, and refuses a result that does not read, that another program wrote, that judged a
build, another assignment, another program or another profile, or that refused what the generator accepted. It reads the
vocabulary and the adapter catalog as this engine's, so no caller can hand it others, and returns beside the plan where
it came from: the generator, the result record's digest, the checker's digest, and whether the command line ran the
checker or the result was handed over as text, which only an experimental build may use. A plan is its canonical text,
one spelling, whose sha256 is its identity (`crates/archogen-plan/src/contents.rs`): what a consumer reads of it, each task's
timing and trigger, each route's interrupt and separation, each node's contexts, is read back from that text, by a reader
that refuses anything the writer would not write, and every plan the lowering writes goes through that reader. Each rule
cites its section of `docs/decisions/decision_runtime-plan.md`; the record's worked texts are held equal to what the
code writes by `crates/archogen-plan/tests/worked.rs`; and every probe system of the design's review that the lowering
decides is a fixture beside the outcome the lowering gives it, in `crates/archogen-plan/tests/corpus/`, a description
the language's check refuses held at that check's verdict; hand-written records hold the refusals of a record, in their
order (`crates/archogen-plan/tests/records.rs`). An outcome is what the code writes, so a reviewer judges each against
the record; a finding is a defect when it carries a reproducer. A bounded exhaustive checker,
`crates/archogen-plan/tests/bounded.rs` (`M4.1.13`), generates universes of small systems — some hundred thousand — has
the configuration checker judge every record the search writes for them, and holds every plan and every refusal to a
reading of the record written by a context that never read the lowering's code. The plan counts the users of a node that
takes a channel — a service, a policy, or hardware with requirements of its own; an instance aside — as the joint rules
do (model §8 rule 4): the system's tasks that reach it, directly or through services, policies and bindings, and the
interrupts whose context holds it, but for its own timer route — at the provider whose channel it takes, or that the
instance whose channel it takes programs — where nothing else that route runs reaches it. [OSEK](ledger.md#osek-os)
counts a shared resource's users by the tasks and interrupt routines that access it (§8.6); the exception for a node's
own route is the record's reading, which OSEK's count does not make. The lowering reads each node's contexts and its
multiplexer from the joint rules (`Joint::contexts`, `M4.1.2.4`), which counted the tasks and services naming a service
by `uses`, over every system of a description, until then.

An adapter's instance is judged by the joint rules as one provider (model §8 rules 14 and 15, `M3.2.3.1`): its offers
computed from the device it programs — a value its kind writes, a fact passed through as the relation reads it at the
device, derived there or offered, a bound it is obliged to at its requester's tightest — and itself a requester of that
device, served when a requester it serves is, a caller at its kind's level, owning nothing and composing nothing, its
device's route carrying its requesters. Rule 14 holds a deadline and every time its requester reaches to one device.
A fact that describes only a counter's compare, how it is wired or how fine its step is, reads none of its time, so
rule 14 reads none of those five.
The search proposes one in a second phase (`M3.2.3.2`): it runs over the hardware alone first, and only when that
concludes with no plan and some instance meets some pair does it run again, each pair's candidates its hardware and then
its instances, under what is left of the budget — so a plan that needs no adapter binds none. An instance tried for a
pair no candidate meets takes part in its code where its kind's `beneath` holds, with its answer, or where its device
states part of that side, with that side's answer there: a device stating none of it decides nothing, which keeps F10
`infeasible-configuration`. Until `archogen-assignment/2` (`M3.2.5`), a plan binding an instance is not written: the
engine API answers `unimplemented`, naming the instance.

An account of a requirement (`explain`, `M3.4.13.1`) is the same run with its pairs selected by fact, then by requester
or task: each provider of the hardware in the policy's order with its answer to the pair — satisfied when it meets
every requirement, otherwise the first of refused, absent, unsupported, unknown and undescribed, the ranking a failed
pair's code is read by, one function for both; the pair's `SR-H18` diagnostic when it has no candidate; and with a
plan the provider bound and, for each other candidate, the joint rules' first conflict or unknown with it in the bound
one's place, every other binding unchanged, or none where they accept that assignment too. A candidate ordered before
the bound one makes an assignment the walk judged and refused; one after it, an assignment the walk never reached. A
search stopped before its pairs — a refusal of what the description writes, or of the system's map — judges none.

### What is not claimed

The relation is a predicate on one requirement and one provider. Per fact its direction induces a preorder and
nothing more: no join, no meet, no best offer, and no lattice (§7).

## Today and ahead

The relation is a library, complete for `M3.1`, and held to the design's executable model
(`crates/archogen-plan-check/src/model/`) over the model checker's whole universe by `crates/eadl-resolve/tests/production.rs`.
Measured `2026-10-06`: 154 381 provider-and-requirement pairs judged alike, 72 234 provider declarations read alike,
and the horizon agreeing with a simulation of the counter's reads on every case of the checker's grid. Each rule
has a catalogued mutation that its test must kill (`xtask/mutations.txt`).

The search chooses over it (`M3.4.2`) and writes the plan the checker reads (`M3.4.3`); `archogen resolve`, which runs
it, is `M3.4.4`'s. Over the tracked examples it
resolves every one of `s0-heartbeat`'s descriptions and refuses each other example: their timers offer a width and no modulus,
from which the vocabulary derives no horizon, and their policies' constraints wait on the catalog — `missing-fact` —
while `alternative-timer`'s absolute deadline, which its timer declares absent, is offered past that absence by the
engine's adapter `deadline-over-delay`. Its timer states the modulus the adapter needs since `2026-10-27` (leaf
`M3.4.7.1`, `docs/specs/adapters/decision_engine-adapters.md` §7, §8), so its refusal is its policy's constraints
alone, the catalog's.

`archogen check` does not call it yet, so no verdict a description gets today comes from it. Leaf `M3.4` wires it in,
over the providers of each system's hardware — a block no system uses decides nothing, as an unused device costs
nothing — deciding a description's `missing-fact` or `infeasible-configuration` from the
enumeration and keeping or migrating every verdict it moves. `M3.2` adds adapters, designed in
`docs/specs/adapters/decision_engine-adapters.md`: providers the engine supplies, each programming one of the description's
devices, bound only where no assignment of the description's own providers holds — `/1`'s are a deadline programmed
through a relative delay and an epoch extender, each plan binding one conditional on the obligations it names, and a
mediating kind, which `/1` has none of, offered only where a requirer allows mediation. A plan's obligations are on
every face that shows it: `archogen resolve` and `archogen explain` print each, and the engine API's `Plan` and the
[MCP](ledger.md#mcp-specification) tools list them (`M3.2.6.1`, [The engine API](engine-api.md)). The search and the configuration checker each hold the
mediation gate both ways, each tested over a test catalog that holds a mediating kind (`M3.2.6.1`'s search half in
`.3.2`, the checker's in `M3.2.6.3`). The integration tier's `resolve-installed` step runs `archogen resolve` from an
install over a plan through each kind, the install's checker accepting it (`M3.2.6.2`,
[Verification](verification.md)). Their catalog is read today,
`docs/semantics/adapters/adapters.eadl` (`M3.2.2`), and held to the checker's own table; the search binds them
(`M3.2.3.2`); `M3.3` judges what one provider cannot: its
channels and owned regions (`M3.3.2`), connectivity and routes (`M3.3.3`) and privilege given a level (`M3.3.4.1`) are
judged today, over an assignment a test proposes, and placement over a system's map (`M3.3.5.1`); ordering is
unsupported under this profile (`M3.3.5.2`), the initialization graph derived (`M3.3.7`), and one function held at
one provider (`M3.3.6`); `M3.7` makes a catalog
record a provider.
