#!/bin/bash

set -uo pipefail

cd "$(dirname "$0")"
bash ./build.sh

cd crates/bitcraft-mini

cargo expand
