# Push cadence: `N = 400` commits ahead of `origin/main`

- **Type:** `decision`
- **Date:** `2026-09-28`
- **Status:** `active`
- **Owner / source:** director ruling, `2026-09-28`, in answer to "what is N in this project?" — which
  had no answer: the layer-A template provides the field and nobody had ever filled it in
- **Threshold:** `PUSH_AT_COMMITS_AHEAD=400` — the one machine-readable copy. `scripts/push_cadence.sh` reads it,
  refuses a second copy in this record or a title that disagrees with it, and reports the live distance

## The decision

Push when the branch reaches **400 commits** ahead of `origin/main`. The threshold lives in **one
machine-readable place** and is reported by a check, so the number has a single producer rather than
a copy in every document that mentions it. Since `PROGRAM.23` that place is this record's **Threshold**
field, and `bash scripts/push_cadence.sh` is the check: it exits `0` below the threshold, `3` when a push is
due, and `2` when it cannot tell.

No push happens without the director's explicit authorization — this decision sets *when a push is
due*, not a standing permission to perform one.

## Why

`MEMORY_ARCHITECTURE.md` gives the rationale and no number: "**Push regularly** — the remote is your
crash insurance; an unpushed commit dies with the machine" (`:239`), "The single point of failure is
**not committing / not pushing**" (`:432`), and layer 1 survives a machine loss only because "it's
committed, and pushed it's off-machine" (`:57`). Its resume-pointer template even carries the slot —
`:193`, `(ahead of origin: <N>; push at ~<threshold>)` — and archogen left it empty, so the cadence
existed only as an operator instruction (batch BWFSC, default 100 slices) that the PNT loop can never
trigger, because PNT has no fixed BWFSC by definition. Measured consequence: the branch went
unpushed from `32e6b14` (`2026-09-13`) at roughly 4.9 commits/day.

**The recommendation put to the director was different, and is recorded so the choice is visible
rather than silent:** `25` commits **or 7 days, whichever comes first**. The reasoning was that crash
insurance scales with *elapsed time*, not with commit count — a fixed N halves the exposure window if
the rate doubles and never fires at all if work stalls — and that at the measured rate `400` is
roughly **82 days** of work existing only on one volume, which is what `MEMORY_ARCHITECTURE.md:104`
calls "survives nothing". The director chose `400`. That is a legitimate call: it trades durability
against push overhead and review churn, and the trade is the director's to make. It is recorded here
so a future session does not re-litigate it, and so the number is not mistaken for a measurement.

## How to apply

- **The threshold is one number with one producer**, this record's field. `PROGRAM.23` put it where a check
  reads it and has `MEMORY.md` point at the check, rather than letting `400` be retyped into every document that
  mentions pushing — which is precisely the defect `M1.23`, `M1.24` and `S0.8` each fixed, and
  [[a-moved-measurement-needs-a-census-of-its-copies]] is the card. An ungated threshold is prose:
  [[a-rule-only-in-the-prompt-is-enforced-nowhere]].
- ⛔ **The check must not be able to deadlock the repository.** A hard gate at `N` that refuses
  commits would strand the work it is meant to protect, and the remedy it demands — push — has its own
  precondition: `COMMIT.md` step 2 requires `make integration` before a push, which currently **fails**
  on the emulator step. So at `N`, a blocking check plus a red integration tier means the repository
  can neither commit nor push. `PROGRAM.10` (reclassify the emulator step per §14.3's quarantine
  clause) must therefore land **before** `PROGRAM.23`'s check is given teeth, or the check must report
  loudly without blocking until it has.
  *Amended `2026-09-30`:* the reclassification landed in `PROGRAM.10.1` — the emulator step is now
  quarantined under `M2.8`, so `make integration` reads `incomplete` rather than `failed`, and step 2
  permits a push past it after reading what it names. The premise above is lifted for that step; any
  other failing step still blocks, as it should.
- **The live count is never written down.** It moves with every commit, so documents point at
  `bash scripts/push_cadence.sh` instead of quoting a number. This record states the *threshold*, which only moves by decision.
- ⭐ **The check reports and never blocks** (`PROGRAM.23`). Nothing on the commit path or in a tier calls it, so no
  threshold can strand a commit; "due" is its exit `3` and a loud line for whoever asks.
- Reaching `N` is a prompt to push, not an automatic push. Read `make integration` first, and if it
  reports `incomplete`, read what it names before deciding to proceed (§14.3).
