#!/bin/bash

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1

HELP="$(cat <<-EOF
ricky.codes build tool

USAGE:
    bash build.sh [OPTIONS]

OPTIONS:
    --help              Prints help information
    --gen               Generate + minify HTML...
    --build             Runs cargo web deploy --target=wasm32-unknown-unknown
    --min               Minify deployed *.js files with uglify

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
        ${PARTIALS}/footer.html | html-minifier \
    --collapse-whitespace \
    --remove-comments \
    --remove-optional-tags \
    --remove-redundant-attributes \
    --remove-script-type-attributes \
    --remove-tag-whitespace \
    --use-short-doctype \
    --minify-css; } > static/index.html
}

check() {
    cargo clean && cargo check
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
        uglifyjs "$f" \
            --compress \
            --mangle \
            --output "$f"
    done
}

if [ -z "$ARG" ]
then
    gen
    build
    min
else
    [ "$ARG" == "--help" ] && _help
    [ "$ARG" == "--gen" ] && gen
    [ "$ARG" == "--min" ] && min
    [ "$ARG" == "--build" ] && build
    [ "$ARG" == "--check" ] && check
    exit 0
fi
