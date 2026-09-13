# The boundary corpus

Paired worked cases for the controlling boundary
(`docs/decisions/decision_eadl-engine-boundary.md`): declarations that eADL **may** carry, and
declarations that belong to the engine. `ROADMAP.md` §4.3 requires this corpus, and fixture
**F27** mechanizes it — accepted functional guarantees are interpreted, forbidden
implementation fields are rejected, and each rejection names the test that failed.

21 cases: **10 accepted**, **11 rejected**, of which **5 are ambiguous** and
carry an explicit argument rather than an assertion.

> The surface syntax here is **draft**. M1 fixtures settle the real spelling
> (`ROADMAP.md` §5.1). What is not draft is the classification: which *meaning* belongs on
> which side of the boundary. That is what F27 tests and what a syntax change must preserve.

## How to read a case

Each `.eadl` file opens with a metadata comment block and then the declaration itself:

```text
; case: <name>
; verdict: accept | reject
; ambiguous: yes | no
; tests: externality=<pass|fail> implementation-independence=<pass|fail> non-prescription=<pass|fail>
; failing-test: <name>          (rejected cases only)
; rationale: <why this verdict>
; other-side: <what the other side owns>
```

**A wrapped value stays one value.** A line opens a header only when it is *unindented* — the
ordinary `; key: value` spacing — **and** the text before its first colon is a bare kebab-case
key. Continuation lines are indented further:

```text
; rationale: the test that settles it is
;   implementation-independence: a different timer with the same width and rate
;   is interchangeable here
```

Both conditions are needed, and each was added after the other alone failed on a real case in
this directory. Indentation alone fails because every comment begins with a space after the
`;`. The key shape alone fails because a rationale can wrap onto a line that *is* exactly a key
followed by a colon — which is precisely what
[`counter-width-and-rate`](accept/counter-width-and-rate.eadl) does. An empty `;` line closes
the block.

The three tests are the ones in the boundary decision:

| Test | Asks |
| --- | --- |
| `externality` | does it state an offered feature, required functionality, architectural connection, operating condition, or externally testable guarantee? |
| `implementation-independence` | would it remain valid for a different implementation with the same relevant functionality and guarantees? |
| `non-prescription` | can its interpretation be stated without prescribing an algorithm, instruction sequence, code provider, data structure, or executable model body? |

A case fails on the **first** test that rejects it; `failing-test` records which. Several
rejected cases would fail more than one, and their rationale says so.

## Accepted — these belong in eADL

| Case | Ambiguous | Failing test | Why |
| --- | --- | --- | --- |
| [`absolute-deadline-service`](accept/absolute-deadline-service.eadl) | no | — | Names a required service and the externally observable bounds it must meet |
| [`access-authority`](accept/access-authority.eadl) | no | — | States the authority condition the function must be reachable under |
| [`addressable-region`](accept/addressable-region.eadl) | yes | — | AMBIGUOUS, resolved ACCEPT |
| [`atomic-observation`](accept/atomic-observation.eadl) | no | — | Constrains what an observation may be, not how it is obtained |
| [`counter-width-and-rate`](accept/counter-width-and-rate.eadl) | yes | — | AMBIGUOUS, resolved ACCEPT |
| [`fixed-priority-execution`](accept/fixed-priority-execution.eadl) | no | — | Required policy behavior with the task constraints that make it checkable |
| [`interface-units-and-ranges`](accept/interface-units-and-ranges.eadl) | no | — | Declares the representation contract at the architectural interface: what the |
| [`power-state-availability`](accept/power-state-availability.eadl) | no | — | An operating condition under which the required functionality must still hold |
| [`required-ordering-guarantee`](accept/required-ordering-guarantee.eadl) | no | — | An externally required ordering property between two observable effects |
| [`time-horizon`](accept/time-horizon.eadl) | no | — | States an externally testable guarantee about observed time |

## Rejected — these belong to the engine

| Case | Ambiguous | Failing test | Why |
| --- | --- | --- | --- |
| [`atomic-read-retry-loop`](reject/atomic-read-retry-loop.eadl) | no | non-prescription | A retry sequence is one way to obtain a coherent observation |
| [`code-provider-selection`](reject/code-provider-selection.eadl) | no | externality | Names the code that must be used |
| [`context-save-layout`](reject/context-save-layout.eadl) | no | non-prescription | Prescribes the register save order and frame shape |
| [`execution-bound`](reject/execution-bound.eadl) | yes | externality | AMBIGUOUS, resolved REJECT |
| [`initialization-order`](reject/initialization-order.eadl) | yes | implementation-independence | AMBIGUOUS, resolved REJECT |
| [`lowering-instruction`](reject/lowering-instruction.eadl) | no | non-prescription | An instruction to the generator about where output goes |
| [`ready-queue-structure`](reject/ready-queue-structure.eadl) | no | non-prescription | Names the data structure that realizes the policy |
| [`register-programming-sequence`](reject/register-programming-sequence.eadl) | no | non-prescription | A write sequence with an ordering and an acknowledgment is a driver |
| [`retry-permitted`](reject/retry-permitted.eadl) | yes | non-prescription | AMBIGUOUS, resolved REJECT |
| [`rollover-algorithm`](reject/rollover-algorithm.eadl) | no | non-prescription | The declaration carries the epoch-extension procedure |
| [`simulator-model-body`](reject/simulator-model-body.eadl) | no | non-prescription | An executable state-transition body |

## The ambiguous cases are the point

Obvious cases do not teach the boundary; they only confirm it. §4.3 requires that "ambiguous
new fields require a worked classification case before adoption", and the five ambiguous cases
here are the ones a reasonable author gets wrong in both directions:

- **`counter-width-and-rate`** and **`addressable-region`** *look* like implementation detail
  and are **accepted**: they are architectural facts a substituting implementation must also
  have.
- **`execution-bound`**, **`initialization-order`** and **`retry-permitted`** *look* like
  requirements and are **rejected**: a WCET is evidence about a binary, a boot sequence
  describes one realization, and a permission field narrows the engine to strategies someone
  happened to think of.

Two neighbouring pairs are worth reading together, because the boundary runs *between* them:

| Accepted | Rejected | What separates them |
| --- | --- | --- |
| [`atomic-observation`](accept/atomic-observation.eadl) | [`atomic-read-retry-loop`](reject/atomic-read-retry-loop.eadl) | the guarantee versus one way of obtaining it |
| [`addressable-region`](accept/addressable-region.eadl) | [`register-programming-sequence`](reject/register-programming-sequence.eadl) | where the device is versus how it is driven |
| [`required-ordering-guarantee`](accept/required-ordering-guarantee.eadl) | [`initialization-order`](reject/initialization-order.eadl) | a required ordering property versus a chosen boot sequence |

## Adding a case

1. Apply the three tests and write the answers down. If the verdict is not immediate, it is an
   ambiguous case: say so in `ambiguous:` and make the rationale an **argument**, naming the
   test that settles it.
2. Prefer adding a **pair** — the accepted contract and the rejected procedure that satisfies
   it. A pair teaches the boundary; a lone case only records a verdict.
3. Re-run the classification review when introducing a new kind or feature family. The
   boundary is re-established at every extension, not established once.

⚠️ The mechanical check is a floor. §4.3 is explicit that "human review still checks semantic
intent, because a field can hide an algorithm behind an innocent name." A corpus that only
contains cases the checker already handles has stopped doing its job.
