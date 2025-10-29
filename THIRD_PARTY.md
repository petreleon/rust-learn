# Third-party libraries & software

This project uses several third-party Rust crates and external tools/services. This document lists the main dependencies, their versions (where specified in `Cargo.toml` or Docker images), and links to upstream projects so you can review licenses, docs, and installation instructions.

## Rust crates (from Cargo.toml)
- actix-web = 4.11 — https://crates.io/crates/actix-web
- actix-service = 2.0.3 — https://crates.io/crates/actix-service
- futures = 0.3 — https://crates.io/crates/futures
- infer = 0.19.0 — https://crates.io/crates/infer
- diesel = 2.2.12 (features: postgres, r2d2, chrono, numeric) — https://crates.io/crates/diesel
- dotenvy = 0.15 — https://crates.io/crates/dotenvy
- diesel_migrations = 2.2.0 — https://crates.io/crates/diesel_migrations
- chrono = 0.4 (serde) — https://crates.io/crates/chrono
- serde = 1.0 (derive) — https://crates.io/crates/serde
- serde_urlencoded = 0.7 — https://crates.io/crates/serde_urlencoded
- serde_json = 1 — https://crates.io/crates/serde_json
- bcrypt = 0.17 — https://crates.io/crates/bcrypt
- jsonwebtoken = 9 — https://crates.io/crates/jsonwebtoken
- log = 0.4 — https://crates.io/crates/log
- strum = 0.27, strum_macros = 0.27 — https://crates.io/crates/strum
- time = 0.3.41 — https://crates.io/crates/time
- ethers = 2.0.14 — https://crates.io/crates/ethers
- ethers-solc = 2 (feature: svm) — https://crates.io/crates/ethers-solc
- minio = 0.3.0 — https://crates.io/crates/minio
- http = 1 — https://crates.io/crates/http
- hex = 0.4 — https://crates.io/crates/hex
- tokio = 1 (rt, macros, io-util, sync) — https://crates.io/crates/tokio
- anyhow = 1.0 — https://crates.io/crates/anyhow
- bigdecimal = 0.4 (serde) — https://crates.io/crates/bigdecimal

## Dev / test crates
- bip39 = 1.1 — https://crates.io/crates/bip39
- getrandom = 0.2 — https://crates.io/crates/getrandom

## External tools & services

- Docker & Docker Compose — used to run the app, Postgres, MinIO, and anvil (Foundry). https://www.docker.com/
- Colima — lightweight container runtime for macOS (optional for some setups). https://github.com/abiosoft/colima
- Homebrew — recommended package manager in README for macOS users. https://brew.sh/
- OpenSSL — used for generating RSA keys in README instructions. https://www.openssl.org/

## Docker images / runtime services used in docker-compose.yml
- postgres:latest — PostgreSQL database used for the app. https://hub.docker.com/_/postgres
- ghcr.io/foundry-rs/foundry:nightly — Anvil (local Ethereum node) for blockchain integration testing. https://github.com/foundry-rs/foundry
- MinIO (built from `docker/minio.Dockerfile`) — S3-compatible object storage for file uploads. https://min.io/

## CLI tools referenced in Dockerfile / project
- diesel_cli — installed into the app image with `--features postgres` and used to run migrations. https://crates.io/crates/diesel_cli

## Notes and license links
- Most Rust crates are MIT/Apache-2.0 or similar; check each crate’s page for exact license information.