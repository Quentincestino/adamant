#![no_std]
#![no_main]
#![feature(asm, abi_x86_interrupt, panic_info_message, lang_items)]
#![allow(improper_ctypes, unused_attributes, unused_assignments, dead_code)]

mod allocator;
mod arch;
mod log;
mod panic;
mod serial;
mod stivale2;

use arch::*;

#[no_mangle]
extern "C" fn entry_point(stivale2_struct: usize) -> ! {
    x86_64::arch_init();
    ok!("Hello world !");
    stivale2::set_stivale_addr(stivale2_struct);

    allocator::init();

    loop {
        // If we don't halt cpu just gonna die
        arch::x86_64::painless_halt();
    }
}
