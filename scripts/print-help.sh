#!/usr/bin/env bash
set -euo pipefail

site_name="${1:-ricky.codes}"

cat <<EOF
${site_name} build tool

USAGE:
    ./build.sh [OPTIONS]

OPTIONS:
    --help              Prints help information
    --gen, --generate   Generate + minify HTML...
    --build, --build-wasm
                        Runs cargo web deploy --target=wasm32-unknown-unknown
                        (deploys site to ./target/deploy)
    --watch             Starts a local static server + rebuilds wasm on changes
                        (avoids cargo web start runtime panic)
    --min, --minify     Minify deployed *.js files with uglify
    --lint              Run shellcheck against project shell scripts
    --check-links       Check external links referenced in source/static assets
    --test              Run tests

Running "bash build.sh" (with zero options) will --generate --build-wasm and --minify (in that order)
You can pass multiple options; they run in the order provided.
EOF
