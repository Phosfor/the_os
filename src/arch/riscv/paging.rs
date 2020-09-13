use core::fmt;
use core::ops::{Index, IndexMut};

use bit_field::*;
use bitflags::bitflags;

pub struct PhysAddr(usize);
impl PhysAddr {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}
pub struct VirtAddr(usize);
impl VirtAddr {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

bitflags! {
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: usize {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const RESERVED1 =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub fn valid(&self) -> bool {
        self.0.get_bit(0)
    }

    #[inline]
    pub fn set_valid(&mut self, value: bool) {
        self.0.set_bit(0, value);
    }

    #[inline]
    pub fn ppn(&self) -> usize {
        self.0 >> 10
    }

    #[inline]
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.ppn() << 12)
    }

    #[inline]
    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0)
    }

    #[inline]
    pub fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }

    #[inline]
    pub fn set(&mut self, addr: PhysAddr, flags: PageTableFlags) {
        self.0 = (addr.0 & !0x3FF) >> 2 | flags.bits();
    }
}


const TABLE_SIZE: usize = 4096;
const ENTRY_COUNT: usize = TABLE_SIZE / core::mem::size_of::<PageTableEntry>();

#[repr(align(4096))]
#[repr(C)]
pub struct PageTable([PageTableEntry; ENTRY_COUNT]);

impl PageTable {
    #[inline]
    pub const fn new() -> Self {
        Self([PageTableEntry::new(); ENTRY_COUNT])
    }

    /// Returns an iterator over the entries of the page table.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.0.iter()
    }

    /// Returns an iterator that allows modifying the entries of the page table.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PageTable {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}