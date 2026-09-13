# `ev-01-watchdog-kick`

A periodic supervisor task must observe a liveness signal from two worker tasks and refresh a
hardware watchdog before it expires. The platform offers a watchdog block with a fixed timeout
and a refresh operation; refreshing early is permitted, refreshing late resets the system.

Requirements the description must express, without naming any implementation:

- a required refresh service whose worst-case interval between refreshes is bounded below the
  declared watchdog timeout;
- a liveness observation relationship between the supervisor and each worker that does not
  require inter-task communication (the profile excludes it);
- a fault response when a worker's liveness is not observed within its declared window.

What makes this a non-trivial evaluation case: the refresh bound is a *system-level* timing
requirement that depends on the supervisor's own response time, so it is a requirement whose
satisfaction is decided by the analysis rather than by direct matching.
