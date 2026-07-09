//! 进程级可观测性辅助。
//!
//! 目前提供致命错误的结构化上报，字段命名遵循工作区可观测性规范
//! （见 infinity-error `docs/OBSERVABILITY.md`）。

use infinity_error::{InfinityError, field};

/// 按可观测性规范结构化上报致命错误。
///
/// 字段名统一取自 [`infinity_error::field`]，与工作区其余日志保持一致；
/// 同时向 stderr 兜底输出，覆盖「日志尚未初始化」（例如配置加载失败）的窗口。
pub fn report_fatal(err: &InfinityError) {
    tracing::error!(
        { field::KIND } = err.code(),
        { field::STATUS } = err.status_code(),
        { field::CLASS } = err.class().as_str(),
        { field::ROOT_CAUSE } = %err.root_cause(),
        { field::CHAIN } = err.chain_string(),
        "admin server exited with error"
    );

    // 兜底：配置或日志初始化阶段失败时，tracing 尚无 subscriber，事件会被丢弃。
    eprintln!("fatal [{}]: {}", err.code(), err.chain_string());
}
