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

## How much of this a machine can check

Fixture **F27** enforces the part a machine can. It refuses the constructs that are
implementation *by definition* — an `implementation` or `model` body, a `provider` selection, an
`emit` instruction, a `permits` strategy, a `wcet` bound, an `init-order` sequence, a
`save-order` layout, a `read-sequence` protocol — and each refusal names the test that failed,
the question that test asks, and where the content belongs:

```text
error[boundary-implementation-in-description]: `wcet` is implementation, and eADL contains no implementation
  --> examples/sensor.eadl:4:5
  |
4 |     (wcet 850 us)
  |     ^^^^^^^^^^^^^ this is an execution bound — evidence about a binary, not a property of the system described
  = hint: it fails the `externality` test — does it state an offered feature, required functionality,
          architectural connection, operating condition, or externally testable guarantee?
          It belongs to the engine build manifest: §7.3 keeps bounds, their origin, and their target
          and binary identity outside eADL.
```

### Why a construct registry and not a keyword scan

The tempting implementation looks for imperative-sounding words. The corpus disproves it in one
case. This declaration is **accepted** and contains `write` twice:

```text
(defplatform soc.bus
  (requires (ordering (before (write device.control))
                      (after  (write memory.buffer)))))
```

Here `write` names an *observable effect* that an ordering requirement is stated over, not a
step to perform. What separates the two is the **construct** the content sits inside: everything
under an `implementation`, `model`, `provider` or `emit` block is a procedure; the same word
elsewhere is a reference.

### What F27 proves, and what it does not

It runs three arms, and the third is what makes the first two mean anything:

- **agreement** — the classifier's verdict matches every corpus case's recorded verdict, and its
  named failing test matches the recorded one;
- **coverage** — every registered construct is exercised by a worked case;
- **mutation** — seeding a forbidden field into an accepted case flips it to rejected with the
  right test, and removing the offending construct from a rejected case flips it to accepted.

Without mutation, a classifier that accepted everything would still pass the accept half of the
agreement arm.

⚠️ It does **not** establish that an accepted declaration is sound. §4.3 requires human review
for intent, because a field can hide an algorithm behind an innocent name. Acceptance means
"contains no construct the registry knows to be implementation" — and the gap between those two
sentences is where the interesting mistakes live.

The full decision, including the direction of every obligation and the review procedure for
new kinds, is `docs/decisions/decision_eadl-engine-boundary.md`.
