use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

pub async fn authorize_reward_action(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    match action {
        RewardAuthorizationAction::ApproveRewardAmount => {
            store
                .has_platform_permission(actor_user_id, Permission::ApproveRewardAmount)
                .await
        }
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id } => {
            store
                .has_course_permission(
                    actor_user_id,
                    course_id,
                    Permission::ApproveStudentRewardCandidate,
                )
                .await
        }
        RewardAuthorizationAction::ExecuteRewardPayout => {
            store
                .has_platform_permission(actor_user_id, Permission::ExecuteRewardPayout)
                .await
        }
        RewardAuthorizationAction::ManageCourseRewardRules { course_id } => {
            store
                .has_course_permission(
                    actor_user_id,
                    course_id,
                    Permission::ManageCourseRewardRules,
                )
                .await
        }
        RewardAuthorizationAction::ManageOrganizationRewardFraudBlock => {
            has_any_platform_permission(
                store,
                actor_user_id,
                &[
                    Permission::BlockRewardOrganization,
                    Permission::ManageRewardFraudBlocks,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ManageRewardFraudBlock => {
            store
                .has_platform_permission(actor_user_id, Permission::ManageRewardFraudBlocks)
                .await
        }
        RewardAuthorizationAction::ManageRewardPolicy => {
            store
                .has_platform_permission(actor_user_id, Permission::SetRewardPolicy)
                .await
        }
        RewardAuthorizationAction::ManageTeacherRewardFraudBlock => {
            has_any_platform_permission(
                store,
                actor_user_id,
                &[
                    Permission::BlockRewardTeacher,
                    Permission::ManageRewardFraudBlocks,
                ],
            )
            .await
        }
        RewardAuthorizationAction::RecordRewardCompensation => {
            has_any_platform_permission(
                store,
                actor_user_id,
                &[Permission::ReconcileWallets, Permission::ManageWallets],
            )
            .await
        }
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id } => {
            has_any_course_permission(
                store,
                actor_user_id,
                course_id,
                &[
                    Permission::SubmitCourseRewardEvent,
                    Permission::CreateRewardableCourseEvent,
                ],
            )
            .await
        }
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id } => {
            store
                .has_organization_permission(
                    actor_user_id,
                    organization_id,
                    Permission::SubmitOrgCourseRewardEvent,
                )
                .await
        }
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id } => {
            store
                .has_course_permission(actor_user_id, course_id, Permission::ViewCourseRewardStatus)
                .await
        }
        RewardAuthorizationAction::ViewRewardFraudBlocks => {
            has_any_platform_permission(
                store,
                actor_user_id,
                &[
                    Permission::ViewRewardAudit,
                    Permission::ManageRewardFraudBlocks,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ViewRewardAudit => {
            store
                .has_platform_permission(actor_user_id, Permission::ViewRewardAudit)
                .await
        }
    }
}

async fn has_any_platform_permission(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    permissions: &[Permission],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if store
            .has_platform_permission(actor_user_id, permission)
            .await?
        {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn has_any_course_permission(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    course_id: i32,
    permissions: &[Permission],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if store
            .has_course_permission(actor_user_id, course_id, permission)
            .await?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
