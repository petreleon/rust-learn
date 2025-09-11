#!/bin/sh
set -e

STATE_DIR=/anvil/state

echo "[entrypoint] Preparing /anvil directory" >&2
mkdir -p /anvil "$STATE_DIR"
chmod 775 /anvil "$STATE_DIR" || true
if id foundry >/dev/null 2>&1; then
  chown -R foundry:foundry /anvil || true
fi

MNEMONIC="${ETH_MNEMONIC:-test test test test test test test test test test test junk}"
BASE_ARGS="--host 0.0.0.0 --port 8545 --chain-id 31337 -m \"$MNEMONIC\""

if [ -d "$STATE_DIR" ] && [ "$(ls -A "$STATE_DIR" 2>/dev/null)" ]; then
  echo "[entrypoint] Existing state directory detected: $STATE_DIR" >&2
else
  echo "[entrypoint] Starting fresh state directory at $STATE_DIR" >&2
fi

if id foundry >/dev/null 2>&1; then
  exec su foundry -c "anvil $BASE_ARGS --state $STATE_DIR"
else
  exec sh -c "anvil $BASE_ARGS --state $STATE_DIR"
fi
