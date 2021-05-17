// TODO: Prefix colors
use crate::serial::*;

const OK: &str = "[ OK ]    ";
pub fn ok(message: &str) {
    write(OK, message);
}

const INFO: &str = "[ INFO ]  ";
pub fn info(message: &str) {
    write(INFO, message);
}

const WARN: &str = "[ WARN ]  ";
pub fn warn(message: &str) {
    write(WARN, message);
}

const ERROR: &str = "[ ERROR ] ";
pub fn error(message: &str, panic: bool) {
    if panic {
        error_abort(message);
    } else {
        error_no_abort(message);
    }
}

fn error_abort(message: &str) {
    write(ERROR, message);
    write(ERROR, "This error will make the kernel panic.");
}

fn error_no_abort(message: &str) {
    write(ERROR, message);
}

fn write(log_level: &str, msg: &str) {
    serial_print(log_level);
    serial_print(msg);
    serial_print("\n");
}
