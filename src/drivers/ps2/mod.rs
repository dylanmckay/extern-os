pub use self::command::Command;
pub mod command;

pub struct Keyboard;

impl Keyboard
{
    pub fn get_scancode() -> Option<u8> {
        let mut port = ::io::Port::open(::io::port::Address(0x60));

        port.write_u8(0xf0);
        port.write_u8(0x00);
        let c = port.read_u8();

        if c != 0xfe { Some(port.read_u8()) } else { None }
    }
}

