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

**Round 2** — `2026-11-04`: not acceptable, 12 findings. Its reader ran the toolchain, an install of `5c136b19`'s code, over 196
descriptions, and found "a build deletes nothing it did not write" still false by several routes. The explain answer and the
arithmetic of the run's bound held. Of round 1, H1-2 was not closed; H1-3 and H1-13 held but for the rows below.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| H2-1 | defect | `target` and `Cargo.lock` were exempted by name, whatever they held, so a file of mine under them, or the description being built, was deleted; a test asserted it | What those names hold is carried to the new package, not judged and not deleted. `.DS_Store` with them, the desktop's. Cargo's output now survives a rebuild too |
| H2-2 | defect | an earlier build was checked once and later removed whole, so a file written meanwhile went with exit 0 | Removal by the list: each file only if still as written, then the directories that held them where left empty. What appears meanwhile stays at `DIR.replaced`, and the command names it |
| H2-3 | defect | the run's bound came before `(released-by (unknown CAUSE))`, and hid its code, its task and its repair | The unknown cause is refused first |
| H2-4 | defect | an empty directory of mine passed as no foreign thing, and went | A directory that holds no listed file is no build's, empty or not |
| H2-5 | gap | `provenance.json` was outside every check: a note in it unnoticed by `explain` and deleted by `build`, a link accepted, a pipe hanging both, `"files": []` accepted | Format `/4`'s `self`, its own digest over its text. A regular file, read within a bound. A list lacking what every build writes is no build's |
| H2-6 | gap | `explain` asked `edited` and never `foreign`, so a `build.rs` added changed what the package did and was called healthy | `explain` asks both, as `build` does |
| H2-7 | gap | a source-released task's own minimum separation entered the hyperperiod, though the run releases it only at arrivals: a plan refused that should run, and a header overclaiming | The hyperperiod is of what the run repeats: each timer-released task's period and each arriving source's separation |
| H2-8 | gap | deadlines and jitters set the tick, though no table holds them, and a refusal blamed the wrong duration | The tick is of those same durations |
| H2-9 | gap | an empty `--out` was renamed away, and a private directory's mode lost | The new package takes the mode of what it replaces; the record says an empty directory is removed too, and a mount point fails safe |
| H2-10 | wording | "the earlier one stays at" was false once removal stopped part way | It says what `DIR.replaced` keeps, naming it; a test holds the new package whole against a clean build |
| H2-11 | gap | a failed restore was dropped, and the command said nothing of where the earlier build was | The refusal names where it is. Not forced in a test: it needs something to appear at `DIR` between two renames |
| H2-12 | gap | `--out missing/../x` read as absent and resolved after; a stale hint; a test line asserting nothing; `std::env::args` panicking on a non-UTF-8 argument, exit 101, a code no status has; a lossy name compared with a list; a description at a definition's path left unrecorded | Refused; reworded; removed; `args_os` and `usage`; a name that is no text is foreign; the language filtered by name and text |

Said, and left as it stands: `.DS_Store` deeper than the top still refuses, naming it; durability across a power loss is not claimed,
there being no sync call. The two renames say nothing of that.

