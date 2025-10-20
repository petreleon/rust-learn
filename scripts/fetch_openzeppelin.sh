#!/bin/sh
set -e

# Default tag to checkout if not specified
OZ_TAG=${OZ_TAG:-v4.9.3}
DEST_DIR=/usr/src/app/ethereum/contracts/lib/openzeppelin-contracts

echo "[fetch_openzeppelin] Ensuring directory: $(dirname "$DEST_DIR")"
mkdir -p "$(dirname "$DEST_DIR")"

if [ -d "$DEST_DIR/.git" ]; then
  echo "[fetch_openzeppelin] OpenZeppelin repo already present, fetching tags and checking out $OZ_TAG"
  cd "$DEST_DIR"
  git fetch --all --tags || true
  git checkout "$OZ_TAG" || git switch --detach "$OZ_TAG"
else
  echo "[fetch_openzeppelin] Cloning OpenZeppelin $OZ_TAG into $DEST_DIR"
  git clone --depth 1 --branch "$OZ_TAG" https://github.com/OpenZeppelin/openzeppelin-contracts.git "$DEST_DIR"
fi

echo "[fetch_openzeppelin] Done. Current HEAD:" 
cd "$DEST_DIR" && git rev-parse --abbrev-ref HEAD || git rev-parse --verify HEAD

exit 0
