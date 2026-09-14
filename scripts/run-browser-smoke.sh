#!/usr/bin/env bash
set -euo pipefail

server_port="${SERVER_PORT:-8765}"
python3 -m http.server "${server_port}" --directory target/deploy &
server_pid=$!
trap 'kill "${server_pid}" 2>/dev/null || true' EXIT INT TERM

SERVER_PORT="${server_port}" node scripts/browser-smoke.mjs
