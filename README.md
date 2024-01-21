`README.md` is currently being worked on.

## Build
```sh
git clone https://github.com/XandrCopyrighted/xFetch.git
cd xFetch
rustup override set nightly-2024-01-20
rustup component add rust-src --toolchain nightly-2024-01-20-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```
