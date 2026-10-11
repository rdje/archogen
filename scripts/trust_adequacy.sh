#!/usr/bin/env bash
# M3.6.9.5.2.1: expose the Python diagnostic's --self-test to the integration runner's scripts/*.sh census.
set -euo pipefail
exec python3 "$(dirname "$0")/trust_adequacy.py" "$@"
