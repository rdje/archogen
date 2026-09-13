# `ev-03-burst-tolerant-logger`

A sporadic task released by an external event whose arrivals may burst: a declared minimum
separation, plus a declared maximum number of releases within a longer window.

Requirements the description must express:

- a sporadic release model with both a minimum separation and a burst bound;
- a constrained deadline relative to the release event;
- the required behavior when a release arrives while the previous job has not completed.

What makes this a non-trivial evaluation case: the admitted workload model in §3.1 declares
"sporadic releases with declared minimum separation". A burst bound is *additional* arrival
information the initial analysis may not admit. The honest outcomes are therefore either a
supported analysis that uses it, or `unsupported-profile` naming what is not modeled — and
silently ignoring the burst bound while analyzing on minimum separation alone would be a
defect this case is designed to expose.
