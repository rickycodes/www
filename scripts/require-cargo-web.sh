#!/usr/bin/env sh
set -eu

if ! command -v cargo-web >/dev/null 2>&1; then
  echo "error: no such subcommand: 'web'"
  echo
  echo "cargo-web is required for this project."
  echo "This repo expects a legacy nightly toolchain:"
  echo "  rustup toolchain install nightly-2019-08-01-x86_64-unknown-linux-gnu"
  echo "  cargo +nightly-2019-08-01-x86_64-unknown-linux-gnu install cargo-web --version 0.6.26 --locked"
  echo
  echo "Then ensure the wasm target exists:"
  echo "  rustup target add wasm32-unknown-unknown --toolchain nightly-2019-08-01-x86_64-unknown-linux-gnu"
  exit 1
fi
