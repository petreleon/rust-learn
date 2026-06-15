use std::sync::Arc;

use actix_web::web;

use crate::application::identity::get_user_profile::{
    GetUserProfileCommand, UserProfileReadUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::identity::dto::UserProfileResponse;
use crate::http::identity::errors::get_user_profile_error;

pub(super) async fn get_user(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn UserProfileReadUseCase>>,
) -> Result<web::Json<UserProfileResponse>, ApiError> {
    let command = GetUserProfileCommand {
        requester_user_id: requester.user_id(),
        target_user_id: path.into_inner(),
    };
    use_case
        .get_user_profile(command.clone())
        .await
        .map(UserProfileResponse::from)
        .map(web::Json)
        .map_err(|error| get_user_profile_error(command.target_user_id, error))
}
