# The transport's row in the checkers' programs record: a review reopened by `RP-H10`

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `stopped` — rounds 16 to 18 answered; round 19 stopped `2026-10-26` by the director's ruling, no
  mutation testing before feature completeness, which the rounds had become; no round found zero defects, so the
  review did not close by its rule, and the suites' adequacy is `M7.6`'s.
- **Owner / source:** leaf `M3.5.1.6.3` (`docs/tasks/M3.md`). The rounds of the review of
  [`decision_trust-role-programs.md`](../specs/trust/decision_trust-role-programs.md) reopened by its hand-off `RP-H10`
  — "`crates/archogen-transport`'s controls are written into `decision_trust-role-programs.md` §5's row, through that
  record's review, before a checker compiles it" — numbered on from the closed rounds 1 to 15, whose history is
  archived, frozen, in
  [`../review-history/decision_trust-role-programs-reviews.md`](../review-history/decision_trust-role-programs-reviews.md).

## The fact / decision

Each round is a new context, read-only, which did not write the row nor answer an earlier round, fenced from the
sealed evaluation set, given the row, the package `crates/archogen-transport`, its tests, the generator's server over
it and the records the row cites.

## Rounds

**Round 16**, `2026-10-26`, the review reopened by `RP-H10` — "`crates/archogen-transport`'s controls are written into
`decision_trust-role-programs.md` §5's row, through that record's review, before a checker compiles it" — a new
context, read-only and fenced, reading the working tree's row against the package, its tests and the generator's
server over it (`M3.5.1.6.1`), with 18 hand mutants of the package run on scratch copies. Verdict: not acceptable, 6
defects and 6 remarks. The class held; the controls said more than the tests held, the package wrote a tool's `_meta`
twice, and the risk left out the project's own reading of the protocol.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R16-D1 | defect | "falsified by mutation" is untrue and unrecorded: 16 of 18 mutants survive the package's own tests, 9 killed only by the generator's | the suite rewritten: 21 hand mutants — the round's 18 and three of R16-D4's fix — each killed by the package's own tests, apart from every role's tools, recorded in `M3.5.1.6.3` |
| R16-D2 | defect | the coverage listed is overstated: no legacy `tools/call`, no `error` response, no line at the bound or split across reads, ids unchecked, `-32022`'s `data` unchecked | each added: a legacy call; both kinds of response unanswered; a line exactly at the bound and one over, read whole and through a 7-byte buffer; every answer's id asserted; `-32022`'s `data`, a version that is no string, each `_meta` piece alone, capabilities no object |
| R16-D3 | defect | faults inside the stated risk survive every control: an answer under another id, the era misselected, an unterminated line dropped, a CRLF line answered, malformed `initialize` accepted, `call` reached for an unknown name | each killed by a case of its own; the fake panics when `call` is reached for a name `knows` does not admit |
| R16-D4 | defect | the framing appends `_meta` beside a tool's own, so the strict reader refuses the answer; a result that is no object is passed out | fixed in the package: the server's identity merged into a tool's `_meta`; a tool naming the server's key there, writing `resultType`, or answering no object is `-32603`; cases in both eras |
| R16-D5 | defect | the record reader's control does not exist yet and is not marked *pending*; it cannot catch an answer under another id | marked *pending* `M3.5.1.6.4`, saying what it catches and that the framing's tests hold the id |
| R16-D6 | defect | the residual risk leaves out the project's one reading of the protocol, which every server and both test sets share | stated in the risk as residual; a control apart from it — answers checked against the pinned revisions' published schemas — filed as `M3.5.1.6.5` |
| R16-R1 | remark | the cited suite is untracked | it lands with the row (`M3.5.1.6.3`) |
| R16-R2 | remark | "the transcript in both eras" is ambiguous | the two-era test named |
| R16-R3 | remark | U+2028 goes out raw, an accepted choice the risk should name | named in the risk; a case pins `\n` and `\r` escaped and U+2028 written as it is |
| R16-R4 | remark | name id association in the risk | named |
| R16-R5 | remark | each checker's server's tests are a control to come | listed *pending* `M3.5.1.6.4` |
| R16-R6 | remark | checked and holding: the class, "every pair it joins", the generator's tests run unchanged, every commit running them | no change |

