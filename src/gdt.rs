/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::{
    VirtAddr,
    structures::{
        tss::TaskStateSegment,
        gdt::{GlobalDescriptorTable as GDTx64, Descriptor, SegmentSelector}
    },
    instructions::{segmentation::set_cs, tables::load_tss}
};
use lazy_static::lazy_static;
use core::mem::size_of;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/****************************************************************/
//                           Types                              //
/****************************************************************/

#[allow(dead_code)]
pub struct GDT32Entry {
    limit_l:           u16, //< limit bits 0..15
    base_l:            u16, //< base bits 0..15
    base_m:            u8, //< bas bits 16..23
    access:            u8, //< access u8
    flags_and_limit_h: u8, //< flags -- 4..7, limit 16..20 -- 0..3
    base_h:            u8, //< base 24..32
}

impl GDT32Entry {

    #[inline(always)]
    pub const fn null() -> Self {
        const NULL: GDT32Entry = GDT32Entry {
            limit_l:           0,
            base_l:            0,
            base_m:            0,
            access:            0,
            flags_and_limit_h: 0,
            base_h:            0
        };
        NULL
    }

    pub const fn new(limit: u32, base: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_l: (limit as u16) & 0xFFFF,
            base_l: (base & 0xFFFF) as u16,
            base_m: ((base >> 16) & 0xFF) as u8,
            access,
            flags_and_limit_h: (flags >> 4) | (limit >> 16) as u8,
            base_h: (base >> 24) as u8
        }
    }
}

#[allow(dead_code)]
pub struct GlobalDescriptorTable32 {
    null: GDT32Entry, //< always null
    code: GDT32Entry,
    data: GDT32Entry
}

impl GlobalDescriptorTable32 {
    pub const fn new(code: GDT32Entry, data: GDT32Entry) -> Self {
        Self {
            null: GDT32Entry::null(),
            code,
            data
        }
    }

    pub const fn pointer(&self) -> GlobalDescriptorTable32Pointer {
        GlobalDescriptorTable32Pointer::new(&self)
    }

    pub fn load(&self) {
        unsafe { asm!("lgdt [{}]", in(reg) &self.pointer(), options(readonly, nostack, preserves_flags)) }
    }
}

#[allow(dead_code)]
pub struct GlobalDescriptorTable32Pointer {
    size:    u32,
    address: u32
}

impl GlobalDescriptorTable32Pointer {
    pub const fn new(gdt: &GlobalDescriptorTable32) -> Self {
        Self {
            size: size_of::<GlobalDescriptorTable32>() as u32,
            address: unsafe { gdt as *const GlobalDescriptorTable32 as u32 }
        }
    }
}

struct Selectors {
    code: SegmentSelector,
    tss : SegmentSelector
}

/****************************************************************/
//                           Statics                            //
/****************************************************************/

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GDTx64, Selectors) = {
        let mut gdt = GDTx64::new();
        let code = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss  = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code, tss })
    };
}

/****************************************************************/
//                     Other functions                          //
/****************************************************************/

pub fn init() {
    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code);
        load_tss(GDT.1.tss);
    }
}
