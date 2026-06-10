use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations, reward_policies, users};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED,
};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::models::reward_execution_job::REWARD_EXECUTION_STATUS_QUEUED;
use rust_learn::models::reward_fraud_block::{
    NewRewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::user::User;
use rust_learn::repositories::reward_audit_event_repository;
use rust_learn::repositories::reward_candidate_repository::{
    self, RewardCandidateFilter,
};
use rust_learn::repositories::reward_execution_job_repository;
use rust_learn::repositories::reward_fraud_block_repository::{
    self, RewardFraudBlockFilter,
};
use rust_learn::repositories::reward_policy_repository;
use rust_learn::repositories::user_repository::create_user;
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
    let user = create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

// ── reward_candidate_repository ──

#[actix_web::test]
async fn test_candidate_create_and_find() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "candidate_student").await;
    let course = create_course(&mut conn, &unique_string("candidate_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        NewRewardCandidate {
            course_id: course.id,
            student_user_id: student.id(),
            submitter_user_id: student.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("candidate_key"),
            evidence: json!({"completion_percentage": 100.0}),
            status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
        },
    )
    .await
    .expect("failed to create candidate");

    assert_eq!(candidate.course_id, course.id);
    assert_eq!(candidate.student_user_id, student.id());
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);

    let found = reward_candidate_repository::find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate not found by id");
    assert_eq!(found.id, candidate.id);
}

#[actix_web::test]
async fn test_candidate_find_by_idempotency_key() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_idem").await;
    let course = create_course(&mut conn, &unique_string("cand_idem_course")).await;
    let key = unique_string("idem_key");

    let created = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &key),
    )
    .await
    .unwrap();

    let found = reward_candidate_repository::find_candidate_by_idempotency_key(
        &mut conn, &key,
    )
    .await
    .unwrap()
    .expect("candidate not found by idempotency key");
    assert_eq!(found.id, created.id);

    let none = reward_candidate_repository::find_candidate_by_idempotency_key(
        &mut conn, "nonexistent",
    )
    .await
    .unwrap();
    assert!(none.is_none());
}

#[actix_web::test]
async fn test_candidate_list_and_count() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_list").await;
    let course_a = create_course(&mut conn, &unique_string("cand_list_a")).await;
    let course_b = create_course(&mut conn, &unique_string("cand_list_b")).await;

    for i in 0..3 {
        let status = if i == 0 {
            REWARD_STATUS_TEACHER_APPROVED.to_string()
        } else {
            REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string()
        };
        let mut candidate = new_candidate(
            if i < 2 { course_a.id } else { course_b.id },
            student.id(),
            &unique_string(&format!("list_key_{i}")),
        );
        candidate.status = status;
        reward_candidate_repository::create_candidate(&mut conn, candidate)
            .await
            .unwrap();
    }

    let all = reward_candidate_repository::list_candidates(
        &mut conn,
        RewardCandidateFilter::default(),
    )
    .await
    .unwrap();
    assert!(all.len() >= 3);

    let by_course = reward_candidate_repository::list_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course_a.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(by_course.len(), 2);

    let count = reward_candidate_repository::count_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course_a.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(count, 2);
}

