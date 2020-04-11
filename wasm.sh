#!/usr/bin/env bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/lispinrust.wasm

cargo build --target $TARGET --release

mkdir -p web

