#!/bin/bash

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/
ARG=$1

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
    --minify-css; } > static/index.html
}

build() {
    # build
    echo 'Building...'
    cargo web deploy --target=wasm32-unknown-unknown --release
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

if [ -z "$ARG" ]
then
    gen
    build
    min
else
    [ "$ARG" == "gen" ] && gen
    [ "$ARG" == "min" ] && min
    [ "$ARG" == "build" ] && build
fi
