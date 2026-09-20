# Reproducing LS-001

Self-contained. Everything this issue needs is described here and held in this directory;
nothing outside it is read.

## Environment this was measured on

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

## 1. Get a LinkedSpec checkout

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout ad290bdb4
```

The nested dependencies are needed for this issue:

```sh
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

`git submodule update --init --recursive` also works but pulls optional repositories this
issue does not need.

## 2. Run the reproducer

```sh
bash repro.sh /path/to/linkedspec
```

## Verdict

The exit code is the verdict — no output parsing needed:

| Exit | Meaning |
| --- | --- |
| `0` | The recorded observation still reproduces — the defect is present |
| `3` | Behaviour **changed**; this may mean the issue is fixed |
| `2` | Could not run (missing argument or prerequisite) |

The run recorded at the revisions above is frozen in [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt).
