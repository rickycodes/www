#!/usr/bin/env sh
set -eu

node_bin_dir="${NODE_BIN_DIR:-./node_modules/.bin}"

if [ ! -x "${node_bin_dir}/html-minifier" ] || [ ! -x "${node_bin_dir}/uglifyjs" ]; then
  echo "error: missing node build tools."
  echo "Run: npm ci"
  return 1 2>/dev/null || exit 1
fi

PATH="${node_bin_dir}:${PATH}"
export PATH
