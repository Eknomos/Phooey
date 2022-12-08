#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'master'
DB_USER=${POSTGRES_USER:=master}
# Check if a custom password has been set, otherwise default to 'password123'
DB_PASSWORD="${POSTGRES_PASSWORD:=password123}"
# Check if a custom database name has been set, otherwise default to 'phooey'
DB_NAME="${POSTGRES_DB:=phooey}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000

>&2 echo "Postgres is up and running on port ${DB_PORT}!"