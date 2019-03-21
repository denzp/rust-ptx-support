use core::panic::PanicInfo;

#[cfg(feature = "macros")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::prelude::*;

    let (file, line) = match info.location() {
        Some(location) => (location.file(), location.line()),
        None => ("unknown", 0),
    };

    cuda_printf!(
        "Kernel panicked at '%s:%u' on block(%lu,%lu,%lu) and thread(%lu,%lu,%lu).\n",
        file,
        line,
        Context::block().index().x,
        Context::block().index().y,
        Context::block().index().z,
        Context::thread().index().x,
        Context::thread().index().y,
        Context::thread().index().z,
    );

    unsafe {
        core::intrinsics::breakpoint();
        core::hint::unreachable_unchecked();
    }
}

#[cfg(not(feature = "macros"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::ptr::null;

    extern "C" {
        fn __assertfail(
            message: *const u8,
            file: *const u8,
            line: u32,
            function: *const u8,
            char_size: u64,
        );
    }

    let (file, line) = match info.location() {
        Some(location) => (location.file(), location.line()),
        None => ("unknown", 0),
    };

    unsafe {
        __assertfail("Panicked".as_ptr(), file.as_ptr(), line, null(), 1);
        core::hint::unreachable_unchecked();
    }
}
