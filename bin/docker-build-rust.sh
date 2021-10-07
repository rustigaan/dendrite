#!/bin/bash

BIN="$(cd "$(dirname "$0")" ; pwd)"
PROJECT="$(dirname "${BIN}")"

source "${BIN}/lib-verbose.sh"

"${BIN}/create-local-settings.sh"

source "${PROJECT}/etc/settings-local.sh"

(

  export RUST_VERSION
  RUST_TAG="${DOCKER_REPOSITORY}/rust:${RUST_VERSION}"
  info "RUST_TAG=[${RUST_TAG}]"
  cd "${PROJECT}/docker/rust"
  docker build --build-arg RUST_VERSION -t "${RUST_TAG}" .
)
