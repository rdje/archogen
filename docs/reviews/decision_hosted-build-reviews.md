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

**Round 5** — `2026-11-05`: not acceptable, 11 findings, only R5-5 touching a user's bytes, and it needs another program writing in the
stage during the run. Its reader built a rig that acts while a build runs, as the tests' launcher does, and ran 117 builds over 39 modes, 51
refusal routes, 47 lengths, 47 cases during a run, 320 racing runs, 300 parallel builds and 6,690 truncation points. R4-1, R4-3 and R4-5
held as answered, R4-2 on its routes, R4-4 but for R5-6, and none regressed rounds 1 to 3. The answers below were measured by re-running that
rig against an install of them.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R5-1 | defect | a standard output closed before the run, or open only for reading, exits with the command's own code and says nothing, where the chapter said "takes nothing" is `70`; round 4's history row says "a closed standard output" of what was a pipe with no reader | The runtime gives the first `/dev/null` and takes the second as one that wrote, so no failure exists to see. The sentences now say what it can see, a write that fails, and what it cannot, a test holding the limit. Round 4's row stands as written, this one correcting it |
| R5-2 | defect | a third route: `check`'s last line goes straight through the line buffer, its error dropped, leaving nothing for `main`'s flush to fail on: 91 of 463 cuts truncated with exit 0 | Routes were the wrong shape. `Watched` remembers any write that failed and the executable reads it, so there is one route; `emit` no longer says it itself, so it is said once. A test cuts three commands' reports at every byte |
| R5-3 | defect | `Place::open` called only a name too long `usage`; a link to nothing as a parent, a closed parent and a stage lost to a parallel run were `tool-failure`, against the chapter's row 3; `resolve --out` at `.`, an empty name, `q/` or a directory likewise | Whatever stops the place opening is `usage`, its repair by its cause, and a parent's failure names the parent. `resolve` refuses what names no file, and a directory, before it writes |
| R5-4 | gap | a path with room for its stage and not its files was refused late, with false hints, one inviting a bogus bug report | Held before anything is made, against the longest file a build writes and in absolute form, as the checker is handed it. What a module's own long name brings is `usage` when written. A test derives this host's limit, so it holds on either |
| R5-5 | defect | a carried name silently replaced what was put in the stage, `mine` gone with exit 0, and a `target/mine.txt` in the stage is carried on, not named as the chapter said | A carried name that would replace something stays at `DIR.replaced` and is named. The chapter says what lands under a carried name is carried on. A test for each |
| R5-6 | wording | "what came to `DIR.replaced`… nothing is placed" is false where nothing stood at `DIR`, which never uses that name | Said as it is, in the record and the chapter, a test holding the package placed and the next build naming it |
| R5-7 | wording | "from the moment it is made" and "stays private throughout" were false at both ends: a stage at the umask's mode until a second call, seen at `755` in 417 of its samples over a `700`; a mode read before the run and never held again, so a directory closed meanwhile reopened | Both made true rather than reworded: the mode goes to the call that makes the stage, and is held again just before the first rename, the stage closed as it is by then. A test reads the mode while the checker runs, and closes a directory during a run |
| R5-8 | gap | `.part` standing refused with no repair, stopping every later `resolve` after a killed one, and `unwritten` with none, against §5.5 | Each has its repair; the `.part` refusal says what may have left it |
| R5-9 | gap | a refused `resolve` left the parents it made, as `build` no longer does; "what already stands at that name" read as `--out` is false, here and in round 4's row | Parents gone on every way out. `write_whole` names `PATH.part`, and says a plan replaces a file or a link standing at its own name, as an output named by the user does; a test holds that |
| R5-10 | wording | hints not true of their routes: a flag on `--out` told to write its parent; a file as a parent told to find room; `chmod 600 d; build --out d` exit 0 with a `next:` line cargo cannot follow, then "not an earlier build" of one that is | Hints by the error's cause. A directory this user cannot read and search is refused before anything, saying `chmod u+rx`, found by trying rather than by bits, so it holds for a group's directory and for root. A rename's hint lists what to check. Found beside it: at `555` the `next:` line could not run, cargo exiting `101`, and the command now says to open it first |
| R5-11 | gap | `what_stays_of_an_earlier_build_keeps_the_mode_it_had` asserted what stays only if something stayed, excused by running as root | `modes_bind` tries a closed directory, so what stays is required unless a mode binds nobody; `what_stands_in_the_way…`, which held the same excuse, likewise |

