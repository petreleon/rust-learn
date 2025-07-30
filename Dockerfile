# Start from the latest Rust image as the base
FROM rust:latest

# # Install locales and set the default locale to en_US.UTF-8
# RUN apt-get update && apt-get install -y locales && rm -rf /var/lib/apt/lists/* \
#     && localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8
# ENV LANG en_US.utf8

# Install necessary packages for building Rust projects
# This includes curl, build-essential for compiling dependencies, and libpq-dev for PostgreSQL support
RUN apt-get update && apt-get install -y libpq-dev

# # Optionally upgrade all packages to their latest versions
# RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && rm -rf /var/lib/apt/lists/*

# # Install Rust and Cargo using Rustup by downloading and running the Rustup installer script
# # This makes Rust and Cargo available in the PATH
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# RUN rustup update nightly && rustup default nightly

# Install Diesel CLI with only the "postgres" feature
# This step is necessary if you're using Diesel for ORM in a Rust project with PostgreSQL
RUN cargo install diesel_cli --no-default-features --features postgres

# Set /usr/src/app as the working directory
# This directory will be the root directory of your Rust project within the container
WORKDIR /usr/src/app

# RUN cargo build

# RUN chmod 777 -R ./target

