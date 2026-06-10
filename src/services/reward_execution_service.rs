use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, external_transactions, internal_transactions,
    reward_candidates, reward_policies, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
use crate::models::notification::{NewNotification, Notification};
use crate::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_RECONCILED, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED,
    REWARD_AUDIT_EVENT_WALLET_CREDITED, REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED, REWARD_STATUS_TOKEN_CONFIRMED,
    REWARD_STATUS_TOKEN_PENDING, REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_payout_record::NewRewardPayoutRecord;
use crate::models::reward_policy::{
    RewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::models::reward_wallet_credit_record::NewRewardWalletCreditRecord;
use crate::models::transaction::{
    ExternalTransaction, NewExternalTransaction, NewInternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};
use crate::models::wallet::Wallet;
use crate::repositories::persistent_state_repository::get_persistent_state;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_audit_event_repository;
use crate::repositories::reward_candidate_repository;
use crate::repositories::reward_payout_record_repository;
use crate::repositories::reward_wallet_credit_record_repository;
use crate::services::wallet_service;
use crate::utils::notifications::{
    create_notification, reward_wallet_credit_notification,
};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub const REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER: &str = "presigner_transfer";
pub const REWARD_PAYOUT_METHOD_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYOUT_METHOD_MINT: &str = "mint";
pub const REWARD_PAYOUT_METHOD_OFF_CHAIN: &str = "off_chain";
pub const REWARD_TRANSACTION_TYPE_WALLET_CREDIT: &str = "reward_wallet_credit";

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: String,
    pub payout_method: String,
    pub requires_token_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditResult {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub credit_record_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub amount: BigDecimal,
    pub credited: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditNotificationResult {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub notification_id: Option<i64>,
    pub transaction_id: i64,
    pub amount: BigDecimal,
    pub notified: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardReconciliationResult {
    pub candidate_id: i64,
    pub wallet_credit_created: bool,
    pub notification_created: bool,
    pub external_transaction_link_repaired: bool,
    pub internal_transaction_link_repaired: bool,
    pub final_status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationRequest {
    pub chain_id: i64,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: i64,
    pub event_type: String,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationResult {
    pub candidate_id: i64,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub payout_record_id: i64,
    pub inserted_external_transaction: bool,
}

#[derive(Debug, PartialEq)]
pub enum RewardExecutionError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Database(String),
}

impl From<diesel::result::Error> for RewardExecutionError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardExecutionError::NoActivePolicy,
            other => RewardExecutionError::Database(other.to_string()),
        }
    }
}

pub async fn plan_reward_payout(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let candidate = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
    ensure_candidate_ready_for_payout(&candidate)?;

    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }

    let policy = resolve_active_reward_policy(conn, candidate.course_id, &candidate.event_type)
        .await?
        .ok_or(RewardExecutionError::NoActivePolicy)?;
    let payout_method = select_payout_method(conn, &policy).await?;
    let requires_token_confirmation = payout_method != REWARD_PAYOUT_METHOD_OFF_CHAIN;

    let plan = RewardPayoutPlan {
        candidate_id: candidate.id,
        policy_id: policy.id,
        amount,
        payment_strategy: policy.payment_strategy,
        payout_method,
        requires_token_confirmation,
    };

    log::info!(
        "event=reward_payout_planned candidate_id={} policy_id={} amount={} payment_strategy={} payout_method={} requires_token_confirmation={}",
        plan.candidate_id,
        plan.policy_id,
        plan.amount,
        plan.payment_strategy,
        plan.payout_method,
        plan.requires_token_confirmation
    );

    Ok(plan)
}

pub async fn plan_reward_payout_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    plan_reward_payout(conn, candidate_id).await
}

pub async fn credit_reward_wallet(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                credit_reward_wallet_for_candidate(conn, &candidate, false, None).await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.credited,
        result.credit_record_id,
        result.transaction_id,
        result.internal_transaction_id
    );

    Ok(result)
}

