#!/bin/bash

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1
HTML=static/index.html
E_ASSERT_FAILED=99

HELP="$(cat <<-EOF
ricky.codes build tool

USAGE:
    bash build.sh [OPTIONS]

OPTIONS:
    --help              Prints help information
    --gen               Generate + minify HTML...
    --build             Runs cargo web deploy --target=wasm32-unknown-unknown
    --min               Minify deployed *.js files with uglify
    --test              Run tests

Running "bash build.sh" with zero options will --gen --build and --min (in that order)
This is not a sophisticated script, one [OPTION] (singular) at a time or None() only pls k thnx.
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
    --remove-tag-whitespace \
    --use-short-doctype \
    --minify-css; } > $HTML
}

build() {
    # build
    echo 'Building...'
    cargo web deploy --target=wasm32-unknown-unknown
}

min() {
    # minify gen and static js files
    echo 'Minify...'
    jsFiles='target/deploy/*.js'
    for f in $jsFiles; do
        npx uglify-es "$f" \
            --compress \
            --mangle \
            --output "$f"
    done
}

tests() {
    # test this script
    HELP=$(bash build.sh --help)
    if [[ ! $HELP == *"ricky.codes build tool"* ]]; then
        echo "build.sh --help failed (unexpected text) $LINENO"
        exit $E_ASSERT_FAILED
    fi
    # test gen
    if [[ -f "$HTML" ]]; then
        rm "$HTML"
    fi
    bash build.sh --gen
    if [[ ! -f "$HTML" ]]; then
        echo "build.sh --gen failed (no HTML file) $LINENO"
        exit $E_ASSERT_FAILED
    fi
    cargo clean && cargo check
}

if [ -z "$ARG" ]
then
    gen
    build
    min
else
    [ "$ARG" == "--test" ] && tests
    [ "$ARG" == "--help" ] && _help
    [ "$ARG" == "--gen" ] && gen
    [ "$ARG" == "--min" ] && min
    [ "$ARG" == "--build" ] && build
    exit 0
fi
