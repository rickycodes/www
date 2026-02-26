#!/usr/bin/env bash
set -euo pipefail

partials_dir="${1:-src/partials}"
output_path="${2:-static/index.html}"
projects_dir="${3:-${partials_dir}/projects}"

if ! command -v html-minifier >/dev/null 2>&1; then
  echo "error: html-minifier is not installed or not in PATH."
  exit 1
fi

echo "Generate + minify HTML..."
{
  cat "${partials_dir}/doctype.html" \
    "${partials_dir}/header.html" \
    "${partials_dir}/about.html" \
    "${partials_dir}/cv.html" \
    "${partials_dir}/copyright.html" \
    "${projects_dir}"/* \
    "${partials_dir}/footer.html" | html-minifier \
    --collapse-whitespace \
    --remove-comments \
    --remove-optional-tags \
    --remove-redundant-attributes \
    --remove-script-type-attributes \
    --use-short-doctype \
    --minify-css
} > "${output_path}"

echo "HTML deployed to ${output_path}"
