# Lightweight Dockerfile to build only the `worker` binary
# - Build stage: rust toolchain (Debian-based)
# - Final stage: minimal Debian runtime containing only the built binary

FROM rust:latest AS builder

ARG DEBIAN_FRONTEND=noninteractive

# Install native dependencies required to build and link the worker
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config libssl-dev build-essential git ca-certificates ffmpeg \
 && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo

WORKDIR /usr/src/worker

# Copy manifest first to leverage Docker layer caching for dependencies
# Cargo.lock is excluded from context via .dockerignore, copy only Cargo.toml
COPY Cargo.toml ./
RUN cargo fetch || true

# Copy the whole workspace - building a workspace crate may require workspace sources
COPY . .

# Limit parallelism to reduce memory usage during linking
ENV CARGO_BUILD_JOBS=1

# Build the worker binary in release mode
RUN cargo build --release --bin worker

# --- Final runtime image ---
# Use the same base as the builder to ensure compatible glibc/runtime libraries
FROM rust:latest AS runtime

# Install only runtime packages required by the worker binary
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    ffmpeg \
 && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/worker/target/release/worker /usr/local/bin/worker
RUN chmod +x /usr/local/bin/worker

ENTRYPOINT ["/usr/local/bin/worker"]
