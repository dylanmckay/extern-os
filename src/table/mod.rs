pub mod gdt;

use prelude::*;
use core::mem;

/// A type of table.
pub trait TableKind
{
    type Entry;

    fn update_descriptor(descriptor: Descriptor);
}

/// The descriptor of a table.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Descriptor
{
    pub size: u16,
    pub offset: u32,
}

/// A table.
pub struct Table<T: TableKind>
{
    enabled: bool,
    entries: Vec<T::Entry>,
}

impl<T: TableKind> Table<T>
{
    pub fn empty() -> Self {
        Table {
            enabled: false,
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: T::Entry) {
        self.entries.push(entry);
        self.update();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.update();
    }

    pub fn descriptor(&self) -> Descriptor {
        Descriptor {
            size: self.size_in_bytes() as u16,
            offset: self.entries.as_ptr() as u32,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        self.entries.len() * mem::size_of::<T::Entry>()
    }

    pub fn is_enabled(&self) -> bool { self.enabled }

    fn update(&self) {
        if self.enabled {
            T::update_descriptor(self.descriptor());
        }
    }
}

