/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::instructions::{segmentation::set_cs, tables::load_tss};
use lazy_static::lazy_static;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/****************************************************************/
//                           Types                              //
/****************************************************************/

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
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
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
