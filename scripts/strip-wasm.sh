#!/usr/bin/env bash
set -euo pipefail

wasm_file="${1:-target/deploy/rickycodes_bg.wasm}"

if [[ ! -f "${wasm_file}" ]]; then
  echo "error: ${wasm_file} not found."
  exit 1
fi

if ! command -v wasm-strip >/dev/null 2>&1; then
  echo "Skipping wasm strip: wasm-strip is not installed."
else
  wasm-strip "${wasm_file}"
  echo "Stripped wasm binary: ${wasm_file}"
fi

# Rust's release profile already performs size optimization. Avoid a Binaryen
# reserialization here: distro Binaryen versions can rewrite wasm-bindgen's
# externref table in a way that fails during browser initialization.
echo "Skipping wasm-opt: Rust release optimization preserves wasm-bindgen tables."
