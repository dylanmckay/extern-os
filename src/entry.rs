#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![no_std]

pub mod libc;
pub mod vga;

#[no_mangle]
pub extern "C" fn kernel_main(_argc: isize, _argv: *const *const u8) -> isize {
    use vga;

    let mut vmem = unsafe { core::slice::from_raw_parts_mut(vga::MEMORY, 80*25) };
    loop {
        for cell in vmem.iter_mut() {
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

