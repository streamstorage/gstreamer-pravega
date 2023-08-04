#
# Copyright (c) Dell Inc., or its subsidiaries. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#

ARG DOCKER_REPOSITORY=""
ARG FROM_IMAGE

# Build image that that will contain the GStreamer source code.
FROM "${DOCKER_REPOSITORY}${FROM_IMAGE}-dev" as builder-base
WORKDIR /usr/src/gstreamer-pravega
RUN rustup component add clippy rustfmt

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -o Dpkg::Options::="--force-confnew" install -y --no-install-recommends libgtk-3-dev
RUN apt-get clean && rm -rf /var/lib/apt/lists/*


# Install Cargo Chef build tool.
FROM builder-base as chef-base
ARG RUST_JOBS=1
RUN cargo install cargo-chef --jobs ${RUST_JOBS} --version 0.1.62 --locked


# Create Cargo Chef recipe.
FROM chef-base as planner
COPY Cargo.toml .
COPY Cargo.lock .
COPY apps apps
COPY deepstream/pravega_protocol_adapter deepstream/pravega_protocol_adapter
COPY gst-plugin-pravega gst-plugin-pravega
COPY integration-test integration-test
COPY pravega-video pravega-video
COPY pravega-video-server pravega-video-server
RUN cargo chef prepare --recipe-path recipe.json


# Download and build Rust dependencies for gstreamer-pravega.
FROM chef-base as cacher
COPY --from=planner /usr/src/gstreamer-pravega/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# Build GStreamer Pravega libraries and applications.
FROM builder-base as pravega-dev

ARG RUST_JOBS=1

## Copy over the cached dependencies.
COPY --from=cacher /usr/src/gstreamer-pravega/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

COPY Cargo.toml .
COPY Cargo.lock .
COPY apps apps
COPY deepstream/pravega_protocol_adapter deepstream/pravega_protocol_adapter
COPY gst-plugin-pravega gst-plugin-pravega
COPY integration-test integration-test
COPY pravega-video pravega-video
COPY pravega-video-server pravega-video-server

# Build gst-plugin-pravega.
RUN cargo build --package gst-plugin-pravega --locked --release --jobs ${RUST_JOBS}
RUN cp -v target/release/*.so /usr/lib/gstreamer-1.0/
RUN mkdir /gst-pravega && cp -v target/release/*.so /gst-pravega/

## Build pravega-video-server.
RUN cargo install --locked --jobs ${RUST_JOBS} --path pravega-video-server
COPY pravega-video-server/resources /opt/pravega-video-server/resources
ENV PRAVEGA_VIDEO_SERVER_RESOURCE_DIR=/opt/pravega-video-server/resources

## Build misc. Rust apps.
RUN cargo install --locked --jobs ${RUST_JOBS} --path apps --bin \
      rtsp-camera-simulator
RUN cargo install --locked --jobs ${RUST_JOBS} --path integration-test --bin \
      longevity-test

## Install Python apps.
COPY python_apps python_apps
ENV PATH=/usr/src/gstreamer-pravega/python_apps:$PATH


# Build base production image including OS dependencies.
FROM "${DOCKER_REPOSITORY}${FROM_IMAGE}-prod" as prod-base
RUN useradd -ms /bin/bash gstreamer-pravega
USER gstreamer-pravega


# Build production image without debug symbols
FROM prod-base as prod
COPY python_apps /usr/src/gstreamer-pravega/python_apps
ENV PYTHONPATH=/usr/src/gstreamer-pravega/python_apps/lib
ENV PATH=/usr/src/gstreamer-pravega/python_apps:$PATH
COPY --from=pravega-dev /gst-pravega/ /usr/lib/gstreamer-1.0/
COPY --from=pravega-dev /usr/src/gstreamer-pravega/target/release/rtsp-camera-simulator /usr/local/bin/
COPY --from=pravega-dev /usr/src/gstreamer-pravega/target/release/pravega-video-server /usr/local/bin/
COPY pravega-video-server/resources /opt/pravega-video-server/resources
ENV PRAVEGA_VIDEO_SERVER_RESOURCE_DIR=/opt/pravega-video-server/resources
COPY docker/entrypoint.sh /entrypoint.sh
CMD ["/entrypoint.sh"]
