# `ev-02-dual-rate-sampler`

Two sampling tasks at rates that are not harmonic (their periods have a large least common
multiple), sharing one observable output. The output device admits one writer at a time.

Requirements the description must express:

- two periodic tasks with the declared rates and constrained deadlines;
- exclusive access to the output device, with the required behavior when both would write;
- an ordering guarantee between a sample and the write that reports it.

What makes this a non-trivial evaluation case: exclusive ownership of a single device by two
requesters is exactly the F08 shape — the engine must either find a selected realization that
supplies the sharing semantics with bounded overhead, or refuse. The non-harmonic periods mean
the response-time recurrence does not terminate after one or two steps, so the analysis is
exercised rather than short-circuited.
