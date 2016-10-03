use vga;

use core::fmt;

pub struct Terminal
{
    vga: vga::Buffer,

    row: usize,
    column: usize,

    style: vga::Style,
}

impl Terminal
{
    pub fn new(vga: vga::Buffer) -> Self {
        Terminal {
            vga: vga,
            row: 0,
            column: 0,
            style: vga::Style::with(vga::Color::White).on(vga::Color::Black),
        }
    }

    pub fn putc(&mut self, c: char) {
        let (width, _) = self.vga.dimensions();

        match c {
            '\n' => {
                self.row += 1;
                self.column = 0;
            },
            '\r' => self.column = 0,
            '\t' => {
                for _ in 0..2 { self.putc(' ') }
            },
            c => {
                self.vga[(self.column, self.row)] = vga::Cell {
                    character: c as u8,
                    style: self.style,
                };

                self.column += 1;
            },
        }

        if self.column >= width {
            self.row += 1;
            self.column = 0
        }
    }

    pub fn position(&mut self, x: usize, y: usize) {
        self.row = y;
        self.column = x;
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.chars() { self.putc(c) }
    }

    pub fn clear(&mut self) {
        self.vga.clear(vga::Cell {
            character: ' ' as u8,
            style: self.style,
        });

        self.row = 0;
        self.column = 0;
    }
}

impl fmt::Write for Terminal
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.puts(s);
        Ok(())
    }
}

