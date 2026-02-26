#!/bin/sh

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1
OUTPUT=static/index.html

_HELP="help"
BUILD="build"
BUILD_WASM="build-wasm"
GEN="gen"
GENERATE="generate"
WATCH="watch"
MIN="min"
MINIFY="minify"
TEST="test"
LINT="lint"
CHECK_LINKS="check-links"
SITE_NAME="ricky.codes"
NODE_BIN_DIR="${NODE_BIN_DIR:-./node_modules/.bin}"

_help() {
    bash scripts/print-help.sh "$SITE_NAME"
}

lint() {
    bash scripts/lint-shell.sh
}

check_links() {
    echo "Checking external links..."
    bash scripts/check-external-links.sh
}

gen() {
    . scripts/require-node-tools.sh
    bash scripts/generate-html.sh "$PARTIALS" "$OUTPUT" "$PROJECTS"
    # append ASCII art inside an HTML comment
    {
        printf "\n"
        printf "<!--\n"
        cat cat.txt
        printf "\n-->\n"
    } >> "$OUTPUT"
    # copy ascii text to static so we can fetch
    cp cat.txt static/
    bash scripts/write-build-meta.sh
    bash scripts/write-search-metadata.sh "$SITE_NAME"
}

build() {
    sh scripts/require-cargo-web.sh
    # build
    echo 'Building...'
    cargo web deploy --target=wasm32-unknown-unknown
}

watch() {
    bash scripts/watch.sh
}

min() {
    . scripts/require-node-tools.sh
    bash scripts/minify-deploy-js.sh "target/deploy"
}

tests() {
    bash scripts/run-tests.sh "$SITE_NAME" "$OUTPUT"
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
    [ "$ARG" = "--$GEN" ] || [ "$ARG" = "$GEN" ] || [ "$ARG" = "--$GENERATE" ] || [ "$ARG" = "$GENERATE" ] && gen
    [ "$ARG" = "--$MIN" ] || [ "$ARG" = "$MIN" ] || [ "$ARG" = "--$MINIFY" ] || [ "$ARG" = "$MINIFY" ] && min
    [ "$ARG" = "--$LINT" ] || [ "$ARG" = "$LINT" ] && lint
    [ "$ARG" = "--$CHECK_LINKS" ] || [ "$ARG" = "$CHECK_LINKS" ] && check_links
    [ "$ARG" = "--$BUILD" ] || [ "$ARG" = "$BUILD" ] || [ "$ARG" = "--$BUILD_WASM" ] || [ "$ARG" = "$BUILD_WASM" ] && build
    [ "$ARG" = "--$WATCH" ] || [ "$ARG" = "$WATCH" ] && watch
    exit 0
fi
