use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, notifications, platform_roles,
    reward_candidates, reward_payout_records, reward_policies, reward_wallet_credit_records,
    role_permission_platform, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_audit_event::{
    REWARD_AUDIT_EVENT_TOKEN_CONFIRMED, REWARD_AUDIT_EVENT_WALLET_CREDITED,
    REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, RewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN,
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::persistent_state_repository::set_persistent_state;
use rust_learn::repositories::reward_audit_event_repository::list_reward_audit_events;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_execution_service::{
    credit_reward_wallet, credit_reward_wallet_for_actor, notify_reward_wallet_credit,
    plan_reward_payout, reconcile_reward_candidate, record_reward_token_confirmation,
    record_reward_token_confirmation_for_actor, RewardExecutionError,
    RewardTokenConfirmationRequest, REWARD_PAYOUT_METHOD_MINT,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER, REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
};
use serde_json::json;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn unique_hash(prefix: &str) -> String {
    format!(
        "0x{}{:x}{:x}",
        prefix,
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    )
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
}

async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description.eq(Some(
                "test-only reward execution permission bundle".to_string(),
            )),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom platform role");

    for permission in permissions {
        diesel::insert_into(role_permission_platform::table)
            .values((
                role_permission_platform::platform_role_id.eq(Some(role_id)),
                role_permission_platform::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom platform role permission");
    }

    role_id
}

async fn assign_platform_role_id(conn: &mut AsyncPgConnection, user_id: i32, role_id: i32) {
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    payment_strategy: &str,
) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: payment_strategy.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
) -> RewardCandidate {
    let candidate = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("reward_execution_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .get_result::<RewardCandidate>(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate.id))
        .set((
            reward_candidates::approved_amount.eq(approved_amount),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
            reward_candidates::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
        .expect("failed to update reward candidate amount")
}

#[actix_web::test]
async fn execute_reward_payout_permission_gates_token_confirmation_and_wallet_credit() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PayoutPermissionCourse")).await;
    let student = create_user_helper(&mut conn, "payout_permission_student").await;
    let submitter = create_user_helper(&mut conn, "payout_permission_submitter").await;
    let executor = create_user_helper(&mut conn, "payout_permission_executor").await;
    let no_permission_user = create_user_helper(&mut conn, "payout_permission_denied").await;
    let executor_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("REWARD_PAYOUT_EXECUTOR"),
        &[Permissions::EXECUTE_REWARD_PAYOUT],
    )
    .await;
    assign_platform_role_id(&mut conn, executor.id(), executor_role_id).await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(19)),
    )
    .await;
    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x0000000000000000000000000000000000000101".to_string(),
        transaction_hash: unique_hash("permission"),
        log_index: 5,
        event_type: "transfer".to_string(),
        from_address: Some("0x0000000000000000000000000000000000000102".to_string()),
        to_address: "0x0000000000000000000000000000000000000103".to_string(),
        amount: BigDecimal::from(19),
    };

    let denied_confirmation = record_reward_token_confirmation_for_actor(
        &mut conn,
        no_permission_user.id(),
        candidate.id,
        request.clone(),
    )
    .await
    .expect_err("token confirmation should require reward payout execution permission");
    assert!(matches!(
        denied_confirmation,
        RewardExecutionError::PermissionDenied(permission)
            if permission == Permissions::EXECUTE_REWARD_PAYOUT.to_string()
    ));

    let status_after_denied_confirmation = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(
        status_after_denied_confirmation,
        REWARD_STATUS_TOKEN_PENDING
    );

    record_reward_token_confirmation_for_actor(&mut conn, executor.id(), candidate.id, request)
        .await
        .expect("payout executor should record token confirmation");

    let denied_wallet_credit =
        credit_reward_wallet_for_actor(&mut conn, no_permission_user.id(), candidate.id)
            .await
            .expect_err("wallet credit should require reward payout execution permission");
    assert!(matches!(
        denied_wallet_credit,
        RewardExecutionError::PermissionDenied(permission)
            if permission == Permissions::EXECUTE_REWARD_PAYOUT.to_string()
    ));

    let credit_record_count_after_denied: i64 = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(candidate.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("wallet credit records should be countable");
    assert_eq!(credit_record_count_after_denied, 0);

    let credited = credit_reward_wallet_for_actor(&mut conn, executor.id(), candidate.id)
        .await
        .expect("payout executor should credit the reward wallet");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(19));

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("reward execution audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED);
    assert_eq!(audit[0].actor_user_id, Some(executor.id()));
    assert_eq!(audit[1].event_type, REWARD_AUDIT_EVENT_WALLET_CREDITED);
    assert_eq!(audit[1].actor_user_id, Some(executor.id()));
}

