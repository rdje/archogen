# The controlling eADL/engine boundary

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** `ROADMAP.md` controlling boundary + §4.1, §4.3; director's clarification
  recorded in the roadmap preamble. Established at leaf `M0.1`.

## The fact / decision

**eADL describes hardware and OS features, functionalities, their externally required
behavior, and constraints. It contains no implementation.**

Algorithms, device implementations, register-programming sequences, simulator models,
implementation-provider selection, lowering rules, code layouts, and code generation belong
to the engine and its versioned knowledge bases. This boundary takes precedence over
ambiguous wording in any source draft, and over any later convenience argument.

The boundary is about **the meaning of a declaration, not how technical its vocabulary
sounds**. A constraint on observable functionality can belong in eADL even when it uses words
like "counter", "modulus", or "privilege". The *procedure used to satisfy it* belongs to the
engine even when it sounds descriptive.

## The three tests

Apply all three to every proposed eADL field.

1. **Externality.** Does it state an offered feature, a required functionality, an
   architectural connection, an operating condition, or an externally testable guarantee?
2. **Implementation-independence.** Would the declaration remain valid for a *different*
   implementation with the same relevant functionality and guarantees?
3. **Non-prescription.** Can its interpretation be stated without prescribing an algorithm,
   an instruction sequence, a code provider, a data structure, or an executable model body?

If test 1 fails, or test 3 fails, the field belongs in engine knowledge or engine
configuration instead. Test 2 is the one that catches the subtle cases: a field that survives
substituting the implementation is a contract; one that does not is a description of *this*
implementation wearing a contract's clothes.

A concrete architectural identity or an addressable region may legitimately constrain a
platform. That is **not** permission to put its driver in the language.

## The worked cases

These eight are the reference corpus. Each names what eADL may say and what the engine owns.

| Case | eADL may state | The engine owns |
| --- | --- | --- |
| Time horizon | "time observations must be unambiguous across at least 60 seconds"; or an offered modular-counter width and rate where architecturally relevant | wrap handling, epoch extension, read frequency, the arithmetic, and the proof that the requirement is met |
| Atomic observation | "a counter observation must not combine inconsistent halves" | native wide read, high/low/high retry, synchronization, or firmware-mediated access |
| Deadline delivery | "an absolute-deadline service with a specified supported horizon and delivery bound" | comparator programming, delay-to-deadline adaptation, acknowledgment and race handling |
| Power availability | "this time service remains available in the declared idle state" | power sequencing, clock-source switching, the state-machine model, the validation procedure |
| Access authority | "this function is available at the required privilege, or through an allowed mediation boundary" | trap path, firmware ABI, privileged instructions, capability-check code |
| Scheduling | "static-priority preemptive task execution with these task constraints" | ready-queue structure, dispatch algorithm, save/restore code, cost charging |
| Ordering and coherence | an architecturally offered coherence property, or an externally required ordering guarantee | barriers, cache maintenance, buffer synchronization, executable memory and device models |
| Representation units | declared units and acceptable ranges at the architectural interface | scaling, rounding, conversion helpers, overflow checks |

Two of these are the ones people get wrong, so they are stated explicitly:

- **A wrap limit or an atomic-read guarantee is not automatically an implementation detail.**
  "Observations stay unambiguous for 60 s" is a contract a different timer can also satisfy.
- **Conversely, asking an eADL author to write the rollover algorithm or the retry loop
  crosses the boundary**, no matter how briefly it is written.

Native access atomicity may be a *fact in engine platform knowledge* while the required
coherence of the exposed observation is an *eADL contract*. Both can be relevant at once;
keep the distinction explicit when they are.

## The direction of the obligation

An eADL requirement is an **obligation the engine must discharge**, never a claim the engine
may trust:

- A declared capability is not evidence. "Offered" records a claim whose evidential status is
  tracked separately (`ROADMAP.md` §5.3).
- A refinement declaration is an obligation to check, not permission to trust (§5.1.1).
- If no known realization exists, the engine reports **missing engine capability**. It must
  not declare the requested function logically impossible unless that has actually been
  established (§5.4).

## What this rules out

- Adding implementation syntax as an escape hatch for missing engine support. If a
  functional description cannot be realized, the defect is in engine knowledge, and the fix is
  a tracked engine work item — not a new eADL field that smuggles the procedure in.
- `defkind` becoming a host-code evaluator or an implementation template language (§5.6).
  Sharing syntax with other declarations does not make new functionality executable.
- Treating the engine's typed internal representations — interpreted requirements, bound
  systems, runtime plans, emission structures — as "exposing implementation in eADL". They are
  engine implementation structures and always were.

## How to apply

1. **Every new field or kind gets the three tests, written down.** An ambiguous case requires
   a worked classification case before adoption — the corpus under `docs/semantics/boundary/`
   (leaf `M0.2`) is where it lands.
2. **F27 checks mechanically what it can**: schema rejection of forbidden implementation
   fields, and correct interpretation of accepted ones (leaf `M0.3`). Human review still
   checks semantic intent, because a field can hide an algorithm behind an innocent name.
   The mechanical check is a floor, not the decision.
3. **Re-run the classification review when introducing a new kind or feature family.** The
   boundary is not established once; it is re-established at every extension.
4. **When a stop/rework trigger fires** — "a supposedly functional eADL example needs
   driver/model/code-generation instructions" (§17) — the required response is to repair the
   boundary and move the implementation into engine knowledge. Not to relax the boundary.

Related: [[decision_zero-dependency-engine-core]] — the same instinct applied to tooling.
