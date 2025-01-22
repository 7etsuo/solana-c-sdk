#!/bin/bash
set -e

cargo build --release
cbindgen --config cbindgen.toml --crate solana-c-sdk --output ./header/solana_sdk.h