# Every other git repository is strictly read-only — in both directions

- **Type:** `decision`
- **Date:** `2026-09-27` (absolute — never "yesterday"/"last week")
- **Status:** `active`
- **Owner / source:** director directive (session startup §20–§21), the director's clarification of
  `2026-09-27`, and a published upstream disclosure: LinkedSpec
  `8b5b5ffd8ea415b9b6d97387da8289e8f18606f3`, "RGX-CONSUMER-BUILD-REPORTS.1.3.1 — record
  publication and repository-boundary violation", dated `2026-09-27`.

## The fact / decision

The repository boundary runs **in both directions**, and both halves are binding:

- **Outbound.** No archogen agent may create a commit, branch, tag, or file inside **any** git
  repository other than this one — including the vendored checkouts under `vendor/`. Reading
  another repository, fetching its published history, checking out a commit it has published,
  building it by its documented route, and moving **our own** submodule pin to such a commit are
  all consumption and are permitted. Writing there is a breach.
- **Inbound.** A change delivered into this repository by another project's agent is not thereby
  archogen's. It lands through a task-tree leaf that records **who authorized it, what it touched,
  and what was preserved**, and it may not set a verdict that only archogen's own measurement can
  set.

## Why

### The incident, correctly stated

⚠️ **This record was first written with the direction reversed, and the correction is kept visible
rather than silently applied.** The first version read the upstream disclosure as a claim that
*archogen* had written into LinkedSpec. The director corrected it on `2026-09-27`: **LinkedSpec's
agent modified a few `.md` files in *this* repository** to notify archogen that the logged bug
reports had been fixed. The director objected to LinkedSpec, which has since made every other
repository strictly read-only in its own bootstrap and durable memory, preserved an unapplied
recovery patch on its side, and left archogen in control of its own tree. It was a one-time error
and is not expected to recur.

So the crossing was **inbound**. What was audited here, with tools rather than assumption:

| Question | Command | Result |
| --- | --- | --- |
| Did a foreign-authored commit land here? | `git log -12 --format='%h \| A:%an <%ae> \| C:%cn <%ce>'` | No — every commit carries the single local identity |
| Was a commit created and then discarded? | `git reflog --date=short` | Linear: only `commit:` and `checkout:` entries; no `reset`, `rebase` or `amend` |
| What did the inbound content touch? | `git show --name-only 82ee99a` | `docs/feedback/linkedspec/**` plus archogen's own live docs (`CHANGELOG.md`, `DEV_NOTES.md`, `MEMORY.md`, `docs/tasks/M1.md`) |
| Did it reach any code path? | same, filtered | **No** — no `crates/`, `scripts/`, `xtask/`, `Cargo.*` or `Makefile` path |
| Is anything broken? | `make focused`; `cargo test --all` | exit `0`; **421 passed, 0 failed** over 36 suites — the same count as at `M1.11`, as expected since no crate depends on the vendored checkout |

⭐ **The inbound write was handled correctly, and that is worth stating as precisely as the
error.** Leaf `M1.18` recorded the authorization ("the director asked LinkedSpec to notify both
the director and ARCHOGEN"), preserved the original observations by hash (36 non-state feedback
files SHA-256 unchanged), and **refused to let the notice set `verified`** — "Only ARCHOGEN may set
verified after its own adoption and rerun". An external agent's claim about its own fix entered
the tree as a *claim*, attributed and quarantined, not as a result.

### The part of the original finding that survives

The outbound rule was **nowhere in the committed repository**: `grep -rn 'READ-ONLY' CLAUDE.md
AGENTS.md` → no match, and neither `README.md`, `DOCTRINE_ENFORCEMENT.md` nor
`scripts/check_doctrines.project.sh` stated it. It existed only in the director's session prompt.

⛔ **The measured cost of that absence is not hypothetical — it is this record's first draft.** An
agent that resumed from git alone would not have known the boundary existed, and an agent working
from the prompt alone mis-attributed the direction of a published incident and committed a durable
record saying so. `MEMORY_ARCHITECTURE.md`: *information that exists only in the live conversation
is not yet saved.* `DOCTRINE_ENFORCEMENT.md`: *a rule that lives only in a doc is a suggestion.*
A rule that lives only in a prompt is weaker than either.

## How to apply

1. **Never write across the boundary.** No commit, branch, tag, stash, config change, or file
   creation in another repository or in a vendored submodule's own history. This includes
   "helpfully" fixing an upstream bug where it lives — report it through
   `docs/feedback/<vendor>/` instead, which is what the LS-001 … LS-007 register is for.
2. **A pin move is consumption, and it is owned.** Moving `vendor/<submodule>` to a commit the
   upstream has published is permitted on the director's instruction, is recorded in a task-tree
   leaf, and is committed **in this repository** (the gitlink is our file, not theirs). Leaf
   `M1.19` owns the LinkedSpec move to `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`.
3. **Submodules stay black boxes** (§20 of the director's directives): integrate through their
   published instructions, public APIs and contracts; do not inspect or reconstruct their
   internals, even though the source is on disk. Having access is not permission.
4. **An externally delivered change is a claim, not a result.** Give it a leaf, record the
   authorization and the files it touched, preserve what it overwrote, and never let it move a
   verdict that archogen's own measurement owns.
5. **Owned follow-up:** `PROGRAM.11` states both directions in the durable docs a resuming agent
   actually reads, and adds a mechanical check for the outbound symptom that is visible from here —
   a vendored checkout carrying local commits or local modifications.

Related: [[decision_findings-for-director-review]] (what needs the director, not an agent).
