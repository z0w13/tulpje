#!/usr/bin/env bash
# Utility to inject service IP addresses from docker, rather than using the hostnames
# which doesn't work when running on the host

set -euo pipefail
test -n "${DEBUG:-}" && set -x

# Set the script and project directory
SCRIPT_DIR="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"


get_container_ip() {
  docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$1"
}

source "$PROJECT_DIR/.env"

export RABBITMQ_ADDRESS=amqp://$(get_container_ip "tulpje-rabbitmq-1"):5672
export DISCORD_PROXY=$(get_container_ip "tulpje-discord_proxy-1"):80
export DISCORD_GATEWAY_QUEUE=http://$(get_container_ip "tulpje-gateway_queue-1"):80
export REDIS_URL=redis://$(get_container_ip "tulpje-valkey-1"):6379
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@$(get_container_ip "tulpje-postgres-1")/${POSTGRES_DB}"

export SHARD_ID=0
export HANDLER_ID=0

exec "$@"
