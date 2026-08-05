use domain::{
    AgentVersion,
    VersionRegistry,
    SharedVersion,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct VersionService {
    registry: VersionRegistry,
}

impl VersionService {
    pub fn new(
        registry: VersionRegistry,
    ) -> Self {
        Self { registry }
    }

    pub async fn create_version(
        &self,
        agent_id: Uuid,
        version: String,
        description: String,
        prompt: String,
        model: String,
        temperature: f32,
    ) -> AgentVersion {

        let version = AgentVersion::new(
            agent_id,
            version,
            description,
            prompt,
            model,
            temperature,
        );

        self.registry
            .insert(version.clone())
            .await;

        version
    }

    pub async fn get_version(
        &self,
        id: Uuid,
    ) -> Option<SharedVersion> {
        self.registry.get(id).await
    }

    pub async fn list_versions(
        &self,
    ) -> Vec<SharedVersion> {
        self.registry.list().await
    }
}