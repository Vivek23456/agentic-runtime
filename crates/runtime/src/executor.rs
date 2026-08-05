use std::{future::Future, pin::Pin, time::Duration};

use tokio::time::sleep;
use tracing::info;

use crate::{
    error::RuntimeError,
    task::Task,
};

/// Boxed future returned by executors.
pub type BoxFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait implemented by all task executors.
pub trait Executor: Send + Sync + 'static {
    fn execute(&self, task: Task) -> BoxFuture<'static, Result<(), RuntimeError>>;
}

/// Default executor used during development.
///
/// Right now it only simulates work.
/// Later we'll replace it with real executors
/// (LLM, Python, Shell, Workflow, etc.).
#[derive(Default)]
pub struct DefaultExecutor;

impl Executor for DefaultExecutor {
    fn execute(
        &self,
        task: Task,
    ) -> BoxFuture<'static, Result<(), RuntimeError>> {
        Box::pin(async move {
            info!(
                task_id = %task.id,
                task_name = %task.name,
                "Executing task"
            );

            sleep(Duration::from_secs(2)).await;

            info!(
                task_id = %task.id,
                "Task completed"
            );

            Ok(())
        })
    }
}