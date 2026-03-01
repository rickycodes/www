#!/usr/bin/env bash
set -euo pipefail

wasm_file="${1:-target/deploy/rickycodes.wasm}"

if [[ ! -f "${wasm_file}" ]]; then
  echo "error: ${wasm_file} not found."
  exit 1
fi

if ! command -v wasm-strip >/dev/null 2>&1; then
  echo "Skipping wasm strip: wasm-strip is not installed."
  exit 0
fi

wasm-strip "${wasm_file}"
echo "Stripped wasm binary: ${wasm_file}"
