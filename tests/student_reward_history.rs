use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records, transactions,
    transactions_external_transactions, transactions_internal_transactions, users,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_WALLET_CREDITED,
};
use rust_learn::models::role::CourseRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::wallet::Wallet;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::wallet_service;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

async fn setup_conn(
    pool: &rust_learn::db::DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_verified_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    let user = create_user(
        conn,
        prefix,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user");

    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to verify user email");

    user
}

async fn create_course(conn: &mut AsyncPgConnection, title_prefix: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string(title_prefix),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role should exist");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    amount: BigDecimal,
) -> i64 {
    let candidate_id = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("student_reward_history_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(amount)),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(chrono::Utc::now())),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .execute(conn)
        .await
        .expect("failed to set approved amount");

    candidate_id
}

struct RewardFinancialFixture {
    payout_transaction_id: i64,
    external_transaction_id: i64,
    wallet_transaction_id: i64,
    internal_transaction_id: i64,
    wallet_credit_record_id: i64,
    transaction_hash: String,
}

async fn create_reward_financial_records(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    wallet: &Wallet,
    amount: BigDecimal,
) -> RewardFinancialFixture {
    let payout_transaction_id = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create payout transaction");
    let transaction_hash = unique_string("student_reward_history_tx");
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount.clone()),
            external_transactions::blockchain_address.eq("0xstudenthistory"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xcontract")),
            external_transactions::transaction_hash.eq(Some(transaction_hash.clone())),
            external_transactions::log_index.eq(Some(0_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xtreasury")),
            external_transactions::to_address.eq(Some("0xstudenthistory")),
        ))
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create external transaction");
    diesel::insert_into(transactions_external_transactions::table)
        .values((
            transactions_external_transactions::transaction_id.eq(payout_transaction_id),
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to link payout transaction");
    diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to create payout record");

    let wallet_transaction_id = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet.id),
            internal_transactions::amount.eq(amount),
        ))
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create internal transaction");
    diesel::insert_into(transactions_internal_transactions::table)
        .values((
            transactions_internal_transactions::transaction_id.eq(wallet_transaction_id),
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to link wallet credit transaction");
    let wallet_credit_record_id = diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet.id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit record");

    RewardFinancialFixture {
        payout_transaction_id,
        external_transaction_id,
        wallet_transaction_id,
        internal_transaction_id,
        wallet_credit_record_id,
        transaction_hash,
    }
}

#[actix_web::test]
async fn student_reward_history_shows_owned_permitted_rewards_with_payment_refs() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let student = create_verified_user(&mut conn, "student_reward_history_student").await;
    let other_student = create_verified_user(&mut conn, "student_reward_history_other").await;
    let submitter = create_verified_user(&mut conn, "student_reward_history_submitter").await;
    let course = create_course(&mut conn, "StudentRewardHistoryCourse").await;
    let hidden_course = create_course(&mut conn, "StudentRewardHistoryHiddenCourse").await;
    assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    assign_course_role(&mut conn, other_student.id(), course.id, "STUDENT").await;
    let wallet = wallet_service::link_user_wallet(&mut conn, student.id())
        .await
        .expect("student wallet should link")
        .wallet;

    let amount = BigDecimal::from(12);
    let visible_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let financials =
        create_reward_financial_records(&mut conn, visible_candidate_id, &wallet, amount.clone())
            .await;
    let other_student_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        other_student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let hidden_candidate_id = create_reward_candidate(
        &mut conn,
        hidden_course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_WALLET_CREDITED,
        amount.clone(),
    )
    .await;
    let filtered_candidate_id = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        amount,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reward_candidates::reward_candidate_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/reward-candidates/me/history?status=wallet_credited")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let history: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(history.len(), 1);
    let row = &history[0];
    assert_eq!(
        row["reward_candidate_id"].as_i64(),
        Some(visible_candidate_id)
    );
    assert_eq!(row["course_id"].as_i64(), Some(i64::from(course.id)));
    assert_eq!(row["course_title"].as_str(), Some(course.title.as_str()));
    assert_eq!(row["status"], REWARD_STATUS_WALLET_CREDITED);
    assert_eq!(row["approved_amount"], "12");
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(other_student_candidate_id)
    );
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(hidden_candidate_id)
    );
    assert_ne!(
        row["reward_candidate_id"].as_i64(),
        Some(filtered_candidate_id)
    );

    let wallet_credit = &row["wallet_credit"];
    assert_eq!(
        wallet_credit["reward_wallet_credit_record_id"].as_i64(),
        Some(financials.wallet_credit_record_id)
    );
    assert_eq!(
        wallet_credit["wallet_id"].as_i64(),
        Some(i64::from(wallet.id))
    );
    assert_eq!(
        wallet_credit["transaction_id"].as_i64(),
        Some(financials.wallet_transaction_id)
    );
    assert_eq!(
        wallet_credit["internal_transaction_id"].as_i64(),
        Some(financials.internal_transaction_id)
    );
    assert_eq!(wallet_credit["amount"], "12");

    let token_transaction = &row["token_transaction"];
    assert_eq!(
        token_transaction["payout_transaction_id"].as_i64(),
        Some(financials.payout_transaction_id)
    );
    assert_eq!(
        token_transaction["external_transaction_id"].as_i64(),
        Some(financials.external_transaction_id)
    );
    assert_eq!(token_transaction["amount"], "12");
    assert_eq!(token_transaction["chain_id"].as_i64(), Some(31337));
    assert_eq!(token_transaction["event_type"], "Transfer");
    assert_eq!(
        token_transaction["transaction_hash"].as_str(),
        Some(financials.transaction_hash.as_str())
    );

    let invalid_req = test::TestRequest::get()
        .uri("/reward-candidates/me/history?status=not-a-real-status")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let invalid_resp = test::call_service(&app, invalid_req).await;
    assert_eq!(invalid_resp.status(), StatusCode::BAD_REQUEST);
}
