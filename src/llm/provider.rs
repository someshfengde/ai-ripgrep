// generic provider for LLM
use crate::config::LLMConfig;
use crate::llm::{claude, groq, openai};

pub struct LLMProvider {
    config: LLMConfig,
}

impl LLMProvider {
    pub fn new(config: &LLMConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn query(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self.config.provider.as_str() {
            "openai" => openai::query_openai(&self.config, prompt),
            "groq" => groq::query_groq(&self.config, prompt),
            "claude" => claude::query_claude(&self.config, prompt),
            _ => Err(format!("Unsupported provider: {}", self.config.provider).into()),
        }
    }
}
