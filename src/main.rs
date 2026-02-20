#![no_std]
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(pure_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pure_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pure_os::test_panic_handler(info)
}

/// This is the new entry point -> _start is the name linker looks for as a default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    println!("Here comes the numbers: `{}` `{}` !", 42, 1.337);

    pure_os::init();

    x86_64::instructions::interrupts::int3(); // TODO: Replace with custom IDT

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    let x = 1;
    assert_eq!(1, x);
}
