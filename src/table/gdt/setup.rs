use table::gdt;

pub fn setup() {
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

