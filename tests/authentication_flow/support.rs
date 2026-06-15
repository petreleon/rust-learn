pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bcrypt::verify;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::RunQueryDsl;
pub(crate) use rust_learn::application::identity::login::LoginUseCase;
pub(crate) use rust_learn::application::identity::register::RegisterUseCase;
pub(crate) use rust_learn::application::identity::request_password_reset::RequestPasswordResetUseCase;
pub(crate) use rust_learn::application::identity::resend_verification::ResendVerificationUseCase;
pub(crate) use rust_learn::application::identity::reset_password::ResetPasswordUseCase;
pub(crate) use rust_learn::application::identity::verify_email::VerifyEmailUseCase;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::accounts::{
    find_user_by_email, find_user_by_id,
};
pub(crate) use rust_learn::infra::postgres::identity::email_verification_tokens::create_email_verification_token;
pub(crate) use rust_learn::infra::postgres::identity::login_use_case::PostgresLoginUseCase;
pub(crate) use rust_learn::infra::postgres::identity::password_reset_tokens::create_password_reset_token;
pub(crate) use rust_learn::infra::postgres::identity::registration_use_case::PostgresRegisterUseCase;
pub(crate) use rust_learn::infra::postgres::identity::request_password_reset_use_case::PostgresRequestPasswordResetUseCase;
pub(crate) use rust_learn::infra::postgres::identity::resend_verification_use_case::PostgresResendVerificationUseCase;
pub(crate) use rust_learn::infra::postgres::identity::reset_password_use_case::PostgresResetPasswordUseCase;
pub(crate) use rust_learn::infra::postgres::identity::verify_email_use_case::PostgresVerifyEmailUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{
    authentications, email_verification_tokens, password_reset_tokens, user_role_platform, users,
};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::identity::identity_token_hash;
pub(crate) use rust_learn::infra::tokens::jwt::decode_jwt;
pub(crate) use std::sync::Arc;

pub(crate) fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}+{}-{}@example.com", prefix, std::process::id(), ts)
}

pub(crate) fn unique_token(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}-{}-{}", prefix, std::process::id(), ts)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) fn auth_test_app(
    pool: DbPool,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .configure(|cfg| rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool))
        .app_data(login_use_case_data(&pool))
        .app_data(register_use_case_data(&pool))
        .app_data(request_password_reset_use_case_data(&pool))
        .app_data(reset_password_use_case_data(&pool))
        .app_data(resend_verification_use_case_data(&pool))
        .app_data(verify_email_use_case_data(&pool))
        .service(web::scope("/api").service(rust_learn::http::identity::auth_scope()))
}

pub(crate) fn login_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn LoginUseCase>> {
    web::Data::new(Arc::new(PostgresLoginUseCase::new(pool.clone())))
}

pub(crate) fn register_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn RegisterUseCase>> {
    web::Data::new(Arc::new(PostgresRegisterUseCase::new(pool.clone())))
}

pub(crate) fn request_password_reset_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn RequestPasswordResetUseCase>> {
    web::Data::new(Arc::new(PostgresRequestPasswordResetUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn reset_password_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn ResetPasswordUseCase>> {
    web::Data::new(Arc::new(PostgresResetPasswordUseCase::new(pool.clone())))
}

pub(crate) fn resend_verification_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn ResendVerificationUseCase>> {
    web::Data::new(Arc::new(PostgresResendVerificationUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn verify_email_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn VerifyEmailUseCase>> {
    web::Data::new(Arc::new(PostgresVerifyEmailUseCase::new(pool.clone())))
}
