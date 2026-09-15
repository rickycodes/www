#!/usr/bin/env bash
set -euo pipefail

wasm_cache_dir="${1:-.ci-cache/wasm}"
deploy_dir="${2:-target/deploy}"
git_sha="${3:-$(git rev-parse --short HEAD)}"

if [[ ! -f "${wasm_cache_dir}/rickycodes_bg.wasm" || ! -f "${wasm_cache_dir}/rickycodes.js" ]]; then
  echo "error: missing cached wasm bundle in ${wasm_cache_dir}."
  exit 1
fi

./build.sh --gen

rm -rf "${deploy_dir}"
mkdir -p "${deploy_dir}"

cp -R static/. "${deploy_dir}/"
cp "${wasm_cache_dir}/rickycodes_bg.wasm" "${deploy_dir}/rickycodes_bg.wasm"
cp "${wasm_cache_dir}/rickycodes.js" "${deploy_dir}/rickycodes.js"

bash scripts/version-wasm-loader.sh "${deploy_dir}/rickycodes.js" "${git_sha}"
. scripts/require-node-tools.sh
bash scripts/minify-deploy-js.sh "${deploy_dir}"

echo "ricky.codes" > "${deploy_dir}/CNAME"
