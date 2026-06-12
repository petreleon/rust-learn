const PASSWORD_RESET_REQUEST_MESSAGE: &str =
    "If an account matches that email, a password reset link has been sent.";

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[post("/forgot-password")]
pub async fn forgot_password(
    pool: web::Data<db::DbPool>,
    req: web::Json<ForgotPasswordRequest>,
) -> impl Responder {
    let email = normalize_email(&req.email);
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Email is required");
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user = match User::find_by_email(&email, &mut conn).await.optional() {
        Ok(user) => user,
        Err(err) => {
            log::error!("event=password_reset_lookup_failed error={err}");
            return HttpResponse::InternalServerError().body("Failed to request password reset");
        }
    };

    if let Some(user) = user {
        let reset_token = match generate_verification_token() {
            Ok(token) => token,
            Err(err) => {
                log::error!("event=password_reset_token_generate_failed error={}", err);
                return HttpResponse::InternalServerError()
                    .body("Failed to create password reset token");
            }
        };

        if let Err(err) = PasswordResetToken::create_for_user(
            &mut conn,
            user.id(),
            verification_token_hash(&reset_token),
        )
        .await
        {
            log::error!("event=password_reset_token_store_failed error={err}");
            return HttpResponse::InternalServerError().body("Failed to request password reset");
        }

        print_mock_password_reset_email(&user.email, &user.name, &reset_token);
    } else {
        log::info!(
            "event=password_reset_requested_unknown_email email_hash={}",
            email_log_hash(&email)
        );
    }

    HttpResponse::Ok().body(PASSWORD_RESET_REQUEST_MESSAGE)
}

#[post("/reset-password")]
pub async fn reset_password(
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
