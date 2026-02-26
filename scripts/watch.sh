#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-web >/dev/null 2>&1; then
  echo "error: no such subcommand: 'web'"
  echo
  echo "cargo-web is required for this project."
  echo "Install it with an older stable toolchain (cargo-web is not compatible with latest stable):"
  echo "  rustup toolchain install 1.63.0-x86_64-unknown-linux-gnu"
  echo "  cargo +1.63.0-x86_64-unknown-linux-gnu install cargo-web --version 0.6.26 --locked"
  echo
  echo "Then ensure the wasm target exists:"
  echo "  rustup target add wasm32-unknown-unknown --toolchain nightly-2019-08-01-x86_64-unknown-linux-gnu"
  exit 1
fi

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
