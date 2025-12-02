use super::address;
use crate::core::error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryInterface: Send + Sync {
    async fn create_address(&self, address: address::ActiveModel) -> AppResult<address::Model>;
    async fn update_address(&self, address: address::ActiveModel) -> AppResult<address::Model>;
    async fn find_address_by_id(&self, id: i64) -> AppResult<Option<address::Model>>;
    async fn delete_address(&self, id: i64) -> AppResult<()>;
    async fn find_addresses_by_user_id(&self, user_id: i64) -> AppResult<Vec<address::Model>>;
}
