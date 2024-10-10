#!/bin/bash
set -e

cargo build --release
cbindgen --config cbindgen.toml --crate solana-c-sdk --output ./header/solana_lib.h
gcc test.c -L target/release/ -lsolana_c_sdk -o test
./test