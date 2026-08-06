use async_trait::async_trait;

use config::OpenAIConfig;

use openai_rust_sdk::{
    models::CreateResponseRequest,
    OpenAIClient,
};

use domain::InterferenceRequest;
use crate::{
    
    InferenceResponse,
    Provider,
    ProviderError,
    Usage,
};

pub struct OpenAIProvider {
    config: OpenAIConfig,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(
        config: OpenAIConfig,
    ) -> Result<Self, ProviderError> {

        let client = OpenAIClient::with_base_url(
            config.api_key.clone(),
            config.base_url.clone(),
        )
        .map_err(|e| ProviderError::Internal(e.to_string()))?;

        Ok(Self {
            config,
            client,
        })
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn execute(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, ProviderError> {

       async fn execute(
    &self,
    request: InferenceRequest,
) -> Result<InferenceResponse, ProviderError> {

    let sdk_request = CreateResponseRequest::new_text(
        request.model,
        request.prompt,
    );

    let sdk_response = self
        .client
        .create_response_v2(&sdk_request)
        .await
        .map_err(|e| ProviderError::Internal(e.to_string()))?;

    Ok(InferenceResponse {
        content: sdk_response.output_text(),

        usage: Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        },

        finish_reason: "stop".to_string(),
    })
}

        Ok(InferenceResponse {
            content: "OpenAI Provider Skeleton".to_string(),
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
            finish_reason: "stop".to_string(),
        })
    }
}