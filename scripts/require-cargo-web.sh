#!/usr/bin/env sh
set -eu

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
