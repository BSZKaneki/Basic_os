pub mod hardware_interface;
pub mod console_manager;

use hardware_interface::VgaHardware;
use console_manager::Console;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

lazy_static! {
    /// A globally protected Console instance tied directly to VGA hardware.
    pub static ref WRITER: Mutex<Console<VgaHardware>> = Mutex::new(
        // Safety: We are initializing this static exactly once on boot.
        unsafe { Console::new(VgaHardware::new(), 0x0F) } // 0x0F is White on Black
    );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}