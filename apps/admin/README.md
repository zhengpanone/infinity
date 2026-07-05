# Infinity Admin Server

> **Package:** `infinity-admin-server`
> **Binary:** `admin`
> **Type:** 二进制应用（Binary）

Infinity 平台的**管理后台服务**。当前为骨架示例，演示如何组合使用 Infinity 的基础库，可作为其他服务的参考模板。

---

## 目录

- [概述](#概述)
- [依赖的基础库](#依赖的基础库)
- [快速开始](#快速开始)
- [代码结构](#代码结构)
- [日志输出](#日志输出)
- [开发规范](#开发规范)
- [路线图](#路线图)

---

## 概述

`admin` 是 Infinity 工作区中的一个应用 crate（位于 `apps/admin`），负责平台的后台管理能力。

- **包名**：`infinity-admin-server`（workspace 内唯一）
- **二进制名**：`admin`（部署 / CLI 中使用的简短名）

当前实现是一个**启动骨架**，完成日志初始化、领域类型创建与启动耗时统计，展示三个基础库的组合方式。真实业务能力将在后续里程碑中填充。

---

## 依赖的基础库

| 库 | 用途 | 本服务中的用法 |
|----|------|----------------|
| [`infinity-config`](../../crates/infinity-config) | 分层配置加载与校验 | 从 `configs/` 加载 `AppConfig` |
| [`infinity-error`](../../crates/infinity-error) | 工作区统一错误 | `Result` 传播、致命错误结构化上报 |
| [`infinity-logger`](../../crates/infinity-logger) | 统一日志基础设施 | 初始化日志、输出结构化日志 |
| [`infinity-common`](../../crates/infinity-common) | 共享领域类型 | `UserId` / `TenantId` |
| [`infinity-utils`](../../crates/infinity-utils) | 无业务含义的纯工具 | 时间戳、启动耗时统计 |

依赖方向：

```
admin ──► infinity-config
      ├─► infinity-error
      ├─► infinity-logger
      ├─► infinity-common ──► infinity-utils
      └─► infinity-utils
```

`infinity-common` 单向依赖 `infinity-utils`，符合工作区的分层约定。

---

## 快速开始

### 运行

```bash
# 从工作区根目录运行
cargo run -p infinity-admin-server

# 或使用二进制名（构建后）
cargo build -p infinity-admin-server
./target/debug/admin
```

### 调整日志级别

通过 `RUST_LOG` 环境变量覆盖默认级别：

```bash
# 只看 info 及以上
RUST_LOG=info cargo run -p infinity-admin-server

# 查看某模块的 trace
RUST_LOG=admin=trace cargo run -p infinity-admin-server
```

---

## 代码结构

```
apps/admin
├── Cargo.toml        # 包定义、[[bin]] 短名、依赖
├── README.md         # 本文档
└── src
    └── main.rs       # 入口：日志初始化 + 启动流程
```

### `main.rs` 核心流程

```rust
fn main() -> ExitCode {
    // 失败时结构化上报致命错误并返回非零退出码
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            report_fatal(&err);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    // 1. 加载配置，再据此初始化日志
    let config = load_config()?;
    init_logger(config)?;

    // 2. 创建租户与默认管理员（领域类型）
    let tenant = TenantId::generate();
    let admin = Admin::new("root", tenant);

    // 3. 执行启动流程并统计耗时
    let started = infinity_utils::time::now_millis();
    bootstrap(config, &admin)?;
    let elapsed = infinity_utils::time::now_millis() - started;
    Ok(())
}
```

---

## 致命错误上报

`main` 把启动流程收敛在 `run() -> Result<()>` 中，任何步骤出错都以 `?` 向上传播，
最终由 `report_fatal` 按工作区[可观测性规范](../../crates/infinity-error/docs/OBSERVABILITY.md)
统一记录——字段名取自 `infinity_error::field`，保证与其余日志一致：

```rust
tracing::error!(
    { field::KIND }       = err.code(),          // error.kind
    { field::STATUS }     = err.status_code(),    // error.status
    { field::CLASS }      = err.class().as_str(), // error.class: client / server
    { field::ROOT_CAUSE } = %err.root_cause(),    // error.root_cause
    { field::CHAIN }      = err.chain_string(),   // error.chain
    "admin server exited with error"
);
```

配置或日志初始化阶段失败时 `tracing` 尚无 subscriber，`report_fatal` 会额外向
stderr 兜底输出，保证致命错误始终可见。

---

## 日志输出

运行后输出示例（控制台，Debug 级别）：

```text
INFO  admin: admin server starting version="0.1.1" app="infinity"
INFO  admin: admin server config loaded host="0.0.0.0" port=8080
INFO  admin: default admin account created admin_id="019f2696-…" tenant_id="019f2696-…" username="root"
DEBUG admin: running bootstrap tasks username="root"
INFO  admin: admin server ready elapsed_ms=0
```

- ID 由 `infinity-utils` 的 UUID v7 生成，天然按时间有序
- 日志使用**结构化字段**（`admin_id = ...`）而非格式化字符串

---

## 开发规范

本服务遵循工作区统一规范（详见 [CODING_STANDARD](../../crates/infinity-logger/docs/CODING_STANDARD.md)）：

- `run()` 返回 `Result`，以 `?` 传播错误；`main()` 返回 `ExitCode` 并统一上报致命错误
- 业务代码**禁止** `unwrap()` / `expect()` / `panic!()`
- 公开项与关键函数编写 RustDoc
- 结构化日志优先使用字段
- 导入顺序：std → 外部 crate → 本地

提交前检查：

```bash
cargo fmt
cargo clippy -p infinity-admin-server -- -D warnings
cargo run -p infinity-admin-server
```

---

## 路线图

当前为骨架示例，后续计划：

- [x] 接入 `infinity-config` 加载配置
- [ ] 接入 `infinity-database` 连接数据库
- [ ] 接入 `infinity-web` 提供 HTTP 接口
- [ ] 管理员认证与权限（`infinity-auth`）
- [ ] 优雅关闭与健康检查

---

**Infinity Workspace · Admin Server**
