# Use an official Rust runtime as a parent image
FROM rust:latest

# Install PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Compile the current project
RUN cargo build --release

# Make the binary executable
RUN chmod +x ./target/release/rust-learn

