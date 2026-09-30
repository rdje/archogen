# LS-008 — one successful preparation writes a 784 MB log, and the guide says to keep it

| Field | Value |
| --- | --- |
| **ID** | `LS-008` |
| **State** | `open` |
| **Severity** | Minor — archogen's proposal. Whether this is a defect or a documented cost is LinkedSpec's call |
| **Kind** | Cost |
| **Component** | RGX's published preparation interface, `make -C <checkout>/rgx bootstrap`, and the PGEN generator's progress output |
| **Affects** | LinkedSpec `2ac834913` / RGX `f6e5acdc9` / PGEN `d9d41c28` |
| **Reproducer** | [`repro.sh`](repro.sh) — replay a captured log (`--log`), or run the preparation (`--run`) |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

One **successful** run of the published preparation interface, fresh after a pin change, wrote a log of
**784 062 751 bytes** in **4 008 986 lines**. Of those, **3 971 805** lines (782 326 257 bytes) are PGEN's own
tagged progress output: `[PGEN][DBG]`, `[PGEN][LOW]`, `[PGEN][HIGH]` and `[PGEN][MED]`. The build's
other output, including every compiler warning, is 37 181 lines.

The Rust integration guide says of this command: *"If the command fails, stop before building the
application and preserve its exit status and full log."* That sentence is about a failed run, and this
report measures a successful one, which is the whole run. A failed run writes some part of it, depending on
how far it gets; that was not measured here. A run that fails after the generator has done most of its work
leaves a log of this order, which a consumer following the guide must keep for each failed attempt. On a
small volume or a CI runner, that is a disk filled by the diagnostic step. A consumer that does not keep it
loses the evidence the guide asks for.

The exact figures, how they were taken, and the log's first and last lines are frozen in
[`evidence/MEASURED.txt`](evidence/MEASURED.txt).

## Why this is not a defect report about correctness

The preparation is correct. It exits `0`, and it produces the parser it promises. This report is about what
it costs to follow the guide's failure procedure. It was found while re-measuring an earlier report, and
kept out of that report's verdict because it is a property of the interface, not evidence about that report.

## Proposed remedy — for LinkedSpec to choose from, not an implementation

- a quiet or log-level control on the published interface, so a consumer can keep the lines that explain a
  failure without keeping the generator's per-step progress; or
- a documented volume expectation beside the guide's failure procedure, so a consumer can provision for
  it, or keep only the head and tail with that in mind.

Either would make the guide's failure procedure affordable. Neither is prescribed here.

## The measurement

Taken `2026-09-30` on Darwin arm64 with rustc 1.95.0, at the revisions above: one fresh run of `make -C
<checkout>/rgx bootstrap` through the documented storage wrapper, with the generated parser and PGEN's
`rust/target` removed first, as the published contract says after a pin change. The run exited `0` and
produced its twelve generated files. The same figures for bytes, lines and `[PGEN][DBG]` lines were first
observed on `2026-09-27` at the same revisions, and this run reproduced them exactly.

## History

- `2026-09-30` — archogen: filed. Measured on a fresh run at `2ac834913` / RGX `f6e5acdc9` / PGEN `d9d41c28`.
  The first observation, on `2026-09-27`, was recorded in archogen's re-measurement of `LS-004`. It gave the
  line count once as 4 008 986 and once, by a transcription error in that report's evidence, as 41. This run
  settles it at 4 008 986.
