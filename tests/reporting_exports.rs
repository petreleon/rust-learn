use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    courses, courses_organizations, organizations, reward_candidates, reward_execution_jobs,
    teacher_applications, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TOKEN_CONFIRMED,
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
    diesel::insert_into(wallets::table)
        .values(NewWallet {
            user_id: None,
            organization_id: Some(organization_id),
            value: BigDecimal::from(0),
        })
        .execute(conn)
        .await
        .expect("failed to create organization wallet");
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
