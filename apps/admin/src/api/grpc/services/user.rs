//! gRPC `UserService` 实现。
//!
//! 把 `infinity-proto` 生成的服务端 trait 适配到应用层
//! [`UserService`](crate::services::user_service::UserService)：
//! proto 请求 → DTO/查询 → 应用层 → [`UserVO`] → proto 响应。

use std::sync::Arc;

use infinity_proto::{
    common::CommonId,
    user::{
        CreateUserRequest, GetUserRequest, GetUsersRequest, UserResponse,
        get_user_request::Identifier,
        user_service_server::{UserService as UserServiceGrpc, UserServiceServer},
    },
};
use tonic::{Request, Response, Status};

use crate::{
    api::grpc::converter::user_converter::UserConverter,
    domain::dto::user::CreateUserDTO,
    services::user_service::{UserQuery, UserService},
};

pub struct UserGrpcService {
    user_service: Arc<dyn UserService>,
}

impl UserGrpcService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }

    /// 包装为可注册到 tonic `Server` 的服务实例。
    pub fn into_server(self) -> UserServiceServer<Self> {
        UserServiceServer::new(self)
    }
}

#[tonic::async_trait]
impl UserServiceGrpc for UserGrpcService {
    async fn get_user_by_id(
        &self,
        _request: Request<CommonId>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn get_users(
        &self,
        _request: Request<GetUsersRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(username = %req.username, "grpc create_user");

        let dto = CreateUserDTO {
            username: req.username,
            email: req.email,
            phone: req.phone,
            password: req.password,
            display_name: req.display_name,
            avatar_url: req.avatar_url,
            role_ids: Vec::new(),
            permission_ids: Vec::new(),
        };

        let vo = self
            .user_service
            .create_user(dto)
            .await
            .map_err(status_from_error)?;

        Ok(Response::new(UserResponse {
            user: Some(UserConverter::to_proto(&vo)),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let query = match req.identifier {
            Some(Identifier::UserId(id)) => {
                let uuid: uuid::Uuid = id
                    .parse()
                    .map_err(|_| Status::invalid_argument(format!("invalid user_id: {id}")))?;
                UserQuery::Id(uuid)
            }
            Some(Identifier::Email(email)) => UserQuery::Email(email),
            Some(Identifier::Username(name)) => UserQuery::Username(name),
            Some(Identifier::Phone(phone)) => UserQuery::Phone(phone),
            None => return Err(Status::invalid_argument("identifier is required")),
        };

        let vo = self
            .user_service
            .get_user(query)
            .await
            .map_err(status_from_error)?;

        Ok(Response::new(UserResponse {
            user: Some(UserConverter::to_proto(&vo)),
        }))
    }
}

/// 把统一错误按分类映射为 gRPC `Status`，保持与 HTTP 层一致的语义。
fn status_from_error(err: infinity_error::InfinityError) -> Status {
    use infinity_error::ErrorKind;

    let msg = err.to_string();
    match err.kind() {
        ErrorKind::Validation | ErrorKind::Parse => Status::invalid_argument(msg),
        ErrorKind::NotFound => Status::not_found(msg),
        ErrorKind::Conflict => Status::already_exists(msg),
        ErrorKind::Unauthorized => Status::unauthenticated(msg),
        ErrorKind::Forbidden => Status::permission_denied(msg),
        ErrorKind::Unsupported => Status::unimplemented(msg),
        // 服务端错误不外泄内部细节。
        _ => Status::internal("internal server error"),
    }
}
