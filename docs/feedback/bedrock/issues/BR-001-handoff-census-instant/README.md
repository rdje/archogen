# BR-001 — The handoff census judges one instant

| Field | Value |
| --- | --- |
| **ID** | `BR-001` |
| **State** | `open` |
| **Severity** | Minor |
| **Kind** | Robustness |
| **Component** | `scripts/check_no_background_jobs.sh` |
| **Affects** | bedrock `835547e`, the census last changed in `a860103` |
| **Reproducer** | [`repro.sh`](repro.sh), the observation in [`evidence/`](evidence/) |
| **Reported by** | archogen |

## Summary

The census refuses a handoff while any process of this user names the checkout on its command line or holds a file
under it. It looks once. A process that lives a few seconds is named if it happens to be alive at that instant, and
not otherwise, so the verdict depends on when the census runs. Measured `2026-10-05` by three review readers of
archogen: iTerm's `pidinfo --git-state <repo> 4 1`, respawned by the terminal every few seconds, each 4–5 s old when
seen, made the census refuse at one run and pass at the next. Measured `2026-10-10` on bedrock `835547e` by
[`repro.sh`](repro.sh): a helper alive 4 s, named and refused; the same census a moment after it exited, passing.

## Proposal

Count a process only when a second look, a few seconds after the first, still holds it — the same PID and command
line. A job that can rewrite tracked files outlives a few seconds; a helper does not. It is a property, as the census's
own header asks — *"detection is now a property, not a vocabulary"* — where an ignore list, `.doctrine/handoff_ignore`
in `835547e`, is a vocabulary that must name each helper ahead of time, every terminal's own.

archogen samples twice in a wrapper of its own, beside the census, since its copy of the census is the template's and
an update would undo an edit; the census itself is bedrock's to change.

## Reproduce

```text
bash repro.sh /path/to/bedrock
```

Exit 0: reproduced — the census refused while a process of a few seconds' life ran, and named it. Exit 3: changed — it
did not. Exit 2: could not run. [`SETUP.md`](SETUP.md) has the environment; [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt)
the run this report was written from.

## History

- `2026-10-10` — archogen: reported, measured on bedrock `835547e` (reproduced, exit 0).
