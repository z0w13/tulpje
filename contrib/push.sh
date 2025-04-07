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

echo " [*] image tag:" "${TULPJE_VERSION:-$IMAGE_TAG}"

echo " [-] building images..."
IMAGE_PATHS=(
  $(nix build \
    --no-link \
    --print-out-paths \
    .#docker-handler \
    .#docker-gateway \
    .#docker-http-proxy \
    .#docker-gateway-queue)
)

# Push the images
for imagePath in ${IMAGE_PATHS[@]}; do
  imageName=$(tar -axf "$imagePath" manifest.json -O | jq -r '.[0].RepoTags[0]| split(":") | .[0]')
  echo " [-] pushing ${DOCKER_REPO}/$imageName${IMAGE_SUFFIX} ..."
  skopeo \
    --insecure-policy \
    copy \
    "docker-archive:$imagePath" \
    "docker://${DOCKER_REPO}/${imageName}${IMAGE_SUFFIX}"
done
