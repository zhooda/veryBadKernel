#![no_std] // don't link the Rust standard library
#![no_main] // disable Rust main fn entry point
#![feature(custom_test_frameworks)]
#![test_runner(very_bad_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use very_bad_kernel::println;

entry_point!(kernel_main);

/// Type checked safe entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use very_bad_kernel::{memory, memory::BootInfoFrameAllocator};
    use x86_64::{
        VirtAddr,
        structures::paging::{Translate, Page},
    };

    println!("INFO: Initializing very_bad_kernel v{}", "0.1.0");
    very_bad_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("INFO: Halting CPU until interrupt");
    very_bad_kernel::hlt_loop();
}

/// this function is called on panic
#[cfg(not(test))] // use this panic handler on non test builds
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    very_bad_kernel::hlt_loop();
}

#[cfg(test)] // use this panic handler on non test builds
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    very_bad_kernel::test_panic_handler(info)
}
