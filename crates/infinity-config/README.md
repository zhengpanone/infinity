# Infinity Config

Infinity 工作区的分层配置加载库。

`infinity-config` 的入口风格与 `infinity-logger` 保持一致：
使用 `Config::builder()` 进行链式配置，使用 `Config::init()` 做默认初始化，
使用 `Config::from_config(...)` 直接接入已经构造好的配置值。

## 特性

- 使用 `serde` 的类型化配置
- 支持 TOML / YAML / JSON 单文件加载
- 支持基础 TOML + 环境覆盖文件
- 支持进程级只读配置单例
- Builder 风格 API
- 统一的 `Validate` 校验机制

## 安装

```toml
[dependencies]
infinity-config = "0.1"
```

## 快速开始

```rust
use infinity_config::Config;

fn main() -> Result<(), infinity_config::ConfigError> {
    let config = Config::builder()
        .dir("configs")
        .init()?;

    println!("{}:{}", config.server.host, config.server.port);
    Ok(())
}
```

## Builder 用法

```rust
use infinity_config::{Config, Environment};

fn main() -> Result<(), infinity_config::ConfigError> {
    let config = Config::builder()
        .dir("configs")
        .env(Environment::Prod)
        .build()?;

    assert_eq!(config.server.port, 80);
    Ok(())
}
```

## 入口

```rust
use infinity_config::{Config, config::AppConfig};

# fn main() -> Result<(), infinity_config::ConfigError> {
// 每个进程只选择一种初始化路径。
Config::init()?;

let current = Config::current()?;
println!("{}", current.app.name);

let _typed = AppConfig::default();
# Ok(())
# }
```

## 配置文件

目录加载会先读取 `application.toml`，再按需叠加 `application-<env>.toml`。

```toml
[app]
name = "Infinity"
version = "0.1.0"

[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://postgres:postgres@localhost/infinity"
max_connections = 10

[redis]
host = "127.0.0.1"
port = 6379

[ai]
provider = "openai"
```

环境后缀对应关系：

- `Environment::Dev` -> `application-dev.toml`
- `Environment::Test` -> `application-test.toml`
- `Environment::Prod` -> `application-prod.toml`

## 校验

所有类型化配置都实现了 `Validate`。`Config::builder().init()` 和
`ConfigManager::init(...)` 都会在写入全局单例前执行校验。

## 开发

```bash
cargo fmt -p infinity-config
cargo test -p infinity-config
cargo test
```
