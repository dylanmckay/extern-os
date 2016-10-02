//! Support routines required by the compiler.

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern "C" fn __mulodi4() {
    unimplemented!();
}

