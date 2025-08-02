# Use official Debian as a parent image
FROM debian:stable-slim

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        gnupg \
        lsb-release \
        unzip \
        python3 \
        python3-pip \
        && rm -rf /var/lib/apt/lists/*

# Download and install Go Ethereum (geth) from source
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        build-essential \
        golang \
        && rm -rf /var/lib/apt/lists/*

RUN git clone --branch release/1.16 --single-branch https://github.com/ethereum/go-ethereum.git /opt/go-ethereum && \
    cd /opt/go-ethereum && \
    make geth && \
    cp build/bin/geth /usr/local/bin/geth && \
    rm -rf /opt/go-ethereum

# Copy requirements.txt and install Python dependencies
COPY requirements.txt /tmp/requirements.txt
RUN pip3 install --no-cache-dir -r /tmp/requirements.txt

# Create a working directory
WORKDIR /root/.ethereum

# Expose default geth ports
EXPOSE 8545 30303

# Default command (can be overridden)
CMD ["geth"]
