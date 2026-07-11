use infinity_error::{InfinityError, Result};

use crate::domain::{
    dto::user::{CreateUserDTO, UpdateUserDTO},
    types::{
        Email, Url, UserId,
        ids::{PermissionId, RoleId},
        phone_number::Phone,
        username::Username,
    },
};

pub struct CreateUserCommand {
    pub username: Username,
    pub email: Email,
    pub phone: Option<Phone>,
    pub password: String,
    pub display_name: String,
    pub avatar_url: Option<Url>,
    pub role_ids: Vec<RoleId>,
    pub permission_ids: Vec<PermissionId>,
}

impl TryFrom<CreateUserDTO> for CreateUserCommand {
    type Error = InfinityError;

    fn try_from(dto: CreateUserDTO) -> Result<Self> {
        Ok(Self {
            username: Username::new(&dto.username)?,
            email: Email::new(&dto.email)?,
            phone: dto.phone.map(|value| Phone::new(&value)).transpose()?,
            password: dto.password,
            display_name: dto.display_name,
            avatar_url: dto.avatar_url.map(|value| Url::new(&value)).transpose()?,
            role_ids: dto.role_ids.into_iter().map(RoleId::from).collect(),
            permission_ids: dto
                .permission_ids
                .into_iter()
                .map(PermissionId::from)
                .collect(),
        })
    }
}

pub struct UpdateUserCommand {
    pub id: UserId,
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<Url>,
}

impl TryFrom<UpdateUserDTO> for UpdateUserCommand {
    type Error = InfinityError;

    fn try_from(dto: UpdateUserDTO) -> Result<Self> {
        Ok(Self {
            id: UserId::from(dto.id),
            username: dto
                .username
                .map(|value| Username::new(&value))
                .transpose()?,
            email: dto.email.map(|value| Email::new(&value)).transpose()?,
            phone: dto.phone.map(|value| Phone::new(&value)).transpose()?,
            password: dto.password,
            display_name: dto.display_name,
            avatar_url: dto.avatar_url.map(|value| Url::new(&value)).transpose()?,
        })
    }
}
