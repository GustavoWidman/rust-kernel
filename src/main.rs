#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod utils;

use core::panic::{self, PanicInfo};
use utils::{console, print, println, sleep};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();

    loop {
        x86_64::instructions::hlt();
    }
}

fn main() {
    console.lock().typewrite(b"VGA driver test");
    console.lock().typewrite(b"by r3dlust");
    sleep!(5000);
    console.lock().clear();
    sleep!(1000);
    print!(b"This doesnt have a newline attached, ");
    sleep!(3000);
    println!(b"but this does...");
    sleep!(3000);
    println!(b"See? It's a newline!");
    sleep!(3000);
    println!(
        b"When I type really long lines like this one, they end up wrapping around forcibly and going to the next line automatically so you can read the text without any issues!"
    );
    sleep!(5000);
    println!(b"Now I'm going to clear the screen!");
    sleep!(3000);
    console.lock().clear();
    sleep!(1000);
    console.lock().typewrite(b"This is a typewriter effect!");
    console
        .lock()
        .typewrite(b"You can also clear the screen with a similar effect...");
    sleep!(5000);
    console.lock().clear_fancy();
    sleep!(1000);
    println!(b"Now I'm going to draw a hollow box 20 pixels wide and 5 pixels tall");
    sleep!(5000);
    console.lock().clear();
    sleep!(1000);
    // draw a hollow box 20 pixels wide and 5 pixels tall
    let BOX_WIDE = 20;
    let BOX_TALL = 5;
    for i in 0..BOX_WIDE {
        // top
        console.lock().set_pixel(i, 0, '-' as u8, 0x0f);

        // bottom
        console.lock().set_pixel(i, BOX_TALL - 1, '-' as u8, 0x0f);

        // delays are added to make it look like its really drawing
        // the box instead of just spawning it in all at once
        sleep!(100);
    }

    for i in 0..BOX_TALL {
        // left
        console.lock().set_pixel(0, i, '|' as u8, 0x0f);

        // right
        // dont add -1 to box_wide because we want to draw the last pixel
        console.lock().set_pixel(BOX_WIDE, i, '|' as u8, 0x0f);

        // delays are added to make it look like its really drawing
        // the box instead of just spawning it in all at once
        sleep!(100);
    }

    sleep!(5000);
    console.lock().clear_fancy();
    sleep!(1000);
    console
        .lock()
        .typewrite(b"That's it! I hope you enjoyed this demo!");
    sleep!(5000);
    console.lock().clear_fancy();
    sleep!(1000);
    panic!("Kernel panic just to test panic_handler");

    // result should look like this:
    //
    // |------------------|
    // |                  |
    // |                  |
    // --------------------
    //
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console.lock().clear();
    console
        .lock()
        .println(b"Kernel panic. Go kill yourself.", 0x0c);
    let info_string: &[u8] = info.message().as_str().unwrap().as_bytes();
    let mut buf = [0u8; 64];
    let panic_origin = format_no_std::show(
        &mut buf,
        format_args!(
            "{} - {}:{}",
            info.location().unwrap().file(),
            info.location().unwrap().line(),
            info.location().unwrap().column()
        ),
    )
    .unwrap();
    console.lock().print(b"\nPanic message: ", 0x0c);
    console.lock().println(info_string, 0x0f);
    console.lock().print(b"\nPanic location: ", 0x0c);
    console.lock().println(panic_origin.as_bytes(), 0x0f);
    loop {}
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    loop {}
}
