use std::sync::Arc;
use crate::{
    executor::{DefaultExecutor, Executor},
    handle::TaskHandle,
    queue::{QueueError, TaskQueue},
    registry::{SharedTask, TaskRegistry},
    tasks::Task,
    worker::Worker,
    events::EventBus,
};



pub struct RuntimeManager<E: Executor> {
    registry: TaskRegistry,
    queue: TaskQueue,
    event_bus: EventBus,
    executor: Arc<E>,
    workers: Vec<tokio::task::JoinHandle<()>>,
}

impl RuntimeManager<DefaultExecutor> {
    /// Creates a runtime using the default executor.
    pub fn new(worker_count: usize, queue_capacity: usize) -> Self {
        let registry = TaskRegistry::new();
        let event_bus = EventBus::new(1024);

        let queue = TaskQueue::new(queue_capacity);
        let executor = Arc::new(DefaultExecutor::default());

        let mut workers = Vec::with_capacity(worker_count);

        for id in 0..worker_count {
let worker = Worker::new(
    id,
    registry.clone(),
    executor.clone(),
);
            let handle = worker.start(queue.receiver());

            workers.push(handle);
        }

        Self {
    registry,
    queue,
    event_bus,
    executor,
    workers,
}
    }
}

impl<E: Executor> RuntimeManager<E> {
    /// Submit a task for asynchronous execution.
   pub async fn submit(
    &self,
    task: Task,
) -> Result<TaskHandle, QueueError> {
   let mut task = task;

task.mark_queued();

let id = task.id;

// Store a copy in the registry
self.registry.insert(task.clone()).await;

// Send the original to the queue
self.queue.push(task).await?;

Ok(TaskHandle::new(id))
}
pub async fn task(
    &self,
    id: uuid::Uuid,
) -> Option<SharedTask> {
    self.registry.get(id).await
}
pub fn event_bus(&self) -> EventBus {
    self.event_bus.clone()
}

pub async fn tasks(
    &self,
) -> Vec<SharedTask> {
    self.registry.list().await
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