#[actix_web::test]
async fn test_candidate_filter_by_status() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_status").await;
    let course = create_course(&mut conn, &unique_string("cand_status_course")).await;

    reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("status_a")),
    )
    .await
    .unwrap();

    let mut approved = new_candidate(course.id, student.id(), &unique_string("status_b"));
    approved.status = REWARD_STATUS_TEACHER_APPROVED.to_string();
    reward_candidate_repository::create_candidate(&mut conn, approved)
        .await
        .unwrap();

    let pending = reward_candidate_repository::list_candidates(
        &mut conn,
        RewardCandidateFilter {
            course_id: Some(course.id),
            status: Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
}

#[actix_web::test]
async fn test_candidate_teacher_decision_update() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_teacher").await;
    let teacher = create_user_helper(&mut conn, "cand_teacher_actor").await;
    let course = create_course(&mut conn, &unique_string("cand_teacher_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("teacher_dec")),
    )
    .await
    .unwrap();

    let now = Utc::now();
    let updated = reward_candidate_repository::update_teacher_decision(
        &mut conn,
        candidate.id,
        teacher.id(),
        REWARD_STATUS_TEACHER_APPROVED,
        Some("looks good"),
        now,
    )
    .await
    .unwrap();

    assert_eq!(updated.status, REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(updated.teacher_approver_user_id, Some(teacher.id()));
    assert_eq!(
        updated.teacher_decision_reason,
        Some("looks good".to_string())
    );
    assert!(updated.teacher_decided_at.is_some());
}

#[actix_web::test]
async fn test_candidate_amount_decision_update() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "cand_amount").await;
    let reviewer = create_user_helper(&mut conn, "cand_amount_rev").await;
    let course = create_course(&mut conn, &unique_string("cand_amount_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("amount_dec")),
    )
    .await
    .unwrap();

    let amount = BigDecimal::from(100);
    let now = Utc::now();
    let updated = reward_candidate_repository::update_amount_decision(
        &mut conn,
        candidate.id,
        reviewer.id(),
        "amount_approved",
        Some(amount.clone()),
        Some("approved amount"),
        now,
    )
    .await
    .unwrap();

    assert_eq!(updated.status, "amount_approved");
    assert_eq!(updated.amount_reviewer_user_id, Some(reviewer.id()));
    assert_eq!(updated.approved_amount, Some(amount));
    assert!(updated.amount_decided_at.is_some());
}

// ── reward_fraud_block_repository ──

#[actix_web::test]
async fn test_fraud_block_create_and_find() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_creator").await;
    let teacher = create_user_helper(&mut conn, "fraud_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "suspicious activity".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    assert_eq!(block.teacher_user_id, Some(teacher.id()));

    let found = reward_fraud_block_repository::find_reward_fraud_block(&mut conn, block.id)
        .await
        .unwrap();
    assert_eq!(found.id, block.id);
}

#[actix_web::test]
async fn test_fraud_block_list_and_filter() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_list_creator").await;

    let fraud_teacher = create_user_helper(&mut conn, "fraud_t1").await;

    let teacher_block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(fraud_teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "teacher fraud".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let fraud_course = create_course(&mut conn, &unique_string("fraud_course")).await;

    let course_block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(fraud_course.id),
            reward_policy_id: None,
            reason: "course fraud".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let (_blocks, total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter::default(),
    )
    .await
    .unwrap();
    assert!(total >= 2);

    let (teacher_blocks, teacher_total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter {
            scope_type: Some(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(teacher_total >= 1);
    assert!(teacher_blocks
        .iter()
        .any(|b| b.id == teacher_block.id));
    assert!(!teacher_blocks
        .iter()
        .any(|b| b.id == course_block.id));
}

#[actix_web::test]
async fn test_fraud_block_revoke() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_revoke_creator").await;
    let revoker = create_user_helper(&mut conn, "fraud_revoker").await;
    let teacher = create_user_helper(&mut conn, "fraud_revoke_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "to revoke".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    assert!(block.revoked_at.is_none());

    let now = Utc::now();
    let revoked = reward_fraud_block_repository::revoke_reward_fraud_block(
        &mut conn, block.id, revoker.id(), now,
    )
    .await
    .unwrap();

    assert!(revoked.revoked_at.is_some());
    assert_eq!(revoked.revoked_by_user_id, Some(revoker.id()));
}

#[actix_web::test]
async fn test_fraud_block_active_filter() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_active_creator").await;
    let teacher = create_user_helper(&mut conn, "fraud_active_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None, course_id: None, reward_policy_id: None,
            reason: "active filter test".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let (active_blocks, _) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter { active: Some(true), ..Default::default() },
    )
    .await
    .unwrap();
    assert!(active_blocks.iter().any(|b| b.id == block.id));

    reward_fraud_block_repository::revoke_reward_fraud_block(
        &mut conn, block.id, creator.id(), Utc::now(),
    )
    .await
    .unwrap();

    let (active_after, _) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter { active: Some(true), ..Default::default() },
    )
    .await
    .unwrap();
    assert!(!active_after.iter().any(|b| b.id == block.id));
}

// ── reward_execution_job_repository ──

#[actix_web::test]
async fn test_execution_job_enqueue_and_find() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "exec_student").await;
    let course = create_course(&mut conn, &unique_string("exec_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("exec_cand")),
    )
    .await
    .unwrap();

    let job = reward_execution_job_repository::enqueue_reward_execution_job(
        &mut conn, candidate.id,
    )
    .await
    .unwrap();

    assert_eq!(job.reward_candidate_id, candidate.id);
    assert_eq!(job.status, REWARD_EXECUTION_STATUS_QUEUED);

    let found = reward_execution_job_repository::find_job_by_candidate(
        &mut conn, candidate.id,
    )
    .await
    .unwrap()
    .expect("execution job not found");
    assert_eq!(found.id, job.id);
}

#[actix_web::test]
async fn test_execution_job_enqueue_is_idempotent() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "exec_idem_student").await;
    let course = create_course(&mut conn, &unique_string("exec_idem_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("exec_idem_cand")),
    )
    .await
    .unwrap();

    let job1 = reward_execution_job_repository::enqueue_reward_execution_job(
        &mut conn, candidate.id,
    )
    .await
    .unwrap();
    let job2 = reward_execution_job_repository::enqueue_reward_execution_job(
        &mut conn, candidate.id,
    )
    .await
    .unwrap();

    assert_eq!(job1.id, job2.id);
}

