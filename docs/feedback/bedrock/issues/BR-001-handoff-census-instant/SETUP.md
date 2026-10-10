# Reproducing BR-001

Self-contained. Everything this issue needs is described here and held in this directory; nothing outside it is read.

## Environment the observation was measured on

| Component | Revision |
| --- | --- |
| bedrock | `835547e`, its census last changed in `a860103` |
| bash | the platform's `bash` on the path |
| Platform | Darwin arm64 |

## What the reproducer needs

A bedrock checkout. It runs that checkout's `scripts/check_no_background_jobs.sh` twice, read-only, and starts one
helper of its own — a shell that waits on a four-second `sleep`, its command line naming the checkout — so the census
has a short-lived process to judge. It writes no file. Other processes of yours that name the checkout may make either
run of the census refuse; run it with none.
