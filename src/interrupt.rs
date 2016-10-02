
#[naked]
pub fn interrupt_handler() {
    unsafe {
        asm!("pushal");
        asm!("cld");

        // Do something.

        asm!("popal");
        asm!("iret");
    }
}

