global enable_interrupts
enable_interrupts:
    sti

global disable_interrupts
disable_interrupts:
    cli

global load_idt
load_idt:
    lidt [rdi]