# A system's devices on a target: matched by what is asked of them, never by what they are called

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — written by leaf `M4.4.1`, with the module it describes; held by its tests, and reviewed in `docs/reviews/decision_device-correspondence-reviews.md`, closed on round 9, which found no defect
- **External sources:** none: this record decides how the engine reads its own descriptions
- **Owner / source:** leaf `M4.4.1` (`docs/tasks/M4.md`), for `RTP-H8` in `docs/decisions/decision_runtime-plan.md`; `ROADMAP.md` §3.2, §5.3 and §5.4.

## The fact / decision

**In plain words.** A description is written against a platform of its own, such as `soc.playground` with a `console.uart`. A target is a
real machine with devices of its own, such as `target.console`. To run the system on the target, nothing says the two names are one device,
and nothing has to. The description says what its services and tasks ask for: observable output, a counter whose horizon reaches 60 s. So
the engine gives the system the target's platform in place of its own and searches again. Whatever device is then bound for "`console.write`
needs observable output" is the device `console.uart` was. If no device can serve, the answer is the same refusal any system gets.

**The rule.** Let `S` be a system with a plan `A` over its own hardware, and `T` a target.

1. **`S` on `T` is `S` with `T`'s platform in its `platform` clause.** The hardware the clause's own `uses` names is replaced by that platform, and a
   clause given where `S` wrote none. All else a clause holds stays, a requirement or a service, since it is the system's own side (`docs/semantics/model.md`
   §7 rule 4). The description's hardware is then gone from what is read, and so is another system, but for one it names, which `M1.46` is to refuse; so nothing the target replaces can answer for it, and nothing either says is read for what the report shows, as what a kept declaration says still is, since neither is part of what runs there. The system is read as any system (§8 rule 7): its hardware is what that platform composes. Search, joint rules and checker run
   as on any system, with what `/2` adds to them for a system on a target (below), each requirement judged again at `T`'s providers, and a need on a fact the vocabulary does not declare by presence, as §7 rule 1 has it: the search binds nothing for one (`OnTarget::presence`). Presence there counts what asks: the system, by `uses` or `needs` wherever it writes them, its tasks and its clause among it, what those reach, another system it names at that system's own level, its tasks being its own (model §8 rule 4), and the target's own. What is reached only through hardware the target replaces is not reached there. What answers is the machine: the search binds a block, a platform or an adapter over one and nothing else, so a service's offer answers nothing there, reached or not, and a plan it let come would be for a machine without the device. A reached service that declares a fact absent has still said so: where the machine says nothing of it the refusal is `infeasible-configuration`, and stating the fact at the target passes. What the system does not reach neither asks nor answers, its name still being a declaration's. Nothing here decides what satisfies what.
2. **So everything `archogen-runtime-plan/2` lowers of a device is `T`'s.** A plan `A_T` of `S` on `T` binds only `T`'s devices, and an adapter
   only over them; the lowering reads `A_T` and never reads a device of the description. A description's rate, width, base address and regions do
   not reach a target build, `DC-H3` asking of the description's own map what refuses its own forms, and the analysis reads `T`'s.
3. **A device of the description is the target device bound for the same requirement.** Writing `under(p)` for `p`, or for the device an
   adapter instance `p` programs, device `d` is every `under(A_T(r, f))` where `under(A(r, f)) = d`, `r` a requester and `f` a fact. One device may be
   two, and two one: what they share the joint rules have judged at `T`. `crates/eadl-resolve/src/target.rs`, `correspond`, reads this off the two
   plans, with the hardware no requirement reaches and each requirement only one plan binds.

**What a target's file is** (`Target::read`). It declares hardware alone, each declaration with a name of its own, one platform nothing else in the file composes, and that platform composes
every other declaration, so "the target's hardware" and "every block and platform its file declares" are one set. `targets/riscv-virt-up.eadl`
reads so, unchanged.

