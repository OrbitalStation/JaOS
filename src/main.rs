#![no_std]
#![no_main]

#[allow(unused_imports)]
use os::{print, println};

use bootloader::{BootInfo, entry_point};

extern crate alloc;

entry_point!(kernel);

fn kernel(_boot_info: &'static BootInfo) -> ! {
    os::init();

    unsafe {
        os::pci::scan();
    }

    os::absolute_end();
}
