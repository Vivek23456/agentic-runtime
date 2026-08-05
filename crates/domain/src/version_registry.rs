use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::AgentVersion;

pub type SharedVersion = Arc<RwLock<AgentVersion>>;

#[derive(Clone, Default)]
pub struct VersionRegistry {
    versions: Arc<RwLock<HashMap<Uuid, SharedVersion>>>,
}

impl VersionRegistry {
    pub fn new() -> Self {
        Self {
            versions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(
        &self,
        version: AgentVersion,
    ) {
        let id = version.id;

        self.versions
            .write()
            .await
            .insert(id, Arc::new(RwLock::new(version)));
    }

    pub async fn get(
        &self,
        id: Uuid,
    ) -> Option<SharedVersion> {
        self.versions
            .read()
            .await
            .get(&id)
            .cloned()
    }

    pub async fn list(
        &self,
    ) -> Vec<SharedVersion> {
        self.versions
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }
}