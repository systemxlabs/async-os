use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use async_task::{Runnable, ScheduleInfo, Task};
use spin::Mutex;

pub static EXECUTOR: Executor = Executor::new();

pub struct Executor {
    queue: Mutex<VecDeque<Runnable>>,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push_back(&self, runnable: Runnable) {
        self.queue.lock().push_back(runnable);
    }

    pub fn run(&self) {
        while let Some(runnable) = self.queue.lock().pop_front() {
            runnable.run();
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    
    let schedule = move |runnable: Runnable| {
        EXECUTOR.push_back(runnable);
    };
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    task.detach();
}