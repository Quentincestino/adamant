use core::mem::MaybeUninit;

use crate::{info, ok};

mod bitmap;

use super::mem_map::{MemoryAreaType, MemoryMap};
use bitmap::*;

const KiB: usize = 1024;
const PAGE_SIZE: usize = 4 * KiB;

// This function represents a memory range that can be translated in [start_addr; end_addr]
pub struct MemoryRange {
    start_addr: usize,
    end_addr: usize,
}

// This function represents a page range that can be translated in [start_page; end_page]
#[derive(Debug)]
pub struct PageRange {
    start_page_index: isize,
    end_page_index: isize,
}

impl PageRange {
    pub fn is_valid(&self) -> bool {
        self.start_page_index >= 0 && self.end_page_index >= 0
    }
}

static mut BITMAP: PmmBitmap = PmmBitmap::null();
static mut MEMORY_MAP: MaybeUninit<MemoryMap> = MaybeUninit::uninit();

pub fn init_pmm(memory_map: MemoryMap) {
    // We set the memory map
    unsafe {
        MEMORY_MAP = MaybeUninit::new(memory_map);
    }

    // Gets usable entries for PMM
    let usable = memory_map
        .iter()
        .filter(move |area| area.area_type == MemoryAreaType::Usable);

    let mut available_bytes = 0u64;

    let mut bitmap_addr: *mut u8 = core::ptr::null_mut(); // This gonna be the start address of our bitmap

    for &entry in usable.clone() {
        // We actually need a pointer to allocate the PMM's Bitmap
        if bitmap_addr.is_null() {
            bitmap_addr = entry.start_addr as *mut u8;
        };
        info!("{}", entry);
        available_bytes += entry.end_addr - entry.start_addr; // We count this whole entry as a usable memory range
    }

    info!("There is {} available bytes.", available_bytes);
    let page_count = available_bytes as usize / PAGE_SIZE; // We consider that one page is PAGE_SIZE long
    info!("{} Pages created.", page_count);

    initialize_bitmap(bitmap_addr, page_count);
    info!("Bitmap length: {}", unsafe { BITMAP.len() });
    set_bitmap_zone_used();
}

// This function creates the PMM's bitmap. We do not mark it as unsafe because we are 100% sure that bitmap_addr is usable
fn initialize_bitmap(bitmap_addr: *mut u8, page_count: usize) {
    unsafe {
        BITMAP = PmmBitmap::with_capacity(bitmap_addr, page_count / 8);
        ok!(
            "PMM's bitmap starts at {:#08x} and ends at {:#08x}",
            bitmap_addr as usize,
            BITMAP.end() as usize
        );
    }
}

// Actually the bitmap is allocated in a memory map zone being marked as usable.
// Since we do not want to erase the bitmap to allocate something else we must set the bitmap's memory as used on itself !
fn set_bitmap_zone_used() {
    unsafe {
        let bitmap_end = core::mem::size_of::<PmmBitmap>() + BITMAP.len();
        ok!("End ? {:#08x}", bitmap_end);

        for i in (0..bitmap_end).step_by(PAGE_SIZE) {
            bitmap_set_bit((i / PAGE_SIZE) as u64, &mut BITMAP);
        }
    }
}

// Finds the `count` free pages on the bitmap and returns a `PageRange`
fn find_free_pages<'a>(count: usize) -> Result<PageRange, &'a str> {
    static mut last_free: usize = unsafe { BITMAP.end() as usize }; // The last free page we found

    let mut free_count = 0usize; // Counts pages we already found
    let mut page_range = PageRange {
        start_page_index: -1,
        end_page_index: 0,
    }; // The returned page range

    let bitmap_page_number = unsafe { BITMAP.len() };
    let bitmap = unsafe { &BITMAP };

    for i in unsafe { last_free }..bitmap_page_number {
        if !bitmap_is_bit_set(i as u64, &bitmap) {
            free_count += 1; // We found another page ! Yay !

            // It's the first free page we found
            if page_range.start_page_index == -1 {
                page_range.start_page_index = i as isize;
            }

            page_range.end_page_index = i as isize; // Atm that's the last page we found

            // We found all we needed
            if free_count == count {
                unsafe {
                    last_free = i + 1; // Now `i` is the last page we found that was free
                }
                return Ok(page_range);
            }
        }
    }
    Err("Unable to find free pages")
}

pub fn alloc<'a>(count: usize) -> Result<PageRange, &'a str> {
    let page = find_free_pages(count)?;

    for i in page.start_page_index..=page.end_page_index {
        bitmap_set_bit(i as u64, unsafe { &mut BITMAP });
    }

    Ok(page)
}

pub fn __dump_bitmap() {
    bitmap::__dump_bitmap(unsafe { &BITMAP });
}