pub async fn credit_reward_wallet_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                credit_reward_wallet_for_candidate(conn, &candidate, false, Some(actor_user_id))
                    .await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit actor_user_id={} candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
        actor_user_id,
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.credited,
        result.credit_record_id,
        result.transaction_id,
        result.internal_transaction_id
    );

    Ok(result)
}

pub async fn notify_reward_wallet_credit(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                notify_reward_wallet_credit_for_candidate(conn, &candidate, false, None).await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit_notification candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.notified,
        result.notification_id,
        result.transaction_id
    );

    Ok(result)
}

pub async fn notify_reward_wallet_credit_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                notify_reward_wallet_credit_for_candidate(
                    conn,
                    &candidate,
                    false,
                    Some(actor_user_id),
                )
                .await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit_notification actor_user_id={} candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
        actor_user_id,
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.notified,
        result.notification_id,
        result.transaction_id
    );

    Ok(result)
}

pub async fn reconcile_reward_candidate(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    reconcile_reward_candidate_with_actor(conn, candidate_id, None).await
}

pub async fn reconcile_reward_candidate_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    reconcile_reward_candidate_with_actor(conn, candidate_id, Some(actor_user_id)).await
}

async fn reconcile_reward_candidate_with_actor(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    actor_user_id: Option<i32>,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
        Box::pin(async move {
            let mut candidate =
                reward_candidate_repository::find_candidate(conn, candidate_id).await?;
            ensure_candidate_reconcilable(&candidate)?;
            let initial_status = candidate.status.clone();

            let payout_record =
                reward_payout_record_repository::find_reward_payout_record_by_candidate(
                    conn,
                    candidate.id,
                )
                .await?;
            let external_transaction_link_repaired = match payout_record.as_ref() {
                Some(record) => {
                    ensure_external_transaction_link(
                        conn,
                        record.transaction_id,
                        record.external_transaction_id,
                    )
                    .await?
                }
                None => false,
            };

            let mut credit_record =
                reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
                    conn,
                    candidate.id,
                )
                .await?;
            let mut wallet_credit_created = false;
            if credit_record.is_none() && should_create_reconciliation_wallet_credit(&candidate) {
                wallet_credit_created = credit_reward_wallet_for_candidate(
                    conn,
                    &candidate,
                    payout_record.is_some(),
                    actor_user_id,
                )
                .await?
                .credited;
                candidate = reward_candidate_repository::find_candidate(conn, candidate.id).await?;
                credit_record =
                    reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
                        conn,
                        candidate.id,
                    )
                    .await?;
            }

            let internal_transaction_link_repaired = match credit_record.as_ref() {
                Some(record) => {
                    ensure_internal_transaction_link(
                        conn,
                        record.transaction_id,
                        record.internal_transaction_id,
                    )
                    .await?
                }
                None => false,
            };

            let mut notification_created = false;
            if credit_record.is_some() {
                let notification_result =
                    notify_reward_wallet_credit_for_candidate(
                        conn,
                        &candidate,
                        true,
                        actor_user_id,
                    )
                    .await?;
                notification_created = notification_result.notified;
                candidate = reward_candidate_repository::find_candidate(conn, candidate.id).await?;
            }

            if wallet_credit_created
                || notification_created
                || external_transaction_link_repaired
                || internal_transaction_link_repaired
            {
                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: candidate.id,
                        actor_user_id,
                        event_type: REWARD_AUDIT_EVENT_RECONCILED.to_string(),
                        from_status: Some(initial_status),
                        to_status: candidate.status.clone(),
                        reason: None,
                        metadata: serde_json::json!({
                            "wallet_credit_created": wallet_credit_created,
                            "notification_created": notification_created,
                            "external_transaction_link_repaired": external_transaction_link_repaired,
                            "internal_transaction_link_repaired": internal_transaction_link_repaired,
                        }),
                    },
                )
                .await?;
            }

            Ok(RewardReconciliationResult {
                candidate_id: candidate.id,
                wallet_credit_created,
                notification_created,
                external_transaction_link_repaired,
                internal_transaction_link_repaired,
                final_status: candidate.status,
            })
        })
    })
    .await?;

    log::info!(
        "event=reward_reconciled candidate_id={} wallet_credit_created={} notification_created={} external_transaction_link_repaired={} internal_transaction_link_repaired={} final_status={}",
        result.candidate_id,
        result.wallet_credit_created,
        result.notification_created,
        result.external_transaction_link_repaired,
        result.internal_transaction_link_repaired,
        result.final_status
    );

    Ok(result)
}

