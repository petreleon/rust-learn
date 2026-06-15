use crate::http_support::{assign_organization_permission_role, assign_organization_role};
use crate::support::*;
use crate::wallet_org_audit_helpers::{
    assert_organization_wallet_audit, OrganizationWalletAuditExpectation,
};

#[actix_web::test]
async fn organization_wallet_audit_includes_source_org_reward_rows_and_gates_access() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "wallet_org_audit_admin").await;
    let org_reporter = create_test_user(&mut conn, "wallet_org_audit_reporter").await;
    let stranger = create_test_user(&mut conn, "wallet_org_audit_stranger").await;
    let student = create_test_user(&mut conn, "wallet_org_audit_student").await;
    let submitter = create_test_user(&mut conn, "wallet_org_audit_submitter").await;
    let org = create_test_organization(&mut conn).await;
    let course_id = create_test_course(&mut conn).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    assign_organization_permission_role(
        &mut conn,
        org_reporter.id(),
        org.id,
        Permissions::VIEW_ORG_REWARD_REPORTS,
    )
    .await;
    let wallet = link_organization_wallet(&mut conn, org_admin.id(), org.id)
        .await
        .expect("organization wallet should link")
        .wallet;

    let amount = BigDecimal::from(75);
    diesel::update(wallets::table.find(wallet.id))
        .set(wallets::value.eq(amount.clone()))
        .execute(&mut conn)
        .await
        .expect("failed to seed organization wallet balance");

    let internal_transaction_id: i64 = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet.id),
            internal_transactions::amount.eq(amount.clone()),
        ))
        .returning(internal_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization internal transaction");
    let wallet_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("organization_budget_adjustment"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization wallet transaction");
    diesel::insert_into(transactions_internal_transactions::table)
        .values((
            transactions_internal_transactions::transaction_id.eq(wallet_transaction_id),
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link organization internal transaction");

    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id: student.id(),
            submitter_user_id: submitter.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: Some(org.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("wallet_org_audit_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: REWARD_STATUS_TOKEN_CONFIRMED.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization reward candidate");
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(amount.clone())),
            reward_candidates::amount_reviewer_user_id.eq(Some(org_admin.id())),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to mark organization reward amount");

    let payout_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization reward payout transaction");
    let external_transaction_id: i64 = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount.clone()),
            external_transactions::blockchain_address.eq("0xorgstudent"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xorgcontract")),
            external_transactions::transaction_hash.eq(Some(unique_string("wallet_org_audit_tx"))),
            external_transactions::log_index.eq(Some(1_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xorgtreasury")),
            external_transactions::to_address.eq(Some("0xorgstudent")),
        ))
        .returning(external_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization reward external transaction");
    diesel::insert_into(transactions_external_transactions::table)
        .values((
            transactions_external_transactions::transaction_id.eq(payout_transaction_id),
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link organization external transaction");
    let payout_record_id: i64 = diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .returning(reward_payout_records::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create organization reward payout record");
    drop(conn);

    assert_organization_wallet_audit(
        &pool,
        OrganizationWalletAuditExpectation {
            org_id: org.id,
            wallet_id: wallet.id,
            stranger_id: stranger.id(),
            reporter_id: org_reporter.id(),
            internal_transaction_id,
            wallet_transaction_id,
            external_transaction_id,
            payout_transaction_id,
            candidate_id,
            payout_record_id,
        },
    )
    .await;
}