**Round 17**, `2026-10-26`, a new context, read-only and fenced, reading round 16's answers against the code, re-running
round 16's 21 mutants (each killed) and 17 of its own. Verdict: not acceptable, 4 defects and 7 remarks. Round 16's
answers held for what round 16 named, the `_meta` fix correct and no new fault on any answer path; but the suite had
been written to kill a list, not to hold the row's claim, and 16 of the 17 new mutants survived it. The answer changed
the method, as its R17-R6 proposed: one case per answer site, the whole line asserted, held to the source by a census,
so an answer path added later cannot go untested — both rounds' 38 mutants killed by the package's own tests after.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R17-D1 | defect | "every JSON-RPC error and the id each answer carries" is untrue: five paths' ids unchecked, two paths unreached, five mutants killed only by the generator's tests | the table: every answer site of `src/lib.rs`, marked `// answer:` before its `error(` or `result(` call, has a case asserting the whole line under its own id; a census reads the source and refuses a site with no case or a case with no site |
| R17-D2 | defect | ten faults inside the stated risk survive every control: ids, id-less messages, a too-long last line dropped, `initialize`'s capabilities, `tools/list`'s `ttlMs`, a tool's non-object `_meta` | each now fails a whole-line case or a framing case: the unterminated too-long last line, the id-less messages, a tool whose `_meta` is a string |
| R17-D3 | defect | "round 16's 21 hand mutants" is misattributed, and the mutants live only under ignored `target/` | the row cites no mutant count: its control is the table and its census, which live in the tracked suite; the mutants, both rounds' 38, are falsification recorded in `M3.5.1.6.3` |
| R17-D4 | defect | the fix's behaviour is recorded nowhere normative, and the ledger's revalidation trigger fired unanswered | `Tools::call`'s contract states it; the MCP record's *As built* and the book say it; the pinned `2026-07-28` `schema.ts` re-read for `Result`'s `_meta` and `resultType`, its sha256 unchanged, recorded in the ledger's entry |
| R17-R1 | remark | U+0085 and U+2029 are missing from the risk | named; a case pins them written as they are |
| R17-R2 | remark | the schema check sits only in the risk column, and the ledger pins `schema.ts` alone | `M3.5.1.6.5` says a JSON schema is ledgered first |
| R17-R3 | remark | `M3.5.1.6.4` does not carry the read-back control the row hands it | it does |
| R17-R4 | remark | no test holds the per-answer flush | a writer recording each flush holds one per answer |
| R17-R5 | remark | the suite is untracked | it lands with the row |
| R17-R6 | remark | the method: one table of every site, the whole line asserted, a census of the sites | adopted |
| R17-R7 | remark | checked and holding: the class, R16-D3 to D6, R16-R2, R4 and R5, clippy and rustfmt, every test | no change |

**Round 18**, `2026-10-26`, a new context, read-only and fenced, re-running rounds 16 and 17's 38 mutants (each killed),
instrumenting every site to check each case reaches the one it names (each did), and writing 15 mutants of its own.
Verdict: not acceptable, 4 defects and 6 remarks. Two were faults of the package: a tool's bad `_meta` let through
under `2025-11-25`, and an id whose exponent overflowed `i64`, accepted or, in a debug build, a panic. The others showed
the table holding its sites and not the conditions inside them. The answer fixed both faults and changed the method a
second time: every answer is now compared, line for line, with a model of the MCP record's rules written apart from the
code, over generated sequences of well-formed requests each with up to three members changed, crossing `initialize`;
and the table's cases are held to their sites by a trace the package records, the writers naming their sites by
literal, so a dead site or a case reaching another is found.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R18-D1 | defect | a tool's non-object or server-naming `_meta` is refused under `2026-07-28` alone, though the contract says either era | the framing checks a tool's result once, before either era answers it; cases in both eras |
| R18-D2 | defect | an exponent past `i64` reads as integral, and `5e-9223372036854775808` overflows a negation: a panic in a debug build | the exponent read saturating, its magnitude compared unsigned; both ids cases of their own, and the model's own integer test |
| R18-D3 | defect | the census sees one file's lines and direct writer calls, never `Failure` paths, and ties a case to a site by label alone | the writers and `refused` take their site as a literal, which the census reads across lines; `json::write` has three callers, counted; each case runs under `traced`, which must show its site, and every site must be reached |
| R18-D4 | defect | ten faults inside the risk survive: legacy routes, `initialize`'s legs, an exponent id, `params: null`, a form-feed line | every condition is the model's to judge over the generated corpus; each also a case of its own in the table |
| R18-R1 | remark | U+2029 credited to the MCP record, which names U+2028 and U+0085 | the record names all three |
| R18-R2 | remark | `M3.5.1.6.5` claims more than a schema check finds | narrowed to answers' shapes, the misreading of which answer residual |
| R18-R3 | remark | the mutants are recorded nowhere tracked | the table below, each with its verdict against the suites this round answers with |
| R18-R4 | remark | a tool's result naming a member twice passes the framing, and the reader refuses the answer | the framing reads a tool's result back before answering, `-32603` when it does not |
| R18-R5 | remark | the record's status line says the review closed | it says the review reopened by `RP-H10` |
| R18-R6 | remark | checked and holding: the class, the generator's tests unchanged, clippy, rustfmt, every test | no change |

