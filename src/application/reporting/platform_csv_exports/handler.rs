use crate::application::reporting::platform_csv_exports::store::PlatformCsvExportStore;
use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformDelegatedPermissionExportRowOutput,
    PlatformRewardApprovalExportRowOutput, PlatformTeacherApplicationExportRowOutput,
    PlatformTokenPayoutExportRowOutput, PlatformWalletCreditExportRowOutput,
};

pub async fn load_platform_teacher_application_exports(
    store: &mut impl PlatformCsvExportStore,
) -> Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError> {
    store.load_platform_teacher_application_exports().await
}

pub async fn load_platform_reward_approval_exports(
    store: &mut impl PlatformCsvExportStore,
) -> Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError> {
    store.load_platform_reward_approval_exports().await
}

pub async fn load_platform_token_payout_exports(
    store: &mut impl PlatformCsvExportStore,
) -> Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError> {
    store.load_platform_token_payout_exports().await
}

pub async fn load_platform_wallet_credit_exports(
    store: &mut impl PlatformCsvExportStore,
) -> Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError> {
    store.load_platform_wallet_credit_exports().await
}

pub async fn load_platform_delegated_permission_exports(
    store: &mut impl PlatformCsvExportStore,
) -> Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError> {
    store.load_platform_delegated_permission_exports().await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_platform_reward_approval_exports;
    use crate::application::reporting::platform_csv_exports::store::PlatformCsvExportStore;
    use crate::application::reporting::platform_csv_exports::*;
    use crate::domain::rewards::candidate::event_type::RewardEventType;
    use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::domain::teacher_applications::scope::TeacherApplicationScope;
    use crate::domain::teacher_applications::status::TeacherApplicationStatus;

    #[test]
    fn loads_platform_csv_export_rows_through_store_port() {
        let mut store = FakePlatformCsvExportStore { called: false };

        let rows = block_on(load_platform_reward_approval_exports(&mut store))
            .expect("platform reward approval export rows should load");

        assert!(store.called);
        assert_eq!(rows[0].reward_candidate_id, 42);
    }

    struct FakePlatformCsvExportStore {
        called: bool,
    }

    impl PlatformCsvExportStore for FakePlatformCsvExportStore {
        fn load_platform_teacher_application_exports(
            &mut self,
        ) -> BoxFuture<
            '_,
            Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError>,
        > {
            ready(Ok(vec![PlatformTeacherApplicationExportRowOutput {
                application_id: 7,
                applicant_user_id: 8,
                requested_scope: TeacherApplicationScope::Platform,
                requested_organization_id: None,
                requested_course_id: None,
                organization_sponsor_id: None,
                status: TeacherApplicationStatus::Submitted,
                reviewer_id: None,
                decision_reason: String::new(),
                portfolio_links: "[]".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                decided_at: None,
            }]))
            .boxed()
        }

        fn load_platform_reward_approval_exports(
            &mut self,
        ) -> BoxFuture<'_, Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError>>
        {
            self.called = true;
            ready(Ok(vec![PlatformRewardApprovalExportRowOutput {
                reward_candidate_id: 42,
                course_id: 1,
                student_user_id: 2,
                submitter_user_id: 3,
                source_scope: RewardCandidateSourceScope::Course,
                source_organization_id: None,
                event_type: RewardEventType::CourseCompletion,
                status: RewardCandidateStatus::AmountApproved,
                teacher_approver_user_id: Some(4),
                teacher_decision_reason: "approved".to_string(),
                teacher_decided_at: None,
                amount_reviewer_user_id: Some(5),
                approved_amount: "10".to_string(),
                amount_decision_reason: "ok".to_string(),
                amount_decided_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]))
            .boxed()
        }

        fn load_platform_token_payout_exports(
            &mut self,
        ) -> BoxFuture<'_, Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError>>
        {
            ready(Ok(Vec::new())).boxed()
        }

        fn load_platform_wallet_credit_exports(
            &mut self,
        ) -> BoxFuture<'_, Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError>>
        {
            ready(Ok(Vec::new())).boxed()
        }

        fn load_platform_delegated_permission_exports(
            &mut self,
        ) -> BoxFuture<
            '_,
            Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError>,
        > {
            ready(Ok(Vec::new())).boxed()
        }
    }
}
