#!/bin/bash

BIN="$(cd "$(dirname "$0")" ; pwd)"

source "${BIN}/lib-verbose.sh"

## /usr/local/rustup
RUST_VERSION='1.52'
SUB_DIR='/src'

RUSTUP_VOLUME="rustup-${RUST_VERSION}"

if docker volume inspect "${RUSTUP_VOLUME}" >/dev/null 2>&1
then
  :
else
  docker run --rm -v "${RUSTUP_VOLUME}:/opt/rustup" "rust:${RUST_VERSION}" cp -r /usr/local/rustup /opt/rustup
fi

docker run --rm -ti -v "${RUSTUP_VOLUME}:/usr/local/rustup" -v "${HOME}${SUB_DIR}:${HOME}${SUB_DIR}" -w "$(pwd)" -e "USER=${USER}" "rust:${RUST_VERSION}" "$@"
