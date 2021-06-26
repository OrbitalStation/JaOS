#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(arbitrary_enum_discriminant)]
#![feature(global_asm)]
#![feature(asm)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(in_band_lifetimes)]
#![feature(const_raw_ptr_deref)]
#![feature(const_fn_trait_bound)]
#![no_std]

pub mod tty;
pub mod idt;
pub mod gdt;
pub mod mem;
pub mod pci;
pub mod oll;
pub mod page;
pub mod time;
pub mod hash;
pub mod r#enum;
pub mod keyboard;
pub mod allocator;

pub extern crate alloc;

pub fn init(boot_info: &'static bootloader::BootInfo) {
    gdt::init();

    idt::init();
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut mapper = unsafe { mem::init(x86_64::VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { mem::BootInfoFrameAllocator::new(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    page::mark_p4_based_on_ram(boot_info.physical_memory_offset, &boot_info.memory_map);
}

pub fn exit() -> ! {
    tty::set_color(tty::VGA::make(tty::Color::Blue, tty::Color::Default));
    println!("Finishing...");
    x86_64::instructions::interrupts::disable();
    loop { x86_64::instructions::hlt() }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    tty::set_color(tty::VGA::make(tty::Color::LightRed, tty::Color::Black));
    println!("{}", info);
    if crate::oll::is_debug_mode_on() {
        unsafe { crate::oll::USING &= 0xFD }
        crate::oll::take("panic!");
    }
    exit()
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
