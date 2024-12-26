use crate::task::{Task, TidHandle};
use alloc::collections::BTreeMap;
use alloc::{boxed::Box, collections::vec_deque::VecDeque, sync::Arc};
use spin::{Mutex, Once};

pub static EXECUTOR: Once<Executor> = Once::new();

pub fn init_executor() {
    EXECUTOR.call_once(|| Executor::new());
}

pub struct Executor {
    ready: Arc<Mutex<VecDeque<Box<Task>>>>,
    pending: Arc<Mutex<BTreeMap<TidHandle, Box<Task>>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            ready: Arc::new(Mutex::new(VecDeque::new())),
            pending: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn add(&self, task: Box<Task>) {
        self.ready.lock().push_back(task);
    }

    pub fn fetch(&self) -> Option<Box<Task>> {
        self.ready.lock().pop_front()
    }
}
