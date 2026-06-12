#[post("/register")]
pub async fn register(
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
        Err(err) => {
            return registration_db_error_response(err, &req.email);
        }
    };

    print_mock_verification_email(
        &inserted_user.email,
        &inserted_user.name,
        &verification_token,
    );

    HttpResponse::Ok().body("Registration successful")
}

#[get("/verify-email")]
pub async fn verify_email(
    pool: web::Data<db::DbPool>,
    query: web::Query<VerifyEmailQuery>,
) -> impl Responder {
    let token = query.token.trim();
    if token.is_empty() {
        return HttpResponse::BadRequest().body("Verification token is required");
    }

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let token_hash = verification_token_hash(token);
    match EmailVerificationToken::verify(&mut conn, &token_hash).await {
        Ok(EmailVerificationResult::Verified) => {
            HttpResponse::Ok().body("Email verified successfully")
        }
        Ok(EmailVerificationResult::AlreadyVerified) => {
            HttpResponse::Ok().body("Email already verified")
        }
        Ok(EmailVerificationResult::Expired) => {
            HttpResponse::BadRequest().body("Verification token expired")
        }
        Ok(EmailVerificationResult::Invalid) => {
            HttpResponse::BadRequest().body("Invalid verification token")
        }
        Err(err) => {
            log::error!("event=email_verification_failed error={}", err);
            HttpResponse::InternalServerError().body("Failed to verify email token")
        }
    }
}

#[get("/user_id")]
pub async fn user_id(req: HttpRequest) -> impl Responder {
    match authenticated_user_id(&req) {
        Ok(user_id) => HttpResponse::Ok().body(format!("Hello! Your ID is {}", user_id)),
        Err(response) => response,
    }
}

pub async fn jwks() -> impl Responder {
    match public_jwks_from_env() {
        Ok(jwks) => HttpResponse::Ok().json(jwks),
        Err(err) => {
            log::error!("event=jwks_build_failed error={}", err);
            HttpResponse::InternalServerError().body("Failed to build JWKS response")
        }
    }
}

// Define the scope for authentication-related routes
pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(forgot_password)
        .service(login)
        .service(register)
        .service(resend_verification)
        .service(reset_password)
        .service(verify_email)
        .service(user_id)
}

#[cfg(test)]
mod tests;
