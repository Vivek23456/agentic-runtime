use async_trait::async_trait;

use crate::{
    InferenceRequest,
    InferenceResponse,
    ProviderError,
};

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