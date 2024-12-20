use spin::Mutex;

use crate::allocator::RecycleAllocator;

pub static TID_ALLOCATOR: Mutex<RecycleAllocator> = Mutex::new(RecycleAllocator::new(1));

#[derive(Debug)]
pub struct TidHandle(pub usize);

impl Drop for TidHandle {
    fn drop(&mut self) {
        TID_ALLOCATOR.lock().dealloc(self.0);
    }
}

pub fn alloc_tid() -> TidHandle {
    TidHandle(TID_ALLOCATOR.lock().alloc())
}