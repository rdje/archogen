#!/usr/bin/env bash
# M3.6.9.5.4.1: register the evidence diagnostic's --self-test in the scripts/*.sh census.
set -euo pipefail
exec python3 "$(dirname "$0")/trust_witness.py" "$@"
