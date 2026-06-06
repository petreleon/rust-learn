#!/usr/bin/env bash
set -euo pipefail

if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi

for bin_dir in /opt/homebrew/bin /usr/local/bin; do
  if [[ -d "$bin_dir" && ":$PATH:" != *":$bin_dir:"* ]]; then
    export PATH="$bin_dir:$PATH"
  fi
done

# The committed local .env defaults are container-friendly. When tests run on
# the host against Compose-published ports, map Docker DNS names to localhost.
if [[ "${DATABASE_URL:-}" == *@db:5432/* ]]; then
  export DATABASE_URL="${DATABASE_URL/@db:5432/@localhost:5433}"
fi

if [[ "${S3_INTERNAL_DOMAIN:-}" == "rustfs" ]]; then
  export S3_INTERNAL_DOMAIN="localhost"
fi

if [[ -n "${HOST_ETH_RPC_URL:-}" ]]; then
  export ETH_RPC_URL="$HOST_ETH_RPC_URL"
elif [[ -z "${ETH_RPC_URL:-}" || "${ETH_RPC_URL:-}" == "http://anvil:8545" || "${ETH_RPC_URL:-}" == "http://geth:8545" ]]; then
  export ETH_RPC_URL="http://localhost:8545"
fi

case "${ETH_HOST:-}" in
  anvil|geth)
    export ETH_HOST="localhost"
    ;;
esac

if [[ -d /opt/homebrew/opt/libpq/lib ]]; then
  export PQ_LIB_DIR="${PQ_LIB_DIR:-/opt/homebrew/opt/libpq/lib}"
  export LIBRARY_PATH="${LIBRARY_PATH:+$LIBRARY_PATH:}/opt/homebrew/opt/libpq/lib"
  export DYLD_LIBRARY_PATH="${DYLD_LIBRARY_PATH:+$DYLD_LIBRARY_PATH:}/opt/homebrew/opt/libpq/lib"
fi

if [[ $# -eq 0 ]]; then
  exec cargo test
fi

exec "$@"
