use actix_web::{get, HttpResponse, Responder};

use crate::http::extractors::auth_user::AuthUserId;

#[get("/user_id")]
pub(super) async fn user_id(user: AuthUserId) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! Your ID is {}", user.into_inner()))
}
