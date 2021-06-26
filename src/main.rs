#![no_std]
#![no_main]

#[allow(unused_imports)]
use os::{print, println, debug};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel);

fn kernel(boot_info: &'static BootInfo) -> ! {
    os::init(boot_info);

    debug! {
        for i in (0..5) {
            println!("{}", i);
        }
    };

    // let mut hasher = os::hash::polynomial::Hasher::new(os::hash::polynomial::P_ENGLISH_BOTH);
    // hasher.write_str("xedni");
    // println!("{}", hasher.finish());

    // let p4 = os::page::get_4th_page_table(x86_64::VirtAddr::new(boot_info.physical_memory_offset));
    // let p3 = os::page::get_next_page_table(&mut p4.entries[0], boot_info.physical_memory_offset).unwrap();
    // let p2 = os::page::get_next_page_table(&mut p3.entries[0], boot_info.physical_memory_offset).unwrap();
    // let p1 = os::page::get_next_page_table(&mut p2.entries[0], boot_info.physical_memory_offset).unwrap();
    // println!("{}", p3.entries[1]);

    // debug! {
    //     for i in [0, 2, 71].iter() => {
    //         println!("{}", i);
    //     }
    // };

    // let mut s = os::alloc::string::String::new();
    // keyboard::readline(&mut s);
    // println!("{}", s);

    // let i4 = &mut page::get_4th_page_table(x86_64::VirtAddr::new(boot_info.physical_memory_offset)).entries[3];
    // let i3 = &mut page::get_next_page_table(i4, boot_info.physical_memory_offset).entries[0];
    // let i2 = &mut page::get_next_page_table(i3, boot_info.physical_memory_offset).entries[0];
    // let i1 = &mut page::get_next_page_table(i2, boot_info.physical_memory_offset).entries[164];
    //
    // println!("i4 = {}\n i3 = {}\n  i2 = {}\n   i1 = {}", i4, i3, i2, i1);

    os::exit();
}
