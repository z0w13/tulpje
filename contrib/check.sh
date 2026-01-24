#!/usr/bin/env bash
##############
#
# Run through a bunch of checks, tests, etc to see if we can actually deploy
#
##############

set -euo pipefail
test -n "${DEBUG:-}" && set -x

# Set the script and project directory
SCRIPT_DIR="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Make sure we're in the project directory
cd "$PROJECT_DIR"

export RUSTFLAGS="-Dwarnings"

echo "* auditing dependencies..."
cargo audit

echo "* running clippy..."
cargo clippy --quiet

echo "* building binaries..."
cargo build --release --quiet

echo "* running tests..."
cargo test --release --quiet

echo "* building docker images..."
nix build --no-link --print-out-paths \
  ".#docker-handler" \
  ".#docker-gateway" \
  ".#docker-http-proxy" \
  ".#docker-gateway-queue"
