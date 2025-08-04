# Use official Ethereum geth image as base
FROM ethereum/client-go:stable

# Install Python3 and pip3
USER root
RUN apk add --no-cache python3 py3-pip

# Copy requirements.txt and install Python dependencies
COPY requirements.txt /tmp/requirements.txt
# Create Python virtual environment and install requirements
RUN python3 -m venv /venv && \
    /venv/bin/pip install --no-cache-dir -r /tmp/requirements.txt

# Copy Python script and entrypoint
COPY derive_eth_account.py /ethereum/derive_eth_account.py
COPY geth-entrypoint.sh /geth-entrypoint.sh

# Ensure entrypoint script is executable
RUN chmod +x /geth-entrypoint.sh

# Set working directory
WORKDIR /root/.ethereum

# Expose default geth ports
EXPOSE 8545 30303

# Set entrypoint
ENV PATH="/venv/bin:$PATH"
ENTRYPOINT ["sh", "/geth-entrypoint.sh"]
