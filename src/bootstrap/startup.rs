use std::sync::Arc;

use crate::bootstrap::app_state::AppState;
use crate::bootstrap::contract_startup::deploy_startup_contracts;
use crate::bootstrap::readiness::RuntimeReadinessUseCase;
use crate::config::db_setup::version_updater;
use crate::db;
use crate::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;
use crate::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use crate::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use crate::infra::postgres::content::media_url_use_case::PostgresContentMediaUrlUseCase;
use crate::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use crate::infra::postgres::content::upload_url_use_case::PostgresContentUploadUrlUseCase;
use crate::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
use crate::infra::postgres::learning::course_deletion_use_case::PostgresCourseDeletionUseCase;
use crate::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
use crate::infra::postgres::learning::course_lifecycle_use_case::PostgresCourseLifecycleUseCase;
use crate::infra::postgres::learning::course_organization_use_case::PostgresCourseOrganizationsUseCase;
use crate::infra::postgres::learning::course_read_use_case::PostgresCourseReadUseCase;
use crate::infra::postgres::learning::course_update_use_case::PostgresCourseUpdateUseCase;
use crate::infra::postgres::notifications::notification_inbox_use_case::PostgresNotificationInboxUseCase;
use crate::infra::postgres::notifications::notification_preferences_use_case::PostgresNotificationPreferencesUseCase;
use crate::infra::postgres::reporting::organization_reward_dashboard_use_case::PostgresOrganizationRewardDashboardUseCase;
use crate::infra::postgres::reporting::organization_summary_use_case::PostgresOrganizationSummaryUseCase;
use crate::infra::postgres::reporting::platform_csv_export_use_case::PostgresPlatformCsvExportsUseCase;
use crate::infra::postgres::reporting::platform_fraud_dashboard_use_case::PostgresPlatformFraudDashboardUseCase;
use crate::infra::postgres::reporting::platform_reward_dashboard_use_case::PostgresPlatformRewardDashboardUseCase;
use crate::infra::postgres::reporting::platform_summary_use_case::PostgresPlatformSummaryUseCase;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_use_case::PostgresPlatformWalletReconciliationUseCase;
use crate::infra::postgres::rewards::course_reward_candidate_use_case::PostgresCourseRewardCandidatesUseCase;
use crate::infra::postgres::rewards::platform_reward_candidate_use_case::PostgresPlatformRewardCandidatesUseCase;
use crate::infra::postgres::rewards::reward_amount_decision_use_case::PostgresRewardAmountDecisionUseCase;
use crate::infra::postgres::rewards::reward_candidate_audit_use_case::PostgresRewardCandidateAuditUseCase;
use crate::infra::postgres::rewards::reward_candidate_submission_use_case::PostgresRewardCandidateSubmissionUseCase;
use crate::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
use crate::infra::postgres::rewards::reward_history_use_case::PostgresStudentRewardHistoryUseCase;
use crate::infra::postgres::rewards::reward_policy_use_case::PostgresRewardPolicyUseCase;
use crate::infra::postgres::rewards::teacher_reward_candidate_decision_use_case::PostgresTeacherRewardCandidateDecisionUseCase;
use crate::infra::postgres::wallet::wallet_audit_use_case::PostgresWalletAuditUseCase;
use crate::infra::postgres::wallet::wallet_deposit_intent_use_case::PostgresWalletDepositIntentUseCase;
use crate::infra::postgres::wallet::wallet_link_use_case::PostgresWalletLinkUseCase;
use crate::infra::postgres::wallet::wallet_read_use_case::PostgresWalletReadUseCase;
use crate::infra::postgres::wallet::wallet_retirement_use_case::PostgresWalletRetirementUseCase;
use crate::infra::postgres::wallet::wallet_token_tax_use_case::PostgresWalletTokenTaxUseCase;
use crate::utils::notifications::NotificationsState;
use crate::utils::s3_utils::S3State;

