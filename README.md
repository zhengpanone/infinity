# Infinity

## Overview

> 🚀 A Modern Enterprise Platform Built with Rust.

Infinity 是一个基于 Rust 构建的现代企业级开发平台（Enterprise Platform），旨在提供高性能、可扩展、模块化的后台服务框架。采用 Cloud Native + AI Native 架构设计，专注于构建高性能、高可靠、高扩展性的业务系统，涵盖：

AI
ERP
WMS
Workflow
Authentication
API Gateway
File Storage
Notification
Scheduler
Monitoring

所有模块均采用 Cargo Workspace 统一管理，可按需组合部署。

## Design Philosophy

Infinity 遵循以下设计原则：

- High Performance
- Modular Architecture
- AI Native
- Cloud Native
- Async First
- Clean Architecture
- Domain Driven Design (DDD)
- API First
- Infrastructure as Code
- Developer Friendly

---

## ✨ Features

- 🚀 High Performance
    - Rust
    - Tokio
    - Async/Await

- 🌐 Web Framework
    - Axum
    - Tower
    - Hyper

- 🗄 Database
    - SQLx
    - PostgreSQL
    - MySQL
    - SQLite

- 🔐 Security
    - JWT
    - OAuth2
    - OpenID Connect

- ⚡ RPC
    - gRPC
    - Connect
    - HTTP/2
    - HTTP/3

- 🤖 AI Ready
    - OpenAI Compatible
    - Ollama
    - MCP
    - RAG

- ☁ Cloud Native
    - Docker
    - Kubernetes
    - Helm
    - Prometheus
    - Grafana
    - OpenTelemetry

- 📦 Storage
    - S3 Compatible
    - MinIO

- 📈 Observability
    - tracing
    - OpenTelemetry
    - Prometheus
    - Grafana

- ⚙ Configuration
    - TOML
    - YAML
    - Environment Variables

- 📨 Message Queue
    - NATS
    - Kafka
    - Redis Streams

- 🔍 Search
    - Meilisearch
    - Elasticsearch

- 🧰 Cache
    - Redis

- 📄 API
    - OpenAPI
    - Swagger UI

---

# Architecture

```
                +----------------------+
                |      Web / App       |
                +----------+-----------+
                           |
                      HTTP / gRPC
                           |
                 +---------v---------+
                 |      Gateway      |
                 +---------+---------+
                           |
        +------------------+------------------+
        |                  |                  |
+-------v------+   +--------v------+   +-------v------+
| AI Service   |   | ERP Service   |   | WMS Service  |
+--------------+   +---------------+   +--------------+
        |                  |                  |
        +------------------+------------------+
                           |
                    Shared Crates
                           |
        +------------------+------------------+
        | Config | DB | Cache | Common | Auth |
        +------------------+------------------+
                           |
                PostgreSQL / Redis / MinIO



```

---

# Workspace

```
infinity/

├── apps/
│   ├── gateway
│   ├── auth
│   ├── ai
│   ├── erp
│   ├── wms
│   ├── workflow
│   └── admin
│
├── crates/
│   ├── common
│   ├── config
│   ├── database
│   ├── cache
│   ├── auth
│   ├── logger
│   ├── grpc
│   ├── web
│   ├── utils
│   └── errors
│
├── proto/
│
├── migrations/
│
├── docs/
│
├── examples/
│
├── scripts/
│
├── Cargo.toml
└── README.md
```

---

# Tech Stack

| Category | Technology |
|-----------|------------|
| Language | Rust |
| Runtime | Tokio |
| Web | Axum |
| HTTP | Hyper |
| Middleware | Tower |
| ORM | SQLx |
| RPC | tonic / Connect |
| Serialization | Serde |
| Config | config-rs |
| Logging | tracing |
| Metrics | OpenTelemetry |
| Cache | Redis |
| MQ | NATS |
| Database | PostgreSQL / MySQL / SQLite |
| Object Storage | MinIO |
| Search | Meilisearch |
| AI | Ollama / OpenAI API |
| Authentication | JWT / OAuth2 |
| Documentation | utoipa |

---

# Planned Modules

| Module | Description |
|---------|-------------|
| Gateway | API Gateway |
| Auth | Authentication & Authorization |
| AI | LLM / Agent / RAG |
| ERP | Enterprise Resource Planning |
| WMS | Warehouse Management |
| Workflow | BPM Workflow |
| Message | Notification Center |
| Scheduler | Distributed Job Scheduler |
| File | File Storage |
| Audit | Audit Log |
| Monitor | Monitoring |
| System | System Management |

---

# Development

```bash
cargo build

cargo test

cargo fmt

cargo clippy
```

---

# License

MIT Licenseob Scheduler |
| File | File Storage |
| Audit | Audit Log |
| Monitor | Monitoring |
| System | System Management |

---

# Development

```bash
cargo build

cargo test

cargo fmt

cargo clippy
```

---

# License

MIT License