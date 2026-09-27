# Reproducing LS-001

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
| PGEN | `d9d41c28dca86dd9ec4a6f3668c3a8c71cecf97d` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |
| Consuming application | a Cargo **workspace** root with explicit members — the report's case |

## 1. Get a LinkedSpec checkout

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout 2ac834913      # the re-measurement; ad290bdb4 for the original
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

Both nested checkouts are needed, because the second manifest under test lives in `subs/pgen`.
`git submodule update --init --recursive` also works but pulls optional repositories this issue does
not need. At this revision the vendor's guide routes *preparation* through RGX's public bootstrap;
initializing the nested checkout here is only what makes the manifest exist to be probed.

## 2. Run the instruments

`remeasure.sh` needs a consuming application whose root is a Cargo workspace. By default it takes the
repository the script lives in; pass `--app-root` to measure another one.

```sh
bash remeasure.sh /path/to/linkedspec                  # verdict at this revision; writes nothing
bash remeasure.sh /path/to/linkedspec --app-root /path/to/app
bash remeasure.sh --self-test                          # 5 synthetic arms, no checkout needed
bash repro.sh /path/to/linkedspec                      # the historical observation
```

`repro.sh` copies the checkout into `.repro-work/` under this directory before touching anything, so
the checkout you pass it is never modified. Its Part 2 patches the *copy's* manifests to demonstrate
the proposed fix; the vendor's guide at the adopted revision forbids that as a remedy, so Part 2 is
historical and `remeasure.sh` does not reproduce it.

## Verdict

⚠️ The two instruments answer different questions, so their exit codes mean different things.

`remeasure.sh` — "does the documented layout resolve inside a Cargo workspace?":

| Exit | Meaning |
| --- | --- |
| `0` | The defect is **gone** — every manifest the documented route touches resolves |
| `1` | **Still present** — a manifest collides with the application workspace |
| `2` | Could not run, or not applicable: no cargo, a manifest missing, an application root with no `[workspace]`, or a failure that is not this defect |

`repro.sh` — "does the recorded observation still reproduce?":

| Exit | Meaning |
| --- | --- |
| `0` | Both halves behave as recorded — the defect is present and the proposed patch fixes it |
| `1` | Did not behave as described, which at a fixed revision is the expected outcome |
| `2` | Could not run (missing argument or prerequisite) |

The run recorded at the original revisions is frozen in
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt). The re-measurement at `2ac834913` — self-test,
the BEFORE arm without the consumer's documented exclusion, the AFTER arm with it, and a census
showing the consuming workspace itself unchanged — is frozen in
[`evidence/REMEASURED.txt`](evidence/REMEASURED.txt).
