#!/bin/sh
RUSTFLAGS="-Zlocation-detail=none -Zthreads=8" cargo +nightly build --target x86_64-unknown-linux-gnu --profile release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