Said, and left as it stands: the mount point, a directory owned by another user and a failing `set_permissions` stayed unforced, as the reader
could not run them. Cargo's own deeper paths under `target/` may still overflow a path a build fitted: that is cargo's, at `cargo run`.

**Round 6** — `2026-11-05`: not acceptable, 17 findings, of which R6-1 needs no other program and blocked later builds, and R6-4 touches a
user's bytes but needs another program writing in the stage. Its reader ran 92 commands against four kinds of stream, 1,365 truncation
points, 48 modes, 240 builds under three umasks with about 7 million polls of the stage, 56 place cases beside the earlier rig's 51 and 25, and
300 parallel builds. R5-2, R5-5 and R5-7 held, R5-1's limit is as stated, and a failing `set_permissions`, which round 5's history said stayed
unforced, was forced in 40 racing runs and said each time.

**The method changed with this round.** The rounds found 15, 12, 9, 5, 11 and 17: not converging, and most of round 6 was one shape, two
commands reading a place differently and one rule told in several sentences that had drifted apart. So besides answering each finding, every
route of `--out` is now a row of a table in the book that a test runs, both ways, and the record and the code state no route. A later finding
of this kind lands on a row and a fixture, not on another sentence (`decision_executable-design-reviews.md`, items 3 and 5). The closure rule is
unchanged.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R6-1 | defect | older than this commit: a standard error nobody reads failed the sibling's relay, so an accepted build stopped, left its stage and blocked every later one; what a failing standard error does varied by command | The relay goes on stream by stream and decides nothing: the program has judged. `report` returns the refusal's own status. So a standard error nobody reads changes nothing, for every command, and the chapter says so. A test of the real relay, and one of the build |
| R6-2 | defect | the path probe read `std::path::absolute`, which keeps `..`, where the checker is handed a canonical path: `--out ../../../out` from a long working directory refused, and named absolutely built | Judged once the parents stand, canonically, by `place::room`, as `consume::absolute` names it. A test from a deep working directory |
| R6-3 | defect | `resolve`'s `PLAN.check` lay outside its place: refused after the plan landed, as `tool-failure`, with false hints | The place is judged whole before anything moves, `.check` and room for its record among it. Rows `check-stands`, `check-stands-unchecked` and `no-room-check` hold the earlier plan untouched |
| R6-4 | defect | what is put in the stage at a name the package writes was silently overwritten, and a link there written through, for the checker's whole run | The stage is private, `700`, so only the build's own user can reach it, and its files are created and its directories walked, never written over or through: what came is refused and the stage kept, what came in it. Rows `filled-in-stage` and `linked-in-stage` |
| R6-5 | defect | `resolve` took `missing/../d`, which `build` refuses for this very reason | Both read a place by `place::read`. Row `back-through` |
| R6-6 | wording | `resolve` with a file as a parent was told of a link to nothing | The same order as `build`: what goes beside it is read first, so it is `NotADirectory`. Row `file-on-the-way` |
| R6-7 | wording | "leave room in its name" was false of a whole path too long and of a parent's name | The error cannot tell them apart, so the one repair names each cause and what to do |
| R6-8 | gap | only `--out` was probed: a description too long from where it was named gets "report it", a bogus bug report | `consume::absolute` refuses `usage` where what it names has no room, and where it cannot be named at all. A test from a deep working directory |
| R6-9 | wording | "is not an earlier build" of one with a directory unreadable one level down | Reworded to what is known: it cannot be told to be only what a build wrote. The repair adds opening it. Row `closed-within` |
| R6-10 | wording | "cargo cannot run in it until you open it" is true only of a package with nothing carried | It says what cargo needs, and to open it if cargo stops on that |
| R6-11 | wording | `unwritten` spoke of a report and a package to `archogen mcp`, which has neither | The sentence says only what holds of every command |
| R6-12 | wording | `lib.rs`: "Errors from the writers are reported as `Status::ToolFailure`", false since `Watched` | Said as it is, and a test holds a library caller's verdict through a stream that fails |
| R6-13 | gap | tests and comments holding less than stated: "not named" unasserted, no link in `resolve`'s test, the record's room miscounted, "`check` when the executable flushes" | Asserted, a row `over-a-link`, the arithmetic right at `limit - 30`, the comment corrected |
| R6-14 | gap | older than this commit: `\|\|` in the relay lost what the sibling said on a working stream when the other failed | The same answer as R6-1, a test of each direction |
| R6-15 | gap | a mode closed to reads while the build ran gave a `next:` line cargo could not follow, and no word | The line reads all three of the owner's bits |
| R6-16 | gap | "A directory keeps its mode", and nothing else said: an access control list lost, a group likely lost, a private plan opened | The package keeps the group too, a test where a group can differ, which this volume cannot show. The chapter says a list, an owner and a flag are not kept. A plan takes the mode of the file it replaces |
| R6-17 | wording | `chmod u+rx` cannot open a directory an access control list closes | The repair says "if its mode is what closes it" |

