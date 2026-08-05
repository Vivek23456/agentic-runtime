use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum AgentFramework {
    LangGraph,
    CrewAI,
    AutoGen,
    OpenAIAgents,
    PydanticAI,
    Custom,
}