pub async fn record_reward_token_confirmation(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    record_reward_token_confirmation_with_actor(conn, candidate_id, request, None).await
}

pub async fn record_reward_token_confirmation_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    record_reward_token_confirmation_with_actor(conn, candidate_id, request, Some(actor_user_id))
        .await
}

async fn record_reward_token_confirmation_with_actor(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
    actor_user_id: Option<i32>,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    validate_token_confirmation_request(&request)?;

    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                if let Some(existing_record) =
                    reward_payout_record_repository::find_reward_payout_record_by_candidate(
                        conn,
                        candidate_id,
                    )
                    .await?
                {
                    return Ok(RewardTokenConfirmationResult {
                        candidate_id,
                        transaction_id: existing_record.transaction_id,
                        external_transaction_id: existing_record.external_transaction_id,
                        payout_record_id: existing_record.id,
                        inserted_external_transaction: false,
                    });
                }

                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                if candidate.status != REWARD_STATUS_TOKEN_PENDING {
                    return Err(RewardExecutionError::InvalidStatus(
                        "reward candidate must be token pending before token confirmation"
                            .to_string(),
                    ));
                }

                let (transaction_id, external_transaction_id, inserted_external_transaction) =
                    record_external_reward_transaction(conn, &request).await?;
                let payout_record = reward_payout_record_repository::create_reward_payout_record(
                    conn,
                    NewRewardPayoutRecord {
                        reward_candidate_id: candidate.id,
                        transaction_id,
                        external_transaction_id,
                    },
                )
                .await?;
                let updated = mark_candidate_token_confirmed(conn, candidate.id).await?;
                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: updated.id,
                        actor_user_id,
                        event_type: REWARD_AUDIT_EVENT_TOKEN_CONFIRMED.to_string(),
                        from_status: Some(candidate.status.clone()),
                        to_status: updated.status,
                        reason: None,
                        metadata: serde_json::json!({
                            "transaction_id": transaction_id,
                            "external_transaction_id": external_transaction_id,
                            "payout_record_id": payout_record.id,
                            "inserted_external_transaction": inserted_external_transaction,
                        }),
                    },
                )
                .await?;

                Ok(RewardTokenConfirmationResult {
                    candidate_id: candidate.id,
                    transaction_id,
                    external_transaction_id,
                    payout_record_id: payout_record.id,
                    inserted_external_transaction,
                })
            })
        })
        .await?;

    log::info!(
        "event=reward_token_confirmed candidate_id={} transaction_id={} external_transaction_id={} payout_record_id={} inserted_external_transaction={}",
        result.candidate_id,
        result.transaction_id,
        result.external_transaction_id,
        result.payout_record_id,
        result.inserted_external_transaction
    );

    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<(), RewardExecutionError> {
    let permission = Permissions::EXECUTE_REWARD_PAYOUT.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission).await? {
        Ok(())
    } else {
        Err(RewardExecutionError::PermissionDenied(permission))
    }
}

fn ensure_candidate_ready_for_payout(
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be amount approved before payout planning".to_string(),
        ))
    }
}

fn approved_positive_amount(
    candidate: &RewardCandidate,
) -> Result<BigDecimal, RewardExecutionError> {
    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }
    Ok(amount)
}

