use std::sync::Arc;

use crate::application::access_control::list_roles::RoleCatalogUseCase;
use crate::application::content::manage_chapter::ChapterUseCases;
use crate::application::content::manage_content_item::ContentItemUseCases;
use crate::application::content::process_upload_job::ContentProcessingUseCase;
use crate::application::content::request_media_url::ContentMediaUrlUseCase;
use crate::application::content::request_upload_url::ContentUploadUrlUseCase;
use crate::application::identity::current_session::CurrentSessionUseCase;
use crate::application::notifications::notification_inbox::NotificationInboxUseCase;
use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::application::operations::readiness_check::ReadinessUseCase;
use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionUseCase;
use crate::application::rewards::list_candidate_audit::RewardCandidateAuditUseCase;
use crate::application::rewards::list_course_candidates::CourseRewardCandidatesUseCase;
use crate::application::rewards::list_platform_candidates::PlatformRewardCandidatesUseCase;
use crate::application::rewards::list_reward_history::StudentRewardHistoryUseCase;
use crate::application::rewards::manage_fraud_block::RewardFraudBlockUseCase;
use crate::application::rewards::manage_reward_policy::RewardPolicyUseCase;
use crate::db::DbPool;
use crate::utils::notifications::NotificationsState;
use crate::utils::s3_utils::S3State;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub s3: S3State,
    pub notifications: NotificationsState,
    pub role_catalog_use_case: Arc<dyn RoleCatalogUseCase>,
    pub current_session_use_case: Arc<dyn CurrentSessionUseCase>,
    pub notification_inbox_use_case: Arc<dyn NotificationInboxUseCase>,
    pub notification_preferences_use_case: Arc<dyn NotificationPreferencesUseCase>,
    pub chapter_use_cases: Arc<dyn ChapterUseCases>,
    pub content_item_use_cases: Arc<dyn ContentItemUseCases>,
    pub content_upload_url_use_case: Arc<dyn ContentUploadUrlUseCase>,
    pub content_media_url_use_case: Arc<dyn ContentMediaUrlUseCase>,
    pub content_processing_use_case: Arc<dyn ContentProcessingUseCase>,
    pub reward_fraud_block_use_case: Arc<dyn RewardFraudBlockUseCase>,
    pub reward_candidate_audit_use_case: Arc<dyn RewardCandidateAuditUseCase>,
    pub course_reward_candidates_use_case: Arc<dyn CourseRewardCandidatesUseCase>,
    pub platform_reward_candidates_use_case: Arc<dyn PlatformRewardCandidatesUseCase>,
    pub teacher_reward_candidate_decision_use_case: Arc<dyn TeacherRewardCandidateDecisionUseCase>,
    pub student_reward_history_use_case: Arc<dyn StudentRewardHistoryUseCase>,
    pub reward_policy_use_case: Arc<dyn RewardPolicyUseCase>,
    pub readiness_use_case: Arc<dyn ReadinessUseCase>,
}
