use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::utils::request_auth::authenticated_user_id;

#[get("/user_id")]
pub(super) async fn user_id(req: HttpRequest) -> impl Responder {
    match authenticated_user_id(&req) {
        Ok(user_id) => HttpResponse::Ok().body(format!("Hello! Your ID is {}", user_id)),
        Err(response) => response,
    }
}
