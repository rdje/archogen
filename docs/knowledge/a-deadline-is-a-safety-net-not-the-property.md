---
slug: a-deadline-is-a-safety-net-not-the-property
answers:
  - "My test passes alone and fails beside other builds — is it flaky, or is something wrong?"
  - "How long should a test wait for a process it starts?"
  - "A test checks that a hang is stopped — how do I keep it from failing on a slow machine?"
  - "How do I reproduce a failure that only happens on a loaded host?"
type: knowledge
date: 2026-10-22
---

# A deadline in a test is a safety net, never the property

## The question

A test passes alone and on a quiet machine, and fails when other builds run beside it. Nothing it
checks changed. Is it flaky, and what do you do about it?

## The answer

**It is not flaky; its deadline is too close to the time the work takes.** Every deadline in a test
is one of two things, and each has its own rule:

- **On something that must finish**, the deadline exists to end a hang, not to time the work. Make
  it one only a hang reaches — minutes, not a few multiples of a quiet run.
- **On something that must be stopped**, keep the deadline tight, and make the hang outlast every
  bound the test asserts, so that "stopped" and "waited for" stay far apart however slow the host.

One deadline shared by both sides is the defect's own shape: a sweep given `--timeout 1` must let
an unmutated baseline *pass* and a mutant *hang* within the same second.

Measured on `PROGRAM.99` (`2026-10-22`): the 30 s limit on a bounded child had been sized against a
6.23 s slowest body; eleven days later the slowest took 11.78 s, a 2.6× margin. A load re-created
at ×3.9 reproduced three children past the limit without a hang among them, and two sweep controls
racing a 1 s baseline; with the rules applied, the same loads passed.

## Why

A quiet run's time is the least the work will ever take, never what it takes on the host that runs
the gate. A host shared with other builds — or a CI runner with fewer processors — multiplies it,
and a margin of a few multiples is spent long before anything hangs. The failure then reads as a
regression in a change that did not touch the test, so it is waived or it blocks unrelated work.

## How to apply

- **Classify each deadline first**: the side that must finish, or the side that must be stopped.
  Size the first from what it is for, never from one machine's speed.
- **Reproduce under load before repairing.** `scripts/exec_pressure.sh --probe` measures what a load
  does to this host and refuses to claim a load that did not at least double a fresh program's
  time; `--workers N -- <command>` runs the suite under it. Re-run the same load after the fix.
- **Keep the failure's output.** Libtest captures a passing test's stderr (`--nocapture` shows it),
  and a runner that keeps only a tail loses the failure list it was asked to explain.
- A harness whose own outer limit must see a hang's finished failure may set a tighter inner one
  (`ARCHOGEN_PARENT_TEST_LIMIT`), and owns what that costs on a loaded host.

Related: [[a-gate-is-only-as-sharp-as-its-fixtures]].
