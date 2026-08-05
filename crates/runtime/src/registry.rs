use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::task::Task;

#[derive(Clone, Default)]
pub struct TaskRegistry {
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, task: Task) {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id, task);
    }

    pub async fn get(&self, id: Uuid) -> Option<Task> {
        let tasks = self.tasks.read().await;
        tasks.get(&id).cloned()
    }

    pub async fn update(&self, task: Task) {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id, task);
    }

    pub async fn contains(&self, id: Uuid) -> bool {
        let tasks = self.tasks.read().await;
        tasks.contains_key(&id)
    }

    pub async fn len(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks.len()
    }

    pub async fn remove(&self, id: Uuid) -> Option<Task> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(&id)
    }
}