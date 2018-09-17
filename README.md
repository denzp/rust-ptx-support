# Rust PTX Support
[![Current Version](https://img.shields.io/crates/v/ptx-support.svg)](https://crates.io/crates/ptx-support)
[![Current Version](https://img.shields.io/crates/v/ptx-support-macros.svg)](https://crates.io/crates/ptx-support-macros)

## ❗️ EXPERIMENTAL and 🛠 HIGHLY UNSTABLE
Please expect major changes in the APIs.

## Ergonomics questions and missing features
- [x] [safe `cuda_printf!` macro](examples/cuda-println/src/lib.rs#L15)
- [ ] proper panic handler (based on `__assertfail` syscall)
- [ ] convinient block and thread accessors (temporary API is here)
- [ ] dynamic memory allocation (based on `malloc` / `free` syscalls)

## Examples
Please checkout examples and play around with `cuda_printf!`.
