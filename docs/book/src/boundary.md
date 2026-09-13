# The boundary: functionality versus implementation

Everything in archogen rests on one rule:

> **eADL describes hardware and OS features, functionalities, their externally required
> behavior, and constraints. It contains no implementation.**

Algorithms, device implementations, register-programming sequences, simulator models,
provider selection, lowering rules, code layouts, and code generation belong to the engine
and its versioned knowledge bases.

This is what makes a description *reusable*. A description that encodes how one timer is
programmed is a driver with extra steps; a description that states what the time service must
guarantee can be satisfied by a different timer, on a different board, through a different
adapter — and the engine has to prove it did.

## It is about meaning, not vocabulary

The boundary is **not** "technical words go in the engine". A constraint on observable
functionality belongs in eADL even when it says *counter*, *modulus*, or *privilege*. A
procedure belongs to the engine even when it is phrased descriptively.

Two consequences people reliably get backwards:

- A **wrap limit** or an **atomic-read guarantee** is not automatically an implementation
  detail. "Time observations must be unambiguous across at least 60 seconds" is a contract
  that many different timers can satisfy.
- Asking an author to write the **rollover algorithm** or the **retry loop** crosses the
  boundary, however briefly it is written.

Both can be live at once. Native access atomicity may be a fact in *engine platform
knowledge*, while the required coherence of the exposed observation is an *eADL contract*.

## The three tests

Every proposed field faces all three:

1. **Externality** — does it state an offered feature, a required functionality, an
   architectural connection, an operating condition, or an externally testable guarantee?
2. **Implementation-independence** — would it remain valid for a different implementation
   with the same relevant functionality and guarantees?
3. **Non-prescription** — can its interpretation be stated without prescribing an algorithm,
   an instruction sequence, a code provider, a data structure, or an executable model body?

Fail test 1 or test 3 and the field belongs in engine knowledge. Test 2 catches the subtle
case: a field that survives substituting the implementation is a contract; one that does not
is a description of *this* implementation wearing a contract's clothes.

## Worked cases

| Case | eADL may state | The engine owns |
| --- | --- | --- |
| Time horizon | "observations unambiguous across ≥ 60 s", or an offered counter width and rate | wrap handling, epoch extension, arithmetic, and the proof the requirement is met |
| Atomic observation | "an observation must not combine inconsistent halves" | native wide read, high/low/high retry, synchronization, firmware access |
| Deadline delivery | "an absolute-deadline service with this horizon and delivery bound" | comparator programming, delay-to-deadline adaptation, race handling |
| Power availability | "this time service remains available in the declared idle state" | power sequencing, clock switching, the state model, the validation procedure |
| Access authority | "available at the required privilege, or through an allowed mediation boundary" | trap path, firmware ABI, privileged instructions, capability checks |
| Scheduling | "static-priority preemptive execution with these task constraints" | ready-queue structure, dispatch, save/restore code, cost charging |
| Ordering and coherence | an offered coherence property, or a required ordering guarantee | barriers, cache maintenance, executable memory and device models |
| Representation units | declared units and acceptable ranges at the architectural interface | scaling, rounding, conversion helpers, overflow checks |

## A requirement is an obligation, not a claim to trust

- A **declared capability is not evidence**. "Offered" records a claim; its evidential status
  is tracked separately.
- A **refinement declaration is an obligation to check**, not permission to believe.
- When nothing can realize a requirement, the engine reports **missing engine capability** —
  never "impossible", unless that has actually been established.

## What the boundary rules out

Adding implementation syntax as an escape hatch for missing engine support. If a functional
description cannot be realized, the defect is in engine knowledge, and the fix is tracked
engine work — not a new eADL field that smuggles the procedure in.

This is enforced, not merely believed: fixture **F27** checks that schemas reject forbidden
implementation fields and interpret the accepted ones correctly. The mechanical check is a
floor — human review still checks intent, because a field can hide an algorithm behind an
innocent name.

The full decision, including the direction of every obligation and the review procedure for
new kinds, is `docs/decisions/decision_eadl-engine-boundary.md`.
