#!/usr/bin/env bash
set -euo pipefail
test -n "${DEBUG:-}" && set -x

# Set the script and project directory
SCRIPT_DIR="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Make sure we're in the project directory
cd "$PROJECT_DIR"

if test -z "${DOCKER_REPO:-}"; then
    echo " [!] DOCKER_REPO env var is empty please specify remote repository"
    exit 1
fi

if [[ -z "${IMAGE_TAG:-}" ]]; then
  GIT_TAG="$(git describe --abbrev=0 --match 'v*')"
  TULPJE_VERSION="${GIT_TAG#v}"
  export IMAGE_SUFFIX=":${TULPJE_VERSION}"
else
  export IMAGE_SUFFIX=":${IMAGE_TAG}"
fi

echo " [*] image tag:" "${TULPJE_VERSION}"

# Build binaries
echo " [-] building binaries..."
cargo build --target=x86_64-unknown-linux-musl --release

# Build images
echo " [-] building images..."
docker compose --profile=full build

echo " [-] tagging images..."
docker tag "discord-proxy$IMAGE_SUFFIX"  "$DOCKER_REPO/tulpje/discord-proxy$IMAGE_SUFFIX"
docker tag "tulpje-handler$IMAGE_SUFFIX" "$DOCKER_REPO/tulpje/handler$IMAGE_SUFFIX"
docker tag "tulpje-gateway$IMAGE_SUFFIX" "$DOCKER_REPO/tulpje/gateway$IMAGE_SUFFIX"
docker tag "gateway-queue$IMAGE_SUFFIX"  "$DOCKER_REPO/tulpje/gateway-queue$IMAGE_SUFFIX"

echo " [-] pushing images..."
docker push "$DOCKER_REPO/tulpje/discord-proxy$IMAGE_SUFFIX"
docker push "$DOCKER_REPO/tulpje/handler$IMAGE_SUFFIX"
docker push "$DOCKER_REPO/tulpje/gateway$IMAGE_SUFFIX"
docker push "$DOCKER_REPO/tulpje/gateway-queue$IMAGE_SUFFIX"
