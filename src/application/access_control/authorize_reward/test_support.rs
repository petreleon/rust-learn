use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permission::Permission;

#[derive(Default)]
pub(crate) struct FakeRewardAuthorizationStore {
    pub platform_permissions: Vec<Permission>,
    pub course_permissions: Vec<(i32, Permission)>,
    pub organization_permissions: Vec<(i32, Permission)>,
    pub platform_checks: Vec<Permission>,
    pub course_checks: Vec<(i32, Permission)>,
    pub organization_checks: Vec<(i32, Permission)>,
}

impl RewardAuthorizationStore for FakeRewardAuthorizationStore {
    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        let permission = permission_from_action(&action);
        let allowed = match scope {
            AccessScope::Platform(_) => {
                self.platform_checks.push(permission);
                self.platform_permissions.contains(&permission)
            }
            AccessScope::Course(scope) => {
                self.course_checks.push((scope.course_id(), permission));
                self.course_permissions
                    .contains(&(scope.course_id(), permission))
            }
            AccessScope::Organization(scope) => {
                self.organization_checks
                    .push((scope.organization_id(), permission));
                self.organization_permissions
                    .contains(&(scope.organization_id(), permission))
            }
        };

        ready(Ok(allowed)).boxed()
    }
}

fn permission_from_action(action: &AccessAction) -> Permission {
    match action.permission_name() {
        "APPROVE_REWARD_AMOUNT" => Permission::ApproveRewardAmount,
        "APPROVE_STUDENT_REWARD_CANDIDATE" => Permission::ApproveStudentRewardCandidate,
        "BLOCK_REWARD_ORGANIZATION" => Permission::BlockRewardOrganization,
        "BLOCK_REWARD_TEACHER" => Permission::BlockRewardTeacher,
        "CREATE_REWARDABLE_COURSE_EVENT" => Permission::CreateRewardableCourseEvent,
        "EXECUTE_REWARD_PAYOUT" => Permission::ExecuteRewardPayout,
        "MANAGE_COURSE_REWARD_RULES" => Permission::ManageCourseRewardRules,
        "MANAGE_REWARD_FRAUD_BLOCKS" => Permission::ManageRewardFraudBlocks,
        "MANAGE_WALLETS" => Permission::ManageWallets,
        "RECONCILE_WALLETS" => Permission::ReconcileWallets,
        "SET_REWARD_POLICY" => Permission::SetRewardPolicy,
        "SUBMIT_COURSE_REWARD_EVENT" => Permission::SubmitCourseRewardEvent,
        "SUBMIT_ORG_COURSE_REWARD_EVENT" => Permission::SubmitOrgCourseRewardEvent,
        "VIEW_COURSE_REWARD_STATUS" => Permission::ViewCourseRewardStatus,
        "VIEW_REWARD_AUDIT" => Permission::ViewRewardAudit,
        permission => panic!("unsupported fake reward permission: {permission}"),
    }
}
