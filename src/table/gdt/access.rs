/// Defines a ring level.
#[derive(Copy, Clone, Debug)]
pub enum Ring
{
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

/// Either a direction or a confiorming.
#[derive(Copy, Clone, Debug)]
pub enum DC
{
    Direction(Direction),
    Conforming(Conforming),
}

/// The direction for a data selector.
#[derive(Copy, Clone, Debug)]
pub enum Direction
{
    GrowsUp   = 0,
    GrowsDown = 1,
}

/// Conforming bit for code selectors.
#[derive(Copy, Clone, Debug)]
pub enum Conforming
{
    /// The code in segment can only be executed from the same
    /// privilege level specified in the access.
    Equal,
    /// The code in segment can only be executed from the same
    /// privilege level specified in the access, or a higher level.
    EqualOrLower = 1,
}

#[derive(Copy, Clone, Debug)]
pub struct Access
{
    pub present: bool,
    pub privilege: Ring,
    pub executable: bool,
    pub direction_conforming: DC,
    pub readable_writable: bool,
    pub accessed: bool,
}

impl Access
{
    pub fn null() -> Self {
        Access {
            present: false,
            privilege: Ring::Ring0,
            executable: false,
            direction_conforming: DC::Direction(Direction::GrowsUp),
            readable_writable: false,
            accessed: false,
        }
    }

    pub fn executable(privilege: Ring, conforming: Conforming, writable: bool) -> Self {
        Access {
            present: true,
            privilege: privilege,
            executable: true,
            direction_conforming: DC::Conforming(conforming),
            readable_writable: writable,
            accessed: false,
        }
    }

    pub fn data(privilege: Ring, direction: Direction, readable: bool) -> Self {
        Access {
            present: true,
            privilege: privilege,
            executable: false,
            direction_conforming: DC::Direction(direction),
            readable_writable: readable,
            accessed: false,
        }
    }

    pub fn encode(&self) -> u8 {
        let mut data = 0b00010000;

        if self.present { data |= 0b10000000 };
        if self.executable { data |= 0b0000100 };
        if self.readable_writable { data |= 0b10 };
        if self.accessed { data |= 0b1 };

        match self.direction_conforming {
            DC::Direction(Direction::GrowsDown) |
                DC::Conforming(Conforming::EqualOrLower) => {
                data |= 0b100;
            },
            _ => (),
        }

        let privilege_mask = match self.privilege {
            Ring::Ring0 => 0,
            Ring::Ring1 => 0b00100000,
            Ring::Ring2 => 0b01000000,
            Ring::Ring3 => 0b01100000,
        };

        data |= privilege_mask;
        data
    }
}

