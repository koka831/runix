use core::ptr::Unique;

#[allow(dead_code)]

#[repr(u8)]
pub enum Color {
    Black   = 0,
    Blue    = 1,
    Green   = 2,
}

pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[repr(C)]
pub struct Char {
    pub ascii: u8,
    pub color: ColorCode,
}

pub const TERM_WIDTH: usize = 80;
pub const TERM_HEIGHT: usize = 25;
pub type Buffer = [[Char; TERM_WIDTH]; TERM_HEIGHT];


#[allow(dead_code)]
pub struct ConsoleWriter {
    col_pos: usize,
    color: ColorCode,
    buf: Unique<Buffer>,
}

impl ConsoleWriter
