use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    courses, courses_organizations, organizations, reward_candidates, teacher_applications, wallets,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{
    Course, NewCourse, COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED,
};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_EVENT_COURSE_COMPLETION, REWARD_SOURCE_COURSE,
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_FAILED,
};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::teacher_application::{
    NewTeacherApplication, TEACHER_APPLICATION_SCOPE_PLATFORM, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::wallet::NewWallet;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    create_user(
        conn,
        &format!("{} User", prefix.replace('_', " ")),
        &email,
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
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

async fn create_course(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    title: &str,
    lifecycle_status: &str,
) -> Course {
    let course = diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result::<Course>(conn)
        .await
        .expect("failed to create course");

    diesel::update(courses::table.find(course.id))
        .set(courses::lifecycle_status.eq(lifecycle_status))
        .execute(conn)
        .await
        .expect("failed to set course lifecycle status");

    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id: course.id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");

    courses::table
        .find(course.id)
        .get_result(conn)
        .await
        .expect("failed to reload course")
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

async fn create_sponsored_teacher_application(
    conn: &mut AsyncPgConnection,
    applicant_user_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: None,
            requested_course_id: None,
            experience_summary: "Organization-sponsored Rust teaching candidate.".to_string(),
            organization_sponsor_id: Some(organization_id),
            portfolio_links: json!(["https://example.test/portfolio"]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
            idempotency_key: Some(unique_string("dashboard_teacher_application")),
        })
        .execute(conn)
        .await
        .expect("failed to create sponsored teacher application");
}

async fn create_org_wallet(conn: &mut AsyncPgConnection, organization_id: i32, value: BigDecimal) {
    diesel::insert_into(wallets::table)
        .values(NewWallet {
            user_id: None,
            organization_id: Some(organization_id),
            value,
        })
        .execute(conn)
        .await
        .expect("failed to create organization wallet");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
) {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: Some(organization_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("dashboard_reward_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    if let Some(amount) = approved_amount {
        diesel::update(reward_candidates::table.find(candidate_id))
            .set(reward_candidates::approved_amount.eq(Some(amount)))
            .execute(conn)
            .await
            .expect("failed to set approved amount");
    }
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn alert_kind_exists(body: &Value, expected: &str) -> bool {
    body["alerts"]
        .as_array()
        .map(|alerts| {
            alerts
                .iter()
                .any(|alert| alert["kind"].as_str() == Some(expected))
        })
        .unwrap_or(false)
}

fn missing_permission_exists(body: &Value, section: &str, expected: &str) -> bool {
    body[section]["missing_permissions"]
        .as_array()
        .map(|permissions| {
            permissions
                .iter()
                .any(|permission| permission.as_str() == Some(expected))
        })
        .unwrap_or(false)
}

#[actix_web::test]
async fn organization_dashboard_returns_scoped_operational_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardOrg")).await;
    let operator = create_test_user(&mut conn, "dashboard_operator").await;
    let learner = create_test_user(&mut conn, "dashboard_learner").await;
    let teacher = create_test_user(&mut conn, "dashboard_teacher").await;
    let applicant = create_test_user(&mut conn, "dashboard_applicant").await;
    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, learner.id(), org.id, "STUDENT").await;
    assign_organization_role(&mut conn, teacher.id(), org.id, "TEACHER").await;

    let published_course = create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardPublishedCourse"),
        COURSE_STATUS_PUBLISHED,
    )
    .await;
    create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardNeedsChangesCourse"),
        COURSE_STATUS_NEEDS_CHANGES,
    )
    .await;
    create_sponsored_teacher_application(&mut conn, applicant.id(), org.id).await;
    create_org_wallet(&mut conn, org.id, BigDecimal::from(125)).await;
    create_reward_candidate(
        &mut conn,
        published_course.id,
        org.id,
        learner.id(),
        teacher.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(40)),
    )
    .await;
    create_reward_candidate(
        &mut conn,
        published_course.id,
        org.id,
        learner.id(),
        teacher.id(),
        REWARD_STATUS_FAILED,
        None,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(operator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["organization"]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(body["health"]["status"].as_str(), Some("attention"));
    assert_eq!(body["members"]["available"].as_bool(), Some(true));
    assert_eq!(body["members"]["total"].as_i64(), Some(3));
    assert_eq!(body["courses"]["total"].as_i64(), Some(2));
    assert_eq!(body["courses"]["published"].as_i64(), Some(1));
    assert_eq!(body["courses"]["needs_changes"].as_i64(), Some(1));
    assert_eq!(body["teacher_applications"]["submitted"].as_i64(), Some(1));
    assert_eq!(body["rewards"]["available"].as_bool(), Some(true));
    assert_eq!(body["rewards"]["reward_candidate_count"].as_i64(), Some(2));
    assert_eq!(body["rewards"]["approved_reward_count"].as_i64(), Some(1));
    assert_eq!(
        body["rewards"]["approved_amount_total"].as_str(),
        Some("40")
    );
    assert_eq!(body["rewards"]["failed_count"].as_i64(), Some(1));
    assert_eq!(body["wallet"]["available"].as_bool(), Some(true));
    assert_eq!(body["wallet"]["wallet_count"].as_i64(), Some(1));
    assert_eq!(body["wallet"]["balance_total"].as_str(), Some("125"));
    assert_eq!(
        body["operator_permissions"]["can_view_reports"].as_bool(),
        Some(true)
    );
    assert!(alert_kind_exists(&body, "teacher_applications_submitted"));
    assert!(alert_kind_exists(&body, "courses_need_changes"));
    assert!(alert_kind_exists(&body, "reward_reconciliation"));
}

#[actix_web::test]
async fn organization_dashboard_gates_sensitive_sections_for_basic_member() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardBasicOrg")).await;
    let student = create_test_user(&mut conn, "dashboard_basic_student").await;
    assign_organization_role(&mut conn, student.id(), org.id, "STUDENT").await;
    create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardBasicCourse"),
        COURSE_STATUS_PUBLISHED,
    )
    .await;
    create_org_wallet(&mut conn, org.id, BigDecimal::from(50)).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["members"]["available"].as_bool(), Some(true));
    assert_eq!(body["courses"]["available"].as_bool(), Some(true));
    assert_eq!(body["courses"]["published"].as_i64(), Some(1));
    assert_eq!(
        body["teacher_applications"]["available"].as_bool(),
        Some(false)
    );
    assert_eq!(body["rewards"]["available"].as_bool(), Some(false));
    assert_eq!(body["wallet"]["available"].as_bool(), Some(false));
    assert_eq!(
        body["operator_permissions"]["can_view_reports"].as_bool(),
        Some(false)
    );
    assert!(missing_permission_exists(
        &body,
        "rewards",
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert!(missing_permission_exists(
        &body,
        "teacher_applications",
        "VIEW_ORG_TEACHER_APPLICATIONS"
    ));
    assert!(missing_permission_exists(
        &body,
        "wallet",
        "MANAGE_ORG_WALLETS"
    ));
}

#[actix_web::test]
async fn organization_dashboard_denies_outsiders_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardDeniedOrg")).await;
    let outsider = create_test_user(&mut conn, "dashboard_outsider").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