#[actix_web::test]
async fn treasury_policy_uses_presigner_when_contract_is_available() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TreasuryPresignerCourse")).await;
    let student = create_user_helper(&mut conn, "treasury_presigner_student").await;
    let submitter = create_user_helper(&mut conn, "treasury_presigner_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;
    set_persistent_state(
        &mut conn,
        "learn_token_presigner_address",
        "0x00000000000000000000000000000000000000aa",
    )
    .await
    .expect("failed to set presigner address");

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(10)),
    )
    .await;

    let plan = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect("amount-approved candidate should produce a payout plan");

    assert_eq!(plan.payment_strategy, REWARD_PAYMENT_TREASURY_TRANSFER);
    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER);
    assert!(plan.requires_token_confirmation);
    assert_eq!(plan.amount, BigDecimal::from(10));
}

#[actix_web::test]
async fn mint_method_requires_explicit_mint_policy() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("ExplicitMintCourse")).await;
    let student = create_user_helper(&mut conn, "explicit_mint_student").await;
    let submitter = create_user_helper(&mut conn, "explicit_mint_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_MINT).await;
    set_persistent_state(
        &mut conn,
        "learn_token_presigner_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to set presigner address");

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(12)),
    )
    .await;

    let plan = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect("mint policy should produce a mint payout plan");

    assert_eq!(plan.payment_strategy, REWARD_PAYMENT_MINT);
    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_MINT);
    assert!(plan.requires_token_confirmation);
}

#[actix_web::test]
async fn candidate_must_be_amount_approved_before_payout_planning() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PendingPayoutCourse")).await;
    let student = create_user_helper(&mut conn, "pending_payout_student").await;
    let submitter = create_user_helper(&mut conn, "pending_payout_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        None,
    )
    .await;

    let denied = plan_reward_payout(&mut conn, candidate.id)
        .await
        .expect_err("candidate must be amount approved before payout planning");

    assert!(matches!(denied, RewardExecutionError::InvalidStatus(_)));
}

