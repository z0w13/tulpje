#!/usr/bin/env bash
set -euo pipefail

##############
#
# Run through a bunch of checks, tests, etc to see if we can actually deploy
#
##############

export RUSTFLAGS="-Dwarnings"

HOST_TARGET="$(rustc --version --verbose | pcregrep -o1 'host: (.*)')"

TARGETS=("x86_64-unknown-linux-gnu" "x86_64-unknown-linux-musl")

echo "* auditing dependencies..."
cargo audit

echo "* running clippy..."
cargo clippy --quiet

for target in ${TARGETS[@]}; do
  # clean up the target/release folder otherwise some weird issues happen with GLIBC and serde
  rm -rf target/release

  if [[ "$target" = "$HOST_TARGET" ]]; then
    rustBin=cargo
  else
    rustBin=cross
  fi

  echo "* building binaries ($target)..."
  $rustBin build --target=$target --release --quiet

  echo "* running tests ($target)..."
  $rustBin test --target=$target --release --quiet
done

echo "* building containers..."
docker compose --profile=full build
