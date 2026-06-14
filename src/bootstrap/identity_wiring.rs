use std::sync::Arc;

use actix_web::web;

use crate::application::identity::assign_platform_role::PlatformRoleAssignmentUseCase;
use crate::application::identity::current_session::CurrentSessionUseCase;
use crate::application::identity::get_user_profile::UserProfileReadUseCase;
use crate::application::identity::list_users::UserListUseCase;
use crate::application::identity::login::LoginUseCase;
use crate::application::identity::register::RegisterUseCase;
use crate::application::identity::request_password_reset::RequestPasswordResetUseCase;
use crate::application::identity::resend_verification::ResendVerificationUseCase;
use crate::application::identity::reset_password::ResetPasswordUseCase;
use crate::application::identity::verify_email::VerifyEmailUseCase;
use crate::db::DbPool;
use crate::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
use crate::infra::postgres::identity::login_use_case::PostgresLoginUseCase;
use crate::infra::postgres::identity::platform_role_assignment_use_case::PostgresPlatformRoleAssignmentUseCase;
use crate::infra::postgres::identity::registration_use_case::PostgresRegisterUseCase;
use crate::infra::postgres::identity::request_password_reset_use_case::PostgresRequestPasswordResetUseCase;
use crate::infra::postgres::identity::resend_verification_use_case::PostgresResendVerificationUseCase;
use crate::infra::postgres::identity::reset_password_use_case::PostgresResetPasswordUseCase;
use crate::infra::postgres::identity::user_list_use_case::PostgresUserListUseCase;
use crate::infra::postgres::identity::user_profile_read_use_case::PostgresUserProfileReadUseCase;
use crate::infra::postgres::identity::verify_email_use_case::PostgresVerifyEmailUseCase;

#[derive(Clone)]
pub struct IdentityUseCases {
    pub current_session: Arc<dyn CurrentSessionUseCase>,
    pub login: Arc<dyn LoginUseCase>,
    pub platform_role_assignment: Arc<dyn PlatformRoleAssignmentUseCase>,
    pub register: Arc<dyn RegisterUseCase>,
    pub request_password_reset: Arc<dyn RequestPasswordResetUseCase>,
    pub reset_password: Arc<dyn ResetPasswordUseCase>,
    pub resend_verification: Arc<dyn ResendVerificationUseCase>,
    pub user_list: Arc<dyn UserListUseCase>,
    pub user_profile: Arc<dyn UserProfileReadUseCase>,
    pub verify_email: Arc<dyn VerifyEmailUseCase>,
}

pub fn build_identity_use_cases(pool: &DbPool) -> IdentityUseCases {
    IdentityUseCases {
        current_session: Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())),
        login: Arc::new(PostgresLoginUseCase::new(pool.clone())),
        platform_role_assignment: Arc::new(PostgresPlatformRoleAssignmentUseCase::new(
            pool.clone(),
            crate::utils::notifications::NotificationsState::new(pool.clone()),
        )),
        register: Arc::new(PostgresRegisterUseCase::new(pool.clone())),
        request_password_reset: Arc::new(PostgresRequestPasswordResetUseCase::new(pool.clone())),
        reset_password: Arc::new(PostgresResetPasswordUseCase::new(pool.clone())),
        resend_verification: Arc::new(PostgresResendVerificationUseCase::new(pool.clone())),
        user_list: Arc::new(PostgresUserListUseCase::new(pool.clone())),
        user_profile: Arc::new(PostgresUserProfileReadUseCase::new(pool.clone())),
        verify_email: Arc::new(PostgresVerifyEmailUseCase::new(pool.clone())),
    }
}

pub fn configure_identity_app_data(cfg: &mut web::ServiceConfig, identity: &IdentityUseCases) {
    cfg.app_data(web::Data::new(identity.current_session.clone()))
        .app_data(web::Data::new(identity.login.clone()))
        .app_data(web::Data::new(identity.platform_role_assignment.clone()))
        .app_data(web::Data::new(identity.register.clone()))
        .app_data(web::Data::new(identity.request_password_reset.clone()))
        .app_data(web::Data::new(identity.reset_password.clone()))
        .app_data(web::Data::new(identity.resend_verification.clone()))
        .app_data(web::Data::new(identity.user_list.clone()))
        .app_data(web::Data::new(identity.user_profile.clone()))
        .app_data(web::Data::new(identity.verify_email.clone()));
}
