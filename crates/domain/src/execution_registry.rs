use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::Execution;

/// Shared execution reference.
///
/// Multiple components can safely access
/// and modify the same execution.
pub type SharedExecution = Arc<RwLock<Execution>>;

#[derive(Clone, Default)]
pub struct ExecutionRegistry {
    executions: Arc<RwLock<HashMap<Uuid, SharedExecution>>>,
}

impl ExecutionRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an execution into the registry.
    pub async fn insert(&self, execution: Execution) {
        let id = execution.id;

        let shared = Arc::new(RwLock::new(execution));

        let mut executions = self.executions.write().await;

        executions.insert(id, shared);
    }

    /// Retrieve a shared execution.
    pub async fn get(
        &self,
        id: Uuid,
    ) -> Option<SharedExecution> {
        let executions = self.executions.read().await;

        executions.get(&id).cloned()
    }

    /// Remove an execution.
    pub async fn remove(
        &self,
        id: Uuid,
    ) -> Option<SharedExecution> {
        let mut executions = self.executions.write().await;

        executions.remove(&id)
    }

    /// Returns true if the execution exists.
    pub async fn contains(
        &self,
        id: Uuid,
    ) -> bool {
        let executions = self.executions.read().await;

        executions.contains_key(&id)
    }

    /// Number of executions.
    pub async fn len(&self) -> usize {
        let executions = self.executions.read().await;

        executions.len()
    }

    /// Returns a list of all executions.
    pub async fn list(&self) -> Vec<SharedExecution> {
        let executions = self.executions.read().await;

        executions.values().cloned().collect()
    }

    /// Returns true when the registry is empty.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}