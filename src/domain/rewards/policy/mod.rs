pub const REWARD_POLICY_SCOPE_PLATFORM: &str = "platform";
pub const REWARD_POLICY_SCOPE_ORGANIZATION: &str = "organization";
pub const REWARD_POLICY_SCOPE_COURSE: &str = "course";

pub const REWARD_PAYMENT_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYMENT_MINT: &str = "mint";
pub const REWARD_PAYMENT_OFF_CHAIN: &str = "off_chain";

pub const REWARD_EVENT_ASSESSMENT_COMPLETION: &str = "assessment_completion";
pub const REWARD_EVENT_COURSE_COMPLETION: &str = "course_completion";
pub const REWARD_EVENT_MANUAL_COMPLETION: &str = "manual_completion";
pub const REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT: &str = "administrative_adjustment";

pub fn normalize_scope_type(scope_type: &str) -> Option<String> {
    let normalized = scope_type.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_POLICY_SCOPE_PLATFORM
        | REWARD_POLICY_SCOPE_ORGANIZATION
        | REWARD_POLICY_SCOPE_COURSE => Some(normalized),
        _ => None,
    }
}

pub fn normalize_event_type(event_type: &str) -> Option<String> {
    let normalized = event_type
        .trim()
        .to_ascii_lowercase()
        .replace(['-', ' '], "_");
    match normalized.as_str() {
        REWARD_EVENT_ASSESSMENT_COMPLETION
        | REWARD_EVENT_COURSE_COMPLETION
        | REWARD_EVENT_MANUAL_COMPLETION
        | REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Some(normalized),
        _ => None,
    }
}

pub fn normalize_payment_strategy(payment_strategy: &str) -> Option<String> {
    let normalized = payment_strategy.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER | REWARD_PAYMENT_MINT | REWARD_PAYMENT_OFF_CHAIN => {
            Some(normalized)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
