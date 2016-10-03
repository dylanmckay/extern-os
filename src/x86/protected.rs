pub fn enable() {
    unsafe {
        asm!("mov %eax, %cr0");
        asm!("or %al, 1"); // Set PE (Protection Enable) bit in CR0 (Control Register 0)
        asm!("mov %cr0, %eax");
    }
}
