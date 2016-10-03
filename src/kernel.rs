#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(conservative_impl_trait)]
#![feature(collections)]
#![feature(alloc)]
#![feature(drop_types_in_const)]

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate extern_os_allocator;
extern crate collections;
extern crate alloc;
extern crate rlibc;

pub use self::error::Error;

pub mod io;
pub mod vga;
pub mod drivers;
pub mod interrupt;
pub mod table;
pub mod bios;
pub mod support;
pub mod prelude;
#[macro_use]
pub mod debug;
pub mod error;
pub mod x86;

/// A kernel.
pub trait Kernel
{
    fn initialize();

    fn step();

    fn deinitialize();

    #[allow(unreachable_code)]
    fn run() {
        Self::initialize();
        loop { Self::step(); }
        Self::deinitialize();
    }
}

pub type CurrentKernel = x86::Kernel;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let vga = vga::Buffer::new();
    let mut terminal = vga::Terminal::new(vga);

    terminal.clear();
    debug::initialize(terminal);

    CurrentKernel::run();
}

