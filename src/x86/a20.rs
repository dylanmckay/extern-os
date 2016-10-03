/// Enables the A20 line.
pub fn enable() {
    // FIXME: check if the line is already enabled.
    self::fast();
}

/// Performs the fast A20 line enable.
/// Not supported on all processors.
pub fn fast() {
    unsafe {
        asm!("inb $$0x92, %al");
        asm!("orb $$2, %al");
        asm!("outb %al, $$0x92");
    }
}

