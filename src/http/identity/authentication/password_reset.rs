use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{non_truncating_hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::Deserialize;

use crate::application::identity::password_policy::validate_password_strength;
use crate::db;
use crate::models::password_reset_token::{PasswordResetResult, PasswordResetToken};
use crate::utils::email::verification_token_hash;

#[derive(Deserialize)]
pub(super) struct ResetPasswordRequest {
    token: String,
    password: String,
}

#[post("/reset-password")]
pub(super) async fn reset_password(
    pool: web::Data<db::DbPool>,
    req: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let token = req.token.trim();
    if token.is_empty() {
        return HttpResponse::BadRequest().body("Password reset token is required");
    }
    if let Err(message) = validate_password_strength(&req.password) {
        return HttpResponse::BadRequest().body(message);
    }

    let password_hash = match non_truncating_hash(&req.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            log::error!("event=password_reset_hash_failed error={}", err);
            return HttpResponse::InternalServerError().body("Failed to reset password");
        }
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let token_hash = verification_token_hash(token);

    let result = conn
        .transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let result = PasswordResetToken::consume(conn, &token_hash).await?;
                let PasswordResetResult::Reset {
                    user_id: reset_user_id,
                } = result
                else {
                    return Ok(result);
                };
                use crate::db::schema::authentications::dsl as auths;
                let updated = diesel::update(
                    auths::authentications
                        .filter(auths::user_id.eq(reset_user_id))
                        .filter(auths::type_authentication.eq("password")),
                )
                .set(auths::info_auth.eq(Some(password_hash)))
                .execute(conn)
                .await?;
                if updated == 0 {
                    return Err(DieselError::NotFound);
                }
                Ok(PasswordResetResult::Reset {
                    user_id: reset_user_id,
                })
            })
        })
        .await;

    match result {
        Ok(PasswordResetResult::Reset { .. }) => {
            HttpResponse::Ok().body("Password updated successfully")
        }
        Ok(PasswordResetResult::Expired) => {
            HttpResponse::BadRequest().body("Password reset token expired")
        }
        Ok(PasswordResetResult::Invalid) => {
            HttpResponse::BadRequest().body("Invalid password reset token")
        }
        Err(err) => {
            log::error!("event=password_reset_failed error={err}");
            HttpResponse::InternalServerError().body("Failed to reset password")
        }
    }
}
