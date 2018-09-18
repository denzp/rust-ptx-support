#![deny(warnings)]
#![no_std]
#![cfg_attr(
    target_os = "cuda",
    feature(platform_intrinsics, proc_macro_non_items, core_intrinsics)
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
