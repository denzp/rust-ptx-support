#![deny(warnings)]
#![no_std]
#![cfg_attr(
    target_os = "cuda",
    feature(proc_macro_hygiene, core_intrinsics, stdsimd)
)]

#[cfg(target_os = "cuda")]
mod context;

#[cfg(target_os = "cuda")]
mod panic_handler;

#[cfg(target_os = "cuda")]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use ptx_support_macros::*;

    pub use crate::context::Context;
}
