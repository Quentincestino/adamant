use crate::allocator::mem_map::*;
use stivale::memory::MemoryMapEntryType;
use stivale::{memory::MemoryMapIter, StivaleStructure};

// This function converts the stivale2 memory map to a bootloader-agnostic one.
pub fn stivale2_to_memmap(stivale2_struct: StivaleStructure) -> MemoryMap {
    let memmap = stivale2_struct.memory_map();

    match memmap {
        Some(map) => {
            return _stivale_memmap_translate(map.iter());
        }
        None => {
            panic!("No memory map tag found in Stivale2 structure passed to the entry point.",);
        }
    }
}

fn _stivale_memmap_translate(map: MemoryMapIter) -> MemoryMap {
    let mut memory_map = [MemoryArea::default(); MAX_ENTRIES];

    for (index, entry) in map.enumerate() {
        let area_type = match entry.entry_type() {
            MemoryMapEntryType::Usable => MemoryAreaType::Usable,
            MemoryMapEntryType::Reserved => MemoryAreaType::Reserved,
            MemoryMapEntryType::AcpiReclaimable => MemoryAreaType::AcpiReclaimable,
            MemoryMapEntryType::AcpiNvs => MemoryAreaType::AcpiNotReclaimable,
            MemoryMapEntryType::BadMemory => MemoryAreaType::BadMemory,
            MemoryMapEntryType::BootloaderReclaimable => MemoryAreaType::BootloaderReclaimable,
            MemoryMapEntryType::Kernel => MemoryAreaType::KernelContaining,
        };
        let start_addr = entry.start_address();
        let end_addr = entry.end_address();
        let size = entry.size();

        memory_map[index] = MemoryArea::new(start_addr, end_addr, size, area_type);
    }

    memory_map
}
