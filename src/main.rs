#![no_std]
#![no_main]
#![feature(llvm_asm, asm, abi_x86_interrupt)]
#![allow(improper_ctypes, unused_attributes, unused_assignments, dead_code)]

mod allocator;
mod arch;
mod header;
mod log;
mod serial;

use arch::*;
use core::panic::PanicInfo;
use log::*;

#[no_mangle]
extern "C" fn entry_point(stivale2_struct: usize) -> ! {
    x86::arch_init();
    header::init_stivale2_struct(stivale2_struct);

    ok("Welcome to Adamant !");

    loop {}
}

#[panic_handler]
fn panic(_infos: &PanicInfo) -> ! {
    error(
        "y0 nibba looks like you shitted something if you are here now",
        true,
    );
    loop {}
}
