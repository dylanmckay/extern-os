#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(conservative_impl_trait)]

#![no_std]

pub mod libc;
pub mod vga;

#[no_mangle]
pub extern "C" fn kernel_main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut buffer = vga::Buffer::new();

    loop {
        for cell in buffer.cells_mut() {
            cell.character = 'a' as u8;
        }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern "C" fn __mulodi4() {
    unimplemented!();
}

