//! Admin gRPC 服务。
//!
//! 基于 `tonic` 提供 gRPC 服务：示例 Greeter、健康检查与反射（reflection），
//! 复用 [`super::shutdown_signal`] 做优雅关闭，错误统一收敛为
//! [`InfinityError`](infinity_error::InfinityError)。

use std::net::SocketAddr;

use infinity_config::config::AppConfig;
use infinity_error::{ErrorKind, Result, ResultExt};
use infinity_proto::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use infinity_proto::hello::{HelloReply, HelloRequest};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use tonic_health::server::health_reporter;
use tonic_reflection::server::Builder as ReflectionBuilder;

/// 示例 Greeter 服务实现（占位）。
#[derive(Default)]
struct GrpcHelloService;

#[tonic::async_trait]
impl GreeterService for GrpcHelloService {
    /// 一元 RPC：返回问候语。
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        tracing::debug!(?request, "received SayHello request");

        let req = request.into_inner();
        let reply = HelloReply {
            message: format!("Hello, {}!", req.name),
            timestamp: None,
            status: infinity_proto::common::Status::Success as i32,
        };

        Ok(Response::new(reply))
    }

    async fn bid_hello(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(HelloReply::default()))
    }

    async fn lot_of_replies(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(HelloReply::default()))
    }

    async fn lot_of_greetings(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(HelloReply::default()))
    }
}

/// 绑定配置中的 gRPC `host:port` 并启动服务，直到收到关闭信号后优雅退出。
///
/// 配置缺省 `[grpc]` 段时回退到 [`GrpcConfig`](infinity_config::config::GrpcConfig)
/// 的默认值（`0.0.0.0:50051`），与其余可选配置的处理方式一致。
pub(crate) async fn serve(config: &AppConfig) -> Result<()> {
    let grpc = config.grpc.clone().unwrap_or_default();
    let addr: SocketAddr = format!("{}:{}", grpc.host, grpc.port)
        .parse()
        .with_context(ErrorKind::Config, || {
            format!("invalid grpc address {}:{}", grpc.host, grpc.port)
        })?;

    // 健康检查：标记 Greeter 为 SERVING。
    let (health_reporter, health_service) = health_reporter();
    health_reporter
        .set_serving::<GreeterServiceServer<GrpcHelloService>>()
        .await;

    // 反射服务：便于 grpcurl 等客户端自描述发现。
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(infinity_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .with_context(ErrorKind::Internal, || {
            "failed to build gRPC reflection service".to_owned()
        })?;

    tracing::info!(addr = %addr, "admin gRPC server listening");

    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GreeterServiceServer::new(GrpcHelloService))
        .serve_with_shutdown(addr, super::shutdown_signal())
        .await
        .with_context(ErrorKind::Web, || {
            format!("admin gRPC server error on {addr}")
        })?;

    tracing::info!("admin gRPC server stopped");
    Ok(())
}
