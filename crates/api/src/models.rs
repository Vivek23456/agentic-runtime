use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub payload: Value,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub task_id: Uuid,
}