/****************************************************************/
//                            Uses                              //
/****************************************************************/

use spin;
use alloc::{
    vec::Vec,
    string::String
};
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use core::{
    fmt::Debug,
    convert::From
};
use crate::print;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

const KB_PORT: u16 = 0x60;

/****************************************************************/
//                            Types                             //
/****************************************************************/

pub type Scancode = u8;
pub type Argument  = *const u8;
pub type HandlerFn = fn(Scancode, Argument);

#[derive(Copy, Clone)]
pub struct Entry {
    handler:  HandlerFn,
    argument: Argument
}

unsafe impl Send for Entry { }

impl Entry {
    pub fn new(handler: HandlerFn, argument: Argument) -> Self {
        Entry { handler, argument }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Key {
    Error,
    Escape,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
	Num0,
    Minus,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    SquareLeft,  //< [
    SquareRight, //< ]
    Enter,
    ControlLeft,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Apostrophe,
    ShiftLeft,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    ShiftRight,
    PrintScreen,
    AltLeft,
    Space,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    NumLock,
    ScrollLock,
    Home,
    Up,
    PageUp,
    NumpadMinus,
    Left,
    Center,
    Right,
    NumpadPlus,
    End,
    Down,
    PageDown,
    Insert,
    Delete,

    Skip1,
    Skip2,
    Skip3,

    F11,
    F12,

    Count //< Count of all keys
}

impl Key {
    pub fn state(self) -> KeyState {
        match self {
            Key::Error => KeyState::Release,
            other => KeyState::from(unsafe { STATES.lock()[other as usize / 8] & (1 << (other as usize % 8)) } != 0)
        }
    }

    pub fn is_pressed(self) -> bool {
        self.state() == KeyState::Press
    }

    pub fn is_released(self) -> bool {
        self.state() == KeyState::Release
    }
    
    pub fn as_low_char(self) -> Option <char> {
        match self {
            Key::Num1 => Some('1'),
            Key::Num2 => Some('2'),
            Key::Num3 => Some('3'),
            Key::Num4 => Some('4'),
            Key::Num5 => Some('5'),
            Key::Num6 => Some('6'),
            Key::Num7 => Some('7'),
            Key::Num8 => Some('8'),
            Key::Num9 => Some('9'),
            Key::Num0 => Some('0'),
            Key::Minus | Key::NumpadMinus => Some('-'),
            Key::Equal | Key::NumpadPlus => Some('='),
            Key::Backspace => Some('\x08'),
            Key::Tab => Some('\t'),
            Key::Q => Some('q'),
            Key::W => Some('w'),
            Key::E => Some('e'),
            Key::R => Some('r'),
            Key::T => Some('t'),
            Key::Y => Some('y'),
            Key::U => Some('u'),
            Key::I => Some('i'),
            Key::O => Some('o'),
            Key::P => Some('p'),
            Key::SquareLeft => Some('['),
            Key::SquareRight => Some(']'),
            Key::Enter => Some('\n'),
            Key::A => Some('a'),
            Key::S => Some('s'),
            Key::D => Some('d'),
            Key::F => Some('f'),
            Key::G => Some('g'),
            Key::H => Some('h'),
            Key::J => Some('j'),
            Key::K => Some('k'),
            Key::L => Some('l'),
            Key::Semicolon => Some(';'),
            Key::Quote => Some('\''),
            Key::Apostrophe => Some('`'),
            Key::Backslash => Some('\\'),
            Key::Z => Some('z'),
            Key::X => Some('x'),
            Key::C => Some('c'),
            Key::V => Some('v'),
            Key::B => Some('b'),
            Key::N => Some('n'),
            Key::M => Some('m'),
            Key::Comma => Some(','),
            Key::Dot => Some('.'),
            Key::Slash => Some('/'),
            Key::Space => Some(' '),
            _ => None
        }
    }

    pub fn as_high_char(self) -> Option <char> {
        match self {
            Key::Num1 => Some('!'),
            Key::Num2 => Some('@'),
            Key::Num3 => Some('#'),
            Key::Num4 => Some('$'),
            Key::Num5 => Some('%'),
            Key::Num6 => Some('^'),
            Key::Num7 => Some('&'),
            Key::Num8 => Some('*'),
            Key::Num9 => Some('('),
            Key::Num0 => Some(')'),
            Key::Minus | Key::NumpadMinus => Some('_'),
            Key::Equal | Key::NumpadPlus => Some('+'),
            Key::Backspace => Some('\x08'),
            Key::Tab => Some('\t'),
            Key::Q => Some('Q'),
            Key::W => Some('W'),
            Key::E => Some('E'),
            Key::R => Some('R'),
            Key::T => Some('T'),
            Key::Y => Some('Y'),
            Key::U => Some('U'),
            Key::I => Some('I'),
            Key::O => Some('O'),
            Key::P => Some('P'),
            Key::SquareLeft => Some('{'),
            Key::SquareRight => Some('}'),
            Key::Enter => Some('\n'),
            Key::A => Some('A'),
            Key::S => Some('S'),
            Key::D => Some('D'),
            Key::F => Some('F'),
            Key::G => Some('G'),
            Key::H => Some('H'),
            Key::J => Some('J'),
            Key::K => Some('K'),
            Key::L => Some('L'),
            Key::Semicolon => Some(':'),
            Key::Quote => Some('"'),
            Key::Apostrophe => Some('~'),
            Key::Backslash => Some('|'),
            Key::Z => Some('Z'),
            Key::X => Some('X'),
            Key::C => Some('C'),
            Key::V => Some('V'),
            Key::B => Some('B'),
            Key::N => Some('N'),
            Key::M => Some('M'),
            Key::Comma => Some('<'),
            Key::Dot => Some('>'),
            Key::Slash => Some('?'),
            Key::Space => Some(' '),
            _ => None
        }
    }

    pub fn as_char(self) -> Option <char> {
        if char_up() { self.as_high_char() } else { self.as_low_char() }
    }
}

impl From <Scancode> for Key {
    fn from(mut x: Scancode) -> Self {
        x &= 0x7F;
        unsafe { *(&x as *const Scancode as *const Self) }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum KeyState {
    Release,
    Press
}

impl KeyState {
    pub fn is_pressed(self) -> bool {
        match self {
            Self::Release => false,
            Self::Press => true
        }
    }

    pub fn is_released(self) -> bool {
        match self {
            Self::Release => true,
            Self::Press => false
        }
    }
}

impl From <Scancode> for KeyState {
    fn from(x: Scancode) -> Self {
        if x & 0x80 == 0 { Self::Press } else { Self::Release }
    }
}

impl From <bool> for KeyState {
    fn from(x: bool) -> Self {
        if x { Self::Press } else { Self::Release }
    }
}

/****************************************************************/
//                           Statics                            //
/****************************************************************/

lazy_static! {
    static ref HANDLERS: spin::Mutex <Vec <Entry>> = spin::Mutex::new(Vec::new());
}

static mut STATES: spin::Mutex <[u8; Key::Count as usize / 8]> = spin::Mutex::new([0; Key::Count as usize / 8]);

/****************************************************************/
//                     Other functions                          //
/****************************************************************/

pub fn keyboard_isr() {
    let mut port = Port::new(KB_PORT);
    let scancode: Scancode = unsafe { port.read() };
    let key = Key::from(scancode);
    let state = KeyState::from(scancode);
    if key == Key::CapsLock && state.is_pressed() {
        unsafe {
            if caps() {
                STATES.lock()[0] &= 0xFE;
            } else {
                STATES.lock()[0] |= 1;
            }
        }
    }
    let byte = key as usize / 8;
    let bit  = key as usize % 8;
    unsafe {
        match state {
            KeyState::Release => STATES.lock()[byte] &= !(1 << bit),
            KeyState::Press => STATES.lock()[byte] |= 1 << bit
        }
    }
    for entry in HANDLERS.lock().iter() {
        (entry.handler)(scancode, entry.argument);
    }
}

pub fn register_handler(handler: HandlerFn, argument: Argument) {
    HANDLERS.lock().push(Entry::new(handler, argument))
}

pub fn change_handler_arg(id: usize, argument: Argument) {
    HANDLERS.lock()[id].argument = argument;
}

pub fn get_handler_arg(id: usize) -> Argument {
    HANDLERS.lock()[id].argument
}

pub fn pop_handler() -> Option <Entry> {
    HANDLERS.lock().pop()
}

pub fn caps() -> bool {
    unsafe { STATES.lock()[0] & 1 != 0 }
}

pub fn shift() -> bool {
    Key::ShiftLeft.is_pressed() || Key::ShiftRight.is_pressed()
}

pub fn char_up() -> bool {
    caps() != shift()
}

pub fn read_until(s: &mut String, delim: char) {
    s.clear();
    register_handler(|scancode: Scancode, s: Argument| unsafe {
        if KeyState::from(scancode).is_released() { return }
        match Key::from(scancode).as_char() {
            None => return,
            Some(symbol) => {
                print!("{}", symbol);
                let s = &mut *(s as *mut u8 as *mut String);
                match symbol {
                    '\x08' => { s.pop(); },
                    symbol => s.push(symbol)
                }
            }
        }
    }, s as *mut String as Argument);
    while !s.ends_with(delim) { x86_64::instructions::hlt() }
    pop_handler();
}

#[inline(always)]
pub fn readline(s: &mut String) {
    read_until(s, '\n')
}
