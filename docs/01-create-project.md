# Creating the Infinity Workspace

本文档介绍如何从零开始创建 **Infinity** 项目。

## Prerequisites

安装以下开发环境：

| Software   | Recommended Version |
| ---------- | ------------------- |
| Rust       | stable              |
| Cargo      | latest              |
| Git        | latest              |
| Docker     | latest              |
| PostgreSQL | 17+ (Optional)      |
| Redis      | 8+ (Optional)       |
| Protobuf   | latest (Optional)   |

验证安装：

```bash
rustc --version
cargo --version
git --version
```

---

# 1. Create Project

创建项目目录。

```bash
mkdir infinity
cd infinity
```

初始化 Git 仓库。

```bash
git init
```

---

# 2. Initialize Cargo Workspace

初始化 Cargo。

```bash
cargo init --vcs none
```

执行完成后，目录如下：

```text
infinity
├── Cargo.toml
└── src
    └── main.rs
```

由于 Infinity 使用 **Cargo Workspace**，删除默认生成的源码。

```bash
rm -rf src
```

---

# 3. Configure Workspace

修改根目录 `Cargo.toml`。

```toml
[workspace]
resolver = "3"

members = [
    "apps/*",
    "crates/*",
]

[workspace.package]
edition = "2024"
version = "0.1.0"
license = "MIT"

[workspace.dependencies]

tokio = { version = "1", features = ["full"] }

axum = "0.8"

tower = "0.5"

hyper = "1"

serde = { version = "1", features = ["derive"] }

serde_json = "1"

sqlx = { version = "0.8", default-features = false, features = [
    "runtime-tokio",
    "postgres",
    "mysql",
    "sqlite",
    "chrono",
    "uuid",
    "macros"
] }

uuid = { version = "1", features = ["v7", "serde"] }

chrono = { version = "0.4", features = ["serde"] }

tracing = "0.1"

tracing-subscriber = "0.3"

thiserror = "2"

anyhow = "1"
```

Workspace 将统一管理所有依赖版本。

---

# 4. Create Directory Structure

创建目录。

```bash
mkdir apps
mkdir crates
mkdir configs
mkdir docs
mkdir examples
mkdir migrations
mkdir proto
mkdir scripts
```

项目结构：

```text
infinity

├── apps/
├── crates/
├── configs/
├── docs/
├── examples/
├── migrations/
├── proto/
├── scripts/

├── Cargo.toml
└── README.md
```

---

# 5. Create Shared Crates

创建公共库。

```bash
cargo new crates/common --lib

cargo new crates/config --lib

cargo new crates/logger --lib

cargo new crates/database --lib

cargo new crates/cache --lib

cargo new crates/errors --lib

cargo new crates/auth --lib

cargo new crates/web --lib

cargo new crates/grpc --lib

cargo new crates/utils --lib

cargo new crates/macros --lib
```

目录如下：

```text
crates

├── auth
├── cache
├── common
├── config
├── database
├── errors
├── grpc
├── logger
├── macros
├── utils
└── web
```

---

# 6. Create Applications

创建业务服务。

```bash
cargo new apps/gateway --name infinity-gateway-server

cargo new apps/auth --name infinity-auth-server

cargo new apps/ai --name infinity-ai-server

cargo new apps/erp --name infinity-erp-server

cargo new apps/wms --name infinity-wms-server

cargo new apps/workflow --name infinity-workflow-server

cargo new apps/scheduler --name infinity-scheduler-server

cargo new apps/admin --name infinity-admin-server
```

目录如下：

```text
apps

├── admin
├── ai
├── auth
├── erp
├── gateway
├── scheduler
├── wms
└── workflow
```

---

# 7. Build Workspace

查看 Workspace 成员。

```bash
cargo metadata
```

编译整个 Workspace。

```bash
cargo build
```

运行指定服务。

```bash
cargo run -p infinity-gateway
```

运行测试。

```bash
cargo test
```

格式化代码。

```bash
cargo fmt
```

静态检查。

```bash
cargo clippy --workspace
```

---

# Final Project Structure

```text
infinity
│
├── apps
│   ├── gateway
│   ├── auth
│   ├── ai
│   ├── erp
│   ├── wms
│   ├── workflow
│   ├── scheduler
│   └── admin
│
├── crates
│   ├── auth
│   ├── cache
│   ├── common
│   ├── config
│   ├── database
│   ├── errors
│   ├── grpc
│   ├── logger
│   ├── macros
│   ├── utils
│   └── web
│
├── configs
├── docs
├── examples
├── migrations
├── proto
├── scripts
│
├── Cargo.toml
├── README.md
├── .gitignore
└── LICENSE
```

---

# Next Step

创建完成 Workspace 后，建议按照以下顺序进行开发：

1. Config（配置中心）
2. Logger（日志系统）
3. Error（统一错误处理）
4. Common（公共类型）
5. Utils（工具库）
6. Database（SQLx）
7. Cache（Redis）
8. Web（Axum）
9. gRPC（tonic / Connect）
10. Auth（JWT / OAuth2）
11. Gateway
12. AI
13. ERP
14. Workflow
15. WMS

以上顺序能够保证基础设施先稳定，再逐步构建业务模块，减少后续重构成本。
