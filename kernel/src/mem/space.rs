use core::ops::{Add, Range};

use alloc::collections::btree_map::BTreeMap;
use log::info;
use spin::Mutex;

use crate::{
    config::{PAGE_SIZE_4K, TRAMPOLINE},
    dtb::MACHINE_META,
    mem::PTEFlags,
};

use super::{PageTable, PhysAddr, VirtAddr};

pub static KERNEL_SPACE: Mutex<AddrSpace> = Mutex::new(AddrSpace::empty());

unsafe extern "C" {
    fn stext();
    fn strampoline();
    fn etrampoline();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sstack();
    fn estack();
    fn sbss();
    fn ebss();
    fn ekernel();
}

pub fn init_kernel_space() {
    let mut space = AddrSpace::new();

    let meta = MACHINE_META.get().expect("dtb parsed");
    let phys_mem_end = meta.phys_mem_start + meta.phys_mem_size;

    log::info!(
        "[kernel] .text [{:#x}, {:#x}) [{:#x}, {:#x})",
        stext as usize,
        strampoline as usize,
        etrampoline as usize,
        etext as usize
    );
    space.page_table.map_range_linear(
        (stext as usize).into()..(strampoline as usize).into(),
        PTEFlags::R | PTEFlags::X | PTEFlags::V,
    );
    space.page_table.map_range_linear(
        (etrampoline as usize).into()..(etext as usize).into(),
        PTEFlags::R | PTEFlags::X | PTEFlags::V,
    );

    log::info!(
        "[kernel] .text.trampoline [{:#x}, {:#x})",
        strampoline as usize,
        etrampoline as usize,
    );
    space.page_table.map_range_linear(
        (strampoline as usize).into()..(etrampoline as usize).into(),
        PTEFlags::R | PTEFlags::X | PTEFlags::V,
    );

    log::info!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize,
        erodata as usize
    );
    space.page_table.map_range_linear(
        (srodata as usize).into()..(erodata as usize).into(),
        PTEFlags::R | PTEFlags::V,
    );

    log::info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize,
        edata as usize
    );
    space.page_table.map_range_linear(
        (sdata as usize).into()..(edata as usize).into(),
        PTEFlags::R | PTEFlags::W | PTEFlags::V,
    );

    log::info!(
        "[kernel] .stack [{:#x}, {:#x})",
        sstack as usize,
        estack as usize
    );
    space.page_table.map_range_linear(
        (sstack as usize).into()..(estack as usize).into(),
        PTEFlags::R | PTEFlags::W | PTEFlags::V,
    );

    log::info!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    space.page_table.map_range_linear(
        (sbss as usize).into()..(ebss as usize).into(),
        PTEFlags::R | PTEFlags::W | PTEFlags::V,
    );

    log::info!(
        "[kernel] physical mem [{:#x}, {:#x})",
        ekernel as usize,
        phys_mem_end,
    );
    space.page_table.map_range_linear(
        (ekernel as usize).into()..phys_mem_end.into(),
        PTEFlags::R | PTEFlags::W | PTEFlags::V,
    );

    for virtio_dev in meta.virtio.iter() {
        let end = virtio_dev.base_address + virtio_dev.size;
        log::info!(
            "[kernel] virtio mmio [{:#x}, {:#x})",
            virtio_dev.base_address,
            end,
        );
        space.page_table.map_range_linear(
            virtio_dev.base_address.into()..end.into(),
            PTEFlags::R | PTEFlags::W | PTEFlags::V,
        );
    }

    space.map_trampoline();

    *KERNEL_SPACE.lock() = space;
    kernel_space_test();
}

pub struct AddrSpace {
    page_table: PageTable,
    areas: BTreeMap<VirtAddr, MemoryArea>,
}

pub struct MemoryArea {
    va_range: Range<VirtAddr>,
    area_type: AreaType,
    pages: BTreeMap<VirtAddr, PhysAddr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaType {
    // For user.
    /// Segments from user elf file, e.g. text, rodata, data, bss
    Elf,
    /// User Stack
    Stack,
    /// User Heap
    Heap,
    /// Mmap
    Mmap,
    /// Shared memory
    Shm,
}

impl AddrSpace {
    pub const fn empty() -> Self {
        Self {
            page_table: PageTable::empty(),
            areas: BTreeMap::new(),
        }
    }

    pub fn new() -> Self {
        let page_table = PageTable::try_new().expect("create kernel page table");
        Self {
            page_table,
            areas: BTreeMap::new(),
        }
    }

    pub fn switch(&self) {
        let page_table_root = self.page_table.root_paddr().as_usize();
        unsafe {
            riscv::register::satp::set(riscv::register::satp::Mode::Sv39, 0, page_table_root >> 12);
            riscv::asm::sfence_vma_all();
        }
    }

    pub fn map_trampoline(&mut self) {
        let trampoline_start = strampoline as usize;
        let trampoline_end = etrampoline as usize;
        assert_eq!(trampoline_end - trampoline_start, PAGE_SIZE_4K);
        info!(
            "map trampoline [{:#x}, {:#x}) -> [{:#x}, {:#x}]",
            trampoline_start,
            trampoline_end,
            TRAMPOLINE,
            usize::MAX
        );
        self.page_table.map_region(
            TRAMPOLINE.into(),
            trampoline_start.into(),
            1,
            PTEFlags::R | PTEFlags::X | PTEFlags::V,
        );
    }
}

pub fn kernel_space_test() {
    let mut space = KERNEL_SPACE.lock();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert_eq!(
        space.page_table.translate(mid_text).unwrap(),
        PhysAddr::from(mid_text.as_usize()),
    );
    assert_eq!(
        space.page_table.translate(mid_rodata).unwrap(),
        PhysAddr::from(mid_rodata.as_usize()),
    );
    assert_eq!(
        space.page_table.translate(mid_data).unwrap(),
        PhysAddr::from(mid_data.as_usize()),
    );
}
