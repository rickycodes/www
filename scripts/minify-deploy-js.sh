#!/usr/bin/env bash
set -euo pipefail

deploy_dir="${1:-target/deploy}"

if ! command -v uglifyjs >/dev/null 2>&1; then
  echo "error: uglifyjs is not installed or not in PATH."
  exit 1
fi

# Remove noisy cargo-web wasm boot log before minifying.
sed -i "s/\"Finished loading \Rust wasm module 'rickycodes'\"//g" "${deploy_dir}/rickycodes.js"

echo "Minify..."
for f in "${deploy_dir}"/*.js; do
  uglifyjs "$f" \
    --compress \
    --mangle \
    --output "$f"
done

echo "JavaScript deployed to ${deploy_dir}/*.js"
