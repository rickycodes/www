#!/usr/bin/env bash
set -euo pipefail

BUILT_AT_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
GIT_SHA="${1:-}"
if [[ -z "${GIT_SHA}" ]]; then
  echo "error: git SHA argument is required."
  exit 1
fi
RUNNER_OS_VAL="${RUNNER_OS:-$(uname -s)}"
RUNNER_ARCH_VAL="${RUNNER_ARCH:-$(uname -m)}"
CPU_CORES="$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo "unknown")"

cat > static/build-meta.json <<EOF
{"built_at_utc":"$BUILT_AT_UTC","git_sha":"$GIT_SHA","runner_os":"$RUNNER_OS_VAL","runner_arch":"$RUNNER_ARCH_VAL","cpu_cores":"$CPU_CORES"}
EOF
