use crate::{mem::AddrSpace, trap::TrapContext};

use super::TidHandle;

pub struct Task {
    tid: TidHandle,
    space: AddrSpace,
    trap_context: TrapContext,
}