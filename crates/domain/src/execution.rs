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

    pub task_id: Option<Uuid>,

    pub status: ExecutionStatus,

    pub started_at: Option<DateTime<Utc>>,

    pub finished_at: Option<DateTime<Utc>>,

    pub result: Option<String>,
}
impl Execution {
    pub fn new(
        agent_version_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            agent_version_id,
            task_id: None,
            status: ExecutionStatus::Pending,
            started_at: None,
            finished_at: None,
            result: None,
        }
    }

    pub fn mark_running(&mut self) {
        self.status = ExecutionStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_completed(
        &mut self,
        result: String,
    ) {
        self.status = ExecutionStatus::Completed;
        self.finished_at = Some(Utc::now());
        self.result = Some(result);
    }
    pub fn attach_task(
    &mut self,
    task_id: Uuid,
) {
    self.task_id = Some(task_id);
}

    pub fn mark_failed(&mut self) {
        self.status = ExecutionStatus::Failed;
        self.finished_at = Some(Utc::now());
    }
}