#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! command -v curl >/dev/null 2>&1; then
  echo "error: curl is required."
  exit 1
fi

mapfile -t URLS < <(
  grep -RhoE "https?://[^\"'()<>[:space:]]+" src/partials static/*.css static/*.js 2>/dev/null |
    sort -u
)

if [[ ${#URLS[@]} -eq 0 ]]; then
  echo "No external links found."
  exit 0
fi

FAILED=()

check_url() {
  local url="$1"
  local code

  code="$(curl -L --silent --show-error --output /dev/null \
    --write-out "%{http_code}" \
    --connect-timeout 10 --max-time 20 \
    --retry 2 --retry-delay 1 --retry-connrefused \
    --head "$url" || true)"

  if [[ "$code" =~ ^[23][0-9][0-9]$ ]]; then
    return 0
  fi

  code="$(curl -L --silent --show-error --output /dev/null \
    --write-out "%{http_code}" \
    --connect-timeout 10 --max-time 20 \
    --retry 2 --retry-delay 1 --retry-connrefused \
    "$url" || true)"

  [[ "$code" =~ ^[23][0-9][0-9]$ ]]
}

for url in "${URLS[@]}"; do
  printf "Checking %s ... " "$url"
  if check_url "$url"; then
    echo "ok"
  else
    echo "failed"
    FAILED+=("$url")
  fi
done

if [[ ${#FAILED[@]} -gt 0 ]]; then
  echo
  echo "External link check failed for:"
  printf '%s\n' "${FAILED[@]}"
  exit 1
fi

echo
echo "All external links are reachable."
