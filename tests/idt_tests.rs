#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(Basic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

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
fn test_breakpoint_does_not_crash() {
    x86_64::instructions::interrupts::int3();
}

#[test_case]
fn test_multiple_breakpoints() {
    for _ in 0..3 {
        x86_64::instructions::interrupts::int3();
    }
}