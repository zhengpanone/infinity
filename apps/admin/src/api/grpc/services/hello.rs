use chrono::Utc;
use infinity_proto::hello::{HelloReply, HelloRequest, greeter_service_server::GreeterService};
use tonic::{Request, Response, Status, Streaming};

use crate::api::grpc::converter::timestamp_converter::TimestampConverter;

/// 示例 Greeter 服务实现（占位）。
#[derive(Default)]
pub struct GrpcHelloService;

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
            timestamp: TimestampConverter::to_proto_opt(Some(Utc::now())),
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
