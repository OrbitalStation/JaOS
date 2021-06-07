#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![no_std]

pub mod tty;
pub mod idt;
pub mod gdt;
pub mod mem;
pub mod pci;
pub mod allocator;

extern crate alloc;

pub fn init() {
    gdt::init();

    idt::init();
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn absolute_end() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Black));
    println!("{}", info);
    absolute_end();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
