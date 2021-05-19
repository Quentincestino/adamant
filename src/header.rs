use core::mem::MaybeUninit;

use stivale::*;

static STACK: [u8; 4096] = [0; 4096];
static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[4095] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const HeaderFramebufferTag).cast());

static mut STIVALE_STRUCT: MaybeUninit<StivaleStructure> = MaybeUninit::uninit();
pub fn init_stivale2_struct(addr: usize) {
    unsafe {
        STIVALE_STRUCT = MaybeUninit::new(load(addr));
    }
}
