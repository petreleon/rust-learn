FROM rust:bookworm AS app_builder

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config libssl-dev build-essential git ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"

WORKDIR /usr/src/app

COPY . .
# Build with a single job to reduce memory pressure during linking. Fail if worker cannot be built.
ENV CARGO_BUILD_JOBS=1
RUN cargo build -j1 --release --bin worker
RUN cargo build -j1 --release --bin rust-learn

FROM debian:12-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash ca-certificates coreutils ffmpeg git libpq5 libssl3 libstdc++6 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=app_builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=app_builder /usr/src/app/target/release/worker /usr/local/bin/worker
COPY --from=app_builder /usr/src/app/target/release/rust-learn /usr/local/bin/rust-learn

WORKDIR /usr/src/app
COPY diesel.toml ./diesel.toml
COPY migrations ./migrations
COPY scripts ./scripts
COPY ethereum/artifacts ./ethereum/artifacts

RUN chmod +x /usr/local/bin/worker /usr/local/bin/rust-learn /usr/local/bin/diesel \
 && chmod +x ./scripts/*.sh \
 && ldconfig \
 && diesel --version
