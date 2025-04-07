build:
    nix build .

check:
  contrib/check.sh

gateway: (cargo-build "tulpje-gateway") (run-local "target/debug/tulpje-gateway")
handler: (cargo-build "tulpje-handler") (run-local "target/debug/tulpje-handler")

cargo-build package:
  cargo build -p {{ package }}

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
