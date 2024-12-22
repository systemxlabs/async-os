use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::task::{Task, TidHandle};

pub static EXECUTOR: Executor = Executor::new();

pub struct Executor {
    ready: Arc<Mutex<VecDeque<Task>>>,
    pending: Arc<Mutex<BTreeMap<TidHandle, Task>>>,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            ready: Arc::new(Mutex::new(VecDeque::new())),
            pending: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn add(&self, task: Task) {
        self.ready.lock().push_back(task);
    }

    pub fn fetch(&self) -> Option<Task> {
        self.ready.lock().pop_front()
    }
}