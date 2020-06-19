#![deny(warnings)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx, proc_macro_hygiene))]
#![cfg_attr(target_os = "cuda", no_std)]

#[no_mangle]
#[cfg(target_os = "cuda")]
pub unsafe extern "ptx-kernel" fn example_kernel(a: f64, b: f64) {
    use ptx_support::prelude::*;

    cuda_printf!(
        "Hello from block(%lu,%lu,%lu) and thread(%lu,%lu,%lu), with grid (%lu,%lu,%lu)\n",
        Context::block().index().x,
        Context::block().index().y,
        Context::block().index().z,
        Context::thread().index().x,
        Context::thread().index().y,
        Context::thread().index().z,
        Context::grid().dims().x,
        Context::grid().dims().y,
        Context::grid().dims().z,
    );

    if Context::block().index() == (0, 0, 0) && Context::thread().index() == (0, 0, 0) {
        cuda_printf!("\n");
        cuda_printf!("extra formatting:\n");
        cuda_printf!("int(%f + %f) = int(%f) = %d\n", a, b, a + b, (a + b) as i32);
        cuda_printf!("ptr(\"%s\") = %p\n", "first", "first".as_ptr());
        cuda_printf!("ptr(\"%s\") = %p\n", "other", "other".as_ptr());
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

    println!("You should now see messages right from the kernel:");

    kernel
        .launch(&[Any(&11.63), Any(&15.36)], Grid::x(2), Block::x(8))
        .unwrap_err();
}
