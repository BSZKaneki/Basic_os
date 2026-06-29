// src/keyboard.rs
use spin::Mutex;
use x86_64::instructions::port::Port;
use pc_keyboard::{layouts, DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1};
use crate::print;

// Use PS2Keyboard instead of Keyboard
static KEYBOARD: Mutex<PS2Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
    PS2Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
);

pub(crate) fn handle_interrupt() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(_key) => (),
            }
        }
    }
}