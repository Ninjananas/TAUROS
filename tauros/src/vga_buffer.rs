use core::fmt;

extern "C" {
    static DIRECT_MAP_OFFSET: u64;
}

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
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer {
    pub col_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                let row = BUFFER_HEIGHT - 1;
                let col = self.col_position;
                let cc = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: cc,
                };
                self.col_position = (self.col_position + 1) % BUFFER_WIDTH;
                if self.col_position == 0 {
                    self.new_line();
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // First values are treated as ColorCode indications (fg or bg)
                0x00..=0x0f => self.color_code = ColorCode((self.color_code.0 & (0xf0)) | (byte)),
                0x10..=0x1f => self.color_code = ColorCode((byte << 4) | (self.color_code.0 % 16)),

                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let chr = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = chr;
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.col_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut w = Writer {
        col_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe {&mut *((0xb8000 + DIRECT_MAP_OFFSET) as *mut Buffer)},
    };
    w.write_fmt(args).unwrap();
}
