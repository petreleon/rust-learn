# Start from the latest Rust image as the base
FROM rust:latest

# Install necessary packages for building Rust projects and compiling Solidity
# - libpq-dev: PostgreSQL client libs/headers for Diesel
# - solc: Solidity compiler needed by ethers-solc when not using SVM downloads
# - git, ca-certificates: useful for fetching dependencies like OpenZeppelin sources
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        libpq-dev \
        solc \
        git \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Diesel CLI with only the "postgres" feature
# This step is necessary if you're using Diesel for ORM in a Rust project with PostgreSQL
RUN cargo install diesel_cli --no-default-features --features postgres

# Set /usr/src/app as the working directory
# This directory will be the root directory of your Rust project within the container
WORKDIR /usr/src/app

