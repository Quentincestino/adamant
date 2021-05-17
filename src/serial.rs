#[allow(dead_code)]
#[repr(u16)]
pub enum SerialPort {
    COM1 = 0x3F8,
    COM2 = 0x2A8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

fn outb(port: SerialPort, val: u8) {
    let port = port as u16;
    unsafe {
        llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
    }
}
pub fn serial_print(string: &str) {
    for c in string.as_bytes() {
        outb(SerialPort::COM1, *c);
    }
    outb(SerialPort::COM1, b'\n');
}
