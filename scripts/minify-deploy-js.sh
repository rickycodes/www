#!/usr/bin/env bash
set -euo pipefail

deploy_dir="${1:-target/deploy}"

if ! command -v uglifyjs >/dev/null 2>&1; then
  echo "error: uglifyjs is not installed or not in PATH."
  exit 1
fi

echo "Minifying JavaScript in ${deploy_dir}..."
shopt -s nullglob
js_files=("${deploy_dir}"/*.js)
if [[ ${#js_files[@]} -eq 0 ]]; then
  echo "No JavaScript files found in ${deploy_dir}."
  exit 0
fi

# Remove noisy cargo-web wasm boot log before minifying.
sed -i "s/\"Finished loading \Rust wasm module 'rickycodes'\"//g" "${deploy_dir}/rickycodes.js"

for f in "${js_files[@]}"; do
  echo "    ${f}"
  uglifyjs "$f" \
    --compress \
    --mangle \
    --output "$f"
done

echo "Minified ${#js_files[@]} JavaScript file(s)."
