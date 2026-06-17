use core::fmt;

// Color definitions for VGA text mode
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Combines foreground and background colors into a single byte
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    // Create a new ColorCode from foreground and background colors
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Represents a single character on the VGA text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,

    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Represents the VGA text buffer (2D array of ScreenChar)
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer for the VGA buffer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    // Write a single byte to the buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 2;
                let col = self.column_position;
                let color_code = self.color_code;
                let target_ptr = &mut self.buffer.chars[row][col] as *mut ScreenChar;
                unsafe {
                    core::ptr::write_volatile(
                        target_ptr,
                        ScreenChar {
                            ascii_character: byte,
                            color_code,
                        },
                    );
                }
                self.column_position += 1;
            }
        }
    }

    // Shift all lines up by one and clear the last line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            let src_ptr = &self.buffer.chars[row][0] as *const ScreenChar;
            let dst_ptr = &mut self.buffer.chars[row - 1][0] as *mut ScreenChar;

            unsafe {
                // Copy the entire row at once
                core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, BUFFER_WIDTH);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // Clear a row by filling it with spaces
    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
        }
    }
}

// Implement core::fmt::Write for Writer so we can use write! and writeln!
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    // Write a string to the buffer, handling non-printable bytes
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// Example function for manual testing (commented out)
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 20,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}

use lazy_static::lazy_static;
use spin::Mutex;

// Global WRITER instance, protected by a spinlock for safe concurrent access
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Macro for print! that uses our VGA buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// Macro for println! that uses our VGA buffer
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Internal function used by print!/println! macros to write formatted text
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
