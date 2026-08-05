use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::{
    executor::Executor,
    queue::SharedReceiver,
};

pub struct Worker {
    id: usize,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn start(
        self,
        receiver: SharedReceiver,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            info!("Worker {} started.", self.id);

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

                        if let Err(err) = Executor::execute(task).await {
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