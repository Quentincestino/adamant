fn main() {
    nasm_rs::Build::new()
        .target("x86_64-none-none")
        .file("src/arch/x86/gdt.asm")
        .compile("x86_64_gdt")
        .expect("Can't compile gdt asm");
}
