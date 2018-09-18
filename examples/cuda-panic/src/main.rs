use cuda::driver;
use cuda::driver::{Any, Block, Device, Grid};

use std::ffi::CString;

fn main() {
    driver::initialize().expect("Unable to initialize CUDA");

    let ptx_bytecode =
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
            .load_module(&ptx_bytecode)
            .expect("Unable to create module")
    };

    let kernel = {
        module
            .function(&kernel_name)
            .expect("Unable to find the kernel")
    };

    println!("You should now see messages right from the kernel:");

    kernel
        .launch(&[Any(&10i32), Any(&0i32)], Grid::x(2), Block::x(8))
        .expect("Unable to run the kernel");
}
