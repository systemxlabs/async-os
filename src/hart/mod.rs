use core::arch::asm;

use riscv::register::sstatus::{self, FS};

use crate::config::MAX_HARTS;

const HART_EACH: Hart = Hart::new();
static mut HARTS: [Hart; MAX_HARTS] = [HART_EACH; MAX_HARTS];

pub struct Hart {
    hart_id: usize,
}

impl Hart {
    pub const fn new() -> Self {
        Self { hart_id: 0 }
    }
}

pub fn init(hart_id: usize) {
    unsafe {
        set_local_hart(hart_id);
        // sstatus::set_fs(FS::Initial);
    }
}

unsafe fn get_hart(hart_id: usize) -> &'static mut Hart {
    &mut HARTS[hart_id]
}

pub unsafe fn set_local_hart(hart_id: usize) {
    let hart = get_hart(hart_id);
    hart.hart_id = hart_id;
    let hart_addr = hart as *const _ as usize;
    asm!("mv tp, {}", in(reg) hart_addr);
}

pub fn local_hart() -> &'static mut Hart {
    unsafe {
        let tp: usize;
        asm!("mv {}, tp", out(reg) tp);
        &mut *(tp as *mut Hart)
    }
}
