#!/usr/bin/env bash
set -euo pipefail
test -n "${DEBUG:-}" && set -x

# Set the script and project directory
SCRIPT_DIR="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Make sure we're in the project directory
cd "$PROJECT_DIR"

echo " [*] image tag: ${IMAGE_TAG:-latest}"
export IMAGE_SUFFIX=":${IMAGE_TAG:-latest}"

if [[ $# -gt 0 ]]; then
  PACKAGE_NAMES=( "$@" )
else
  PACKAGE_NAMES=(
    ".#docker-handler"
    ".#docker-gateway"
    ".#docker-http-proxy"
    ".#docker-gateway-queue"
  )
fi

echo " [-] building images..."
IMAGE_PATHS=(
  $(nix build --no-link --print-out-paths "${PACKAGE_NAMES[@]}")
)

# import the images into docker
for imagePath in ${IMAGE_PATHS[@]}; do
  # parse image name from the archive
  imageName=$(tar -axf "$imagePath" manifest.json -O | jq -r '.[0].RepoTags[0]| split(":") | .[0]')
  echo " [-] importing $imageName${IMAGE_SUFFIX} ..."
  skopeo \
    --insecure-policy \
    copy \
    "docker-archive:$imagePath" \
   "docker-daemon:${imageName}${IMAGE_SUFFIX}"
done
