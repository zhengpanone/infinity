use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// AI 提供商配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AiConfig {
    pub provider: String,

    #[serde(default)]
    pub api_key: Option<String>,

    #[serde(default)]
    pub base_url: Option<String>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".into(),
            api_key: None,
            base_url: None,
        }
    }
}

impl Validate for AiConfig {
    fn validate(&self) -> Result<()> {
        if self.provider.trim().is_empty() {
            return Err(ConfigError::validation("ai.provider must not be empty"));
        }
        Ok(())
    }
}
