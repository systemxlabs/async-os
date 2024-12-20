use alloc::vec::Vec;

pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    pub const fn new(init_val: usize) -> Self {
        RecycleAllocator {
            current: init_val,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }
    pub fn dealloc(&mut self, id: usize) {
        assert!(id < self.current);
        assert!(
            !self.recycled.iter().any(|rid| *rid == id),
            "id {} has been deallocated!",
            id
        );
        self.recycled.push(id);
    }
}
