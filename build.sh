#!/bin/sh

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
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

if [ "$#" -eq 0 ]
then
    gen
    build
    min
    echo 'Done.'
else
    while [ "$#" -gt 0 ]; do
        case "$1" in
            --"$TEST"|"$TEST")
                tests
                ;;
            --"$_HELP"|"$_HELP")
                _help
                ;;
            --"$GEN"|"$GEN"|--"$GENERATE"|"$GENERATE")
                gen
                ;;
            --"$MIN"|"$MIN"|--"$MINIFY"|"$MINIFY")
                min
                ;;
            --"$LINT"|"$LINT")
                lint
                ;;
            --"$CHECK_LINKS"|"$CHECK_LINKS")
                check_links
                ;;
            --"$BUILD"|"$BUILD"|--"$BUILD_WASM"|"$BUILD_WASM")
                build
                ;;
            --"$WATCH"|"$WATCH")
                watch
                ;;
            *)
                echo "Unknown option: $1"
                echo "Run ./build.sh --help"
                exit 1
                ;;
        esac
        shift
    done
fi
