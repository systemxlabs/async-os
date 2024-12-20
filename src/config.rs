pub const PAGE_SIZE_4K: usize = 0x1000;
pub const MAX_HARTS: usize = 8;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE_4K + 1;

pub const BOOT_STACK_SIZE: usize = 64 * 1024; // 64KB
pub const KERNEL_STACK_SIZE: usize = 2 * 1024 * 1024; // 1MB
