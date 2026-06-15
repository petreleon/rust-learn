use super::DelegationRuleError;
use crate::domain::access_control::permissions::Permissions;

pub fn normalize_permission(permission: &str) -> Result<String, DelegationRuleError> {
    parse_delegatable_reward_permission(permission).map(String::from)
}

pub fn normalize_filter_permission(
    permission: Option<String>,
) -> Result<Option<String>, DelegationRuleError> {
    permission
        .as_deref()
        .map(normalize_permission_name)
        .transpose()
}

pub(super) fn ensure_platform_permission_scope(
    permission: &str,
) -> Result<(), DelegationRuleError> {
    let permission = parse_delegatable_reward_permission(permission)?;
    if matches!(
        permission,
        Permissions::APPROVE_REWARD_AMOUNT
            | Permissions::EXECUTE_REWARD_PAYOUT
            | Permissions::VIEW_REWARD_AUDIT
            | Permissions::MANAGE_REWARD_FRAUD_BLOCKS
            | Permissions::BLOCK_REWARD_TEACHER
            | Permissions::BLOCK_REWARD_ORGANIZATION
    ) {
        Ok(())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission cannot be delegated at platform scope".to_string(),
        ))
    }
}

pub(super) fn ensure_organization_permission_scope(
    permission: &str,
) -> Result<(), DelegationRuleError> {
    let permission = parse_delegatable_reward_permission(permission)?;
    if matches!(
        permission,
        Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT
            | Permissions::VIEW_ORG_REWARD_REPORTS
            | Permissions::MANAGE_ORG_REWARD_BUDGET
    ) {
        Ok(())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission cannot be delegated at organization scope".to_string(),
        ))
    }
}

pub(super) fn ensure_course_permission_scope(permission: &str) -> Result<(), DelegationRuleError> {
    let permission = parse_delegatable_reward_permission(permission)?;
    if matches!(
        permission,
        Permissions::SUBMIT_COURSE_REWARD_EVENT
            | Permissions::CREATE_REWARDABLE_COURSE_EVENT
            | Permissions::APPROVE_STUDENT_REWARD_CANDIDATE
            | Permissions::VIEW_COURSE_REWARD_STATUS
            | Permissions::GRADE_REWARDABLE_ASSESSMENT
            | Permissions::MANAGE_COURSE_REWARD_RULES
    ) {
        Ok(())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission cannot be delegated at course scope".to_string(),
        ))
    }
}

fn normalize_permission_name(permission: &str) -> Result<String, DelegationRuleError> {
    let permission = parse_delegation_permission(permission)?;
    if delegatable_reward_permissions().contains(&permission) {
        Ok(permission.into())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "unsupported delegated permission".to_string(),
        ))
    }
}

fn parse_delegatable_reward_permission(
    permission: &str,
) -> Result<Permissions, DelegationRuleError> {
    let permission = parse_delegation_permission(permission)?;
    if delegatable_reward_permissions().contains(&permission) {
        Ok(permission)
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission is not delegatable for reward work".to_string(),
        ))
    }
}

fn parse_delegation_permission(permission: &str) -> Result<Permissions, DelegationRuleError> {
    permission.trim().parse().map_err(|_| {
        DelegationRuleError::InvalidInput("unsupported delegated permission".to_string())
    })
}

fn delegatable_reward_permissions() -> &'static [Permissions] {
    &[
        Permissions::APPROVE_REWARD_AMOUNT,
        Permissions::EXECUTE_REWARD_PAYOUT,
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        Permissions::BLOCK_REWARD_TEACHER,
        Permissions::BLOCK_REWARD_ORGANIZATION,
        Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
        Permissions::VIEW_ORG_REWARD_REPORTS,
        Permissions::MANAGE_ORG_REWARD_BUDGET,
        Permissions::SUBMIT_COURSE_REWARD_EVENT,
        Permissions::CREATE_REWARDABLE_COURSE_EVENT,
        Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        Permissions::VIEW_COURSE_REWARD_STATUS,
        Permissions::GRADE_REWARDABLE_ASSESSMENT,
        Permissions::MANAGE_COURSE_REWARD_RULES,
    ]
}
