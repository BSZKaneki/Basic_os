mod gdt;
mod idt;
pub mod pic;
pub mod timer;
pub mod keybord; // Add this line!


/// Call this once from `_start` before enabling interrupts.
pub fn init() {
    gdt::init_gdt();
    idt::init_idt();
    unsafe {
        pic::PICS.lock().initialize();
    }
}