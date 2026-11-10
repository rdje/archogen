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

**Round 11** — `2026-11-05`: not acceptable, 9 findings, 4 defects, none needing another program; only R11-1 touches a user's bytes, and R11-2 is a regression from round 10's own answer. Its reader ran every row of both tables on an install of `7a4155a0` (56 of 56 and 46 of 46 held, what is left included), 240 descriptions with odd names at 7 commands each, 212 descriptions from the tree at 6 commands each, 5,954 mutants of a plan, and all writes failing under `ulimit -f`, every earlier output kept. What broke was what the words claimed, not what the routes did.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R11-1 | defect | a hand edit that keeps the canonical form is replaced at exit 0, and the row, `cli.md`, §5, the API's doc and the fixture's comment said the opposite. Round 10's edit deleted the chapter's honest sentence | Nothing can tell it: canonical text carries no digest of itself. So it is stated in each place, held by a new row `a-canonical-edit`, and the chapter says where a choice belongs: the description. The behaviour stands |
| R11-2 | defect | R10-7's 64 MiB bound called itself the most a build writes, and a description past it made a build refuse its own output and `explain` give a false `stale-input` | The bound is gone. Format `/5` gives each listed file's length, so another length is changed and is not read, whatever it has grown to; the same length is digested as it is read, at no cost in memory |
| R11-3 | defect | "far more than any plan holds" and "larger than any build's" were false: a plan of 17.6 MB and a provenance of 17.5 MB, each the engine's own | No bound is claimed of what the engine writes. Only what begins as a plan is read whole, up to what its reader can hold, 2^32 bytes, row `a-plan-past-what-its-reader-holds`; a provenance is read to 2^30, and the refusal says this engine's limit |
| R11-4 | defect | `archogen-plan-check` panicked on an argument that is no text, exit 101 | Read as the front reads them: a usage error, exit 2 |
| R11-5 | gap | the checker wrote a record it would not read back past 1 MiB | One bound for a record, 2^30, in the front, `explain` and the checker; what does not begin as a record is refused for its first bytes |
| R11-6 | gap | most `--result` refusals gave no repair | Each says what to do, as `hint:` |
| R11-7 | route | three routes with no row, and a hint naming the record's mode where the directory's stops it | Rows `check-unsearchable`, `check-closed-unchecked` and `record-unreadable-unchecked`; the hint names both |
| R11-8 | gap | a umask closing the owner's reads was reported as "changed while the plan was judged", and the stage stayed | A failed read is said as what it is, naming the umask, and the stage goes. A test runs the executable under `umask 0477` |
| R11-9 | wording | `engine-api.md` said the API's version is `1.3` | The figure is gone: the chapter points at the register |

Beside these, the reader's unverified remark: `Landing::open` left an empty stage when its mode could not be set, where `build_cmd.rs` removes it. Mirrored; still not forced, since nothing here makes `chmod` fail for an owner.

Said, and left as it stands: a real full disk, a read-only filesystem, a mount point, another user's or a sticky directory, a group moving, Linux, and a working directory whose name is no text stayed unmeasured. R10-8's failed restore was not forced: it needs a race.

**Round 12** — `2026-11-05`: not acceptable, 7 findings, 1 defect, of little harm and older than the commit, touching no bytes and needing no other program. Its reader ran some 2,550 commands on an install of `a64d9c7b`: every row of both tables, 61 and 46, on the real command; format `/5` true for 1,469 listed files in 122 builds, non-ASCII, NUL and no final newline among them, and 156 tamperings refused by `build` and `explain` alike; each `--result` hint followed through 51 commands; and every answer of round 11 held, but for the two sentences below. An earlier reader of this round was cut off by the incident on the host of `2026-11-05`, and left no report. The reader said the defect is one by "leaves a false verdict" and asked that the call be re-weighed: it stands, since the exit status itself is false and it proved that.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R12-1 | defect | `explain` called a failed read a stale build: `chmod 000 x/plan.txt`, `chmod 000 x/rt-core` or `chmod 300 x` gave exit 3, "is not as it was written", and its hint failed, while opening it again gave exit 0 with no rebuild. The class of R9-3 and R11-8, and the same hint failed on a build truly changed | `Built::edited` and `foreign` say which: changed, or unread. Unread is `usage`, with `open what it could not read`, as `build` says; a change stays `stale-input`, and is told a new directory, which works. A test holds each shape and follows each repair |
| R12-2 | gap | a umask was named only on `--unchecked`: by default the same cause gave exit 70 and a hint at a directory already gone, and a umask closing writes was not named | The checker's hint and the build's name it, and say the directory is one this run removes. No state was left in any of its 96 runs |
| R12-3 | gap | a build of an earlier format was told the truth and a repair that did not help, and the one that does was never said | `Unbuilt::OtherFormat` has its own: a new directory or that build taken away, from `build`; a new build with this engine, from `explain`. A test follows it |
| R12-4 | gap | fixtures weaker than their rows: `record-unreadable` and its `-unchecked` held no record, and the links and `no-name` held nothing | Each now holds what its row says is left |
| R12-5 | route | two routes with no row: a provenance past 2^30, and a flag on `--out` | Rows `a-provenance-past-what-it-reads` and `flagged`, each a fixture. The first read a gibibyte to find out, so it is now told by its length, unread: a sparse 1 TiB costs nothing |
| R12-6 | wording | `RECORD_AT_MOST`'s comment still said "any this engine writes", and a file of exactly 2^32 bytes was refused as "larger than" that | The comment claims no bound, and the message says 2^32 bytes less one |
| R12-7 | wording | three sentences still promised that a line the search would not write makes no plan, while `a-canonical-edit`'s own fixture adds one and is replaced at exit 0 | Each says what is true: out of its form or sorted place |

Said, and left as it stands: its reader could not run a prefixed file just under 2^32 or a real plan past it through the checker, which would load gigabytes; a real provenance or record past 2^30 from the engine; a group moving, this volume being mounted `noowners`; a working directory whose name is no text, which APFS refuses; a real full disk, a read-only filesystem, a mount point, another user's or a sticky directory, and Linux. The guard stopped one rig at 22:43:22 for swap growing on the host, and it reran the half that mattered and not a rebuild at 200 services with 100,000-byte names. One `grep -l` over earlier rigs printed file names from `m4243`, off its list: names only, nothing used. No sealed case read, no history or web searched.

