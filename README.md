# Rust PTX Support
[![Current Version](https://img.shields.io/crates/v/ptx-support.svg)](https://crates.io/crates/ptx-support)
[![Current Version](https://img.shields.io/crates/v/ptx-support-macros.svg)](https://crates.io/crates/ptx-support-macros)

## ❗️ EXPERIMENTAL and 🛠 HIGHLY UNSTABLE
Please expect major changes in the APIs.

## Ergonomics questions and missing features
- [x] [Safe `cuda_printf!` macro](examples/cuda-println/src/main.rs#L10)
- [x] Proper panic handler
- [ ] Convinient block and thread accessors (temporary API is here)
- [ ] Dynamic memory allocation (based on `malloc` / `free` syscalls)
