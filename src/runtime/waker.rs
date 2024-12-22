use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::task::Wake;
use spin::Mutex;
use crate::task::{Task, TidHandle};

pub struct TaskWaker {
    ready: Arc<Mutex<VecDeque<Task>>>,
    pending: Arc<Mutex<BTreeMap<TidHandle, Task>>>,
    target: TidHandle,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        if let Some(task) = self.pending.lock().remove(&self.target) {
            self.ready.lock().push_back(task);
        }
    }
}