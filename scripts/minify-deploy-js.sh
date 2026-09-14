#!/usr/bin/env bash
set -euo pipefail

deploy_dir="${1:-target/deploy}"

if ! command -v terser >/dev/null 2>&1; then
  echo "error: terser is not installed or not in PATH."
  exit 1
fi

echo "Minifying JavaScript in ${deploy_dir}..."
shopt -s nullglob
js_files=("${deploy_dir}"/*.js)
if [[ ${#js_files[@]} -eq 0 ]]; then
  echo "No JavaScript files found in ${deploy_dir}."
  exit 0
fi

for f in "${js_files[@]}"; do
  echo "    ${f}"
  terser_args=(--compress --mangle --output "$f")
  case "$(basename "$f")" in
    detect.js|rickycodes.js)
      terser_args+=(--module)
      ;;
  esac
  terser "$f" "${terser_args[@]}"
done

echo "Minified ${#js_files[@]} JavaScript file(s)."
