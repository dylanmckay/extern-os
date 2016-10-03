use prelude::*;
use vga;

use core::fmt;

pub struct Terminal
{
    pub vga: vga::Buffer,

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

    pub fn scroll(&mut self, offset: isize) {
        if offset == 0 { return };

        let(width, height) = self.vga.dimensions();

        let mut rows: Vec<Vec<vga::Cell>> = (0..height).into_iter().map(|row_number| {
            (0..width).into_iter().map(|column_number| self.vga[(column_number, row_number)]).collect()
        }).collect();

        let empty_row: Vec<_> = (0..width).into_iter().map(|_| vga::Cell {
            character: ' ' as u8,
            style: self.style,
        }).collect();

        if offset > 0 {
            for _ in 0..offset {
                rows.insert(0, empty_row.clone());
            }

            rows.drain(height..);
        } else {
            let rows_to_remove = -offset as usize;
            rows.drain(0..rows_to_remove);

            while rows.len() < height {
                rows.push(empty_row.clone());
            }
        }

        for (row_number, row) in rows.into_iter().enumerate() {
            for (column_number, cell) in row.into_iter().enumerate() {
                self.vga[(column_number, row_number)] = cell;
            }
        }
    }

    pub fn putc(&mut self, c: char) {
        let (width, height) = self.vga.dimensions();

        match c {
            '\n' => {
                self.row += 1;
                self.column = 0;

                if self.row >= height {
                    self.scroll(-1);
                }
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

        self.update_cursor();
    }

    pub fn position(&mut self, x: usize, y: usize) {
        self.row = y;
        self.column = x;

        self.update_cursor();
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

        self.update_cursor();
    }

    fn update_cursor(&mut self) {
        self.vga.set_cursor_position(self.column, self.row);
    }
}

impl fmt::Write for Terminal
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.puts(s);
        Ok(())
    }
}