Said, and left as it stands: a mount point, a full disk, a read-only filesystem and another user's directory stayed unmeasured, as `hdiutil` was refused and no second account exists. A write returning `Ok(0)` is unverified, no real stream being known to do it.

**Round 7** — `2026-11-05`: not acceptable, 22 findings. The rows held as written, each one this reader ran behaving so: the method's failure was the question no test can answer, the routes no row named. Four lose a user's bytes with no other program acting, R7-1 the worst: `resolve --out` naming its own description destroyed it, at exit 0 under `--unchecked`. R6-1, R6-2, R6-4 for the front's writes, R6-5, R6-6, R6-14, R6-15 and R6-17 held. Its reader built a release of `b48408e9` in a checkout of its own, the trust inventory refusing `cargo xtask install` there because the main repository's `.cargo/config.toml` lies above a nested tree, so its binaries did not pass that inventory, and it says so. R7-1, R7-2, R7-3, R7-8 and R7-21 were measured again on that install before they were answered, and each behaved as the reader said.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R7-1 | defect | older: `resolve --out` naming its own description, or a module it imports, destroyed that file, through a link too, at exit 0 under `--unchecked` | A plan is refused where `PLAN` is a file the description was read from, by device and inode. Rows `its-own-description`, `its-own-description-by-a-link` and `a-module-it-imports` |
| R7-2 | defect | older: a `PLAN.check` directory that cannot take the record lost the earlier plan, then refused with a false "disagree" | `resolve`'s order changed: the checker judges `PLAN.part`, and it lands only on acceptance. Row `no-record`. The "wrote no record" repair now says to look where the record goes first |
| R7-3 | defect | a regression of R6-16's answer: a mode closed to the owner's reads landed the plan where the checker could not read it | Written at `0600`, the file's mode and group given only as it lands. Row `over-a-file`, at four modes, `000` among them |
| R7-4 | defect | "never lands to be refused after" was false wherever the checker could not judge | The same order. Rows `no-checker`, `reads-once` and `refused` hold what stood at `PLAN` untouched |
| R7-5 | defect | older: the checker's own `fs::write` went through a link at its `.part`, writing outside the stage | It creates that file new, and refuses what stands there: a test of the real executable. The record's own name stays a rename's to fill, the one name that does, and the chapter says what comes there is replaced. Needs another program |
| R7-6 | defect | older: the stage swapped for a link while the checker ran landed a link at `--out`, and removed the earlier build | The stage is held by what it is, device and inode, before anything is written in it after the checker and again before it lands. Row `stage-swapped`. Needs another program |
| R7-7 | defect | row `disagreed` claimed words the record-unsound half never says, and its fixture ran only the first half | Split: `record-unsound` and `disagreed`, each with its own fixture |
| R7-8 | defect | a description that reads only once reached a bogus report, or hung for ever on a pipe | Refused before the checker runs, as `usage`: it reads the description again. Row `reads-once` in both tables. `--unchecked` runs no checker and is unaffected |
| R7-9 | wording | two repairs wrong under a closed ancestor: `TOO_LONG`, and "check that it exists" of a file just read | Each by the error's kind |
| R7-10 | wording | "mode `700`, from when it is made" was false where nothing stood, by an instant measured at 981 of 1,422,114 polls; an inheriting list is no mode | The chapter and the record say both. The instant is empty, and no wider than what the package keeps |
| R7-11 | route | landing refused, a parent closed meanwhile left a stage unsaid, blocking the next build | Row `parent-closed-meanwhile`, and it says the stage could not be removed |
| R7-12 | route | `ok` with something left aside that then blocks the next build | Row `stays-aside` |
| R7-13 | route | an earlier build with a listed file gone, or another format | Rows `listed-gone` and `other-format` |
| R7-14 | route | a link to a directory, and a loop, on the way | Rows `link-to-a-directory-on-the-way` and `link-loop-on-the-way` |
| R7-15 | route | `--out` swapped for a file, and an empty one filled, meanwhile | Rows `swapped-meanwhile` and `came-in-empty` |
| R7-16 | route | the stage gone got a bogus report; parallel `resolve`s likewise | Row `stage-gone`, with the repair of R7-2 |
| R7-17 | route | failures with no row | Row `no-checker`. A failing write and a module's long name stay untabled: nothing portable forces them, and the rig has measured both |
| R7-18 | route | the plan table's omissions: a parent closed, a pipe at `PLAN` replaced, a reused `.check`, a link there, a stale "ok" record left by `--unchecked`, `q/.` | Rows for each. A pipe is refused, not replaced, and `--unchecked` removes the record it would otherwise stand beside |
| R7-19 | route | `--out --unchecked` takes a flag as its value | Row `a-flag-as-its-value`, by design, as the parser says |
| R7-20 | gap | fixtures weaker than their rows: `over-a-file` at one mode, `edited` and `closed-within` asserting one thing of "all of it", launchers with no `.part` | Four modes; a snapshot held whole; the `.part` route a test of the real checker |
| R7-21 | gap | older: a `next:` line unquoted, which a shell cannot follow in a directory with a space | Quoted where it needs to be, a test holding it |
| R7-22 | wording | sentences against rows: step 3's "whatever stops it" against a flag on `--out`, step 4's "so `tool-failure`", "every route", and §5's "what the build's own user fills is refused" against `came-in-stage` | Each said as it is. "The routes" replaces "every route", and the chapter says a route no row names is promised nothing |

