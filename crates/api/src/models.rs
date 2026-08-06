use serde::{Deserialize, Serialize};

use uuid::Uuid;




use domain::{AgentFramework, ProgrammingLanguage};

#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub description: String,
    pub framework: AgentFramework,
    pub language: ProgrammingLanguage,
    pub repository: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateAgentResponse {
    pub agent_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub framework: AgentFramework,
    pub language: ProgrammingLanguage,
    pub repository: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub agent_id: Uuid,
    pub version: String,
    pub description: String,
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub struct CreateVersionResponse {
    pub version_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub version: String,
    pub description: String,
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
}
#[derive(Debug, Deserialize)]
pub struct CreateExecutionRequest {
    pub version_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateExecutionResponse {
    pub execution_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResponse {
    pub id: Uuid,
    pub version_id: Uuid,
    pub task_id: Option<Uuid>,
    pub status: String,
    pub result: Option<String>,
}