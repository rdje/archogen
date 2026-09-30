#!/usr/bin/env bash
# scripts/no_std_build.sh — the integration tier's `no-std-build` step: compile the runtime core for the pinned
# target's Rust triple (leaf `M2.8.3.3`).
#
# ⭐ WHY A SCRIPT. The triple was a literal in `xtask/src/main.rs`, and §3.2's "explicit ISA feature set" belongs in
# the pinned target file, where a reader looks for it. So the triple is declared once, as `RUST_TARGET` in
# `targets/riscv-virt-up.env`, and this step reads it from there. The runner reads the same key to say that the
# target is not installed, rather than failing the build (`requires: target-env:…` in `xtask`).
#
# CONTRACT: cargo's exit code. Run from the repository root.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
ENV_FILE="targets/riscv-virt-up.env"
set -a; . "$ENV_FILE"; set +a
[ -n "${RUST_TARGET:-}" ] || { echo "no-std-build: $ENV_FILE declares no RUST_TARGET" >&2; exit 1; }
exec cargo build --quiet -p rt-core --target "$RUST_TARGET"
