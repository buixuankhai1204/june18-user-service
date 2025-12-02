use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityLoaderTrait, QueryFilter};
use crate::core::error::AppResult;
use crate::domain::user::user::{ActiveModel, Model, ModelEx};
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use crate::domain::{address, user};

#[async_trait]
impl UserRepositoryInterface for user::user::Entity {
    async fn create_user(conn: &DatabaseTransaction, user: ActiveModel) -> AppResult<Model> {
        todo!()
    }

    async fn update_user(conn: &DatabaseTransaction, user: ActiveModel) -> AppResult<Model> {
        todo!()
    }

    async fn find_user_by_id(conn: &DatabaseTransaction, id: i64) -> AppResult<Option<ModelEx>> {
        let user = user::user::Entity::load()
            .filter_by_id(id)
            .with(address::address::Entity)
            .one(conn)
            .await?;
        Ok(user)
    }

    async fn find_user_by_username(
        conn: &DatabaseTransaction,
        username: &str,
    ) -> AppResult<Option<ModelEx>> {
        let user = user::user::Entity::load()
            .filter(user::user::Column::Username.eq(username))
            .with(address::address::Entity)
            .one(conn)
            .await?;
        Ok(user)
    }

    async fn find_user_by_email(
        conn: &DatabaseTransaction,
        email: &str,
    ) -> AppResult<Option<ModelEx>> {
        let user = user::user::Entity::load()
            .filter(user::user::Column::Email.eq(email))
            .with(address::address::Entity)
            .one(conn)
            .await?;
        Ok(user)
    }

    async fn delete_user(conn: &DatabaseTransaction, id: i64) -> AppResult<()> {
        todo!()
    }

    async fn username_exists(conn: &DatabaseTransaction, username: &str) -> AppResult<bool> {
        todo!()
    }

    async fn email_exists(conn: &DatabaseTransaction, email: &str) -> AppResult<bool> {
        todo!()
    }

    async fn list_users(
        conn: &DatabaseTransaction,
        page: u64,
        page_size: u64,
    ) -> AppResult<Vec<Model>> {
        todo!()
    }
}
