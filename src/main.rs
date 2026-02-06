#![no_std]
#![no_main] // disable Rust-level entry points
mod vga_buffer;

use core::panic::PanicInfo;

/// This is the new entry point -> _start is the name linker looks for as a default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    println!("Here comes numbers: `{}` `{}` !", 42, 1.337);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
