#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Basic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use Basic_os::println;
use core::panic::PanicInfo;
use bootloader::BootInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    Basic_os::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Basic_os::test_panic_handler(info)
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many_lines() {
    for i in 0..50 {
        println!("line {}", i);
    }
}