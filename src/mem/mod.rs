mod addr;
mod page_table;
mod pte;
mod space;

pub use addr::*;
pub use page_table::*;
pub use pte::*;
pub use space::*;

pub fn init() {
    init_kernel_space();
    // swich_kernel_space();
}

pub fn swich_kernel_space() {
    KERNEL_SPACE.lock().switch();
}
