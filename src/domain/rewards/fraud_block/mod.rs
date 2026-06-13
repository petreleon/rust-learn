pub const REWARD_FRAUD_BLOCK_SCOPE_TEACHER: &str = "teacher";
pub const REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION: &str = "organization";
pub const REWARD_FRAUD_BLOCK_SCOPE_COURSE: &str = "course";
pub const REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY: &str = "reward_policy";

pub fn normalize_scope_type(scope_type: &str) -> Option<String> {
    match scope_type.trim() {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Some(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => {
            Some(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string())
        }
        REWARD_FRAUD_BLOCK_SCOPE_COURSE => Some(REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Some(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY.to_string())
        }
        _ => None,
    }
}

pub fn scope_matches_target(
    scope_type: &str,
    has_teacher: bool,
    has_organization: bool,
    has_course: bool,
    has_reward_policy: bool,
) -> bool {
    match scope_type {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => has_teacher,
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => has_organization,
        REWARD_FRAUD_BLOCK_SCOPE_COURSE => has_course,
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => has_reward_policy,
        _ => false,
    }
}

#[cfg(test)]
mod tests;
