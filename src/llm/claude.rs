use crate::config::LLMConfig;

pub fn query_claude(
    config: &LLMConfig,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Placeholder implementation for OpenAI API call
    Ok(format!("OpenAI response for prompt '{}'", prompt))
}
