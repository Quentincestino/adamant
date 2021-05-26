#[macro_export]
macro_rules! kernel_panic {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Panic, ::core::format_args!($($arg)*), file!(), line!());
    }}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(infos: &PanicInfo) -> ! {
    // We gonna get theses args but we still need to pattern match these values
    match (infos.location(), infos.message()) {
        (Some(location), Some(message)) => {
            kernel_panic!("Panic at {}: {}", location, message)
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
