use std::sync::Arc;

use crate::{
    executor::{DefaultExecutor, Executor},
    queue::TaskQueue,
    task::Task,
    worker::Worker,
    queue::QueueError,
};

pub struct RuntimeManager<E: Executor> {
    queue: TaskQueue,
    executor: Arc<E>,
    workers: Vec<tokio::task::JoinHandle<()>>,
}

impl RuntimeManager<DefaultExecutor> {
    /// Creates a runtime using the default executor.
    pub fn new(worker_count: usize, queue_capacity: usize) -> Self {
        let queue = TaskQueue::new(queue_capacity);
        let executor = Arc::new(DefaultExecutor::default());

        let mut workers = Vec::with_capacity(worker_count);

        for id in 0..worker_count {
            let worker = Worker::new(id, executor.clone());

            let handle = worker.start(queue.receiver());

            workers.push(handle);
        }

        Self {
            queue,
            executor,
            workers,
        }
    }
}

impl<E: Executor> RuntimeManager<E> {
    /// Submit a task for asynchronous execution.
    pub async fn submit(&self, task: Task) -> Result<(), QueueError> {
        self.queue.push(task).await
    }

    /// Returns the number of worker tasks spawned.
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    /// Returns a clone of the executor.
    pub fn executor(&self) -> Arc<E> {
        Arc::clone(&self.executor)
    }
}