**Round 3** — `2026-11-04`: not acceptable, 9 findings, and its one defect needs another program working in the same directory in a
window of milliseconds. Its reader re-derived the tick, the horizon, and every release and arrival count of the 117 plans that build, from
each `plan.txt`, against what each run printed, and found none wrong. Eleven of round 2's twelve answers held, H2-2 but for a link or a pipe.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-1 | defect | `Built::remove` held a file to less than `Built::edited`: a link was followed or removed, a pipe hung the command after the package was placed, and a listed directory swapped for a link lost byte-identical copies outside the build | Each path held as `edited` holds it, a regular file reached through real directories, and what went aside held to being a directory; a test for each route. The window left is stated: `std` has no directory-relative open |
| R3-2 | gap | a rename silently replaces an empty directory that came during the window, at `DIR` and at `DIR.replaced` | Held again just before each, refusing if something came. The instant left is stated: `std` has no rename that refuses to replace |
| R3-3 | gap | the stage took a private `--out`'s mode only at the end, and a kept stage never did | It takes the mode when it is made |
| R3-4 | gap | a test named for H2-3 no longer held it: its comment false since H2-7, nothing beside an unknown cause over the bound | It runs that very system, refused for its cost alone and for its cause with `chime` |
| R3-5 | gap | tests holding less than their names: no listed file edited meanwhile, `.DS_Store` in no test, no other checker's digest, `states no separation` untested, six of twelve files | Each added, the twelve read from the build itself |
| R3-6 | wording | the chapter's "a build deletes nothing" against the empty directory it takes; "exactly" omitting the parents it made; the stage taking what is dropped in it; empty directories unnamed; "whole or absent" against an empty `--out`; a hint wrong on one route | Said; empty directories named; the hint withheld there |
| R3-7 | wording | "unchanged … is `stale-input`" held only if the edit still reads, in the same format, and "nothing else there" overlooked the carried names | Said: a provenance that no longer reads, or names another format, is `usage`, being no build this engine reads |
| R3-8 | wording | leftovers from before H2-8 in the chapter and record: "the plan's durations", "a duration past 64 bits", "each naming the task" | Reworded to what the run repeats and to what each refusal names |
| R3-9 | gap | older than this review: a description that is a link read its modules from two places, the front's beside the link and the checker's beside its target, so a build said "report it", falsely | `consume.rs` keeps the description's own name under its canonical directory, so both read through the link; a test |


**Round 4** — `2026-11-05`, reported `2026-11-04`: not acceptable, 5 findings, each defect a refusal that leaves `--out` as it was. Its
reader worked over round 3's answers alone, on an install of `d14c6fba`'s code. Each was first measured on an install of `68f62a87`'s
code, each behaving as the reader said; an install of the answers' own snapshot measured each after.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R4-1 | defect | a regression from R3-3's answer: `mkdir ro; chmod 555 ro; archogen build S --out ro` was `tool-failure`, exit 70, with a false hint, since the stage took `DIR`'s mode before anything was written | The stage closes from the first what that mode closes, its owner able to write; the package takes the exact mode last, on every way out once it stands; what went aside is opened to its owner the same way, and what stays takes back its mode. Tests at `555` and `500`, empty and over an earlier build with what cargo left carried, a stage at each, and what stays |
| R4-2 | defect | a `--out` name too long for its stage read as absent, any error taken for absence, so it was refused late as `tool-failure`, and a failed `open` left the parents it made | Only `NotFound` is absence, the rest `usage` at once. A path reads as absent up to its first missing name, so under parents that do not stand yet only making the stage can tell: that error is `usage` too. A failed `open` removes the parents, never the stage, which it did not make. A test for each route |
| R4-3 | wording | "a file whose bytes match" was false of the instant the record disclaims: what stands at that path at that moment goes, whatever its bytes | Said so in the record, `build_cmd.rs` and `built.rs` |
| R4-4 | wording | what appeared at `DIR` or `DIR.replaced` while a build ran was refused, where the record and the chapter said it stayed with the package in place; those refusals printed a false hint; what is put in the stage lands with it; the message for a `DIR.replaced` a finished build left named only other causes | Each said as it is. Each refusal has its own repair, by a type of them, one that finds what stood at `DIR` gone among them; a test for each and for what lands. The message says what may have left each name, and no more than may |
| R4-5 | wording | "whole or absent" and "deletes nothing" unqualified in the chapter's "In one minute" and `build_cmd.rs`; "the one thing it removes" against the stage's contents; "the plan's durations" in `docs/book/src/s0.md` and a comment of `crates/archogen-emit/tests/emit.rs` | `DIR` holds at every moment what stood there, nothing, or the whole package; what a build deletes that it did not write is named wherever that is stated; the rest reworded |

Seen beside it, and answered with these: `archogen resolve --out` at a directory left its `.part`: what it wrote beside the plan goes if
the plan cannot land, and what already stands at that name is refused and left alone, a test holding both. A closed standard output
exited `70` with nothing said, by two routes, the library's `emit` and the executable's flush: each says so now, `archogen_cli::unwritten`, a
test holding both, and the record and `cli.md` state it. Its code stays the older design's, for every command.
