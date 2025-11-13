#!/usr/bin/env bash
set -euo pipefail

# Keep behavior consistent with current Compose command (fetch submodules on start)
if command -v git >/dev/null 2>&1; then
  # Avoid noisy "dubious ownership" warnings when the workspace is bind-mounted
  git config --global --add safe.directory '*' || true
  git config --global --add safe.directory /usr/src/app || true
  git submodule update --init --recursive || true
fi

# Default to sleeping unless explicitly told to run in production mode
if [[ "${PROD_MODE:-}" == "TRUE" ]]; then
  echo "[entrypoint] PROD_MODE=TRUE: applying Diesel migrations and starting the app"

  # Best-effort wait for Postgres to be ready and run migrations (retry loop)
  # Diesel CLI is available in the image; DB settings come from .env via docker-compose
  for i in {1..30}; do
    if diesel migration run; then
      echo "[entrypoint] Migrations applied"
      break
    fi
    echo "[entrypoint] Diesel migration attempt $i failed; retrying in 2s..."
    sleep 2
  done

  # Start the Rust app (use release mode in production)
  exec cargo run --release
else
  echo "[entrypoint] PROD_MODE is not TRUE (got: '${PROD_MODE:-unset}'). Not launching app; sleeping indefinitely."
  exec sleep infinity
fi
