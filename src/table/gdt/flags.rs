#[derive(Copy, Clone, Debug)]
pub enum Size
{
    Bit16 = 0,
    Bit32 = 1,
}

/// The granularity of the limit set in the entry.
#[derive(Copy, Clone, Debug)]
pub enum Granularity
{
    Byte = 0,
    Page = 1,
}

#[derive(Copy, Clone, Debug)]
pub struct Flags
{
    pub granularity: Granularity,
    pub size: Size,
}

impl Flags
{
    pub fn null() -> Self {
        Flags {
            granularity: Granularity::Byte,
            size: Size::Bit16,
        }
    }

    pub fn encode(&self) -> u8 {
        let granularity_number = self.granularity as u8;
        let size_number = self.size as u8;

        (granularity_number << 7) | (size_number << 6)
    }
}

