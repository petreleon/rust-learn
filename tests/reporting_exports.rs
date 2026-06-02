use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::schema::{
    courses, courses_organizations, delegated_permissions, external_transactions,
    internal_transactions, organizations, reward_candidates, reward_execution_jobs,
    reward_fraud_blocks, reward_payout_records, reward_policies, reward_wallet_credit_records,
    teacher_applications, transactions, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::delegated_permission::{NewDelegatedPermission, DELEGATED_SCOPE_PLATFORM};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TOKEN_CONFIRMED,
};
use rust_learn::models::reward_fraud_block::{
    NewRewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{OrganizationRole, PlatformRole};
use rust_learn::models::teacher_application::{
    NewTeacherApplication, TEACHER_APPLICATION_SCOPE_PLATFORM, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::models::wallet::NewWallet;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::json;
use serde_json::Value;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = format!("{}@example.com", unique_string(name));
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn create_organization(conn: &mut AsyncPgConnection) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: unique_string("ReportingOrg"),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn create_course(conn: &mut AsyncPgConnection) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("ReportingCourse"),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn link_course_to_org(conn: &mut AsyncPgConnection, course_id: i32, organization_id: i32) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}

async fn create_org_wallet(conn: &mut AsyncPgConnection, organization_id: i32) {
    create_org_wallet_with_value(conn, organization_id, BigDecimal::from(0)).await;
}

async fn create_org_wallet_with_value(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    value: BigDecimal,
) -> i32 {
    diesel::insert_into(wallets::table)
        .values(NewWallet {
            user_id: None,
            organization_id: Some(organization_id),
            value,
        })
        .returning(wallets::id)
        .get_result(conn)
        .await
        .expect("failed to create organization wallet")
}

async fn create_teacher_application(conn: &mut AsyncPgConnection, applicant_user_id: i32) -> i64 {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: None,
            requested_course_id: None,
            experience_summary: "reporting dashboard applicant".to_string(),
            organization_sponsor_id: None,
            portfolio_links: json!([]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
        })
        .returning(teacher_applications::id)
        .get_result(conn)
        .await
        .expect("failed to create teacher application")
}

async fn create_sponsored_teacher_application(
    conn: &mut AsyncPgConnection,
    applicant_user_id: i32,
    organization_id: i32,
) -> i64 {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: Some(organization_id),
            requested_course_id: None,
            experience_summary: "organization reward dashboard applicant".to_string(),
            organization_sponsor_id: Some(organization_id),
            portfolio_links: json!([]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
        })
        .returning(teacher_applications::id)
        .get_result(conn)
        .await
        .expect("failed to create sponsored teacher application")
}

async fn create_reward_candidate_with_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
) -> i64 {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("report_reward_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set(reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))))
        .execute(conn)
        .await
        .expect("failed to set reward candidate amount");

    candidate_id
}

async fn create_failed_reward_execution_job(conn: &mut AsyncPgConnection, candidate_id: i64) {
    diesel::insert_into(reward_execution_jobs::table)
        .values((
            reward_execution_jobs::reward_candidate_id.eq(candidate_id),
            reward_execution_jobs::status.eq("failed"),
            reward_execution_jobs::attempts.eq(3),
            reward_execution_jobs::last_error.eq(Some("token transfer failed")),
        ))
        .execute(conn)
        .await
        .expect("failed to create failed reward execution job");
}

async fn mark_reward_approval_decisions(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    teacher_user_id: i32,
    amount_reviewer_user_id: i32,
) {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::teacher_approver_user_id.eq(Some(teacher_user_id)),
            reward_candidates::teacher_decision_reason.eq(Some("course reward approved")),
            reward_candidates::teacher_decided_at.eq(Some(Utc::now())),
            reward_candidates::amount_reviewer_user_id.eq(Some(amount_reviewer_user_id)),
            reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))),
            reward_candidates::amount_decision_reason.eq(Some("platform amount approved")),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
            reward_candidates::updated_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await
        .expect("failed to mark reward approval decisions");
}

