FROM grafana/grafana-oss:latest

USER root

ARG GF_GID="0"
ENV GF_PATHS_PLUGINS "/var/lib/grafana-plugins"
ENV GF_DEFAULT_APP_MODE "development"

RUN mkdir -p "$GF_PATHS_PLUGINS" && \
    chown -R grafana:${GF_GID} "$GF_PATHS_PLUGINS"

USER grafana

COPY ./dist ${GF_PATHS_PLUGINS}/pravega-video-player
