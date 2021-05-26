pub mod mem_map;
pub mod pmm;

use crate::stivale2::stivale_struct;

pub fn init() {
    mem_map::stivale2_to_memmap(stivale_struct());
}
