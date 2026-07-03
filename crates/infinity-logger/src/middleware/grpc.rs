//! gRPC 集成。
//!
//! 提供 Tonic 拦截器，为每个 gRPC 请求注入 Request ID 并创建追踪 span。

use tonic::{Request, Status, service::Interceptor};

use crate::middleware::request_id;

/// gRPC Request ID 拦截器。
///
/// 为每个进入的请求生成 Request ID，并写入请求的 metadata。
///
/// # Examples
///
/// ```no_run
/// use infinity_logger::middleware::grpc::RequestIdInterceptor;
///
/// let interceptor = RequestIdInterceptor::default();
/// ```
#[derive(Debug, Clone, Default)]
pub struct RequestIdInterceptor;

impl Interceptor for RequestIdInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let id = request_id::generate();
        if let Ok(value) = id.parse() {
            request
                .metadata_mut()
                .insert(request_id::REQUEST_ID_HEADER, value);
        }
        Ok(request)
    }
}
