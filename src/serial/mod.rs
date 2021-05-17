#[allow(dead_code)]
#[repr(u16)]
pub enum SerialPort {
    COM1 = 0x3F8,
    COM2 = 0x2A8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

// OUT
fn outb(port: SerialPort, val: u8) {
    let port = port as u16;
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val);
    }
}

// IN
fn inb(addr: u16) -> u8 {
    let mut val = 0u8;
    unsafe { asm!("in al, dx", in("dx") addr, out("al") val) }

    val
}

// The default COM port that we use.
const DEFAULT_COM: SerialPort = SerialPort::COM1;
// Prints a string on the DEFAULT_COM port
pub fn serial_print(string: &str) {
    for c in string.as_bytes() {
        while !serial_available(DEFAULT_COM) {} // Waits for serial to be available
        outb(DEFAULT_COM, *c);
    }
}

// Checks if a byte is waiting for us on a serial port
fn serial_received(serial: SerialPort) -> bool {
    let addr = (serial as u16 + 5) as u16;
    let val = inb(addr);
    (val & 1) == 0
}

// Checks if the serial port received the last byte we sent
fn serial_available(serial: SerialPort) -> bool {
    let addr = (serial as u16 + 5) as u16;
    let val = inb(addr);
    (val & 0x20) == 32
}
