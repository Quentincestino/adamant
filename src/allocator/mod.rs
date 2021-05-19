pub mod mem_map;
pub mod pmm;

use crate::stivale2::get_stivale2_structure;
pub fn init() {
    mem_map::stivale2_to_memmap(get_stivale2_structure());
}
