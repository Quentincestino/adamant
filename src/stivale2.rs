use stivale::*;

// The size of the stack used by the kernel
const STACK_SIZE: usize = 16384;
// The stack used by the kernel
static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

// The Stivale2 Tag that initializes the framebuffer
static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
// StivaleHeader, used by Limine
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[STACK_SIZE - 1] as *const u8)
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
