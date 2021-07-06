#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(arbitrary_enum_discriminant)]
#![feature(asm)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(in_band_lifetimes)]
#![feature(const_raw_ptr_deref)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_fn_trait_bound)]
#![no_std]

#[cfg(not(feature = "tty"))]
compile_error!("Disabling of feature `tty` is not supported yet");

#[cfg(feature = "tty")]
pub mod tty;

pub mod idt;

pub mod gdt;

#[cfg(feature = "pci")]
pub mod pci;

#[cfg(all(feature = "oll", feature = "hash", feature = "keyboard"))]
pub mod oll;

#[cfg(all(feature = "hdd", feature = "allocator", feature = "enum", feature = "time"))]
pub mod hdd;

#[cfg(all(feature = "page", feature = "enum"))]
pub mod page;

#[cfg(feature = "time")]
pub mod time;

#[cfg(all(feature = "hash", feature = "allocator"))]
pub mod hash;

#[cfg(feature = "enum")]
pub mod r#enum;

#[cfg(all(feature = "keyboard", feature = "allocator"))]
pub mod keyboard;

#[cfg(feature = "allocator")]
pub mod allocator;

#[cfg(feature = "allocator")]
pub extern crate alloc;

pub fn init(boot_info: &'static bootloader::BootInfo) {
    gdt::init();

    idt::init();
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    #[cfg(feature = "allocator")] {
        let mut mapper = unsafe { allocator::frame::init(x86_64::VirtAddr::new(boot_info.physical_memory_offset)) };
        let mut frame_allocator = unsafe { allocator::frame::BootInfoFrameAllocator::new(&boot_info.memory_map) };
        allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
    }

    #[cfg(all(feature = "page", feature = "enum"))]
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

    #[cfg(all(feature = "oll", feature = "hash", feature = "keyboard"))]
    if crate::oll::is_debug_mode_on() {
        unsafe { crate::oll::USING &= 0xFD }
        crate::oll::take("panic!");
    }
    exit()
}
