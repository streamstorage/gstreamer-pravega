#!/usr/bin/env bash

set -ex
ROOT_DIR=$(readlink -f $(dirname $0)/..)
pushd ${ROOT_DIR}/pravega-video-server
cargo build --release
popd


pushd ${ROOT_DIR}/hackathon
rm -rf ./target
mkdir -p target
cp -r ../pravega-video-server/resources ./target/
cp ../target/release/pravega-video-server ./target/
docker build -f pravega-video-server.Dockerfile -t registry.appdev.fun/pravega-video-server .
docker push registry.appdev.fun/pravega-video-server
rm -rf ./target
popd
