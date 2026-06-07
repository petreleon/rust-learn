FROM rust:bookworm

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates ffmpeg git libpq-dev libssl-dev pkg-config \
 && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt

ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_TARGET_DIR=/usr/local/cargo/target
ENV PATH="/usr/local/cargo/bin:${PATH}"
ENV CARGO_BUILD_JOBS=1

WORKDIR /usr/src/app
