#!/usr/bin/env bash
set -euo pipefail

##############
#
# Run through a bunch of checks, tests, etc to see if we can actually deploy
#
##############

export RUSTFLAGS="-Dwarnings"

TARGETS=("x86_64-unknown-linux-gnu" "x86_64-unknown-linux-musl")

echo "* auditing dependencies..."
cargo audit

echo "* running clippy..."
cargo clippy --quiet

for target in "${TARGETS[@]}"; do
  echo "* building binaries ($target)..."
  cargo build --target="$target" --release --quiet
done

for target in "${TARGETS[@]}"; do
  echo "* running tests ($target)..."
  cargo test --target="$target" --release --quiet
done

echo "* building containers..."
docker compose --profile=full build
