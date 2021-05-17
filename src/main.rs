#![no_std]
#![no_main]
#![feature(llvm_asm, asm, abi_x86_interrupt)]
#![allow(unconditional_panic)]

mod arch;
mod header;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn entry_point(_header_addr: usize) -> ! {
    serial::serial_print("Salut ça gaze ?");
    arch::x86::arch_init();
    unsafe {
        asm!("int 0");
    }
    loop {}
}

#[panic_handler]
fn panic(_infos: &PanicInfo) -> ! {
    serial::serial_print("Yo nibba looks like you did a dogshit wow!");
    loop {}
}
