use diesel_async::AsyncPgConnection;

use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockError, RewardFraudBlockOutput,
};
use crate::domain::rewards::fraud_block::RewardFraudBlockAuditEventType;
use crate::infra::notifications::create_notifications_bulk;
use crate::infra::postgres::models::notification::NewNotification;
use crate::infra::postgres::rewards::reward_fraud_block_notification_recipients::reward_fraud_block_notification_recipients;

pub(super) async fn notify_reward_fraud_block_transition(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlockOutput,
    event_type: RewardFraudBlockAuditEventType,
) -> Result<(), RewardFraudBlockError> {
    let recipients = reward_fraud_block_notification_recipients(conn, block).await?;
    if recipients.is_empty() {
        return Ok(());
    }

    let event_type = event_type.as_str();
    let scope_type = block.scope_type.as_str();
    let title = format!("reward_fraud_block:{event_type}");
    let body = format!(
        "Reward fraud block #{} was {} for {} scope. Reason: {}",
        block.id, event_type, scope_type, block.reason
    );
    let notifications = recipients
        .iter()
        .map(|user_id| NewNotification {
            user_id: Some(*user_id),
            title: title.as_str(),
            body: body.as_str(),
        })
        .collect::<Vec<_>>();

    create_notifications_bulk(conn, notifications.as_slice())
        .await
        .map_err(|error| RewardFraudBlockError::Database(error.to_string()))?;

    Ok(())
}
