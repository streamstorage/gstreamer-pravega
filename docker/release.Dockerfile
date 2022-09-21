ARG FROM_IMAGE

FROM "${FROM_IMAGE}" AS build-stage

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.63.0

RUN apt-get update && \
    apt-get install -y wget git

RUN set -eux; \
    rustArch="x86_64-unknown-linux-gnu"; \
    url="https://static.rust-lang.org/rustup/archive/1.25.1/${rustArch}/rustup-init"; \
    wget --quiet "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

WORKDIR /root

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Shanghai
RUN apt-get install -y gcc libgstreamer-plugins-base1.0-dev libgtk2.0-dev libgtk-3-dev
RUN git clone --recursive https://github.com/pravega/gstreamer-pravega; \
    cd gstreamer-pravega && \
    cargo build --package gst-plugin-pravega --locked --release

FROM scratch AS export-stage
COPY --from=build-stage /root/gstreamer-pravega/target/release/*.so /
