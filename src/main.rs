#![no_std] // don't link the Rust standard library
#![no_main] // disable Rust main fn entry point
#![feature(custom_test_frameworks)]
#![test_runner(very_bad_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use very_bad_kernel::println;

/// This is the entry point since the linker looks for
/// a function `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("INFO: Initializing very_bad_kernel{}", "!");

    very_bad_kernel::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, return address pushed to stack
    }

    // uncomment line to trigger stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("INFO: Resumed execution after double fault");

    loop {}
}

// this function is called on panic
#[cfg(not(test))] // use this panic handler on non test builds
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)] // use this panic handler on non test builds
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    very_bad_kernel::test_panic_handler(info)
}
