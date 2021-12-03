#!/usr/bin/false

PROJECT_ETC="$(cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd)"
PROJECT="$(dirname "${PROJECT_ETC}")"

source "${PROJECT}/dendrite-lib/etc/settings-local.sh"
