use std::sync::Arc;

use domain::ExecutionRequest;
use providers::Provider;

use crate::{
    error::RuntimeError,
    executor::{BoxFuture, Executor},
    tasks::Task,
};

pub struct InferenceExecutor {
    provider: Arc<dyn Provider>,
}

impl InferenceExecutor {
    pub fn new(provider: Arc<dyn Provider>) -> Self {
        Self { provider }
    }
}

impl Executor for InferenceExecutor {
    fn execute(
        &self,
        task: Task,
    ) -> BoxFuture<'static, Result<(), RuntimeError>> {
        let provider = self.provider.clone();

        Box::pin(async move {
            match task.execution {
                ExecutionRequest::Inference(request) => {
                    let response = provider
                        .execute(request)
                        .await
                        .map_err(|e| RuntimeError::Internal(e.to_string()))?;

                    println!("{}", response.content);
                }
            }

            Ok(())
        })
    }
}