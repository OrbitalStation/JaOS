use core::{
    fmt::Debug,
    convert::From
};
use x86_64::instructions::hlt;

pub const ZERO: Time = Time::new();

static mut TIMER: u64 = 0;

pub type Seconds = f32;
pub type Milliseconds = u32;
pub type Microseconds = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time(Microseconds);

impl Time {
    pub const fn new() -> Self {
        Time { 0: 0 }
    }

    pub const fn seconds(seconds: Seconds) -> Self {
        Time { 0: (1000000f32 * seconds) as Microseconds }
    }

    pub const fn milliseconds(milliseconds: Milliseconds) -> Self {
        Time { 0: milliseconds as Microseconds * 1000 }
    }

    pub const fn microseconds(microseconds: Microseconds) -> Self {
        Time { 0: microseconds }
    }
}

impl From <Seconds> for Time {
    fn from(seconds: Seconds) -> Self {
        Self::seconds(seconds)
    }
}

impl From <Time> for Seconds {
    fn from(time: Time) -> Self {
        (time.0 as Self) / 1000000f32
    }
}

impl From <Milliseconds> for Time {
    fn from(milliseconds: Milliseconds) -> Self {
        Self::milliseconds(milliseconds)
    }
}

impl From <Time> for Milliseconds {
    fn from(time: Time) -> Self {
        (time.0 / 1000) as Self
    }
}

impl From <Microseconds> for Time {
    fn from(microseconds: Microseconds) -> Self {
        Self::microseconds(microseconds)
    }
}

impl From <Time> for Microseconds {
    fn from(time: Time) -> Self {
        time.0
    }
}

pub fn timer() -> u64 {
    unsafe { TIMER }
}

#[allow(arithmetic_overflow)]
pub fn sleep(time: Time) {
    unsafe {
        let wait = TIMER + (Seconds::from(time) * 18.) as u64;
        while TIMER < wait { hlt() }
    }
}

#[allow(arithmetic_overflow)]
pub fn timer_isr() {
    unsafe { TIMER += 1 }
}
