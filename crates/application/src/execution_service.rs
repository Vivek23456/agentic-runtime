use std::sync::Arc;

use domain::{
    Execution,
    ExecutionRegistry,
    SharedExecution,
    VersionRegistry,
};

use runtime::{
    DefaultExecutor,
    RuntimeManager,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct ExecutionService {
    execution_registry: ExecutionRegistry,
    version_registry: VersionRegistry,
    runtime: Arc<RuntimeManager<DefaultExecutor>>,
    
}

impl ExecutionService {
    pub fn new(
        execution_registry: ExecutionRegistry,
        version_registry: VersionRegistry,
        runtime: Arc<RuntimeManager<DefaultExecutor>>,
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
        // Verify the version exists.
        if self.version_registry.get(version_id).await.is_none() {
            return Err("Agent version not found");
        }

        let execution = Execution::new(version_id);

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
    ) -> Arc<RuntimeManager<DefaultExecutor>> {
        Arc::clone(&self.runtime)
    }
}