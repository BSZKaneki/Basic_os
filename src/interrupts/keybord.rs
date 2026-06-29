// src/keyboard.rs
use x86_64::instructions::port::Port;
use crate::print; // Import your custom VGA print! macro

/// Handle a keyboard interrupt (IRQ 1).
/// This function is called from the IDT whenever a key is pressed.
pub(crate) fn handle_interrupt() {
    // 1. Read the raw scancode from the keyboard controller port (0x60).
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    // 2. Filter out "key release" codes (which have bit 7 set, i.e., >= 0x80)
    // so we only print when a key is pressed down.
    if scancode < 0x80 {
        let character = match scancode {
            0x02 => Some('1'),
            0x03 => Some('2'),
            0x04 => Some('3'),
            0x05 => Some('4'),
            0x06 => Some('5'),
            0x07 => Some('6'),
            0x08 => Some('7'),
            0x09 => Some('8'),
            0x0A => Some('9'),
            0x0B => Some('0'),
            0x10 => Some('q'),
            0x11 => Some('w'),
            0x12 => Some('e'),
            0x13 => Some('r'),
            0x14 => Some('t'),
            0x15 => Some('y'),
            0x16 => Some('u'),
            0x17 => Some('i'),
            0x18 => Some('o'),
            0x19 => Some('p'),
            0x1E => Some('a'),
            0x1F => Some('s'),
            0x20 => Some('d'),
            0x21 => Some('f'),
            0x22 => Some('g'),
            0x23 => Some('h'),
            0x24 => Some('j'),
            0x25 => Some('k'),
            0x26 => Some('l'),
            0x2C => Some('z'),
            0x2D => Some('x'),
            0x2E => Some('c'),
            0x2F => Some('v'),
            0x30 => Some('b'),
            0x31 => Some('n'),
            0x32 => Some('m'),
            0x39 => Some(' '),  // Spacebar
            0x1C => Some('\n'), // Enter key
            _ => None,
        };

        if let Some(c) = character {
            print!("{}", c);
        }
    }
}