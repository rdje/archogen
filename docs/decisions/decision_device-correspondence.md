# A system's devices on a target: matched by what is asked of them, never by what they are called

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — written by leaf `M4.4.1`, with the module it describes; held by its tests, and under review in `docs/reviews/decision_device-correspondence-reviews.md`
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
   §7 rule 4). The system is then read as any system (§8 rule 7): its hardware is what that platform composes. Search, joint rules and checker run
   unchanged, each requirement judged again at `T`'s providers. Nothing here decides what satisfies what.
2. **So everything `archogen-runtime-plan/2` lowers of a device is `T`'s.** A plan `A_T` of `S` on `T` binds only `T`'s devices, and an adapter
   only over them; the lowering reads `A_T` and never reads a device of the description. A description's rate, width, base address and regions do
   not reach a target build, and the analysis reads `T`'s.
3. **A device of the description is the target device bound for the same requirement.** Writing `under(p)` for `p`, or for the device an
   adapter instance `p` programs, device `d` is every `under(A_T(r, f))` where `under(A(r, f)) = d`, `r` a requester and `f` a fact. One device may be
   two, and two one: what they share the joint rules have judged at `T`. `crates/eadl-resolve/src/target.rs`, `correspond`, reads this off the two
   plans, with the hardware no requirement reaches and each requirement only one plan binds.

**What a target's file is** (`Target::read`). It declares hardware alone, each declaration with a name of its own, one platform nothing else in the file names, and that platform composes
every other declaration, so "the target's hardware" and "every block and platform its file declares" are one set. `targets/riscv-virt-up.eadl`
reads so, unchanged.

**What is refused before any search** (`on_target`), each with its repair:

| What | Code | Why |
| --- | --- | --- |
| a name both the description and the target declare | `schema-duplicate-name` | a name means one declaration, and a device taken for another by name is what this rule exists not to do |
| a target's file naming what only the description declares, or a description naming what only the target declares, by `uses`, `needs` or `refines` | `invalid-description`, `infeasible-configuration` | each file says what it holds with no help from the other. Read together, the target's name would find the description's device and bind it, and the description could be read only beside that one target |
| the system, a task, or a service or a policy the system reaches on the target naming a block or platform itself, by `uses` or `needs`, anywhere but its `platform` clause's own `uses` | `infeasible-configuration` | no requirement stands for a name, so nothing says which target device it is. The repair is to say what is asked of it |
| the one platform the system's clause names, or one up the chain it refines, declaring a fact absent that the target offers | `refinement-violated` | model §3 rule 3 and `ROADMAP.md` §5.3: an absence is a constraint something relies on, and no requirement carries it. It is read as rule 3 reads an offer |

Only that one platform speaks for the machine, since all the system's hardware is what it is made of. An absence covers what it is made of and no more, and another declaration's offer is "two statements of two devices" (model §2 rule 1). So a block's own absence, a composed platform's and that of one of two platforms a clause names are not carried: another device may offer the fact, and a requirement that relies on one is judged at whichever device serves it.

**What the description's hardware is, then.** One platform the system can run on, and its proof that the requirements can be met there. A
description need not have a plan of its own to be put on a target, and `correspond` needs both.

**Hand-offs.** Each is carried, word for word, by the leaf it names.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `DC-H1` | `M4.1.7` | A target's file states what systems' requirements read of its devices, a counter's width, modulus, coherence and relative delay among them, each with what checks it, since `targets/riscv-virt-up.eadl` holds nothing its agreement does not check and a system asking them of it is refused `missing-fact` until then. |
| `DC-H2` | `M4.1.7` | The configuration checker puts a system on a target by a reader of its own and judges the plan there, the target in the plan's identity, as it reads a plan today; `crates/eadl-resolve/src/target.rs` is the generator's. |
| `DC-H3` | `M4.1.7` | The command that names a target runs `archogen check`'s judgement on the description and on the target's file each alone, before they are read together, since each must stand alone and read together a schema error of one can be hidden by the other. |
| `DC-H4` | `M4.1.7` | A target with no placement memory is refused, since no playground provides memory there, and what the profile asks of a machine, one active core among it, is judged of any target named, where `cargo xtask target-agreement` holds only the shipped ones. |

## Why

- **Requirements are the contract.** `ROADMAP.md` §5.3 asks that refinement "preserve the obligations actually relied upon", and that "adding an
  unused device is not automatically an invalid refinement". A system relies on what its closure requires, which is exactly what is judged again. What the replaced hardware itself required goes with it, since it said what that machine asks of itself; what the profile asks of any machine is `DC-H4`.
- **It adds no judgement of its own.** Channels, ownership, connectivity, routes, placement, privilege and functions already read one system's hardware
  (model §8), so each holds on a target with no new rule. `§5.4`'s "device connectivity, interrupt routes, privilege, and memory placement" are the
  joint rules' there as here.
- **It answers `M3.3.5`'s R8.** The remark that `periodic-three`'s `device.timer` at `0x1000_0000` is `riscv-virt-up`'s UART held only if a description's
  regions reach a target. They do not: the platform that wrote them is replaced, and it is the target's own map the joint rules judge.

**Alternatives weighed.**

- **By name.** A target's `target.console` and a description's `console.uart` share no name, and two machines' devices should not have to.
- **A map written by hand**, `console.uart` is `target.console`, one per system and target. Each pair would be a claim, and checking it means judging the
  requirements at the target's device, which is this rule; so where facts tell devices apart it adds a file and no knowledge. Where they do not, the
  policy already orders candidates (`Policy::prefer`), by device name: one order for the whole system, so two requesters asking the same fact get one device under any order, and what differs is then said as a fact.
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
- ⚠️ **Silence is not an offer.** A target's file holds what its agreement checks, so a fact it does not state is not offered: a system declaring `interrupt-source` absent lands on `riscv-virt-up`, whose controller is present and unstated (`decision_target-platform-description.md` §2). That is how model §3 rule 3 reads an absence, and `DC-H1` is how a fact comes to be stated. A fact stated with any value counts as offered, as that rule reads it, so `(multicore false)` is refused as offering `multicore`.
- ⚠️ **What is claimed is what is judged.** Two plans agreeing on requirements says nothing of a run on one matching a run on the other: that comparison is `M4.9`'s.
- A service naming a device is refused here with a repair that works on a target, before the search's own refusal (`M1.49`), whose repair, to compose the device into the platform, does nothing where that platform is replaced.
- Nothing yet names a target on the command line, and `/2` does not exist: `M4.1.7` owns the input and the lowering, `M4.1.6` which device's alarm releases tasks.
- Today, of the five `examples/`, only `s0-heartbeat` has a plan of its own and on a target: the rest wait on `M3.7`, each refused `missing-fact` on either. `tests/target.rs`
  therefore holds the rule's other cases on fixtures written there, a target stating what `riscv-virt-up`'s file does not yet.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 14 | 5 (DC1-1 to DC1-5), beside 7 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: only the hardware a clause names is replaced, and all else it holds is judged on the target; each file stands alone, in both directions; only the platform all the hardware is made of speaks for the machine; a target's file holds one declaration per name |
