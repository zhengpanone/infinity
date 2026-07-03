use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub api_key: String,
}
