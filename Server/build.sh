#!/bin/bash

set -euo pipefail

cd "$(dirname "$0")"

# Clear protobuf and language files
rm -f protobuf/*.proto
rm -rf cs-src/*

# Copy stdb proto files
cp ../../spacetimedb/crates/spacetimedb/protobuf/WebSocket.proto ./protobuf/

cargo clean
STDB_LANG=cs cargo build --target wasm32-unknown-unknown --release --all
# cargo build --target wasm32-unknown-unknown
# wasm2wat ./target/wasm32-unknown-unknown/release/bitcraft_mini.wasm > ./bitcraft-mini-module-wat

# Export the protobuf
mkdir -p ./cs-src
for file in ./protobuf/* ; do
	protoc --proto_path=./protobuf/ --csharp_out=./cs-src $file
done

# Copy everything that was generated into the the client repo
rm ../Client/Assets/_Project/autogen/*.cs
rsync -a ./cs-src/ ../Client/Assets/_Project/autogen

# Update the running module
MODULE=$(pwd)/target/wasm32-unknown-unknown/release/bitcraft_mini.wasm
cd ../../spacetimedb/
cargo run update "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
# cargo run init "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
