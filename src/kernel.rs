#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(conservative_impl_trait)]

#![no_std]

pub mod vga;
pub mod support;

/// `libc` functions.
///
/// Taken from the [rlibc](https://crates.io/crates/rlibc) crate.
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
