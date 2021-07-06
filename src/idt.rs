/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::{
    structures::idt::{
        InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode
    },
    registers::control::Cr2
};
use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;
use crate::{
    gdt,
};

#[cfg(all(feature = "keyboard", feature = "allocator"))]
use crate::keyboard::keyboard_isr;

#[cfg(feature = "time")]
use crate::time::timer_isr;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const PIC1: u8 = 0x20;
pub const PIC2: u8 = PIC1 + 8;

/****************************************************************/
//                            Types                             //
/****************************************************************/

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1,
    Keyboard
}

/****************************************************************/
//                           Macros                             //
/****************************************************************/

#[macro_export]
macro_rules! irq_end {
    ($index:expr) => { unsafe { PICS.lock().notify_end_of_interrupt($index as u8) } };
}

/****************************************************************/
//                           Statics                            //
/****************************************************************/

pub static PICS: spin::Mutex <ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC1, PIC2) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(divide0);

        idt.debug.set_handler_fn(debug);

        idt.non_maskable_interrupt.set_handler_fn(nmi);

        idt.breakpoint.set_handler_fn(breakpoint);

        idt.overflow.set_handler_fn(overflow);

        idt.bound_range_exceeded.set_handler_fn(bound);

        idt.invalid_opcode.set_handler_fn(opcode);

        idt.device_not_available.set_handler_fn(device);

        unsafe { idt.double_fault.set_handler_fn(double).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); }

        idt.invalid_tss.set_handler_fn(tss);

        idt.segment_not_present.set_handler_fn(segment);

        idt.stack_segment_fault.set_handler_fn(stack);

        idt.general_protection_fault.set_handler_fn(protection);

        idt.page_fault.set_handler_fn(page);

        idt.x87_floating_point.set_handler_fn(x87);

        idt.alignment_check.set_handler_fn(alignment);

        idt.machine_check.set_handler_fn(machine);

        idt.simd_floating_point.set_handler_fn(simd);

        idt.virtualization.set_handler_fn(virtualization);

        idt.security_exception.set_handler_fn(security);

        /* IRQs */

        idt[InterruptIndex::Timer as usize].set_handler_fn(timer);

        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard);

        /* Other */

        idt
    };
}

/****************************************************************/
//                     Other functions                          //
/****************************************************************/

pub fn init() {
    IDT.load();
}

/****************************************************************/
//                            IRQs                              //
/****************************************************************/

extern "x86-interrupt" fn keyboard(_isf: InterruptStackFrame) {
    #[cfg(all(feature = "keyboard", feature = "allocator"))]
    keyboard_isr();

    irq_end!(InterruptIndex::Keyboard);
}


extern "x86-interrupt" fn timer(_isf: InterruptStackFrame) {
    #[cfg(feature = "time")]
    timer_isr();

    irq_end!(InterruptIndex::Timer);
}

/****************************************************************/
//                            ISRs                              //
/****************************************************************/

/****************************************************************/
//                           basic                              //
/****************************************************************/

extern "x86-interrupt" fn divide0(_isf: InterruptStackFrame) {
    panic!("Divide by zero occurred!")
}

extern "x86-interrupt" fn debug(_isf: InterruptStackFrame) {
    panic!("Int 0x1(Debug) occurred!")
}

extern "x86-interrupt" fn nmi(_isf: InterruptStackFrame) {
    panic!("Int 0x2(NMI) occurred!")
}

extern "x86-interrupt" fn breakpoint(_isf: InterruptStackFrame) {
    panic!("Int 0x3(Breakpoint) occurred!")
}

extern "x86-interrupt" fn overflow(_isf: InterruptStackFrame) {
    panic!("Overflow occurred!")
}

extern "x86-interrupt" fn bound(_isf: InterruptStackFrame) {
    panic!("Int 0x5(Bound) occurred!")
}

extern "x86-interrupt" fn opcode(_isf: InterruptStackFrame) {
    panic!("Int 0x6(Opcode) occurred!")
}

extern "x86-interrupt" fn device(_isf: InterruptStackFrame) {
    panic!("Int 0x7(Device) occurred!")
}

extern "x86-interrupt" fn double(isf: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Exception: Double Fault\nCode = {}\n{:#?}", error_code, isf);
}

extern "x86-interrupt" fn tss(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0xA(TSS) occurred!")
}

extern "x86-interrupt" fn segment(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0xB(Segment) occurred!")
}

extern "x86-interrupt" fn stack(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0xC(Stack) occurred!")
}

extern "x86-interrupt" fn protection(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0xD(Protection) occurred!")
}

extern "x86-interrupt" fn page(isf: InterruptStackFrame, code: PageFaultErrorCode) {
    panic!("Exception: Page Fault\nAccessed address: {:?}\nError code: {:?}\n{:#?}", Cr2::read(), code, isf);
}

extern "x86-interrupt" fn x87(_isf: InterruptStackFrame) {
    panic!("Int 0x10(x87) occurred!")
}

extern "x86-interrupt" fn alignment(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0x11(Alignment) occurred!")
}

extern "x86-interrupt" fn machine(isf: InterruptStackFrame) -> ! {
    panic!("Interrupt 0x12(Machine check, #MC) occurred.\n{:#?}.\nAborting.", isf);
}

extern "x86-interrupt" fn simd(_isf: InterruptStackFrame) {
    panic!("Int 0x13(SIMD) occurred!")
}

extern "x86-interrupt" fn virtualization(_isf: InterruptStackFrame) {
    panic!("Int 0x14 occurred!")
}

extern "x86-interrupt" fn security(_isf: InterruptStackFrame, _code: u64) {
    panic!("Int 0x1E(Security) occurred!")
}

/****************************************************************/
//                           other                              //
/****************************************************************/