async fn credit_reward_wallet_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_credit: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    let amount = approved_positive_amount(candidate)?;
    let credit_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await?;

    if let Some(record) = credit_record.as_ref() {
        return Ok(RewardWalletCreditResult {
            candidate_id: candidate.id,
            wallet_id: record.wallet_id,
            credit_record_id: Some(record.id),
            transaction_id: Some(record.transaction_id),
            internal_transaction_id: Some(record.internal_transaction_id),
            amount,
            credited: false,
        });
    }

    if [
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
    ]
    .contains(&candidate.status.as_str())
    {
        return Err(RewardExecutionError::InvalidStatus(
            "wallet credited candidate is missing a reward wallet credit record".to_string(),
        ));
    }

    if candidate.status == REWARD_STATUS_NEEDS_RECONCILIATION {
        if !allow_reconciliation_credit {
            return Err(RewardExecutionError::InvalidStatus(
                "needs reconciliation candidate requires confirmed payout evidence before wallet credit"
                    .to_string(),
            ));
        }
    } else {
        ensure_wallet_credit_allowed(conn, candidate).await?;
    }

    let wallet = wallet_service::link_user_wallet(conn, candidate.student_user_id)
        .await?
        .wallet;
    let wallet = credit_wallet_balance(conn, wallet.id, amount.clone()).await?;
    let internal_transaction_id =
        create_internal_transaction(conn, wallet.id, amount.clone()).await?;
    let transaction_id = create_wallet_credit_transaction(conn, internal_transaction_id).await?;
    let credit_record = reward_wallet_credit_record_repository::create_reward_wallet_credit_record(
        conn,
        NewRewardWalletCreditRecord {
            reward_candidate_id: candidate.id,
            wallet_id: wallet.id,
            transaction_id,
            internal_transaction_id,
        },
    )
    .await?;
    let updated = mark_candidate_wallet_credited(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDITED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": wallet.id,
                "credit_record_id": credit_record.id,
                "transaction_id": transaction_id,
                "internal_transaction_id": internal_transaction_id,
            }),
        },
    )
    .await?;

    Ok(RewardWalletCreditResult {
        candidate_id: candidate.id,
        wallet_id: wallet.id,
        credit_record_id: Some(credit_record.id),
        transaction_id: Some(transaction_id),
        internal_transaction_id: Some(internal_transaction_id),
        amount,
        credited: true,
    })
}

async fn notify_reward_wallet_credit_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let allowed_to_inspect_notification = [
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
        || (allow_reconciliation_repair
            && [
                REWARD_STATUS_AMOUNT_APPROVED,
                REWARD_STATUS_TOKEN_CONFIRMED,
                REWARD_STATUS_NEEDS_RECONCILIATION,
            ]
            .contains(&candidate.status.as_str()));

    if !allowed_to_inspect_notification {
        return Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be wallet credited before notification".to_string(),
        ));
    }

    let amount = approved_positive_amount(candidate)?;
    let credit_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await?
        .ok_or_else(|| {
            RewardExecutionError::InvalidStatus(
                "wallet credited candidate is missing a reward wallet credit record".to_string(),
            )
        })?;

    if let Some(notification_id) = credit_record.notification_id {
        return Ok(RewardWalletCreditNotificationResult {
            candidate_id: candidate.id,
            wallet_id: credit_record.wallet_id,
            notification_id: Some(notification_id),
            transaction_id: credit_record.transaction_id,
            amount,
            notified: false,
        });
    }

    let can_create_missing_notification = candidate.status == REWARD_STATUS_WALLET_CREDITED
        || (allow_reconciliation_repair
            && [
                REWARD_STATUS_AMOUNT_APPROVED,
                REWARD_STATUS_TOKEN_CONFIRMED,
                REWARD_STATUS_NEEDS_RECONCILIATION,
            ]
            .contains(&candidate.status.as_str()));

    if !can_create_missing_notification {
        return Err(RewardExecutionError::InvalidStatus(
            "notified candidate is missing its notification reference".to_string(),
        ));
    }

    let course_title = courses::table
        .find(candidate.course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await?;
    let message = reward_wallet_credit_notification(
        candidate.course_id,
        course_title,
        amount.to_string(),
        credit_record.wallet_id,
        credit_record.transaction_id,
    );
    let notification_id = create_notification(
        conn,
        candidate.student_user_id,
        message.title,
        message.body,
    )
    .await
    .map_err(|e| RewardExecutionError::Database(e.to_string()))?;
    reward_wallet_credit_record_repository::mark_reward_wallet_credit_record_notified(
        conn,
        credit_record.id,
        notification_id,
    )
    .await?;
    let updated = mark_candidate_notified(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": credit_record.wallet_id,
                "notification_id": notification_id,
                "transaction_id": credit_record.transaction_id,
            }),
        },
    )
    .await?;

    Ok(RewardWalletCreditNotificationResult {
        candidate_id: candidate.id,
        wallet_id: credit_record.wallet_id,
        notification_id: Some(notification_id),
        transaction_id: credit_record.transaction_id,
        amount,
        notified: true,
    })
}

