const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// A generic interface representing any text-based output screen.
pub trait TextDisplay {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn write_char(&mut self, x: usize, y: usize, character: u8, color: u8);
    fn copy_row(&mut self, src_row: usize, dst_row: usize);
    fn clear_row(&mut self, row: usize, fill_color: u8);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Direct interface to the VGA text mode memory.
pub struct VgaHardware {
    buffer: &'static mut Buffer,
}

impl VgaHardware {
    /// Create an instance of the hardware interface.
    /// 
    /// # Safety
    /// This function is unsafe because the caller must guarantee that `0xb8000`
    /// is mapped to a valid VGA buffer and that no other part of the code is 
    /// writing to this memory address concurrently.
    pub unsafe fn new() -> Self {
        Self {
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
}

impl TextDisplay for VgaHardware {
    fn width(&self) -> usize {
        BUFFER_WIDTH
    }

    fn height(&self) -> usize {
        BUFFER_HEIGHT
    }

    fn write_char(&mut self, x: usize, y: usize, character: u8, color: u8) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            unsafe {
                core::ptr::write_volatile(
                    &mut self.buffer.chars[y][x],
                    ScreenChar {
                        ascii_character: character,
                        color_code: color,
                    },
                );
            }
        }
    }

    fn copy_row(&mut self, src_row: usize, dst_row: usize) {
        if src_row < BUFFER_HEIGHT && dst_row < BUFFER_HEIGHT {
            let src_ptr = &self.buffer.chars[src_row][0] as *const ScreenChar;
            let dst_ptr = &mut self.buffer.chars[dst_row][0] as *mut ScreenChar;
            unsafe {
                core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, BUFFER_WIDTH);
            }
        }
    }

    fn clear_row(&mut self, row: usize, fill_color: u8) {
        if row < BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row][col],
                        ScreenChar {
                            ascii_character: b' ',
                            color_code: fill_color,
                        },
                    );
                }
            }
        }
    }
}