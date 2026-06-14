# bitshuffle-rs

Unsafe-only Rust FFI bindings for the core upstream Bitshuffle API.

This crate always builds Bitshuffle from the bundled `bitshuffle/` git submodule.
It does not provide safe wrappers, and it intentionally does not build or expose
Bitshuffle LZ4/ZSTD APIs.

## Submodule

Clone with submodules:

```sh
git clone --recurse-submodules <repo-url>
```

Or initialize the submodule after cloning:

```sh
git submodule update --init --recursive
```
