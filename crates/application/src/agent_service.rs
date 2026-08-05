use domain::{
    Agent,
    AgentFramework,
    AgentRegistry,
    ProgrammingLanguage,
};

#[derive(Clone)]
pub struct AgentService {
    registry: AgentRegistry,
}

impl AgentService {
    pub fn new(registry: AgentRegistry) -> Self {
        Self { registry }
    }

    pub async fn create_agent(
        &self,
        name: String,
        description: String,
        framework: AgentFramework,
        language: ProgrammingLanguage,
        repository: Option<String>,
    ) -> Agent {
        let agent = Agent::new(
            name,
            description,
            framework,
            language,
            repository,
        );

        self.registry.insert(agent.clone()).await;

        agent
    }

    pub async fn list_agents(
        &self,
    ) -> Vec<domain::SharedAgent> {
        self.registry.list().await
    }

    pub async fn get_agent(
        &self,
        id: uuid::Uuid,
    ) -> Option<domain::SharedAgent> {
        self.registry.get(id).await
    }
}