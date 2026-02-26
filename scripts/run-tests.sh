#!/usr/bin/env bash
set -euo pipefail

site_name="${1:-ricky.codes}"
output="${2:-static/index.html}"

fail() {
  echo "$1"
  exit 99
}

echo "warming up ${site_name} test suite!"

./build.sh --lint

help_output="$(./build.sh --help)"
check_help="$(echo "${help_output}" | grep "${site_name} build tool" || true)"
if [[ -z "${check_help}" ]]; then
  fail "build.sh --help test failed (unexpected text)"
fi

if [[ -f "${output}" ]]; then
  rm "${output}"
fi

./build.sh --gen
if [[ ! -f "${output}" ]]; then
  fail "build.sh --gen test failed (no HTML file)"
fi

simple="$(grep -ir 'simple\|simply' src/ || true)"
if [[ -n "${simple}" ]]; then
  fail "'Nothing in life is simple' test failed ${simple}"
fi

echo "Running cargo clean and cargo check"
cargo clean && cargo check

echo "all tests passed!"
