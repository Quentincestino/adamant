use crate::info;

use super::mem_map::{MemmapImpl, MemoryAreaType, MemoryMap};

pub fn init_pmm(memory_map: MemoryMap) {
    let (map, last_index) = memory_map.marked_as(MemoryAreaType::Usable);
    let usable_memory = &map[0..last_index];
    for entry in usable_memory {
        info!("{}", entry);
    }
}
