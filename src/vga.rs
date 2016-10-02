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

impl Color
{
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0  => Some(Color::Black),
            1  => Some(Color::Blue),
            2  => Some(Color::Green),
            3  => Some(Color::Cyan),
            4  => Some(Color::Red),
            5  => Some(Color::Magenta),
            6  => Some(Color::Brown),
            7  => Some(Color::LightGrey),
            8  => Some(Color::DarkGrey),
            9  => Some(Color::LightBlue),
            10 => Some(Color::LightGreen),
            11 => Some(Color::LightCyan),
            12 => Some(Color::LightRed),
            13 => Some(Color::LightMagenta),
            14 => Some(Color::LightBrown),
            15 => Some(Color::White),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Style(u8);

impl Style
{
    pub fn new(foreground: Color, background: Color) -> Self {
        Style(((background as u8) << 4) | foreground as u8)
    }

    pub fn with(color: Color) -> Self {
        Style::new(color, Color::Black)
    }

    pub fn on(mut self, background: Color) -> Self {
        self.set_background(background);
        self
    }

    pub fn background(&self) -> Color {
        Color::from_code((self.0 & 0xf0) >> 4).unwrap()
    }

    pub fn set_background(&mut self, color: Color) {
        self.0 |= (color as u8) << 4;
    }

    pub fn foreground(&self) -> Color {
        Color::from_code(self.0 & 0x0f).unwrap()
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.0 |= color as u8;
    }
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct Cell {
    pub character: u8,
    pub style: Style,
}

/// A VGA buffer.
pub struct Buffer
{
    width: usize,
    height: usize,

    memory: &'static mut [Cell],
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

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn dimensions(&self) -> (usize, usize) { (self.width, self.height) }

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