Said, and left as it stands: a mount point, a real full disk, a read-only filesystem, another user's directory and names that are no text stayed unmeasured. A plan's group cannot be seen to move on this volume, owners being disabled. Its reader, its first rig still pointed at round 6's directory, created `target/m4237/run/a1` there against its brief, deleted exactly that and said so; nothing tracked moved.

**Round 8** — `2026-11-05`: not acceptable, 19 findings, 9 defects, each with no other program acting; R8-1 to R8-4 can lose a user's bytes. Most trace to one cause: `resolve` decided what it might replace or remove by asking whether a thing was what the description was read from, and each way of asking had a door. Its reader ran about 216 commands on an install of `b32a4421`, and 29 of 30 and 42 of 43 rows held, the two exceptions words only. R7-2, R7-3, R7-5, R7-6, R7-8 for a real pipe, R7-11 to R7-16 and R7-19 held.

**So the answer is one principle, and less code.** An output replaces only what its own kind wrote, which is what `build` already did: `same_file`, which round 7 added, is gone. Everything a run writes stays in one private stage until it lands, as a build's does.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R8-1 | defect | older: `plan-check --result` naming its own description or plan destroyed it at exit 0 | A result replaces only a record or an empty file, refused `usage` before anything is judged: a test of the real executable, over a description, a plan and a file of mine |
| R8-2 | defect | a description standing at `PLAN.check/plan-check.json` was deleted, with a false reason | A record replaces only a record. Rows `record-no-record` and `record-no-record-unchecked` |
| R8-3 | defect | `--unchecked` deleted through a link at `PLAN.check`, and a user's file of the record's name in a real directory | It clears only what reads as a record, through no link. Row `check-a-link-unchecked` |
| R8-4 | defect | `/dev/fd/N` and `/dev/stdin` passed both of round 7's guards: `--unchecked` replaced the description, and a regular file read twice gave a bogus report | What stands at `PLAN` is no plan however the description is named: row `its-own-description-by-a-descriptor`. The front reads the description again as the checker will, and refuses `usage` where it reads differently |
| R8-5 | defect | a run that judged nothing deleted the accepted plan's record | The record lands from the stage, so nothing beside `PLAN` moves until then. Rows `reads-once` and `no-record` hold it |
| R8-6 | defect | a landing refused after acceptance left an `ok` record of a plan that exists nowhere | The record goes first, under a name of its own, and only takes its name once the plan stands; a landing refused puts everything back. Row `closed-meanwhile` |
| R8-7 | defect | a record that could not be removed read as one that had been written, so `.part` stayed for a report that did not exist | There is no earlier record in a stage: the test reads the stage |
| R8-8 | defect | `--unchecked` over a closed `.check` left the earlier `ok` record, silently | Moved aside before the plan lands, or the command refuses and nothing moves |
| R8-9 | defect | a failed landing removed a user's empty `.check` | Only one this run made goes |
| R8-10 | route | what comes during `resolve` | Each path held again before it lands, and what lands is what was judged, read back and compared. The limit is stated in `cli.md`. Needs another program |
| R8-11 | route | the plan table's omissions | Rows `link-to-a-directory-on-the-way`, `link-loop-on-the-way` and `record-unsound`. A flag on `PLAN` is not portable to force, and the rest of its list follows from the principle |
| R8-12 | gap | both `no-checker` rows quoted words the real command never prints, one from a test's fake | Each row runs the real executable, and quotes it |
| R8-13 | gap | fixtures weaker than their rows | `closed-to-writes` at both modes, `check-stands-unchecked` and `check-a-link` asserting what they say, `record-unsound` the assignment; `no-record` holds the earlier record |
| R8-14 | gap | older: a `next:` line with a leading dash read as a flag | `followable`, a pure function with a test of each case |
| R8-15 | wording | "each path is held again" was false of `resolve` | Now true of it, and `cli.md` states the limit |
| R8-16 | wording | "Every route" and "every way" against "a route they do not name" | Reworded |
| R8-17 | wording | `resolve_cmd.rs`'s stated order stale, and `check_plan`'s `# Errors` | Rewritten to the design |
| R8-18 | wording | "unless the build knows something else wrote there" against rows that keep a stage for a report | Says what the table says |
| R8-19 | wording | `room` called every error too long; the checker's error named the wrong file | Each repair by its kind; it names the `.part` |

