# Review history — `decision_hosted-build.md`

The independent review of [`docs/decisions/decision_hosted-build.md`](../decisions/decision_hosted-build.md), the hosted build's
design, with its code: the emitter, `archogen build`, `archogen explain` of a build and the simulator (leaf `M4.2.3.3`). Each
round is a reader who had not seen the reasoning that wrote the record, asked whether the record and the code hold together and
answer to `RTP-H1`, `RTP-H2`, `RTP-H3` and `RTP-H9`, never a hunt for code mutants. Its reader ran nothing, and traced each
reproducer through the code. H1-1, H1-2, H1-3, H1-5, H1-6, H1-7, H1-8 and H1-9 were then run against an install of `a1659ac6`, and each behaved as the reader said, before it was answered.

**Round 1** — `2026-11-04`: not acceptable, 15 findings.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| H1-1 | defect | `DIR.building` was deleted with no check of what it held: a note there, a build named `x.building`, a description kept there | A stage is made by `create_dir`, so only by this run; anything standing at `DIR.building` or `DIR.replaced` is refused and left alone (§5) |
| H1-2 | defect | an earlier build was any directory whose `provenance.json` held a substring, and all of it was removed: a file of mine beside a copied provenance, a description kept in a build; a test asserted the deletion | Format `/3` lists every file and its digest. A directory is replaced only if every listed file is unchanged and nothing else stands in it, `target/` and `Cargo.lock` being cargo's; else refused and left alone |
| H1-3 | defect | `remove_dir_all` then `rename`: a failure half way damaged the earlier build and deleted the new one | Two renames, so `DIR` is at every moment whole or absent, the first undone if the second fails; trouble removing what was replaced leaves the new package standing and says so. The chapter's "whole or not at all" reworded to that |
| H1-4 | defect | §5 kept the stage only for a record of `tool-failure`; every other disagreement deleted the record that proved it | The stage stays wherever the checker left a record the front will not build on |
| H1-5 | defect | a selector that misses on a healthy build said `stale-input`; a build of one system asked for another said "edited"; an account was printed and then called stale | A request that misses is `usage`; `--system` is held to the build's; a stale build prints no account, each route tested |
| H1-6 | defect | `explain` said "accepted" on the strength of two fields of the record, with `src/main.rs` and `plan.txt` edited, and a producer of `nobody` | It holds every listed file to its digest first, and the record to the plan, the program, the producer's name and the provenance's checker digest. `crate::built` says what digests do not catch: a forger who rewrites the provenance too |
| H1-7 | defect | a stated zero separation never arrived and the header did not say so | The header names it, one function deciding silence for the header, the tables and the horizon |
| H1-8 | defect | a symlinked `--out` was followed by the check, and replaced by the rename | Refused: name the directory itself |
| H1-9 | defect | `RTP-H1`'s second repair for `(unstated ITEM)`, stating a separation, was never offered | Offered, each cause's repair tested |
| H1-10 | defect | a tick refusal named no source, and blamed rounding for a least common multiple; no test reached either | Named, and a hyperperiod past the ticks says so, in the record and the chapter; unit tests reach both |
| H1-11 | gap | a hyperperiod that fits `u64` has no bound: `1 ms` against `2305843009213693951 ms` never finishes | `MOST_RELEASES`, `2^20` releases and arrivals, refused `unsupported-profile` naming the count, in the record's §3 |
| H1-12 | gap | "what each table entry came from" held only of `TASKS`, and a dead `continue` meant the test could not see it | Entries for `SOURCES` and `INITIALISATION`; the test follows every entry, and counts them |
| H1-13 | gap | tests asserting less than their names: one arrival for "its stated separation"; six files of thirteen for "every generated file"; "exact trace"; a bless mode that passes; `!accepts` untested; one basename and no tree for "wherever it lies" | Four arrivals counted; every file; renamed; a bless that never passes; `!accepts` and the rest tested; a tree at two depths. `harness.rs`'s one-input-one-trace stands: it guards against a source of nondeterminism entering |
| H1-14 | gap | `RTP-H1` held only implicitly: `(enabled …)` never read, the slot never held to the plan's index, no sub-millisecond test; the profile still naming `M4.2` for one context at a time; `RTP-H2` in no child leaf | The emitter reads the enabled set and holds each slot to its index, unit tests editing a plan the lowering would never write; a 250 us period and `rank-gaps` run; the profile's part names the harness; `M4.2.5` filed for `RTP-H2` |
| H1-15 | wording | the chapter's order put the output directory last; §2's every duration; "the path it was given"; `a/b` left behind; two digests both called the plan; "as the plan infers" of a task's own source; a diamond listed twice | Each corrected, and the parents a refused build made are removed, and a module imported twice is recorded once |

Also said, and left as it stands: the generator cannot compile against the runtime, so a table it writes is first compiled when
the package is, a test building and running each shape; the admissions hash `COPIES`'s extent and not the copied bytes, so the test
is the only binding, and the record says so.
