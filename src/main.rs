#![no_std] // don't link the Rust standard library
#![no_main] // disable Rust main fn entry point

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, World!";

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this is the entry point since the linker looks for
    // a function `_start` by default

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }
    
    loop {}
}