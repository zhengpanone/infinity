# ADR-0003: Configuration Driven

**状态**: Accepted  
**日期**: 2026-06-10  
**作者**: @maintainer

---

## 背景

需要设计配置管理方式。

核心问题：
- 如何存储配置？
- 如何传递配置？
- 如何验证配置？

---

## 决策

采用 **Configuration Driven** 设计。

### 架构

```
Builder → LoggerConfig → Validation → Init
```

---

## 理由

### 单一数据源

所有配置统一存储在 `LoggerConfig`：

```rust
pub struct LoggerConfig {
    pub level: LogLevel,
    pub console: ConsoleConfig,
    pub file: FileConfig,
    pub json: JsonConfig,
    pub format: FormatConfig,
}
```

### 职责分离

```
Builder    - 构建配置
Config     - 存储配置
Init       - 消费配置
```

### 可序列化

```rust
#[derive(Serialize, Deserialize)]
pub struct LoggerConfig {
    // ...
}
```

支持：
- TOML 文件
- 环境变量
- 配置中心

---

## 配置来源

### 优先级

```
Builder > Environment > File > Default
```

### 示例

```rust
// 1. 默认配置
let config = LoggerConfig::default();

// 2. 从文件加载
let config = LoggerConfig::from_file("logger.toml")?;

// 3. Builder 覆盖
let config = Logger::builder()
    .config(config)
    .level(LogLevel::Debug)  // 覆盖
    .build();
```

---

## 配置验证

### 时机

在 `init()` 之前验证：

```rust
impl LoggerBuilder {
    pub fn init(self) -> Result<()> {
        self.validate()?;  // ← 验证
        init::init(&self.config)
    }
    
    fn validate(&self) -> Result<()> {
        // 检查目录权限
        // 检查文件名合法性
        // ...
    }
}
```

### 验证规则

1. 目录可写
2. 文件名合法
3. 配置项冲突检查

---

## 配置不可变

初始化后配置不可修改：

```rust
// ✅ 初始化前配置
let config = Logger::builder()
    .level(LogLevel::Info)
    .build();

// ❌ 初始化后不可改
// config.level = LogLevel::Debug;  // 编译错误
```

**理由**: 避免运行时状态不一致。

**未来**: 通过 Reload 支持动态调整。

---

## 配置文件格式

### TOML

```toml
level = "info"

[console]
enabled = true
ansi = true

[file]
enabled = true
directory = "./logs"
```

### 加载

```rust
let config = LoggerConfig::from_file("logger.toml")?;
Logger::builder()
    .config(config)
    .init()?;
```

---

## 环境变量

### RUST_LOG

遵循 `tracing` 惯例：

```bash
RUST_LOG=debug cargo run
RUST_LOG=info,my_app=debug cargo run
```

### 自定义环境变量（规划）

```bash
LOGGER_LEVEL=debug
LOGGER_FILE_DIR=/var/log
```

---

## 影响

### 优点

1. **可测试** - 配置可独立测试
2. **可预测** - 行为完全由配置决定
3. **可扩展** - 新增配置不影响现有代码
4. **可维护** - 配置与逻辑分离

### 缺点

1. 配置结构较复杂
2. 需要维护默认值

---

## 备选方案

### 方案 1: 全局状态

```rust
// ❌ 不推荐
static CONFIG: Mutex<LoggerConfig> = ...;
```

**缺点**: 全局可变状态，难以测试。

### 方案 2: 每次传递

```rust
// ❌ 不推荐
fn log_message(config: &LoggerConfig, msg: &str) {
    // ...
}
```

**缺点**: 所有函数都需要传递配置。

---

## 相关

- [DESIGN.md](../DESIGN.md) - 设计原则
- [CONFIGURATION.md](../CONFIGURATION.md) - 配置指南

---

## 修订历史

| 日期       | 变更           |
|-----------|----------------|
| 2026-06-10 | 初始决策       |
