#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(conservative_impl_trait)]
#![feature(collections)]

#![no_std]

extern crate extern_os_allocator;
extern crate collections;

pub mod vga;
pub mod interrupt;
pub mod support;
pub mod rlibc;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut buffer = vga::Buffer::new();

    loop {
        for cell in buffer.cells_mut() {
            cell.character = 'a' as u8;
        }
    }
}
