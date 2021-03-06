use crate::serial::SerialPort;

use ::core::format_args;
use core::fmt::{Arguments, Write};

pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Error,
    Panic,
}

pub static mut LOG_COM: SerialPort = SerialPort::COM1;

pub const ANSI_RESET: &str = "\u{001b}[0m";
pub const BOLD: &str = "\u{001b}[1m";

pub const INFO_CYAN: &str = "\u{001b}[36m";
pub const OK_GREEN: &str = "\u{001b}[32m";
pub const WARN_YELLOW: &str = "\u{001b}[33m";
pub const ERROR_RED: &str = "\u{001b}[31m";
pub const PANIC_MAGENTA: &str = "\u{001b}[35m";

pub fn log(level: LogLevel, message: Arguments, file: &str, line: u32) {
    match level {
        LogLevel::Info => {
            _print_log(
                format_args!("{}{}[ INFO ]{}  ", BOLD, INFO_CYAN, ANSI_RESET),
                message,
                file,
                line,
            );
        }
        LogLevel::Ok => {
            _print_log(
                format_args!("{}{}[ OK ]{}    ", BOLD, OK_GREEN, ANSI_RESET),
                message,
                file,
                line,
            );
        }
        LogLevel::Warn => {
            _print_log(
                format_args!("{}{}[ WARN ]{}  ", BOLD, WARN_YELLOW, ANSI_RESET),
                message,
                file,
                line,
            );
        }
        LogLevel::Error => {
            _print_log(
                format_args!("{}{}[ ERROR ]{} ", BOLD, ERROR_RED, ANSI_RESET),
                message,
                file,
                line,
            );
        }
        LogLevel::Panic => {
            _print_log(
                format_args!("{}{}[ PANIC ]{} ", BOLD, PANIC_MAGENTA, ANSI_RESET),
                message,
                file,
                line,
            );
        }
    }
}

pub fn _print_log(prefix: Arguments, msg: Arguments, file: &str, line: u32) {
    let mut com = unsafe { LOG_COM };
    let _ = com.write_fmt(prefix);
    let _ = com.write_fmt(format_args!(
        "{}at {}:{}:{} {}\n",
        INFO_CYAN, file, line, ANSI_RESET, msg
    ));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Info, ::core::format_args!($($arg)*), file!(), line!());
    }}
}

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Ok, ::core::format_args!($($arg)*), file!(), line!());
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Warn, ::core::format_args!($($arg)*), file!(), line!());
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log::log($crate::log::LogLevel::Error, ::core::format_args!($($arg)*), file!(), line!());
    }}
}
