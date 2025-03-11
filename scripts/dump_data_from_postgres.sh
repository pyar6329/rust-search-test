#!/bin/bash

set -e

CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))
PROJECT_ROOT="${CURRENT_DIR}/.."

psql "host=${DB_HOST:-localhost} port=${DB_PORT:-5432} sslmode=disable dbname=${DB_NAME} user=${DB_USER:-postgres} password=${DB_PASSWORD:-postgres}" \
  -c "${PSQL_QUERY};" -t \
  > "${PROJECT_ROOT}/sample.json"