#[actix_web::test]
async fn token_confirmed_candidate_credits_wallet_once() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TokenConfirmedCreditCourse")).await;
    let student = create_user_helper(&mut conn, "token_confirmed_credit_student").await;
    let submitter = create_user_helper(&mut conn, "token_confirmed_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
        Some(BigDecimal::from(15)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("token-confirmed candidate should credit wallet");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(15));
    let credit_record_id = credited
        .credit_record_id
        .expect("wallet credit should create credit record");

    let wallet_value = wallets::table
        .find(credited.wallet_id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should exist");
    assert_eq!(wallet_value, BigDecimal::from(15));

    let transaction_id = credited
        .transaction_id
        .expect("wallet credit should create transaction");
    let internal_transaction_id = credited
        .internal_transaction_id
        .expect("wallet credit should create internal transaction");
    let transaction_type = transactions::table
        .find(transaction_id)
        .select(transactions::type_)
        .first::<String>(&mut conn)
        .await
        .expect("wallet credit transaction should exist");
    assert_eq!(transaction_type, REWARD_TRANSACTION_TYPE_WALLET_CREDIT);

    let internal_amount = internal_transactions::table
        .find(internal_transaction_id)
        .select(internal_transactions::amount)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet credit internal transaction should exist");
    assert_eq!(internal_amount, BigDecimal::from(15));

    let link_count = transactions_internal_transactions::table
        .filter(transactions_internal_transactions::transaction_id.eq(transaction_id))
        .filter(
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("wallet credit transaction link should be queryable");
    assert_eq!(link_count, 1);

    let credit_record = reward_wallet_credit_records::table
        .find(credit_record_id)
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reward wallet credit record should exist");
    assert_eq!(credit_record.reward_candidate_id, candidate.id);
    assert_eq!(credit_record.wallet_id, credited.wallet_id);
    assert_eq!(credit_record.transaction_id, transaction_id);
    assert_eq!(
        credit_record.internal_transaction_id,
        internal_transaction_id
    );
    assert_eq!(credit_record.notification_id, None);

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_WALLET_CREDITED);

    let duplicate = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("wallet credit should be idempotent after status update");
    assert!(!duplicate.credited);
    assert_eq!(duplicate.credit_record_id, credited.credit_record_id);
    assert_eq!(duplicate.transaction_id, credited.transaction_id);
    assert_eq!(
        duplicate.internal_transaction_id,
        credited.internal_transaction_id
    );

    let wallet_value_after_duplicate = wallets::table
        .find(credited.wallet_id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should still exist");
    assert_eq!(wallet_value_after_duplicate, BigDecimal::from(15));
}

#[actix_web::test]
async fn wallet_credit_notification_persists_context_and_is_idempotent() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardNotificationCourse")).await;
    let student = create_user_helper(&mut conn, "reward_notification_student").await;
    let submitter = create_user_helper(&mut conn, "reward_notification_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
        Some(BigDecimal::from(21)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("token-confirmed candidate should credit wallet");
    let transaction_id = credited
        .transaction_id
        .expect("wallet credit should create a transaction");

    let sent = notify_reward_wallet_credit(&mut conn, candidate.id)
        .await
        .expect("wallet credited candidate should notify the student");
    assert!(sent.notified);
    assert_eq!(sent.wallet_id, credited.wallet_id);
    assert_eq!(sent.transaction_id, transaction_id);
    assert_eq!(sent.amount, BigDecimal::from(21));

    let notification_id = sent
        .notification_id
        .expect("notification result should include notification id");
    let notification = notifications::table
        .find(notification_id)
        .first::<rust_learn::models::notification::Notification>(&mut conn)
        .await
        .expect("reward wallet notification should exist");
    assert_eq!(notification.user_id, Some(student.id()));
    assert_eq!(notification.title, "reward:wallet_credited");
    assert!(notification
        .body
        .contains(&format!("course #{}", course.id)));
    assert!(notification.body.contains(course.title.as_str()));
    assert!(notification.body.contains("21 LearnToken"));
    assert!(notification
        .body
        .contains(&format!("wallet #{}", credited.wallet_id)));
    assert!(notification
        .body
        .contains(&format!("Transaction #{}", transaction_id)));

    let credit_record = reward_wallet_credit_records::table
        .find(
            credited
                .credit_record_id
                .expect("wallet credit should create record"),
        )
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reward wallet credit record should exist");
    assert_eq!(credit_record.notification_id, Some(notification_id));
    assert!(credit_record.notified_at.is_some());

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_NOTIFIED);

    let duplicate = notify_reward_wallet_credit(&mut conn, candidate.id)
        .await
        .expect("duplicate wallet credit notification should be idempotent");
    assert!(!duplicate.notified);
    assert_eq!(duplicate.notification_id, Some(notification_id));
    assert_eq!(duplicate.transaction_id, transaction_id);

    let notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward wallet notifications should be countable");
    assert_eq!(notification_count, 1);

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("wallet credit audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_WALLET_CREDITED);
    assert_eq!(
        audit[0].from_status.as_deref(),
        Some(REWARD_STATUS_TOKEN_CONFIRMED)
    );
    assert_eq!(audit[0].to_status, REWARD_STATUS_WALLET_CREDITED);
    assert_eq!(
        audit[1].event_type,
        REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED
    );
    assert_eq!(
        audit[1].from_status.as_deref(),
        Some(REWARD_STATUS_WALLET_CREDITED)
    );
    assert_eq!(audit[1].to_status, REWARD_STATUS_NOTIFIED);
}

#[actix_web::test]
async fn token_policy_cannot_credit_wallet_before_token_confirmation() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("BlockedCreditCourse")).await;
    let student = create_user_helper(&mut conn, "blocked_credit_student").await;
    let submitter = create_user_helper(&mut conn, "blocked_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(11)),
    )
    .await;

    let denied = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect_err("token payout should not credit wallet before token confirmation");
    assert!(matches!(denied, RewardExecutionError::InvalidStatus(_)));
}

#[actix_web::test]
async fn off_chain_policy_can_credit_wallet_after_amount_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("OffChainCreditCourse")).await;
    let student = create_user_helper(&mut conn, "off_chain_credit_student").await;
    let submitter = create_user_helper(&mut conn, "off_chain_credit_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_OFF_CHAIN).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(9)),
    )
    .await;

    let credited = credit_reward_wallet(&mut conn, candidate.id)
        .await
        .expect("off-chain policy should credit wallet after amount approval");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(9));
}

