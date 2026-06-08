# syntax=docker/dockerfile:1.7

FROM rust:bookworm AS app_builder

ARG DEBIAN_FRONTEND=noninteractive
ARG DIESEL_CLI_VERSION=2.3.10
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config libssl-dev build-essential git ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,id=rust-learn-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rust-learn-cargo-git,target=/usr/local/cargo/git \
    cargo install diesel_cli --version "${DIESEL_CLI_VERSION}" --locked --no-default-features --features postgres

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"

WORKDIR /usr/src/app

COPY . .
# Build with a single job to reduce memory pressure during linking.
ENV CARGO_BUILD_JOBS=1
RUN --mount=type=cache,id=rust-learn-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rust-learn-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rust-learn-app-target,target=/usr/src/app/target \
    cargo build --locked -j1 --release --bin rust-learn --features app-bin \
 && cp /usr/src/app/target/release/rust-learn /usr/local/bin/rust-learn-build

FROM debian:12-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash ca-certificates coreutils libpq5 libssl3 libstdc++6 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=app_builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=app_builder /usr/local/bin/rust-learn-build /usr/local/bin/rust-learn

WORKDIR /usr/src/app
COPY diesel.toml ./diesel.toml
COPY migrations ./migrations
COPY scripts ./scripts
COPY ethereum/artifacts ./ethereum/artifacts

RUN chmod +x /usr/local/bin/rust-learn /usr/local/bin/diesel \
 && chmod +x ./scripts/*.sh \
 && ldconfig \
 && diesel --version
