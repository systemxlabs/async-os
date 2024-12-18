use core::ops::Range;

use crate::{
    KError, KResult,
    allocator::PHYS_FRAME_ALLOCATOR,
    config::PAGE_SIZE_4K,
    mem::{addr::PhysAddr, align_up},
};
use alloc::vec;
use alloc::vec::Vec;

use super::{
    VirtAddr,
    pte::{PTEFlags, PageTableEntry},
};

const SV39_TABLE_PTE_COUNT: usize = 512;

pub struct PageTable {
    root_paddr: PhysAddr,
    intrm_tables: Vec<PhysAddr>,
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            root_paddr: PhysAddr::new(usize::MAX),
            intrm_tables: Vec::new(),
        }
    }

    pub fn try_new() -> Option<Self> {
        let root_paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1, PAGE_SIZE_4K)?;
        unsafe { core::ptr::write_bytes(root_paddr.as_usize() as *mut u8, 0, PAGE_SIZE_4K) };
        Some(Self {
            root_paddr,
            intrm_tables: vec![root_paddr],
        })
    }

    pub fn root_paddr(&self) -> PhysAddr {
        self.root_paddr
    }

    pub fn map(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: PTEFlags) {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
        let pte = self.get_entry_mut(vaddr, true);
        if pte.is_unused() {
            *pte = PageTableEntry::new(paddr, flags);
        } else {
            panic!("Already mapped");
        }
    }

    pub fn map_region(
        &mut self,
        vaddr: VirtAddr,
        paddr: PhysAddr,
        num_pages: usize,
        flags: PTEFlags,
    ) {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
        for i in 0..num_pages {
            self.map(vaddr + i * PAGE_SIZE_4K, paddr + i * PAGE_SIZE_4K, flags);
        }
    }

    pub fn map_range_linear(&mut self, range_va: Range<VirtAddr>, flags: PTEFlags) {
        assert!(range_va.start.is_aligned(PAGE_SIZE_4K));
        let start = range_va.start.as_usize();
        let end = align_up(range_va.end.as_usize(), PAGE_SIZE_4K);
        let num_pages = (end - start) / PAGE_SIZE_4K;
        self.map_region(start.into(), start.into(), num_pages, flags);
    }

    pub fn query_page(&mut self, vpn: VirtAddr) -> (PhysAddr, PTEFlags) {
        assert_eq!(vpn.as_usize() & (PAGE_SIZE_4K - 1), 0);
        let pte = self.get_entry_mut(vpn, false);
        (pte.ppn(), pte.flags())
    }

    pub fn translate(&mut self, vaddr: VirtAddr) -> KResult<PhysAddr> {
        let pte = self.get_entry_mut(vaddr, false);
        if pte.is_valid() {
            let offset = vaddr.as_usize() & (PAGE_SIZE_4K - 1);
            let paddr = pte.ppn().as_usize() + offset;
            Ok(paddr.into())
        } else {
            Err(KError::MemNotMapped)
        }
    }

    fn table_of_mut<'a>(&self, paddr: PhysAddr) -> &'a mut [PageTableEntry] {
        let ptr = paddr.as_usize() as _;
        // as we did identical mapping, so vaddr = paddr
        unsafe { core::slice::from_raw_parts_mut(ptr, SV39_TABLE_PTE_COUNT) }
    }

    fn next_table_mut<'a>(
        &mut self,
        entry: &mut PageTableEntry,
        create_if_absent: bool,
    ) -> &'a mut [PageTableEntry] {
        if entry.is_unused() && create_if_absent {
            let paddr = PHYS_FRAME_ALLOCATOR
                .lock()
                .alloc_frames(1, PAGE_SIZE_4K)
                .expect("No enough frames");
            self.intrm_tables.push(paddr);
            *entry = PageTableEntry::new(paddr, PTEFlags::V);
        }
        if entry.is_valid() {
            self.table_of_mut(entry.ppn())
        } else {
            panic!("Not mapped");
        }
    }

    fn get_entry_mut(&mut self, vaddr: VirtAddr, create_if_absent: bool) -> &mut PageTableEntry {
        let table1 = self.table_of_mut(self.root_paddr);
        let table1_pte_index = (vaddr.as_usize() >> (12 + 18)) & (SV39_TABLE_PTE_COUNT - 1);
        let table1_pte = &mut table1[table1_pte_index];

        let table2 = self.next_table_mut(table1_pte, create_if_absent);
        let table2_pte_index = (vaddr.as_usize() >> (12 + 9)) & (SV39_TABLE_PTE_COUNT - 1);
        let table2_pte = &mut table2[table2_pte_index];

        let table3 = self.next_table_mut(table2_pte, create_if_absent);
        let table3_pte_index = (vaddr.as_usize() >> 12) & (SV39_TABLE_PTE_COUNT - 1);
        let table3_pte = &mut table3[table3_pte_index];

        table3_pte
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        for paddr in self.intrm_tables.iter() {
            PHYS_FRAME_ALLOCATOR.lock().dealloc_frames(*paddr, 1);
        }
    }
}
