use crate::serial;

#[repr(packed)]
pub struct GdtPointer {
    len: u16,
    addr: u64,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Segment {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    flags: u8,
    limit_high_and_granularity: u8,
    base_high: u8,
}

impl Segment {
    pub const fn null() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            flags: 0,
            limit_high_and_granularity: 0,
            base_high: 0,
        }
    }

    pub const fn new(flags: u8, granularity: u8) -> Self {
        Segment {
            limit_low: u16::MAX,
            base_low: 0,
            base_mid: 0,
            flags: flags,
            limit_high_and_granularity: granularity,
            base_high: 0,
        }
    }
}

#[link(name = "x86_64_gdt")]
extern "C" {
    fn load_gdt(gdt_descriptor: *const GdtPointer);
}

const GDT_ENTRIES: usize = 5;

// Setup the GDT
static mut GDT: [Segment; GDT_ENTRIES] = [Segment::null(); GDT_ENTRIES];
static mut GDT_POINTER: GdtPointer = GdtPointer { len: 0, addr: 0 };

// We use only two values for granularity on the GDT so let's setup some consts
const CODE_GRANULARITY: u8 = /*Limit High */0b1111_0101/*Granularity*/;
const DATA_GRANULARITY: u8 = /*Limit High */0b1111_0001/*Granularity*/;

pub fn gdt_init() {
    unsafe {
        GDT[0] = Segment::null(); // Null segment
        GDT[1] = Segment::new(0b10011010, CODE_GRANULARITY); // Kernel Code
        GDT[2] = Segment::new(0b10010010, DATA_GRANULARITY); // Kernel Data
        GDT[3] = Segment::new(0b11111010, CODE_GRANULARITY); // User Code
        GDT[4] = Segment::new(0b11110010, DATA_GRANULARITY); // User Data

        GDT_POINTER = GdtPointer {
            len: (core::mem::size_of::<[Segment; GDT_ENTRIES]>() - 1) as u16,
            addr: (&GDT as *const _) as u64,
        };

        load_gdt(&GDT_POINTER as *const _);
        serial::serial_print("GDT Loaded without triple fault OwO")
    }
}
