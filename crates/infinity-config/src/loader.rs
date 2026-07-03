use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{ConfigError, Result};

/// 支持的配置文件格式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Toml,
    Yaml,
    Json,
}

impl Format {
    /// 根据文件扩展名推断格式。
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "toml" => Some(Format::Toml),
            "yaml" | "yml" => Some(Format::Yaml),
            "json" => Some(Format::Json),
            _ => None,
        }
    }
}

/// 按指定格式反序列化字符串。
pub fn parse_str<T: DeserializeOwned>(content: &str, format: Format) -> Result<T> {
    match format {
        Format::Toml => toml::from_str(content).map_err(ConfigError::from),
        Format::Yaml => serde_yaml::from_str(content).map_err(ConfigError::from),
        Format::Json => serde_json::from_str(content).map_err(ConfigError::from),
    }
}

/// 加载并反序列化单个配置文件。
pub fn load_file<T, P>(path: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.exists() {
        return Err(ConfigError::file_not_found(path.display().to_string()));
    }
    let format = Format::from_path(path).ok_or_else(|| {
        ConfigError::InvalidFormat(format!(
            "unsupported configuration file extension: {}",
            path.display()
        ))
    })?;
    let content = std::fs::read_to_string(path)?;
    parse_str(&content, format)
}

/// 加载基础 TOML 文件，并可选合并环境覆盖文件。
pub fn load_layered_toml<T, P>(base: P, overlay: Option<P>) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let base = base.as_ref();
    if !base.exists() {
        return Err(ConfigError::file_not_found(base.display().to_string()));
    }

    let mut merged: toml::Value = toml::from_str(&std::fs::read_to_string(base)?)?;

    if let Some(overlay) = overlay {
        let overlay = overlay.as_ref();
        if overlay.exists() {
            let overlay_value: toml::Value = toml::from_str(&std::fs::read_to_string(overlay)?)?;
            merge_toml(&mut merged, &overlay_value);
        }
    }

    let merged_str =
        toml::to_string(&merged).map_err(|e| ConfigError::ParseError(e.to_string()))?;
    toml::from_str(&merged_str).map_err(ConfigError::from)
}

fn merge_toml(base: &mut toml::Value, overlay: &toml::Value) {
    match (base, overlay) {
        (toml::Value::Table(b), toml::Value::Table(o)) => {
            for (key, value) in o {
                match b.get_mut(key) {
                    Some(existing) => merge_toml(existing, value),
                    None => {
                        b.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        (base, overlay) => *base = overlay.clone(),
    }
}
