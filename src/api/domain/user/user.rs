use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, EntityResponse};
use crate::application::user::user_service_interface::UserServiceInterface;
use crate::presentation::user::user::UserSerializer;
use crate::util::claim::UserClaims;
use axum::extract::State;
use axum::Json;
use log::error;
use sea_orm::TransactionTrait;

#[utoipa::path(
    get,
    path = "/v1/me",
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success get user profile", body =
        EntityResponse<UserSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_profile(
    State(state): State<AppState>,
    claims: UserClaims,
) -> AppResult<Json<EntityResponse<UserSerializer>>> {
    log::info!("Get profile user id: {}.", claims.user_id);
    let tx = state.db.begin().await?;
    match state.user_service.get_profile(&tx, claims.user_id).await {
        Ok(result) => Ok(Json(EntityResponse {
            message: "Successfully get profile.".to_string(),
            data: Some(result),
            total: 1,
        })),
        Err(err) => {
            log::warn!("Unsuccessfully get profile user: {err:?}.");
            Err(err)
        },
    }
}

#[utoipa::path(
    post,
    path = "/v1/logout",
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success logout", body = EntityResponse<String>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_logout(
    State(state): State<AppState>,
    claims: UserClaims,
) -> AppResult<Json<EntityResponse<String>>> {
    log::info!("Logout user id: {}", claims.user_id);
    let tx = state.db.begin().await?;

    match state.user_service.logout(&tx, claims.user_id).await {
        Ok(_) => {
            log::info!("Success logout user id: {}", claims.user_id);
            Ok(Json(EntityResponse {
                message: "Successfully logged out.".to_string(),
                data: Some("Successfully logged out.".to_string()),
                total: 1,
            }))
        },
        Err(err) => {
            error!("Unsuccessfully logout user: {err:?}");
            Err(err)
        },
    }
}