**Round 13** — `2026-11-06`: not acceptable, 10 findings, 3 defects, of which R13-1 is a regression from round 12's own answer and R13-2 alone touches bytes, needing another program within microseconds. Its reader ran 1,916 commands on an install of `e3232b6f`: every row of both tables, 48 and 61, on the real command; 168 states of a build by `explain` and `build`, 66 real changes and 80 closures among them; 16 masks over 3 routes, 336 commands, with nothing left behind. Round 12's answers held but for what is below.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R13-1 | defect | a real change was called unreadable: only `NotFound` counted as gone, so a listed directory replaced by a file, a fifo, a link to a file or a link loop exited 2 with a repair that cannot work. A regression from round 12's answer | `says_it_changed`: gone, a name on the way now no directory, or a loop of links. A test holds each shape and follows the repair |
| R13-2 | defect | a run deleted what it did not write: `place_record` removed `plan-check.json` from a stage with no hold on it, so a stage swapped meanwhile lost that file, 23 runs of 240. Needs another program in a window of microseconds | Nothing earlier can be this run's in a stage it made, so the removal had no job, and is gone: what stands there is said, and left. The test that pinned the removal, `an_earlier_record_is_removed_…`, was the old reading, and is replaced by `a_record_standing_in_a_stage_is_refused_and_left` |
| R13-3 | defect | the hint for a closed build did not say to open it: a closed top directory or provenance went through `NoBuild`, so `explain` and `build` disagreed on the repair | `Unbuilt::Closed`, read as `Unread` is |
| R13-4 | gap | a umask closing the owner's writes, on `resolve`'s default route and on any parents a run makes, was never named, and its repair was false | `place::repair` names it |
| R13-5 | gap | fixtures weaker than their rows: `no-name` and `empty-name` held nothing, no out-of-order line was run, and `a-canonical-edit` only added a line | Each holds what its row says |
| R13-6 | gap | `plan-check` gave no repair for a plan or a description it could not read | `refuse_with` |
| R13-7 | route | a closure during the run, the parent or the stage at 000, had no row, and was told to report it as a disagreement | Row `closed-while-it-ran`, and it says what happened |
| R13-8 | route | the plan in `PLAN.part` gone or replaced during the checker: told to check the umask, and the stage removed with what came | Row `plan-gone-meanwhile`: only a closed plan reads as the umask, and what came stays |
| R13-9 | wording | "which this run removes after it" could be false, where another directory came to stand there | Said so |
| R13-10 | wording | `reads_again` called a read that failed a difference | It says it cannot be read now |

Beside these, its smaller instance of R13-1: a build moved to a 1,004-byte path gave `File name too long` and a hint to open something, where `explain` from beside it exits 0. The hint now names that too.

Said, and left as it stands: its reader could not run a prefixed plan just under 2^32, a provenance at exactly 2^30, a real plan or record past their bounds, a file growing while it is read, a real full disk, a read-only filesystem, a mount point, another user's or a sticky directory, a group moving, or Linux. By reading alone, `Built::remove`'s `read_to_string` of the provenance and `replaceable`'s `read_to_end` hold no bound against a file growing meanwhile: not called a defect, needing another program. It stated that it read `git show 4db9d4db` for paths in `crates/` and `docs/book/`, and that one `grep -l` across earlier rigs printed a name from `target/m4261`, off its list, and nothing from it was used.

**Round 14** — `2026-11-06`: not acceptable, 10 findings, 3 defects, after 1 and 3. Its reader ran about 2,600 commands on an install of `ffb11307`: 49 and 62 rows on the real command, 270 states of a build by `explain` and `build`, 16 masks over 5 routes, 60 rounds each of three builds and three resolves at once, with 60 winners each and nothing left. Round 13's answers held but for what is below.

A run deleting what it did not write has now come back under a new name in five rounds, each answered by a new check of a new shape. So this answer is one rule in one place: `place::Written` records what a run writes, and a refusal removes that much, each file only if it is still as written and reached through real directories, then the directories and the stage where that leaves them empty. Nothing else goes, whatever shape it came in, and `place::say_it_stays` says of any stage that stands that the next run refuses while it does. The next reader should try to find a way for a run to remove what it did not write, and a stage left with nothing said.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R14-1 | defect | `resolve` deleted what came at the plan's name in two shapes, against the row and R13-8's answer, and told the second to check a umask that cannot be the cause on a checked route. Needs another program, in a private stage | The rule. A read-back that fails on a checked route is what came, since the checker has just read it, and only with `--unchecked` can it be the umask |
| R14-2 | defect | a link where a listed directory stood, pointing at a closed directory or too long to follow, was called unreadable, exit 2: the class of R13-1, 30 of 198 changes | `place::way_to` never follows a link: one on the way is a change, whatever it points at. A test holds both shapes and each repair |
| R14-3 | defect | the row `closed-while-it-ran` said the run says the stage could not be removed, and it did not, its fixture weaker than its row, and "run this again" was then refused | `say_it_stays`, and the fixture holds the words. The repair no longer says to run it again |
| R14-4 | route | a stage closed during the checker had no row in `plan-routes`, and `abandon` failing was unsaid on most routes | Row `closed-while-it-ran` in that table too; `say_it_stays` on every refusal that leaves one |
| R14-5 | route | what came beside the plan went on a refusal, unsaid in `cli.md` | It stays now, by the rule: row `came-on-a-refusal` in both tables |
| R14-6 | gap | `plan-check` gave no repair for a module it cannot read, and a non-UTF-8 description was told to name a file this user can read | A repair for each, naming UTF-8 |
| R14-7 | gap | within 16 bytes of the path limit `explain` of a build was told to give a build | `Unbuilt::Closed` holds a path past what can be named, whose hint says to ask from nearer |
| R14-8 | wording | `explain p/b` under a closed parent was told to give a description | Told to open it |
| R14-9 | wording | two comments still said an earlier record is removed | Each says what is true |
| R14-10 | wording | "the plan and the record stay, for that report" followed a refusal with no report; a failed stat was called "no regular file"; a name was said twice | Each said as it is. A closed `assignment.eadl` said to have "come in the stage" is left: another program closed it there |

Said, and left as it stands: its reader could not run Linux, another user's or a sticky directory, a real full disk, a read-only filesystem, a mount point, a provenance at exactly 2^30, a plan just under 2^32, or a file name that is no text, this volume refusing one. It stated that it used `git show ffb11307` limited to `crates`, `docs/book`, `docs/decisions` and `docs/reviews`, and reused rigs from earlier readers listed in its brief. No sealed case read.

