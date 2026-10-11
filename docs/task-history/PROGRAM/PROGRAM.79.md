- ID: `PROGRAM.79`
  Status: `done` — filed, started and closed `2026-10-11`
  Goal: the accepted designs' folder has room for the designs still in review and those the milestones owe.
  Reproduce / issue: `M3.6.9`'s review round 1 answer, the doctrine gate → *"README-ROUTES: docs/specs/: 396402 bytes
  in total, over its ceiling of 393216"*; at `2b66082`, `git ls-files docs/specs | xargs cat | wc -c` → `389283`.
  Direction: the measurement, weighed in `docs/decisions/decision_specs-folder-ceiling.md` against compacting and a
  total by subject, as the record's "a later raise is a dated paragraph here" asks.
  Decided `2026-10-11`: a raise, to 524 288 bytes, by the record's dated paragraph; 300 288 at its last raise,
  `b91358a`, and 88 995 bytes in the 46 commits since — three trust designs moved in.
  Verification: `2026-10-11` — `bash scripts/check_readme_routes.sh` → *"OK (24 destination(s) governed)"*, and its
  self-test → *"28 pass / 0 fail"*, the policy's cell and its decision's maximum agreeing; the doctrine gate at commit.
  Commit: `ARCHOGEN-PROGRAM-0600 (leaf PROGRAM.79)`
