use core::slice;
use core::ops;

pub const MEMORY: *mut Cell = 0xb8000 as _;

#[repr(u8)]
#[derive(Clone)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct Cell {
    pub character: u8,
    pub color: Color,
}

/// A VGA buffer.
pub struct Buffer
{
    pub width: usize,
    pub height: usize,

    pub memory: &'static mut [Cell],
}

impl Buffer
{
    pub fn new() -> Self {
        let width = 80;
        let height = 25;

        let memory = unsafe {
            slice::from_raw_parts_mut(MEMORY, width*height)
        };

        Buffer {
            width: width,
            height: height,
            memory: memory,
        }
    }

    pub fn clear(&mut self, cell: Cell) {
        for c in self.memory.iter_mut() {
            *c = cell.clone();
        }
    }

    pub fn cells<'a>(&'a self) -> impl Iterator<Item=&'a Cell> {
        self.memory.iter()
    }

    pub fn cells_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut Cell> {
        self.memory.iter_mut()
    }
}

impl ops::Index<(usize, usize)> for Buffer
{
    type Output = Cell;

    fn index(&self, (x,y): (usize, usize)) -> &Cell {
        &self.memory[calculate_index(x, y, self.width)]
    }
}

impl ops::IndexMut<(usize, usize)> for Buffer
{
    fn index_mut(&mut self, (x,y): (usize, usize)) -> &mut Cell {
        &mut self.memory[calculate_index(x, y, self.width)]
    }
}

/// Calculates the index of a cell in a given VGA buffer.
fn calculate_index(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