async fn create_reward_payout_export_records(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    amount: BigDecimal,
) -> (i64, i64, String) {
    let payout_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create payout transaction");
    let transaction_hash = unique_string("platform_export_tx");
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount),
            external_transactions::blockchain_address.eq("0xexportstudent"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xexportcontract")),
            external_transactions::transaction_hash.eq(Some(transaction_hash.clone())),
            external_transactions::log_index.eq(Some(7_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xexporttreasury")),
            external_transactions::to_address.eq(Some("0xexportstudent")),
        ))
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create external transaction");
    let payout_record_id = diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .returning(reward_payout_records::id)
        .get_result(conn)
        .await
        .expect("failed to create reward payout record");

    (payout_record_id, external_transaction_id, transaction_hash)
}

async fn create_reward_wallet_credit_export_record(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    wallet_id: i32,
    amount: BigDecimal,
) -> i64 {
    let wallet_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id: i64 = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet_id),
            internal_transactions::amount.eq(amount),
        ))
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create internal transaction");

    diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet_id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(conn)
        .await
        .expect("failed to create reward wallet credit record")
}

async fn create_platform_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
    grantee_user_id: i32,
) -> i64 {
    diesel::insert_into(delegated_permissions::table)
        .values(NewDelegatedPermission {
            grantor_user_id,
            grantee_user_id,
            permission: Permissions::APPROVE_REWARD_AMOUNT.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("export delegated permission activity".to_string()),
            expires_at: None,
        })
        .returning(delegated_permissions::id)
        .get_result(conn)
        .await
        .expect("failed to create delegated permission")
}

async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    creator_user_id: i32,
) -> i64 {
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
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: Some(creator_user_id),
        })
        .returning(reward_policies::id)
        .get_result(conn)
        .await
        .expect("failed to create reward policy")
}

async fn create_fraud_block(
    conn: &mut AsyncPgConnection,
    created_by_user_id: i32,
    scope_type: &str,
    teacher_user_id: Option<i32>,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    reward_policy_id: Option<i64>,
    reason: &str,
    expires_at: Option<chrono::DateTime<Utc>>,
) -> i64 {
    diesel::insert_into(reward_fraud_blocks::table)
        .values(NewRewardFraudBlock {
            scope_type: scope_type.to_string(),
            teacher_user_id,
            organization_id,
            course_id,
            reward_policy_id,
            reason: reason.to_string(),
            evidence_reference: Some(format!("case://{}", unique_string("fraud_report"))),
            created_by_user_id,
            expires_at,
        })
        .returning(reward_fraud_blocks::id)
        .get_result(conn)
        .await
        .expect("failed to create fraud block")
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn platform_admin_can_read_and_export_platform_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_platform_admin").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/reports/platform/summary")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["total_users"].as_i64().unwrap_or_default() >= 1);
    assert!(body["total_organizations"].as_i64().unwrap_or_default() >= 1);
    assert!(body["total_courses"].as_i64().unwrap_or_default() >= 1);

    let req = test::TestRequest::get()
        .uri("/reports/platform/summary.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let csv = String::from_utf8(test::read_body(resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("metric,value"));
    assert!(csv.contains("courses,"));
}

