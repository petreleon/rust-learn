use super::DelegationRuleError;

pub fn normalize_permission(permission: &str) -> Result<String, DelegationRuleError> {
    let permission = permission.trim();
    if delegatable_reward_permissions().contains(&permission) {
        Ok(permission.to_string())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission is not delegatable for reward work".to_string(),
        ))
    }
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
    if matches!(
        permission,
        "APPROVE_REWARD_AMOUNT"
            | "EXECUTE_REWARD_PAYOUT"
            | "VIEW_REWARD_AUDIT"
            | "MANAGE_REWARD_FRAUD_BLOCKS"
            | "BLOCK_REWARD_TEACHER"
            | "BLOCK_REWARD_ORGANIZATION"
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
    if matches!(
        permission,
        "SUBMIT_ORG_COURSE_REWARD_EVENT" | "VIEW_ORG_REWARD_REPORTS" | "MANAGE_ORG_REWARD_BUDGET"
    ) {
        Ok(())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission cannot be delegated at organization scope".to_string(),
        ))
    }
}

pub(super) fn ensure_course_permission_scope(permission: &str) -> Result<(), DelegationRuleError> {
    if matches!(
        permission,
        "SUBMIT_COURSE_REWARD_EVENT"
            | "CREATE_REWARDABLE_COURSE_EVENT"
            | "APPROVE_STUDENT_REWARD_CANDIDATE"
            | "VIEW_COURSE_REWARD_STATUS"
            | "GRADE_REWARDABLE_ASSESSMENT"
            | "MANAGE_COURSE_REWARD_RULES"
    ) {
        Ok(())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "permission cannot be delegated at course scope".to_string(),
        ))
    }
}

fn normalize_permission_name(permission: &str) -> Result<String, DelegationRuleError> {
    let permission = permission.trim();
    if all_supported_delegation_permissions().contains(&permission) {
        Ok(permission.to_string())
    } else {
        Err(DelegationRuleError::InvalidInput(
            "unsupported delegated permission".to_string(),
        ))
    }
}

fn delegatable_reward_permissions() -> &'static [&'static str] {
    &[
        "APPROVE_REWARD_AMOUNT",
        "EXECUTE_REWARD_PAYOUT",
        "VIEW_REWARD_AUDIT",
        "MANAGE_REWARD_FRAUD_BLOCKS",
        "BLOCK_REWARD_TEACHER",
        "BLOCK_REWARD_ORGANIZATION",
        "SUBMIT_ORG_COURSE_REWARD_EVENT",
        "VIEW_ORG_REWARD_REPORTS",
        "MANAGE_ORG_REWARD_BUDGET",
        "SUBMIT_COURSE_REWARD_EVENT",
        "CREATE_REWARDABLE_COURSE_EVENT",
        "APPROVE_STUDENT_REWARD_CANDIDATE",
        "VIEW_COURSE_REWARD_STATUS",
        "GRADE_REWARDABLE_ASSESSMENT",
        "MANAGE_COURSE_REWARD_RULES",
    ]
}

fn all_supported_delegation_permissions() -> &'static [&'static str] {
    delegatable_reward_permissions()
}
