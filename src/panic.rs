#[macro_export]
macro_rules! kernel_panic {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Panic, ::core::format_args!($($arg)*));
        $crate::serial::serial_print("\n", unsafe { $crate::log::LOG_COM });
    }}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(infos: &PanicInfo) -> ! {
    // We gonna get theses args but we still need to pattern match these values
    match (infos.location(), infos.message()) {
        (Some(location), Some(message)) => {
            kernel_panic!("Panic at {}: {}", location, message.clone())
        }
        _ => unreachable!(),
    }
    unsafe {
        // Anyway if there is an error, we gonna die no matter what
        asm!("cli");

        loop {
            // If we don't halt cpu just gonna die
            asm!("hlt");
        }
    }
}