Every mutant of rounds 16 to 18, against the package's own tests as round 18's answer leaves them — those whose text
the rewrite moved written again against the code, the same fault:

| Mutant | Change | Verdict |
| --- | --- | --- |
| G1 | answer built by `json::write` under id `"y"` | killed |
| G2 | a new `Failure` path | killed |
| G3 | two answer calls on one marked line | killed |
| G4 | a marker renamed (control) | killed |
| G5 | `fractional-id` site made dead | killed |
| M1 | bound `>` → `>=` | killed |
| M2 | keep only the last chunk of a line | killed |
| M3 | a client's `error` response answered | killed |
| M4 | modern era needs both `_meta` keys | killed |
| M5 | error answers drop their id | killed |
| M6 | `knows` not consulted | killed |
| M7 | `clientCapabilities` of any type | killed |
| M8 | a non-string version read as another | killed |
| M9 | `initialize` without `clientInfo` | killed |
| M10 | an unterminated last line dropped | killed |
| M11 | no `resultType` on a modern call result | killed |
| M12 | legacy `tools/call` answered as `tools/list` | killed |
| M13 | a modern `Failure` answered under id `"x"` | killed |
| M13b | a modern result answered under id `"x"` | killed |
| M14 | `-32022` without `data` | killed |
| M15 | a CR-only line not blank | killed |
| M16 | `Internal` answered `-32602` | killed |
| M17 | `tool_error`'s `isError` false | killed |
| M18 | a tool's `_meta` written beside the server's, not merged | killed |
| M19 | a tool's `resultType` let through | killed |
| M20 | a tool's result that is no object answered as an empty one | killed |
| N1 | `initialize`'s result under id `"x"` | killed |
| N2 | `2026-07-28` `-32601` under id `"x"` | killed |
| N3 | `2025-11-25` `-32601` under id `"x"` | killed |
| N4 | early tool method's `-32602` under id `"x"` | killed |
| N5 | malformed `initialize`'s `-32602` under id `"x"` | killed |
| N6 | unknown method before `initialize` answered as a result | killed |
| N7 | the same, answered `-32602` | killed |
| N8 | every id-less message left unanswered | killed |
| N9 | an id-less message without `jsonrpc` left unanswered | killed |
| N10 | `initialize`'s `capabilities` without `tools` | killed |
| N11 | `server/discover` without `2025-11-25` | killed |
| N12 | `2026-07-28` `tools/list` without `ttlMs` and `cacheScope` | killed |
| N13 | `server/discover` without `capabilities` | killed |
| N14 | a tool's `_meta` that is a string passed through | killed |
| N15 | a tool's `_meta` replaced, not merged | killed |
| N16 | no `flush` after an answer | killed |
| N17 | a too-long last line with no newline dropped | killed |
| P1 | legacy `tools/list` with a cursor answered with the list | killed |
| P2 | legacy unknown tool reaches `call` | killed |
| P3 | legacy arguments that are no object passed to the tool | killed |
| P4 | legacy call naming no tool answered as one naming an empty tool | killed |
| P5 | fractional id with an exponent accepted | killed |
| P6 | `initialize`'s `capabilities` of any type | killed |
| P7 | `initialize` without `protocolVersion` accepted | killed |
| P8 | malformed `initialize` selects the earlier era | killed |
| P9 | `params: null` read as empty | killed |
| P10 | form-feed line skipped unanswered | killed |

**Round 19**, `2026-10-26`, started and stopped the same day: the director re-stated the ruling of `2026-10-21` —
"Stop doing mutation testing until the project is feature complete … Mutation testing although important is a time
hog" — and rounds 16 to 18 had been driven by reviewers' hand mutants, each answer adding tests to kill them. The row
stands as round 18's answers leave it: its class and risk held by every round, its controls existing tests, none
claimed by a count of mutants. The review of `RP-H10`'s row closes there, at the director's ruling; what the rounds
kept asking — how many of the faults the stated risk names the transport's suites would catch — is the mutation
campaign's after the features are in (`M7.6`).
