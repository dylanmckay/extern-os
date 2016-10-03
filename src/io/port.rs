/// An IO port address.
pub struct Address(pub u16);

/// An IO port.
pub struct Port
{
    pub address: Address,
}

impl Port
{
    pub fn open(address: Address) -> Self {
        Port {
            address: address,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.address.0) : "memory" : "intel", "volatile");
        }
        value
    }

    pub fn write_u8(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.address.0) : "memory" : "intel", "volatile");
        }
    }
}

