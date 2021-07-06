#[cfg(all(feature = "page", feature = "enum"))]
mod private {
    use x86_64::{
        PhysAddr,
        structures::paging::PhysFrame
    };
    use core::fmt::{Formatter, Write, Debug};
    use crate::enum_flags;

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

}

#[cfg(all(feature = "page", feature = "enum"))]
pub use private::*;
