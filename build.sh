#!/bin/sh

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1
OUTPUT=static/index.html
E_ASSERT_FAILED=99

_HELP="help"
BUILD="build"
GEN="gen"
WATCH="watch"
MIN="min"
TEST="test"
SITE_NAME="ricky.codes"

HELP="$(cat <<-EOF
$SITE_NAME build tool

USAGE:
    ./build.sh [OPTIONS]

OPTIONS:
    --$_HELP              Prints help information
    --$GEN               Generate + minify HTML...
    --$BUILD             Runs cargo web deploy --target=wasm32-unknown-unknown
                        (deploys site to ./target/deploy)
    --$WATCH             Starts a local static server + rebuilds wasm on changes
                        (avoids cargo web start runtime panic)
    --$MIN               Minify deployed *.js files with uglify
    --$TEST              Run tests

Running "bash build.sh" (with zero options) will --$GEN --$BUILD and --$MIN (in that order)
This is not a sophisticated script, one [OPTION] (singular) at a time or none.
EOF
)"

_help() {
    printf "%s\n" "$HELP"
}

msg() {
    echo "$1 deployed to $2"
}

require_cargo_web() {
    if ! command -v cargo-web >/dev/null 2>&1; then
        echo "error: no such subcommand: 'web'"
        echo
        echo "cargo-web is required for this project."
        echo "Install it with an older stable toolchain (cargo-web is not compatible with latest stable):"
        echo "  rustup toolchain install 1.63.0-x86_64-unknown-linux-gnu"
        echo "  cargo +1.63.0-x86_64-unknown-linux-gnu install cargo-web --version 0.6.26 --locked"
        echo
        echo "Then ensure the wasm target exists:"
        echo "  rustup target add wasm32-unknown-unknown --toolchain nightly-2019-08-01-x86_64-unknown-linux-gnu"
        exit 1
    fi
}

gen() {
    echo 'Generate + minify HTML...'
    { cat ${PARTIALS}/doctype.html \
        ${PARTIALS}/header.html \
        ${PARTIALS}/about.html \
        ${PARTIALS}/cv.html \
        ${PARTIALS}/copyright.html \
        ${PROJECTS}* \
        ${PARTIALS}/footer.html | npx html-minifier \
    --collapse-whitespace \
    --remove-comments \
    --remove-optional-tags \
    --remove-redundant-attributes \
    --remove-script-type-attributes \
    --use-short-doctype \
    --minify-css; } > $OUTPUT
    msg "HTML" "$OUTPUT"
    # newline
    echo "\n" >> "$OUTPUT"
    # begin html comment
    echo "<!--" >> "$OUTPUT"
    # add ascii cat
    cat cat.txt >> "$OUTPUT"
    # end html comment
    echo "-->" >> "$OUTPUT"
    # copy ascii text to static so we can fetch
    cp cat.txt static/
}

build() {
    require_cargo_web
    # build
    echo 'Building...'
    cargo web deploy --target=wasm32-unknown-unknown
}

watch() {
    require_cargo_web
    echo 'Starting local server with rebuild loop...'
    cargo web deploy --target=wasm32-unknown-unknown || exit 1

    SERVER_PORT=${SERVER_PORT:-8000}

    if command -v python3 >/dev/null 2>&1; then
        python3 -m http.server "$SERVER_PORT" --directory target/deploy &
        SERVER_PID=$!
    else
        echo "python3 not found; cannot start local server."
        echo "Serve ./target/deploy with your preferred static file server."
        exit 1
    fi

    trap 'kill "$SERVER_PID" 2>/dev/null' EXIT INT TERM
    echo "Serving ./target/deploy at http://127.0.0.1:${SERVER_PORT}"

    if command -v watchexec >/dev/null 2>&1; then
        echo "Watching src/, static/, Cargo.toml via watchexec..."
        watchexec -w src -w static -w Cargo.toml -- cargo web deploy --target=wasm32-unknown-unknown
    else
        echo "watchexec is not installed; watching is disabled."
        echo "Install with: cargo +stable install watchexec-cli"
        echo "Server is running; press Ctrl+C to stop."
        wait "$SERVER_PID"
    fi
}

min() {
    # replace console.log before minify
    sed -i "s/\"Finished loading \Rust wasm module 'rickycodes'\"//g" target/deploy/rickycodes.js
    # minify gen and static js files
    echo 'Minify...'
    jsFiles='target/deploy/*.js'
    for f in $jsFiles; do
        npx uglify-js "$f" \
            --compress \
            --mangle \
            --output "$f"
    done
    msg "JavaScript" "$jsFiles"
}

fail() {
    echo "$1 $2"
    exit $E_ASSERT_FAILED
}

tests() {
    echo "warming up $SITE_NAME test suite!"
    # test help
    HELP=$(./build.sh --help)
    CHECK_HELP=$(echo "$HELP" | grep "$SITE_NAME build tool")
    if [ -z "$CHECK_HELP" ]; then
        fail "build.sh --help test failed (unexpected text)"
    fi
    # test gen
    if [ -f "$OUTPUT" ]; then
        rm "$OUTPUT"
    fi
    bash build.sh --gen
    if [ ! -f "$OUTPUT" ]; then
        fail "build.sh --$GEN test failed (no HTML file)"
    fi
    # nothing in life is simple
    SIMPLE=$(grep -ir 'simple\|simply' src/)
    if [ -n "$SIMPLE" ]; then
        fail "'Nothing in life is simple' test failed $SIMPLE"
    fi
    echo "Running cargo clean and cargo check"
    cargo clean && cargo check
    # test the full build
    # build
    # ex=$?
    # if [ $ex -gt 0 ]; then
    #    fail "The build failed :("
    # fi
    echo "all tests passed!"
}

if [ -z "$ARG" ]
then
    gen
    build
    min
    echo 'Done.'
else
    [ "$ARG" = "--$TEST" ] || [ "$ARG" = "$TEST" ] && tests
    [ "$ARG" = "--$_HELP" ] || [ "$ARG" = "$_HELP" ] && _help
    [ "$ARG" = "--$GEN" ] || [ "$ARG" = "$GEN" ] && gen
    [ "$ARG" = "--$MIN" ] || [ "$ARG" = "$MIN" ] && min
    [ "$ARG" = "--$BUILD" ] || [ "$ARG" = "$BUILD" ] && build
    [ "$ARG" = "--$WATCH" ] || [ "$ARG" = "$WATCH" ] && watch
    exit 0
fi
