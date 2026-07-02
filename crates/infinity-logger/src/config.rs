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
    pub format: FormatConfnig,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console: ConsoleConfig::default(),
            file: FileConfig::default(),
            json: JsonConfig::default(),
            format: FormatConfnig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rotation {
    Never,

    Minutely,

    Hourly,

    Daily,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Daily
    }
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
pub struct FormatConfnig {
    pub with_target: bool,

    pub with_file: bool,

    pub with_line_number: bool,

    pub with_thread_name: bool,

    pub with_thread_id: bool,

    pub with_level: bool,

    pub with_timer: bool,
}

impl Default for FormatConfnig {
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
