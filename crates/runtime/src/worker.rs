use std::sync::Arc;

use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::{
    executor::Executor,
    queue::SharedReceiver,
};

pub struct Worker<E: Executor> {
    id: usize,
    executor: Arc<E>,
}

impl<E: Executor> Worker<E> {
    pub fn new(id: usize, executor: Arc<E>) -> Self {
        Self { id, executor }
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
                        info!(
                            worker = self.id,
                            task_id = %task.id,
                            task_name = %task.name,
                            "Received task"
                        );

                        if let Err(err) = self.executor.execute(task).await {
                            error!(
                                worker = self.id,
                                error = %err,
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