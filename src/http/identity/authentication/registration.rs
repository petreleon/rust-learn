use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{non_truncating_hash, DEFAULT_COST};
use chrono::NaiveDate;
use diesel::result::Error as DieselError;
use diesel_async::AsyncConnection;
use serde::Deserialize;

use super::password_policy::validate_password_strength;
use super::support::{normalize_email, registration_db_error_response};
use crate::db;
use crate::models::authentication::Authentication;
use crate::models::email_verification_token::EmailVerificationToken;
use crate::models::role::PlatformRole;
use crate::models::user::{NewUser, User};
use crate::models::user_role_platform::UserRolePlatform;
use crate::utils::email::{
    generate_verification_token, print_mock_verification_email, verification_token_hash,
};

#[derive(Deserialize)]
pub(super) struct RegisterRequest {
    email: String,
    password: String,
    name: String,
    date_of_birth: Option<NaiveDate>,
}

#[post("/register")]
pub(super) async fn register(
    pool: web::Data<db::DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
    }

    if let Err(message) = validate_password_strength(&req.password) {
        return HttpResponse::BadRequest().body(message);
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let hashed_password = match non_truncating_hash(&req.password, DEFAULT_COST) {
        Ok(password_hash) => password_hash,
        Err(err) => {
            log::error!("event=auth_password_hash_failed error={}", err);
            return HttpResponse::InternalServerError().body("Failed to register user");
        }
    };

    let verification_token = match generate_verification_token() {
        Ok(token) => token,
        Err(err) => {
            log::error!(
                "event=email_verification_token_generate_failed error={}",
                err
            );
            return HttpResponse::InternalServerError()
                .body("Failed to create email verification token");
        }
    };
    let token_hash = verification_token_hash(&verification_token);

    let new_user_data = NewUser {
        name: req.name.to_string(),
        email,
        date_of_birth: req.date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
        email_verified: false,
    };

    let inserted_user = match conn
        .transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let inserted_user = User::create(new_user_data, conn).await?;
                let role_id = PlatformRole::find_by_name("STUDENT", conn).await?;
                UserRolePlatform::assign(conn, inserted_user.id(), role_id).await?;

                let new_auth = Authentication {
                    user_id: inserted_user.id(),
                    type_authentication: "password".to_string(),
                    info_auth: hashed_password,
                };
                Authentication::create(new_auth, conn).await?;
                EmailVerificationToken::create_for_user(conn, inserted_user.id(), token_hash)
                    .await?;

                Ok(inserted_user)
            })
        })
        .await
    {
        Ok(user) => user,
        Err(err) => return registration_db_error_response(err, &req.email),
    };

    print_mock_verification_email(
        &inserted_user.email,
        &inserted_user.name,
        &verification_token,
    );

    HttpResponse::Ok().body("Registration successful")
}
