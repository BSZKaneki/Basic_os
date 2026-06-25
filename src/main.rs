#![no_main]
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(Basic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use Basic_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    Basic_os::init();

    println!("Hello World from our custom architecture!");
    println!("Numbers: {} and {}", 42, 13.37);

    for i in 0..30 {
        println!("This is line count: {}", i);
    }

    println!("We have successfully scrolled!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Basic_os::test_panic_handler(info)
}