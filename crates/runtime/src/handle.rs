use uuid::Uuid;

/// Handle returned after submitting a task.
#[derive(Debug, Clone)]
pub struct TaskHandle {
    id: Uuid,
}

impl TaskHandle {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}