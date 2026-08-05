use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,
    Queued,
    Running,
    Retrying,
    Completed,
    Failed,
    Cancelled,
}
impl TaskStatus {
    pub fn can_transition_to(self, next: TaskStatus) -> bool {
        matches!(
            (self, next),
            (TaskStatus::Created, TaskStatus::Queued)
                | (TaskStatus::Queued, TaskStatus::Running)
                | (TaskStatus::Queued, TaskStatus::Cancelled)
                | (TaskStatus::Running, TaskStatus::Completed)
                | (TaskStatus::Running, TaskStatus::Failed)
                | (TaskStatus::Running, TaskStatus::Cancelled)
                | (TaskStatus::Failed, TaskStatus::Retrying)
                | (TaskStatus::Retrying, TaskStatus::Queued)
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_can_transition_to_queued() {
        assert!(TaskStatus::Created.can_transition_to(TaskStatus::Queued));
    }

    #[test]
    fn queued_can_transition_to_running() {
        assert!(TaskStatus::Queued.can_transition_to(TaskStatus::Running));
    }

    #[test]
    fn completed_is_terminal() {
        assert!(TaskStatus::Completed.is_terminal());
    }

    #[test]
    fn cancelled_is_terminal() {
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn invalid_transition_should_fail() {
        assert!(
            !TaskStatus::Completed.can_transition_to(TaskStatus::Running)
        );
    }
}