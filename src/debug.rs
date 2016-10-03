use prelude::*;
use core;

/// Where debug messages go to die.
static mut DESTINATION: Option<Box<core::fmt::Write>> = None;

/// Writes a debugging message.
macro_rules! debug {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        ::debug::with_debugger(|destination| {
            ::core::fmt::write(destination,
                format_args!($fmt_string $( , $arg )+)
            ).unwrap();
        });
    }
}

/// Sets up the debugging output destination.
pub fn initialize<W: core::fmt::Write+'static>(destination: W) {
    unsafe {
        DESTINATION = Some(Box::new(destination) as _);
    }
}

pub fn with_debugger<F>(f: F) where F: FnOnce(&'static mut core::fmt::Write) {
    unsafe {
        if let Some(destination) = DESTINATION.as_mut() {
            f(destination.as_mut())
        } else {
            // The messge has nowhere to go.
        }
    }
}

