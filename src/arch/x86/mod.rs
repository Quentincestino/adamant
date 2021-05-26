mod gdt;
mod idt;
mod interrupts;

use self::{gdt::gdt_init, idt::idt_init};

pub fn arch_init() {
    gdt_init();
    idt_init();
}

#[inline(always)]
pub fn bochs_magic_breakpoint() {
    unsafe {
        asm!("xchg bx, bx");
    }
}

#[inline(always)]
pub fn painless_halt() {
    unsafe {
        crate::warn!("Entered on a painless halt, something went wrong ?");

        asm!("cli");
        asm!("hlt");
    }
}
