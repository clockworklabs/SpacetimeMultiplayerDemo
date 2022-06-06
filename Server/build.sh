#!/bin/bash

set -euo pipefail

# cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-unknown-unknown
wasm2wat ./target/wasm32-unknown-unknown/release/rust_wasm_test.wasm > crates/rust-wasm-test/wat

# Export the protobuf
for file in ./protobuf/* ; do
	protoc --proto_path=./protobuf/ --csharp_out=./cs-src $file
done
