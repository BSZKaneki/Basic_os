#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Basic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;




#[unsafe(no_mangle)]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    // This now initializes both the GDT, IDT, and the global PICS
    Basic_os::init();
    
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Basic_os::test_panic_handler(info)
}

/// Tests that the global PIC is correctly remapped and active 
/// with the standard offsets (32 and 40).
#[test_case]
fn test_pic_interrupt_bounds() {
    // We lock the global static PICS instance that was set up in `_start`
    let pics = Basic_os::interrupts::pic::PICS.lock();

    // Verify Master PIC ranges (32 to 39)
    assert!(pics.handles_interrupt(32)); // IRQ 0 (Timer)
    assert!(pics.handles_interrupt(33)); // IRQ 1 (Keyboard)
    assert!(pics.handles_interrupt(39)); // IRQ 7

    // Verify Slave PIC ranges (40 to 47)
    assert!(pics.handles_interrupt(40)); // IRQ 8 (RTC)
    assert!(pics.handles_interrupt(47)); // IRQ 15

    // Verify non-PIC ranges
    assert!(!pics.handles_interrupt(0));   // CPU Exception
    assert!(!pics.handles_interrupt(14));  // CPU Exception
    assert!(!pics.handles_interrupt(48));  // Outside PIC range
}

/// Tests that sending End-of-Interrupt (EOI) signals to the global PIC 
/// instance behaves safely without causing hardware exceptions.
#[test_case]
fn test_global_pic_eoi_transmission() {
    // We lock the global PICS instance for operations.
    // EOI calls are unsafe, so they must be in an unsafe block.
    unsafe {
        let mut pics = Basic_os::interrupts::pic::PICS.lock();

        // 1. Send EOI for a Master PIC interrupt (vector 32)
        pics.notify_end_of_interrupt(32);

        // 2. Send EOI for a Slave PIC interrupt (vector 40)
        pics.notify_end_of_interrupt(40);
        
        // 3. Send EOI for an invalid vector (vector 14)
        // This should be safely ignored by the guard condition.
        pics.notify_end_of_interrupt(14);
    }
}