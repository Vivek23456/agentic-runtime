use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::Agent;

/// Shared agent reference.
///
/// Multiple components can safely access
/// and modify the same agent.
pub type SharedAgent = Arc<RwLock<Agent>>;

#[derive(Clone, Default)]
pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<Uuid, SharedAgent>>>,
}

impl AgentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an agent into the registry.
    pub async fn insert(&self, agent: Agent) {
        let id = agent.id;

        let shared = Arc::new(RwLock::new(agent));

        let mut agents = self.agents.write().await;

        agents.insert(id, shared);
    }

    /// Retrieve a shared agent.
    pub async fn get(
        &self,
        id: Uuid,
    ) -> Option<SharedAgent> {
        let agents = self.agents.read().await;

        agents.get(&id).cloned()
    }

    /// Remove an agent.
    pub async fn remove(
        &self,
        id: Uuid,
    ) -> Option<SharedAgent> {
        let mut agents = self.agents.write().await;

        agents.remove(&id)
    }

    /// Returns true if the agent exists.
    pub async fn contains(
        &self,
        id: Uuid,
    ) -> bool {
        let agents = self.agents.read().await;

        agents.contains_key(&id)
    }

    /// Number of agents.
    pub async fn len(&self) -> usize {
        let agents = self.agents.read().await;

        agents.len()
    }

    /// Returns a list of all agents.
    pub async fn list(&self) -> Vec<SharedAgent> {
        let agents = self.agents.read().await;

        agents.values().cloned().collect()
    }

    /// Returns true when the registry is empty.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}