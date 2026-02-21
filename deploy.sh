#!/bin/bash

# sync to http
function http() {
    rsync -avze "ssh -p $PORT" ./target/deploy/ "${USER}@${HOST}:${DEPLOY_PATH}"
}

# sync to dat
function dat() {
    rsync -r --size-only target/deploy/* ignore/dat/
    dat sync --dir=ignore/dat/
}

function prompt() {
    read -p "${1} " -n 1 -r
    echo    # (optional) move to a new line
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        $2
    fi
}

# build
echo "Building..."
./build.sh
# TODO: make blog components optional and re-enable
# hugo --source ~/projects/blog/



# combine website and blog
# mv ~/projects/blog/public/ ./target/deploy/blog/

prompt "Deploy to http?" http
# this seems busted
# prompt "Sync to dat?" dat
