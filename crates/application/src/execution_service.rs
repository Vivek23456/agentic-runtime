use std::sync::Arc;

use uuid::Uuid;

use domain::{
    Execution,
    ExecutionRegistry,
    ExecutionRequest,
    InferenceRequest,
    SharedExecution,
    VersionRegistry,
};

use runtime::{
    RuntimeManager,
    Task,
};

use crate::InferenceExecutor;

#[derive(Clone)]
pub struct ExecutionService {
    execution_registry: ExecutionRegistry,
    version_registry: VersionRegistry,
    runtime: Arc<RuntimeManager<InferenceExecutor>>,
}

impl ExecutionService {
    pub fn new(
        execution_registry: ExecutionRegistry,
        version_registry: VersionRegistry,
        runtime: Arc<RuntimeManager<InferenceExecutor>>,
    ) -> Self {
        Self {
            execution_registry,
            version_registry,
            runtime,
        }
    }

    pub async fn create_execution(
        &self,
        version_id: Uuid,
    ) -> Result<Execution, &'static str> {

        let version = self
            .version_registry
            .get(version_id)
            .await
            .ok_or("Agent version not found")?;

        let version = version.read().await;

        let inference = InferenceRequest {
            model: version.model.clone(),
            prompt: version.prompt.clone(),
            temperature: version.temperature,
        };

        let execution_request =
            ExecutionRequest::Inference(inference);

        let mut execution = Execution::new(version_id);

        let task = Task::new(
            format!("Execution {}", execution.id),
            execution_request,
        );

        let handle = self
            .runtime
            .submit(task)
            .await
            .map_err(|_| "Runtime submission failed")?;

        execution.attach_task(handle.id());

        self.execution_registry
            .insert(execution.clone())
            .await;

        Ok(execution)
    }

    pub async fn get_execution(
        &self,
        id: Uuid,
    ) -> Option<SharedExecution> {
        self.execution_registry.get(id).await
    }

    pub async fn list_executions(
        &self,
    ) -> Vec<SharedExecution> {
        self.execution_registry.list().await
    }

    pub fn runtime(
        &self,
    ) -> Arc<RuntimeManager<InferenceExecutor>> {
        Arc::clone(&self.runtime)
    }
}