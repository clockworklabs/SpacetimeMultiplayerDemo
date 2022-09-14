#!/bin/bash

set -uoe pipefail

cd "$(dirname "$0")"

rm -rf ./crates/bitcraft-mini-expand/src
mkdir -p ./crates/bitcraft-mini-expand/src
touch ./crates/bitcraft-mini-expand/src/lib.rs

set +e
cargo clean
PROC_MACRO_DEBUG=1 cargo build > ./crates/bitcraft-mini-expand/src/lib.rs
