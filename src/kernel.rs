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
pub mod terminal;
pub mod support;
pub mod rlibc;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let vga = vga::Buffer::new();
    let mut terminal = terminal::Terminal::new(vga);

    terminal.clear();
    terminal.puts("foo bar baz bing");
}

