//! 服务运行时。
//!
//! 承载 admin 对外提供的两种服务——HTTP（[`http`]）与 gRPC（[`grpc`]），
//! 二者共用同一套优雅关闭信号（[`shutdown_signal`]），由 [`serve_all`] 并发驱动。

pub(crate) mod grpc;
pub(crate) mod http;

use std::sync::Arc;

use infinity_config::config::AppConfig;
use infinity_database::Database;
use infinity_error::Result;

/// 并发启动 HTTP 与 gRPC 服务，任一出错即整体退出；
/// 收到关闭信号（Ctrl-C / SIGTERM）时两者一起优雅退出。
///
/// 数据库句柄目前只有 HTTP 服务用到（探活与查询），gRPC 暂不需要。
pub(crate) async fn serve_all(config: &AppConfig, db: Arc<Database>) -> Result<()> {
    tracing::info!("starting HTTP and gRPC servers");
    tokio::try_join!(http::serve(config, db), grpc::serve(config))?;
    Ok(())
}

/// 等待进程终止信号（Ctrl-C，或 Unix 上的 `SIGTERM`），触发优雅关闭。
///
/// 信号处理器安装失败时记录错误但不 panic，避免因可观测性问题拖垮进程。
/// 供 HTTP 与 gRPC 两个服务共用同一套关闭信号逻辑。
pub(crate) async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!(error = %err, "failed to install Ctrl-C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(err) => tracing::error!(error = %err, "failed to install SIGTERM handler"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("shutdown signal received, starting graceful shutdown");
}
