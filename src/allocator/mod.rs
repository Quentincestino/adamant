use crate::info;

mod bootloaders;
pub mod mem_map;
pub mod pmm;

pub fn init() {
    let memory_map = mem_map::memory_map();
    info!("Intiializing the physical memory manager...");
    pmm::init_pmm(memory_map);
}
