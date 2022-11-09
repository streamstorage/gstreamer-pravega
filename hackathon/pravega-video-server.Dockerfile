FROM ubuntu:22.04

RUN mkdir -p /opt/pravega-video-server
RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /opt/pravega-video-server

COPY ./target /opt/pravega-video-server/

CMD ["./pravega-video-server"]
