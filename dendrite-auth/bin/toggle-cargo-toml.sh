#!/bin/bash

BIN="$(cd "$(dirname "$0")" ; pwd)"
PROJECT="$(dirname "${BIN}")"

source "${BIN}/verbose.sh"
source "${BIN}/lib-sed-ext.sh"

USE_CRATES_IO='false'
if [[ ".$1" = '.--use-crates-io' ]]
then
  USE_CRATES_IO='true'
fi

# dendrite = "^0.6" # { path = "../rustic-dendrite" } #
if "${USE_CRATES_IO}"
then
  sed "${SED_EXT}" -i -e 's/^([^=#]*)= *[{] *path *= *("[^"]*") *[}] *# *("[^"]*") *#$/\1= \3 # { path = \2 } #/' "${PROJECT}/Cargo.toml"
else
  sed "${SED_EXT}" -i -e 's/^([^=#]*)= *("[^"]*") *# *[{] *path *= *("[^"]*") *[}] *#$/\1= { path = \3 } # \2 #/' "${PROJECT}/Cargo.toml"
fi
