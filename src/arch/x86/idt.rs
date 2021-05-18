use crate::log::*;

#[repr(packed)]
pub struct IdtPointer {
    len: u16,
    addr: u64,
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    attributes: u8,
    offset_mid: u16,
    reserved: u8, // DO NOT TOUCH ME OWO
    offset_high: u32,
    zero: u32,
}

#[repr(u8)]
pub enum InterruptTypes {
    InterruptTrap16 = 0b0111,
    InterruptGate16 = 0b0110,
    InterruptGate32 = 0b1110,
    InterruptTrap32 = 0b1111,
}

type HandlerInterrupt = unsafe extern "x86-interrupt" fn(InterruptStackFrame);

use bitflags::bitflags;

use crate::{arch::x86::interrupts, serial};

bitflags! {
    pub struct IdtFlags: u8 {
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SS = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP = 0xF;
    }
}

impl IdtEntry {
    pub const fn null() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            attributes: 0,
            offset_mid: 0,
            reserved: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    fn set_flags(&mut self, flags: IdtFlags) {
        self.attributes = flags.bits;
    }

    fn set_offset(&mut self, selector: u16, base: usize) {
        self.selector = selector;
        self.offset_low = base as u16;
        self.offset_mid = (base >> 16) as u16;
        self.offset_high = (base >> 32) as u32;
    }

    pub fn set_function(&mut self, handler: HandlerInterrupt) {
        self.set_flags(IdtFlags::PRESENT | IdtFlags::RING_0 | IdtFlags::INTERRUPT);
        self.set_offset(8, handler as usize)
    }
}

#[repr(packed)]
pub struct InterruptStackFrame {
    pub instruction_pointer: usize,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: usize,
    pub stack_segment: u64,
}

pub unsafe fn load_idt(idtptr: *const IdtPointer) {
    asm!("lidt [{}]", in(reg) idtptr, options(nostack));
}

const IDT_ENTRIES: usize = 256;
static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::null(); IDT_ENTRIES];
static mut IDT_POINTER: IdtPointer = IdtPointer { len: 0, addr: 0 };

pub fn idt_init() {
    unsafe {
        IDT[0] = IdtDescriptor::trap32(divide_by_zero);
        IDT[1] = IdtDescriptor::trap32(debug);
        IDT[2] = IdtDescriptor::trap32(nmi);
        IDT[3] = IdtDescriptor::trap32(breakpoint);
        IDT[4] = IdtDescriptor::trap32(overflow);
        IDT[5] = IdtDescriptor::trap32(table_overflow);
        IDT[6] = IdtDescriptor::trap32(invalid_instruction);
        IDT[7] = IdtDescriptor::trap32(unavailable_device);
        IDT[8] = IdtDescriptor::trap32(double_fault);
        IDT[9] = IdtDescriptor::trap32(coproc_segment);
        IDT[10] = IdtDescriptor::trap32(invalid_tss);
        IDT[11] = IdtDescriptor::trap32(not_present_segment);
        IDT[12] = IdtDescriptor::trap32(invalid_stack_segment);
        IDT[13] = IdtDescriptor::trap32(general_protection_fault);
        IDT[14] = IdtDescriptor::trap32(page_fault);
        IDT[15] = IdtDescriptor::trap32(reserved_15); // NOT SUPPOSED TO BE HANDLED
        IDT[16] = IdtDescriptor::trap32(fpu_fault);
        IDT[17] = IdtDescriptor::trap32(align_fault);
        IDT[18] = IdtDescriptor::trap32(check_exception);
        IDT[19] = IdtDescriptor::trap32(simd_exception);
        IDT[20] = IdtDescriptor::trap32(virtualization_exception);

        IDT_POINTER = IdtPointer {
            len: (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
            addr: (&IDT as *const _) as u64,
        };

        IDT[0].set_function(interrupts::divide_by_zero);
        load_idt(&IDT_POINTER as *const _);
        asm!("sti");
        info("IDT Loaded without triple fault UwU");
    }
}