Said, and left as it stands: a group moving, a real full disk, a read-only filesystem, a mount point and another user's directory stayed unmeasured. A failed `chown` of a plan's group is now a warning, the plan standing, as a build's is: reading only, on this volume.

**Round 9** — `2026-11-05`: not acceptable, 16 findings, 6 defects, R9-1 and R9-2 losing a user's bytes and each needing no other program. The principle's main claim held: no description or module can be mistaken for a plan. Its reader ran about 530 commands on an install of `7cf7a353`, and every row of both tables matched on the real command. R8-1, R8-2, R8-5, R8-7, R8-9 and R8-4's first half held. R9-1, R9-2 and R9-4 were measured again on that install before they were answered, each behaving as the reader said.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R9-1 | defect | any file beginning with the 29 bytes of a plan's first line counted as a plan and was replaced at exit 0, a plan with three weeks of notes in it among them | A plan is known by how it reads: text that begins so, every line one form, within a bound. So notes, prose and binary after the prefix are no plan. Row `a-plan-with-notes-in-it`. An edit within a form is not seen, and the chapter says so |
| R9-2 | defect | a failed landing under `--unchecked` deleted `plan-check.json.part`, which this run neither wrote nor checked, through a link too | `land` now tracks what each step moved, and undoes only that. Row `flagged-plan-unchecked` |
| R9-3 | defect | `--unchecked` over a `.check` that cannot be searched left the earlier `ok` record beside a plan it did not judge, at exit 0: an error was read as "no record" | `reads_as_record` returns the error, and an unanswerable question is a refusal, nothing moved. Rows `check-unsearchable-unchecked` and `record-unreadable` |
| R9-4 | defect | a plan could land and then be refused, a record's name flagged leaving the new plan beside the earlier record | The record takes its name before the plan moves, the earlier one held aside to go back. Row `flagged-record` |
| R9-5 | defect | a description at a shipped kind module's path was falsely refused, the search finding the kind module first | The comparison is of every source against its own name and text, a kind module passed over by both |
| R9-6 | defect | older: the guards covered the description and not its modules: a module that is a pipe hung, and a link to a descriptor gave a bogus report | Every source held to being a regular file that reads the same. Row `a-module-that-reads-once` |
| R9-7 | gap | a whole file read just to refuse it: 1 GiB sparse, 1.08 GB of memory | Each read within its bound |
| R9-8 | gap | a record did not keep the mode of what it replaces, though a plan does | It does, private until it lands, for `--result` and for `PLAN.check`. Row `a-record-keeps-its-mode` and a test of the real executable |
| R9-9 | gap | `--result` took any error as absence, then exit 70 after the whole judgement, and a stale `.part` blocked with no repair | Only a name found absent is absent; a parent that is no directory and a stale `.part` are `usage` before anything is judged, a test of each |
| R9-10 | gap | not reachable yet: a record of another format called "no record" | "No record this engine reads", in both places |
| R9-11 | gap | fixtures weaker than their rows | `check-reused` from an earlier record of another description; `check-a-link-unchecked` holds the link; `parent-closed-meanwhile` the words |
| R9-12 | route | "reads differently" had no row and no test | Row `reads-differently`, where the platform's `/dev/stdin` reads so |
| R9-13 | route | `.part` waiting, `.check` closed, a flag on `PLAN` | Rows `a-record-waiting`, `check-closed`, `flagged-plan`; a waiting name is now found before the checker runs |
| R9-14 | route | a stage swapped or holding a stray file left a `.part` unsaid | Held by what it is, as a build's is, and it says what stays. Rows `stage-swapped` and `came-in-stage` |
| R9-15 | wording | `cli.md` and §5 disagreed on an empty record; "lands only on acceptance" ignored `--unchecked` | Both said as they are |
| R9-16 | wording | the `--result` rule was only in §5 and an error message | In `plan-check.md` and `--help` |

