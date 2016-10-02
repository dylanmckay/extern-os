#![feature(lang_items)]
#![feature(start)]
#![no_std]

#[no_mangle]
pub extern "C" fn kernel_main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

