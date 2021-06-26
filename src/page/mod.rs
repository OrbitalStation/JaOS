use x86_64::{
    structures::paging::PhysFrame,
    registers::control::Cr3,
    VirtAddr, PhysAddr
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use core::fmt::{Formatter, Write, Debug};
use crate::{enum_flags};

pub const PAGE_SIZE: usize = 4096;
pub const ENTRY_COUNT: usize = 512;

enum_flags! {
    pub enum PageEntryFlags: u64 {
        Present             = 0,  //< Is entry present?
        Writable            = 1,  //< Is page that entry refers to writable?
        UserAccessible      = 2,  //< Is user accessible?
        WriteThroughCaching = 3,  //< Which policy of caching to use?
        DisableCache        = 4,  //< Use cache or not?
        Accessed            = 5,  //< Was the page accessed to(Automatically set by the CPU)?
        Dirty               = 6,  //< Did write occur to the page(Automatically set by the CPU)?
        HugePage            = 7,  //< Use huge pages or not(Only in P2 and P3, in P1 and P4 must be 0)?
        Global              = 8,  //< Is page provided for all address spaces?
        Free                = 9,  //< Is page free for usage by OS(OS-specific)?
        NoExecute           = 63, //< Can code on page be executed?
    } of #[repr(u64)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameError {
    FrameNotPresent,
    HugeFrame
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PageEntryAddress(u64);

impl ::core::convert::From <u64> for PageEntryAddress {
    #[inline]
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl ::core::convert::From <PageEntryAddress> for u64 {
    #[inline]
    fn from(x: PageEntryAddress) -> Self {
        x.0
    }
}

impl PageEntryAddress {
    pub const fn from_mut(x: &'a mut u64) -> &'a mut Self {
        unsafe { &mut *(x as *mut u64 as *mut Self) }
    }

    pub const fn get(self) -> u64 {
        self.0 & 0x000F_FFFF_FFFF_F000
    }

    pub const fn phys(self) -> PhysAddr {
        PhysAddr::new_truncate(self.0)
    }

    pub fn set(&mut self, address: u64) {
        self.0 = (self.0 & (PageEntryFlags::All as u64)) | address
    }

    #[inline]
    pub const fn from(x: u64) -> Self {
        Self(x)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PageEntry(u64);

impl PageEntry {
    #[inline]
    pub const fn empty() -> Self {
        Self { 0: 0 }
    }

    #[inline]
    pub const fn is_unused(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn set_unused(&mut self) {
        self.0 = 0
    }

    #[inline]
    pub const fn flags(&mut self) -> &mut PageEntryFlags {
        PageEntryFlags::from_mut(&mut self.0)
    }

    #[inline]
    pub const fn cflags(self) -> PageEntryFlags {
        PageEntryFlags::from(self.0)
    }

    #[inline]
    pub const fn address(&mut self) -> &mut PageEntryAddress {
        PageEntryAddress::from_mut(&mut self.0)
    }

    #[inline]
    pub const fn caddress(self) -> PageEntryAddress {
        PageEntryAddress::from(self.0)
    }

    #[inline]
    pub fn frame(&mut self) -> Result <PhysFrame, FrameError> {
        if !self.flags().contain(PageEntryFlags::Present) {
            Err(FrameError::FrameNotPresent)
        } else if self.flags().contain(PageEntryFlags::HugePage) {
            Err(FrameError::HugeFrame)
        } else {
            Ok(PhysFrame::containing_address(self.address().phys()))
        }
    }
}

impl core::fmt::Display for PageEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("PageEntry(flags = ")?;
        self.cflags().fmt(f)?;
        f.write_str(", address = 0x")?;
        core::fmt::UpperHex::fmt(&self.caddress().get(), f)?;
        f.write_char(')')
    }
}

#[repr(align(4096))]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PageTable {
    pub entries: [PageEntry; ENTRY_COUNT]
}

impl PageTable {
    pub fn new() -> Self {
        const EMPTY: PageEntry = PageEntry::empty();
        PageTable { entries: [EMPTY; ENTRY_COUNT] }
    }
}

#[repr(align(4096))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union Page {
    bytes: [u8;  PAGE_SIZE],
    words: [u16; PAGE_SIZE / 2],
    longs: [u32; PAGE_SIZE / 4],
    quads: [u64; PAGE_SIZE / 8]
}

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

pub fn get_table_by_page_number(offset: u64, n: u64) -> Option <&'static mut PageTable> {
    let p2 = (n as usize / ENTRY_COUNT) % ENTRY_COUNT;
    let p3 = (n as usize / (ENTRY_COUNT * ENTRY_COUNT)) % ENTRY_COUNT;
    let p4 = (n as usize / (ENTRY_COUNT * ENTRY_COUNT * ENTRY_COUNT)) % ENTRY_COUNT;
    get_next_page_table(&mut get_next_page_table(&mut get_next_page_table(&mut get_4th_page_table(VirtAddr::new(offset)).entries[p4], offset)?.entries[p3], offset)?.entries[p2], offset)
}

pub fn get_4th_page_table(offset: VirtAddr) -> &'static mut PageTable {
    let (p4, _) = Cr3::read_raw();
    unsafe { &mut *((offset + p4.start_address().as_u64()).as_mut_ptr()) }
}

pub fn get_next_page_table(entry: &mut PageEntry, offset: u64) -> Option <&'static mut PageTable> {
    // if m.start_address().as_u64() == 0xff53f000f000 {
    //     print!("{:?}\n", offset);
    // }
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
