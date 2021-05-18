mod gdt;
mod idt;
mod interrupts;

use gdt::*;

pub fn arch_init() {
    let _gdt: *const [Segment; GDT_ENTRIES] = gdt::gdt_init();
    idt::idt_init();
}
