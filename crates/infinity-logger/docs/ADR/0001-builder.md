# ADR-0001: Builder Pattern

**状态**: Accepted  
**日期**: 2026-06-01  
**作者**: @maintainer

---

## 背景

需要为 Infinity Logger 设计初始化 API。

可选方案：
1. 函数参数
2. 配置结构体
3. Builder 模式
4. 宏

---

## 决策

采用 **Builder 模式**。

### API 设计

```rust
Logger::builder()
    .level(LogLevel::Debug)
    .console(true)
    .file(true)
    .directory("./logs")
    .init()?;
```

---

## 理由

### 优点

1. **灵活性** - 可选参数，按需配置
2. **类型安全** - 编译期检查
3. **链式调用** - 流畅的 API
4. **可扩展** - 新增配置不破坏现有代码
5. **符合 Rust 惯例** - 常见模式

### 缺点

1. 代码量较大
2. 需要维护 Builder 和 Config 两套结构

---

## 备选方案

### 方案 1: 函数参数

```rust
init_logger(LogLevel::Debug, true, true, "./logs")?;
```

**缺点**:
- 参数过多难以记忆
- 不灵活
- 添加参数破坏兼容性

### 方案 2: 配置结构体

```rust
let config = LoggerConfig {
    level: LogLevel::Debug,
    console: true,
    file: true,
    directory: "./logs".into(),
};
init_logger(config)?;
```

**缺点**:
- 需要显式构造结构体
- 不如 Builder 流畅

### 方案 3: 宏

```rust
init_logger! {
    level: Debug,
    console: true,
    file: true,
};
```

**缺点**:
- 宏复杂难维护
- IDE 支持差
- 类型检查弱

---

## 实现细节

### Builder 结构

```rust
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    pub fn level(mut self, level: LogLevel) -> Self {
        self.config.level = level;
        self
    }
    
    pub fn init(self) -> Result<()> {
        init::init(&self.config)
    }
}
```

### 关键设计

1. **消费 self** - 避免重复初始化
2. **返回 Self** - 支持链式调用
3. **最终 init()** - 统一初始化入口

---

## 影响

- 所有用户通过 Builder 初始化
- 配置变更不破坏 API
- 示例代码统一风格

---

## 相关

- [DESIGN.md](../DESIGN.md)
- [ARCHITECTURE.md](../ARCHITECTURE.md)

---

## 修订历史

| 日期       | 变更           |
|-----------|----------------|
| 2026-06-01 | 初始决策       |
