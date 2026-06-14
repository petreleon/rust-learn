use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::identity::get_user_profile::{
    GetUserProfileCommand, UserProfileReadUseCase,
};
use crate::application::identity::user_profile::UserProfileError;
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::identity::dto::UserProfileResponse;

pub(super) async fn get_user(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn UserProfileReadUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let command = GetUserProfileCommand {
        requester_user_id: requester.user_id,
        target_user_id: path.into_inner(),
    };
    match use_case.get_user_profile(command).await {
        Ok(user) => HttpResponse::Ok().json(UserProfileResponse::from(user)),
        Err(UserProfileError::Forbidden) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        Err(UserProfileError::NotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!(
                "event=user_fetch_failed user_id={} error={}",
                command.target_user_id,
                user_profile_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

fn user_profile_error_log(error: &UserProfileError) -> String {
    match error {
        UserProfileError::Forbidden => "forbidden".to_string(),
        UserProfileError::NotFound => "not_found".to_string(),
        UserProfileError::Database(message) => message.clone(),
    }
}
