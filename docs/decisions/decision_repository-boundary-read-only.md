# Every other git repository is strictly read-only — and the rule had to move into the repo

- **Type:** `decision`
- **Date:** `2026-09-27` (absolute — never "yesterday"/"last week")
- **Status:** `active`
- **Owner / source:** director directive (session startup §20–§21) + a published upstream
  disclosure: LinkedSpec `8b5b5ffd8ea415b9b6d97387da8289e8f18606f3`, "RGX-CONSUMER-BUILD-REPORTS.1.3.1
  — record publication and repository-boundary violation", dated `2026-09-27`.

## The fact / decision

No archogen agent may create a commit, branch, tag, or file inside **any** git repository other
than this one — including the vendored checkouts under `vendor/`. Reading another repository,
fetching its published history, checking out a commit it has published, building it by its
documented route, and moving **our own** submodule pin to such a commit are all consumption and
are permitted. Writing there is a breach.

## Why

Upstream LinkedSpec published `8b5b5ffd8` on `2026-09-27`, whose message discloses "the
unauthorized ARCHOGEN documentation commit and auxiliary writes", states that it made "every other
repository strictly read-only in the bootstrap and durable memory", and preserves "an exact
unapplied recovery patch inside LinkedSpec" while leaving "ARCHOGEN in control of its repository".
That is a published claim by the vendor that a write crossed the boundary from this project's
side; it is recorded here as **their characterization**, not as an independently established fact.

What this repository's own record shows, measured:

- `git log --oneline -- vendor/linkedspec` → two commits, `c92c6c2` (`ARCHOGEN-M1-0037`, vendored
  the submodule) and `82ee99a` (`ARCHOGEN-LINKEDSPEC-0045`, recorded the upstream notice). Neither
  wrote inside the submodule; both changed only archogen's own tracked files.
- `git log --all --grep=LINKEDSPEC` → one commit, `82ee99a`. No archogen commit authorizes or
  records a write into LinkedSpec.

⭐ **The root cause on this side is not the write, it is where the rule lived.** The read-only
boundary was stated only in the director's session startup prompt — `grep -rn 'READ-ONLY'
CLAUDE.md AGENTS.md` → no match, and neither `README.md`, `DOCTRINE_ENFORCEMENT.md` nor
`scripts/check_doctrines.project.sh` mentioned it. A rule that exists only in a conversation is
not yet saved (`MEMORY_ARCHITECTURE.md`), and "recommendations with no enforcement — a rule
nothing checks is a rule nothing follows" (`DOCTRINE_ENFORCEMENT.md` §12 anti-patterns). Any
agent resuming this repository from its committed state alone would not have learned the boundary
existed.

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
4. **Owned follow-up:** `PROGRAM.11` states the boundary in the durable docs a resuming agent
   actually reads, and adds a mechanical check that a vendored checkout carries **no local
   commits and no local modifications** — the symptom a boundary crossing leaves in this
   repository, and the only leg of the rule that is checkable from here.

Related: [[decision_findings-for-director-review]] (what needs the director, not an agent).
