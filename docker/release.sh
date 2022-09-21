#!/bin/bash

## to build
set -ex

ROOT_DIR=$(readlink -f $(dirname $0)/..)
FROM_IMAGE=${FROM_IMAGE:-ubuntu:20.04}

# in case to build with proxy, add the following params
#   --build-arg http_proxy=http://localhost:1090 --build-arg https_proxy=http://localhost:1090 --network=host \
DOCKER_BUILDKIT=1 docker build --progress=plain \
    --build-arg FROM_IMAGE=${FROM_IMAGE} \
    -f ${ROOT_DIR}/docker/release.Dockerfile \
    -o ${ROOT_DIR}/target/lib \
    ${ROOT_DIR}/docker
