use std::fmt;

/// 用于选择环境覆盖配置的运行环境。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Dev,
    Test,
    Prod,
}

impl Environment {
    /// 从 `APP_ENV`，再从 `RUN_MODE` 检测运行环境。
    ///
    /// 未知或未设置时回退到 [`Environment::Dev`]。
    pub fn detect() -> Self {
        std::env::var("APP_ENV")
            .or_else(|_| std::env::var("RUN_MODE"))
            .ok()
            .map(|s| Self::from_str_lossy(&s))
            .unwrap_or_default()
    }

    /// 解析环境值，失败时回退到 [`Environment::Dev`]。
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "prod" | "production" | "release" => Environment::Prod,
            "test" | "testing" => Environment::Test,
            _ => Environment::Dev,
        }
    }

    /// 返回 `application-<env>.toml` 中使用的后缀。
    pub fn as_str(self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Test => "test",
            Environment::Prod => "prod",
        }
    }

    pub fn is_prod(self) -> bool {
        matches!(self, Environment::Prod)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
