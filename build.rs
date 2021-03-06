fn main() {
    nasm_rs::Build::new()
        .target("x86_64-none-none")
        .file("src/arch/x86_64/gdt.asm")
        .file("src/arch/x86_64/idt.asm")
        .compile("x86_64_arch")
        .expect("Can't compile x86_64 asm");
}
