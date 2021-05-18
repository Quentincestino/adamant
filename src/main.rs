#![no_std]
#![no_main]
#![feature(llvm_asm, asm, abi_x86_interrupt)]
#![allow(unconditional_panic)]

mod arch;
mod header;
mod log;
mod serial;

use core::panic::PanicInfo;
use log::*;

#[no_mangle]
extern "C" fn entry_point(_header_addr: usize) -> ! {
    ok("Salut ça gaze ?");
    arch::x86::arch_init();
    unsafe {
        asm!("int 0");
        info("ECHO");
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
