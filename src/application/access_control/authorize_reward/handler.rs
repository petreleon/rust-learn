use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn authorize_reward_action(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    let actor = AccessActor::user(actor_user_id);
    match action {
        RewardAuthorizationAction::ApproveRewardAmount => {
            can_scope(
                store,
                actor,
                AccessScope::platform(),
                Permissions::APPROVE_REWARD_AMOUNT,
            )
            .await
        }
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id } => {
            can_scope(
                store,
                actor,
                AccessScope::course(course_id),
                Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
            )
            .await
        }
        RewardAuthorizationAction::ExecuteRewardPayout => {
            can_scope(
                store,
                actor,
                AccessScope::platform(),
                Permissions::EXECUTE_REWARD_PAYOUT,
            )
            .await
        }
        RewardAuthorizationAction::ManageCourseRewardRules { course_id } => {
            can_scope(
                store,
                actor,
                AccessScope::course(course_id),
                Permissions::MANAGE_COURSE_REWARD_RULES,
            )
            .await
        }
        RewardAuthorizationAction::ManageOrganizationRewardFraudBlock => {
            has_any_permission(
                store,
                actor,
                AccessScope::platform(),
                &[
                    Permissions::BLOCK_REWARD_ORGANIZATION,
                    Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ManageRewardFraudBlock => {
            can_scope(
                store,
                actor,
                AccessScope::platform(),
                Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
            )
            .await
        }
        RewardAuthorizationAction::ManageRewardPolicy => {
            can_scope(
                store,
                actor,
                AccessScope::platform(),
                Permissions::SET_REWARD_POLICY,
            )
            .await
        }
        RewardAuthorizationAction::ManageTeacherRewardFraudBlock => {
            has_any_permission(
                store,
                actor,
                AccessScope::platform(),
                &[
                    Permissions::BLOCK_REWARD_TEACHER,
                    Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
                ],
            )
            .await
        }
        RewardAuthorizationAction::RecordRewardCompensation => {
            has_any_permission(
                store,
                actor,
                AccessScope::platform(),
                &[Permissions::RECONCILE_WALLETS, Permissions::MANAGE_WALLETS],
            )
            .await
        }
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id } => {
            has_any_permission(
                store,
                actor,
                AccessScope::course(course_id),
                &[
                    Permissions::SUBMIT_COURSE_REWARD_EVENT,
                    Permissions::CREATE_REWARDABLE_COURSE_EVENT,
                ],
            )
            .await
        }
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id } => {
            can_scope(
                store,
                actor,
                AccessScope::organization(organization_id),
                Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
            )
            .await
        }
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id } => {
            can_scope(
                store,
                actor,
                AccessScope::course(course_id),
                Permissions::VIEW_COURSE_REWARD_STATUS,
            )
            .await
        }
        RewardAuthorizationAction::ViewRewardFraudBlocks => {
            has_any_permission(
                store,
                actor,
                AccessScope::platform(),
                &[
                    Permissions::VIEW_REWARD_AUDIT,
                    Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
                ],
            )
            .await
        }
        RewardAuthorizationAction::ViewRewardAudit => {
            can_scope(
                store,
                actor,
                AccessScope::platform(),
                Permissions::VIEW_REWARD_AUDIT,
            )
            .await
        }
    }
}

async fn can_scope(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    scope: AccessScope,
    permission: Permissions,
) -> Result<bool, RewardAuthorizationError> {
    store
        .can(actor, AccessAction::permission(permission), scope)
        .await
}

async fn has_any_permission(
    store: &mut impl RewardAuthorizationStore,
    actor: AccessActor,
    scope: AccessScope,
    permissions: &[Permissions],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if can_scope(store, actor, scope, permission).await? {
            return Ok(true);
        }
    }
    Ok(false)
}
