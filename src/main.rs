#![no_std]
#![no_main]

#[allow(unused_imports)]
use os::{print, println};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel);

fn kernel(boot_info: &'static BootInfo) -> ! {
    os::init(boot_info);

    let mut sum = 0;
    for frame in boot_info.memory_map.iter() {
        if frame.region_type == bootloader::bootinfo::MemoryRegionType::Kernel {
            sum += frame.range.end_addr() - frame.range.start_addr()
        }
    }

    println!("total: {}", sum);

    //println!("{}", bootloader_locator)

    // unsafe {
    //     //os::pci::scan();
    //     os::hdd::scan()
    // }
    //
    // let arr = [27u8; 1024];
    //
    // //println!("{:?}", arr);
    //
    // unsafe {
    //     println!("{:?}", os::hdd::ata_access(os::hdd::Direction::Read, 3, 0x1000, 1, arr.as_slice()));
    // }

    //println!("{:?}", arr);

    os::exit();
}
