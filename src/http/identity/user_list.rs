use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::identity::list_users::UserListUseCase;
use crate::application::identity::user_profile::UserProfileError;
use crate::http::identity::dto::{ListUsersRequest, UsersResponse};

pub(super) async fn list_users(
    use_case: web::Data<Arc<dyn UserListUseCase>>,
    query: web::Query<ListUsersRequest>,
) -> impl Responder {
    match use_case.list_users(query.into_inner().into()).await {
        Ok(users) => HttpResponse::Ok().json(UsersResponse::from(users)),
        Err(e) => {
            log::error!(
                "event=user_list_failed error={}",
                user_profile_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to load users")
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
