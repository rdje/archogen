# `main` protected on the personal account: the ruleset, the settings and the batch workflow

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — confirmed by the director, "Apply now", and §1 applied on `2026-10-28` (`M2.7.6.6.3`): the
  ruleset is id `25999805`, and each setting reads back equal to this record
- **Owner / source:** findings §11 (`decision_findings-for-director-review.md`), ruled by the director on `2026-10-27`:
  - "Protect main + reviewer";
  - kept on the personal account `rdje`: "All of my project are on Personal Account";
  - a second human reviewer being "difficult to find", "Me, agents on a machine acct".

  Leaf `M2.7.6.6` (`docs/tasks/M2.md`); `M2.7.6`'s requirements of the hosting.

## The fact / decision

In plain words: on GitHub, `main` changes only when a pull request is merged, after the required checks pass and the
director approves it. The agents keep committing on their local `main` as before. When a batch is due, they push it to
a branch of its own, open one pull request with an independent agent's review of it, and GitHub merges it once the
director has approved. Nobody pushes to `main` directly, the director included.

### 1. What is applied, exactly

**The repository's settings** (`PATCH /repos/rdje/archogen`), so the hosting makes every merge commit and no other
kind of merge:

```json
{ "allow_merge_commit": true, "allow_squash_merge": false, "allow_rebase_merge": false, "delete_branch_on_merge": true }
```

Two settings §11 asks for already hold, read on `2026-10-27` (`GET /repos/rdje/archogen/actions/permissions/workflow`):
`"default_workflow_permissions":"read"`, and `"can_approve_pull_request_reviews":false`.

**The ruleset** (`POST /repos/rdje/archogen/rulesets`), whose bypass list is empty, so it binds the director's own
account too:

```json
{
  "name": "main",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": { "ref_name": { "include": ["~DEFAULT_BRANCH"], "exclude": [] } },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" },
    { "type": "pull_request", "parameters": {
        "required_approving_review_count": 1,
        "dismiss_stale_reviews_on_push": true,
        "require_code_owner_review": true,
        "require_last_push_approval": true,
        "required_review_thread_resolution": false,
        "required_reviewers": [],
        "require_extra_approval_for_unattributed_changes": true,
        "allowed_merge_methods": ["merge"] } },
    { "type": "required_status_checks", "parameters": {
        "strict_required_status_checks_policy": true,
        "do_not_enforce_on_create": false,
        "required_status_checks": [
          { "context": "enforce", "integration_id": 15368 },
          { "context": "focused", "integration_id": 15368 },
          { "context": "integration", "integration_id": 15368 },
          { "context": "trust-gate", "integration_id": 15368 },
          { "context": "catalog-check", "integration_id": 15368 } ] } }
  ]
}
```

- The checks are each job CI runs on a pull request: `enforce` (`doctrines.yml`), `focused` and `integration`
  (`rust.yml`), `trust-gate` (`trust-gate.yml`) and `catalog-check` (`catalog-check.yml`).
- `15368` is the `github-actions` app, read from `origin/main`'s own check runs on `2026-10-27`. A status from any
  other source does not satisfy a check.
