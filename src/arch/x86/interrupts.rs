use crate::serial;

use super::idt::*;

pub extern "x86-interrupt" fn divide_by_zero(isf: InterruptStackFrame) {
    serial::serial_print("Wait nibba did you rly divide by zero ?");
}
