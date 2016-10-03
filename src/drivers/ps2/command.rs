use io;

/// A PS/2 keyboard command.
#[derive(Debug)]
pub enum Command
{
    SetLights(Lights),
    GetScanCode(ScanCodeSet),
    Echo,
}

#[derive(Copy, Clone, Debug)]
pub struct Lights
{
    pub scroll_lock: bool,
    pub number_lock: bool,
    pub caps_lock: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum ScanCodeSet
{
    /// The current scan code set.
    Current,
    Other { number: u8 },
}

impl Command
{
    /// Encodes this command into a command byte, and possibly
    /// a data byte.
    pub fn encode(&self) -> (u8, Option<u8>) {
        match *self {
            Command::SetLights(ref lights) => (0xED, Some(lights.encode())),
            Command::GetScanCode(ref set) => (0xF0, Some(set.encode())),
            Command::Echo => (0xEE, None),
        }
    }

    pub fn send(&self) {
        let mut port = io::Port::open(io::port::Address(0x60));

        let (command_byte, data_byte) = self.encode();
        port.write_u8(command_byte);

        if let Some(data_byte) = data_byte { port.write_u8(data_byte) };
    }
}

impl Lights
{
    pub fn encode(&self) -> u8 {
        let scroll_mask = if self.scroll_lock { 0b100 } else { 0 };
        let number_mask = if self.scroll_lock { 0b010 } else { 0 };
        let caps_mask = if self.scroll_lock { 0b001 } else { 0 };

        scroll_mask | number_mask | caps_mask
    }
}

impl ScanCodeSet
{
    pub fn encode(&self) -> u8 {
        match *self {
            ScanCodeSet::Current => 0,
            ScanCodeSet::Other { number } => number,
        }
    }
}