**What is refused before any search** (`on_target`, and of a target's own file `Target::read`), each with its repair:

| What | Code | Why |
| --- | --- | --- |
| a name both the description and the target declare | `schema-duplicate-name` | a name means one declaration, and a device taken for another by name is what this rule exists not to do |
| a target's file naming what only the description declares, or a description naming what only the target declares, by `uses`, `needs` or `refines` | `invalid-description`, `infeasible-configuration` | each file says what it holds with no help from the other. Read together, the target's name would find the description's device and bind it, and the description could be read only beside that one target |
| the system, a task, another system it names at that system's own level, or a service or a policy the system reaches on the target naming a block or platform itself, by `uses` or `needs`, anywhere but its `platform` clause's own `uses` | `infeasible-configuration` | no requirement stands for a name, so nothing says which target device it is. The repair is to say what is asked of it |
| the one declaration the system's clause stands on, or one up the chain it refines, declaring a fact absent that the target offers | `refinement-violated` | model §3 rule 3 and `ROADMAP.md` §5.3: an absence is a constraint something relies on, and no requirement carries it. It is read as rule 3 reads an offer |
| two `platform` clauses on the system | `schema-cardinality` | a system has at most one, and read as one they would hide it |
| a target's `uses` or `needs` holding a list, which a system would not read as hardware | `invalid-description` | one set, or every system on it is refused by the search, far from where this is written |
| hardware the system has, or a target's, that refines what leads back to it, through what they compose and refine | `invalid-description` | a refinement says the same thing again and brings no blocks, so no machine stands on a loop, and what a clause stands on could not be read |

A clause stands on one declaration where all the hardware it names is that one, however often it is named, or is of it: what it composes, as model §2 rule 1 reads composition, by `uses` and never `needs`, or what it refines. What it refines says it again and brings no blocks (model §8 rule 7), so what the refined one composes is not of it. In a cycle of composition there is no whole: one stands for it, and each of its absences speaks, so no name moves the verdict. The chain up stops at what is no hardware, of which §3 does not speak. And it stands on that one only where all the hardware the system has is of it: a block it only `needs` is a device beside it, which the system has whichever way the clause is written. Only that one speaks for the machine, since all the system's hardware is what it is made of. An absence covers what it is made of and no more, and another declaration's offer is "two statements of two devices" (model §2 rule 1). So what a composed block or platform declares absent, and one of two that a clause stands on, are not carried, though it happen to be all there is beneath: it spoke of itself and not of the machine, and a requirement that relies on one is judged at whichever device serves it.

**What the description's hardware is, then.** One platform the system can run on, and its proof that the requirements can be met there. A
description need not have a plan of its own to be put on a target, nor pass presence over hardware of its own. `correspond` needs a plan on each side, and no pass of `check`.

**Hand-offs.** Each is carried, word for word, by the leaf it names.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `DC-H1` | `M4.1.7` | A target's file states what systems' requirements read of its devices, a counter's width, modulus, coherence and relative delay among them, each with what checks it, since `targets/riscv-virt-up.eadl` holds nothing its agreement does not check and a system asking them of it is refused `missing-fact` until then. |
| `DC-H2` | `M4.1.7` | The configuration checker puts a system on a target by a reader of its own and judges the plan there, the target in the plan's identity, as it reads a plan today; `crates/eadl-resolve/src/target.rs` is the generator's. |
| `DC-H3` | `M4.1.7` | The command that names a target runs `archogen check`'s judgement on the target's file alone, and every pass on the description itself but what presence says of what is required, and of the search each reading of a description's own forms that can refuse it `invalid-description`, each asked alone, since a search that stops at a first stage hides what a later one would say, and not what a plan's absence says, a map's `missing-fact` among it; its contradictions, two values of one fact, a module tree's refusals, the second value `resolved_declarations` returns, and a declaration named like a fact, which the engine's `prepare` asks of a tree and of a plain description, among them; then presence on what `on_target` gives, `OnTarget::report`, before the search: a description need not pass presence over hardware of its own, and presence judges what the search binds nothing for, of what the system asks. It carries the report's closure and what lies outside it, as model §2 rule 4 has it, and it reports each code `on_target` refuses under the verdict `check` gives that code, `refinement-violated` as `infeasible-configuration`. |
| `DC-H4` | `M4.1.7` | A target with no placement memory is refused, since no playground provides memory there, and what the profile asks of a machine, one active core among it, is judged of any target named, where `cargo xtask target-agreement` holds only the shipped ones. |

## Why

- **Requirements are the contract.** `ROADMAP.md` §5.3 asks that refinement "preserve the obligations actually relied upon", and that "adding an
  unused device is not automatically an invalid refinement". A system relies on what its closure requires, which is exactly what is judged again. What the replaced hardware itself required goes with it, since it said what that machine asks of itself; what the profile asks of any machine is `DC-H4`.
- **It adds no judgement of its own.** Channels, ownership, connectivity, routes, placement, privilege and functions already read one system's hardware
  (model §8), so each holds on a target with no new rule of this record's: `/2` adds the release requester and what an alarm asks of the device bound, which the search and the joint rules then read as any (`decision_release-alarm.md`, `RA-H1`). `§5.4`'s "device connectivity, interrupt routes, privilege, and memory placement" are the
  joint rules' there as here.
- **It answers `M3.3.5`'s R8.** The remark that `periodic-three`'s `device.timer` at `0x1000_0000` is `riscv-virt-up`'s UART held only if a description's
  regions reach a target. They do not: the platform that wrote them is replaced, and it is the target's own map the joint rules judge.

**Alternatives weighed.**

- **By name.** A target's `target.console` and a description's `console.uart` share no name, and two machines' devices should not have to.
- **A map written by hand**, `console.uart` is `target.console`, one per system and target. Each pair would be a claim, and checking it means judging the
  requirements at the target's device, which is this rule; so where facts tell devices apart it adds a file and no knowledge. Where they do not, the
  policy already orders candidates (`Policy::prefer`), by device name: one order for the whole system, so two requesters asking the same shared fact get one device under any order, and what differs is then said as a fact.
- **Device by device, as a refinement of the description's device** (model §3). A value "is kept only by the same value", so a 64-bit target counter would
  not stand for a 32-bit one, though every requirement is better met; `target.timer` offers no `region device.timer`, so every concrete region would
  refuse; and neither a split nor an adapter a better device no longer needs can be said.
- **The target's platform refining the description's**, `(refines soc.playground)` in the target's file. A target's file would name each system that runs on it,
  and §3's rules 1 and 2 fail as above. Only rule 3, absences, says something requirements do not, and is kept.
- **A target's file holding facts for a system's sake.** It holds what its agreement checks (`decision_target-platform-description.md`). What is missing is said
  as `missing-fact`, and added with its check: `DC-H1`.

## How to apply

- A `/2` plan names only a target's devices; a line naming a description's device, were one wanted, is a report made by `correspond`.
- ⚠️ **Two devices no requirement tells apart are not told apart.** With two UARTs on a target, "which is the console" is the policy's order, then the search's stable
  one, and that order is one for the whole system, so it cannot give two requesters two equal UARTs. A designer who cares says what differs. This is the limit of matching by what is asked, stated rather than worked around.
- ⚠️ **Silence is not an offer.** A target's file holds what its agreement checks, so a fact it does not state is not offered: a system declaring `interrupt-source` absent lands on `riscv-virt-up`, whose controller is present and unstated (`decision_target-platform-description.md` §2). That is how model §3 rule 3 reads an absence, by the fact's name, and `DC-H1` is how a fact comes to be stated. So a fact stated with any value counts as offered, and a derived fact's absence meets only its own name, `(absent unambiguous-horizon)` landing on a target that offers its inputs.
- ⚠️ **Older than this, and said.** A need on a fact the search derives, `(needs unambiguous-horizon)` itself, is `missing-fact` from presence though the search binds it: a service's need always was, and since this record's round 3 a system's is too. No corpus file has the shape. Every `uses` is a request and a `needs` an edge, so a composed device's need is inside the closure, used or not, which `ROADMAP.md` §5.3's "unused device" may want narrowed. On a description's own hardware a service's offer still answers presence, as on a target it no longer does, and `(f false)` answers a need outside the vocabulary. Each is `M3.4.17`'s.
- ⚠️ **What is claimed is what is judged.** Two plans agreeing on requirements says nothing of a run on one matching a run on the other: that comparison is `M4.9`'s.
- A service naming a device is refused here with a repair that works on a target, before the search's own refusal (`M1.49`), whose repair, to compose the device into the platform, does nothing where that platform is replaced.
- Nothing yet names a target on the command line, and `/2` does not exist: `M4.1.7` owns the input and the lowering, `M4.1.6` which device's alarm releases tasks.
- Today, of the five `examples/`, only `s0-heartbeat` has a plan of its own and on a target: the rest wait on `M3.7`, or in `bounded-queue`'s case on a later profile, each refused `missing-fact` by the search on either. `tests/target.rs`
  therefore holds the rule's other cases on fixtures written there, a target stating what `riscv-virt-up`'s file does not yet.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 14 | 5 (DC1-1 to DC1-5), beside 7 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: only the hardware a clause names is replaced, and all else it holds is judged on the target; each file stands alone, in both directions; only the platform all the hardware is made of speaks for the machine; a target's file holds one declaration per name |
| 2 | 10 | 3 (DC2-1 to DC2-3), beside 3 gaps and 4 of wording | `2026-11-06`, not acceptable — answered: the description's hardware is gone from what a system reads on a target, so nothing replaced can answer for it, and presence judges there what the search binds nothing for; `DC-H3` asks of what `on_target` gives; a clause stands on what it means, however it is written |
| 3 | 9 | 6 (DC3-1 to DC3-6), beside 2 gaps and 1 of wording | `2026-11-06`, not acceptable — answered: what a system needs is requested wherever it writes it, at the root, in every description; presence on a target counts what asks, so an unreached service says what it offers and nothing else; two clauses are refused, and `DC-H3` asks of the description itself what it no longer shows on the target; one reading of composition, and a cycle named whole is one machine; presence says what `check` says of what the profile excludes |
| 4 | 6 | 2 (DC4-1, DC4-2), beside 3 gaps and 1 of wording | `2026-11-06`, not acceptable — answered: on a target what answers is the machine, so a service's offer answers nothing there, reached or not; a clause stands on one declaration only where all the hardware the system has is of it, and what it refines brings no blocks; each of a cycle's absences speaks, so no name moves the verdict |
| 5 | 7 | 5 (DC5-1 to DC5-5), beside 1 gap and 1 of wording | `2026-11-06`, not acceptable, each low in reach and none yielding a wrong device — answered by a sweep beside the shapes: every system in the repository's trees is put on a target by a test, what comes binding only that target's devices and presence refusing nothing the search resolves. `DC-H3` keeps a contradiction, and carries the report's metadata; a name of the description is a declaration's; hardware refining its own parts is refused; a false sentence on the closure is gone |
| 6 | 9 | 4 (DC6-1 to DC6-4), beside 3 gaps and 2 of wording | `2026-11-06`, not acceptable, each low in reach and none binding a wrong device — answered: another system named asks, and what is reached only through what the target replaces is not reached; `DC-H3` names what a tree's own refusal asks and what `M3.4.7` will add; a loop through refinements is refused, and only a loop; what answers nothing stays visible; the sweep holds what the harm was, a plan with nothing on the machine to answer it, over modules and two machines |
| 7 | 10 | 2 (DC7-1, DC7-2), beside 5 gaps and 3 of wording | `2026-11-06`, not acceptable, each low in reach and none binding a wrong device — answered: another system named asks at its own level, its tasks being its own; `DC-H3` keeps what refuses a description's own forms wherever it is said, and says which verdict a refusal reports; a target's own loop is refused; the sweep holds what round 7's reader's held |
| 8 | 9 | 1 (DC8-1), beside 5 gaps and 3 of wording | `2026-11-06`, not acceptable, low in reach and binding no wrong device — answered: a replaced name answers another system named no more than it answers this one; `DC-H3` asks each reading alone, a stopped search hiding what a later one would say; a target's `uses` holding a list is refused; the sweep holds the partition, what is gone and the real target |
| 9 | 6 | 0, beside 3 gaps and 3 of wording | `2026-11-06`, **acceptable: the review closes** — 244 systems on 9 targets, 30,000 generated pairs and two metamorphic relations of its reader's own, a target that is a system's own hardware renamed and a description with all its hardware removed, each without a difference; the gaps and wording answered as it closed |
