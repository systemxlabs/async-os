use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use spin::Mutex;

use super::Task;

pub static TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());

pub struct TaskManager {
    queue: VecDeque<Arc<Task>>,
}

impl TaskManager {
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<Task>) {
        self.queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<Task>> {
        self.queue.pop_front()
    }
}