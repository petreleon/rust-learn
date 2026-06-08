# syntax=docker/dockerfile:1.7

# Lightweight Dockerfile to build only the `worker` binary
# - Build stage: rust toolchain (Debian-based)
# - Final stage: minimal Debian runtime containing only the built binary

FROM rust:bookworm AS builder

ARG DEBIAN_FRONTEND=noninteractive

# Install native dependencies required to build and link the worker
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config libssl-dev build-essential git ca-certificates \
 && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo

WORKDIR /usr/src/worker

# Copy manifest first to leverage Docker layer caching for dependencies.
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,id=rust-learn-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rust-learn-cargo-git,target=/usr/local/cargo/git \
    cargo fetch --locked

# Copy the whole workspace - building a workspace crate may require workspace sources
COPY . .

# Limit parallelism to reduce memory usage during linking
ENV CARGO_BUILD_JOBS=1

# Build the worker binary in release mode
RUN --mount=type=cache,id=rust-learn-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rust-learn-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rust-learn-worker-target,target=/usr/src/worker/target \
    cargo build --locked --release --bin worker --features worker-bin \
 && cp /usr/src/worker/target/release/worker /usr/local/bin/worker-build

# --- Final runtime image ---
# Keep only the worker binary and runtime libraries in the final image.
FROM debian:12-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive

# Install only runtime packages required by the worker binary
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    libstdc++6 \
    ffmpeg \
 && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/bin/worker-build /usr/local/bin/worker
COPY scripts/worker-healthcheck.sh /usr/local/bin/worker-healthcheck
RUN chmod +x /usr/local/bin/worker /usr/local/bin/worker-healthcheck

ENTRYPOINT ["/usr/local/bin/worker"]
