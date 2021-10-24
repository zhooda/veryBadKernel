#![no_std] // don't link the Rust standard library
#![no_main] // disable Rust main fn entry point

use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this is the entry point since the linker looks for
    // a function `_start` by default
    loop {}
}