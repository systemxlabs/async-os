use core::arch::asm;

use alloc::sync::Arc;
use riscv::register::sstatus::{self, FS};

use crate::{
    config::{KERNEL_STACK_SIZE, MAX_HARTS}, task::{IdleTask, Task}, trap::TrapContext
};

const HART_EACH: Hart = Hart::empty();
static mut HARTS: [Hart; MAX_HARTS] = [HART_EACH; MAX_HARTS];

pub struct Hart {
    hart_id: usize,
    kernel_stack: [u8; KERNEL_STACK_SIZE],
    task: Option<Arc<Task>>,
    idle: IdleTask,
}

impl Hart {
    pub const fn empty() -> Self {
        Self {
            hart_id: 0,
            kernel_stack: [0; KERNEL_STACK_SIZE],
            task: None,
            idle: IdleTask::new(),
        }
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

pub fn current_task() -> Option<Arc<Task>> {
    local_hart().task.clone()
}