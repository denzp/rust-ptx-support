#![deny(warnings)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx))]
#![cfg_attr(target_os = "cuda", no_std)]

#[no_mangle]
#[cfg(target_os = "cuda")]
pub unsafe extern "ptx-kernel" fn example_kernel(a: i32, b: i32) {
    use ptx_support::prelude::*;

    if Context::thread().index() == (1, 0, 0) {
        assert_eq!(a, b);
    }
}

#[cfg(not(target_os = "cuda"))]
fn main() {
    use cuda::driver;
    use cuda::driver::{Any, Block, Device, Grid};
    use std::ffi::CString;

    driver::initialize().expect("Unable to initialize CUDA");

    let ptx_assembly =
        CString::new(include_str!(env!("KERNEL_PTX_PATH"))).expect("Unable to create sources");

    let kernel_name = CString::new("example_kernel").expect("Unable to create kernel name string");

    let context = {
        Device(0)
            .expect("Unable to get CUDA device 0")
            .create_context()
            .expect("Unable to create CUDA context")
    };

    let module = {
        context
            .load_module(&ptx_assembly)
            .expect("Unable to create module")
    };

    let kernel = {
        module
            .function(&kernel_name)
            .expect("Unable to find the kernel")
    };

    println!("You should now see a panic right from the kernel:");

    kernel
        .launch(&[Any(&10i32), Any(&0i32)], Grid::x(2), Block::x(8))
        .unwrap_err();
}
