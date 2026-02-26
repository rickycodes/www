#!/usr/bin/env bash
set -euo pipefail

if ! command -v shellcheck >/dev/null 2>&1; then
  echo "error: shellcheck is not installed."
  echo "Install shellcheck and retry."
  exit 1
fi

echo "Running shellcheck..."
shellcheck ./*.sh scripts/*.sh