#[actix_web::test]
async fn token_confirmation_records_external_transaction_and_candidate_link() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("TokenConfirmationCourse")).await;
    let student = create_user_helper(&mut conn, "token_confirmation_student").await;
    let submitter = create_user_helper(&mut conn, "token_confirmation_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(17)),
    )
    .await;

    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
        transaction_hash: unique_hash("reward"),
        log_index: 3,
        event_type: "transfer".to_string(),
        from_address: Some("0x00000000000000000000000000000000000000dd".to_string()),
        to_address: "0x00000000000000000000000000000000000000ee".to_string(),
        amount: BigDecimal::from(17),
    };

    let confirmation = record_reward_token_confirmation(&mut conn, candidate.id, request.clone())
        .await
        .expect("token-pending candidate should record token confirmation");
    assert!(confirmation.inserted_external_transaction);

    let external = external_transactions::table
        .find(confirmation.external_transaction_id)
        .first::<rust_learn::models::transaction::ExternalTransaction>(&mut conn)
        .await
        .expect("external transaction should exist");
    assert_eq!(external.chain_id, Some(request.chain_id));
    assert_eq!(
        external.contract_address.as_deref(),
        Some(request.contract_address.as_str())
    );
    assert_eq!(
        external.transaction_hash.as_deref(),
        Some(request.transaction_hash.as_str())
    );
    assert_eq!(external.log_index, Some(request.log_index));
    assert_eq!(external.event_type.as_deref(), Some("transfer"));
    assert_eq!(
        external.from_address.as_deref(),
        request.from_address.as_deref()
    );
    assert_eq!(
        external.to_address.as_deref(),
        Some(request.to_address.as_str())
    );
    assert_eq!(external.amount, request.amount);

    let payout_record = reward_payout_records::table
        .find(confirmation.payout_record_id)
        .first::<rust_learn::models::reward_payout_record::RewardPayoutRecord>(&mut conn)
        .await
        .expect("reward payout record should exist");
    assert_eq!(payout_record.reward_candidate_id, candidate.id);
    assert_eq!(
        payout_record.external_transaction_id,
        confirmation.external_transaction_id
    );
    assert_eq!(payout_record.transaction_id, confirmation.transaction_id);

    let transaction_link_count = transactions_external_transactions::table
        .filter(transactions_external_transactions::transaction_id.eq(confirmation.transaction_id))
        .filter(
            transactions_external_transactions::external_transaction_id
                .eq(confirmation.external_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("external transaction link should be queryable");
    assert_eq!(transaction_link_count, 1);

    let candidate_status = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(candidate_status, REWARD_STATUS_TOKEN_CONFIRMED);

    let duplicate = record_reward_token_confirmation(&mut conn, candidate.id, request)
        .await
        .expect("duplicate token confirmation should be idempotent");
    assert!(!duplicate.inserted_external_transaction);
    assert_eq!(duplicate.transaction_id, confirmation.transaction_id);
    assert_eq!(
        duplicate.external_transaction_id,
        confirmation.external_transaction_id
    );
    assert_eq!(duplicate.payout_record_id, confirmation.payout_record_id);

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("token confirmation audit events should load");
    assert_eq!(
        audit.len(),
        1,
        "duplicate token confirmation must not create a second audit transition"
    );
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED);
    assert_eq!(
        audit[0].from_status.as_deref(),
        Some(REWARD_STATUS_TOKEN_PENDING)
    );
    assert_eq!(audit[0].to_status, REWARD_STATUS_TOKEN_CONFIRMED);
}

