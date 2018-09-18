#![deny(warnings)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_non_items))]
#![cfg_attr(target_os = "cuda", no_std)]

#[cfg(target_os = "cuda")]
pub use self::device::*;

#[cfg(target_os = "cuda")]
mod device {
    use ptx_support::prelude::*;

    #[no_mangle]
    pub unsafe extern "ptx-kernel" fn example_kernel(a: i32, b: i32) {
        if Context::thread().index() == (1, 0, 0) {
            assert_eq!(a, b);
        }
    }
}
