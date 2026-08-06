use application::{
    AgentService,
    ExecutionService,
    VersionService,
};

#[derive(Clone)]
pub struct AppState {
    pub agent_service: AgentService,
    pub version_service: VersionService,
    pub execution_service: ExecutionService,
}