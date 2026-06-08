#!/usr/bin/env bash
set -euo pipefail

if [[ $# -eq 0 ]]; then
  exit 0
fi

binary="$1"

if [[ -f "$binary" && ! -x "$binary" ]]; then
  chmod +x "$binary"
fi

exec "$@"
