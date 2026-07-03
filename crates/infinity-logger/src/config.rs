//! 日志配置类型。
//!
//! 所有 Builder 方法最终都会修改 [`LoggerConfig`]，初始化时统一消费此配置。

use serde::{Deserialize, Serialize};

/// Logger配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggerConfig {
    /// 默认日志级别
    pub level: LogLevel,

    /// console 配置
    pub console: ConsoleConfig,

    /// 文件日志配置
    pub file: FileConfig,

    /// JSON 日志配置
    pub json: JsonConfig,

    /// 日志格式
    pub format: FormatConfig,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console: ConsoleConfig::default(),
            file: FileConfig::default(),
            json: JsonConfig::default(),
            format: FormatConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConsoleConfig {
    pub enabled: bool,

    /// 是否启用 ANSI 颜色
    pub ansi: bool,

    pub target: bool,

    pub file: bool,

    pub line_number: bool,

    pub thread_name: bool,

    pub thread_id: bool,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ansi: true,
            target: true,
            file: false,
            line_number: false,
            thread_name: false,
            thread_id: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FileConfig {
    pub enabled: bool,

    pub directory: String,

    pub filename: String,

    pub rotation: Rotation,

    pub max_file: usize,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            directory: "./logs".into(),
            filename: "application".into(),
            rotation: Rotation::Daily,
            max_file: 30,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rotation {
    Never,

    Minutely,

    Hourly,

    #[default]
    Daily,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct JsonConfig {
    pub enabled: bool,

    pub pretty: bool,

    pub flatten: bool,
}

impl Default for JsonConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            pretty: false,
            flatten: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    pub with_target: bool,

    pub with_file: bool,

    pub with_line_number: bool,

    pub with_thread_name: bool,

    pub with_thread_id: bool,

    pub with_level: bool,

    pub with_timer: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            with_target: true,
            with_file: true,
            with_line_number: true,
            with_thread_name: true,
            with_thread_id: false,
            with_level: true,
            with_timer: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(config.console.enabled);
        assert!(!config.file.enabled);
        assert!(!config.json.enabled);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Trace.as_str(), "trace");
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Warn.as_str(), "warn");
        assert_eq!(LogLevel::Error.as_str(), "error");
    }

    #[test]
    fn test_default_rotation() {
        assert_eq!(Rotation::default(), Rotation::Daily);
    }

    #[test]
    fn test_default_file_config() {
        let config = FileConfig::default();
        assert_eq!(config.directory, "./logs");
        assert_eq!(config.filename, "application");
        assert_eq!(config.max_file, 30);
    }

    #[test]
    fn test_config_serialization() {
        let config = LoggerConfig::default();
        let toml = toml::to_string(&config).expect("serialize");
        assert!(toml.contains("level = \"info\""));
    }

    #[test]
    fn test_config_deserialization() {
        let toml = r#"
            level = "debug"

            [console]
            enabled = true
        "#;
        let config: LoggerConfig = toml::from_str(toml).expect("deserialize");
        assert_eq!(config.level, LogLevel::Debug);
        assert!(config.console.enabled);
    }

    #[test]
    fn test_config_roundtrip() {
        let config = LoggerConfig::default();
        let toml = toml::to_string(&config).expect("serialize");
        let parsed: LoggerConfig = toml::from_str(&toml).expect("deserialize");
        assert_eq!(parsed.level, config.level);
        assert_eq!(parsed.file.rotation, config.file.rotation);
    }
}
