#[cfg(all(feature = "page", feature = "enum"))]
mod types;

#[cfg(all(feature = "page", feature = "enum"))]
mod private {

    use x86_64::{
        structures::paging::PhysFrame,
        registers::control::Cr3,
        VirtAddr
    };
    use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

    pub use super::types::*;

    /* Marks pages in page tables based on already used(accessed) pages in RAM */
    pub fn mark_p4_based_on_ram(offset: u64, mmap: &'static MemoryMap) {
        let mut counter;
        let mut table;
        let mut entry;
        for region in mmap.iter() {
            if region.region_type != MemoryRegionType::Usable { continue }
            counter = 0;
            table = get_table_by_page_number(offset, region.range.start_frame_number).unwrap();
            entry = (region.range.start_frame_number + counter) % ENTRY_COUNT as u64;
            'outer: loop {
                while entry != ENTRY_COUNT as u64 {
                    if entry + counter == region.range.end_frame_number {
                        break 'outer
                    }
                    table.entries[entry as usize].flags().add(PageEntryFlags::Free);
                    entry += 1;
                }
                counter += entry;
                table = match get_table_by_page_number(offset, region.range.start_frame_number + counter) {
                    Some(x) => x,
                    None => break
                };
                entry = (region.range.start_frame_number + counter) % ENTRY_COUNT as u64;
            }
        }
    }

    pub fn get_table_by_page_number(offset: u64, n: u64) -> Option<&'static mut PageTable> {
        let p2 = (n as usize / ENTRY_COUNT) % ENTRY_COUNT;
        let p3 = (n as usize / (ENTRY_COUNT * ENTRY_COUNT)) % ENTRY_COUNT;
        let p4 = (n as usize / (ENTRY_COUNT * ENTRY_COUNT * ENTRY_COUNT)) % ENTRY_COUNT;
        get_next_page_table(&mut get_next_page_table(&mut get_next_page_table(&mut get_4th_page_table(VirtAddr::new(offset)).entries[p4], offset)?.entries[p3], offset)?.entries[p2], offset)
    }

    pub fn get_4th_page_table(offset: VirtAddr) -> &'static mut PageTable {
        let (p4, _) = Cr3::read_raw();
        unsafe { &mut *((offset + p4.start_address().as_u64()).as_mut_ptr()) }
    }

    pub fn get_next_page_table(entry: &mut PageEntry, offset: u64) -> Option<&'static mut PageTable> {
        unsafe {
            Some(&mut *((match VirtAddr::try_new(match entry.frame() {
                Ok(x) => x,
                Err(_) => PhysFrame::containing_address(entry.address().phys())
            }.start_address().as_u64() + offset) {
                Ok(x) => x,
                Err(_) => return None
            }).as_mut_ptr()))
        }
    }

}

#[cfg(all(feature = "page", feature = "enum"))]
pub use private::*;
