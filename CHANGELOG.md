# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the JSON reader and writer in a package of their own

`ARCHOGEN-M3-0880` (leaf `M3.5.1.7`), `2026-10-23`.

- The MCP server's strict JSON reader and its writer move whole into `crates/archogen-json`, so a checker's result
  record will be read by the same reader as the server's messages; `archogen_cli::json` still names it.

## archogen — sha256 in a package of its own

`ARCHOGEN-M3-0879` (leaf `M3.5.1.2`), `2026-10-23`.

- The engine's SHA-256 moves, with its tests and its fixture, into `crates/archogen-digest`, so a checker can name
  the digest of what it judged without compiling the evidence vocabulary; `archogen_evidence::sha256` still names it.

## archogen — the no-process gate closes every route to a process

`ARCHOGEN-PROGRAM-0878` (leaf `PROGRAM.105`), `2026-10-22`.

- `NO-SUBPROCESS` matched names on single lines, so an alias, a split `use`, a foreign `execve` and `posix_spawn` passed
  it. It now reads code with comments and literals stripped and refuses the three routes a dependency-free Rust
  workspace has: `std::process` beyond `ExitCode` and `exit`, foreign functions, and inline assembly.

## archogen — the checkers' programs and the git-style front: the design, reviewed

`ARCHOGEN-M3-0877` (leaf `M3.5.1`), `2026-10-22`.

- Design the ruled shape: `archogen-plan-check` and `archogen-analyze`, each its own executable writing a result record;
  `archogen resolve` and `archogen analyze` run them, the launcher compiled into the `archogen` executable alone, each
  sibling verified against an install manifest derived from the trust inventory. `resolve --unchecked` is the search
  alone, offered alike on the command line and programmatically.
- One independent review, nine defects and seven remarks answered; it also found four ways past `NO-SUBPROCESS`'s
  patterns, filed as `PROGRAM.105`.

## archogen — ruled: each checker its own executable, behind a git-style front

`ARCHOGEN-M3-0876` (leaf `M3.5.1`), `2026-10-22`.

- The director ruled findings §16 (a): the scheduling and configuration checkers become executables of their own, and
  `archogen analyze` and `archogen resolve` run them as git runs its sibling commands; the programmatic surface still
  runs nothing. `M3.5.1` designs it, `M2.21` follows.

## archogen — for the director: the checkers' executables

`ARCHOGEN-M3-0875` (leaf `M3.5.1`), `2026-10-22`.

- `ROADMAP.md` §10.2 writes `archogen analyze` as a subcommand, but one program may not run two roles and the product
  spawns no process; findings §16 puts the choice to the director, recommending a separate executable per checker.

## archogen — file: the mutation campaign as a release gate

`ARCHOGEN-M7-0874` (leaf `M7.6`), `2026-10-22`.

- `ROADMAP.md` §14 asks for mutation testing, which the director postponed until the functionality is locked; `M7.6`
  makes the first supported release wait on the campaign, so the postponement cannot become an omission.

## archogen — the configuration checker: the design, reviewed

`ARCHOGEN-M3-0873` (leaf `M3.5`), `2026-10-22`.

- Design `archogen-check`, the independent configuration checker (F12): its own program, which writes a result record
  the build consumes, since the product spawns no process; it re-reads and re-checks the description and judges a
  plan's every binding and joint rule.
- One independent review, thirteen defects and eight remarks answered: the relation it shares with the generator is one
  derivation, disclosed; the catalogue's 58 model entries follow their files unchanged otherwise, under the director's
  decision; names are qualified and the plan names the elaborated program's digest.

## archogen — lifetimes and execution contexts decided; the joint constraints closed

`ARCHOGEN-M3-0871` (leaf `M3.3.8`), `2026-10-22`.

- Under `rt-static-up-v1` every part lives from its initialisation to a halt, so no lifetime rule is needed; which
  contexts a provider supports is a catalog record's statement, judged as a privilege level is once one states it, and
  initialisation waits on no interrupt. `M3.3`, §5.4's joint constraints, is closed.

## archogen — one function, one provider

`ARCHOGEN-M3-0870` (leaf `M3.3.6.2`), `2026-10-22`.

- The joint check keeps one requester's facts of one function at one provider: a counter's width and rate split
  across two timers is refused, or reported `missing-fact` where describing a fact would make one timer whole. A
  qualifier, such as a power state, is judged at every provider the requester relies on.
