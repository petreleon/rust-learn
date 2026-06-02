use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, reward_candidates, transactions, wallets};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_COMPLETED,
};
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::reward_candidate_repository::find_candidate;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_compensation_service::{
    record_reward_compensation, RewardCompensationError, RewardCompensationRequest,
    REWARD_TRANSACTION_TYPE_COMPENSATION,
};
use serde_json::json;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_completed_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
) -> i64 {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("reward_compensation_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: REWARD_STATUS_COMPLETED.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(chrono::Utc::now())),
        ))
        .execute(conn)
        .await
        .expect("failed to mark reward candidate completed");

    candidate_id
}

#[actix_web::test]
async fn reward_compensation_records_adjust_wallet_without_mutating_candidate_decision() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("CompensationCourse")).await;
    let admin = create_user_helper(&mut conn, "compensation_admin").await;
    let stranger = create_user_helper(&mut conn, "compensation_stranger").await;
    let student = create_user_helper(&mut conn, "compensation_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    let candidate_id =
        create_completed_reward_candidate(&mut conn, course.id, student.id(), admin.id()).await;

    let denied = record_reward_compensation(
        &mut conn,
        stranger.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key: unique_string("compensation_denied"),
        },
    )
    .await
    .expect_err("user without platform wallet permission should not compensate rewards");
    assert!(matches!(
        denied,
        RewardCompensationError::PermissionDenied(_)
    ));

    let idempotency_key = unique_string("compensation_create");
    let compensation = record_reward_compensation(
        &mut conn,
        admin.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key: idempotency_key.clone(),
        },
    )
    .await
    .expect("admin should record reward compensation");
    assert!(compensation.created);
    assert_eq!(compensation.record.reward_candidate_id, candidate_id);
    assert_eq!(compensation.record.amount, BigDecimal::from(5));
    assert_eq!(compensation.record.reason, "manual make-good");
    assert_eq!(compensation.wallet.value, BigDecimal::from(5));

    let duplicate = record_reward_compensation(
        &mut conn,
        admin.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key,
        },
    )
    .await
    .expect("idempotent compensation retry should succeed");
    assert!(!duplicate.created);
    assert_eq!(duplicate.record.id, compensation.record.id);
    assert_eq!(duplicate.wallet.value, BigDecimal::from(5));

    let transaction_type = transactions::table
        .find(compensation.record.transaction_id)
        .select(transactions::type_)
        .first::<String>(&mut conn)
        .await
        .expect("compensation transaction should exist");
    assert_eq!(transaction_type, REWARD_TRANSACTION_TYPE_COMPENSATION);

    let wallet_value = wallets::table
        .find(compensation.wallet.id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should remain queryable");
    assert_eq!(wallet_value, BigDecimal::from(5));

    let candidate_after_compensation = find_candidate(&mut conn, candidate_id)
        .await
        .expect("candidate should remain queryable");
    assert_eq!(candidate_after_compensation.status, REWARD_STATUS_COMPLETED);
    assert_eq!(
        candidate_after_compensation.approved_amount,
        Some(BigDecimal::from(10))
    );
}
