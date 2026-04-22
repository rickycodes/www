#!/usr/bin/env bash
set -euo pipefail

wasm_file="${1:-target/deploy/rickycodes.wasm}"

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

if ! command -v wasm-opt >/dev/null 2>&1; then
  echo "Skipping wasm-opt: wasm-opt is not installed."
  exit 0
fi

# Further shrink and simplify the module (Binaryen).
wasm-opt -Oz "${wasm_file}" -o "${wasm_file}"
echo "Optimized wasm binary (-Oz): ${wasm_file}"
