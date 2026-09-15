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
                        Builds with Cargo and packages with wasm-bindgen
                        (deploys site to ./target/deploy)
    --watch             Starts a local static server + rebuilds wasm on changes
    --min, --minify     Minify deployed *.js files with terser
    --lint              Run shellcheck against project shell scripts
    --check-links       Check external links referenced in source/static assets
    --test              Run tests

TOOLCHAIN:
    rust-toolchain.toml installs the pinned stable Rust toolchain and wasm target.
    Install the matching wasm-bindgen CLI with:
      cargo install wasm-bindgen-cli --version 0.2.128 --locked

Running "bash build.sh" (with zero options) will --generate --build-wasm and --minify (in that order)
You can pass multiple options; they run in the order provided.
EOF
