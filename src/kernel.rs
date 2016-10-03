#![feature(lang_items)]
#![feature(start)]
#![feature(link_args)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(conservative_impl_trait)]
#![feature(collections)]
#![feature(alloc)]
#![feature(drop_types_in_const)]

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate extern_os_allocator;
extern crate collections;
extern crate alloc;
extern crate rlibc;

pub use self::error::Error;

pub mod io;
pub mod vga;
pub mod drivers;
pub mod interrupt;
pub mod table;
pub mod bios;
pub mod support;
pub mod prelude;
#[macro_use]
pub mod debug;
pub mod error;
pub mod x86;

/// A kernel.
pub trait Kernel
{
    fn initialize();

    fn step();

    fn deinitialize();

    #[allow(unreachable_code)]
    fn run() {
        Self::initialize();
        loop { Self::step(); }
        Self::deinitialize();
    }
}

pub type CurrentKernel = x86::Kernel;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let vga = vga::Buffer::new();
    let mut terminal = vga::Terminal::new(vga);

    terminal.clear();
    debug::initialize(terminal);

    CurrentKernel::run();
}

fn setup_gdt() {
    use table::gdt;

    let entries = [
        // The null entry.
        gdt::Entry::null(),

        /// The entry for ring0 code
        gdt::Entry {
            base: 0,
            limit: 0x000fffff,
            access: gdt::Access::executable(gdt::access::Ring::Ring0, gdt::access::Conforming::Equal, false),
            flags: gdt::Flags { granularity: gdt::flags::Granularity::Page, size: gdt::flags::Size::Bit32 },
        },
        /// The entry for ring0 data
        gdt::Entry {
            base: 0,
            limit: 0x000fffff,
            access: gdt::Access::data(gdt::access::Ring::Ring0, gdt::access::Direction::GrowsUp, true),
            flags: gdt::Flags { granularity: gdt::flags::Granularity::Page, size: gdt::flags::Size::Bit32 },
        },
    ];

    *gdt::get() = entries.iter().cloned().collect();
    gdt::get().enable();
}

