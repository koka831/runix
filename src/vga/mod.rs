use core::ptr::Unique;

#[allow(dead_code)]


#[repr(u8)] 
pub enum Color {
    Black       = 0x0,
    Blue        = 0x1,
    Green       = 0x2,
    Cyan        = 0x3,
    Red         = 0x4,
    Magenta     = 0x5,
    Brown       = 0x6,
    LightGray   = 0x7,
    DarkGray    = 0x8,
    LigtBlue    = 0x9,
    LightGreen  = 0xa,
    LightCyan   = 0xb,
    LightRed    = 0xc,
    Pink        = 0xd,
    Yellow      = 0xe,
    White       = 0xf,
}

#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }

    pub const fn new_u8(fg: u8, bg: u8) -> ColorCode {
        ColorCode((bg) << 4 | fg)
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
    x: usize,
    y: usize,
    color: ColorCode,
    buf: Unique<Buffer>,
}

impl ConsoleWriter {

    // parse bytecodes and output to buf
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => { self.y += 1; }
            byte => {
                if self.x >= TERM_WIDTH {
                    self.y += 1;
                }

                let row = self.y;
                let col = self.x;
                let color = self.color;

                self.buffer()[row][col] = Char {
                    ascii: byte,
                    color: color,
                };
                self.x += 1;
            }
        }
    }
/*
    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b)
        }
    }
*/
    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buf.as_mut() }
    }

}

/// for write! / writeln! macros
use core::fmt::Write;
use core::fmt;
impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b)
        }
        Ok(())
    }
}

/// TODO: check how to test codes with side effect just like I/O
#[allow(dead_code)]
pub fn test_print_console() {
    let writer = ConsoleWriter {
        x: 1,
        y: 1,
        color: ColorCode::new(Color::Blue, Color::Green),
        buf: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };

    //writer.write_str("Hello, World!");
}

/// TODO: need way somehow to run testcases on vm like qemu
#[allow(dead_code)]
pub fn test_print_color() {
    let mut row;
    let mut col = 15;
    for j in 0..8 {
        row = 0;
        for k in 0..8 {
            let mut writer = ConsoleWriter {
                x: row,
                y: col,
                color: ColorCode::new_u8(j, k),
                buf: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
            };
            // TODO#fix: writer shoult manage buffer position
            write!(writer, "fg:{},bg:{}", j, k).unwrap();
            row += "fg:i,bg:j".len();
        }
        col += 1;
    }
}
