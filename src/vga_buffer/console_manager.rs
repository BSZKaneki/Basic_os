use crate::vga_buffer::hardware_interface::TextDisplay;
use core::fmt;

pub struct Console<D: TextDisplay> {
    display: D,
    cursor_x: usize,
    cursor_y: usize,
    color: u8,
}

impl<D: TextDisplay> Console<D> {
    /// Create a new console wrapping a specific hardware display.
    pub fn new(display: D, default_color: u8) -> Self {
        let mut console = Self {
            display,
            cursor_x: 0,
            cursor_y: 0,
            color: default_color,
        };
        console.clear();
        console
    }

    /// Clear the entire screen and reset the cursor to the top-left.
    pub fn clear(&mut self) {
        for row in 0..self.display.height() {
            self.display.clear_row(row, self.color);
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Set a new text color.
    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    /// Write a raw byte to the screen, handling newlines and scrolling.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // If we reach the end of a line, wrap to the next line
                if self.cursor_x >= self.display.width() {
                    self.new_line();
                }

                self.display.write_char(self.cursor_x, self.cursor_y, byte, self.color);
                self.cursor_x += 1;
            }
        }
    }

    /// Move the cursor to the next line, triggering a scroll if we are at the bottom.
    fn new_line(&mut self) {
        self.cursor_x = 0;
        
        let max_y = self.display.height() - 1;
        if self.cursor_y < max_y {
            // We still have room on the screen, just move down
            self.cursor_y += 1;
        } else {
            // We hit the bottom! Scroll everything up by 1 row
            for row in 1..=max_y {
                self.display.copy_row(row, row - 1);
            }
            // Clear the newly blank bottom row
            self.display.clear_row(max_y, self.color);
        }
    }
}

// Implement standard formatting traits so write! and writeln! macros work
impl<D: TextDisplay> fmt::Write for Console<D> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII and newlines
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Fallback character for non-printable ASCII
                _ => self.write_byte(0xfe), 
            }
        }
        Ok(())
    }
}