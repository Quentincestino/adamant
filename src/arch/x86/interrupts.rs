use super::idt::*;
use crate::log::*;

// 0
pub extern "x86-interrupt" fn divide_by_zero(_isf: &InterruptStackFrame) {
    error("Wait nibba did you rly divide by zero ?", false);
}

// 1
pub extern "x86-interrupt" fn debug(_isf: &InterruptStackFrame) {
    info("Debug interrupt called");
}

// 2
pub extern "x86-interrupt" fn nmi(_isf: &InterruptStackFrame) {
    error("NMI Interrupt, gonna explode", true);
    panic!("NMI Interrupt");
}

// 3
pub extern "x86-interrupt" fn breakpoint(_isf: &InterruptStackFrame) {
    error("Broke up", false);
}

// 4
pub extern "x86-interrupt" fn overflow(_isf: &InterruptStackFrame) {
    unreachable!();
}

// 5
pub extern "x86-interrupt" fn table_overflow(_isf: &InterruptStackFrame) {
    unreachable!();
}

// 6
pub extern "x86-interrupt" fn invalid_instruction(_isf: &InterruptStackFrame) {
    error("Wait wtf how did you just get here ???", true);
    panic!("Invalid Instruction encountered");
}

// 7
pub extern "x86-interrupt" fn unavailable_device(_isf: &InterruptStackFrame) {
    error("FPU Explosion !", false);
}

// 8
pub extern "x86-interrupt" fn double_fault(_isf: &InterruptStackFrame) {
    error("Do you want me to trigger a triple fault ?", false);
}

// 9
pub extern "x86-interrupt" fn coproc_segment(_isf: &InterruptStackFrame) {
    error("Are you okay ?", true);
    panic!("Wait wat");
}

// 10
pub extern "x86-interrupt" fn invalid_tss(_isf: &InterruptStackFrame) {
    error("Supercyp still doesn't understand you anyway", false);
}

// 11
pub extern "x86-interrupt" fn not_present_segment(_isf: &InterruptStackFrame) {
    error("Fuck GDT", true);
    panic!("Invalid GDT");
}

// 12
pub extern "x86-interrupt" fn invalid_stack_segment(_isf: &InterruptStackFrame) {
    error("Energizer inc.", true);
    panic!("Invalid Stack Segment");
}

// 13
pub extern "x86-interrupt" fn general_protection_fault(_isf: &InterruptStackFrame) {
    error("Like page fault", true);
    panic!("General Protection Fault");
}

// 14
pub extern "x86-interrupt" fn page_fault(_isf: &InterruptStackFrame) {
    error(
        "Have you already reached the page 190/189 on a book ?",
        true,
    );
    panic!("Page fault");
}

// 15
pub extern "x86-interrupt" fn reserved_15(_isf: &InterruptStackFrame) {
    error("Lol no", false);
}

// 16
pub extern "x86-interrupt" fn fpu_fault(_isf: &InterruptStackFrame) {
    error(
        "Still not learn arithmetic with floating point numbers",
        true,
    );
    panic!("FPU Error");
}

// 17
pub extern "x86-interrupt" fn align_fault(_isf: &InterruptStackFrame) {
    error("Alignment is bad man", true);
    panic!("Align fault");
}

// 18
pub extern "x86-interrupt" fn check_exception(_isf: &InterruptStackFrame) {
    error("Yo check it out !", false);
}

// 19
pub extern "x86-interrupt" fn simd_exception(_isf: &InterruptStackFrame) {
    error("Floating point is hard hommy", false);
}

// 20
pub extern "x86-interrupt" fn virtualization_exception(_isf: &InterruptStackFrame) {
    error("KVM Is not supported man", true);
    panic!("Virtualization exception");
}