- A census over the suite finds no function split across hardware and lists five counters whose horizon or wrap
  behaviour no block states yet (the design's approximate count was nine). Model §8 rule 13 and the book explain it.

## archogen — each vocabulary fact names its function

`ARCHOGEN-M3-0869` (leaf `M3.3.6.1`), `2026-10-22`.

- `deffact` gains `function`: every guarantee of `/1` names the function it describes — `counter`, `interrupt`,
  `output` and the rest — or `each` for a qualifier; the vocabulary refuses one missing, misplaced or joining another
  function's facts. The book's matching chapter explains functions; a pending migration note covers the two files.

## archogen — provider bundles: the design, reviewed

`ARCHOGEN-M3-0868` (leaf `M3.3.6`), `2026-10-22`.

- Design `SR-H14`'s bundles: each vocabulary fact names the function it describes, and one requester's facts of one
  function are met by one provider; where describing a fact would make one provider whole the finding is an unknown,
  not a conflict. Qualifiers — a power state — hold at every provider the requester relies on.
- One independent review, seven defects and eight remarks answered; a census over the suite's 38 systems finds no
  function split across hardware, and names the 9 counters whose horizon no block states yet.

## archogen — a block no system uses decides no description's code

`ARCHOGEN-M3-0867` (leaf `M3.4.1`), `2026-10-22`.

- `SR-H18` amended as `M3.4` decided: when the search decides a description's code, a block or platform outside a
  system's hardware takes no part, as an unused device costs nothing; the book's account of `M3.4` says so.

## archogen — the runtime analysis design moves to the specifications

`ARCHOGEN-PROGRAM-0866` (leaf `PROGRAM.102`), `2026-10-22`.

- `docs/decisions/` held 392950 of the director's 393216 bytes. By the ruling of `2026-10-01`, the runtime analysis
  variant's accepted design moves to `docs/specs/analysis/`, every path citing it repointed, `MODEL`'s among them:
  355040 bytes now, room for `M3.3.6` and `M3.4`'s amendment.
- `docs/specs/` had 42 bytes to spare, so its ceiling rises one 64 KiB step, 589824, by its record's measured paragraph.

## archogen — the version register reads a declaration rustfmt wraps

`ARCHOGEN-PROGRAM-0865` (leaf `PROGRAM.104`), `2026-10-22`.

- `VERSION-REGISTER` read declarations a line at a time, so `fixed-priority-with-overheads/2` — the runtime analysis
  variant's model, bumped from `/1` by `M2.11`, wrapped by rustfmt — was never registered. It now reads each declaration
  whole; three arms hold the wrapped shapes and a profile's `id:`.
- The book's register gains `runtime-analysis-model`, pinned by a test that holds the constant to the identifier and
  the path of the record that defines it.

## archogen — uc3 states its level

`ARCHOGEN-M3-0862` (leaf `M3.3.4.5`), `2026-10-22`.

- `examples/alternative-timer`'s timer, console and platform each offer `(reachable-at-privilege machine)`, as every
  other suite description's hardware does; the privilege census now excludes nothing, and `M3.3.4` closes.
- No requirement moved: `archogen check` still answers `13 infeasible-configuration`, its locations one line down. The
  seal that uc3 builds unchanged now says it changed once, before `M3.2`, and that the language freeze refuses any later
  change without a note.

## archogen — the director's delegation: six findings decided

`ARCHOGEN-M3-0861` (leaf `M3.3.4.5`), `2026-10-22`.

- Decided under the director's delegation of low-level technical findings: uc3 gains its privilege level (a); the
  profile's Privilege row stands; `docs/decisions/` gains room by moving specifications (`PROGRAM.102`); the changelog
  becomes curated, per user-visible change (`PROGRAM.103`); `M3.4`'s three questions answered; `M3.6.9.7` and `.8`
  stay postponed with the mutation work. Each decision, its reason and its alternatives are in its leaf.

## archogen — file: the decisions folder at its byte ceiling

`ARCHOGEN-PROGRAM-0860` (leaf `PROGRAM.102`), `2026-10-22`.

- `docs/decisions/` holds 392915 of its ruled 393216 bytes, so `M3.3.6`'s vocabulary rows have nowhere to go; filed
  at the frontier's head with its remedy — records that are specifications moved to `docs/specs/` — and its candidates.

## archogen — the initialization graph, derived

`ARCHOGEN-M3-0859` (leaf `M3.3.7`), `2026-10-22`.

- From one system's assignment the joint check derives what initialises before what, for the plan: providers before
  the requesters bound to them, used services before their users, each edge with its reason; composed parts exempt,
  adapters reached through their bindings.
- Nothing is refused: a cycle is a component the engine initialises as one unit. Four tests fail first under five arms;
  model §8 rule 12 and the book explain it.

## archogen — the initialization graph: the design, reviewed

`ARCHOGEN-M3-0858` (leaf `M3.3.7`), `2026-10-22`.

- Design initialization order as a graph derived from one system's assignment, for `M4.1`'s plan: providers before the
  requesters bound to them, used services before their users, each edge with its reason; nothing refused, a cycle a
  component the engine initialises as one unit, since initialisation runs masked before every release.
- One independent review, five defects and four remarks answered; taken before `M3.3.6`, whose bundles wait on a
  vocabulary decision and on `M3.2`'s adapters.

## archogen — a required ordering is unsupported under this profile

`ARCHOGEN-M3-0857` (leaf `M3.3.5.2`), `2026-10-22`.

- `ordering` keeps no vocabulary entry: `rt-static-up-v1` excludes `dma`, whose obligations name ordering, so its domain
  is the later profile's; a requirement on it in a closure is `unsupported-profile`, pinned by a test.
- The boundary case requests nothing and keeps its frozen verdict; model §8 rule 11 and the book say so.

## archogen — placement, judged once per system

`ARCHOGEN-M3-0856` (leaf `M3.3.5.1`), `2026-10-22`.

- A region's sub-forms follow one grammar in the joint rules and the target agreement: each once, a flag `true` or
  `false`; the agreement refuses `reserved`, which it does not compare.
- Each system's memory map is judged once: reserved ranges in placement memory are holes reported for the plan; a
  device or a second placement region on its bytes, or memory all holes, is `invalid-description`; a placement the plan
  is given must be placement memory. Model §8 rule 10 and the book explain it.

## archogen — placement: the design, reviewed

`ARCHOGEN-M3-0855` (leaf `M3.3.5`), `2026-10-22`.

- Design memory placement: executable, unreserved regions are placement memory; reserved ranges inside it are holes
  the plan avoids, reported as an obligation; a device's registers inside it are a contradictory map, refused once per
  system; M3 chooses the region, `M4.1` the offsets.
- One independent review, five defects and four remarks answered; `SR-H13` amended so `region` stays outside `/1`, its
  sub-forms normative in model §8; ordering is `.2`'s own design.

## archogen — the target states its level, and its agreement reads every claim

`ARCHOGEN-M3-0854` (leaf `M3.3.4.4`), `2026-10-22`.

- `riscv-virt-up` requires `machine` (`REQUIRES_PRIVILEGE`), and its blocks and platform state it; the agreement
  compares each, from the privileged specification's reset and PMP sentences and the spike's measured run at M.
- The agreement now refuses an offer or a clause it does not read, as it refused an uncompared region: the target
  claims nothing it cannot check. The emulator check and the spike pass on the pinned QEMU.

## archogen — every suite system's hardware states its level

`ARCHOGEN-M3-0853` (leaf `M3.3.4.3`), `2026-10-22`.

- 45 blocks and platforms in 21 suite descriptions gain `(reachable-at-privilege machine)`, so the joint rule will refuse
  none of them once wired; a census over each system's hardware holds it, `examples/alternative-timer` excluded by name
  until the director decides.
- A pending migration note names the 21 moved constructs; the frozen verdicts and the third opinion are unchanged; the
  tour's copies of `s0`'s description and the sporadic case moved with them.

## archogen — the privilege level is the profile's

`ARCHOGEN-M3-0852` (leaf `M3.3.4.2`), `2026-10-22`.

- `rt-static-up-v1` gains a Privilege row: the runtime, its tasks and its services run at one level, `machine`, which a
  RISC-V hart starts in at reset — the privileged specification's §2.1.4, quoted and ledgered — and the playground
  models it; `Profile::caller_level` carries it for the search.
- The port record's citation for machine mode is corrected to that row, that sentence and the spike's own instructions.

## archogen — privilege, judged given a level

`ARCHOGEN-M3-0851` (leaf `M3.3.4.1`), `2026-10-22`.

- Given the level a plan runs one system's callers at, the joint check refuses a caller stating another level
  (`unsupported-profile`, first), and a caller bound to a provider not reachable at that level or stating none.
- A block needing a sibling's fact is no caller; routes carry the level; with no level given, nothing is said.
- Seven tests with literal expectations, failing first under five arms; model §8 rule 9 and the book explain it.

