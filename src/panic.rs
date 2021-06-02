#[macro_export]
macro_rules! kernel_panic {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Panic, ::core::format_args!($($arg)*), file!(), line!());
    }}
}

use core::panic::PanicInfo;

use crate::arch::x86_64;

#[panic_handler]
fn panic(infos: &PanicInfo) -> ! {
    // We gonna get theses args but we still need to pattern match these values
    match (infos.location(), infos.message()) {
        (Some(location), Some(message)) => {
            kernel_panic!("Panic at {}: {}", location, message)
        }
        _ => unreachable!(),
    }
    loop {
        // If we don't halt cpu just gonna die
        x86_64::painless_halt();
    }
}
