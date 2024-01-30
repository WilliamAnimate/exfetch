<img src="https://raw.githubusercontent.com/XandrCopyrighted/XandrCopyrighted/main/random/xfetch.jpg" align="right" width="300">

### xFetch
A simple fetch written in Rust.

---

`xFetch` only supports these package managers:
*  pacman
*  yum
*  apt

## Install
* [Arch User Repository](https://aur.archlinux.org/packages/xfetch-bin)
* [GitHub Releases](https://github.com/XandrCopyrighted/xFetch/tags)

## Build
```sh
git clone https://github.com/XandrCopyrighted/xFetch.git
cd xFetch
rustup override set nightly-2024-01-20
rustup component add rust-src --toolchain nightly-2024-01-20-x86_64-unknown-linux-gnu
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile optimized-build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```
