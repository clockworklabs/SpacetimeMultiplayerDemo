#!/bin/bash

set -euo pipefail

[ $# == 0 ] && echo "No deploy command, only building!"

cd "$(dirname "$0")"

# Clear protobuf and language files
rm -f protobuf/*.proto
rm -rf cs-src/*

# Copy stdb proto files
cp ../../SpacetimeDB/crates/spacetimedb/protobuf/WebSocket.proto ./protobuf/

rustup +nightly target add wasm32-unknown-unknown

cargo clean
STDB_LANG=cs cargo +nightly build --target wasm32-unknown-unknown --release --package bitcraft-mini
# cargo build --target wasm32-unknown-unknown
# wasm2wat ./target/wasm32-unknown-unknown/release/bitcraft_mini.wasm > ./bitcraft-mini-module-wat

# Export the protobuf
mkdir -p ./cs-src
for file in ./protobuf/* ; do
	protoc --proto_path=./protobuf/ --csharp_out=./cs-src $file
done

# Copy everything that was generated into the the client repo
rm -f ../Client/Assets/_Project/autogen/*.cs
rsync -a ./cs-src/ ../Client/Assets/_Project/autogen

# Update the running module
MODULE=$(pwd)/target/wasm32-unknown-unknown/release/bitcraft_mini.wasm
cd ../../SpacetimeDB/

if [[ $# > 0 ]]; then
	if [ "$1" == "init" ] ; then
		cargo run init -f "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
	elif [ "$2" == "update" ]; then
		cargo run update "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
	fi
fi
