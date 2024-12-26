use crate::task::{Task, TidHandle};
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::task::Wake;
use spin::Mutex;

pub struct TaskWaker {
    ready: Arc<Mutex<VecDeque<Box<Task>>>>,
    pending: Arc<Mutex<BTreeMap<TidHandle, Box<Task>>>>,
    target: TidHandle,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        if let Some(task) = self.pending.lock().remove(&self.target) {
            self.ready.lock().push_back(task);
        }
    }
}
