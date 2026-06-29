#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(Basic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use Basic_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    Basic_os::init();
    x86_64::instructions::interrupts::enable();


    #[cfg(test)]
    test_main();

    println!("Starting OS tick counter...");
    
    loop {
        // x86_64::instructions::hlt();
        
        // // Clean, safe, and encapsulated function call
        // let current_ticks = Basic_os::interrupts::timer::get_ticks();
        
        // println!("Ticks: {}", current_ticks);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Basic_os::test_panic_handler(info)
}