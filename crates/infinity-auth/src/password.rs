use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher as ArgonPasswordHasher, SaltString, rand_core::OsRng},
};
use infinity_error::{InfinityError, Result};

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String>;
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool>;
}

#[derive(Clone, Default)]
pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    /// 使用 Argon2 算法对密码进行哈希处理，自动生成随机盐值。
    ///
    /// # Errors
    ///
    /// 当 Argon2 哈希计算失败时，返回 `InfinityError::internal` 错误。
    fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| InfinityError::internal(format!("hash password failed: {err}")))?;
        Ok(hash.to_string())
    }

    /// 验证密码与哈希值是否匹配。
    ///
    /// # Errors
    ///
    /// - 当 `password_hash` 无法解析为有效的 `PasswordHash` 格式时，返回内部错误。
    /// - 当密码验证过程中出现非密码不匹配的其他错误时，返回内部错误。
    ///
    /// # Panics
    ///
    /// 不会发生 panic。
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool> {
        let parsed = PasswordHash::new(password_hash)
            .map_err(|err| InfinityError::internal(format!("parse password hash failed: {err}")))?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(err) => Err(InfinityError::internal(format!(
                "verify password failed: {err}"
            ))),
        }
    }
}
