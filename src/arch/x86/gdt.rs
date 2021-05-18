use crate::log::*;

#[repr(packed)]
pub struct GdtPointer {
    len: u16,
    addr: u64,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Segment {
    limit_low: u16,                 // Limit Low (Bit 0 to 15)
    base_low: u16,                  // Base Low (Bit 16 to 31)
    base_mid: u8,                   // Base Mid (Bit 32 to 39)
    flags: u8,                      // Access Byte (Bit 40 to 47)
    limit_high_and_granularity: u8, // Limit high: (Bit 48 to 51) and granularity: (Bit 52 to 55)
    base_high: u8,                  // Base high 56:63
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

#[link(name = "x86_64_arch")]
extern "C" {
    fn load_gdt(gdt_descriptor: *const GdtPointer);
}

const GDT_ENTRIES: usize = 5;

// Setup the GDT
static mut GDT: [Segment; GDT_ENTRIES] = [Segment::null(); GDT_ENTRIES];
static mut GDT_POINTER: GdtPointer = GdtPointer { len: 0, addr: 0 };

// We use only two values for granularity on the GDT so let's setup some consts
const CODE_GRANULARITY: u8 = 0b1111_1000_;
const DATA_GRANULARITY: u8 = 0b1111_0000_;

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
        info("GDT Loaded without triple fault OwO");
    }
}
