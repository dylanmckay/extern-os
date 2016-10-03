

/// Writes a byte to an I/O port.
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!("out $1, $0" : : "{al}"(value), "{dx}"(port) : "memory" : "intel", "volatile");
}

/// Reads a byte from an I/O port.
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in $0, $1" : "={al}"(value) : "{dx}"(port) : "memory" : "intel", "volatile");
    value
}

pub unsafe fn test() {
    outb(0xbeef, 69);
}
