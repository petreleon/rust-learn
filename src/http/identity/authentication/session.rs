use actix_web::get;

use crate::http::extractors::auth_user::AuthUserId;

#[get("/user_id")]
pub(super) async fn user_id(user: AuthUserId) -> String {
    format!("Hello! Your ID is {}", user.into_inner())
}
