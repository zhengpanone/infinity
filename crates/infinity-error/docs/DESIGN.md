# Infinity Error 设计说明

> 项目：infinity-error
> 状态：初始稳定 API
> Edition：Rust 2024
> MSRV：Rust 1.88+
> License：MIT

## 1. 愿景

在数据库、缓存、Web、认证、网关等模块继续扩展之前，Infinity 需要一套共享的错误语义。`infinity-error` 负责提供这套语义，但不接管各个模块自己的恢复策略、重试策略或业务判断。

## 2. 目标

- 提供统一的 `InfinityError` 枚举。
- 提供工作区通用的 `Result<T>` 类型别名。
- 通过稳定的 `ErrorKind` 对错误进行分类。
- 为后续 API 响应提供默认状态码映射。
- 保持依赖最少，让所有 crate 都可以安全依赖它。
- 不绑定 `sqlx`、Redis、Axum、Tonic 或应用 crate。

## 3. 非目标

- 不实现 HTTP 响应渲染。
- 不决定数据库或缓存操作是否重试。
- 不替代领域层自己的校验逻辑。
- 不依赖更高层的 Infinity crate。

## 4. 架构

```text
应用 / 基础设施 crate
        |
        | 在 crate 边界处映射本地错误
        v
InfinityError
        |
        +-- ErrorKind
        +-- code()
        +-- status_code()
        +-- Display / Error
```

## 5. 公共 API

公共 API 有意保持很小：

```rust
pub type Result<T> = std::result::Result<T, InfinityError>;

pub enum ErrorKind { /* 稳定分类；含 ALL 与 from_code */ }

pub enum InfinityError { /* 工作区错误 */ }

/// 在 crate 边界把第三方 Result 转换为工作区 Result，保留类型化来源链。
pub trait ResultExt<T> { /* context / with_context */ }

// 函数内提前返回错误的辅助宏
macro_rules! bail { /* ... */ }
macro_rules! ensure { /* ... */ }
```

各个 crate 在接入共享错误模型后，可以从公开 API 返回 `infinity_error::Result<T>`，
并在边界处用 `ResultExt` 或 `bail!` / `ensure!` 简化转换。

## 6. 错误分类

| 分类 | 用途 |
| ---- | ---- |
| `Config` | 配置加载、解析或校验 |
| `Logger` | 日志初始化和日志配置 |
| `Database` | 数据库连接、查询、连接池、迁移 |
| `Cache` | 缓存连接、序列化、后端失败 |
| `Auth` | 认证和授权 |
| `Web` | HTTP 路由、提取器、响应构造 |
| `Validation` | 调用方提供了无效输入 |
| `NotFound` | 资源不存在 |
| `Conflict` | 状态冲突或资源重复 |
| `Unauthorized` | 缺少凭证或凭证无效 |
| `Forbidden` | 凭证有效但无权访问 |
| `Io` | 文件系统或进程 IO |
| `Parse` | 结构化数据解析失败 |
| `Unsupported` | 功能不可用 |
| `Internal` | 非预期的服务端失败 |
| `Message` | 通用兜底错误 |

## 7. 依赖策略

`infinity-error` 可以依赖 `thiserror` 这类小型基础 crate，但不能依赖预期会反向依赖它的应用 crate 或基础设施 crate。

第三方错误转换应放在已经持有该依赖的边界处。例如，`infinity-database` 可以把 `sqlx::Error` 映射为 `InfinityError::database(...)`。

## 8. HTTP 映射

`ErrorKind::status_code()` 返回供后续 Web 层使用的默认状态码。当前映射保持保守：

- 调用方或输入错误：400、401、403、404、409
- 不支持的功能：501
- 基础设施或内部错误：500

后续 `infinity-web` 可以在此基础上封装 Axum 响应类型。

## 9. 测试策略

测试覆盖：

- 构造函数到错误分类的映射
- 稳定错误码
- 默认 HTTP 状态码
- 展示文本格式
- IO 错误的 source 保留
- 字符串转换行为

## 10. 演进策略

在 `1.0` 之前可以继续新增变体。公共 API 冻结后，应尽量保持既有 `ErrorKind::code()` 稳定，谨慎增加构造函数或元数据。
