#!/usr/bin/env bash
set -euo pipefail

server_port="${SERVER_PORT:-8765}"
python3 -m http.server "${server_port}" --directory target/deploy &
server_pid=$!
trap 'kill "${server_pid}" 2>/dev/null || true' EXIT INT TERM

for attempt in {1..30}; do
  if curl --silent --fail "http://127.0.0.1:${server_port}/" >/dev/null; then
    break
  fi
  if [[ "${attempt}" -eq 30 ]]; then
    echo "HTTP server did not become ready" >&2
    exit 1
  fi
  sleep 0.2
done

SERVER_PORT="${server_port}" node scripts/browser-smoke.mjs
