#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${1:-.env}"
EXAMPLE_FILE="${2:-.env.example}"

if [[ -f "$ENV_FILE" ]]; then
  echo "$ENV_FILE already exists; leaving it unchanged."
  exit 0
fi

if [[ ! -f "$EXAMPLE_FILE" ]]; then
  echo "Missing $EXAMPLE_FILE" >&2
  exit 1
fi

cp "$EXAMPLE_FILE" "$ENV_FILE"
echo "Created $ENV_FILE from $EXAMPLE_FILE."

if ! command -v openssl >/dev/null 2>&1; then
  echo "OpenSSL is not installed; keep the JWT placeholders until you generate keys manually." >&2
  exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is not installed; keep the JWT placeholders until you add generated keys manually." >&2
  exit 0
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

PRIVATE_KEY_FILE="$TMP_DIR/private.key"
PUBLIC_KEY_FILE="$TMP_DIR/public.key"

openssl genpkey -algorithm RSA -out "$PRIVATE_KEY_FILE" -pkeyopt rsa_keygen_bits:2048 >/dev/null 2>&1
openssl rsa -pubout -in "$PRIVATE_KEY_FILE" -out "$PUBLIC_KEY_FILE" >/dev/null 2>&1

python3 - "$ENV_FILE" "$PRIVATE_KEY_FILE" "$PUBLIC_KEY_FILE" <<'PY'
from pathlib import Path
import sys

env_path = Path(sys.argv[1])
private_key = Path(sys.argv[2]).read_text().strip()
public_key = Path(sys.argv[3]).read_text().strip()

lines = env_path.read_text().splitlines()
updated = []
for line in lines:
    if line.startswith("PRIVATE_KEY="):
        updated.append(f'PRIVATE_KEY="{private_key}"')
    elif line.startswith("PUBLIC_KEY="):
        updated.append(f'PUBLIC_KEY="{public_key}"')
    else:
        updated.append(line)

env_path.write_text("\n".join(updated) + "\n")
PY

echo "Generated a local RSA key pair for JWT signing in $ENV_FILE."
echo "Do not commit $ENV_FILE."
