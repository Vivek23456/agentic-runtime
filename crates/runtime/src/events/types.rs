use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeEvent {
    TaskCreated {
        task_id: Uuid,
    },

    TaskQueued {
        task_id: Uuid,
    },

    TaskRunning {
        task_id: Uuid,
    },

    TaskCompleted {
        task_id: Uuid,
    },

    TaskFailed {
        task_id: Uuid,
    },
}