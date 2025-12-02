use crate::core::error::AppResult;
use crate::presentation::user::user::UserSerializer;
use sea_orm::DatabaseTransaction;

pub trait UserServiceInterface: Send + Sync + 'static {
    async fn get_profile(
        &self,
        conn: &DatabaseTransaction,
        user_id: i64,
    ) -> AppResult<UserSerializer>;
    async fn logout(&self, conn: &DatabaseTransaction, id: i64) -> AppResult<bool>;
}
