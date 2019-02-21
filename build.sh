#!/bin/bash

# We check wether DOCKER_ID is set
if [ "${DOCKER_ID}" == '' ]; then
    echo "Please set env variable DOCKER_ID to your docker id"
    echo "You can run it like this: (assuming that's your ID)"
    echo "DOCKER_ID=${USER} ${0}"
    exit 1
fi

function info_log() {
    BLUE='\033[1;34m'
    NC='\033[0m'
    printf "${NC}[${BLUE}*${NC}] %s\n" "${1}"
}

function fail_log() {
    RED='\033[0;31m'
    WHITE='\033[1;37m'
    NC='\033[0m'
    printf "${NC}[${RED}*${NC}] ${WHITE}%s${NC}\n" "${1}"
}

# Then we compile the rust binary
info_log "Compiling binary, this might take a while"

cargo build --release ||\
    fail_log "Compilation failed"

# We strip the binary to reduce size
info_log "Stripping binary"

strip ./target/release/drone-activitypub ||\
    fail_log "Stripping failed"

# And finish by creating the container
info_log "Creating container"

docker build -t "${DOCKER_ID}/drone-activitypub:latest" . ||\
    fail_log "Container failed"
