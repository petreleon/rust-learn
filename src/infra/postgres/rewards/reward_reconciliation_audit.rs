use diesel_async::AsyncPgConnection;

use crate::application::rewards::reconcile_candidate::RewardReconciliationError;
use crate::domain::rewards::audit::RewardAuditEventType;
use crate::infra::postgres::models::reward_audit_event::NewRewardAuditEvent;
use crate::infra::postgres::rewards::reward_audit_records::create_reward_audit_event;
use crate::infra::postgres::rewards::reward_reconciliation_mappers::map_diesel_error;

pub(super) async fn create_reconciliation_audit_event(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
    actor_user_id: Option<i32>,
    initial_status: String,
    final_status: String,
    outcome: &ReconciliationOutcome,
) -> Result<(), RewardReconciliationError> {
    if !outcome.changed() {
        return Ok(());
    }

    create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id,
            actor_user_id,
            event_type: RewardAuditEventType::Reconciled.as_str().to_string(),
            from_status: Some(initial_status),
            to_status: final_status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_credit_created": outcome.wallet_credit_created,
                "notification_created": outcome.notification_created,
                "external_transaction_link_repaired": outcome.external_transaction_link_repaired,
                "internal_transaction_link_repaired": outcome.internal_transaction_link_repaired,
            }),
        },
    )
    .await
    .map_err(map_diesel_error)?;
    Ok(())
}

pub(super) struct ReconciliationOutcome {
    pub wallet_credit_created: bool,
    pub notification_created: bool,
    pub external_transaction_link_repaired: bool,
    pub internal_transaction_link_repaired: bool,
}

impl ReconciliationOutcome {
    fn changed(&self) -> bool {
        self.wallet_credit_created
            || self.notification_created
            || self.external_transaction_link_repaired
            || self.internal_transaction_link_repaired
    }
}
