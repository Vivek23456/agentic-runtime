use std::sync::Arc;
use config::OpenAIConfig;
use application::{
    AgentService,
    ExecutionService,
    InferenceExecutor,
    VersionService,
};



use domain::{
    AgentRegistry,
    ExecutionRegistry,
    VersionRegistry,
};

use providers::{
    OpenAIProvider,
    ProviderRegistry,
};

use runtime::RuntimeManager;

use crate::state::AppState;

pub async fn bootstrap() -> AppState {
    //
    // Domain Registries
    //
    let agent_registry = AgentRegistry::new();
    let version_registry = VersionRegistry::new();
    let execution_registry = ExecutionRegistry::new();

    //
    // Provider Registry
    //
    let provider_registry = ProviderRegistry::new();

    //
    // OpenAI Provider
    //
    let openai_config = OpenAIConfig::from_env();
  let openai = OpenAIProvider::new(openai_config)
    .expect("failed to initialize OpenAI provider");

provider_registry
    .register("openai", openai)
    .await;
    //
    // Runtime
    //
    let executor = Arc::new(
        InferenceExecutor::new(provider_registry.clone()),
    );

    let runtime = Arc::new(
        RuntimeManager::with_executor(
            executor,
            4,
            1024,
        ),
    );

    //
    // Services
    //
    let agent_service = AgentService::new(agent_registry);

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