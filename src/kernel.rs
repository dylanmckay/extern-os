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

extern crate extern_os_allocator;
extern crate collections;
extern crate alloc;
extern crate rlibc;

pub use self::error::Error;

pub mod io;
pub mod vga;
pub mod interrupt;
pub mod table;
pub mod bios;
pub mod support;
pub mod prelude;
#[macro_use]
pub mod debug;
pub mod error;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let vga = vga::Buffer::new();
    let mut terminal = vga::Terminal::new(vga);
    // debug::initialize(terminal);

    terminal.clear();
    terminal.puts("foo bar");

    for _ in 0.. {
        // let bda = bios::data_area();
        // debug!("Data: {:?}", bda);
    }
}

