FROM grafana/grafana-oss:latest

USER root

ARG GF_GID="0"
ENV GF_PATHS_PLUGINS="/var/lib/grafana-plugins"

RUN mkdir -p "$GF_PATHS_PLUGINS" && \
    chown -R grafana:${GF_GID} "$GF_PATHS_PLUGINS"

USER grafana

COPY ./video-panel-1.0.6.zip /tmp/video-panel-1.0.6.zip

RUN unzip /tmp/video-panel-1.0.6.zip -d ${GF_PATHS_PLUGINS}/
