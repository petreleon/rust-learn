#!/bin/sh
set -e

# Derive private key and address from ETH_MNEMONIC
if [ -z "$ETH_MNEMONIC" ]; then
  echo "ETH_MNEMONIC not set. Exiting."
  exit 1
fi


# Use python3 script to derive address and private key
ACCOUNT_INFO=$(python3 /ethereum/derive_eth_account.py)
ADDRESS=$(echo "$ACCOUNT_INFO" | sed -n '1p')
PRIVATE_KEY=$(echo "$ACCOUNT_INFO" | sed -n '2p')

# Import account into geth keystore without writing to a file or outputting to terminal
printf "%s" "$PRIVATE_KEY" | geth account import --password /dev/null /dev/stdin > /dev/null

# Start geth with the derived address as etherbase
exec geth --dev --http --http.addr 0.0.0.0 --http.port 8545 --http.api eth,net,web3,personal,miner --ws --ws.addr 0.0.0.0 --ws.port 8546 --ws.api eth,net,web3,personal,miner --allow-insecure-unlock --miner.etherbase=$ADDRESS --http.corsdomain='*' --http.vhosts='*'
