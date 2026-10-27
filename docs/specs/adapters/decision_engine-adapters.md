# Engine adapters: indirect realization

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — written by leaf `M3.2.1` (`docs/tasks/M3.md`); its review, closed in round 3, below
- **External sources:** `ROADMAP.md` §5.2 ("An adapter that rescales, extends epochs, or translates deadlines is a real
  provider with costs and proof obligations"); §5.4 ("Engine search may realize an OS feature indirectly from
  lower-level hardware functionality. A software deadline service can use a supported relative timer through an
  engine-owned adapter, provided the resulting behavior and bounds satisfy the eADL requirement. Direct hardware offer
  matching is only one realization path. If no known realization exists, explain the missing engine capability"); §7.3
  (every interrupt source's "maximum service cost" recorded); the Linux kernel's timekeeping, as precedent:
  [`linux-timekeeping`](../../book/src/ledger.md#linux-timekeeping) and
  [`linux-sched-clock`](../../book/src/ledger.md#linux-sched-clock)
- **Owner / source:** `M3.2`'s goal and acceptance, and the hand-offs it carries — `SR-H8`, `SR-H9`, `SR-H10` of
  `docs/decisions/decision_substitutability-relation.md`; the notes `M3.3.4`, `M3.3.6`, `M3.3.7` and `M3.5` wrote
  into its leaf, the last: "an adapter enters [the assignment] as a provider kind of its own, named by its engine
  identity, with the engine knowledge the configuration checker needs to judge it declared as that checker's data"

## The fact / decision

An **adapter** is a provider the engine supplies, never one a description writes: a piece of the generated system
that programs one of the description's devices and offers, over it, facts the device does not offer itself. Only the
search offers one, where no assignment of the description's own providers holds.

### 1. Its contract is data, read twice

1. **One file holds every adapter the engine knows**: `docs/semantics/adapters/adapters.eadl`, one `defadapter` form
   per **kind**, written with the kind `docs/semantics/kinds/defadapter.eadl`, as the vocabulary is written with
   `deffact.eadl`. It is engine knowledge: no description writes, imports or names a kind. `archogen check` answers the
   file as a language definition, exit `20`, as it answers `vocabulary.eadl` (`SR-H3`); its verdict is frozen in
   `crates/archogen-cli/tests/verdicts.txt`, its folder is a row of `docs/semantics/conformance.md` beside the
   vocabulary's, and `docs/semantics/BASELINE.txt` freezes its canonical form, so a kind changes only with a migration
   note. Its version is `adapters/1`, registered in `docs/book/src/versions.md` as `adapter-catalog`.
2. **The generator reads the file** (`eadl_resolve::adapter`), and **the configuration checker holds its own table**
   (`archogen_plan_check::adapters`), written from this record under `M3.5`'s fence, never from the generator's code. A
   derivation test holds the two equal kind by kind, as `tests/vocabulary.rs` holds the vocabulary: a fault in reading
   the file cannot hide behind the table, which does not read it. Presence's reading (§7) is a third, of names alone,
   held to the generator's by a test of its own.
3. **A kind declares**, each clause once:
   - `(doc "…")`;
   - `(beneath …)` — what it requires of the one provider it programs, a side the relation reads as any requirer's
     (relation record §1, §3): its `needs` and its constraints;
   - `(offers …)` — what an instance offers, each fact once, each item one of: a value, `(f v)`; the provider's own
     value of another fact of the same domain, `(f (from g))`, which may be one the vocabulary derives there; or a
     bound the instance takes on, `(f (obliged))` (item 5);
   - `(level L)` — the level of `reachable-at-privilege`'s domain its code runs at, rule 9's (§4);
   - `(obligations …)` — what the realization owes, each `(obligation NAME (doc "…"))`;
   - `(costs …)` — the interrupts its code takes, each `(interrupt NAME (doc "…"))`, for the analysis (§4);
   - `(mediates true)` or `false`, absent meaning `false` (`SR-H8`, §6).
4. **Exclusive and shared.** A kind whose offers include `channels` is **exclusive**: each instance serves the one
   requester bound to it, takes its own channel beneath, and offers one. Any other kind is **shared**: one instance per
   provider, its offers read by every requester bound to it, as a shared fact is (model §8 rule 2).
5. **An obliged offer** is a bound the adapter's own code decides — the time its handler takes — which no fact of the
   description gives and no M3 rule computes. Only an exclusive kind may oblige, and only a fact the vocabulary gives a
   `quantity` domain and an `at-most` or `at-least` direction. An instance offers it at the tightest bound its requester
   writes on that fact, over every requirement it writes there, a group's sub-fact and a flat clause alike: the value
   of an `exactly` bound, which the side reader already refuses beside a bound it contradicts; else the least of its
   `at-most` bounds, or the greatest of its `at-least` ones. So the relation reads an ordinary value that meets each of
   them, no outcome is added to model §7 rule 5, and
   the instance carries the obligation to meet that bound, for the realization to meet and the analysis and
   verification to discharge (§9). A requirement writing no bound on it leaves the fact offered bare. So a plan
   binding an adapter is conditional on its obligations, and each face that shows the plan says so (§9); a bound no
   realization can meet is refused where the obligation is discharged, never assumed met.
6. **What the reader refuses**, each `invalid-description` at its form: a kind named twice; a clause twice or unknown;
   a fact offered twice, or not of the vocabulary; `(from g)` across two domains or dimensions; an obliged fact that is
   not a bound, or obliged by a shared kind; a qualifier — a fact the vocabulary marks `(function each)` — offered
   otherwise than passed through from itself, since a kind's level is its `level` clause's; an exclusive kind whose
   `channels` is not `1 tick`, or whose `beneath` takes no channel; a `beneath` the relation's side reader refuses;
   every refusal the relation's offer reader makes of a provider's offers (model §7 rule 3), applied to a kind's
   literal offers — a derived fact beside its inputs, a modulus past two to its width or beside `saturating`, a bound
   beside a value among them; a level outside `reachable-at-privilege`'s domain; a `mediates` other than `true` or
   `false`; an offer of a statement fact.

### 2. `/1`'s catalog

Two kinds, the epoch extender and the deadline translator §5.2 names, each programming one counter device — model §8
rule 13's one provider of its function:

| Kind | Instances | Beneath | Offers | Level, obligations and costs |
| --- | --- | --- | --- | --- |
| `deadline-over-delay` | exclusive | `(needs relative-delay counter-modulus tick-rate observation-coherent)` | `(absolute-deadline true)`, `(supported-horizon (from unambiguous-horizon))`, `(delivery-bound (obliged))`, `(channels 1 tick)`, `(available-in-state (from available-in-state))` | `machine`; `delivery`: an expiry is delivered within the bound the instance offers; `chain`: a deadline farther than one delay reaches is reached by re-arming, each delay computed from a coherent read of the time base its requester reads; cost `expiry`: one interrupt at its provider per deadline delivered and per re-arm |
| `epoch-extender` | shared | `(needs relative-delay counter-modulus tick-rate observation-coherent)` `(wrap-behavior modular)` | `(counter-modulus (pow2 64))`, `(counter-width 64 bit)`, `(tick-rate (from tick-rate))`, `(tick-unit (from tick-unit))`, `(wrap-behavior modular)`, `(observation-coherent true)`, `(available-in-state (from available-in-state))` | `machine`; `update`: the epoch is advanced by a wake programmed within the provider's unambiguous horizon; `coherent`: a read joins epoch and counter so that no update is torn; cost `wake`: one interrupt at its provider per update period, the period the realization states and no longer than that horizon |

- **`deadline-over-delay`** is §5.4's own example. To program an absolute time `T` it reads the time `now` and programs
  the relative delay `T − now`, as a clock event "take[s] a desired time specification value and calculate[s] the
  values to poke into hardware timer registers" ([`linux-timekeeping`](../../book/src/ledger.md#linux-timekeeping)). How
  far ahead it can be programmed is how far its time base disambiguates — the provider's `unambiguous-horizon`, derived
  there from its modulus and rate (relation record §4) — so `supported-horizon` passes it through; a deadline past one
  delay's reach is chained. A requester's `delivery-bound` becomes the instance's obligation.
- **`epoch-extender`** is the relation record §4's: "a horizon that long over that hardware is an epoch extender's, a
  provider in its own right offering the horizon alone, or, where a service needs the counter's other facts of one
  provider (`SR-H14`), its extended modulus and rate beside them", and `M3.3.6`'s "offers its extended width beside its
  modulus and rate". It extends the counter to `(pow2 64)` counts by an epoch advanced from a wake before each wrap,
  as the kernel's generic `sched_clock` extends "low level hardware time counters to full 64-bit ns values" with a
  timer restarted every `wrap_kt` and a latched read ([`linux-sched-clock`](../../book/src/ledger.md#linux-sched-clock)).
  It passes `tick-rate` and `tick-unit` through — the counter's facts a service reads beside its time, which `/1`'s
  vocabulary gives the function `counter` and the kind does not replace — and the qualifier `available-in-state` — so a service needing them
  of one provider (rule 13) finds them at the instance; `unambiguous-horizon` is derived at the instance from its
  modulus and rate, never offered. A counter fact the kind neither offers nor passes through is undescribed at the
  instance, as at any provider that does not state it. Its wake takes a
  channel of `relative-delay` beneath.
- **Not in `/1`:**
  - *A multiplexer.* F08's fixture — two requests on one comparator of one channel — must stay
    `infeasible-configuration`, and an engine that multiplexed of its own accord would never refuse it; model §8 rule
    4's "multiplexing adapter" waits for a catalog that brings the overhead evidence such a kind must state.
  - *A mediator.* Under `rt-static-up-v1` every caller is bound at `machine` (rule 9), so no boundary stands between a
    caller and a device to mediate across. The clause and its gate (§6) exist for a later profile's catalog.
  - *A rescaler.* No description or fixture asks a unit its counter does not report, and the vocabulary has no fact
    for the resolution a rescaled counter keeps, so whether `ns` reported at a `us` resolution meets a requirement for
    `ns` cannot be judged (relation record §6's `tick-unit` row: "rescaling is an adapter's"). A catalog adds one with
    that fact.
  - *A comparator's horizon extender.* F10's comparator is programmed `1 s` ahead and its service asks `10 s`; chaining
    comparator deadlines needs a time base the comparator does not state. So F10 stays a missing engine path.

### 3. The search binds one

1. **An instance meets a pair** where, for a kind in the catalog's order that §6's gate admits and a provider of the
   system's hardware, in the policy's order, at which the kind's `beneath` is satisfied (relation record §3), the
   pair's fact has a value at the instance — offered, or derived from its offers — and the instance meets every
   requirement of the pair. An exclusive instance is the requester's own, `(adapter KIND PROVIDER REQUESTER)`; a shared
   one is `(adapter KIND PROVIDER)`. Its obliged facts take its requester's tightest bounds (§1.5).
2. **Direct realization first, in two phases.** The search runs as `M3.4.2` built it, over the hardware alone; only
   when that concludes with no plan — a pair no provider meets, or every assignment refused — and some instance meets
   some pair does it run again, each pair's candidates its hardware in the policy's order and then its instances,
   under what is left of the budget, its conclusion standing. One walk over both would not keep the order: it varies
   the last pair fastest, so an instance for a late pair would be tried before a direct assignment that changes an
   early one. So an adapter is bound only where no assignment of the description's own providers holds, and a plan
   that needs none binds none.
3. **Its own bindings.** An instance is a requester for its `beneath`, bound to its provider — fixed by the candidate,
   never searched — and the walk judges an assignment's instance bindings with the rest. An instance programs a
   provider of the description's hardware, never another instance: `/1` does not stack adapters.
4. **What a failed search says** (`SR-H18`). An instance tried for a pair takes part in its code beside the hardware's
   providers on `SR-H18`'s own terms — a silent one deciding nothing: one whose `beneath` holds answers the pair; one
   whose `beneath` its provider does not satisfy takes part only where its provider states — offers or declares
   absent — some fact that side names, and answers with the side's answer there, ranked as rule 5 ranks a group's. So
   where a provider states part of what a kind needs and leaves the rest undescribed, describing it makes a
   `missing-fact`; a provider stating none of it, F10's comparator among them, takes no part, and F10 keeps
   `infeasible-configuration`. The message says the search looked at the description's blocks
   and platforms and at the engine's adapters, naming each kind tried and what stopped it, and that the engine knows
   no other path (F10, `M3.4.5`): missing engine support, never impossibility.
5. **`explain`** lists each instance it judged beside the hardware's providers, with its answer and, bound, its
   obligations.

### 4. The joint rules judge one

Model §8, for the generator (`eadl_resolve::joint`) and the checker (its check (5)) alike, each rule amended to say so:

- **Rule 1**: an instance is a provider for its offers and a requester for its `beneath`.
- **Rules 2 and 3**: its `beneath` binding on a consumed fact at `true` takes a channel at its provider; an exclusive
  instance offers its one channel.
- **Rule 5**: it owns no region, so ownership never compares it, and it composes nothing — `M3.3.6`'s note in
  `M3.2`'s leaf, "one that programs its device directly composes it, so rule 5 exempts the pair", needs no exemption
  where nothing is owned. So **rule 12** keeps its edges: its provider precedes it, and it precedes each requester bound to
  it.
- **Rule 7**: it takes the system of the requesters bound to it; its provider must be of that system's hardware.
- **Rule 8**: a binding to an instance makes no route; its own binding beneath makes its provider's timer route, which
  carries the requesters bound to the instance.
- **Rule 9**, amended: an instance is a **caller** at its kind's level — beside the system's own level, its tasks, and
  its services and policies — so the provider it binds must be reachable at that level; and it states that level as a
  block states `reachable-at-privilege`, for the callers bound to it.
- **Rule 13**: it is one provider of its function, as the rule's note for `M3.2` says.
- **Rule 14, new: one time base** (the `M3.3.6` note: "a deadline service's alarm in the time base of the counter
  `time.monotonic` reads, agreement across requesters, is this leaf's to keep"). Where a requester is bound for
  `absolute-deadline`, every requester it reaches through `needs` and `uses`, itself included, that is bound for a fact
  of the function `counter` that is not consumed is bound at the same device — a provider being its own device, an
  instance's its provider's. Two devices are a conflict naming both requesters and both devices: a deadline programmed
  in one counter's time and read in another's. It judges direct bindings too, and `.3` measures what it moves. It is
  deliberately wider than the note: `/1`'s vocabulary cannot tell a time base from another use of a counter, so any
  counter a deadline's requester reaches may be the one its absolute times come from; a description wanting a second
  counter for another purpose writes it as a requester the deadline's does not reach.
- **Obligations**: each instance bound adds its kind's obligations and cost interrupts to the assignment's, once per
  instance, reported beside rules 4, 8 and 10's, never refused.

### 5. The plan names one: `archogen-assignment/2`

The format register makes a provider or requester kind a new version. `/2` is `/1` with two additions: a clause
`(adapters adapters/1)` after `(program …)`, naming the catalog the plan was resolved with, which the checker refuses
when its table is another's; and, wherever `/1` writes a requester or a provider, an instance, `(adapter KIND PROVIDER)`
or `(adapter KIND PROVIDER REQUESTER)` — a form, as `(task SYSTEM TASK)` is, so no description's name reads as one.
Bindings sort as `/1`'s, named requesters, then tasks, then instances by kind, provider and requester. Every plan is
written `/2` from the commit that builds it, one format at a time. The checker reads `/2` alone, refuses a kind its
table does not hold, and judges each instance's `beneath` bindings, its offers and its joint rules by its table — its
half written under `M3.5`'s fence by a reader of this record who has not read the generator's code. `/1`'s golden stays
frozen; `/2` has its own. Obligations are not written: both sides derive them from the bindings and the catalog.

### 6. `SR-H8`: mediation

A kind writing `(mediates true)` is offered only to a requester that wrote `(or-through-mediation allowed)`, bare or
under `exactly` — the relation's one statement, read as its record's §3 rule 6 says. The search and the checker both
hold the gate. `/1`'s catalog holds no mediating kind (§2); a test catalog holds one, which owns no region, and holds
the gate both ways. Whether a mediating kind owns its device without composing it — `M3.3.6`'s note, F09's shape — is
decided by the catalog that first ships one.

### 7. `SR-H9` and presence

`SR-H9`: "An adapter's offer satisfies past a declared absence, model §2 rule 2 amended to say so, and the absence
across providers is judged among the description's blocks and platforms, the adapter not one of them." Presence runs in
`archogen check` before any search, so it must not refuse what an adapter may realize, and must not stop refusing what
none can:

1. **What a kind adds**: each fact its offers name that is not passed through from itself, `(f (from f))`, and that
   its `beneath` does not name — a qualifier never among them, since the reader admits one only passed through from
   itself (§1.6). In `/1`: `deadline-over-delay` adds `absolute-deadline`, `supported-horizon` — passed through from another
   fact — and `delivery-bound` and `channels`; `epoch-extender` adds `counter-width`.
2. **A path, by name**, for a kind: one block or platform of the description — never a service, which no instance
   programs — that offers every fact the kind's `beneath` names, as `needs` or as a constraint's head, bare or valued.
   **A partial path**: one that offers some of them and leaves each other undescribed there, never declaring one absent.
3. **Rules 2 and 3**, for a required fact some kind adds that rule 2 or 3 would refuse — declared absent and offered
   by none, or described by none; a fact some declaration offers is untouched: where a kind adding it has a path, the
   fact is the search's to bind or refuse, and presence refuses nothing for it; otherwise, where one has a partial path,
   it is `missing-fact` — describing the facts that kind needs at that declaration would make a path by name, which the
   search may still refuse by value (item 5) — its hint naming the kind, the declaration and each fact to describe; otherwise rule 2 or 3 reads as before, its hint naming each kind that adds
   the fact and the facts its `beneath` needs. A fact no kind adds is judged as before.
4. Presence reads names alone — `eadl_model` embeds the file and reads each kind's offers and `beneath` heads; it
   needs no vocabulary — so the language check knows an engine path may exist, never whether it does. `eadl_model` is
   compiled by both roles, so the embedded file joins the items they share, which the trust inventory lists and
   `M3.6.5` accepts.
5. **Interim, named and handed on:** until `M3.4.7` makes `archogen check` adopt the search's verdict, a description
   whose path exists by name and fails by value, or lies at a block or platform outside the system's hardware, is
   accepted by `archogen check` and refused by `archogen resolve` — the divergence
   `docs/decisions/decision_absence-per-provider.md` rule 8 names, written into `M3.4.7`'s leaf with this record.
6. A declaration's own absence still contradicts its own offer (model §2 rule 1); an instance is no declaration.

F04's case keeps `11` because no kind adds `wrap-behavior`: the extender names it beneath. F06's keeps `10`, rule 1
untouched. `examples/alternative-timer` moves from `13` to `11`: `timer.delay` offers `relative-delay`, `tick-rate` and
`observation-coherent` and leaves `counter-modulus` undescribed, a partial path for `deadline-over-delay` — describing
the modulus is exactly its repair — and the search says the same where presence lets it run (§3.4). `.4` measures every
tracked description, names each verdict that moves in a migration note, and moves `examples/README.md`'s verdict lines.

### 8. `SR-H10`: `uc3`

`examples/alternative-timer`'s `time.deadline` is met by `deadline-over-delay` over `timer.delay`, which offers
`relative-delay`, once `timer.delay` states its modulus: the kind needs `counter-modulus` beneath, and its
`supported-horizon` is `timer.delay`'s unambiguous horizon, derived from that modulus — `(2^32 − 1) / 1 MHz`,
`4294.967295 s`, past `10 s`. The description states `counter-width 32 bit` and no modulus, so it must gain
`(counter-modulus 4294967296)`: `M3.4.7`'s migration, `counter-modulus` beside `counter-width`, which changes the
description and so goes to the director before it is made (`SR-H10`; relation record §10). `M3.2` makes no change to
it. Its policy's constraints wait on a catalog record (`M3.7`). So `uc3` resolves once `M3.2`, that migration and
`M3.7` have landed, and `examples/README.md`'s seal names the three in `.6`'s commit. `SR-H10`'s last step —
`uc3` building as the README seals — is written into `M3.4.7`'s leaf with this record, with the condition that the
description's change goes to the director first: `M3.2` closes having built the path, and the leaf that migrates the
description finishes it.

### 9. What the plan owes, and who discharges it

Each face that shows a plan binding an instance says it is conditional, naming each obligation and cost: `archogen
resolve` and `archogen explain` print them; the engine API's `Plan` gains its obligations, a field, so a minor; the
[MCP](../../book/src/ledger.md#mcp-specification)
tools give them beside the plan. `M4.1` carries them into the runtime plan — each cost interrupt a source the analysis
charges, as §7.3 asks of every interrupt source — and `M4.8`'s report lists each undischarged obligation, the property
resting on it `not-established` until it is discharged. Both are written into those leaves with this record.

### 10. Children

Each child amends the model, the records and the book where its slice changes normative text, in its own commit, and
is implemented with focused checks passing; full verification is `M3.2`'s last.

- `.2` the catalog file and its kind, its reader and the checker's table held equal, its verdict, baseline and
  conformance row, the register entry, presence's reading held to the reader's;
- `.3` the search's two phases and the generator's joint rules over instances — rules 1–3, 5, 7–9, 12–14 as §4 amends
  them — `explain` with instances, `SR-H18`'s code with them and F10's message; two positive fixtures,
  `crates/eadl-resolve/tests/search/adapter-deadline-over-delay.eadl` and `adapter-epoch-extender.eadl`, and two
  alarms over one timer — of one channel refused, of two resolved with an instance each;
- `.4` presence's rules 2 and 3, the model's §2, and the verdicts they move, measured;
- `.5` `archogen-assignment/2`, the generator's writer and the checker's reader and check (5) — the checker's half by
  a fenced reader — its golden, each positive fixture's plan frozen and accepted by the checker, and `M4.1`'s leaf,
  which carries the plan, moved to `/2`;
- `.6` the faces of §9 and the API's minor, `archogen resolve` end to end over a positive fixture checked by an
  install's checker, `SR-H8`'s test catalog, the README's seal, the matching chapter.

## Why

- **§5.2 and §5.4 ask for it, and bound it.** An adapter is a realization path besides direct matching and must
  satisfy the requirement's behaviour and bounds: its offers are judged by the same relation, and the part no M3 fact
  decides — its own code's time — is an obligation the plan carries rather than a value it invents.
- **Data, read twice, keeps two roles honest.** The checker judges an adapter from its own table, held equal to the
  generator's reading of one file, as the vocabulary already is.
- **The catalog is the smallest that meets the records.** Its two kinds are §5.4's example and the relation record's
  epoch extender; each kind left out has a reason a fixture or the vocabulary gives.
- **Presence yields only where a path may exist,** so a mandatory refusal — F04's, F06's, `uc3`'s while unmigrated —
  never turns into an acceptance on the strength of an adapter that cannot be bound.
- **Prior art agrees on the mechanisms.** A production kernel programs absolute events through relative hardware and
  extends a narrow counter by an epoch advanced before each wrap and read under a latch; the obligations name what that
  code must get right, and the costs what it charges.

## How to apply

- `docs/semantics/adapters/adapters.eadl` and `docs/semantics/kinds/defadapter.eadl`; `crates/eadl-resolve/src/adapter.rs`;
  `crates/archogen-plan-check/src/adapters.rs` (fenced); a derivation test in `crates/eadl-resolve/tests/`; the trust
  inventory's admission of each new embedding of the file, and its row among the items both roles share.
- `crates/eadl-resolve/src/search.rs` and `joint.rs`; the checker's `joint.rs` (fenced); `crates/eadl-model/src/presence.rs`.
- `docs/semantics/model.md` §2 rules 2 and 3, §8 rules 1–3, 5, 7–9, 12–13 and the new rule 14; the relation record's
  §10, dated notes for `SR-H8`, `SR-H9` and `SR-H10`; `docs/book/src/versions.md`'s `assignment-format`, `/2`, and its
  `adapter-catalog` entry; the book's matching chapter; `examples/README.md`'s seal.
- Written into other leaves with this record: `M3.4.7` (§7.5), `M4.1` and `M4.8` (§9).
- Related: [[decision_substitutability-relation]], [[decision_absence-per-provider]], [[decision_engine-api]].

## Review

| Round | Date | Findings | Verdict |
| --- | --- | --- | --- |
| 1 | `2026-10-27` | 25: A1-1 to A1-7 defects; A1-8 to A1-19 gaps; A1-20 to A1-25 wording | not acceptable — answered: presence yields only for what a kind adds and only where a path exists by name; a kind meets a fact it derives; the extender passes the counter's facts through, at `(pow2 64)`; instances compose nothing, exclusive ones per requester; rule 14, one time base; obliged bounds instantiated as values; costs, faces and discharge named and handed on; the file's standing decided; `uc3` needs a modulus, the director's |
| 2 | `2026-10-27` | 12: A2-1, A2-2 defects; A2-3 to A2-7 gaps; A2-8 to A2-12 wording | not acceptable — answered: an instance takes part in a code only where its provider states part of its `beneath`; a partial path is presence's `missing-fact`, `uc3` `13` → `11`; obliged values the tightest bound; `SR-H10`'s last step and the divergence by place written into `M3.4.7`; the reader's refusals completed |
| 3 | `2026-10-27` | 4: A3-1 a gap; A3-2 to A3-4 wording | acceptable — no defect; answered: an `exactly` bound fixes an obliged value; rules 2 and 3's partial path scoped to a fact they would refuse; the qualifier named as one; the hint promising a path by name, not a plan — the review closed |

The rounds, each finding and its answer, are in
[`docs/reviews/decision_engine-adapters-reviews.md`](../../reviews/decision_engine-adapters-reviews.md).
