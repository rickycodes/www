#!/usr/bin/env bash
set -euo pipefail

BUILT_AT_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
GIT_SHA="${1:-}"
if [[ -z "${GIT_SHA}" ]]; then
  echo "error: git SHA argument is required."
  exit 1
fi
cat > static/build-meta.json <<EOF
{"built_at_utc":"$BUILT_AT_UTC","git_sha":"$GIT_SHA"}
EOF
