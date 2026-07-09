use std::future::Future;

use infinity_error::Result;

use crate::{domain::dto::username::Username, models::user::User};

pub trait UserRepository: Send + Sync {
    fn find_by_username<'a>(
        &'a self,
        username: &'a Username,
    ) -> impl Future<Output = Result<Option<User>>> + Send + 'a;
}
