use core::cell::SyncUnsafeCell;

use crate::{mem::AddrSpace, trap::TrapContext};

use super::TidHandle;

pub struct Task {
    tid: TidHandle,
    space: AddrSpace,
    pub trap_context: SyncUnsafeCell<TrapContext>,
}

impl Task {
    pub fn trap_context_mut(&self) -> &mut TrapContext {
        unsafe { &mut *self.trap_context.get() }
    }
}