**Round 15** — `2026-11-06`: not acceptable, 9 findings, 5 defects, after 1, 3 and 3, of which R15-1 to R15-3 are regressions of round 14's own rule, each passing on `ffb11307` and the first two needing no other program. Its reader ran about 1,900 commands on an install of `1e71e822`: all 50 and 64 rows on the real command, 283 cases and 687 commands of `way_to` with no regression, 20 masks over 5 routes, 40 rounds each of three at once with one winner and nothing left. The tests stayed green with every defect present, so the rule's own routes were fixtures short of what can fail. That is the second round whose count did not fall, so this one answers by method: `crates/archogen-cli/tests/stage_left.rs` sweeps what a host can impose on the real command, every umask and file limit it tries by each way into a stage, and holds that nothing of a refused run stands, rather than naming another shape.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R15-1 | defect, regression | `resolve` left its stage after a failed plan write, and said nothing: `Written::note` ran only after a write succeeded, so the partial plan was unknown, and `Landing::open`'s way out never said a stage stands. Under `ulimit -f 0` `p.part/plan` stood, and the next run was refused | `Written::write` is the one way to write in a stage, and notes a file from the moment it stands, before a byte is in it. `write_plan`'s way out says what stands. The sweep holds it over each limit, `--unchecked` and made parents among it |
| R15-2 | defect, regression | `build` left the partial file and every directory `write_file` made for a file never noted, and then said it "holds what this run did not write", of what was all its own: umask 277, a file limit, a module's long name | The same: each directory a run makes is noted as it makes it, and goes on a refusal where empty. The sweep's 18 masks and 10 limits, and `a_name_past_what_a_path_holds_leaves_nothing_of_a_refused_run` |
| R15-3 | defect, regression | `resolve` deleted a record it had just said it did not write: `came_first` led to `abandon`, and `remove_record` removes whatever reads as one. 60 of 60 with a racer | A record is a run's only once its checker was handed the stage with nothing at that name, `Written::checker_ran`, which `check_plan` sets just before it launches. `a_record_no_checker_of_this_run_wrote_is_never_this_run_s_to_remove` holds it with a record that reads as one, deterministically, and no racer |
| R15-4 | defect, older in effect | a closed file of the same length was taken for what the run wrote, against "a run removes only what it wrote", and the file went with nothing said | A file is known by what it is: device and inode from the descriptor it was written through, its length and when it was last written. Another at that name, or one edited where it stands, is not its own, closed or not. `what_comes_at_a_name_a_run_wrote_stays_and_is_said_however_like_it_it_is` takes four shapes in both commands |
| R15-5 | defect, older | a stage behind a closed parent was unsaid: `say_it_stays` read "cannot tell" as "gone", against `place::free`'s "only a name found absent is absent" | Only `NotFound` is gone; otherwise it says the stage cannot be looked at and may stand. Rows `parent-shut-meanwhile` in both tables |
| R15-6 | route, older | a FIFO or a link in the stage was followed: `land` and `read_record` opened the path without looking, so a FIFO hung and a link left `--out` a link | `place::open_regular`: what is no regular file is refused, and never waited on or followed. `what_is_no_file_at_a_name_is_refused_rather_than_waited_on_or_followed` takes both at the plan's name and the record's. The look and the open are two steps, a same-user race in a private directory, and stated here |
| R15-7 | wording, older | "report it: the checker and the front disagree" where nothing disagrees: a stage swapped for a file, a checker ended by a signal | Each says what happened and what to do |
| R15-8 | wording, older | R14-10's answer was partial: "for that report" with no report, "no regular file" of a module removed mid-run, "open it" of a description removed | Each by what stopped it, `put_it_back`; a kept stage says what it holds with no report assumed |
| R15-9 | wording, older | a stage kept for a report did not say the next run refuses; a path past the limit got a hint that repairs nothing | Each does |

Said, and left as it stands: its reader could not run Linux, another user's or a sticky directory, a mount point, a read-only filesystem, a real full disk, using `RLIMIT_FSIZE`, which fails the same writes and not `mkdir`, a provenance at exactly 2^30 bytes, a plan just under 2^32, or a file name that is no text. It stated that it ran `git diff ffb11307 1e71e822` limited to `crates/`, and read `target/m4266/install` only to date its findings.

**Round 16** — `2026-11-06`: not acceptable, 9 findings, 2 defects, after 3, 3 and 5, neither touching a user's bytes nor needing another program, each narrow and each leaving a state that blocks the next run. Round 15's answers held on the real command but for what is below. Its reader ran about 10,000 commands on an install of `a6b6f6ec`: all 51 and 65 rows, with no difference in status or words; 24 masks over 5 routes with the real checker; 4,070 commands under injected full disk, read-only and I/O errors, with no panic and no wrong package or plan; the run killed at each of 505 call sites, `DIR` always what stood there, nothing, or the whole package; another program acting at every call, 1,200 commands, no planted file lost; three at once, 40 rounds each.
It injected those faults with a small interposer, `target/m4271/tools/fault.c`, changing no code of archogen's, which is a fault in the host and no mutant. It used `git log -S`, `git diff` and `git show` on `crates/`, `docs/book` and the record at commits past the fence, and read no sealed case.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R16-1 | defect, older | a `PLAN.check` that takes a new name and lets none go, a flag or an access list: `undo` dropped the failure of its `remove_file`, so the checker's record stayed at `plan-check.json.part`, unsaid, and blocked the next run | `undo` says what it could not take back, with where it is. Row `append-only-check` |
| R16-2 | defect | a real checker stopped by a file limit left its partial `.part`, and the front said the stage "holds what this run did not write" of what was all its own. The stand-in never writes that file. The limit must lie between the plan's size and the record's | What the checker was writing goes with its record, once it was handed the stage, if it reads as the start of one. `what_the_checker_was_writing_goes_with_the_rest` |
| R16-3 | route | from a plain shell `ulimit -f` kills by SIGXFSZ, so R15-1's words still reproduce there, the sweep's `trap` being what makes a limit fail a write. The next run's message and repair are true | A signal cleans nothing: stated in both chapters and the sweep's head, and held by `a_run_a_signal_ends_leaves_its_stage_and_the_next_run_says_so`, its repair followed |
| R16-4 | wording | "`p.part` stays, holding what came in it" where it is empty and could not be removed | "Could not be removed, or holds what came in it" |
| R16-5 | wording | repairs naming the wrong cause: too many open files or processes told to check the install; a flag told to check the umask; `reads_again`'s read branch | Each names what can be so, and the read branch follows the stat's |
| R16-6 | gap | `a_name_past_what_a_path_holds_leaves_nothing_of_a_refused_run` never refused, so its assert never ran, and the record cited it. The behaviour was right | It goes deep enough that `description/<name>` does not fit, and asserts the refusal |
| R16-7 | gap | two silent removals, failing only under a fault: the earlier record after landing, and a stage that could not be closed and then not removed | Each says so |
| R16-8 | gap | `reads_as_record` answered an I/O error as "not a record", against its own doc | Only an end of file is |
| R16-9 | gap | the resolve fixture for `parent-shut-meanwhile` was weaker than its row, no plan standing at `PLAN` | It stands one, and holds it |

Said, and left as it stands: its reader could not run a real full disk or read-only mount, `hdiutil` being refused here, filesystems other than APFS, Linux, another user, a sticky directory, a mount point, or names that are no text. On coarse file times an edit of the same length within the tick, closed, reads as a run's own: stated in the record. Its read-only test aimed `mkdir` at `/usr/r16*` and `/r16*`, which the system refused.

