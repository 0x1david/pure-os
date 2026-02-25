#![no_std]
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(pure_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use pure_os::{
    memory::{self},
    println,
};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pure_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pure_os::test_panic_handler(info)
}

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    pure_os::init();

    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }

    #[cfg(test)]
    test_main();

    println!("Did not crash.");
    pure_os::hlt_loop()
}

#[test_case]
fn trivial_assertion() {
    let x = 1;
    assert_eq!(1, x);
}
