use std::fmt;

pub const REWARD_TRANSACTION_TYPE_WALLET_CREDIT: &str = "reward_wallet_credit";
pub const REWARD_WALLET_CREDIT_NOTIFICATION_TITLE: &str = "reward:wallet_credited";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardWalletCreditTransactionType {
    WalletCredit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletCreditTransactionTypeParseError {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardWalletCreditNotificationMessage {
    pub title: &'static str,
    pub body: String,
}

impl RewardWalletCreditTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletCredit => REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
        }
    }

    pub fn parse(value: &str) -> Result<Self, WalletCreditTransactionTypeParseError> {
        match value {
            REWARD_TRANSACTION_TYPE_WALLET_CREDIT => Ok(Self::WalletCredit),
            other => Err(WalletCreditTransactionTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardWalletCreditTransactionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for WalletCreditTransactionTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward wallet-credit transaction type '{}'",
            self.value
        )
    }
}

pub fn reward_wallet_credit_notification_message(
    course_id: i32,
    course_title: impl AsRef<str>,
    amount: impl AsRef<str>,
    wallet_id: i32,
    transaction_id: i64,
) -> RewardWalletCreditNotificationMessage {
    RewardWalletCreditNotificationMessage {
        title: REWARD_WALLET_CREDIT_NOTIFICATION_TITLE,
        body: format!(
            "Reward for course #{} ({}) was credited: {} LearnToken to wallet #{}. Transaction #{} was recorded.",
            course_id,
            compact_text(course_title, 120),
            compact_text(amount, 80),
            wallet_id,
            transaction_id
        ),
    }
}

fn compact_text(input: impl AsRef<str>, max_chars: usize) -> String {
    let input = input.as_ref().trim();
    if input.chars().count() <= max_chars {
        return input.to_string();
    }

    let mut compacted = input
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    compacted.push_str("...");
    compacted
}

#[cfg(test)]
mod tests {
    use super::{
        reward_wallet_credit_notification_message, RewardWalletCreditTransactionType,
        REWARD_TRANSACTION_TYPE_WALLET_CREDIT, REWARD_WALLET_CREDIT_NOTIFICATION_TITLE,
    };

    #[test]
    fn exposes_stable_wallet_credit_transaction_key() {
        assert_eq!(
            RewardWalletCreditTransactionType::WalletCredit.as_str(),
            REWARD_TRANSACTION_TYPE_WALLET_CREDIT
        );
    }

    #[test]
    fn parses_known_wallet_credit_transaction_type() {
        assert_eq!(
            RewardWalletCreditTransactionType::parse("reward_wallet_credit").unwrap(),
            RewardWalletCreditTransactionType::WalletCredit
        );
    }

    #[test]
    fn rejects_unknown_wallet_credit_transaction_type() {
        assert!(RewardWalletCreditTransactionType::parse("token_transfer").is_err());
    }

    #[test]
    fn wallet_credit_notification_keeps_legacy_title_and_body_shape() {
        let message = reward_wallet_credit_notification_message(7, "Rust 101", "42", 3, 99);

        assert_eq!(message.title, REWARD_WALLET_CREDIT_NOTIFICATION_TITLE);
        assert_eq!(
            message.body,
            "Reward for course #7 (Rust 101) was credited: 42 LearnToken to wallet #3. Transaction #99 was recorded."
        );
    }
}
