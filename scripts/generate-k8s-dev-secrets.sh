#!/usr/bin/env bash
set -euo pipefail

OUT="${1:-k8s/overlays/dev/secrets.patch.yaml}"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

if [[ -f "$OUT" && "${K8S_DEV_SECRETS_FORCE:-0}" != "1" ]]; then
  echo "$OUT already exists; leaving local development secrets unchanged."
  echo "Set K8S_DEV_SECRETS_FORCE=1 to regenerate it."
  exit 0
fi

PRIVATE_KEY_FILE="$TMP_DIR/private.key"
PUBLIC_KEY_FILE="$TMP_DIR/public.key"

if ! command -v openssl >/dev/null 2>&1; then
  echo "openssl is required to generate local Kubernetes JWT keys" >&2
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required to write the Kubernetes secret patch" >&2
  exit 1
fi

openssl genpkey -algorithm RSA -out "$PRIVATE_KEY_FILE" -pkeyopt rsa_keygen_bits:2048 >/dev/null 2>&1
openssl rsa -pubout -in "$PRIVATE_KEY_FILE" -out "$PUBLIC_KEY_FILE" >/dev/null 2>&1
mkdir -p "$(dirname "$OUT")"

python3 - "$OUT" "$PRIVATE_KEY_FILE" "$PUBLIC_KEY_FILE" <<'PY'
from pathlib import Path
import json
import os
import sys

out = Path(sys.argv[1])
private_key = Path(sys.argv[2]).read_text().strip()
public_key = Path(sys.argv[3]).read_text().strip()

postgres_user = os.environ.get("K8S_POSTGRES_USER", "rustlearn")
postgres_password = os.environ.get("K8S_POSTGRES_PASSWORD", "rustlearn")
postgres_db = os.environ.get("K8S_POSTGRES_DB", "rustlearn")
database_url = os.environ.get(
    "K8S_DATABASE_URL",
    f"postgres://{postgres_user}:{postgres_password}@postgres:5432/{postgres_db}",
)
eth_mnemonic = os.environ.get(
    "K8S_ETH_MNEMONIC",
    "test test test test test test test test test test test junk",
)
s3_access_key = os.environ.get("K8S_S3_ACCESS_KEY", "rustfsadmin")
s3_secret_key = os.environ.get("K8S_S3_SECRET_KEY", "rustfsadmin")
admin_password = os.environ.get("K8S_ADMIN_PASSWORD", "rustlearn-admin")

def q(value: str) -> str:
    return json.dumps(value)

def block(value: str, spaces: int = 4) -> str:
    indent = " " * spaces
    return "\n".join(f"{indent}{line}" for line in value.splitlines())

out.write_text(
    f"""apiVersion: v1
kind: Secret
metadata:
  name: postgres-secret
  namespace: rust-learn
type: Opaque
stringData:
  POSTGRES_USER: {q(postgres_user)}
  POSTGRES_PASSWORD: {q(postgres_password)}
  POSTGRES_DB: {q(postgres_db)}
  DATABASE_URL: {q(database_url)}
---
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
  namespace: rust-learn
type: Opaque
stringData:
  PRIVATE_KEY: |
{block(private_key)}
  PUBLIC_KEY: |
{block(public_key)}
  ETH_MNEMONIC: {q(eth_mnemonic)}
  ADMIN_NAME: "admin"
  ADMIN_EMAIL: "admin@example.com"
  ADMIN_PASSWORD: {q(admin_password)}
  ADMIN_DATE_OF_BIRTH: "1990-01-01"
---
apiVersion: v1
kind: Secret
metadata:
  name: s3-secret
  namespace: rust-learn
type: Opaque
stringData:
  S3_ACCESS_KEY: {q(s3_access_key)}
  S3_SECRET_KEY: {q(s3_secret_key)}
""",
    encoding="utf-8",
)
PY

echo "Wrote $OUT"
echo "Do not commit this file."
