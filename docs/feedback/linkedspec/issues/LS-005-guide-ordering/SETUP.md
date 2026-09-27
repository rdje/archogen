# Reproducing LS-005

Self-contained. Everything this issue needs is described here and held in this directory;
nothing outside it is read.

## Environment the original observation was measured on

| Component | Revision |
| --- | --- |
| LinkedSpec | `ad290bdb427bc19a5af81de0f0b07e119c8999ff` |
| RGX | `8763a0e6bea97879f027237439d57725f83ead23` |
| PGEN | `db6f8c6836fefa5a57b1337d3ffbf6f15774089f` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |

LinkedSpec `ad290bdb4` is the commit whose message is
`BACKEND-INTEGRATION-GUIDES.2.2 - deliver Rust Lispish file integration and deployment`.

## Environment the re-measurement was taken on

| Component | Revision |
| --- | --- |
| LinkedSpec | `2ac834913d85c32f532be9b0aab63644838a577a` — the consumer's adopted pin |
| RGX | `f6e5acdc99720349d1e3ecef9f821f365c4db19c` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |

Only the guide is read, so the nested revisions cannot affect either measurement; they are
recorded because a pin is a whole-checkout state.

## 1. Get a LinkedSpec checkout

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout ad290bdb4      # the original observation
git -C linkedspec checkout 2ac834913      # the re-measurement
```

This issue reads only the guide; no submodules and no build are needed.

## 2. Run the instruments

```sh
bash repro.sh     /path/to/linkedspec   # does the frozen observation still reproduce?
bash remeasure.sh /path/to/linkedspec   # is the reported defect gone at this revision?
bash remeasure.sh --self-test           # does that verdict discriminate? (4 arms)
```

## Verdict

⚠️ The two instruments answer different questions, so their exit codes mean **opposite things**.

`repro.sh` — "does the recorded observation still reproduce?":

| Exit | Meaning |
| --- | --- |
| `0` | The frozen observation still reproduces — the defect is present |
| `3` | Behaviour **changed**; that may mean fixed, or only that the guide was renamed |
| `2` | Could not run (missing argument or prerequisite) |

`remeasure.sh` — "is the reported defect gone at this revision?":

| Exit | Meaning |
| --- | --- |
| `0` | Gone — the section sends the reader to preparation before any metadata or build command |
| `1` | Still present |
| `2` | Could not run (no guide, or a section shape the instrument refuses to guess at) |

The run recorded at the original revisions is frozen in
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt). The re-measurement at `2ac834913` — both
instruments' output, the self-test arms, and the guide lines that carry the verdict — is frozen in
[`evidence/REMEASURED.txt`](evidence/REMEASURED.txt).
