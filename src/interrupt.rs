pub fn disable() {
    unsafe { asm!("cli") };
}

pub fn disable_all() {
    disable();
    non_maskable::disable();
}

pub fn enable() {
    unsafe { asm!("sti") };
}

pub fn enable_all() {
    enable();
    non_maskable::enable();
}

pub mod non_maskable
{
    use io;

    pub const PORT_ADDRESS: io::port::Address = io::port::Address(0x70);

    pub fn disable() {
        let val = port().read_u8() | 0x80;
        port().write_u8(val);
    }

    pub fn enable() {
        let val = port().read_u8() & 0x7f;
        port().write_u8(val);
    }

    fn port() -> io::Port { io::Port::open(PORT_ADDRESS) }
}

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

