mod frame;
mod heap;
mod recycle;

pub use frame::*;
pub use heap::*;
pub use recycle::*;

pub fn init() {
    init_frame_allocator();
    init_heap_allocator();
}
