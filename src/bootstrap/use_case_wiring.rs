use std::sync::Arc;

use crate::bootstrap::app_state::AppState;
use crate::bootstrap::readiness::RuntimeReadinessUseCase;
use crate::db::DbPool;
use crate::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;
use crate::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use crate::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use crate::infra::postgres::content::media_url_use_case::PostgresContentMediaUrlUseCase;
use crate::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use crate::infra::postgres::content::upload_url_use_case::PostgresContentUploadUrlUseCase;
use crate::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
use crate::infra::postgres::learning::assessment_read_use_case::PostgresAssessmentReadUseCase;
use crate::infra::postgres::learning::assessment_submission_use_case::PostgresAssessmentSubmissionUseCase;
use crate::infra::postgres::learning::course_creation_use_case::PostgresCourseCreationUseCase;
use crate::infra::postgres::learning::course_deletion_use_case::PostgresCourseDeletionUseCase;
use crate::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
use crate::infra::postgres::learning::course_enrollment_use_case::PostgresCourseEnrollmentUseCase;
use crate::infra::postgres::learning::course_lifecycle_use_case::PostgresCourseLifecycleUseCase;
use crate::infra::postgres::learning::course_organization_use_case::PostgresCourseOrganizationsUseCase;
use crate::infra::postgres::learning::course_read_use_case::PostgresCourseReadUseCase;
use crate::infra::postgres::learning::course_role_assignment_use_case::PostgresCourseRoleAssignmentUseCase;
use crate::infra::postgres::learning::course_update_use_case::PostgresCourseUpdateUseCase;
use crate::infra::postgres::learning::learner_course_catalog_list_use_case::PostgresLearnerCourseCatalogListUseCase;
use crate::infra::postgres::learning::learner_course_detail_use_case::PostgresLearnerCourseDetailUseCase;
use crate::infra::postgres::learning::learner_course_learning_use_case::PostgresLearnerCourseLearningUseCase;
use crate::infra::postgres::learning::learner_progress_use_case::PostgresLearnerProgressUseCase;
use crate::infra::postgres::learning::teacher_course_dashboard_list_use_case::PostgresTeacherCourseDashboardListUseCase;
use crate::infra::postgres::learning::teacher_course_enrollment_workspace_use_case::PostgresTeacherCourseEnrollmentWorkspaceUseCase;
use crate::infra::postgres::learning::teacher_course_students_use_case::PostgresTeacherCourseStudentsUseCase;
use crate::infra::postgres::learning::teacher_course_workspace_use_case::PostgresTeacherCourseWorkspaceUseCase;
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

pub fn build_app_state(pool: DbPool, s3: S3State) -> AppState {
    AppState {
        role_catalog_use_case: Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())),
        current_session_use_case: Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())),
        course_creation_use_case: Arc::new(PostgresCourseCreationUseCase::new(pool.clone())),
        course_deletion_use_case: Arc::new(PostgresCourseDeletionUseCase::new(pool.clone())),
        course_discovery_use_case: Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())),
        course_read_use_case: Arc::new(PostgresCourseReadUseCase::new(pool.clone())),
        learner_course_catalog_use_case: Arc::new(PostgresLearnerCourseCatalogListUseCase::new(
            pool.clone(),
        )),
        learner_course_detail_use_case: Arc::new(PostgresLearnerCourseDetailUseCase::new(
            pool.clone(),
        )),
        learner_course_learning_use_case: Arc::new(PostgresLearnerCourseLearningUseCase::new(
            pool.clone(),
        )),
        teacher_course_dashboard_use_case: Arc::new(
            PostgresTeacherCourseDashboardListUseCase::new(pool.clone()),
        ),
        teacher_course_workspace_use_case: Arc::new(PostgresTeacherCourseWorkspaceUseCase::new(
            pool.clone(),
        )),
        teacher_course_students_use_case: Arc::new(PostgresTeacherCourseStudentsUseCase::new(
            pool.clone(),
        )),
        teacher_course_enrollment_workspace_use_case: Arc::new(
            PostgresTeacherCourseEnrollmentWorkspaceUseCase::new(pool.clone()),
        ),
        course_organizations_use_case: Arc::new(PostgresCourseOrganizationsUseCase::new(
            pool.clone(),
        )),
        course_role_assignment_use_case: Arc::new(PostgresCourseRoleAssignmentUseCase::new(
            pool.clone(),
        )),
        course_enrollment_use_case: Arc::new(PostgresCourseEnrollmentUseCase::new(pool.clone())),
        course_update_use_case: Arc::new(PostgresCourseUpdateUseCase::new(pool.clone())),
        course_lifecycle_use_case: Arc::new(PostgresCourseLifecycleUseCase::new(pool.clone())),
        learner_progress_use_case: Arc::new(PostgresLearnerProgressUseCase::new(pool.clone())),
        course_assessments_use_case: Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())),
        assessment_attempts_use_case: Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())),
        assessment_submission_use_case: Arc::new(PostgresAssessmentSubmissionUseCase::new(
            pool.clone(),
        )),
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
    }
}
