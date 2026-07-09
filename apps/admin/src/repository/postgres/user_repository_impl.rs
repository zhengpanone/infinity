use crate::{
    domain::dto::username::Username, models::user::User,
    repository::user_repository::UserRepository,
};
use infinity_error::ResultExt;
use infinity_error::{ErrorKind, Result};
use sqlx::PgPool;
use tracing::debug;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_by_username<'a>(
        &'a self,
        username: &'a Username,
    ) -> impl std::future::Future<Output = Result<Option<User>>> + Send + 'a {
        async move {
            debug!("find user by username: {}", username);
            let user = sqlx::query_as::<_, User>(
                r#"SELECT * FROM sys_user WHERE username = $1 AND deleted_at IS NULL"#,
            )
            .bind(username.as_ref())
            .fetch_optional(&self.pool)
            .await
            .context(
                ErrorKind::Database,
                format!("failed to query user by username: {}", username),
            )?;
            Ok(user)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::ids::UserId;
    use crate::enums::user::UserStatus;
    use chrono::Utc;
    use infinity_error::InfinityError;
    use sqlx::types::Json;
    use std::sync::Mutex;

    struct MockUserRepository {
        users: Mutex<Vec<Option<User>>>,
        error: Mutex<Option<InfinityError>>,
    }

    impl MockUserRepository {
        fn new(users: Vec<Option<User>>) -> Self {
            Self {
                users: Mutex::new(users),
                error: Mutex::new(None),
            }
        }

        fn with_error(error: InfinityError) -> Self {
            Self {
                users: Mutex::new(Vec::new()),
                error: Mutex::new(Some(error)),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        fn find_by_username<'a>(
            &'a self,
            _username: &'a Username,
        ) -> impl std::future::Future<Output = Result<Option<User>>> + Send + 'a {
            async move {
                if let Some(err) = self.error.lock().unwrap().take() {
                    return Err(err);
                }
                Ok(self.users.lock().unwrap().pop().unwrap_or(None))
            }
        }
    }

    fn make_user(username: &str) -> User {
        User {
            id: UserId::generate(),
            username: username.to_string(),
            email: format!("{username}@example.com"),
            phone: Some("13800138000".to_string()),
            status: UserStatus::Activate,
            password_hash: "hash_placeholder".to_string(),
            display_name: username.to_string(),
            avatar_url: None,
            roles: Json(vec!["user".to_string()]),
            permissions: Json(vec!["read".to_string()]),
            email_verified: true,
            phone_verified: false,
            last_login_at: None,
            login_count: 0,
            failed_login_count: 0,
            last_failed_login_at: None,
            locked_at: None,
            locked_until: None,
            lock_reason: None,
            password_changed_at: None,
            password_expires_at: None,
            is_first_login: true,
            last_activity_at: None,
            timezone: None,
            language: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    fn valid_username(_suffix: &str) -> Username {
        Username::new("test_user_123").expect("test username should pass validation")
    }

    #[tokio::test]
    async fn find_by_username_returns_user_when_found() {
        let expected = make_user("test_user_123");
        let repo = MockUserRepository::new(vec![Some(expected.clone())]);

        let username = valid_username("a");
        let result = repo.find_by_username(&username).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "test_user_123");
    }

    #[tokio::test]
    async fn find_by_username_returns_none_when_not_found() {
        let repo = MockUserRepository::new(vec![None]);

        let username = valid_username("b");
        let result = repo.find_by_username(&username).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn find_by_username_returns_none_when_empty_store() {
        let repo = MockUserRepository::new(vec![]);

        let username = valid_username("c");
        let result = repo.find_by_username(&username).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn find_by_username_returns_database_error() {
        let db_err = InfinityError::database("connection refused");
        let repo = MockUserRepository::with_error(db_err);

        let username = valid_username("d");
        let result = repo.find_by_username(&username).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Database);
        assert!(err.to_string().contains("connection refused"));
    }

    #[tokio::test]
    async fn find_by_username_mock_ignores_username_parameter() {
        let expected = make_user("test_user_123");
        let repo = MockUserRepository::new(vec![Some(expected.clone())]);

        let username = valid_username("e");
        let result = repo.find_by_username(&username).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().username, "test_user_123");
    }

    #[test]
    fn username_new_valid_20_char() {
        let username = Username::new("abcdefghij1234567890").unwrap();
        assert_eq!(username.as_ref(), "abcdefghij1234567890");
    }

    #[test]
    fn username_new_min_length_boundary() {
        assert!(Username::new("abc").is_ok());
        assert!(Username::new("ab").is_err());
    }

    #[test]
    fn username_new_max_length_boundary() {
        assert!(Username::new("abcdefghij1234567890").is_ok());
        assert!(Username::new("abcdefghij1234567890X").is_err());
    }

    #[test]
    fn username_new_with_underscore_20_chars() {
        assert!(Username::new("hello_world_test1234").is_ok());
    }

    #[test]
    fn username_new_rejects_special_chars() {
        assert!(Username::new("hello-world_test123").is_err());
        assert!(Username::new("hello world test123").is_err());
    }

    #[test]
    fn username_new_trims_whitespace() {
        let username = Username::new("  abcdefghij1234567890  ").unwrap();
        assert_eq!(username.as_ref(), "abcdefghij1234567890");
    }

    #[test]
    fn username_display_formats_correctly() {
        let username = Username::new("abcdefghij1234567890").unwrap();
        assert_eq!(username.to_string(), "abcdefghij1234567890");
    }

    #[test]
    fn username_as_ref_returns_str() {
        let username = Username::new("abcdefghij1234567890").unwrap();
        let s: &str = username.as_ref();
        assert_eq!(s, "abcdefghij1234567890");
    }

    #[tokio::test]
    async fn user_repository_impl_new_creates_instance() {
        let pool = PgPool::connect_lazy("postgres://localhost:5432/testdb")
            .expect("lazy pool should be constructable");
        let repo = UserRepositoryImpl::new(pool);
        let _ = repo;
    }
}
