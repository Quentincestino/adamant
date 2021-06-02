#[allow(dead_code)]
#[repr(u16)]
#[derive(Copy, Clone)]
pub enum SerialPort {
    COM1 = 0x3F8,
    COM2 = 0x2A8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

impl core::fmt::Write for SerialPort {
    fn write_fmt(mut self: &mut Self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        serial_print(s, *self);
        core::fmt::Result::Ok(())
    }
}

// OUT
fn outb(port: SerialPort, val: u8) {
    let port = port as u16;
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val);
    }
}

fn _outb_raw(port: u16, val: u8) {
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
pub fn serial_print(string: &str, port: SerialPort) {
    for c in string.bytes() {
        while !serial_available(DEFAULT_COM) {} // Waits for serial to be available
        serial_outb(c, port);
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
    let addr = serial as u16 + 5;
    let val = inb(addr) & 0x20;
    val != 0
}

// Writes a byte on the port
pub fn serial_outb(byte: u8, port: SerialPort) {
    outb(port, byte);
}