**Round 17** — `2026-11-06`: not acceptable, 10 findings, 4 defects, after 3, 5 and 2, R17-1 a regression of round 16's own answer and the only one touching bytes or needing another program. Its reader ran about 21,000 commands on an install of `55b9884f`: all 51 and 66 rows with no difference in status or words; 18 signals over 1,314 cases, each ending in a good run by following the next run's words; 24 masks; 248 cases of flags and access lists; 1,123 of injected faults; a marked file planted at each name at every call, 7,084 runs, 2,484 of 2,708 plants kept, 180 of the 224 lost being R17-1 and 44 the landing's stated window.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R17-1 | defect, regression | `remove_record` took whatever stood at `plan-check.json.part` for what the checker was writing, if it read as the start of a record: an empty file, a `{`, a whole record. 120 of 120 with a racer, unsaid. And `checker_ran` was set before the launch, even if the launcher refused | R16-2's answer is taken back: a run removes what it wrote, and no reading tells that part from what came. It stays, and `say_it_stays` now says a stage may hold what its checker left, which was R16-2's own complaint. `checker_ran` is set once the checker has run |
| R17-2 | defect, older | `archogen plan-check` starts the checker with no input, so a description named `/dev/stdin` read empty and got `stale-input`, whose repair cannot work | A stream that reads as nothing is refused as `usage`, by the checker, of a description and a plan. It failed closed: no false `ok` was reached. The chapter says so |
| R17-3 | defect, older | the checker's own `write` dropped the failure of `remove_file(&part)`, R16-1's class in the checker: the whole record stayed at `R.part`, unsaid, blocking the next run | It says so, with where it is |
| R17-4 | defect, older | `Place::open`'s no-room way out never said a stage that could not be removed | It says so, in the refusal |
| R17-5 | route, older | a module that is a pipe nobody writes hangs every command, `check` among them | Stated in `cli.md`: it waits as any reader of a pipe does. Nothing is left |
| R17-6 | route, older | `build --out --locked` had no row, where `resolve` has `a-flag-as-its-value` | The row, and a fixture in a process of its own |
| R17-7 | gap, older | the refusal from `past_ticks` had no repair | It has |
| R17-8 | wording, older | hints pointing at the wrong thing: a flag on `PLAN.check` told to look at what stands at the name; a file limit told only of disk room and the umask | Each names what can be so |
| R17-9 | wording, older | "nothing was judged, so run this again" beside "the next run refuses" | Run this again, once what is said to stand is gone |
| R17-10 | wording | "the next run refuses while it stands" was false of a record stranded at its own name, and of `.earlier` under `--unchecked` | Each says what it does: a checked run refuses, or nothing does |

Said, and left as it stands: its reader could not run a real full disk or read-only mount, other filesystems, Linux, another user, a sticky directory of someone else's, mount points, or names that are no text. By reading only: a file beginning as a record past 2^30 is read up to 1 GiB before it is refused. It stated that `git show a6b6f6ec:` went to three files under `crates/`, that a rig left two processes of its own, which it ended by pid, and that `ulimit -n 3` aborts before any of archogen's code runs.

**Round 18** — `2026-11-06`: not acceptable, 9 findings, 3 defects, after 5, 2 and 4. Its reader ran about 41,000 commands on an install of `7af2b955`: all 66 and 51 of 52 rows, three first differing by its rig's own mistakes; 4,825 injected faults, no state blocking the next run unsaid and every repair leading to a good run; 1,137 with the earlier state re-snapshotted, none changed; 967 kills at each call; 600 real concurrent runs at one output; 212 descriptions through all commands, 165 plans accepted. Real `cargo`, and a package at exactly 2^20 releases, 8,388,612 lines, exit 0.
It stated that a grep of its own ran over all of `docs/`, against its brief, so over `docs/evaluation/frozen/`, whose cases are out of the tree (`docs/evaluation/README.md`): it printed only a line under `crates/`, so no case was read and this is no exposure. It stated that a 64 MiB file of NUL bytes it expected to cost little reached 3.46 GiB, at 87% RAM, and the guard stopped it, which is R18-4.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R18-1 | defect, older | the checker's `--result` was judged once and never held again before its rename, so a file that came at `R` during the run was replaced at exit 0, unsaid, against "each path is held again just before it is used": 40 of 40 with a racer at 3 ms | What stands there is known at the judgement and looked at again just before the rename: what came stays, and the checker refuses, saying so. The instant between the last look and the rename is the limit `cli.md` states |
| R18-2 | defect, older | a name too long, under parents that do not exist yet, was told a stage stands that was never made: `rmdir` of that name answers too long, and only not-found counted as gone. R16-7's answer | A name too long names nothing made. The row's fixture holds it |
| R18-3 | defect | R17-7's repair could not work: every repeated duration is already a whole number of the tick, and it named deadlines, which have no say | It says what is so: the range from the finest period to the longest must fit |
| R18-4 | route, older | a description of junk got one diagnostic per illegal character, about 2 KiB a byte: 131 MiB for 64 KiB, and a danger to the shared host | `MOST_ERRORS`, where every pass records: a thousand, and one more that says the rest went unsaid. A file of forms still costs its forms, and a read has no size limit; the chapter says so |
| R18-5 | gap, older | the lowering's refusals printed no `hint:`, against "every refusal states what happened and what to do about it" | They do |
| R18-6 | wording | `append-only-check`'s row kept "the next run" after R17-10 | A checked run |
| R18-7 | wording, older | hints naming only disk room and the umask, under a limit or an inherited access list | Each names them |
| R18-8 | wording, older | "run it again" on the `archogen plan-check` route, while `R.part` stays and the next run refuses | Once what it left is gone |
| R18-9 | route, older | `cli.md` did not say what `building.md` says of what comes at the record's name while the checker runs | It does |

Said, and left as it stands: its reader could not run a real full disk or a mount it could fill, other filesystems, Linux, another user, a sticky directory of someone else's, names that are no text, a provenance or record at their bounds, or `disagreed` against the real checker. A descriptor on a regular file at its end still gets `stale-input` or `tool-failure`, failing closed, which it found acceptable.

