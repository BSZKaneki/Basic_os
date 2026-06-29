use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::gdt::DOUBLE_FAULT_IST_INDEX;
use crate::interrupts::pic::PICS;
use crate::interrupts::timer;
use crate::interrupts::keybord;
use crate::println;
use lazy_static::lazy_static;





#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,    // Master IRQ 0
    Keyboard = 33, // Master IRQ 1
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!(
        "EXCEPTION: GENERAL PROTECTION FAULT (code: {})\n{:#?}",
        error_code, stack_frame
    );
    loop {}
}


pub const TIMER_INTERRUPT_INDEX: u8 = 32;

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Simply notify the timer module to increment
    timer::increment();

    // Send EOI to PIC
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}


pub const KEYBOARD_INTERRUPT_INDEX: u8 = 33;

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // 1. Forward execution to your clean keyboard driver
    keybord::handle_interrupt();

    // 2. Notify the PIC that the interrupt is completed
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt[TIMER_INTERRUPT_INDEX].set_handler_fn(timer_interrupt_handler);
        idt[KEYBOARD_INTERRUPT_INDEX].set_handler_fn(keyboard_interrupt_handler);


        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
