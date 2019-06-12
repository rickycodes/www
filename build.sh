#!/bin/bash

PARTIALS='src/partials'
PROJECTS=$PARTIALS/projects/

echo 'Generate & minify HTML...'
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

# build
echo 'Building...'
cargo web deploy --target=wasm32-unknown-unknown --release

# minify gen and static js files
jsFiles='target/deploy/*.js'
for f in $jsFiles; do
    npx uglify-es $f \
        --compress \
        --mangle \
        --output $f
done