// ── reward_audit_event_repository ──

#[actix_web::test]
async fn test_audit_event_create_and_list() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "audit_student").await;
    let course = create_course(&mut conn, &unique_string("audit_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("audit_cand")),
    )
    .await
    .unwrap();

    let event1 = reward_audit_event_repository::create_reward_audit_event(
        &mut conn,
        NewRewardAuditEvent {
            reward_candidate_id: candidate.id,
            actor_user_id: Some(student.id()),
            event_type: REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED.to_string(),
            from_status: None,
            to_status: candidate.status.clone(),
            reason: None,
            metadata: json!({}),
        },
    )
    .await
    .unwrap();

    let event2 = reward_audit_event_repository::create_reward_audit_event(
        &mut conn,
        NewRewardAuditEvent {
            reward_candidate_id: candidate.id,
            actor_user_id: Some(student.id()),
            event_type: "teacher_decision".to_string(),
            from_status: Some(candidate.status),
            to_status: "teacher_approved".to_string(),
            reason: Some("looks good".to_string()),
            metadata: json!({"source": "test"}),
        },
    )
    .await
    .unwrap();

    let events = reward_audit_event_repository::list_reward_audit_events(
        &mut conn, candidate.id,
    )
    .await
    .unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].id, event1.id);
    assert_eq!(events[1].id, event2.id);
}

// ── reward_policy_repository ──

#[actix_web::test]
async fn test_policy_create_and_list() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_course")).await;

    let policy = reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(50),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 3600,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(policy.event_type, REWARD_EVENT_COURSE_COMPLETION);
    assert_eq!(policy.course_id, Some(course.id));
    assert!(policy.active);

    let policies = reward_policy_repository::list_policies(
        &mut conn,
        reward_policy_repository::RewardPolicyFilter {
            course_id: Some(course.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(policies.len() >= 1);
    assert_eq!(policies[0].course_id, Some(course.id));
}

#[actix_web::test]
async fn test_policy_deactivate() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_deact_course")).await;

    let policy = reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(25),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        },
    )
    .await
    .unwrap();
    assert!(policy.active);

    let count = reward_policy_repository::deactivate_active_policies(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
        chrono::Utc::now(),
    )
    .await
    .unwrap();
    assert!(count >= 1);

    let policies = reward_policy_repository::list_policies(
        &mut conn,
        reward_policy_repository::RewardPolicyFilter {
            course_id: Some(course.id),
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(policies.len(), 0);
}

#[actix_web::test]
async fn test_policy_next_version() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_ver_course")).await;

    let v1 = reward_policy_repository::next_policy_version(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
    )
    .await
    .unwrap();
    assert_eq!(v1, 1);

    reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: v1,
            token_amount: BigDecimal::from(50),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 3600,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        },
    )
    .await
    .unwrap();

    let v2 = reward_policy_repository::next_policy_version(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
    )
    .await
    .unwrap();
    assert_eq!(v2, 2);
}

// ── Helpers ──

fn new_candidate(course_id: i32, student_user_id: i32, key: &str) -> NewRewardCandidate {
    NewRewardCandidate {
        course_id,
        student_user_id,
        submitter_user_id: student_user_id,
        source_scope: REWARD_SOURCE_COURSE.to_string(),
        source_organization_id: None,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: key.to_string(),
        evidence: json!({"completion_percentage": 100.0}),
        status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
    }
}