#[actix_web::test]
async fn reconciliation_repairs_reward_side_effects_without_duplicate_payouts() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardReconciliationCourse")).await;
    let student = create_user_helper(&mut conn, "reward_reconciliation_student").await;
    let submitter = create_user_helper(&mut conn, "reward_reconciliation_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(17)),
    )
    .await;

    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x00000000000000000000000000000000000000ff".to_string(),
        transaction_hash: unique_hash("reconcile"),
        log_index: 4,
        event_type: "transfer".to_string(),
        from_address: Some("0x00000000000000000000000000000000000000ab".to_string()),
        to_address: "0x00000000000000000000000000000000000000ac".to_string(),
        amount: BigDecimal::from(17),
    };

    let confirmation = record_reward_token_confirmation(&mut conn, candidate.id, request)
        .await
        .expect("token confirmation should create payout evidence");
    diesel::delete(
        transactions_external_transactions::table
            .filter(
                transactions_external_transactions::transaction_id.eq(confirmation.transaction_id),
            )
            .filter(
                transactions_external_transactions::external_transaction_id
                    .eq(confirmation.external_transaction_id),
            ),
    )
    .execute(&mut conn)
    .await
    .expect("external transaction link should be deletable for reconciliation test");

    let first = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("reconciliation should repair token-confirmed reward");
    assert!(first.external_transaction_link_repaired);
    assert!(first.wallet_credit_created);
    assert!(first.notification_created);
    assert!(!first.internal_transaction_link_repaired);
    assert_eq!(first.final_status, REWARD_STATUS_NOTIFIED);

    let wallet = wallets::table
        .filter(wallets::user_id.eq(Some(student.id())))
        .filter(wallets::organization_id.is_null())
        .first::<rust_learn::models::wallet::Wallet>(&mut conn)
        .await
        .expect("wallet should exist after reconciliation");
    assert_eq!(wallet.value, BigDecimal::from(17));

    let external_link_count = transactions_external_transactions::table
        .filter(transactions_external_transactions::transaction_id.eq(confirmation.transaction_id))
        .filter(
            transactions_external_transactions::external_transaction_id
                .eq(confirmation.external_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("external link should be countable after reconciliation");
    assert_eq!(external_link_count, 1);

    let credit_record = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(candidate.id))
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reconciliation should create reward wallet credit record");
    assert!(credit_record.notification_id.is_some());

    let notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward notifications should be countable");
    assert_eq!(notification_count, 1);

    diesel::delete(
        transactions_internal_transactions::table
            .filter(
                transactions_internal_transactions::transaction_id.eq(credit_record.transaction_id),
            )
            .filter(
                transactions_internal_transactions::internal_transaction_id
                    .eq(credit_record.internal_transaction_id),
            ),
    )
    .execute(&mut conn)
    .await
    .expect("internal transaction link should be deletable for reconciliation test");

    let second = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("reconciliation should repair missing internal link");
    assert!(!second.external_transaction_link_repaired);
    assert!(!second.wallet_credit_created);
    assert!(!second.notification_created);
    assert!(second.internal_transaction_link_repaired);
    assert_eq!(second.final_status, REWARD_STATUS_NOTIFIED);

    let wallet_after_second_reconcile = wallets::table
        .find(wallet.id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should still exist after second reconciliation");
    assert_eq!(wallet_after_second_reconcile, BigDecimal::from(17));

    let notification_count_after_second_reconcile = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward notifications should still be countable");
    assert_eq!(notification_count_after_second_reconcile, 1);

    let third = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("fully repaired reward should reconcile idempotently");
    assert!(!third.external_transaction_link_repaired);
    assert!(!third.wallet_credit_created);
    assert!(!third.notification_created);
    assert!(!third.internal_transaction_link_repaired);
    assert_eq!(third.final_status, REWARD_STATUS_NOTIFIED);
}
