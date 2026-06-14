use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::identity::get_user_profile;
use crate::application::identity::user_profile::UserProfileError;
use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::http::identity::dto::UserProfileResponse;
use crate::infra::postgres::identity::user_profile_store::PostgresUserProfileStore;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::utils::request_auth::authenticated_user;

pub(super) async fn get_user(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    if requester.user_id != user_id {
        match user_permission_platform_request(
            &mut conn,
            requester.user_id,
            &Permissions::VIEW_USER.to_string(),
        )
        .await
        {
            Ok(true) => {}
            Ok(false) => {
                return HttpResponse::Forbidden().body("User does not have the required permission")
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to check user permission")
            }
        }
    }

    let mut store = PostgresUserProfileStore::new(&mut conn);

    match get_user_profile::get_user_profile(&mut store, user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserProfileResponse::from(user)),
        Err(UserProfileError::NotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!(
                "event=user_fetch_failed user_id={} error={}",
                user_id,
                user_profile_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

fn user_profile_error_log(error: &UserProfileError) -> String {
    match error {
        UserProfileError::NotFound => "not_found".to_string(),
        UserProfileError::Database(message) => message.clone(),
    }
}
