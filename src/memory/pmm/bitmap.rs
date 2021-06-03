use core::ptr::null_mut;

use crate::info;

pub struct PmmBitmap {
    start_addr: *mut u8,
    len: usize,
}

impl PmmBitmap {
    pub const fn null() -> Self {
        Self {
            start_addr: null_mut(),
            len: 0,
        }
    }

    pub fn with_capacity(start_addr: *mut u8, len: usize) -> Self {
        assert!(!start_addr.is_null()); // Would cause UB if we try to write there

        let mut to_return = Self { start_addr, len: 0 };

        for _ in 0..len {
            to_return.add_zeroed_entry();
        }

        to_return
    }

    pub fn add_zeroed_entry(&mut self) {
        assert!(!self.start_addr.is_null()); // Would cause UB if we try to write there

        self.len += 1;

        unsafe {
            *((self.start_addr as usize + self.len) as *mut u8) = 0;
        }
    }

    pub const fn len(&self) -> usize {
        self.len + 1
    }

    pub const fn get(&self, index: usize) -> *mut u8 {
        unsafe { (self.start_addr as usize + index) as *mut u8 }
    }

    pub const fn start(&self) -> *mut u8 {
        self.start_addr
    }

    pub const fn end(&self) -> *mut u8 {
        unsafe { (self.start_addr as usize + self.len) as *mut u8 }
    }

    pub fn set_byte(&mut self, array_index: usize, val: u8) {
        unsafe {
            *self.get(array_index) = val;
        }
    }

    pub fn set_bit(&mut self, array_index: usize, bit_index: usize, state: bool) {
        assert!(bit_index <= 7); // Only 8 bits in a byte
        assert!(array_index <= self.len);

        let state: u8 = match state {
            true => 1,
            false => 0,
        };
        unsafe { *self.get(array_index) |= state << bit_index }
    }

    pub fn get_bit(&self, array_index: usize, bit_index: usize) -> bool {
        assert!(bit_index <= 7); // Only 8 bits in a byte
        assert!(array_index <= self.len);

        let bit = unsafe { *self.get(array_index) & (1 >> bit_index) };

        match bit {
            0 => false,
            1 => true,
            _ => panic!(
                "Bit {}:{} is on a weird state: {}",
                array_index, bit_index, bit
            ),
        }
    }
}

#[inline]
// Gets the index of the page address on the bitmap
pub fn get_bitmap_array_index(page_addr: u64) -> usize {
    page_addr as usize / 8
}

#[inline]
// Gets the index of the page index on the bitmap
pub fn get_bitmap_bit_index(page_addr: u64) -> usize {
    page_addr as usize % 8
}

#[inline]
pub fn bitmap_set_bit(page_addr: u64, map: &mut PmmBitmap) {
    let bit: usize = get_bitmap_bit_index(page_addr);
    let byte: usize = get_bitmap_array_index(page_addr);
    map.set_bit(byte, bit, true);
}

#[inline]
pub fn bitmap_clear_bit(page_addr: u64, map: &mut PmmBitmap) {
    let bit: usize = get_bitmap_bit_index(page_addr);
    let byte: usize = get_bitmap_array_index(page_addr);
    map.set_bit(byte, bit, false);
}

#[inline]
pub fn bitmap_is_bit_set(page_addr: u64, map: &PmmBitmap) -> bool {
    let array_index = get_bitmap_array_index(page_addr);
    let bit_index = get_bitmap_bit_index(page_addr);
    map.get_bit(array_index, bit_index)
}

pub fn __dump_bitmap(bitmap: &PmmBitmap) {
    info!("BITMAP DUMP");
    let mut i = 0usize;

    while unsafe { *bitmap.get(i) != 0 } {
        info!("Index {}: {:8b}", i, unsafe { *bitmap.get(i) });
        i += 1;
    }
}
