use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub id: Uuid,

    pub agent_version_id: Uuid,

    pub status: ExecutionStatus,

    pub started_at: Option<DateTime<Utc>>,

    pub finished_at: Option<DateTime<Utc>>,
}