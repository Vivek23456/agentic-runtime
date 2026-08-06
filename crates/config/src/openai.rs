
use std::time::Duration;

#[derive(Debug, Clone)]


pub struct OpenAIConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl OpenAIConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY")
                .expect("OPENAI_API_KEY is missing"),

            model: std::env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-5".to_string()),

            base_url: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),

            timeout: Duration::from_secs(30),
        }
    }
}