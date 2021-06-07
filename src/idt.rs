/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::{
    structures::idt::{
        InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode
    },
    instructions::port::Port,
    registers::control::Cr2
};
use pic8259::ChainedPics;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin;
use lazy_static::lazy_static;
use crate::{print, gdt, absolute_end};

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const PIC1: u8 = 0x20;
pub const PIC2: u8 = PIC1 + 8;

pub static PICS: spin::Mutex <ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC1, PIC2) });

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

macro_rules! irq_end {
    ($index:expr) => { unsafe { PICS.lock().notify_end_of_interrupt($index as u8) } };
}

/****************************************************************/
//                           Statics                            //
/****************************************************************/

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

        idt
    };
}

lazy_static! {
    static ref KEYBOARD: spin::Mutex <Keyboard <layouts::Us104Key, ScancodeSet1>> = spin::Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
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
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key)
            }
        }
    }

    irq_end!(InterruptIndex::Keyboard);
}

extern "x86-interrupt" fn timer(_isf: InterruptStackFrame) {
    print!(".");
    irq_end!(InterruptIndex::Timer);
}

/****************************************************************/
//                            ISRs                              //
/****************************************************************/

extern "x86-interrupt" fn divide0(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn debug(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn nmi(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn breakpoint(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn overflow(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn bound(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn opcode(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn device(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn double(isf: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Exception: Double Fault\nCode = {}\n{:#?}", error_code, isf);
}

extern "x86-interrupt" fn tss(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}

extern "x86-interrupt" fn segment(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}

extern "x86-interrupt" fn stack(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}

extern "x86-interrupt" fn protection(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}

extern "x86-interrupt" fn page(isf: InterruptStackFrame, code: PageFaultErrorCode) {
    print!("Exception: Page Fault\nAccessed address: {:?}\nError code: {:?}\n{:#?}", Cr2::read(), code, isf);
    absolute_end()
}

extern "x86-interrupt" fn x87(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn alignment(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}

extern "x86-interrupt" fn machine(isf: InterruptStackFrame) -> ! {
    panic!("Interrupt 0x12(Machine check, #MC) called.\n{:#?}.\nAborting.", isf);
}

extern "x86-interrupt" fn simd(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn virtualization(_isf: InterruptStackFrame) {
    // dummy
}

extern "x86-interrupt" fn security(_isf: InterruptStackFrame, _code: u64) {
    // dummy
}
