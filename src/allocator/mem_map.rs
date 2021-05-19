use crate::log::{self, error};
use core::mem::MaybeUninit;
use stivale::StivaleStructure;

pub struct MemoryMap {}

pub fn stivale2_to_memmap(stivale2_struct: &StivaleStructure) {
    let memmap = unsafe { &stivale2_struct.memory_map() };

    match memmap {
        Some(map) => {}
        None => {
            error(
                "No memory map tag found in Stivale2 structure passed to the entry point.",
                true,
            );
            panic!("No memory map");
        }
    }
}
