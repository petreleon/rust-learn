pub const REWARD_TOKEN_EVENT_MINT: &str = "mint";
pub const REWARD_TOKEN_EVENT_TRANSFER: &str = "transfer";
pub const REWARD_TOKEN_EVENT_IMPORT: &str = "import";

pub const REWARD_TOKEN_TRANSACTION_TYPE_MINT: &str = "token_mint";
pub const REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER: &str = "token_transfer";
pub const REWARD_TOKEN_TRANSACTION_TYPE_IMPORT: &str = "token_import";

pub fn transaction_type_for_token_event(event_type: &str) -> Option<&'static str> {
    match event_type {
        REWARD_TOKEN_EVENT_MINT => Some(REWARD_TOKEN_TRANSACTION_TYPE_MINT),
        REWARD_TOKEN_EVENT_TRANSFER => Some(REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER),
        REWARD_TOKEN_EVENT_IMPORT => Some(REWARD_TOKEN_TRANSACTION_TYPE_IMPORT),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        transaction_type_for_token_event, REWARD_TOKEN_TRANSACTION_TYPE_IMPORT,
        REWARD_TOKEN_TRANSACTION_TYPE_MINT, REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER,
    };

    #[test]
    fn maps_token_events_to_transaction_types() {
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
    }
}
