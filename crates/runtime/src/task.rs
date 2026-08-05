use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::state::TaskStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus,
    pub payload: Value,
    pub priority: u8,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(name: impl Into<String>, payload: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            status: TaskStatus::Created,
            payload,
            priority: 0,
            retry_count: 0,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        }
    }
}
impl Task {
    pub fn mark_queued(&mut self) {
        self.status = TaskStatus::Queued;
    }

    pub fn mark_running(&mut self) {
        self.status = TaskStatus::Running;
    }

    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }

    pub fn mark_failed(&mut self) {
        self.status = TaskStatus::Failed;
    }

    pub fn mark_cancelled(&mut self) {
        self.status = TaskStatus::Cancelled;
    }
}