#![no_std] // don't link the Rust standard library
#![no_main] // disable Rust main fn entry point

mod vga_buffer;

use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this is the entry point since the linker looks for
    // a function `_start` by default

    println!("this is the result of `println!` => Hello, World{}", "!");
    panic!("some panic message");

    loop {}
}