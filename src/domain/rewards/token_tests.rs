use super::token::{
    transaction_type_for_token_event, RewardTokenEventType, RewardTokenTransactionType,
    REWARD_TOKEN_EVENT_TRANSFER, REWARD_TOKEN_TRANSACTION_TYPE_IMPORT,
    REWARD_TOKEN_TRANSACTION_TYPE_MINT, REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER,
};

#[test]
fn exposes_stable_token_event_keys() {
    assert_eq!(
        RewardTokenEventType::Transfer.as_str(),
        REWARD_TOKEN_EVENT_TRANSFER
    );
    assert_eq!(RewardTokenEventType::Mint.as_str(), "mint");
}

#[test]
fn exposes_stable_token_transaction_keys() {
    assert_eq!(
        RewardTokenTransactionType::Mint.as_str(),
        REWARD_TOKEN_TRANSACTION_TYPE_MINT
    );
    assert_eq!(
        RewardTokenTransactionType::Import.as_str(),
        REWARD_TOKEN_TRANSACTION_TYPE_IMPORT
    );
}

#[test]
fn parses_known_token_event() {
    assert_eq!(
        RewardTokenEventType::parse("transfer").unwrap(),
        RewardTokenEventType::Transfer
    );
    assert_eq!(
        RewardTokenEventType::normalize(" Transfer ").unwrap(),
        RewardTokenEventType::Transfer
    );
    assert_eq!(
        RewardTokenEventType::parse_optional(Some("Import".to_string())).unwrap(),
        Some(RewardTokenEventType::Import)
    );
}

#[test]
fn maps_token_events_to_transaction_types() {
    assert_eq!(
        RewardTokenEventType::Mint.transaction_type(),
        RewardTokenTransactionType::Mint
    );
    assert_eq!(
        RewardTokenEventType::Transfer.transaction_type(),
        RewardTokenTransactionType::Transfer
    );
    assert_eq!(
        transaction_type_for_token_event("mint").unwrap(),
        REWARD_TOKEN_TRANSACTION_TYPE_MINT
    );
    assert_eq!(
        transaction_type_for_token_event("transfer").unwrap(),
        REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER
    );
    assert_eq!(
        transaction_type_for_token_event("import").unwrap(),
        REWARD_TOKEN_TRANSACTION_TYPE_IMPORT
    );
}

#[test]
fn rejects_unknown_token_events() {
    assert!(transaction_type_for_token_event("").is_none());
    assert!(transaction_type_for_token_event("swap").is_none());
    assert!(transaction_type_for_token_event("burn").is_none());
    assert!(RewardTokenEventType::parse("burn").is_err());
}
