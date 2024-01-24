#!/bin/sh
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
