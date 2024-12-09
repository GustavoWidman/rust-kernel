use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_SIZE: usize = 160 * 25;

pub struct VGA {
    buffer: &'static mut [u8; BUFFER_SIZE],
    width: u16,
    height: u16,
    line: u16,
    cursor: u16,
}

impl VGA {
    // although the terminal is 160x25, we can optionally set the VGA to a
    // different height and width logic
    // just to print stuff differently
    pub fn new(width: u16, height: u16) -> Self {
        VGA {
            // sneaky hack to get around the borrow checker and get a mutable reference
            // to a buffer in a specific memory region, allowing safe read/write to it afterwards ;)
            // maybe one of the most genius pieces of code i've ever written
            buffer: unsafe { &mut *(0xb8000 as *mut [u8; BUFFER_SIZE]) },
            width,
            height,
            line: 0,
            cursor: 0,
        }
    }

    pub fn clear(&mut self) {
        for (_, byte) in self.buffer.iter_mut().enumerate() {
            *byte = 0;
        }
        self.line = 0;
        self.cursor = 0;
    }

    pub fn clear_fancy(&mut self) {
        for (_, byte) in self.buffer.iter_mut().enumerate() {
            if byte != &0 {
                *byte = 0;
                sleep!(10);
            }
        }
        self.line = 0;
        self.cursor = 0;
        sleep!(100);
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, byte: u8, color: u8) {
        let index: usize = (y * self.width + x * 2).into();
        if index >= BUFFER_SIZE {
            println!(b"index out of bounds");
            return;
        }

        self.buffer[index] = byte;
        self.buffer[index + 1] = color;
    }

    pub fn print(&mut self, string: &[u8], color: u8) {
        for (_, &byte) in string.iter().enumerate() {
            if byte == b'\n' {
                self.line += 1;
                self.cursor = 0; // reset cursor
            } else {
                let index: usize = ((self.line * self.width) + (self.cursor * 2)).into();
                self.buffer[index] = byte;
                self.buffer[index + 1] = color;

                // each cursor unit is 2 bytes, color byte and actual byte
                self.cursor += 1;
            }
        }
        if self.cursor * 2 >= self.width {
            self.line += (self.cursor as u16 * 2) / self.width;
            self.cursor = 0; // reset cursor
        }
    }

    pub fn println(&mut self, string: &[u8], color: u8) {
        self.print(string, color);
        self.line += 1;
        self.cursor = 0; // reset cursor
    }

    pub fn typewrite(&mut self, string: &[u8]) {
        for (_, &byte) in string.iter().enumerate() {
            if byte == b'\n' {
                self.line += 1;
                self.cursor = 0; // reset cursor
            } else {
                let index: usize = ((self.line * self.width) + (self.cursor * 2)).into();
                self.buffer[index] = byte;
                self.buffer[index + 1] = 0x0f;

                // each cursor unit is 2 bytes, color byte and actual byte
                self.cursor += 1;

                // sleep for a bit to make it look like its typing
                sleep!(50);
            }
        }

        if self.cursor * 2 >= self.width {
            self.line += (self.cursor as u16 * 2) / self.width;
            self.cursor = 0; // reset cursor
        }

        // operate as println
        self.line += 1;
        self.cursor = 0; // reset cursor

        sleep!(100);
    }
}

lazy_static! {
    pub static ref VGA_INSTANCE: Mutex<VGA> = Mutex::new(VGA::new(160, 25));
}

macro_rules! print {
    ($a:expr) => {
        crate::utils::console.lock().print($a, 0x0f)
    };
}

macro_rules! println {
    ($a:expr) => {
        crate::utils::console.lock().println($a, 0x0f)
    };
}

pub(crate) use print;
pub(crate) use println;

use super::sleep;
