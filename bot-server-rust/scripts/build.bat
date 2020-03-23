#!/bin/bash
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release