use actix_web::{HttpResponse, Responder};

use crate::utils::jwt_utils::public_jwks_from_env;

pub async fn jwks() -> impl Responder {
    match public_jwks_from_env() {
        Ok(jwks) => HttpResponse::Ok().json(jwks),
        Err(err) => {
            log::error!("event=jwks_build_failed error={}", err);
            HttpResponse::InternalServerError().body("Failed to build JWKS response")
        }
    }
}
