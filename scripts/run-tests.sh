#!/usr/bin/env bash
set -euo pipefail

site_name="${1:-ricky.codes}"
output="${2:-static/index.html}"

fail() {
  echo "$1"
  exit 99
}

echo "warming up ${site_name} test suite!"

help_output="$(./build.sh --help)"
check_help="$(echo "${help_output}" | grep "${site_name} build tool" || true)"
if [[ -z "${check_help}" ]]; then
  fail "build.sh --help test failed (unexpected text)"
fi

rm -f "${output}"
./build.sh --lint --generate
if [[ ! -f "${output}" ]]; then
  fail "build.sh --gen test failed (no HTML file)"
fi

simple="$(grep -ir 'simple\|simply' src/ || true)"
if [[ -n "${simple}" ]]; then
  fail "'Nothing in life is simple' test failed ${simple}"
fi

echo "Running Rust format, lint, and test checks"
cargo fmt --all -- --check
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
cargo test --locked

echo "all tests passed!"
