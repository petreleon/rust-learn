#!/usr/bin/env bash
set -euo pipefail

# Keep behavior consistent with current Compose command when a Git checkout is
# mounted. Built production images do not include .git metadata.
if [[ -d .git ]] && command -v git >/dev/null 2>&1; then
  # Avoid noisy "dubious ownership" warnings when the workspace is bind-mounted
  git config --global --add safe.directory '*' || true
  git config --global --add safe.directory /usr/src/app || true
  git submodule update --init --recursive || true
fi

# Default to sleeping unless explicitly told to run in production mode
if [[ "${PROD_MODE:-}" == "TRUE" ]]; then
  echo "[entrypoint] PROD_MODE=TRUE: applying Diesel migrations and starting the app"

  # Best-effort wait for Postgres to be ready and run migrations (retry loop)
  # Diesel CLI is available in the image; DB settings come from .env via Docker Compose.
  # Use an empty runtime config so production startup never rewrites schema.rs.
  runtime_diesel_config="$(mktemp)"
  trap 'rm -f "$runtime_diesel_config"' EXIT

  migrations_applied=0
  for i in {1..30}; do
    if diesel migration run --config-file "$runtime_diesel_config"; then
      echo "[entrypoint] Migrations applied"
      migrations_applied=1
      break
    fi
    if [[ "$i" == "30" ]]; then
      break
    fi
    echo "[entrypoint] Diesel migration attempt $i failed; retrying in 2s..."
    sleep 2
  done
  if [[ "$migrations_applied" != "1" ]]; then
    echo "[entrypoint] Migrations failed after 30 attempts; refusing to start the app" >&2
    exit 1
  fi

  # Start the Rust app (using the pre-built binary)
  exec /usr/local/bin/rust-learn
else
  echo "[entrypoint] PROD_MODE is not TRUE (got: '${PROD_MODE:-unset}'). Not launching app; sleeping indefinitely."
  exec sleep infinity
fi
