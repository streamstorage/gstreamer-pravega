#!/usr/bin/env bash

set -ex
ROOT_DIR=$(readlink -f $(dirname $0)/..)
pushd ${ROOT_DIR}/grafana-plugins/pravega-video-player
yarn build
popd

pushd ${ROOT_DIR}/hackathon
cp -r ${ROOT_DIR}/grafana-plugins/pravega-video-player/dist .
docker build -f grafana.Dockerfile -t registry.appdev.fun/grafana .
docker push registry.appdev.fun/grafana
rm -rf ./dist
popd
