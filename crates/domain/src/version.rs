use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentVersion {
    pub id: Uuid,

    pub agent_id: Uuid,

    pub version: String,

    pub description: String,

    pub prompt: String,

    pub model: String,

    pub temperature: f32,

    pub created_at: DateTime<Utc>,
}
impl AgentVersion {
    pub fn new(
        agent_id: Uuid,
        version: String,
        description: String,
        prompt: String,
        model: String,
        temperature: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            agent_id,
            version,
            description,
            prompt,
            model,
            temperature,
            created_at: Utc::now(),
        }
    }
}