#!/bin/sh
RUSTFLAGS="-Zlocation-detail=none -Z threads=8" cargo +nightly build --target x86_64-pc-windows-msvc --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
