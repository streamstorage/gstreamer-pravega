#!/usr/bin/env bash

pushd ${ROOT_DIR}/hackathon
docker build -f grafana.Dockerfile -t registry.appdev.fun/grafana .
docker push registry.appdev.fun/grafana
popd
