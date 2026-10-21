- ID: `PROGRAM.97`
  Status: `done` — the colon path limit surfaced to the director, findings §13
  Goal: `M3.6.9.6.3.2.2.15.2.1.6.2.3` measured that the pinned Cargo fails every build under a target directory
    holding a colon, so the repository cannot live under such a path, against the standing rule that it may be
    moved anywhere. Record it as §13 of `docs/decisions/decision_findings-for-director-review.md` with the
    measurement, what the project already does about it, the options and a recommendation, so it survives a missed
    message. No code changes.
  Verification: §13 written with the measurement of `target/colon-path-assessment/`, what is already done, the
    harmless characters, and a recommendation that waits on nothing; the register's count moved to thirteen.
  Commit: `ARCHOGEN-PROGRAM-0834`

