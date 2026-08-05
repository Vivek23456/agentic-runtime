use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::Task;

/// Shared task reference.
///
/// Multiple components can safely access
/// and modify the same task.
pub type SharedTask = Arc<RwLock<Task>>;

#[derive(Clone, Default)]
pub struct TaskRegistry {
    tasks: Arc<RwLock<HashMap<Uuid, SharedTask>>>,
}

impl TaskRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a task into the registry.
    pub async fn insert(&self, task: Task) {
        let id = task.id;

        let shared = Arc::new(RwLock::new(task));

        let mut tasks = self.tasks.write().await;

        tasks.insert(id, shared);
    }

    /// Retrieve a shared task.
    pub async fn get(
        &self,
        id: Uuid,
    ) -> Option<SharedTask> {
        let tasks = self.tasks.read().await;

        tasks.get(&id).cloned()
    }

    /// Remove a task.
    pub async fn remove(
        &self,
        id: Uuid,
    ) -> Option<SharedTask> {
        let mut tasks = self.tasks.write().await;

        tasks.remove(&id)
    }

    /// Returns true if the task exists.
    pub async fn contains(
        &self,
        id: Uuid,
    ) -> bool {
        let tasks = self.tasks.read().await;

        tasks.contains_key(&id)
    }

    /// Number of tasks.
    pub async fn len(&self) -> usize {
        let tasks = self.tasks.read().await;

        tasks.len()
    }

    /// Returns true when the registry is empty.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}