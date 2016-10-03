use core;

/// An IO port address.
pub struct Address(pub u16);

/// An IO port.
pub struct Port<T>
{
    pub address: Address,
    pub phantom: core::marker::PhantomData<T>,
}

impl<T> Port<T>
{
    pub fn open(address: Address) -> Self {
        Port {
            address: address,
            phantom: core::marker::PhantomData,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.address.0) : "memory" : "intel", "volatile");
        }
        value
    }
}

