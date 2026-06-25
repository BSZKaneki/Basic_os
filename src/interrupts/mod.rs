mod gdt;
mod idt;

/// Call this once from `_start` before enabling interrupts.
pub fn init() {
    gdt::init_gdt();
    idt::init_idt();
}