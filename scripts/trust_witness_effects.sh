#!/usr/bin/env bash
# M3.6.9.5.4.2: expose compiled semantic-claim controls to scripts/*.sh --self-test discovery.
set -euo pipefail
exec python3 "$(dirname "$0")/trust_witness_effects.py" "$@"
