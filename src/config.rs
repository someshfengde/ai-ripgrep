use dirs_next::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
}

impl LLMConfig {
    pub fn new(provider: &str, model: &str, api_key: Option<&str>) -> LLMConfig {
        LLMConfig {
            provider: provider.to_string(),
            model: model.to_string(),
            api_key: api_key.map(|s| s.to_string()),
        }
    }

    pub fn get_config_path() -> PathBuf {
        let mut config_dir = config_dir().expect("Unable to find config directory");
        config_dir.push("ai-ripgrep");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect("Unable to create config directory");
        }
        config_dir.join("llm_config.json")
    }

    pub fn reset_config() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = LLMConfig::get_config_path();
        if config_path.exists() {
            fs::remove_file(config_path)?;
        }
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = LLMConfig::get_config_path();
        let config_json = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_json)?;
        Ok(())
    }

    pub fn load_from_file() -> Result<LLMConfig, Box<dyn std::error::Error>> {
        let config_path = LLMConfig::get_config_path();
        let config_json = fs::read_to_string(config_path)?;
        let config: LLMConfig = serde_json::from_str(&config_json)?;
        Ok(config)
    }
}
