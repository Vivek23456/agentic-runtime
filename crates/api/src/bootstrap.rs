use std::sync::Arc;

use application::{
    AgentService,
    ExecutionService,
    VersionService,
};

use domain::{
    AgentRegistry,
    ExecutionRegistry,
    VersionRegistry,
};

use runtime::{
    DefaultExecutor,
    RuntimeManager,
};

use crate::state::AppState;

pub async fn bootstrap() -> AppState {
    //
    // Domain Registries
    //
    let agent_registry = AgentRegistry::new();

    let version_registry = VersionRegistry::new();

    let execution_registry = ExecutionRegistry::new();

    //
    // Runtime
    //
    let runtime = Arc::new(
        RuntimeManager::<DefaultExecutor>::new(
            4,
            1024,
        ),
    );

    //
    // Services
    //
    let agent_service = AgentService::new(
        agent_registry,
    );

    let version_service = VersionService::new(
        version_registry.clone(),
    );

    let execution_service = ExecutionService::new(
        execution_registry,
        version_registry,
        runtime,
    );

      

    AppState {
        agent_service,
        version_service,
        execution_service,
    }
}
