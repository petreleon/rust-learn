use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permission::Permission;

pub async fn authorize_reward_action(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    let actor = AccessActor::user(actor_user_id);
    match action {
        RewardAuthorizationAction::ApproveRewardAmount => {
            can_platform(store, actor, Permission::ApproveRewardAmount).await
        }
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id } => {
            can_course(
                store,
                actor,
                course_id,
                Permission::ApproveStudentRewardCandidate,
            )
            .await
        }
        RewardAuthorizationAction::ExecuteRewardPayout => {
            can_platform(store, actor, Permission::ExecuteRewardPayout).await
        }
        RewardAuthorizationAction::ManageCourseRewardRules { course_id } => {
            can_course(store, actor, course_id, Permission::ManageCourseRewardRules).await
        }
        RewardAuthorizationAction::ManageOrganizationRewardFraudBlock => {
            has_any_platform_permission(
                store,
                actor,
                &[
                    Permission::BlockRewardOrganization,
                    Permission::ManageRewardFraudBlocks,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ManageRewardFraudBlock => {
            can_platform(store, actor, Permission::ManageRewardFraudBlocks).await
        }
        RewardAuthorizationAction::ManageRewardPolicy => {
            can_platform(store, actor, Permission::SetRewardPolicy).await
        }
        RewardAuthorizationAction::ManageTeacherRewardFraudBlock => {
            has_any_platform_permission(
                store,
                actor,
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
                actor,
                &[Permission::ReconcileWallets, Permission::ManageWallets],
            )
            .await
        }
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id } => {
            has_any_course_permission(
                store,
                actor,
                course_id,
                &[
                    Permission::SubmitCourseRewardEvent,
                    Permission::CreateRewardableCourseEvent,
                ],
            )
            .await
        }
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id } => {
            can_organization(
                store,
                actor,
                organization_id,
                Permission::SubmitOrgCourseRewardEvent,
            )
            .await
        }
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id } => {
            can_course(store, actor, course_id, Permission::ViewCourseRewardStatus).await
        }
        RewardAuthorizationAction::ViewRewardFraudBlocks => {
            has_any_platform_permission(
                store,
                actor,
                &[
                    Permission::ViewRewardAudit,
                    Permission::ManageRewardFraudBlocks,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ViewRewardAudit => {
            can_platform(store, actor, Permission::ViewRewardAudit).await
        }
    }
}

async fn can_platform(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    permission: Permission,
) -> Result<bool, RewardAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission.as_str()),
            AccessScope::Platform,
        )
        .await
}

async fn can_course(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    course_id: i32,
    permission: Permission,
) -> Result<bool, RewardAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission.as_str()),
            AccessScope::Course { course_id },
        )
        .await
}

async fn can_organization(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    organization_id: i32,
    permission: Permission,
) -> Result<bool, RewardAuthorizationError> {
    store
        .can(
            actor,
            AccessAction::permission(permission.as_str()),
            AccessScope::Organization { organization_id },
        )
        .await
}

async fn has_any_platform_permission(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    permissions: &[Permission],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if can_platform(store, actor, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn has_any_course_permission(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    course_id: i32,
    permissions: &[Permission],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if can_course(store, actor, course_id, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}
