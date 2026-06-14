use actix_web::web;

use crate::bootstrap::app_state::AppState;

pub fn configure_app_data(cfg: &mut web::ServiceConfig, app_state: &AppState) {
    cfg.app_data(web::Data::new(app_state.pool.clone()))
        .app_data(web::Data::new(app_state.s3.clone()))
        .app_data(web::Data::new(app_state.notifications.clone()))
        .app_data(web::Data::new(app_state.readiness_use_case.clone()));

    crate::bootstrap::access_control_wiring::configure_access_control_app_data(
        cfg,
        &app_state.access_control_use_cases,
    );
    crate::bootstrap::content_wiring::configure_content_app_data(cfg, &app_state.content_use_cases);
    crate::bootstrap::identity_wiring::configure_identity_app_data(
        cfg,
        &app_state.identity_use_cases,
    );
    crate::bootstrap::kyc_wiring::configure_kyc_app_data(cfg, &app_state.kyc_use_cases);
    crate::bootstrap::learning_wiring::configure_learning_app_data(
        cfg,
        &app_state.learning_use_cases,
    );
    crate::bootstrap::notification_wiring::configure_notification_app_data(
        cfg,
        &app_state.notification_use_cases,
    );
    crate::bootstrap::organization_wiring::configure_organization_app_data(
        cfg,
        &app_state.organization_use_cases,
    );
    crate::bootstrap::reporting_wiring::configure_reporting_app_data(
        cfg,
        &app_state.reporting_use_cases,
    );
    crate::bootstrap::reward_wiring::configure_reward_app_data(cfg, &app_state.reward_use_cases);
    crate::bootstrap::teacher_application_wiring::configure_teacher_application_app_data(
        cfg,
        &app_state.teacher_application_use_cases,
    );
    crate::bootstrap::wallet_wiring::configure_wallet_app_data(cfg, &app_state.wallet_use_cases);
}
