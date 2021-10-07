#!/bin/bash

BIN="$(cd "$(dirname "$0")" ; pwd)"
CRATE="$(dirname "${BIN}")"
PROJECT="$(dirname "${CRATE}")"

source "${PROJECT}/bin/lib-verbose.sh"
source "${PROJECT}/bin/lib-sed-ext.sh"

USE_CRATES_IO='false'
if [[ ".$1" = '.--use-crates-io' ]]
then
  USE_CRATES_IO='true'
fi

# dendrite = "^0.6" # { path = "../rustic-dendrite" } #
if "${USE_CRATES_IO}"
then
  sed "${SED_EXT}" -i -e 's/^([^=#]*)= *[{] *path *= *("[^"]*") *[}] *# *("[^"]*") *#$/\1= \3 # { path = \2 } #/' "${CRATE}/Cargo.toml"
else
  sed "${SED_EXT}" -i -e 's/^([^=#]*)= *("[^"]*") *# *[{] *path *= *("[^"]*") *[}] *#$/\1= { path = \3 } # \2 #/' "${CRATE}/Cargo.toml"
fi
