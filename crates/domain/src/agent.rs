 
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    AgentFramework,
    ProgrammingLanguage,
};

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub description: String,

    pub framework: AgentFramework,
    pub language: ProgrammingLanguage,

    pub repository: Option<String>,

    pub created_at: DateTime<Utc>,
}
impl Agent {
    pub fn new(
        name: String,
        description: String,
        framework: AgentFramework,
        language: ProgrammingLanguage,
        repository: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            framework,
            language,
            repository,
            created_at: Utc::now(),
        }
    }
}