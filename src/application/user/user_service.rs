use crate::core::error::{AppError, AppResult};
use crate::infrastructure::third_party::redis::lib::RedisConnectionPool;
use crate::application::user::user_service_interface::UserServiceInterface;
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use crate::presentation::user::user::UserSerializer;
use log::error;
use rdkafka::producer::FutureProducer;
use sea_orm::DatabaseTransaction;
use std::sync::Arc;
use crate::domain::user;

#[derive()]
pub struct UserService {
    pub redis: Arc<RedisConnectionPool>,
    pub kafka_producer: Arc<FutureProducer>,
}
impl UserService {
    pub fn new(redis: Arc<RedisConnectionPool>, kafka_producer: Arc<FutureProducer>) -> Self {
        Self { redis, kafka_producer }
    }
}
impl UserServiceInterface for UserService {
    async fn get_profile(
        &self,
        conn: &DatabaseTransaction,
        user_id: i64,
    ) -> AppResult<UserSerializer> {
        let info_user = self
            .redis
            .get_and_deserialize_key::<UserSerializer>(
                &format!("profile:user_id:{}", user_id).to_string().into(),
                "UserRelatedResponse",
            )
            .await;

        match info_user {
            Ok(value) => Ok(value),
            Err(error) => {
                error!("Error when get profile from redis: {:#?}", error);
                match user::user::Entity::find_user_by_id(conn, user_id).await {
                    Ok(Some(profile)) => {
                        let _ = self
                            .redis
                            .serialize_and_set_key_with_expiry(
                                &format!("profile:user_id:{}", user_id.to_string())
                                    .to_string()
                                    .into(),
                                &profile,
                                88640,
                            )
                            .await;
                        Ok(UserSerializer::from(profile))
                    },
                    Err(_error) => Err(AppError::EntityNotFoundError {
                        detail: format!("User not found by id {}", user_id),
                    }),
                    _ => {
                        Err(AppError::EntityNotFoundError {
                            detail: format!("User not found by id {}", user_id),
                        })
                    }
                }
            },
        }
    }

    async fn logout(&self, conn: &DatabaseTransaction, user_id: i64) -> AppResult<bool> {
        match self.redis.delete_key(&format!("profile:user_id:{user_id}").to_string().into()).await
        {
            Ok(_) => Ok(true),
            Err(err) => Err(AppError::BadRequestError(err.to_string())),
        }
    }
}
