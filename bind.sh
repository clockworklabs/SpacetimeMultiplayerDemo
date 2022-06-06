#!/bin/bash

cd "$(dirname "$0")"

sudo mount --bind ../spacetimedb ./Server/spacetimedb
