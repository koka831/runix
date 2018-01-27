use core::ptr::Unique;

#[allow(dead_code)]

/// TODO: support 16bit color
#[repr(u8)] 
pub enum Color {
    Black   = 0,
    Blue    = 1,
    Green   = 2,
}

#[derive(Clone, Copy)]
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

impl ConsoleWriter {

    // parse bytecodes and output to buf
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= TERM_WIDTH {
                    self.new_line();
                }

                let row = TERM_HEIGHT - 1;
                let col = self.col_pos;
                let color = self.color;

                self.buffer()[row][col] = Char {
                    ascii: byte,
                    color: color,
                };
                self.col_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b)
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buf.as_mut() }
    }

    fn new_line(&mut self) {}
}

/// TODO: check how to test codes with side effect just like I/O
#[allow(dead_code)]
pub fn test_print_console() {
    let mut writer = ConsoleWriter {
        col_pos: 1,
        color: ColorCode::new(Color::Blue, Color::Green),
        buf: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };

    writer.write_str("Hello, World!");
}
