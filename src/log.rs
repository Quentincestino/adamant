use crate::serial::SerialPort;

use core::fmt::{Arguments, Write};

pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Error,
    Panic,
}

pub static mut LOG_COM: SerialPort = SerialPort::COM1;

pub fn log(level: LogLevel, message: Arguments) {
    let message_head = match level {
        LogLevel::Info => "[ INFO ] ",
        LogLevel::Ok => "[ OK ] ",
        LogLevel::Warn => "[ WARN ] ",
        LogLevel::Error => "[ ERROR ] ",
        LogLevel::Panic => "[ PANIC ] ",
    };

    let mut com = unsafe { LOG_COM };
    let _ = com.write_str(message_head); // We don't care to unwrap
    let _ = com.write_fmt(message); // Same
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Info, ::core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Ok, ::core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Warn, ::core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Error, ::core::format_args!($($arg)*));
    }}
}
