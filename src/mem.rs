/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::{
    VirtAddr, PhysAddr,
    structures::paging::{
        PageTable, OffsetPageTable, PhysFrame, Size4KiB, FrameAllocator
    },
    registers::control::Cr3
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

/****************************************************************/
//                            Types                             //
/****************************************************************/

pub struct BootInfoFrameAllocator {
    mmap: &'static MemoryMap,
    next: usize
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            mmap: memory_map,
            next: 0
        }
    }

    fn usable_frames(&self) -> impl Iterator <Item = PhysFrame> {
        self.mmap.iter().filter(|r| r.region_type == MemoryRegionType::Usable).map(|r| r.range.start_addr()..r.range.end_addr()).flat_map(|r| r.step_by(4096)).map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator <Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option <PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/****************************************************************/
//                     Other functions                          //
/****************************************************************/

pub unsafe fn init(offset: VirtAddr) -> OffsetPageTable <'static> {
    OffsetPageTable::new(get_active_p4(offset), offset)
}

unsafe fn get_active_p4(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (p4, _) = Cr3::read();
    &mut *((phys_offset + p4.start_address().as_u64()).as_mut_ptr())
}
