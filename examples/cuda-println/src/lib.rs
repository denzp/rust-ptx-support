#![deny(warnings)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_non_items))]
#![cfg_attr(target_os = "cuda", no_std)]

#[cfg(target_os = "cuda")]
pub use self::device::*;

#[cfg(target_os = "cuda")]
mod device {
    use ptx_support::prelude::*;
    use ptx_support_macros::cuda_printf;

    #[no_mangle]
    pub unsafe extern "ptx-kernel" fn example_kernel(a: f64, b: f64) {
        cuda_printf!(
            "Hello from block(%lu,%lu,%lu) and thread(%lu,%lu,%lu)\n",
            Context::block().index().x,
            Context::block().index().y,
            Context::block().index().z,
            Context::thread().index().x,
            Context::thread().index().y,
            Context::thread().index().z,
        );

        if Context::block().index() == (0, 0, 0) && Context::thread().index() == (0, 0, 0) {
            cuda_printf!("\n");
            cuda_printf!("extra formatting:\n");
            cuda_printf!("int(%f + %f) = int(%f) = %d\n", a, b, a + b, (a + b) as i32);
            cuda_printf!("ptr(\"%s\") = %p\n", "first", "first".as_ptr());
            cuda_printf!("ptr(\"%s\") = %p\n", "other", "other".as_ptr());
        }
    }
}
