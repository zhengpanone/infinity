use crate::domain::dto::user::{UserQueryDTO, UserSortField};
use crate::domain::types::ids::UserId;
use crate::domain::types::phone_number::Phone;
use crate::domain::types::username::Username;
use crate::domain::types::{Email, Url};
use crate::models::user::User;
use crate::repository::user_repository::{
    CheckUserExists, NewUser, UpdateUser, UserExistsResult, UserRepository,
};
use async_trait::async_trait;
use infinity_error::{ErrorKind, InfinityError, Result, ResultExt};
use infinity_web::{PaginatedData, PaginationParams, SortRule};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::debug;

/// `sys_user` 全字段列，展开为字符串字面量，供各查询的 SELECT / RETURNING 复用，
/// 避免多处手抄漂移；同时保持 `sqlx` 对静态 SQL 的编译期防注入检查。
macro_rules! user_columns {
    () => {
        r#"id, username, email, phone, password_hash, display_name,
           avatar_url, email_verified, phone_verified, status,
           last_login_at, login_count, failed_login_count,
           last_failed_login_at, is_first_login, last_activity_at,
           locked_until, locked_at, lock_reason, password_changed_at,
           password_expires_at,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
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
            " FROM sys_user WHERE id = $1 AND is_deleted = FALSE"
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
            " FROM sys_user WHERE username = $1 AND is_deleted = FALSE"
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

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(concat!(
            "SELECT ",
            user_columns!(),
            " FROM sys_user WHERE email = $1 AND is_deleted = FALSE"
        ))
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .context(
            ErrorKind::Database,
            format!("failed to query user by email: {email}"),
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
        .bind(user.username.as_str())
        .bind(user.email.as_str())
        .bind(user.phone.as_ref().map(Phone::as_str))
        .bind(user.password_hash)
        .bind(user.display_name)
        .bind(user.avatar_url.as_ref().map(Url::as_str))
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create user")?;

        Ok(saved)
    }

    async fn update_by_id(&self, user: UpdateUser) -> Result<Option<User>> {
        let updated = sqlx::query_as::<_, User>(concat!(
            r#"UPDATE sys_user
               SET username = COALESCE($2, username),
               email = COALESCE($3, email),
               phone = COALESCE($4, phone),
               password_hash = COALESCE($5, password_hash),
               display_name = COALESCE($6, display_name),
               avatar_url = COALESCE($7, avatar_url),
               password_changed_at = CASE
                   WHEN $5::VARCHAR IS NOT NULL THEN NOW()
                   ELSE password_changed_at
               END
           WHERE id = $1
             AND is_deleted = FALSE
           RETURNING "#,
            user_columns!()
        ))
        .bind(user.id.as_uuid())
        .bind(user.username.map(Username::into_inner))
        .bind(user.email.map(Email::into_inner))
        .bind(user.phone.map(Phone::into_inner))
        .bind(user.password)
        .bind(user.display_name)
        .bind(user.avatar_url.map(Url::into_inner))
        .fetch_optional(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to update user")?;

        Ok(updated)
    }

    async fn soft_delete(&self, ids: &[UserId]) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }

        let ids = ids.iter().map(|id| id.as_uuid()).collect::<Vec<_>>();
        let result = sqlx::query(
            r#"UPDATE sys_user
               SET is_deleted = TRUE,
                   status = 'deleted',
                   deleted_at = NOW()
               WHERE id = ANY($1)
                 AND is_deleted = FALSE"#,
        )
        .bind(&ids)
        .execute(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to soft delete users")?;

        Ok(result.rows_affected())
    }

    async fn page_list(
        &self,
        query: PaginationParams<UserQueryDTO, UserSortField>,
    ) -> Result<PaginatedData<Vec<User>>> {
        query.validate().map_err(InfinityError::validation)?;

        let limit = query.limit().map_err(InfinityError::validation)?;

        let offset = query.offset().map_err(InfinityError::validation)?;
        // 查询符合条件的总记录数。

        let mut count_builder = QueryBuilder::<Postgres>::new(
            r#"SELECT COUNT(1)::BIGINT
            FROM sys_user
            WHERE is_deleted = FALSE"#,
        );
        push_user_filters(&mut count_builder, &query.filters);

        let total: i64 = count_builder
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await
            .context(ErrorKind::Database, "failed to count users")?;
        // 查询当页数据

        let mut data_builder = QueryBuilder::<Postgres>::new("SELECT ");

        data_builder.push(user_columns!()).push(
            r#" FROM sys_user
                WHERE is_deleted = FALSE
        "#,
        );
        push_user_filters(&mut data_builder, &query.filters);
        push_user_sorts(&mut data_builder, &query.sorts);

        data_builder
            .push(" LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset);

        let users = data_builder
            .build_query_as::<User>()
            .fetch_all(&self.pool)
            .await
            .context(ErrorKind::Database, "failed to query user page")?;
        let total = u64::try_from(total)
            .map_err(|_| InfinityError::database("user count cannot be negative"))?;

        PaginatedData::try_new(users, query.page_num, query.page_size, total)
            .map_err(InfinityError::validation)
    }

    async fn check_exists(&self, query: &CheckUserExists) -> Result<UserExistsResult> {
        let username = query.username.as_ref().map(Username::as_str);

        let email = query.email.as_ref().map(Email::as_str);

        let phone = query.phone.as_ref().map(Phone::as_str);

        let exclude_user_id = query.exclude_user_id.as_ref().map(UserId::as_uuid);

        let result: (Option<bool>, Option<bool>, Option<bool>) = sqlx::query_as(
            r#"
            SELECT
                CASE
                    WHEN $1::VARCHAR IS NULL THEN NULL
                    ELSE EXISTS (
                        SELECT 1
                        FROM sys_user
                        WHERE username = $1
                          AND is_deleted = FALSE
                          AND ($4::UUID IS NULL OR id <> $4)
                    )
                END AS username_exists,

                CASE
                    WHEN $2::VARCHAR IS NULL THEN NULL
                    ELSE EXISTS (
                        SELECT 1
                        FROM sys_user
                        WHERE email = $2
                          AND is_deleted = FALSE
                          AND ($4::UUID IS NULL OR id <> $4)
                    )
                END AS email_exists,

                CASE
                    WHEN $3::VARCHAR IS NULL THEN NULL
                    ELSE EXISTS (
                        SELECT 1
                        FROM sys_user
                        WHERE phone = $3
                          AND is_deleted = FALSE
                          AND ($4::UUID IS NULL OR id <> $4)
                    )
                END AS phone_exists
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(phone)
        .bind(exclude_user_id)
        .fetch_one(&self.pool)
        .await
        .context(
            ErrorKind::Database,
            "failed to check whether user fields exist",
        )?;

        Ok(UserExistsResult {
            username_exists: result.0,
            email_exists: result.1,
            phone_exists: result.2,
        })
    }
}

fn push_user_filters(builder: &mut QueryBuilder<Postgres>, filters: &UserQueryDTO) {
    if let Some(username) = filters
        .username
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder
            .push(" AND username ILIKE ")
            .push_bind(format!("%{username}%"));
    }
    if let Some(email) = filters
        .email
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder
            .push(" AND email ILIKE ")
            .push_bind(format!("%{email}%"));
    }
    if let Some(phone) = filters
        .phone
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder
            .push(" AND phone ILIKE ")
            .push_bind(format!("%{phone}%"));
    }
}

fn push_user_sorts(builder: &mut QueryBuilder<Postgres>, sorts: &[SortRule<UserSortField>]) {
    if sorts.is_empty() {
        builder.push(" ORDER BY username ASC, created_at DESC, id ASC");
        return;
    }
    builder.push(" ORDER BY ");

    for (index, sort) in sorts.iter().enumerate() {
        if index > 0 {
            builder.push(", ");
        }
        builder
            .push(sort.field.as_sql())
            .push(" ")
            .push(sort.order.as_sql());
    }
    builder.push(" , id ASC");
}