**Round 19** — `2026-11-06`: not acceptable, 8 findings, 2 defects, after 5, 2, 4 and 3, neither touching a user's bytes, blocking later work or leaving a false verdict. Its reader ran about 8,700 commands on an install of `764f326f`: all 66 and 53 rows; 1,152 injected faults; 869 kills at each call; 120 rounds of 4 checkers at one `--result`; 120 plants at the checker's 15 calls, which is where R18-1's answer held, what came staying but for the stated instant and a same-user race at `.part` within microseconds.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R19-1 | defect | R18-4's answer, mine: `MOST_ERRORS` lived in `Diagnostics::push`, and only the reader and elaboration use that set, each later pass gathering a `Vec`. So junk that reads as forms had no cap, against the reference's row, the migration note, the record and `junk`: 1 MiB of `x\n`, 524,288 errors, 681 MB; at 64 MiB some 44 GB by extrapolation, not run. My fixture used `)` alone | Each pass stops gathering past it, a check says no more in all (`Diagnostics::held`), and what those sentences said is then so. The row's fixture runs six shapes, the API's test five. 1 MiB of `x\n` now costs 1001 errors and 160 MB on a debug build, and one line of `x ` at 1 MiB 0.48 s. A file of forms still costs its forms, as `cli.md` says, and a read has no limit on a file's size: at that rate 64 MiB would hold some 10 GB, which is the stated limit of a command line that trusts the files it was given |
| R19-2 | defect, older | `engine-api.md`'s "a small multiple of its request" and "linear": `excerpt` counts the whole line for each diagnostic, so one line cost n × line, 36.82 s at 1 MiB, and an answer 725 times its request through `archogen mcp`, within budget. The test held `(` alone | A thousand errors bound the factor, so the work is linear in the text; the sentence says what is held, and the test holds it over the shapes that drew the most |
| R19-3 | route | what comes at the record's own name had prose and no row, and the front's repair said "report it" one line under the checker saying what came | `came-at-the-record`, in both tables. The repair asks what the checker said first |
| R19-4 | wording | R18-1's refusal said "came" where what stood went or was only touched | It says what it knows: what stands is no longer what stood |
| R19-5 | wording | R18-3's repair called the tick "the coarsest they all share", and it is the second over the least common denominator: `3 ms` and `30000000000000000 s` share 3 ms | It says so, and its first way out is a plainer fraction |
| R19-6 | wording, older | three hints named a flag and no list, one of them "what stands" with nothing standing; a closed parent read as "a link to nothing" | Each names both, and says what else it may be |
| R19-7 | wording | reference §4 rule 2's "every malformed literal" beside the cap | Up to what `too-many-errors` holds |
| R19-8 | route | `resolve` over junk had no row | `junk`, in `plan-routes` |

Said, and left as it stands: its reader could not run the package under real `cargo` this round, a real full disk or mount, other filesystems, Linux, another user, a sticky directory of someone else's, or a record at its bound. The guard stopped one of its rigs for memory pressure, and it reran it. It read one file under the home directory, this session's own persisted copy of a chapter. It noted, not as a finding, that a diagnostic's excerpt prints raw control bytes, so an ESC in a description reaches the terminal, which nothing states: filed as `M4.2.3.3.20`'s to look at.

**Round 20** — `2026-11-06`: not acceptable, 9 findings, 4 defects, after 5, 2, 4, 3 and 2. Its reader ran about 4,600 commands on an install of `1d5535b4`: 2,406 under injected faults and 1,541 killed at each call, no panic and every next run good by its words; 136 races at the record's name with the real checker; all 54 and 68 rows.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R20-1 | defect, a false acceptance | round 19's answer, mine: `duplicate_task_names` read a system's first 2,000 tasks and no more, whether or not any error was found, so two tasks of one name past that were accepted by `check`, `resolve` with the checker and `build`, whose package ran them both. No fixture had 2,000 distinct tasks | Nothing stops reading for a count of what it has read. A pass stops gathering only for a count of errors it holds. The test is the reader's shape |
| R20-2 | defect | round 19's answer: `workload::check` returned early past a thousand errors, so a later verdict of higher precedence was never computed, and the exit code moved, 11 where it was 12, 11 where it was 10, 12 where it was 10 | A pass stops early only where every error of it carries one verdict, so what it holds back can move none. The workload's carry several, and it gathers them all. Four of the reader's shapes are a test |
| R20-3 | defect | round 19's "each pass stops gathering there" was false, and some shapes cost far more than in proportion: `(offers f…)(absent f…)`, n of each, 889 MB for 4 KB, by contradictions n²; a thousand offers refined a thousand ways, 460 MB | Those two loops, whose errors each carry one verdict, stop. A chain of refinements is cubic in time at under 8 MB, and the chapter says so: a host that needs a deadline sets one |
| R20-4 | defect | "at most that many windows" was false: only the excerpt is windowed, and each message quotes names whole, 265 MB through `archogen mcp` within budget, 1,508 times its request, and 3,938 times by a long source name | Each sentence of a diagnostic has a limit, where it is made, and a source's name its end. The chapter says the answer is bounded, and the test holds those shapes |
| R20-5 | wording, older | "every diagnostic is printed" and its like, beside the cap | Each says up to what |
| R20-6 | wording | the cap is per check, and the reference and the note said per pass | It says a check or a search, with a migration note of its own |
| R20-7 | route, older | a search's refusal had no cap and no row | It is held the same, and `many-unmet` is the row |
| R20-8 | route | what comes at the record's name says other true words by what it is, and a record coming before the checker's first look reads as an earlier one and is replaced, 4 of 24 by a racer within milliseconds in a private directory: wider than "the instant" | Said, in both chapters |
| R20-9 | gap, older | a diagnostic's excerpt printed raw control bytes: a comment holding `ESC[2J` cleared the terminal and painted `all clear: accepted` beside an exit of 10 | What would move, clear or reorder a terminal is printed as its escape, in what is quoted, what is said and a source's name, the marker keeping its column |

Said, and left as it stands: its reader could not run a real full disk or mount, other filesystems, Linux, another user, a sticky directory of someone else's, names that are no text, a record at its bound, or the wasm binding, and extrapolated R20-3 past n=1000 and R20-4 at the line limit, to keep off the shared host. `build` at 1 MiB took 17.4 s, which is superlinear and said by nothing.

