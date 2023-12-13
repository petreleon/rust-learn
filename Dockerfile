# Use the official Rust image as a parent image
FROM rust:latest as builder

# Set the working directory in the Docker container
WORKDIR /usr/src/rust-learn

# Add target for ARM64
RUN rustup target add aarch64-unknown-linux-musl

# Copy the source code and Cargo manifests into the image
COPY . .

# Build the project in release mode for ARM64
RUN cargo build --target aarch64-unknown-linux-musl --release

# Start the second stage of the build with a minimal Debian image
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/rust-learn/target/aarch64-unknown-linux-musl/release/rust-learn /usr/local/bin/rust-learn

# Set the binary as the entrypoint of the container
ENTRYPOINT ["/usr/local/bin/rust-learn"]
