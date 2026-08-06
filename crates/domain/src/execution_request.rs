use serde::{Deserialize, Serialize};

use crate::InferenceRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionRequest {
    Inference(InferenceRequest),
}