#!/bin/bash

set -uoe pipefail

cd "$(dirname "$0")"
cd crates/bitcraft-mini
# cargo clean
# cargo build --package bitcraft-mini
cargo expand > ../bitcraft-mini-expand/src/lib.rs
