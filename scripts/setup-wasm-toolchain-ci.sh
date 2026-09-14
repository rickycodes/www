#!/usr/bin/env bash
set -euo pipefail

wasm_bindgen_version="0.2.128"

rustup toolchain install 1.98.1 \
  --profile minimal \
  --target wasm32-unknown-unknown \
  --component clippy \
  --component rustfmt

installed_version=""
if command -v wasm-bindgen >/dev/null 2>&1; then
  installed_version="$(wasm-bindgen --version | awk '{print $2}')"
fi

if [[ "${installed_version}" != "${wasm_bindgen_version}" ]]; then
  cargo +1.98.1 install wasm-bindgen-cli \
    --version "${wasm_bindgen_version}" \
    --locked \
    --force
fi

wasm-bindgen --version

sudo apt-get update
sudo apt-get install -y binaryen wabt