pub async fn initialize_app_state() -> std::io::Result<AppState> {
    let pool = db::try_establish_connection().map_err(|error| {
        log::error!("event=db_pool_init_failed error={}", error);
        std::io::Error::other(error)
    })?;

    let s3 = S3State::new_from_env().await.map_err(|error| {
        log::error!("event=s3_init_failed error={:?}", error);
        std::io::Error::other("S3 init failed")
    })?;

    run_startup_tasks(&pool).await?;

    Ok(AppState {
        role_catalog_use_case: Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())),
        current_session_use_case: Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())),
        course_deletion_use_case: Arc::new(PostgresCourseDeletionUseCase::new(pool.clone())),
        course_discovery_use_case: Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())),
        course_read_use_case: Arc::new(PostgresCourseReadUseCase::new(pool.clone())),
        course_organizations_use_case: Arc::new(PostgresCourseOrganizationsUseCase::new(
            pool.clone(),
        )),
        course_update_use_case: Arc::new(PostgresCourseUpdateUseCase::new(pool.clone())),
        course_lifecycle_use_case: Arc::new(PostgresCourseLifecycleUseCase::new(pool.clone())),
        notification_inbox_use_case: Arc::new(PostgresNotificationInboxUseCase::new(pool.clone())),
        notification_preferences_use_case: Arc::new(PostgresNotificationPreferencesUseCase::new(
            pool.clone(),
        )),
        chapter_use_cases: Arc::new(PostgresChapterUseCases::new(pool.clone())),
        content_item_use_cases: Arc::new(PostgresContentItemUseCases::new(pool.clone())),
        content_upload_url_use_case: Arc::new(PostgresContentUploadUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
        content_media_url_use_case: Arc::new(PostgresContentMediaUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
        content_processing_use_case: Arc::new(PostgresContentProcessingUseCase::new(pool.clone())),
        reward_fraud_block_use_case: Arc::new(PostgresRewardFraudBlockUseCase::new(pool.clone())),
        reward_candidate_audit_use_case: Arc::new(PostgresRewardCandidateAuditUseCase::new(
            pool.clone(),
        )),
        reward_amount_decision_use_case: Arc::new(PostgresRewardAmountDecisionUseCase::new(
            pool.clone(),
        )),
        reward_candidate_submission_use_case: Arc::new(
            PostgresRewardCandidateSubmissionUseCase::new(pool.clone()),
        ),
        course_reward_candidates_use_case: Arc::new(PostgresCourseRewardCandidatesUseCase::new(
            pool.clone(),
        )),
        platform_reward_candidates_use_case: Arc::new(
            PostgresPlatformRewardCandidatesUseCase::new(pool.clone()),
        ),
        teacher_reward_candidate_decision_use_case: Arc::new(
            PostgresTeacherRewardCandidateDecisionUseCase::new(pool.clone()),
        ),
        student_reward_history_use_case: Arc::new(PostgresStudentRewardHistoryUseCase::new(
            pool.clone(),
        )),
        reward_policy_use_case: Arc::new(PostgresRewardPolicyUseCase::new(pool.clone())),
        wallet_audit_use_case: Arc::new(PostgresWalletAuditUseCase::new(pool.clone())),
        wallet_deposit_intent_use_case: Arc::new(PostgresWalletDepositIntentUseCase::new(
            pool.clone(),
        )),
        wallet_link_use_case: Arc::new(PostgresWalletLinkUseCase::new(pool.clone())),
        wallet_read_use_case: Arc::new(PostgresWalletReadUseCase::new(pool.clone())),
        wallet_retirement_use_case: Arc::new(PostgresWalletRetirementUseCase::new(pool.clone())),
        wallet_token_tax_use_case: Arc::new(PostgresWalletTokenTaxUseCase::new(pool.clone())),
        organization_reward_dashboard_use_case: Arc::new(
            PostgresOrganizationRewardDashboardUseCase::new(pool.clone()),
        ),
        organization_summary_use_case: Arc::new(PostgresOrganizationSummaryUseCase::new(
            pool.clone(),
        )),
        platform_csv_exports_use_case: Arc::new(PostgresPlatformCsvExportsUseCase::new(
            pool.clone(),
        )),
        platform_fraud_dashboard_use_case: Arc::new(PostgresPlatformFraudDashboardUseCase::new(
            pool.clone(),
        )),
        platform_reward_dashboard_use_case: Arc::new(PostgresPlatformRewardDashboardUseCase::new(
            pool.clone(),
        )),
        platform_summary_use_case: Arc::new(PostgresPlatformSummaryUseCase::new(pool.clone())),
        platform_wallet_reconciliation_use_case: Arc::new(
            PostgresPlatformWalletReconciliationUseCase::new(pool.clone()),
        ),
        readiness_use_case: Arc::new(RuntimeReadinessUseCase::new(pool.clone(), s3.clone())),
        notifications: NotificationsState::new(pool.clone()),
        pool,
        s3,
    })
}

async fn run_startup_tasks(pool: &db::DbPool) -> std::io::Result<()> {
    let mut conn = pool.get().await.map_err(|error| {
        log::error!("event=db_connection_failed phase=startup error={:?}", error);
        std::io::Error::other(format!("Failed to get DB connection from pool: {error}"))
    })?;

    version_updater(&mut conn).await.map_err(|error| {
        log::error!("event=db_version_update_failed error={:?}", error);
        std::io::Error::other(format!("Failed to update database version: {error}"))
    })?;

    deploy_startup_contracts(&mut conn).await;
    Ok(())
}
