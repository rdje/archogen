# `ev-05-privileged-time-access`

A platform where the time service is reachable only through a mediation boundary: the
application privilege level cannot read the counter directly, and access is via a firmware
call with its own declared latency bound.

Requirements the description must express:

- the required time service and its accuracy/horizon contract, unchanged;
- the access authority condition — available at the required privilege, or through an allowed
  mediation boundary;
- the operating condition that the mediation path's latency is bounded.

What makes this a non-trivial evaluation case: the mediation latency is not a detail, it is a
cost that enters the timing ledger, and the choice of mediated versus direct access interacts
with other operations — §5.4's example of choosing firmware control for one operation and
direct control for another being invalid when the two cannot safely coexist. The case tests
whether access authority is treated as a first-class contract or quietly assumed away.
