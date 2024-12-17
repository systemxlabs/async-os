mod frame;
mod heap;

pub use frame::*;
pub use heap::*;

pub fn init() {
    init_frame_allocator();
    init_heap_allocator();
}
