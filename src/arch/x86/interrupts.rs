use super::idt::*;
use crate::log::*;

pub extern "x86-interrupt" fn divide_by_zero(_isf: InterruptStackFrame) {
    error("Wait nibba did you rly divide by zero ?", false);
}
