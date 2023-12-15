# Build argument for architecture selection with a default value for ARM64
ARG TARGET_ARCH=aarch64-unknown-linux-musl

# Base build stage
FROM rust:latest as builder

# Redeclare the argument to make it available in this stage
ARG TARGET_ARCH

# Install musl-tools and aarch64-linux-musl-gcc
RUN apt-get update && apt-get install -y musl-tools && \
    rustup target add ${TARGET_ARCH}

WORKDIR /usr/src/rust-learn

# Use the build argument to set the target architecture
RUN rustup target add ${TARGET_ARCH}

# Copy the source code and Cargo manifests into the image
COPY . .

# Build the project in release mode for the specified target architecture
# Specify the output directory
RUN cargo build --target ${TARGET_ARCH} --release --target-dir /usr/src/target-${TARGET_ARCH}

# Debugging: List the contents of the target directory
RUN ls -R /usr/src/target-${TARGET_ARCH}/

# Final stage
FROM debian:buster-slim

# Reuse ARG TARGET_ARCH in the final stage
ARG TARGET_ARCH

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/target-${TARGET_ARCH}/${TARGET_ARCH}/release/rust-learn /usr/local/bin/rust-learn

# Expose the port the server listens on
EXPOSE 8080

# Set the binary as the entrypoint of the container
ENTRYPOINT ["/usr/local/bin/rust-learn"]
