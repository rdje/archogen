# Mutation testing waits until the functionality is locked

- **Type:** `decision`
- **Date:** `2026-10-21`
- **Status:** `active`
- **Owner / source:** the director, `2026-10-21`, answering findings §14: "Stop this mutation thing. Resume that when
  the project is completed, at this point we can lock the functionality do extensive mutation testing", then "So no
  more mutation testing, we stop that now until the project is complete", and "Every decision you take from now should
  advance the project toward meeting its goals and objectives, that's high priority." Recorded under leaf
  `PROGRAM.98`.

## The fact / decision

Mutation-adequacy work stops now and resumes once the project's functionality is complete and locked, when it is done
extensively in one campaign. Every pending leaf of `M3.6.9` — the survivor assessments, controls, proofs and replays of
`xtask/src/trust.rs`, `trust_generated.rs` and `trust_gate.rs` — is postponed, and the frontier moves to the roadmap's
product work, `M3.3` first. From now on, every decision is taken to advance the project toward its goals and objectives;
that is the first priority.

## Why

From `2026-10-11` to `2026-10-21`, 158 of the 173 commits were `M3.6.9` work and none advanced the product
(`git log --oneline --since=2026-10-11 --grep='leaf M3.6.9' | wc -l` → 158; `git log --oneline --since=2026-10-11 |
wc -l` → 173; the other 15 are `PROGRAM` upkeep). Each survivor opened further children, and the pending lanes still
listed about 1 100 candidate edits (findings §14). A mutation result is bound to the exact source it ran against — the
lanes' own contracts require a fresh binding after any source change — so measuring now, on code that `M3.3` to `M7`
will change, pays for most of it twice; measuring once the functionality is locked pays once. The roadmap's F30 exit
criterion asks that the trust gate surface unreviewed sharing and drift and block acceptance; that gate exists and runs.

## How to apply

- Do not start, extend or replay any mutation-adequacy leaf, sweep or catalogue run until the director resumes it at
  completion. The postponed leaves keep their contracts and their recorded results; they are `blocked`, naming this
  record, never closed.
- What stays: each feature's own tests, the focused tier on every commit that changes Rust, the full CI before every
  push, the trust gate on every push to `main` (judging each commit it brings), and the existing mutation catalogue's
  entries that `cargo test` itself holds (`PROGRAM.61`). The `extended` tier's catalogue step, which CI does not run,
  is left as it is: a regression check of fixed entries, not adequacy work; removing it would change code without
  advancing the project.
- A real defect is still owned and fixed (§15 of the session directives), whatever found it. The one open defect
  investigation in the postponed lane, `M3.6.9.6.3.2.2.15.2.1.8.2` — two generators given one target directory — is
  such a defect, kept `pending` at its priority, not postponed.
- Rigor is proportionate to risk: a real defect gets a reproduction, a fix and a control; evidence procedures built for
  the adequacy campaign are not applied to ordinary product slices.
