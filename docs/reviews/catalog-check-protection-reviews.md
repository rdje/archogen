# The catalog check's protection: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active` — rounds 2 and 3 not completed; the next round is a reviewer the director names (findings
  §11); the review closes on the first round that finds no construction running pull-request code in the check, or
  making it pass without the base's checker
- **Owner / source:** leaf `M2.7.6.4` (`docs/tasks/M2.md`). The design under review is the repository half of
  `M2.7.6`, stated in the headers of `scripts/check_workflow_tokens.sh` (`M2.7.6.1`), `scripts/catalog_check.sh`
  (`M2.7.6.2`) and `.github/workflows/catalog-check.yml` (`M2.7.6.3`), against premise 3 of
  `docs/specs/catalog/decision_catalog-records.md`.

## The fact / decision

**Round 1**, `2026-10-01`, by a context that had not written the design, as an attacker controlling a pull
request's whole tree and able to push branches, but not the hosting's settings. It built its constructions for
real in scratch repositories under `target/`, checked the gate's verdicts against GitHub's own workflow reader
(`@actions/workflow-parser` 0.3.61) and a second YAML reader, and ran cargo `1.95.0`, the pin. Its verdict: 2
defects, both in `WORKFLOW-TOKENS`, which "do not currently meet [the repository half's] premise-3 claim"; the
harness and the workflow sound against the pull-request threat model.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| V1 | defect | the gate stripped a double-quoted scalar's quotes without resolving its escapes, so `"pull_request_t\x61rget"`, an escaped `permissions` key or an escaped `uses` hid a forbidden construct, and the gate passed it | a double-quoted scalar holding a backslash is refused, as one the gate cannot read with certainty; a single-quoted one's doubled quote is resolved; RED arm, red without the refusal |
| V2 | defect | every step of a job had the same path, so one step's `persist-credentials: false` covered another step's checkout | each sequence item has its own path; RED arm with the setting on a sibling step, red without the change |
| V3 | qualification | "alias-only configuration cannot change a `cargo build`" holds only for a built-in subcommand name: an alias named `b`, `clippy` or `fmt` runs, and an alias's body can carry `--config` | true of the harness, which invokes `cargo build` alone; stated in its header, and the open proposal for §3 carries the constraint that the checker invoke cargo by built-in names only (`M2.7.4`'s leaf) |
| V4 | hosting | the event payload's `merge_commit_sha` can be null or stale | the workflow checks out `github.sha`, which GitHub defines as the merge branch's last merge commit for `pull_request` and the merge group's for `merge_group` (GitHub's events reference, read `2026-10-01`); that the required check re-runs on every push is findings §11's |
| V5 | nit | a hand-written tree with a path both a file and a directory crashed the harness with exit 1, the checker's own "fail", and a path listed twice was written last-wins | both refused with exit 3 before anything is written past them, and any write error refused likewise; two RED arms over trees written with `--literally` |

**Round 2**, after round 1 landed (`ARCHOGEN-M2-0292`), was launched twice, each time in a new context with round 1's
brief, and was stopped both times by the agent harness's own safety screening before it reported; no report exists. It
is recorded as not completed, and an agent does not relaunch it in that form. In its place, `2026-10-02`, the gate was
narrowed on three constructions its own principle — refuse what it cannot read with certainty — already covered in
spirit but did not refuse:

| # | Kind | Construction | Answer |
| --- | --- | --- | --- |
| W2.1 | narrowing | a key written twice in one mapping, `persist-credentials: false` then `true`: the gate saw both, and which one a parser keeps is the parser's | refused, naming both lines; RED arm, red without the refusal |
| W2.2 | narrowing | two keys of one mapping that differ only in case: GitHub "converts input names to uppercase letters" (its metadata syntax reference, the ledger's `github-actions-syntax`), so `PERSIST-CREDENTIALS: true` meets `persist-credentials: false` in one variable | refused; RED arm, red without the refusal |
| W2.3 | narrowing | a quoted key, which the gate unquoted but which carries a second escape grammar for single quotes | refused, every workflow here writing its keys plain; RED arm, red without the refusal |

Round 3 is the next: a review of the narrowed reader by a reader rather than an attacker — the gate's subset, set
beside GitHub's workflow schema and the YAML specification, asking what it accepts that GitHub reads otherwise — or
by a reviewer the director names (findings §11 already asks for a second reviewer identity).

**Round 3**, `2026-10-02`, was that reading: a new read-only context, asked to compare the gate's accepted subset with
the YAML 1.2.2 specification and GitHub's workflow and metadata syntax references, quoting them, with no
proof-of-concept file written. It read those, and GitHub's published workflow reader code, and was stopped by the
agent harness's safety screening before its findings were delivered; no findings, count or verdict exist. It is
recorded as not completed, as round 2 is, and its absence of findings is not a verdict. It handed back one remark
without findings: that the syntax pages as published had changed since the gate's header was written. Checked the
same day for the page the gate cites: the metadata syntax reference fetched again is byte-identical to the ledger's
`github-actions-syntax` (sha256 `94bf89e3…`), and both of its quoted sentences stand, so nothing in the header
moves. The workflow syntax reference is cited by no gate and no ledger entry.

Two agent rounds having ended this way, an agent does not launch a fourth. The next round is a reviewer the director
names (findings §11).
