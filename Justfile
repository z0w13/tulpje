build:
    nix build .

check:
  contrib/check.sh

gateway: (run-local "nix run .#tulpje-gateway")
handler: (run-local "nix run .#tulpje-handler")

run-local +command:
  contrib/run-local.sh {{ command }}

up: build-docker
  docker compose --profile=full up

services-up: (build-docker ".#docker-http-proxy" ".#docker-gateway-queue")
  docker compose up -d

services-down:
  docker compose down

build-docker *packages:
    contrib/build-docker.sh {{ packages }}
