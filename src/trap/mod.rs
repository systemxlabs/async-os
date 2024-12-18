mod context;
mod kernel_trap;
mod user_trap;

pub use context::*;
pub use kernel_trap::*;
pub use user_trap::*;

pub fn init() {
    set_kernel_trap();
    // kernel_trap_test();
}
