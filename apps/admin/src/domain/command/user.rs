use infinity_error::Result;

use crate::domain::{
    dto::user::CreateUserDTO,
    types::{
        ids::{PermissionId, RoleId},
        username::Username,
    },
};

pub struct CreateUserCommand {
    pub username: Username,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role_ids: Vec<RoleId>,
    pub permission_ids: Vec<PermissionId>,
}

impl TryFrom<CreateUserDTO> for CreateUserCommand {
    type Error = infinity_error::InfinityError;

    fn try_from(dto: CreateUserDTO) -> Result<Self> {
        Ok(Self {
            username: Username::new(&dto.username)?,
            email: dto.email,
            phone: dto.phone,
            password: dto.password,
            display_name: dto.display_name,
            avatar_url: dto.avatar_url,
            role_ids: dto.role_ids.into_iter().map(RoleId::from).collect(),
            permission_ids: dto.permission_ids.into_iter().map(PermissionId::from).collect(),
        })
    }
}