fn ensure_candidate_reconcilable(candidate: &RewardCandidate) -> Result<(), RewardExecutionError> {
    if [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
    {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate has no confirmed state to reconcile".to_string(),
        ))
    }
}

fn should_create_reconciliation_wallet_credit(candidate: &RewardCandidate) -> bool {
    [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
}

async fn ensure_wallet_credit_allowed(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_TOKEN_CONFIRMED {
        return Ok(());
    }

    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        let plan = plan_reward_payout(conn, candidate.id).await?;
        if plan.payout_method == REWARD_PAYOUT_METHOD_OFF_CHAIN {
            return Ok(());
        }
    }

    Err(RewardExecutionError::InvalidStatus(
        "wallet credit requires token confirmation unless the reward policy is off-chain"
            .to_string(),
    ))
}

async fn credit_wallet_balance(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<Wallet> {
    diesel::update(wallets::table.find(wallet_id))
        .set(wallets::value.eq(wallets::value + amount))
        .get_result(conn)
        .await
}

async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<i64> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
}

async fn create_wallet_credit_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> QueryResult<i64> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(transaction_id)
}

async fn ensure_internal_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    internal_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .on_conflict((
            transactions_internal_transactions::transaction_id,
            transactions_internal_transactions::internal_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}

async fn mark_candidate_wallet_credited(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_WALLET_CREDITED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

async fn mark_candidate_notified(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_NOTIFIED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

fn validate_token_confirmation_request(
    request: &RewardTokenConfirmationRequest,
) -> Result<(), RewardExecutionError> {
    if request.chain_id <= 0 {
        return Err(RewardExecutionError::InvalidInput(
            "chain_id must be positive".to_string(),
        ));
    }
    if request.contract_address.trim().is_empty() {
        return Err(RewardExecutionError::InvalidInput(
            "contract_address is required".to_string(),
        ));
    }
    if request.transaction_hash.trim().is_empty() {
        return Err(RewardExecutionError::InvalidInput(
            "transaction_hash is required".to_string(),
        ));
    }
    if request.log_index < 0 {
        return Err(RewardExecutionError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    if request.to_address.trim().is_empty() {
        return Err(RewardExecutionError::InvalidInput(
            "to_address is required".to_string(),
        ));
    }
    if request.amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "amount must be positive".to_string(),
        ));
    }
    transaction_type_for_event(&request.event_type)?;

    Ok(())
}

async fn record_external_reward_transaction(
    conn: &mut AsyncPgConnection,
    request: &RewardTokenConfirmationRequest,
) -> Result<(i64, i64, bool), RewardExecutionError> {
    if let Some(existing) = find_external_transaction_by_chain_tx_log(
        conn,
        request.chain_id,
        &request.transaction_hash,
        request.log_index,
    )
    .await?
    {
        let transaction_id = match find_transaction_for_external(conn, existing.id).await? {
            Some(transaction_id) => transaction_id,
            None => create_transaction_for_external(conn, existing.id, &request.event_type).await?,
        };
        return Ok((transaction_id, existing.id, false));
    }

    let transaction_id =
        create_transaction(conn, transaction_type_for_event(&request.event_type)?).await?;
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: request.amount.clone(),
            blockchain_address: &request.to_address,
            chain_id: Some(request.chain_id),
            contract_address: Some(&request.contract_address),
            transaction_hash: Some(&request.transaction_hash),
            log_index: Some(request.log_index),
            event_type: Some(request.event_type.as_str()),
            from_address: request.from_address.as_deref(),
            to_address: Some(&request.to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;

    Ok((transaction_id, external_transaction_id, true))
}

async fn find_external_transaction_by_chain_tx_log(
    conn: &mut AsyncPgConnection,
    chain_id: i64,
    transaction_hash: &str,
    log_index: i64,
) -> QueryResult<Option<ExternalTransaction>> {
    external_transactions::table
        .filter(external_transactions::chain_id.eq(chain_id))
        .filter(external_transactions::transaction_hash.eq(transaction_hash))
        .filter(external_transactions::log_index.eq(log_index))
        .first(conn)
        .await
        .optional()
}

async fn find_transaction_for_external(
    conn: &mut AsyncPgConnection,
    external_transaction_id: i64,
) -> QueryResult<Option<i64>> {
    transactions_external_transactions::table
        .filter(
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        )
        .select(transactions_external_transactions::transaction_id)
        .first(conn)
        .await
        .optional()
}

async fn create_transaction_for_external(
    conn: &mut AsyncPgConnection,
    external_transaction_id: i64,
    event_type: &str,
) -> Result<i64, RewardExecutionError> {
    let transaction_id = create_transaction(conn, transaction_type_for_event(event_type)?).await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;
    Ok(transaction_id)
}

async fn create_transaction(
    conn: &mut AsyncPgConnection,
    transaction_type: &str,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: transaction_type,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
}

async fn link_transaction_external(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> QueryResult<usize> {
    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await
}

async fn ensure_external_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .on_conflict((
            transactions_external_transactions::transaction_id,
            transactions_external_transactions::external_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}

async fn mark_candidate_token_confirmed(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_TOKEN_CONFIRMED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

fn transaction_type_for_event(event_type: &str) -> Result<&'static str, RewardExecutionError> {
    match event_type {
        "mint" => Ok("token_mint"),
        "transfer" => Ok("token_transfer"),
        "import" => Ok("token_import"),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported token event type".to_string(),
        )),
    }
}

async fn select_payout_method(
    conn: &mut AsyncPgConnection,
    policy: &RewardPolicy,
) -> Result<String, RewardExecutionError> {
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER => {
            if has_presigner_contract(conn).await? {
                Ok(REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER.to_string())
            } else {
                Ok(REWARD_PAYOUT_METHOD_TREASURY_TRANSFER.to_string())
            }
        }
        REWARD_PAYMENT_MINT => Ok(REWARD_PAYOUT_METHOD_MINT.to_string()),
        REWARD_PAYMENT_OFF_CHAIN => Ok(REWARD_PAYOUT_METHOD_OFF_CHAIN.to_string()),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}

async fn has_presigner_contract(
    conn: &mut AsyncPgConnection,
) -> Result<bool, RewardExecutionError> {
    let address = get_persistent_state(conn, "learn_token_presigner_address").await?;
    Ok(address
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false))
}

async fn resolve_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Option<RewardPolicy>> {
    if let Some(policy) = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()?
    {
        return Ok(Some(policy));
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        if let Some(policy) = reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
            .filter(reward_policies::organization_id.eq_any(organization_ids))
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .order((
                reward_policies::version.desc(),
                reward_policies::created_at.desc(),
            ))
            .first::<RewardPolicy>(conn)
            .await
            .optional()?
        {
            return Ok(Some(policy));
        }
    }

    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
}
