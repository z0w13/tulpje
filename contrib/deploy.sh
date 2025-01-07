#!/usr/bin/env bash
set -euo pipefail
test -n "${DEBUG:-}" && set -x

# Set the script and project directory
SCRIPT_DIR="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Make sure we're in the project directory
cd "$PROJECT_DIR"

# Check that cross is installed for cross-compiling with musl
if ! hash cross 2>/dev/null; then
  echo " [!] \`cross\` binary for cross-compilation not found, please run \`cargo install cross\`"
  exit 1
fi

# Set unique image suffix (current UNIX timestamp)
IMAGE_SUFFIX=":$(date +%s)"
export IMAGE_SUFFIX

echo " [-] Writing secrets from .env to file..."
rm -f _secrets/*
grep -vE '^(#|$)' .env | while IFS= read -r L; do
  # Parse the variable name and value
  varName="$(echo "$L" | cut -d'=' -f1)"

  # Don't overwrite existing environment variables
  if [[ -z "${!varName:-}" ]]; then
    varVal="$(eval echo "$(echo "$L" | cut -d'=' -f2-)")"
    export "${varName}"="$varVal"
  fi

  # Store each var in .env in a separate secrets file
  echo "     - ${varName}"
  echo "${!varName}" > "_secrets/$(echo "$varName" | tr '[:upper:]' '[:lower:]')"
done

# Request shard count from discord if we haven't specified it in env
if [[ -z "${SHARD_COUNT:-}" ]]; then
  echo " [-] Shard count not found in environment, fetching from Discord..."

  SHARD_COUNT="$(cargo run -p tulpje-manager)"
  export SHARD_COUNT
fi

# Default value for handler count
export HANDLER_COUNT="${HANDLER_COUNT:-1}"

echo " [*] Shard count: $SHARD_COUNT"
echo " [*] Handler count: $HANDLER_COUNT"

# Build binaries
echo " [-] Building binaries..."
cross build --target=x86_64-unknown-linux-musl --release

# Build images
echo " [-] Building images..."
docker compose --profile=full build

# Deploy images
echo " [-] Deploying..."
docker stack deploy --detach=false -c compose.swarm.yml tulpje-staging
