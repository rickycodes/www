#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

sh scripts/require-cargo-web.sh

echo "Starting local server with rebuild loop..."
cargo web deploy --target=wasm32-unknown-unknown || exit 1

server_port="${SERVER_PORT:-8000}"

if command -v python3 >/dev/null 2>&1; then
  python3 -m http.server "${server_port}" --directory target/deploy &
  server_pid=$!
else
  echo "python3 not found; cannot start local server."
  echo "Serve ./target/deploy with your preferred static file server."
  exit 1
fi

trap 'kill "${server_pid}" 2>/dev/null' EXIT INT TERM
echo "Serving ./target/deploy at http://127.0.0.1:${server_port}"

if command -v watchexec >/dev/null 2>&1; then
  echo "Watching src/, static/, Cargo.toml via watchexec..."
  watchexec -w src -w static -w Cargo.toml -- cargo web deploy --target=wasm32-unknown-unknown
else
  echo "watchexec is not installed; watching is disabled."
  echo "Install with: cargo +stable install watchexec-cli"
  echo "Server is running; press Ctrl+C to stop."
  wait "${server_pid}"
fi
