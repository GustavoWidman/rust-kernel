mod sleep;
mod vga;

pub(crate) use sleep::sleep;
pub(crate) use sleep::sleep_ms;
pub(crate) use vga::VGA_INSTANCE as console;
pub(crate) use vga::{print, println};
