use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::tasks::Task;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use crate::Task;

    #[tokio::test]
    async fn push_and_receive_task() {
        let queue = TaskQueue::new(8);

        let task = Task::new(
            "example",
            json!({"hello": "world"}),
        );

        queue.push(task).await.unwrap();

        let receiver = queue.receiver();

        let received = {
            let mut rx = receiver.lock().await;
            rx.recv().await
        };

        assert!(received.is_some());
    }
}