use bigdecimal::BigDecimal;

use super::*;

fn bd(value: i64) -> BigDecimal {
    BigDecimal::from(value)
}

#[test]
fn valid_amounts_pass() {
    validate_amounts(&bd(100), &bd(2), Some(&bd(500)), 3_600).unwrap();
}

#[test]
fn zero_token_amount_is_valid() {
    validate_amounts(&bd(0), &bd(1), None, 0).unwrap();
}

#[test]
fn negative_token_amount_fails() {
    assert!(matches!(
        validate_amounts(&bd(-1), &bd(1), None, 0),
        Err(RewardPolicyError::InvalidInput(_))
    ));
}

#[test]
fn zero_multiplier_fails() {
    assert!(validate_amounts(&bd(100), &bd(0), None, 0).is_err());
}

#[test]
fn negative_multiplier_fails() {
    assert!(validate_amounts(&bd(100), &bd(-1), None, 0).is_err());
}

#[test]
fn negative_max_payout_fails() {
    assert!(validate_amounts(&bd(100), &bd(1), Some(&bd(-1)), 0).is_err());
}

#[test]
fn none_max_payout_is_valid() {
    validate_amounts(&bd(100), &bd(1), None, 0).unwrap();
}

#[test]
fn negative_cooldown_fails() {
    assert!(validate_amounts(&bd(100), &bd(1), None, -1).is_err());
}