Said, and left as it stands: a real full disk, a read-only filesystem, a mount point, another user's or a sticky directory, a group moving, and Linux's `/proc/self/fd` route stayed unmeasured. A module edited in the instant between the front's reading it again and the checker's starting is the stated limit's.

**Round 10** — `2026-11-05`: not acceptable, 13 findings, 2 defects, the fewest yet. Its reader ran 334 descriptions checked and unchecked, 190 plans of 1,415 lines each resolved over itself, and 1,330 resolves over seven ways of naming a description, and found `is_a_plan` refusing no plan the engine writes and `reads_again` nothing it should accept. All 53 plan rows and 43 build rows held on the real command.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R10-1 | defect | `is_a_plan`, round 9's answer, looked only at how a line began and ended: a comment after a form, `(note …)`, two forms on one line and prose in parentheses were all replaced at exit 0 | No shape can answer this, so it no longer tries: `archogen_api::is_a_plan` asks the generator's own reader of the format, `eadl_resolve::plan::read`, which takes only the canonical text. Row `a-plan-with-notes-in-it` runs seven shapes, and a test re-resolves every plan the corpora write. A plan of another format is now no plan either, as a record of one is no record |
| R10-2 | defect | older: the stage swapped for a real directory was removed on the refusal route, with what it held: `abandon` checked nothing. Needs another program | `abandon` holds the stage by what it is, in both commands. Row `stage-replaced` in both tables; the race on the real command kept what came in all 46 runs where it acted |
| R10-3 | gap | `--result` naming no file, or in a closed parent, exited 70 with no repair | Naming no file is `usage`; the other says what to check |
| R10-4 | gap | an empty record at a closed mode was taken unread, and the new record kept that mode | It is read like any other, and refused: row `record-unreadable` |
| R10-5 | gap | a module with no room in its absolute path reached a bogus report | Every source is named as the checker will be handed it. Measured on an install with the reader's two files from a 1,012-byte working directory: `resolve` and `build` exit 2 with a true repair, `check` and `--unchecked` exit 0. No row, its fixture needing a tree of its own |
| R10-6 | gap | `--unchecked` refused a name with no room for a `.check` it does not need | A name too long to stand holds no record, and no checker's name needs room without a checker. Row `no-room-check-unchecked` |
| R10-7 | gap | `Built::edited` read a whole listed file to refuse it | Past the most a build writes it is changed, and is not read. A test at a sparse terabyte |
| R10-8 | gap | a failed restore of the earlier record was dropped. Needs another program | Said, naming where it is |
| R10-9 | gap | fixtures weaker than their rows | `flagged-plan-unchecked` holds the plan, `came-in-stage` the record and what the next run says, `disagreed` the assignment |
| R10-10 | route | `build` had no rows for what `cli.md` gained | Rows `reads-differently` and `a-module-that-reads-once`, and step 4 says modules too |
| R10-11 | route | what happens during `resolve` | Row `plan-edited-meanwhile` |
| R10-12 | wording | "private until it lands" was false for a record in `PLAN.check` | Made true rather than reworded: it takes its mode in the stage, before it moves |
| R10-13 | wording | "A build replaces exactly a build" against row `empty`; "known by how it begins" in a test | Both corrected |

Said, and left as it stands: a real full disk, a read-only filesystem, a mount point, another user's or a sticky directory, a group moving, and Linux stayed unmeasured.
