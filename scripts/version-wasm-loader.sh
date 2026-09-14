#!/usr/bin/env bash
set -euo pipefail

loader_file="${1:-target/deploy/rickycodes.js}"
version="${2:-}"

if [[ ! -f "${loader_file}" ]]; then
  echo "error: ${loader_file} not found."
  exit 1
fi

if [[ -z "${version}" ]]; then
  echo "error: version argument is required."
  exit 1
fi

# Ensure the generated wasm fetch URL is cache-busted with the build revision.
sed -E -i \
  "s#rickycodes_bg\\.wasm(\\?v=[^\"']*)?#rickycodes_bg.wasm?v=${version}#g" \
  "${loader_file}"

echo "Versioned wasm loader in ${loader_file} with v=${version}"
