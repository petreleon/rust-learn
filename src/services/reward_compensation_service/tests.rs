use super::*;
use bigdecimal::BigDecimal;

fn compensation_request(
    amount: i64,
    reason: &str,
    idempotency_key: &str,
) -> RewardCompensationRequest {
    RewardCompensationRequest {
        reward_candidate_id: 1,
        amount: BigDecimal::from(amount),
        reason: reason.into(),
        idempotency_key: idempotency_key.into(),
    }
}

#[test]
fn valid_request_passes() {
    validate_compensation_request(&compensation_request(100, "Adjustment", "key-1")).unwrap();
}

#[test]
fn negative_amount_is_valid() {
    validate_compensation_request(&compensation_request(-50, "Negative comp", "key-2")).unwrap();
}

#[test]
fn zero_amount_fails() {
    assert!(validate_compensation_request(&compensation_request(0, "reason", "key")).is_err());
}

#[test]
fn empty_reason_fails() {
    assert!(validate_compensation_request(&compensation_request(100, "   ", "key")).is_err());
}

#[test]
fn empty_idempotency_key_fails() {
    assert!(validate_compensation_request(&compensation_request(100, "reason", "")).is_err());
}
