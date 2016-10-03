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

pub mod vga;
pub mod interrupt;
pub mod gdt;
pub mod table;
pub mod bda;
pub mod terminal;
pub mod support;
pub mod rlibc;
pub mod prelude;
#[macro_use]
pub mod debug;

use collections::string::ToString;
use collections::string::String;
use core::fmt;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let vga = vga::Buffer::new();
    let mut terminal = terminal::Terminal::new(vga);
    debug::initialize(terminal);

    for i in 0.. {
        let bda = bda::bios_data_area();

        debug!("Hello {}! {}\n", "world", i);
    }
}

