pub fn sleep_ms(ms: u32) {
    // this pins a core balls to the fucking walls
    // please fix this eventually.
    // "busy loop"
    let cycles = ms * 5000;
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}

macro_rules! sleep {
    ($a:expr) => {
        crate::utils::sleep_ms($a);
    };
}

pub(crate) use sleep;