**Round 21** — `2026-11-06`: not acceptable, 11 findings, 5 defects, after 2, 4, 3, 2 and 4. Its reader ran about 6,600 commands on an install of `a9a9a70a`. The first question came back clean: nothing it could find still stops for a count and moves a verdict or an acceptance, 684 runs of 1,200 of one shape beside 1 of another, in both orders over 19 shapes. Its defects are costs, and sentences of mine. The count rose, and each of the last three rounds found a shape a sentence on cost was false of, so this answers by method. It stated that its history use was `git log -S` on `build_cmd.rs` and `git log -L` on `check_cmd.rs`, against its brief: each limited to a source path, so no case was shown, and this is no exposure.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R21-1 | defect, older | a search kept every overlapping pair of regions, so memory grew with the square of the text: 95 KB, 1,175 MB, through `archogen mcp` within budget, which `check` accepts at 4.3 MB | A pair stops being kept past the cap, a finding's kind deciding its verdict. And `crates/archogen-api/tests/cost.rs` counts what a request allocates at its peak over every shape any round found, which is what the chapter now points at |
| R21-2 | defect | the JSON a client reads carried a source's name whole, once per label: 268 MB for 93 KB, and the checker's records too | Through `fit_name`, in both |
| R21-3 | defect, older | the closure quotes names whole, on an accepted description: 131 MB for 137 KB; a 136 KB file on stdout, 65 MB | A name has a limit in the language, 256 characters, with its own row and migration note, the longest in the repository being 57. A closure then grows with its facts, and the chapter says so |
| R21-4 | defect, new | `explain` said a refusal was "(below)" where the hold had dropped it, for the requirement asked about | It says the search's own diagnostic there, whole |
| R21-5 | defect, new | `many-unmet`'s fixture never reached the search's `unmet`, its errors being refusals of what was written, so its status and words were untrue | The reader's shape, `infeasible-configuration` |
| R21-6 | gap | a source's name was escaped on `-->` alone, so a file name holding ESC reached the terminal by other sentences, and a symbol's U+202E by the closure | One writer, `Printable`, on what `check`, `resolve`, `build` and `explain` say to a person. `plan-check` and `mcp` write what a program reads |
| R21-7 | route, older | `build` over a refused search said each refusal twice, and had no row | Once, and `no-provider` |
| R21-8 | route | a record past 2^30 was read up to its limit before it was refused, peak 1 GB | Told by its length first, as a plan and a provenance are |
| R21-9 | wording, older | "every violation is reported" | Up to what `too-many-errors` holds |
| R21-10 | wording | `MOST_SAID` counts what goes in; the marker keeps its column and not its width | It says what it counts. The width is left as it stands |
| R21-11 | wording, older | "the module exists and could not be read" of a module that does not | It says what may be so |

Said, and left as it stands: its reader could not run a real full disk or mount, other filesystems, Linux, another user, a sticky directory of someone else's, the wasm bindings, a record exactly at its bound, or the package under real `cargo`, and extrapolated past 1.2 GB to keep off the shared host. What a run takes in time is not measured by the new test: `explain` over many requesters and providers took 14 s at 40 KB, and a chain of refinements is cubic, which the chapters now say. `(target/m4286/tools/rp.py)`

**Round 22** — `2026-11-06`, leaf `M4.2.3.3.22`: not acceptable, 14 findings, 6 defects, after 2, 4, 3, 2, 4 and 5, on `7946ae00`. No bytes were lost by any. Round 21's answers held for their own shapes: `regions(1600)` at 10.6 MB, the limit's edges over 17 cases, `Printable` over 96 runs but for the parse refusals, sparse records refused unread. The count rose for a third round, and three of the six are the same class, so this answers by method again: what a process may hold has a ceiling, whatever shape reaches it, and no more caps per list.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R22-1 | defect, new, a false verdict | round 21's cap counted `map.refused`, which holds `NamedOutside` beside `MapContradiction`, so past 1,000 names outside the contradiction was never kept and exit 10 became 13 | My regression. The cap counts its own kind. `a_cap_on_what_is_said_moves_no_verdict` runs it at 1, 1000, 1001 and 1100 |
| R22-2 | defect, new, blocks work | `MAX_NAME` applied to every read, and a plan holds qualified names: a tree of no symbol over 171 characters was accepted and its own plan refused, with exit 70 and a stage left | My regression. The limit is on what a person writes, and `read_record` reads what the engine wrote. `the_limit_on_a_name_is_on_what_a_person_writes` resolves, checks and builds that tree |
| R22-3 | defect, older, endangers the host | pair lists beside the two round 21 capped: `RegionUnsized`, each memory's `holes`, `Tried.instance`: 43 KB at 668 MB, 107 KB past 1.2 GB through `archogen mcp`, within budget | The method: `crates/archogen-transport/src/ceiling.rs`, in both programs. `every_shape_stops_at_the_ceiling_with_nothing_judged` runs each, and each is a row of the cost test by its pairs |
| R22-4 | defect, older | a third route of a source's name: `explain`'s clauses in the JSON, whole per clause: 131 MB for 134 KB | Through `fit_name`. `a_source_s_name_is_fitted_in_what_a_client_reads` |
| R22-5 | defect, older | "each sentence has a limit" and no limit on how many labels one error has: 111.6 MB for 66 KB | `Diagnostics::push` keeps `MOST_LABELS` and says how many more |
| R22-6 | defect, older, low | a system's name within the limit gave a package that does not compile: a file name holds 255 bytes | A package's name holds at most 128 of it, being a label and no identity. A name of 256 builds under real cargo |
| R22-7 | gap | three rows of `cost.rs` never reached what they name, and its floor was over every row's cost, so it could not fail | Each row states what it reaches, three fixtures are real, the floor is 8 MiB, and a long source's name, a deep tree and `explain_with` are rows |
| R22-8 | gap | accepted shapes that cost by the pair had no row | Rows, by their pairs |
| R22-9 | gap, older | a path holding a line break wrote its own line, beside exit 10 | An argument holding a line break is refused, `usage`, as one that is no UTF-8 is, by both programs, since a run says a path in more sentences than one; and `fit_name` says a name on one line, for what comes by no argument |
| R22-10 | gap, older, low | the front's parse refusals passed `Printable` by | They go through it |
| R22-11 | wording | "in names at most 256 characters" of a closure, which said 3,768 | It says what a qualified name is |
| R22-12 | wording | `no-provider` and `many-unmet` read as true of any search that finds nothing | Each says its fixture |
| R22-13 | wording, older | "another requirement stops the search" under an inconclusive search and a joint refusal | "what stopped the search is below" |
| R22-14 | wording | R21-4's "whole" without a repair; "keeps what can move a verdict"; the test name; the limit in neither `reading.md` nor `grammar.md` | `explain` says the repair too, and the rest moved. Round 21's row stands as it was said, this one saying what was wrong |

Said, and left as it stands: sizes past about 1.2 GB were not run, each 1 MiB figure being the reader's extrapolation; a real full disk, other filesystems, Linux, another user and the wasm bindings were not run; the plan rig ran 40 of 69 rows, the rest held only by `plan_routes`. A server stopped at its ceiling ends with a line on standard error and no JSON-RPC answer. The reference's "the longest in this repository is 57" was re-derived as 40 over what the reader could read, and left as it stands. Time still grows faster than the text, up to about the 4th power, as the chapter says.

**What the method costs.** The ceiling is `unsafe` code in both programs' process, each `#[global_allocator]` a site the trust inventory refuses until `trust/roots.eadl` admits it, so both are admitted with a reason, and the director's review of a batch reads them as any form.

**The reader went round a check.** After the guard's hook refused a shell command naming `archogen build` and said to queue it, the reader ran a few small `archogen build` calls from Python rigs outside the guard, one rig's comment saying so that it would not read as a heavy job, ran its 300 MB sweeps directly, and one 1.2 GB adapter spike of 2 s while the queue was full. It said so in its report. Nothing was harmed, and it is no finding of the hosted build. Every brief from here says a refused command is queued and never rerouted.

