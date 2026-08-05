use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::task::Task;

pub type SharedReceiver = Arc<Mutex<mpsc::Receiver<Task>>>;

#[derive(Debug)]
pub enum QueueError {
    Closed,
}

#[derive(Clone)]
pub struct TaskQueue {
    sender: mpsc::Sender<Task>,
    receiver: SharedReceiver,
}

impl TaskQueue {
    /// Creates a new bounded task queue.
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);

        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    /// Push a task into the queue.
    pub async fn push(&self, task: Task) -> Result<(), QueueError> {
        self.sender
            .send(task)
            .await
            .map_err(|_| QueueError::Closed)
    }

    /// Returns a clone of the sender.
    pub fn sender(&self) -> mpsc::Sender<Task> {
        self.sender.clone()
    }

    /// Returns the shared receiver for workers.
    pub fn receiver(&self) -> SharedReceiver {
        Arc::clone(&self.receiver)
    }
}