- "Branches up to date" (`strict_required_status_checks_policy`) runs the checks on the result of the merge, which is
  §11's alternative to a merge queue. A merge queue is "available on private and public repos on the GitHub Enterprise
  Cloud plan and all public repos owned by organizations" ([merge queue is now generally
  available](../book/src/ledger.md#github-merge-queue)), so not on this account.
- Two keys of the `pull_request` rule were not sent. GitHub fills them on create, and the record writes them as read
  back:
  - `required_reviewers`, the teams a path's change needs, is empty: this account has no teams, and `CODEOWNERS`
    names the owner;
  - `require_extra_approval_for_unattributed_changes` is GitHub's default, "enabled by default, for both new and
    existing rulesets". "When Copilot opens a pull request that isn't attributed to a person, the ruleset requires
    one more approval than the number you configured" ([Available rules for
    rulesets](../book/src/ledger.md#github-available-rules)). No Copilot opens pull requests here, so it binds
    nothing; it is kept, since turning it off would only weaken a rule.

**Applied `2026-10-28`** (`M2.7.6.6.3`), after the director's "Apply now", with `gh` as the director's account:
- the settings by `PATCH /repos/rdje/archogen`;
- the ruleset by `POST /repos/rdje/archogen/rulesets`, which answered id `25999805`.

Read back the same day:
- `GET /repos/rdje/archogen/rulesets/25999805` equals the JSON above, key for key;
- the settings read `allow_merge_commit=true allow_squash_merge=false allow_rebase_merge=false
  delete_branch_on_merge=true`;
- `GET /repos/rdje/archogen/rules/branches/main`, the rules the hosting applies to `main`, lists
  `deletion,non_fast_forward,pull_request,required_status_checks`.

A direct push to `main` is not tried, since a push the rules failed to refuse would land on `main`. The hosting's own
list of the rules it applies to `main` stands for that test. `M2.7.6`'s scratch-branch tests try the rest.

**`.github/CODEOWNERS`**, the only ownership file (`M2.7.6.5`). Every path is owned by the director, so every pull
request needs the director's approval. The doctrine `CODE-OWNERS` (`scripts/check_code_owners.sh`) holds the file to
this block byte for byte, and to the syntax GitHub reads:

```text
# Every path is the director's: docs/decisions/decision_main-protection.md §1.
* @rdje
```

**The machine account** is a collaborator with the **Write** role, not Admin, so it can push branches and open pull
requests, and can neither approve its own nor change the rules.

### 2. The machine account: the director's steps

GitHub's terms allow it: "You may maintain no more than one free machine account in addition to your free Personal
Account", and "A machine account is used exclusively for performing automated tasks" ([GitHub's terms of service,
machine account](../book/src/ledger.md#github-machine-account)).

Done `2026-10-28` (`M2.7.6.6.2`), each step read back:
1. The account is **`rdje-bot`**, created by the director (GitHub user id `349045346`).
2. It is a collaborator of `rdje/archogen` with the **Write** role (`GET /repos/rdje/archogen/collaborators`:
   `"role_name":"write"`; its token reads `"admin":false,"push":true`).
3. Its credentials, none written into the repository:
   - **for pushing**, the key `~/.ssh/id_ed25519_rdje_bot`, generated on this machine without a passphrase, since the
     agents push unattended; its public half is on `rdje-bot`. The alias `github.com-rdje-bot` in `~/.ssh/config`
     alone uses it. `ssh -T git@github.com-rdje-bot` answers "Hi rdje-bot!";
   - **for pull requests**, a classic token of `rdje-bot` with the one scope `public_repo`, stored by the director in
     the macOS Keychain as `archogen-rdje-bot-token` without passing through the session. A classic token, because
     "Only personal access tokens (classic) have write access for public repositories that are not owned by you", and
     fine-grained ones still lack the right "to contribute to repositories where the user is an outside or repository
     collaborator" ([Managing your personal access tokens](../book/src/ledger.md#github-personal-access-tokens)).
4. This repository's local git configuration:
   - the remote **`agent`**, `git@github.com-rdje-bot:rdje/archogen.git`;
   - the commit author `rdje-bot <349045346+rdje-bot@users.noreply.github.com>`.

   `origin` and the director's own key are unchanged.

### 3. The batch workflow

1. The agents commit on local `main`, under `COMMIT.md`'s workflow and every hook, as before.
2. A batch is due at the push cadence (`bash scripts/push_cadence.sh` at its ruled threshold), or to restore failing
   CI, as the cadence ruling already says, and the director is told first.
3. A fresh agent context reviews the batch's diff and writes a summary of what changed and what it found.
4. `bash scripts/open_batch.sh` (`M2.7.6.6.4`):
   - pushes local `main` to `batch/<YYYY-MM-DD>-<n>` as the machine account;
   - opens one pull request into `main`, its body the summary and the commit list;
   - turns on auto-merge with a merge commit.
5. CI runs the required checks on the merge result. The director reads the summary and the diff, and approves.
6. GitHub makes the merge commit, signed with its own key, and deletes the branch.
7. The agents fast-forward local `main` to `origin/main` (`git merge --ff-only origin/main`). A merge commit has
   their batch as its second parent, so local `main` never diverges.

The cadence keeps its meaning: the number of commits local `main` is ahead of `origin/main`.

### 4. What it guarantees, and what it does not

**Guaranteed by the hosting:**
- `main` changes only by a merged pull request;
- its required checks passed, from `github-actions`, on the merge result;
- the director approved it after its last push;
- no force-push and no deletion;
- the director's own account bound like any other.

**Residuals, each stated:**
- **A spoofed status.** A personal account has no ruleset-required workflows, so a pull request's own copy of the
  workflows runs, and could post a status of a required check's name. `CODEOWNERS`' `*` makes the director's approval
  needed for every pull request, one adding or editing a workflow among them, so such a status merges nothing without
  the director. `M2.7.6`'s design states this, and its next review round reads it (`M2.7.6.4`).
- **An approval is only as good as its review.** Each pull request carries an independent agent's summary, and the
  director's approval rests on it and on the diff. The author is the agents and the approver the director, so the
  roles differ, as §11 asks: "Code-owner approval must come from an identity other than the author's".
- **The owner keeps the owner's power.** `rdje` can edit the ruleset. That is recorded, not prevented.
- **Until `CODEOWNERS` is on `main`.** "For code owners to receive review requests, the CODEOWNERS file must be on
  the base branch of the pull request" ([About code owners](../book/src/ledger.md#github-code-owners)). Until the
  first batch has merged it (`M2.7.6.5`), any approval by an account with write access counts:
  - the agents' account could then approve a pull request another account opened, and the agents approve none;
  - every pull request the agents open still needs the director, the one other account with write access.
- **The director's own pull requests.** "Pull request authors cannot approve their own pull requests" ([Approving a
  pull request with required reviews](../book/src/ledger.md#github-approving-reviews)). Every path is the
  director's, so a pull request the director opens cannot merge under these rules. The director's changes reach
  `main` through the agents' batches, or by the owner's power above.

### 5. The named commit

The catalog's premise 3 holds from the first merge commit made under these rules, recorded where the claim tooling
holds it (`M2.7.6.6.5`). From then, a production claim refuses any first-parent commit of `main` that is not a merge the
hosting made and signed (§11).

## Why

- §11's settings are the only way to make CI block a change, rather than report it after it has landed. The catalog's
  facts are trusted by every later build.
- Staying on the personal account loses one setting, ruleset-required workflows. Code ownership of every path gives
  the same protection for the price of the director's approval, which the director gives anyway.
- The director as approver, with the agents on a machine account, separates authoring from approving without a second
  person.

**Alternatives weighed:** an organization on the Team plan (pinned workflows; declined, the director's projects stay
on personal accounts); an automated reviewer as approver (no human in the loop); no approver at all (a weaker premise
3).

## How to apply

- The director confirmed this record (`M2.7.6.6.1`) and created the machine account (`.2`); `.3` applied §1 with
  `gh` and read each setting back equal to it.
- A change to the hosting's settings is a change to this record first, applied the same way and read back.
- Related: [[decision_findings-for-director-review]] §11, [[decision_push-cadence]],
  `docs/specs/catalog/decision_catalog-records.md` (premise 3).