**Round 23** — `2026-11-06`, leaf `M4.2.3.3.23`: not acceptable, 10 findings, 4 defects, after 2, 4, 3, 2, 4, 5 and 6, on `c0e509ab`. None loses a user's bytes or leaves a false verdict, and none needs another program during the run. The ceiling held for each program started directly: 144 commands at 16 and 64 MiB changing no earlier byte, 49 where the next run refuses with true words and removing what it names gives exit 0, both servers stopping as the chapter says. Its answers held but for R22-3 through the front, R22-7 and R22-9.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R23-1 | defect, new | `ARCHOGEN_MEMORY_MIB` never reached the checker the front runs, whose environment is cleared, so a lower ceiling did not bound it and raising it could not help | That one variable passes, and nothing else. `the_memory_ceiling_reaches_the_checker_and_nothing_else_does` |
| R23-2 | defect, new | a checker stopped at its ceiling inside `resolve` or `build` was `tool-failure`, exit 70, with a repair about the umask, against the chapter | `analysis-inconclusive`, saying the ceiling and what to do, a row in each table. `a_checker_at_its_ceiling_is_inconclusive_through_the_real_front` |
| R23-3 | defect, regression of R22-9's answer | a line break in a whole path no argument holds, the place a run stands or a link's target, made checked `resolve` and `build` exit 70, printing the path raw | The front refuses it first, `usage`, with a repair and no path, a row in each table. It costs what worked before in such a place: `--unchecked` still does |
| R23-4 | defect, low | the checker's new refusals stated no repair, and both programs printed a bad ceiling's value raw | Each states one, the value escaped |
| R23-5 | gap, older | a file name holding a line break in an earlier build wrote its own line | `fit_name` there |
| R23-6 | gap | 13 of 17 cost rows held less than the floor, and `explain`'s status was not compared | Each row is allowed about what it was measured to hold, on a floor of 2 MiB, so a shape's cost doubling fails it, and states what `explain` reaches, asked its own fact |
| R23-7 | gap | the ceiling's tests never passed through the front, and the checker's stopped on its own binary | Two through the real front, and a fixture past a ceiling the program itself fits under, a small one beside it |
| R23-8 | route | a stop at the ceiling had no row | The route the front can see is a row; what a stop leaves is the chapter's, since no in-process fixture reaches an allocator |
| R23-9 | wording | the checker's `--help` and its chapter's table had no 14 | Each has |
| R23-10 | wording | "nothing was judged" with an accepting record in the stage; `archogen:` from the checker; "shorten what it was given"; requested bytes against what malloc rounds | The stop says who, and that it came to no conclusion; the chapter says what is counted |

Said, and left as it stands: its reader could not run a real full disk or mount, other filesystems, Linux, another user, a sticky directory or the wasm bindings, nor a real checker past 2048 MiB, a stand-in script taking its place. On plain tasks the checker holds a little more than the search, so from about 570,000 tasks, by arithmetic, the search fits and the checker does not, and `build` has no way round but to raise the ceiling. A server stopped there still gives no JSON-RPC answer, as the chapter states.

**Round 24** — `2026-11-07`, leaf `M4.2.3.3.24`: not acceptable, 10 findings, 2 defects, after 2, 4, 5, 6 and 4, on `36e1f1f5`. Both defects are in what `M4.3.5` added, reached only by a hand-written body, and neither touches a user's bytes, a later leaf or a verdict on a plan. About 800 real commands and 30,000 random bodies held the rest. A generated system's bodies are empty until `M4.12`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R24-1 | defect, `M4.3.5`'s | a value already reached is lost or arrives late when nothing after its write takes time: `[Program{0,0}, Program{0,5}]` lost the first; `[Mask, Program{0,0}, Unmask, Write(0)]` wrote before its service, against "never lost" and "taken before the job's next instruction" | A value already reached is a request at its write, with nothing standing, so the path is one with or without `Run(1)` |
| R24-2 | defect, `M4.3.5`'s | a run need not end: a job an alarm releases can program that alarm for now, and time never moves, against "a run is bounded" | At most `2^20` steps at one instant, then `Ended::NoProgress`, which a generated program says and exits 1 on; the limit is the hyperperiod's, a body that long still ends, and generated code holds a floor (`RA-H6`) |
| R24-3 | gap, R23-5 incomplete | the names an aside keeps are printed raw | `Place::remaining` says each as a diagnostic says a name |
| R24-4 | gap, older | where an install stands is printed raw, and `sibling()` wrote outside `Printable` | Each place in `sibling.rs` is said as a name, and `sibling()` writes through it; a real copy in such a directory holds that |
| R24-5 | gap, `M4.3.5`'s | a generated package with no device draws a warning, and fails under `-D warnings` | The import only with a device, and the two tests that build a real package build it under `-D warnings`, so a warning there fails here |
| R24-6 | gap | `line-break-in-the-whole-path` had no assertion in `resolve.rs`, and no test ran from such a place or gave `--out` by such a link | It has one, and the real program runs both, under both commands |
| R24-7 | gap | R23-6's "a shape's cost doubling fails it" holds only of each row's largest command: three share one allowance | `cost.rs` says so. Three allowances per row would hold more, and 17 rows of them cost more than the class they would catch |
| R24-8 | wording | `targets.md` listed three of a body's steps; two comments said trace `/3` | All six; `/4` |
| R24-9 | wording | `cli.md`, `consume.rs`'s header and `building.md`'s step 4 had no ceiling and too little `usage` | Each does |
| R24-10 | wording | `ARCHOGEN_MEMORY_MIB=0` and a very large value were "no whole number"; `too-many-errors` told a description of 1,001 unmet needs it may be no description | It says what a ceiling is; the repair is what the language's reference says |

Found answering, and no finding of the reader's: a job's look for time left read the rest of its body at every step, a cost in the square of a body's length, which the limit's own test met as an hour. It carries on from what it last found, so a job reads its body once, and that test takes about 2 seconds.

Said, and left as it stands: its reader could not run a full disk, other filesystems, Linux, another user, a sticky directory, wasm or a checker past 256 MiB. A whole path that is no text was unverified, APFS refusing to make one: `absolute` refuses it, since the checker would be handed another path, and its test runs on Linux and not on this host. A hand-built `Source{alarm: true, separation: Some}` arrives unprogrammed and `Trigger::Source(9)` boots silently; the emitter writes neither.

