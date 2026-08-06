

use domain::ExecutionRequest;
use providers::ProviderRegistry;

use runtime::{
    BoxFuture,
    Executor,
    RuntimeError,
    Task,
};

pub struct InferenceExecutor {
    providers: ProviderRegistry,
}

impl InferenceExecutor {
    pub fn new(
        providers: ProviderRegistry,
    ) -> Self {
        Self { providers }
    }
}

impl Executor for InferenceExecutor {
    fn execute(
        &self,
        task: Task,
    ) -> BoxFuture<'static, Result<(), RuntimeError>> {

        let providers = self.providers.clone();

        Box::pin(async move {
            match task.execution {

                ExecutionRequest::Inference(request) => {

                    let provider = providers
                        .get("openai")
                        .await
                        .ok_or(
                            RuntimeError::Internal(
                                "Provider not found".to_string(),
                            ),
                        )?;

                    let response = provider
                        .execute(request)
                        .await
                        .map_err(|e| {
                            RuntimeError::Internal(
                                e.to_string(),
                            )
                        })?;

                    println!("{}", response.content);
                }
            }

            Ok(())
        })
    }
}