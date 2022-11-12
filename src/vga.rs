use core::fmt;
use voladdress::{Safe, VolBlock};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

fn make_vga_entry(ascii: u8, fg: Color, bg: Color) -> u16 {
    let color: u16 = ((fg as u8) | (bg as u8) << 4) as u16;
    return (ascii as u16) | color << 8;
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_SIZE: usize = VGA_WIDTH * VGA_HEIGHT;

pub struct Writer {
    cursor_x: usize,
    fg_color: Color,
    bg_color: Color,
    framebuffer: VolBlock<u16, Safe, Safe, VGA_SIZE>,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let row = VGA_HEIGHT - 1;
                let col = self.cursor_x;

                self.framebuffer
                    .index(row * VGA_WIDTH + col)
                    .write(make_vga_entry(byte, self.fg_color, self.bg_color));
                self.cursor_x += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let character = self.framebuffer.index(row * VGA_WIDTH + col).read();
                self.framebuffer.index((row-1) * VGA_WIDTH + col).write(character);
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.cursor_x = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_WIDTH {
            self.framebuffer
                .index(row * VGA_WIDTH + col)
                .write(make_vga_entry(b' ', self.fg_color, self.bg_color))
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor_x: 0,
        bg_color: Color::Black,
        fg_color: Color::LightGrey,
        framebuffer: unsafe { VolBlock::new(0xb8000) },
    });
}
