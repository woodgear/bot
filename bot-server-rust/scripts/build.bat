#!/bin/bash
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release
if %errorlevel% neq 0 exit /b %errorlevel%