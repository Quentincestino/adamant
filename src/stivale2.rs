use stivale::*;

// The stack used by the kernel
static STACK: [u8; 4096] = [0; 4096];

// The Stivale2 Tag that initializes the framebuffer
static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
// StivaleHeader, used by Limine
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[4095] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const HeaderFramebufferTag).cast());

static mut STIVALE_STRUCT_ADDR: usize = 0;

pub fn set_stivale_addr(addr: usize) {
    unsafe {
        STIVALE_STRUCT_ADDR = addr;
    }
}

pub fn stivale_struct() -> StivaleStructure {
    unsafe { stivale::load(STIVALE_STRUCT_ADDR) }
}
