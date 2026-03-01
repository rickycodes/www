#!/usr/bin/env bash
set -euo pipefail

rustup toolchain install nightly-2019-08-01 --profile minimal
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-08-01
rustup toolchain install 1.63.0 --profile minimal

if command -v cargo-web >/dev/null 2>&1; then
  cargo-web --version
else
  cargo +1.63.0 install cargo-web --version 0.6.26 --locked
fi

sudo apt-get update
sudo apt-get install -y wabt
