fn normalize_event_type(event_type: &str) -> Result<String, RewardPolicyError> {
    let normalized = event_type
        .trim()
        .to_ascii_lowercase()
        .replace(['-', ' '], "_");
    match normalized.as_str() {
        REWARD_EVENT_ASSESSMENT_COMPLETION
        | REWARD_EVENT_COURSE_COMPLETION
        | REWARD_EVENT_MANUAL_COMPLETION
        | REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Ok(normalized),
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy event type".to_string(),
        )),
    }
}

fn normalize_payment_strategy(payment_strategy: &str) -> Result<String, RewardPolicyError> {
    let normalized = payment_strategy.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER | REWARD_PAYMENT_MINT | REWARD_PAYMENT_OFF_CHAIN => {
            Ok(normalized)
        }
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy payment strategy".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
