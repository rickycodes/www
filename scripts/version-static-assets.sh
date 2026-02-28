#!/usr/bin/env bash
set -euo pipefail

html_file="${1:-static/index.html}"
version="${2:-}"

if [[ ! -f "${html_file}" ]]; then
  echo "error: ${html_file} not found."
  exit 1
fi

if [[ -z "${version}" ]]; then
  echo "error: version argument is required."
  exit 1
fi

# Version local CSS/JS href/src references in generated HTML only.
# This matches paths without ":" to avoid touching absolute URLs like "https://...".
sed -E -i \
  -e "s#((href|src)=['\"])([^\"'?:]+\\.css)(\\?[^\"']*)?(['\"])#\\1\\3?v=${version}\\5#g" \
  -e "s#((href|src)=['\"])([^\"'?:]+\\.js)(\\?[^\"']*)?(['\"])#\\1\\3?v=${version}\\5#g" \
  "${html_file}"

echo "Versioned local CSS/JS assets in ${html_file} with v=${version}"
