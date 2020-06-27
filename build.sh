#!/bin/sh

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1
HTML=static/index.html
E_ASSERT_FAILED=99

HELP="$(cat <<-EOF
ricky.codes build tool

USAGE:
    sh build.sh [OPTIONS]

OPTIONS:
    --help              Prints help information
    --gen               Generate + minify HTML...
    --build             Runs cargo web deploy --target=wasm32-unknown-unknown
    --serve             Runs cargo web start --target=wasm32-unknown-unknown
    --min               Minify deployed *.js files with uglify
    --test              Run tests

Running "sh build.sh" (with zero options) will --gen --build and --min (in that order)
This is not a sophisticated script, one [OPTION] (singular) at a time or none.
EOF
)"

_help() {
    echo "$HELP"
}

gen() {
    echo 'Generate + minify HTML...'
    { cat ${PARTIALS}/sig.html & cat ${PARTIALS}/header.html \
        ${PARTIALS}/main.html \
        ${PROJECTS}* \
        ${PARTIALS}/footer.html | npx html-minifier \
    --collapse-whitespace \
    --remove-comments \
    --remove-optional-tags \
    --remove-redundant-attributes \
    --remove-script-type-attributes \
    --use-short-doctype \
    --minify-css; } > $HTML
}

build() {
    # build
    echo 'Building...'
    cargo web deploy --target=wasm32-unknown-unknown
}

serve() {
    # build
    echo 'Starting up local server...'
    cargo web start --target=wasm32-unknown-unknown
}

min() {
    # minify gen and static js files
    echo 'Minify...'
    jsFiles='target/deploy/*.js'
    for f in $jsFiles; do
        uglifyjs "$f" \
            --compress \
            --mangle \
            --output "$f"
    done
}

fail() {
    echo "$1 $2"
    exit $E_ASSERT_FAILED
}

tests() {
    # test help
    HELP=$(sh build.sh --help)
    CHECK_HELP=$(echo "$HELP" | grep "ricky.codes build tool")
    if [ -z "$CHECK_HELP" ]; then
        fail "build.sh --help test failed (unexpected text)"
    fi
    # test gen
    if [ -f "$HTML" ]; then
        rm "$HTML"
    fi
    sh build.sh --gen
    if [ ! -f "$HTML" ]; then
        fail "build.sh --gen test failed (no HTML file)"
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
else
    [ "$ARG" = "--test" ] && tests
    [ "$ARG" = "--help" ] && _help
    [ "$ARG" = "--gen" ] && gen
    [ "$ARG" = "--min" ] && min
    [ "$ARG" = "--build" ] && build
    [ "$ARG" = "--serve" ] && serve
    exit 0
fi
