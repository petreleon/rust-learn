#!/bin/sh
set -e

STATE_DIR=/anvil/state
STATE_FILE="$STATE_DIR/state.json"

echo "[entrypoint] Preparing /anvil directory" >&2
mkdir -p /anvil "$STATE_DIR"
chmod 775 /anvil "$STATE_DIR" || true
if id foundry >/dev/null 2>&1; then
  chown -R foundry:foundry /anvil || true
fi

MNEMONIC="${ETH_MNEMONIC:-test test test test test test test test test test test junk}"
BASE_ARGS="--host 0.0.0.0 --port 8545 --chain-id 31337 -m \"$MNEMONIC\""

if [ -f "$STATE_FILE" ]; then
  first_non_ws="$(tr -d '[:space:]' < "$STATE_FILE" | head -c 1 || true)"
  case "$first_non_ws" in
    "{"|"[")
      echo "[entrypoint] Existing state file detected: $STATE_FILE" >&2
      ;;
    *)
      invalid_state_file="$STATE_FILE.invalid.$(date +%s)"
      echo "[entrypoint] Invalid Anvil state detected; moving $STATE_FILE to $invalid_state_file" >&2
      mv "$STATE_FILE" "$invalid_state_file"
      ;;
  esac
elif [ -d "$STATE_DIR" ] && [ "$(ls -A "$STATE_DIR" 2>/dev/null)" ]; then
  echo "[entrypoint] Existing state directory detected without state file: $STATE_DIR" >&2
else
  echo "[entrypoint] Starting fresh state directory at $STATE_DIR" >&2
fi

if id foundry >/dev/null 2>&1; then
  exec su foundry -c "anvil $BASE_ARGS --state $STATE_DIR"
else
  exec sh -c "anvil $BASE_ARGS --state $STATE_DIR"
fi
