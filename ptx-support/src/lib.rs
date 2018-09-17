#![deny(warnings)]
#![no_std]
#![cfg_attr(target_os = "cuda", feature(platform_intrinsics))]

#[cfg(target_os = "cuda")]
mod context;

#[cfg(target_os = "cuda")]
pub mod prelude {
    pub use crate::context::Context;
}
