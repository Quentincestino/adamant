#![no_std]
#![no_main]
#![feature(llvm_asm, asm, abi_x86_interrupt)]
#![allow(improper_ctypes, unused_attributes, unused_assignments, dead_code)]

mod arch;
mod header;
mod log;
mod serial;

use arch::*;
use core::panic::PanicInfo;
use log::*;

#[no_mangle]
extern "C" fn entry_point(_header_addr: usize) -> ! {
    x86::arch_init();

    unsafe {
        x86::bochs_magic_breakpoint();
        warn("Calling int 0");
        asm!("int 0");

        info("Post Interrupt");
    }
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
