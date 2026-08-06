use std::sync::Arc;

use crate::{
    events::EventBus,
    executor::{DefaultExecutor, Executor},
    handle::TaskHandle,
    queue::{QueueError, TaskQueue},
    registry::{SharedTask, TaskRegistry},
    tasks::Task,
    worker::Worker,
};

pub struct RuntimeManager<E: Executor> {
    registry: TaskRegistry,
    queue: TaskQueue,
    event_bus: EventBus,
    executor: Arc<E>,
    workers: Vec<tokio::task::JoinHandle<()>>,
}

impl RuntimeManager<DefaultExecutor> {
    pub fn new(
        worker_count: usize,
        queue_capacity: usize,
    ) -> Self {
        Self::with_executor(
            Arc::new(DefaultExecutor::default()),
            worker_count,
            queue_capacity,
        )
    }
}

impl<E: Executor> RuntimeManager<E> {
    pub fn with_executor(
        executor: Arc<E>,
        worker_count: usize,
        queue_capacity: usize,
    ) -> Self {
        let registry = TaskRegistry::new();
        let event_bus = EventBus::new(1024);

        let queue = TaskQueue::new(queue_capacity);

        let mut workers = Vec::with_capacity(worker_count);

        for id in 0..worker_count {
            let worker = Worker::new(
                id,
                registry.clone(),
                event_bus.clone(),
                executor.clone(),
            );

            workers.push(worker.start(queue.receiver()));
        }

        Self {
            registry,
            queue,
            event_bus,
            executor,
            workers,
        }
    }

    pub async fn submit(
        &self,
        task: Task,
    ) -> Result<TaskHandle, QueueError> {
        let mut task = task;

        task.mark_queued();

        let id = task.id;

        self.registry.insert(task.clone()).await;

        self.queue.push(task).await?;

        Ok(TaskHandle::new(id))
    }

    pub async fn task(
        &self,
        id: uuid::Uuid,
    ) -> Option<SharedTask> {
        self.registry.get(id).await
    }

    pub async fn tasks(
        &self,
    ) -> Vec<SharedTask> {
        self.registry.list().await
    }

    pub fn event_bus(&self) -> EventBus {
        self.event_bus.clone()
    }

    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    pub fn executor(&self) -> Arc<E> {
        Arc::clone(&self.executor)
    }
}