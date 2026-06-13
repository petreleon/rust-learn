pub const REWARD_TRANSACTION_TYPE_WALLET_CREDIT: &str = "reward_wallet_credit";
pub const REWARD_WALLET_CREDIT_NOTIFICATION_TITLE: &str = "reward:wallet_credited";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardWalletCreditNotificationMessage {
    pub title: &'static str,
    pub body: String,
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
        reward_wallet_credit_notification_message, REWARD_WALLET_CREDIT_NOTIFICATION_TITLE,
    };

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
