#!/bin/sh
set -eu

heartbeat_path="/tmp/worker_alive"
max_age_seconds=180

if [ ! -r "$heartbeat_path" ]; then
  echo "worker heartbeat missing at $heartbeat_path"
  exit 1
fi

last_seen="$(head -n 1 "$heartbeat_path" | tr -d '[:space:]')"
case "$last_seen" in
  ''|*[!0-9]*)
    echo "worker heartbeat is not an epoch timestamp: $last_seen"
    exit 1
    ;;
esac

now="$(date +%s)"
age_seconds=$((now - last_seen))

if [ "$age_seconds" -ge 0 ] && [ "$age_seconds" -lt "$max_age_seconds" ]; then
  exit 0
fi

echo "worker heartbeat is stale: age=${age_seconds}s max=${max_age_seconds}s"
exit 1
