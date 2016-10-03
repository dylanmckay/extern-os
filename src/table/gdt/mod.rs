use table;

static mut GDT: Option<table::Table<Gdt>> = None;
static mut DESCRIPTOR: table::Descriptor = table::Descriptor { size: 0, offset: 0};

pub fn get() -> &'static mut table::Table<Gdt> {
    unsafe {
        if GDT.is_none() {
            GDT = Some(table::Table::empty());
        }
        GDT.as_mut().unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Entry
{
    /// 32-bit base.
    pub base: u32,
    /// 20-bit limit.
    pub limit: u32,
    /// The access byte.
    pub access: u8,
    /// The flags.
    pub flags: u8,
}

#[derive(Copy, Clone)]
pub struct Access
{
    pub present: bool,
    pub privilege: u8,
    pub executable: bool,
    pub direction_confirming: bool,
    pub readable_writable: bool,
    pub accessed: bool,
    pub granularity: bool,
}

#[repr(C, packed)]
pub struct RawEntry(pub [u8; 8]);

impl Entry
{
    pub fn encode(&self) -> RawEntry {
        assert!(self.limit <= 0b11111_11111_11111_11111, "GDT entry limit is too big");
        assert!(self.flags <= 0b1111, "GDT entry flag mask out of range");

        let mut bytes = [0u8; 8];

        // Encode the limit.
        bytes[0] = (self.limit & 0xff) as u8;
        bytes[1] = ((self.limit & 0xff00) >> 8) as u8;
        bytes[6] |= ((self.base & 0b00000000_00001111_00000000_00000000) >> 16) as u8;

        // Encode the base.
        bytes[2] = (self.base & 0xff) as u8;
        bytes[3] = ((self.base & 0xff00) >> 8) as u8;
        bytes[4] = ((self.base & 0xff0000) >> 16) as u8;
        bytes[7] = ((self.base & 0xff000000) >> 24) as u8;

        // Encode the access byte.
        bytes[5] = self.access;

        // Encode the flags.
        bytes[6] |= self.flags & 0b1111;

        RawEntry(bytes)
    }
}

pub struct Gdt;

impl table::TableKind for Gdt
{
    type Entry = Entry;

    fn update_descriptor(descriptor: table::Descriptor) {
        unsafe {
            DESCRIPTOR = descriptor;
            let descriptor_address = &DESCRIPTOR as *const _ as u16;
            asm!("lgdt $0" : : "m"(descriptor_address));
        }
    }
}

