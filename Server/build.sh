#!/bin/bash

set -euo pipefail

cd "$(dirname "$0")"

# Clear protobuf and language files
rm -f protobuf/*.proto
rm -rf cs-src/*

# Copy stdb proto files
cp ../../spacetimedb/crates/spacetimedb/protobuf/WebSocket.proto ./protobuf/

STDB_LANG=cs cargo build --target wasm32-unknown-unknown --release
# cargo build --target wasm32-unknown-unknown
wasm2wat ./target/wasm32-unknown-unknown/release/bitcraft_mini.wasm > ./bitcraft-mini-module-wat

# Export the protobuf
mkdir -p ./cs-src
for file in ./protobuf/* ; do
	protoc --proto_path=./protobuf/ --csharp_out=./cs-src $file
done

# Copy everything that was generated into the the client repo
rm ../Client/Assets/_Project/autogen/*.cs
rsync -a ./cs-src/ ../Client/Assets/_Project/autogen
