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

当前实现在启动骨架之上提供一个**最小可用的 HTTP 服务**：加载配置、初始化日志、创建领域类型，
随后基于 `axum` 监听配置端口，对外暴露健康检查与示例接口，并支持优雅关闭。真实业务能力将在后续里程碑中填充。

---

## 依赖的基础库

| 库 | 用途 | 本服务中的用法 |
|----|------|----------------|
| [`infinity-config`](../../crates/infinity-config) | 分层配置加载与校验 | 从 `configs/` 加载 `AppConfig` |
| [`infinity-error`](../../crates/infinity-error) | 工作区统一错误 | `Result` 传播、致命错误结构化上报 |
| [`infinity-logger`](../../crates/infinity-logger) | 统一日志基础设施 | 初始化日志、请求追踪 Layer |
| [`infinity-web`](../../crates/infinity-web) | Web/HTTP 构建块 | `ApiError` 统一错误响应 |
| [`infinity-database`](../../crates/infinity-database) | 数据库访问层 | 连接池、迁移、`/health` 探活、`AdminRepository` 查询 |
| [`infinity-proto`](../../crates/infinity-proto) | gRPC 契约 | Greeter 服务、reflection 描述符 |
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
    ├── main.rs       # 入口：run() 编排（装配 → 领域 → 服务）+ 致命错误兜底
    ├── startup.rs    # 启动装配：配置加载、日志初始化、启动任务
    ├── telemetry.rs  # 致命错误结构化上报（report_fatal）
    ├── domain.rs     # 领域模型（Admin）
    └── server/
        ├── mod.rs    # serve_all（并发 HTTP+gRPC）+ 共享优雅关闭信号
        ├── http.rs   # axum 路由、健康检查、错误响应映射
        └── grpc.rs   # tonic Greeter、健康检查、反射
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

## HTTP 接口

启动后监听 `configs/` 中 `server.host:server.port`（默认 `0.0.0.0:8080`），
路由挂载了 `infinity-logger` 的请求追踪 Layer（自动记录 method / uri / status / 耗时）。

> 前置：需要可用的 PostgreSQL。地址取自 `configs/` 的 `[database]`（默认
> `postgres://postgres:postgres@localhost/infinity`），启动时自动运行迁移。

| 方法 | 路径 | 说明 | 响应 |
|------|------|------|------|
| GET | `/` | 服务标识 | `infinity-admin-server` |
| GET | `/health` | 健康检查（含数据库探活） | 通：`{"status":"ok","version":"0.1.1"}`；DB 不可达：`503` |
| GET | `/admins/{id}` | 按 ID 查询管理员 | 见下 |

`/health` 会对数据库执行一次 `SELECT 1` 探活，数据库不可达时返回 `503 { "code": "database" }`，
保持探针语义（区别于业务 5xx）。

`/admins/{id}` 走真实数据库查询，并演示 `infinity-web` 的错误→响应映射：

```bash
# 非法 ID → 400 validation
$ curl -s http://127.0.0.1:8080/admins//
{"status":400,"code":"validation","message":"validation error in id: must be a non-empty identifier"}

# 命中 → 200
$ curl -s http://127.0.0.1:8080/admins/<已存在的id>
{"id":"...","tenant_id":"...","username":"..."}

# 查无此人 → 404 not_found
$ curl -s http://127.0.0.1:8080/admins/does-not-exist
{"status":404,"code":"not_found","message":"not found: admin:does-not-exist"}
```

错误响应体由 `infinity_web::ApiError` 统一序列化（`status` / `code` / `message`，
携带时附 `request_id`）；服务端错误（5xx）的 `message` 会被替换为通用文案，避免泄露内部细节。

### 优雅关闭

进程收到 `Ctrl-C`（或 Unix 上的 `SIGTERM`）时停止接收新连接，等待进行中的请求完成后退出。

---

## 日志输出

运行后输出示例（控制台，Debug 级别）：

```text
INFO  admin: admin server starting version="0.1.1" app="Infinity"
INFO  admin: admin server config loaded host="0.0.0.0" port=8080
INFO  admin: default admin account created admin_id="019f2696-…" tenant_id="019f2696-…" username="root"
DEBUG admin: running bootstrap tasks username="root"
INFO  admin: admin bootstrap complete elapsed_ms=0
INFO  admin::http: admin HTTP server listening addr=0.0.0.0:8080
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

- [x] 接入 `infinity-config` 加载配置
- [x] 接入 `infinity-web` 提供 HTTP 接口
- [x] 优雅关闭与健康检查
- [x] 接入 `infinity-database` 连接数据库（连接池 + 迁移 + 示例仓储）
- [ ] 管理员认证与权限（`infinity-auth`）
- [ ] 请求级 Request ID 注入与错误响应关联

---

**Infinity Workspace · Admin Server**
