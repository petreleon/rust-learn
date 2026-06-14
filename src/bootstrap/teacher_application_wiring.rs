use std::sync::Arc;

use actix_web::web;

use crate::application::teacher_applications::decide_application::TeacherApplicationDecisionUseCase;
use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfUseCase;
use crate::application::teacher_applications::list_application_audit::TeacherApplicationAuditUseCase;
use crate::application::teacher_applications::list_applications::TeacherApplicationListUseCase;
use crate::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewUseCase;
use crate::application::teacher_applications::nominate_application::TeacherApplicationNominationUseCase;
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::application::teacher_applications::submit_application::TeacherApplicationSubmitUseCase;
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_audit_use_case::PostgresTeacherApplicationAuditUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_decision_use_case::PostgresTeacherApplicationDecisionUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_list_use_case::PostgresTeacherApplicationListUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_nomination_use_case::PostgresTeacherApplicationNominationUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_notification_use_case::PostgresTeacherApplicationNotificationUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_use_case::PostgresTeacherApplicationPlatformReviewUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_self_use_case::PostgresTeacherApplicationSelfUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_submit_use_case::PostgresTeacherApplicationSubmitUseCase;

#[derive(Clone)]
pub struct TeacherApplicationUseCases {
    pub audit: Arc<dyn TeacherApplicationAuditUseCase>,
    pub decision: Arc<dyn TeacherApplicationDecisionUseCase>,
    pub list: Arc<dyn TeacherApplicationListUseCase>,
    pub nomination: Arc<dyn TeacherApplicationNominationUseCase>,
    pub notification: Arc<dyn TeacherApplicationNotificationUseCase>,
    pub platform_review: Arc<dyn TeacherApplicationPlatformReviewUseCase>,
    pub self_status: Arc<dyn TeacherApplicationSelfUseCase>,
    pub submit: Arc<dyn TeacherApplicationSubmitUseCase>,
}

pub fn build_teacher_application_use_cases(pool: &DbPool) -> TeacherApplicationUseCases {
    TeacherApplicationUseCases {
        audit: Arc::new(PostgresTeacherApplicationAuditUseCase::new(pool.clone())),
        decision: Arc::new(PostgresTeacherApplicationDecisionUseCase::new(pool.clone())),
        list: Arc::new(PostgresTeacherApplicationListUseCase::new(pool.clone())),
        nomination: Arc::new(PostgresTeacherApplicationNominationUseCase::new(
            pool.clone(),
        )),
        notification: Arc::new(PostgresTeacherApplicationNotificationUseCase::new(
            pool.clone(),
            crate::utils::notifications::NotificationsState::new(pool.clone()),
        )),
        platform_review: Arc::new(PostgresTeacherApplicationPlatformReviewUseCase::new(
            pool.clone(),
        )),
        self_status: Arc::new(PostgresTeacherApplicationSelfUseCase::new(pool.clone())),
        submit: Arc::new(PostgresTeacherApplicationSubmitUseCase::new(pool.clone())),
    }
}

pub fn configure_teacher_application_app_data(
    cfg: &mut web::ServiceConfig,
    use_cases: &TeacherApplicationUseCases,
) {
    cfg.app_data(web::Data::new(use_cases.audit.clone()))
        .app_data(web::Data::new(use_cases.decision.clone()))
        .app_data(web::Data::new(use_cases.list.clone()))
        .app_data(web::Data::new(use_cases.nomination.clone()))
        .app_data(web::Data::new(use_cases.notification.clone()))
        .app_data(web::Data::new(use_cases.platform_review.clone()))
        .app_data(web::Data::new(use_cases.self_status.clone()))
        .app_data(web::Data::new(use_cases.submit.clone()));
}
