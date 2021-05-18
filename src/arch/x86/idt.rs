use crate::log::*;

#[repr(packed)]
pub struct IdtPointer {
    len: u16,
    addr: u64,
}

#[derive(Copy, Clone, Default)]
#[repr(packed)]
pub struct IdtDescriptor {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: IdtAttributes,
    offset_middle: u16,
    offset_high: u32,
    zero_last: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct InterruptStackFrame {
    pub rip: u64,
    pub cs: u64,
    pub cflags: u64,
    pub rsp: usize,
    pub ss: u64,
}

type IsrFunction = unsafe extern "x86-interrupt" fn(&InterruptStackFrame);

impl IdtDescriptor {
    pub fn new(function: IsrFunction, selector: u16, attr: IdtAttributes) -> Self {
        let function = function as u64;
        Self {
            offset_low: function as u16,
            selector: selector,
            ist: 0,
            type_attr: attr,
            offset_middle: (function >> 16) as u16,
            offset_high: (function >> 32) as u32,
            zero_last: 0,
        }
    }

    // Used for a no-exception ISR
    pub fn gate32(function: IsrFunction) -> Self {
        Self::new(
            function,
            KERNEL_SEGMENT,
            IdtAttributes::new(GateType::Gate32, DPL::Ring0),
        )
    }

    pub fn trap32(function: IsrFunction) -> Self {
        Self::new(
            function,
            KERNEL_SEGMENT,
            IdtAttributes::new(GateType::Trap32, DPL::Ring0),
        )
    }

    pub const fn zeroed() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: IdtAttributes(0),
            offset_middle: 0,
            offset_high: 0,
            zero_last: 0,
        }
    }
}

#[repr(u8)]
pub enum GateType {
    Trap16 = 0b00000111, // 7
    Gate16 = 0b00000110, // 6
    Gate32 = 0b00001110, // 14
    Trap32 = 0b00001111, // 15
}

#[repr(u8)]
pub enum DPL {
    Ring0 = 0b00000000,
    Ring1 = 0b00100000,
    Ring2 = 0b01000000,
    Ring3 = 0b01100000,
}

#[derive(Copy, Clone, Default)]
pub struct IdtAttributes(u8);

const IDT_PRESENT: u8 = 0b1000_0000;

impl IdtAttributes {
    pub const fn new(gate_type: GateType, dpl: DPL) -> Self {
        IdtAttributes(gate_type as u8 | dpl as u8 | IDT_PRESENT)
    }
    pub const fn zero() -> Self {
        IdtAttributes(0)
    }
}

#[link_name = "x86_64_arch"]
extern "C" {
    fn disable_interrupts();
    fn enable_interrupts();
    fn load_idt(idt: *const IdtPointer);
}

const MAX_ENTRIES: usize = 21;
const KERNEL_SEGMENT: u16 = 0x8;

static mut IDT_POINTER: IdtPointer = IdtPointer { len: 0, addr: 0 };
static mut IDT: [IdtDescriptor; MAX_ENTRIES] = [IdtDescriptor::zeroed(); MAX_ENTRIES];

use super::interrupts::*;

pub fn idt_init() {
    unsafe {
        disable_interrupts();

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
            len: (core::mem::size_of::<[IdtDescriptor; MAX_ENTRIES]>() - 1) as u16,
            addr: (&IDT as *const _) as u64,
        };

        load_idt(&IDT_POINTER as *const _);
        // ...
        enable_interrupts();
        info("IDT Loaded without triple fault UwU");
    }
}
