use std::sync::Arc;

use crate::bootstrap::access_control_wiring::build_access_control_use_cases;
use crate::bootstrap::app_state::AppState;
use crate::bootstrap::content_wiring::build_content_use_cases;
use crate::bootstrap::identity_wiring::build_identity_use_cases;
use crate::bootstrap::kyc_wiring::build_kyc_use_cases;
use crate::bootstrap::learning_wiring::build_learning_use_cases;
use crate::bootstrap::notification_wiring::build_notification_use_cases;
use crate::bootstrap::organization_wiring::build_organization_use_cases;
use crate::bootstrap::readiness::RuntimeReadinessUseCase;
use crate::bootstrap::reporting_wiring::build_reporting_use_cases;
use crate::bootstrap::reward_wiring::build_reward_use_cases;
use crate::bootstrap::teacher_application_wiring::build_teacher_application_use_cases;
use crate::bootstrap::wallet_wiring::build_wallet_use_cases;
use crate::db::DbPool;
use crate::infra::object_storage::S3State;

pub fn build_app_state(pool: DbPool, s3: S3State) -> AppState {
    AppState {
        access_control_use_cases: build_access_control_use_cases(&pool),
        content_use_cases: build_content_use_cases(&pool, &s3),
        identity_use_cases: build_identity_use_cases(&pool),
        kyc_use_cases: build_kyc_use_cases(&pool),
        learning_use_cases: build_learning_use_cases(&pool),
        notification_use_cases: build_notification_use_cases(&pool),
        organization_use_cases: build_organization_use_cases(&pool),
        reporting_use_cases: build_reporting_use_cases(&pool),
        reward_use_cases: build_reward_use_cases(&pool),
        teacher_application_use_cases: build_teacher_application_use_cases(&pool),
        wallet_use_cases: build_wallet_use_cases(&pool),
        readiness_use_case: Arc::new(RuntimeReadinessUseCase::new(pool.clone(), s3.clone())),
        notifications: crate::infra::notifications::NotificationsState::new(pool.clone()),
        pool,
        s3,
    }
}
