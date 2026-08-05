use std::sync::Arc;
use domain::AgentRegistry;
use runtime::{DefaultExecutor, RuntimeManager};

#[derive(Clone)]
pub struct AppState {
    pub runtime: Arc<RuntimeManager<DefaultExecutor>>,
    pub agents: AgentRegistry,
}