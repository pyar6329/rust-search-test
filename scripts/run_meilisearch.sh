#!/bin/bash

set -e

ARGS="$@"
CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))
PROJECT_ROOT="${CURRENT_DIR}/.."

MEILISEARCH_VERSION="1.13"

MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME="meilisearch-test"
MEILISEARCH_CONTAINER_PS_NAME="meilisearch-test"

if ! type docker > /dev/null 2>&1; then
  echo "This command required Docker. Please install Docker."
  exit 1
fi

if [ ! -e "${PROJECT_ROOT}/data" ]; then
  mkdir -p "${PROJECT_ROOT}/data"
fi

case "${ARGS}" in
  "--up" )
    if ! docker network ls | grep "${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}}" > /dev/null 2>&1; then
      docker network create "${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}}"
      echo "${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}} network was created. Please check 'docker network ls'"
    fi

    # add Linuxx option
    HOST_OPTION=""
    if [ "$(uname -s)" = "Linux" ]; then
      echo "host is Linux, so add-host option is set"
      HOST_OPTION="--add-host=host.docker.internal:host-gateway"
    fi

    echo "run meilisearch"
    echo "meilisearch config is here: https://www.meilisearch.com/docs/learn/self_hosted/configure_meilisearch_at_launch"

    docker run -it \
      -d \
      --restart always \
      --name "${MEILISEARCH_CONTAINER_PS_NAME}" \
      -p "${MEILISEARCH_PORT:-7700}:7700" \
      -e MEILI_ENV="development" \
      -e MEILI_MASTER_KEY="${MEILISEARCH_TOKEN:-masterKey}" \
      -e MEILI_NO_ANALYTICS="true" \
      -e TZ="UTC" \
      -e LANG="C.UTF-8" \
      -e LC_ALL="C.UTF-8" \
      -v ${PROJECT_ROOT}/data:/meili_data \
      "getmeili/meilisearch:v${MEILISEARCH_VERSION}"
    ;;
  "--down" )
    docker rm -f "${MEILISEARCH_CONTAINER_PS_NAME}"

    if docker network ls | grep "${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}}" > /dev/null 2>&1; then
      docker network rm -f "${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}}"
    else
      echo "meilisearch network (${DOCKER_NETWORK_NAME:-${MEILISEARCH_CONTAINER_DEFAULT_NETWORK_NAME}}) was already deleted"
    fi
    ;;
  "--log")
    docker logs -f "${MEILISEARCH_CONTAINER_PS_NAME}"
    ;;
  "--shell")
    docker exec -it "${MEILISEARCH_CONTAINER_PS_NAME}" /bin/ash
    ;;
esac
