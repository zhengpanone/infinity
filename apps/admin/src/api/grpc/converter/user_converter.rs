use crate::{
    api::grpc::converter::timestamp_converter::TimestampConverter, domain::vo::user::UserVO,
};
use infinity_proto::user::User as ProtoUser;

/// 用户转换器
pub struct UserConverter;

impl UserConverter {
    /// 转换领域用户到gRPC用户
    pub fn to_proto(user: &UserVO) -> ProtoUser {
        ProtoUser {
            id: user.id.to_string(),
            phone: user.phone.clone(),
            phone_verified: user.phone_verified,
            username: user.username.to_string(),
            email: user.email.to_string(),
            email_verified: user.email_verified,
            display_name: user.display_name.to_string(),
            avatar_url: user.avatar_url.clone(),
            login_count: user.login_count,
            failed_login_count: user.failed_login_count,
            created_at: Some(TimestampConverter::to_proto(user.created_at)),
            updated_at: Some(TimestampConverter::to_proto(user.updated_at)),
        }
    }
}