#[actix_web::test]
async fn platform_reward_dashboard_reports_actionable_reward_audit_work() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_reward_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_reward_moderator").await;
    let stranger = create_test_user(&mut conn, "report_reward_stranger").await;
    let applicant = create_test_user(&mut conn, "report_reward_applicant").await;
    let student = create_test_user(&mut conn, "report_reward_student").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let course = create_course(&mut conn).await;
    create_teacher_application(&mut conn, applicant.id()).await;
    let pending_amount_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_TEACHER_APPROVED,
    )
    .await;
    let failed_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    create_failed_reward_execution_job(&mut conn, failed_candidate).await;
    let mismatch_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(
        body["teacher_applications"]["submitted"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["reward_candidates"]["teacher_approved"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["pending_amount_approvals"]
        .as_array()
        .expect("pending amount rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == pending_amount_candidate));
    assert!(
        body["pending_amount_approval_count"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["payout_failure_count"].as_i64().unwrap_or_default() >= 1);
    assert!(body["payout_failures"]
        .as_array()
        .expect("payout failure rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == failed_candidate));
    assert!(
        body["reconciliation_mismatch_count"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["reconciliation_mismatches"]
        .as_array()
        .expect("reconciliation mismatch rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == mismatch_candidate
            && row["mismatch_type"] == "needs_payout_record"));

    let denied_export_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let denied_export_status = match app.call(denied_export_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(denied_export_status, StatusCode::FORBIDDEN);

    let export_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("teacher_applications,submitted,"));
    assert!(csv.contains("pending_amount_approvals,"));
    assert!(csv.contains("payout_failures,"));
    assert!(csv.contains("reconciliation_mismatches,"));
    assert!(csv.contains("needs_payout_record"));
}

#[actix_web::test]
async fn platform_csv_exports_cover_business_reward_datasets() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_export_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_export_moderator").await;
    let applicant = create_test_user(&mut conn, "report_export_applicant").await;
    let student = create_test_user(&mut conn, "report_export_student").await;
    let submitter = create_test_user(&mut conn, "report_export_submitter").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let org = create_organization(&mut conn).await;
    let wallet_id = create_org_wallet_with_value(&mut conn, org.id, BigDecimal::from(10)).await;
    let course = create_course(&mut conn).await;
    let application_id = create_teacher_application(&mut conn, applicant.id()).await;
    let candidate_id = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    mark_reward_approval_decisions(&mut conn, candidate_id, submitter.id(), platform_admin.id())
        .await;
    let (payout_record_id, external_transaction_id, transaction_hash) =
        create_reward_payout_export_records(&mut conn, candidate_id, BigDecimal::from(10)).await;
    let wallet_credit_record_id = create_reward_wallet_credit_export_record(
        &mut conn,
        candidate_id,
        wallet_id,
        BigDecimal::from(10),
    )
    .await;
    let delegation_id =
        create_platform_delegated_permission(&mut conn, platform_admin.id(), submitter.id()).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    for path in [
        "/reports/platform/teacher-applications.csv",
        "/reports/platform/reward-approvals.csv",
        "/reports/platform/token-payouts.csv",
        "/reports/platform/wallet-credits.csv",
        "/reports/platform/delegated-permissions.csv",
    ] {
        let denied_req = test::TestRequest::get()
            .uri(path)
            .insert_header((
                "Authorization",
                format!("Bearer {}", token_for(platform_moderator.id())),
            ))
            .to_request();
        let denied_status = match app.call(denied_req).await {
            Ok(resp) => resp.status(),
            Err(err) => err.error_response().status(),
        };
        assert_eq!(denied_status, StatusCode::FORBIDDEN);
    }

    let teacher_applications_req = test::TestRequest::get()
        .uri("/reports/platform/teacher-applications.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let teacher_applications_resp = test::call_service(&app, teacher_applications_req).await;
    assert_eq!(teacher_applications_resp.status(), StatusCode::OK);
    let teacher_applications_csv =
        String::from_utf8(test::read_body(teacher_applications_resp).await.to_vec())
            .expect("valid teacher applications csv");
    assert!(teacher_applications_csv.starts_with("application_id,applicant_user_id"));
    assert!(teacher_applications_csv.contains(&format!("{application_id},")));

    let reward_approvals_req = test::TestRequest::get()
        .uri("/reports/platform/reward-approvals.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let reward_approvals_resp = test::call_service(&app, reward_approvals_req).await;
    assert_eq!(reward_approvals_resp.status(), StatusCode::OK);
    let reward_approvals_csv =
        String::from_utf8(test::read_body(reward_approvals_resp).await.to_vec())
            .expect("valid reward approvals csv");
    assert!(reward_approvals_csv.starts_with("reward_candidate_id,course_id"));
    assert!(reward_approvals_csv.contains(&format!("{candidate_id},")));
    assert!(reward_approvals_csv.contains("course reward approved"));
    assert!(reward_approvals_csv.contains("platform amount approved"));

    let token_payouts_req = test::TestRequest::get()
        .uri("/reports/platform/token-payouts.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let token_payouts_resp = test::call_service(&app, token_payouts_req).await;
    assert_eq!(token_payouts_resp.status(), StatusCode::OK);
    let token_payouts_csv = String::from_utf8(test::read_body(token_payouts_resp).await.to_vec())
        .expect("valid token payouts csv");
    assert!(token_payouts_csv.starts_with("reward_payout_record_id,reward_candidate_id"));
    assert!(token_payouts_csv.contains(&format!("{payout_record_id},{candidate_id},")));
    assert!(token_payouts_csv.contains(&external_transaction_id.to_string()));
    assert!(token_payouts_csv.contains(&transaction_hash));

    let wallet_credits_req = test::TestRequest::get()
        .uri("/reports/platform/wallet-credits.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let wallet_credits_resp = test::call_service(&app, wallet_credits_req).await;
    assert_eq!(wallet_credits_resp.status(), StatusCode::OK);
    let wallet_credits_csv = String::from_utf8(test::read_body(wallet_credits_resp).await.to_vec())
        .expect("valid wallet credits csv");
    assert!(wallet_credits_csv.starts_with("reward_wallet_credit_record_id,reward_candidate_id"));
    assert!(wallet_credits_csv.contains(&format!("{wallet_credit_record_id},{candidate_id},")));
    assert!(wallet_credits_csv.contains(&format!(",{wallet_id},")));

    let delegated_permissions_req = test::TestRequest::get()
        .uri("/reports/platform/delegated-permissions.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let delegated_permissions_resp = test::call_service(&app, delegated_permissions_req).await;
    assert_eq!(delegated_permissions_resp.status(), StatusCode::OK);
    let delegated_permissions_csv =
        String::from_utf8(test::read_body(delegated_permissions_resp).await.to_vec())
            .expect("valid delegated permissions csv");
    assert!(delegated_permissions_csv.starts_with("delegated_permission_id,grantor_user_id"));
    assert!(delegated_permissions_csv.contains(&format!("{delegation_id},")));
    assert!(delegated_permissions_csv.contains("APPROVE_REWARD_AMOUNT"));
    assert!(delegated_permissions_csv.contains(",active,"));
}

#[actix_web::test]
async fn platform_fraud_dashboard_reports_active_blocks_by_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_fraud_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_fraud_moderator").await;
    let stranger = create_test_user(&mut conn, "report_fraud_stranger").await;
    let teacher = create_test_user(&mut conn, "report_fraud_teacher").await;
    let expired_teacher = create_test_user(&mut conn, "report_fraud_expired_teacher").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    let policy_id = create_course_reward_policy(&mut conn, course.id, platform_admin.id()).await;

    let teacher_block_id = create_fraud_block(
        &mut conn,
        platform_admin.id(),
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
        Some(teacher.id()),
        None,
        None,
        None,
        "teacher approvals paused for review",
        None,
    )
    .await;
    let organization_block_id = create_fraud_block(
        &mut conn,
        platform_admin.id(),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
        None,
        Some(org.id),
        None,
        None,
        "organization reward activity paused",
        None,
    )
    .await;
    let course_block_id = create_fraud_block(
        &mut conn,
        platform_admin.id(),
        REWARD_FRAUD_BLOCK_SCOPE_COURSE,
        None,
        None,
        Some(course.id),
        None,
        "course rewards under review",
        None,
    )
    .await;
    let policy_block_id = create_fraud_block(
        &mut conn,
        platform_admin.id(),
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
        None,
        None,
        None,
        Some(policy_id),
        "policy payouts paused",
        None,
    )
    .await;
    let expired_block_id = create_fraud_block(
        &mut conn,
        platform_admin.id(),
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
        Some(expired_teacher.id()),
        None,
        None,
        None,
        "expired teacher block",
        Some(Utc::now() - Duration::minutes(5)),
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["active_total"].as_i64().unwrap_or_default() >= 4);
    assert!(
        body["active_by_scope"]["teacher"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["organization"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["course"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["reward_policy"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );

    let active_blocks = body["active_blocks"]
        .as_array()
        .expect("active fraud block rows");
    for block_id in [
        teacher_block_id,
        organization_block_id,
        course_block_id,
        policy_block_id,
    ] {
        assert!(
            active_blocks
                .iter()
                .any(|row| row["id"].as_i64() == Some(block_id)),
            "active fraud dashboard should include block {block_id}"
        );
    }
    assert!(
        !active_blocks
            .iter()
            .any(|row| row["id"].as_i64() == Some(expired_block_id)),
        "expired fraud block must not appear in active fraud dashboard"
    );

    let denied_export_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let denied_export_status = match app.call(denied_export_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(denied_export_status, StatusCode::FORBIDDEN);

    let export_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("fraud_blocks,active_total,"));
    assert!(csv.contains("active_fraud_blocks,"));
    assert!(csv.contains("teacher approvals paused for review"));
    assert!(!csv.contains("expired teacher block"));
}

#[actix_web::test]
async fn organization_reward_dashboard_reports_sponsored_rewards_and_wallets() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "report_org_reward_admin").await;
    let stranger = create_test_user(&mut conn, "report_org_reward_stranger").await;
    let applicant = create_test_user(&mut conn, "report_org_reward_applicant").await;
    let student = create_test_user(&mut conn, "report_org_reward_student").await;
    let submitter = create_test_user(&mut conn, "report_org_reward_submitter").await;
    let org = create_organization(&mut conn).await;
    let other_org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    let other_course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    link_course_to_org(&mut conn, other_course.id, other_org.id).await;
    create_org_wallet_with_value(&mut conn, org.id, BigDecimal::from(77)).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    create_sponsored_teacher_application(&mut conn, applicant.id(), org.id).await;
    let _first_reward = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    let _second_reward = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
    )
    .await;
    let _unrelated_reward = create_reward_candidate_with_status(
        &mut conn,
        other_course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert!(
        body["sponsored_teacher_applications"]["submitted"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["course_reward_count"].as_i64().unwrap_or_default() >= 2);
    assert!(body["approved_reward_count"].as_i64().unwrap_or_default() >= 2);
    assert_eq!(body["approved_amount_total"], "20");
    assert_eq!(body["wallet_balance_total"], "77");
    assert!(body["courses"]
        .as_array()
        .expect("organization reward course rows")
        .iter()
        .any(
            |row| row["course_id"].as_i64() == Some(i64::from(course.id))
                && row["reward_candidate_count"].as_i64().unwrap_or_default() >= 2
                && row["approved_amount_total"] == "20"
        ));
    assert!(
        !body["courses"]
            .as_array()
            .expect("organization reward course rows")
            .iter()
            .any(|row| row["course_id"].as_i64() == Some(i64::from(other_course.id))),
        "organization dashboard must not include another organization's course rewards"
    );

    let export_req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard.csv",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("teacher_applications,submitted,"));
    assert!(csv.contains("reward_candidates,approved_amount_total,20"));
    assert!(csv.contains("wallets,balance_total,77"));
    assert!(csv.contains("courses,"));
}

#[actix_web::test]
async fn organization_admin_can_read_and_export_org_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "report_org_admin").await;
    let stranger = create_test_user(&mut conn, "report_stranger").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    create_org_wallet(&mut conn, org.id).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert_eq!(body["course_count"].as_i64(), Some(1));
    assert_eq!(body["member_count"].as_i64(), Some(1));
    assert_eq!(body["wallet_count"].as_i64(), Some(1));

    let req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary.csv", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let csv = String::from_utf8(test::read_body(resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("metric,value"));
    assert!(csv.contains("organization_name,"));
    assert!(csv.contains("courses,1"));
}
