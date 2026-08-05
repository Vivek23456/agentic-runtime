use std::sync::Arc;

use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::{
    executor::Executor,
    queue::SharedReceiver,
    registry::TaskRegistry,
};

pub struct Worker<E: Executor> {
    id: usize,
    registry: TaskRegistry,
    executor: Arc<E>,
}

impl<E: Executor> Worker<E> {
    pub fn new(
    id: usize,
    registry: TaskRegistry,
    executor: Arc<E>,
) -> Self {
    Self {
        id,
        registry,
        executor,
    }
}

    pub fn start(self, receiver: SharedReceiver) -> JoinHandle<()> {
        tokio::spawn(async move {
            info!(worker = self.id, "Worker started");

            loop {
                let task = {
                    let mut rx = receiver.lock().await;
                    rx.recv().await
                };

                match task {
                    Some(task) => {
    let task_id = task.id;

    info!(
        worker = self.id,
        task_id = %task.id,
        task_name = %task.name,
        "Received task"
    );

    // Mark task as running
    if let Some(shared) = self.registry.get(task_id).await {
        let mut registry_task = shared.write().await;
        registry_task.mark_running();
    }

    // Execute task
    let result = self.executor.execute(task).await;

    // Update final task state
    if let Some(shared) = self.registry.get(task_id).await {
        let mut registry_task = shared.write().await;

        match &result {
            Ok(_) => registry_task.mark_completed(),
            Err(_) => registry_task.mark_failed(),
        }
    }

    // Log failures
    if let Err(err) = result {
        error!(
            worker = self.id,
            error = %err,
            task_id = %task_id,
            "Task execution failed"
        );
    }
}

                    None => {
                        info!(
                            worker = self.id,
                            "Queue closed. Worker shutting down."
                        );

                        break;
                    }
                }
            }
        })
    }
}