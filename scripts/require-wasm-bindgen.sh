#!/usr/bin/env sh
set -eu

required_version="0.2.128"

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "error: wasm-bindgen-cli is required."
  echo "Install it with:"
  echo "  cargo install wasm-bindgen-cli --version ${required_version} --locked"
  exit 1
fi

installed_version="$(wasm-bindgen --version | awk '{print $2}')"
if [ "${installed_version}" != "${required_version}" ]; then
  echo "error: wasm-bindgen-cli ${required_version} is required (found ${installed_version})."
  echo "Install the matching version with:"
  echo "  cargo install wasm-bindgen-cli --version ${required_version} --locked --force"
  exit 1
fi
