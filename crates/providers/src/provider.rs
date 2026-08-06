use async_trait::async_trait;

use crate::ProviderError;
use domain::{InterferenceRequest,InferenceRequest,Usage};

#[async_trait]
pub trait Provider: Send + Sync {
    async fn execute(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, ProviderError>;
}
pub enum ProviderConfig {
    OpenAI(OpenAIConfig),
    Anthropic(AnthropicConfig),
    Gemini(GeminiConfig),
}