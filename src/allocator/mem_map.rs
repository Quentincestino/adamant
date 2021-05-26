use stivale::StivaleStructure;

pub struct MemoryMap {}

pub fn stivale2_to_memmap(stivale2_struct: StivaleStructure) {
    let memmap = stivale2_struct.memory_map();

    match memmap {
        Some(_map) => {
            todo!()
        }
        None => {
            panic!("No memory map tag found in Stivale2 structure passed to the entry point.",);
        }
    }
}
