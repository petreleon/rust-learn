# =======================
# Stage 1: Build Z3 (4.12.1) and solc from source on Debian 12
# =======================
FROM debian:12 AS solc_builder
ARG DEBIAN_FRONTEND=noninteractive
ARG Z3_VERSION=4.12.1
ARG SOLC_VERSION=0.8.26

# Build dependencies (Boost headers, toolchain, Python for Z3)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git curl ca-certificates python3 \
    libboost-all-dev \
 && rm -rf /var/lib/apt/lists/*

# Build & install Z3 >= 4.12.1
RUN git clone --depth 1 --branch z3-${Z3_VERSION} https://github.com/Z3Prover/z3.git /tmp/z3 \
 && cd /tmp/z3 \
 && python3 scripts/mk_make.py --prefix=/usr/local \
 && cd build && make -j"$(nproc)" && make install && ldconfig

# Fetch Solidity release tarball
RUN curl -L -o /tmp/solidity.tar.gz \
      https://github.com/ethereum/solidity/releases/download/v${SOLC_VERSION}/solidity_${SOLC_VERSION}.tar.gz \
 && mkdir -p /src && tar -xzf /tmp/solidity.tar.gz -C /src --strip-components=1

WORKDIR /src
# Build ONLY the solc binary; avoid cmake --install to skip yul-phaser
RUN cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DPEDANTIC=OFF \
 && cmake --build build --target solc --parallel "$(nproc)" \
 && install -m 0755 build/solc/solc /usr/local/bin/solc

# =======================
# Stage 2: Rust dev image with solc
# =======================
FROM rust:latest

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config libssl-dev build-essential git ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy solc + Z3 runtime libs from builder
COPY --from=solc_builder /usr/local/bin/solc /usr/local/bin/solc
COPY --from=solc_builder /usr/local/lib/libz3.so* /usr/local/lib/

# Ensure the dynamic linker can find /usr/local/lib
RUN ldconfig || true

# Sanity check
RUN solc --version

# Your Rust tooling
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app
