#![no_std]
#![no_main] // disable Rust-level entry points

use core::panic::PanicInfo;

/// This is the new entry point -> _start is the name linker looks for as a default
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
