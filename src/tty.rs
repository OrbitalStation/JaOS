/****************************************************************/
//                            Uses                              //
/****************************************************************/

use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const WIDTH : u8 = 80;
pub const HEIGHT: u8 = 25;

pub const DEFAULT_FOREGROUND: Color = Color::Yellow;
pub const DEFAULT_BACKGROUND: Color = Color::Black;

/****************************************************************/
//                            Types                             //
/****************************************************************/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

/* VGA colors */
pub enum Color {
    Black,
    Blue1,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,

    Default, //< Use this one to mark default color(fore: Yellow, back: Black)

    Count //< Count of all colors
}

impl Color {
    const fn const_eq(self, other: Self) -> bool {
        self as u8 == other as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]

/* Represents VGA color */
pub struct VGA(u8);

impl VGA {
    /* Makes VGA color from foreground and background */
    pub const fn make(foreground: Color, background: Color) -> VGA {
        VGA((if background.const_eq(Color::Default) { (DEFAULT_BACKGROUND as u8) << 4 } else { (background as u8) << 4 }) | (if foreground.const_eq(Color::Default) { DEFAULT_FOREGROUND as u8 } else { foreground as u8 }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: VGA,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile <ScreenChar>; WIDTH as usize]; HEIGHT as usize]
}

struct Static {
    x: u8,
    y: u8,
    color: VGA
}

impl Static {
    pub fn new() -> Static {
        Static {
            x: 0,
            y: 0,
            color: VGA::make(Color::Default, Color::Default)
        }
    }

    fn newline(&mut self) {
        self.y += 1;
        self.x = 0;
    }

    fn scroll(&mut self) {
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                unsafe {
                    (&mut *(0xB8000 as *mut Buffer)).chars[(y - 1) as usize][x as usize].write((&mut *(0xB8000 as *mut Buffer)).chars[y as usize][x as usize].read());
                }
            }
        }
        let c = ScreenChar {
            ascii: b' ',
            color: self.color
        };
        for x in 0..WIDTH {
            unsafe {
                (&mut *(0xB8000 as *mut Buffer)).chars[(HEIGHT - 1) as usize][x as usize].write(c);
            }
        }
        self.x = 0;
        self.y -= 1;
    }
}

impl fmt::Write for Static {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut result;
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' | b'\r' => result = self.write_char(byte as char),
                _ => result = self.write_char(0x7E as char)
            }
            match result {
                Err(_) => return result,
                _ => { }
            }
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let byte = c as u8;
        match byte {
            b'\n' => self.newline(),
            b'\r' => {
                unsafe {
                    (&mut *(0xB8000 as *mut Buffer)).chars[self.y as usize][self.x as usize].write(ScreenChar {
                        ascii: b' ',
                        color: self.color
                    });
                };
                self.x -= 1;
            },
            byte => {
                unsafe {
                    (&mut *(0xB8000 as *mut Buffer)).chars[self.y as usize][self.x as usize].write(ScreenChar {
                        ascii: byte,
                        color: self.color
                    });
                };
                self.x += 1;
                if self.x == WIDTH {
                    self.newline();
                }
            }
        }
        if self.y == HEIGHT {
            self.scroll();
        }
        Ok(())
    }
}

/****************************************************************/
//                           Macros                             //
/****************************************************************/

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/****************************************************************/
//                           Statics                            //
/****************************************************************/

lazy_static! {
    static ref TTY: Mutex <Static> = Mutex::new(Static::new());
}

/****************************************************************/
//                     Other functions                          //
/****************************************************************/

#[allow(dead_code)]
pub fn set_color(color: VGA) {
    TTY.lock().color = color;
}

#[allow(dead_code)]
pub fn get_color() -> VGA {
    TTY.lock().color
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    interrupts::without_interrupts(|| {
        TTY.lock().write_fmt(args).unwrap();
    });
}
