use std::sync::Arc;

use crate::application::operations::readiness_check::ReadinessUseCase;
use crate::bootstrap::access_control_wiring::AccessControlUseCases;
use crate::bootstrap::content_wiring::ContentUseCases;
use crate::bootstrap::identity_wiring::IdentityUseCases;
use crate::bootstrap::kyc_wiring::KycUseCases;
use crate::bootstrap::learning_wiring::LearningUseCases;
use crate::bootstrap::notification_wiring::NotificationUseCases;
use crate::bootstrap::organization_wiring::OrganizationUseCases;
use crate::bootstrap::reporting_wiring::ReportingUseCases;
use crate::bootstrap::reward_wiring::RewardUseCases;
use crate::bootstrap::teacher_application_wiring::TeacherApplicationUseCases;
use crate::bootstrap::wallet_wiring::WalletUseCases;
use crate::db::DbPool;
use crate::infra::object_storage::S3State;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub s3: S3State,
    pub(crate) access_control_use_cases: AccessControlUseCases,
    pub(crate) content_use_cases: ContentUseCases,
    pub(crate) identity_use_cases: IdentityUseCases,
    pub(crate) kyc_use_cases: KycUseCases,
    pub(crate) learning_use_cases: LearningUseCases,
    pub(crate) notification_use_cases: NotificationUseCases,
    pub(crate) organization_use_cases: OrganizationUseCases,
    pub(crate) reporting_use_cases: ReportingUseCases,
    pub(crate) reward_use_cases: RewardUseCases,
    pub(crate) teacher_application_use_cases: TeacherApplicationUseCases,
    pub(crate) wallet_use_cases: WalletUseCases,
    pub readiness_use_case: Arc<dyn ReadinessUseCase>,
}
