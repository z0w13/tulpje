build:
    nix build .

check:
  contrib/check.sh

gateway: (run-local "nix run .#tulpje-gateway")
handler: (run-local "nix run .#tulpje-handler")

release *args:
  uv --project tools/release-tulpje run release-tulpje {{ args }}

run-local +command:
  contrib/run-local.sh {{ command }}

sqlx-migrate: database-up
  contrib/run-local.sh sqlx migrate run --source crates/tulpje-handler/migrations

sqlx-prepare: database-up
  contrib/run-local.sh cargo sqlx prepare --workspace

up: build-docker
  docker compose --profile=full up

database-up:
  docker compose up -d postgres

services-up: (build-docker ".#docker-nirn-proxy" ".#docker-gateway-queue")
  docker compose up -d

services-down:
  docker compose down

build-docker *packages:
    contrib/build-docker.sh {{ packages }}
