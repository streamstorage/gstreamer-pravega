#!/bin/bash

#
# Copyright (c) Dell Inc., or its subsidiaries. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#

set -ex

ROOT_DIR=$(readlink -f $(dirname $0)/..)
RUST_JOBS=${RUST_JOBS:-4}
DOCKER_REPOSITORY=${DOCKER_REPOSITORY}
FROM_IMAGE=${FROM_IMAGE:-streamstorage/gstreamer:22.04-1.22.5}

# Build pravega-prod image which includes the binaries for all applications.
if [[ "${BUILD_PROD}" == "1" ]]; then
    if [[ "${PULL_BASE}" == "1" ]]; then
        # Make sure to always have fresh base image.
        docker pull ${DOCKER_REPOSITORY}${FROM_IMAGE}-dev
        docker pull ${DOCKER_REPOSITORY}${FROM_IMAGE}-prod
    fi

    docker build \
        -t streamstorage/pravega-gstreamer:latest-prod \
        --build-arg RUST_JOBS=${RUST_JOBS} \
        --build-arg DOCKER_REPOSITORY=${DOCKER_REPOSITORY} \
        --build-arg FROM_IMAGE=${FROM_IMAGE} \
        --target prod \
        -f ${ROOT_DIR}/docker/Dockerfile \
        ${ROOT_DIR}
fi

# Build pravega-dev image which includes the source code and binaries for all applications.
# This is a cache hit 100%.
if [[ "${BUILD_DEV}" == "1" ]]; then
    if [[ "${PULL_BASE}" == "1" ]]; then
        # Make sure to always have fresh base image.
        docker pull ${DOCKER_REPOSITORY}${FROM_IMAGE}-dev
    fi

    docker build \
        -t streamstorage/pravega-gstreamer:latest-dev \
        --build-arg RUST_JOBS=${RUST_JOBS} \
        --build-arg DOCKER_REPOSITORY=${DOCKER_REPOSITORY} \
        --build-arg FROM_IMAGE=${FROM_IMAGE} \
        --target pravega-dev \
        -f ${ROOT_DIR}/docker/Dockerfile \
        ${ROOT_DIR}
fi
