#!/bin/bash

set -euo pipefail

cd "$(dirname "$0")"

# Validate assumptions
if ! [ -d ../../SpacetimeDB ] ; then
	echo "SpacetimeDB could not be found. It should exist in the same directory as BitCraftMini."
	exit 1
fi

# Clear protobuf and language files
rm -f protobuf/*.proto
rm -rf cs-src/*

# Copy stdb proto files
mkdir -p protobuf
cp ../../SpacetimeDB/crates/spacetimedb/protobuf/WebSocket.proto ./protobuf/

# Install the wasm32 target if it doesn't exist
rustup target add wasm32-unknown-unknown

# Build bitcraft mini
cargo clean
STDB_LANG=cs cargo build --target wasm32-unknown-unknown --release --package bitcraft-mini

# Export the protobuf
mkdir -p ./cs-src
for file in ./protobuf/* ; do
	protoc --proto_path=./protobuf/ --csharp_out=./cs-src $file
done

# Copy everything that was generated into the the client repo
DIR_AUTOGEN=$(readlink -f ../Client/Assets/_Project/autogen)
rm -rf "$DIR_AUTOGEN"
mkdir -p "$DIR_AUTOGEN"
rsync -a ./cs-src/ "$DIR_AUTOGEN" 

# Update the running module
MODULE=$(pwd)/target/wasm32-unknown-unknown/release/bitcraft_mini.wasm
cd ../../SpacetimeDB/

if [[ $# > 0 ]]; then
	if [ "$1" == "init" ] ; then
		cargo run init -f "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
	elif [ "$1" == "update" ]; then
		cargo run update "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470" "bitcraftmini" "$MODULE"
	else 
		echo "Command not understood: $1" 1>&2
	fi
else 
	echo "Info: No deploy command, run with \"init\" to initialize"
fi
