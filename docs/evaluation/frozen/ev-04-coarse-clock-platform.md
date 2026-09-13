# `ev-04-coarse-clock-platform`

The same workload as a previously supported system, on a platform whose timer has a much
coarser tick and a much shorter unambiguous horizon than the original target.

Requirements the description must express: unchanged from the original system. Only the
platform description changes.

What makes this a non-trivial evaluation case: it tests the claim at the heart of the program —
that a functional description is portable across platform realizations — and it tests it in the
direction that can fail. The coarse tick may make a deadline unrepresentable at the required
resolution; the short horizon may violate a declared unambiguous-time requirement. The required
answers are a built system where the platform can satisfy the contract, and a named violated
obligation where it cannot. Rounding a deadline silently to the nearest tick would be a defect.
