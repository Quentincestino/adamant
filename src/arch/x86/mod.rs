mod gdt;
mod idt;
mod interrupts;

pub fn arch_init() {
    gdt::gdt_init();
    idt::idt_init();
}
