use crate::domain::types::ids::UserId;
use crate::domain::types::username::Username;
use crate::models::user::User;
use crate::repository::user_repository::{NewUser, UserRepository};
use async_trait::async_trait;
use infinity_error::{ErrorKind, Result, ResultExt};
use sqlx::PgPool;
use tracing::debug;

/// `sys_user` 全字段列，展开为字符串字面量，供各查询的 SELECT / RETURNING 复用，
/// 避免多处手抄漂移；同时保持 `sqlx` 对静态 SQL 的编译期防注入检查。
macro_rules! user_columns {
    () => {
        r#"id, username, email, phone, password_hash, display_name, avatar_url,
           email_verified, phone_verified, status, last_login_at, login_count,
           failed_login_count, last_failed_login_at, is_first_login, last_activity_at,
           locked_until, locked_at, lock_reason, password_changed_at, password_expires_at,
           created_at, updated_at, deleted_at"#
    };
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        debug!("find user by id: {}", id);

        let user = sqlx::query_as::<_, User>(concat!(
            "SELECT ",
            user_columns!(),
            " FROM sys_user WHERE id = $1 AND deleted_at IS NULL"
        ))
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .context(
            ErrorKind::Database,
            format!("failed to query user by id: {id}"),
        )?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>> {
        debug!("find user by username: {}", username);

        let user = sqlx::query_as::<_, User>(concat!(
            "SELECT ",
            user_columns!(),
            " FROM sys_user WHERE username = $1 AND deleted_at IS NULL"
        ))
        .bind(username.as_ref())
        .fetch_optional(&self.pool)
        .await
        .context(
            ErrorKind::Database,
            format!("failed to query user by username: {username}"),
        )?;

        Ok(user)
    }

    async fn create(&self, user: NewUser) -> Result<User> {
        debug!("create user: {}", user.username);

        let saved = sqlx::query_as::<_, User>(concat!(
            r#"INSERT INTO sys_user (
                id, username, email, phone, password_hash, display_name, avatar_url,
                password_changed_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, NOW()
            ) RETURNING "#,
            user_columns!()
        ))
        .bind(UserId::generate().as_uuid())
        .bind(user.username.as_ref())
        .bind(user.email)
        .bind(user.phone)
        .bind(user.password_hash)
        .bind(user.display_name)
        .bind(user.avatar_url)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create user")?;

        Ok(saved)
    }
}
