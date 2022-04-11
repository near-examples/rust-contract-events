#!/bin/bash
set -e
cd "$(dirname "$0")"
dir='out'
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
mkdir -p "$dir"
cp target/wasm32-unknown-unknown/release/rust_contract_events.wasm "$dir"
