use super::bootloaders::stivale2::stivale2_to_memmap;
use crate::stivale2::stivale_struct;

#[derive(Copy, Clone)]
pub struct MemoryArea {
    pub start_addr: u64,
    pub end_addr: u64,
    pub size: u64,
    pub area_type: MemoryAreaType,
}

impl MemoryArea {
    pub const fn default() -> Self {
        MemoryArea {
            start_addr: 0,
            end_addr: 0,
            size: 0,
            area_type: MemoryAreaType::BadMemory,
        }
    }

    pub const fn new(start: u64, end: u64, size: u64, area_type: MemoryAreaType) -> Self {
        Self {
            start_addr: start,
            end_addr: end,
            size,
            area_type,
        }
    }
}

impl core::fmt::Display for MemoryArea {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Memory Area {{ Start: {:#08x}, End: {:#08x}, Size: {} bytes }}",
            self.start_addr, self.end_addr, self.size
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MemoryAreaType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNotReclaimable,
    BadMemory,
    BootloaderReclaimable,
    KernelContaining,
}

pub const MAX_ENTRIES: usize = 16; // Since we need to put a constant length to our MemoryMap, we choose an arbitrary size.
pub type MemoryMap = [MemoryArea; MAX_ENTRIES];
pub type MemoryMapSubset = [MemoryArea; MAX_ENTRIES];

// Gets the memory map, most of cases from a bootloader, to convert it into a bootloader-agnostic one
pub fn memory_map() -> MemoryMap {
    stivale2_to_memmap(stivale_struct())
}