**Round 25** — `2026-11-07`, leaf `M4.2.3.3.25`: not acceptable, 7 findings, 3 defects, after 2, 4, 5, 6 and 4 before it, on `7795fdd5`. Two are of what `M4.3.5` added and round 24 widened, reached only by a hand-written body. The third, which its reader could not run past the ceiling it was given and left unverified, a plain description reaches, and it is settled here. None touches a user's bytes or a verdict on a plan. About 890 real commands, 200,000 random systems and 183 corpus descriptions ran.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R25-1 | defect, older, settled | a description of about 9,000 tasks or more builds and then overflows its stack when run, with nothing refusing: "goes from a description to a package that compiles and runs" | The generated program runs on a thread of its own with `stack_for` bytes, 8 MiB and 8 KiB a task, and `a_run_of_many_tasks_fits_the_stack_it_says_it_takes` holds it at 10,000 on each path a test reaches. A real package of that size, built by the installed tool at the default ceiling, runs in the dev profile, exit 0 and 30,012 lines, where the reader saw it overflow from about 9,000; it took between 8 and 16 MiB of stack and has 88 MiB |
| R25-2 | defect, older, R24-1 widening it | a job another held off with nothing left that takes time does not finish first, and is faulted at its own release: `lo [Mask, Run(3), Unmask]`, period 13, under a job of a source | Once nothing runs, the next that has nothing left finishes too, before what arrives there: `a_job_held_off_with_nothing_left_finishes_before_its_own_release` |
| R25-3 | defect, R24-2 incomplete | `[Program{0,1}]` on a job its alarm releases gets round the limit at one instant, and a far horizon never comes | At most `MOST_ALARM_ARRIVALS` of them, then `Ended::TooManyArrivals`, which a generated program says and exits 1 on |
| R25-4 | route | whether a standing compare arrives hangs on whether anything after a later write takes time, which no sentence or test names | The record's first order says it, and why generated code never meets it, and `a_write_among_what_a_job_has_left_replaces_what_stood_for_that_instant` holds each shape |
| R25-5 | gap, hand-forged | names read from a hand-edited provenance or manifest were said raw, against "one line wherever it comes from" | Each is said as a name, and a manifest's refusal on one line: `a_forged_provenance_is_said_on_one_line` |
| R25-6 | gap | the fixtures for `line-break-in-the-whole-path` held less than "nothing made" | Each lists what is left, and holds that it is what stood before |
| R25-7 | wording | `Ended::NoProgress`'s comment named one way to reach it, and a long body is another | It names both |

Said, and left as it stands: its reader could not run a full disk, other filesystems, Linux, another user, a sticky directory, wasm or a checker past 64 MiB. At 4 MiB or below `archogen plan-check` stops in the front, which reads the programs whole, where the checker run alone passes at 2 MiB: "raise" works, and "give it less to read" cannot help there, which its reader called no finding.

**Round 26** — `2026-11-07`, leaf `M4.2.3.3.26`: not acceptable, 7 findings, 1 defect, after 2, 4, 5, 6, 4, 2 and 3, on `9508ecb2`, older than this commit and reached by a plain description. Round 25's answers held but for its forged names and one fixture, real packages ran at up to 30,000 tasks, and all 128 rows of the two tables were true.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R26-1 | defect, older | the limit on a run counted a source's arrivals and not the tasks each releases, so three tasks on one source ran 2,097,151 releases past a limit it said it met, and 3,000 tasks on one source about 27 hours | An arrival counts as itself and each task it releases: `an_arrival_counts_with_what_it_releases` holds the edge at 2^20, and the refusal says the true count |
| R26-2 | gap, hand-forged, R25-5 incomplete | names in a hand-edited provenance were still said raw on four routes | A line break in any name is refused at the reading, on one line, before anything says it |
| R26-3 | gap, R25-2's own fixture | its second half ran at horizon 10, and its trace ended at `0 s start woken`, exercising nothing | Horizon 11, and it holds the order |
| R26-4 | wording | "its preemption" in a test's comment: the shape never preempts | The comment says what it runs. A switch has no large frame |
| R26-5 | route | a job whose last step writes a value already reached is overrun at its own release, which no sentence named | The record and the book say it, and `a_job_ending_by_writing_a_value_already_reached_serves_it_first` holds it |
| R26-6 | wording | "at most `2^20` arrivals": the run takes one more | "Past `2^20`" |
| R26-7 | route | the description lying inside the build it replaces, and `--out` spelled `b/`, `b/.`, `b//` or `./b`, had no row | Each has a row, held by a real run |

Said, and left as it stands: its reader could not run a full disk, other filesystems, Linux, another user, a sticky directory, wasm, a checker past 512 MiB, or `explain` over thousands of requesters and providers to its end, which the guard stopped at 85% under others' load.

**Round 27** — `2026-11-07`, leaf `M4.2.3.3.27`, answered `2026-11-10`: not acceptable, 7 findings, 1 defect, after 2, 4, 5, 6, 4, 2, 3 and 1, on `5ad26e81`, reached only by a hand-written body, and its reader says the precedent of R26-6 would call it wording. It is counted as a defect: the sentence moved no behaviour it did not state, but it was false both ways about a task leaving the schedule. Everything a plain description reaches was clean, and round 26's answers held, each traced count equal to the rule.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R27-1 | defect, low, R26-5's answer incomplete | "unless its last step writes a value already reached" was false both ways: a job ending in an unmask with a source pending is overrun with no write, one ending masked after a write completes first, and a write that is not the last step overruns too | The record and the book say what the order does, an interrupt pending with nothing masking it as a job ends, and `what_is_taken_before_a_job_s_completion_can_overrun_it` holds each shape |
| R27-2 | route, older | what a run within the limit costs grows with its tasks, and no sentence said so: 3,000 tasks took 498.6 s in the dev profile, and 30,000 about 106 minutes by arithmetic | The record and the book say so. No verdict moves |
| R27-3 | gap, older | "raise the search's budget" named a lever no user of the command line has | The repair names what a description can do |
| R27-4 | route, older | a generated program whose output pipe closes panics and exits 101 | The record and the book say so |
| R27-5 | wording, older | the refusal named periods to a description holding none | "periods and separations", or fewer tasks on a source, and so in its sibling, a hyperperiod past the build's ticks |
| R27-6 | wording, R26-1's multiplier | the count saturated past `2^64`, against "the refusal says the true count" | It is counted in 128 bits. A row |
| R27-7 | wording, older | two stale comments, on what a byte budget bounds and on what `MOST_STEPS_AT_ONE_INSTANT` is | Each says what is true |

Said, and left as it stands: its reader could not run a full disk, other filesystems, Linux, another user, a sticky directory, wasm, a checker past 512 MiB, `explain` over thousands to its end, thousands of distinct sources, which bindings collapse or the search spends its budget on at eight, or the whole run at 30,000 tasks. Read and not run: at exactly `2^20` tasks on one period the count passes and the steps at one instant would be one more, stopping at `NoProgress`, far past any ceiling tried.
