use std::sync::atomic::{AtomicU8, Ordering};

#[repr(u8)]
pub enum TaskStatus {
    Running             = 0,
    Suspended           = 1,
    Updating     = 2
}

pub struct TaskState {
    status: AtomicU8
}

impl TaskState {
    pub fn new(initial: TaskStatus) -> Self {
        Self {
            status: AtomicU8::new(initial as u8)
        }
    }

    pub fn set(&self, status: TaskStatus) {
        self.status.store(status as u8, Ordering::Relaxed);
    }

    pub fn get(&self) -> TaskStatus {
        match self.status.load(Ordering::Relaxed) {
            0 => TaskStatus::Running,
            1 => TaskStatus::Suspended,
            2 => TaskStatus::Updating,
            _ => TaskStatus::Suspended
        }
    }
}