use async_trait::async_trait;

use crate::ProviderError;
use domain::InferenceRequest;

use crate::InferenceResponse;
#[async_trait]
pub trait Provider: Send + Sync {
    async fn execute(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, ProviderError>;
}
use config::{
    OpenAIConfig,
};