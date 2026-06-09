use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    chapters, contents, course_join_requests, courses, courses_organizations, organizations,
    reward_candidates, reward_policies,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::NewChapter;
use rust_learn::models::content::NewContent;
use rust_learn::models::course::{Course, NewCourse, COURSE_STATUS_PUBLISHED};
use rust_learn::models::course_join_request::{
    NewCourseJoinRequest, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::{
    NewRewardCandidate, REWARD_SOURCE_COURSE, REWARD_STATUS_FAILED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{CourseRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_platform::UserRolePlatform;
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
    create_user(
        conn,
        &format!("{} User", prefix),
        &format!("{}@example.com", unique_string(prefix)),
        Some(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap()),
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn publish_course(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(COURSE_STATUS_PUBLISHED))
        .execute(conn)
        .await
        .expect("failed to publish course");
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

async fn link_course_to_org(conn: &mut AsyncPgConnection, course_id: i32, organization_id: i32) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course and organization");
}

async fn create_chapter(conn: &mut AsyncPgConnection, course_id: i32, title: &str) -> i32 {
    diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id,
            title: title.to_string(),
            order: 0,
        })
        .returning(chapters::id)
        .get_result(conn)
        .await
        .expect("failed to create chapter")
}

async fn create_content(conn: &mut AsyncPgConnection, chapter_id: i32, content_type: &str) {
    diesel::insert_into(contents::table)
        .values(NewContent {
            chapter_id,
            content_type: content_type.to_string(),
            data: None,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to create content");
}

async fn create_join_request(
    conn: &mut AsyncPgConnection,
    requester_user_id: i32,
    course_id: i32,
    status: &str,
) {
    diesel::insert_into(course_join_requests::table)
        .values(NewCourseJoinRequest {
            course_id,
            requester_user_id,
            status: status.to_string(),
        })
        .execute(conn)
        .await
        .expect("failed to create join request");
}

async fn create_reward_policy(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            active: true,
            cooldown_seconds: 0,
            course_id: Some(course_id),
            created_by_user_id: None,
            event_type: "course_completion".to_string(),
            max_payout: None,
            multiplier: BigDecimal::from(1),
            organization_id: None,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            token_amount: BigDecimal::from(25),
            version: 1,
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
) {
    diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            event_type: "course_completion".to_string(),
            evidence: json!({ "source": "teacher dashboard test" }),
            idempotency_key: unique_string("teacher_dashboard_reward"),
            source_organization_id: None,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            status: status.to_string(),
            student_user_id,
            submitter_user_id,
        })
        .execute(conn)
        .await
        .expect("failed to create reward candidate");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn teacher_course_dashboard_returns_scoped_course_health_and_queues() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let teacher = create_test_user(&mut conn, "teacher_dashboard_teacher").await;
    let student = create_test_user(&mut conn, "teacher_dashboard_student").await;
    let pending_learner = create_test_user(&mut conn, "teacher_dashboard_pending").await;
    let outsider = create_test_user(&mut conn, "teacher_dashboard_outsider").await;
    assign_platform_role(&mut conn, outsider.id(), "USER").await;

    let org = create_organization(&mut conn, &unique_string("TeacherDashboardOrg")).await;
    let course = create_course(&mut conn, &unique_string("TeacherDashboardCourse")).await;
    let hidden_course = create_course(&mut conn, &unique_string("HiddenTeacherCourse")).await;
    publish_course(&mut conn, course.id).await;
    publish_course(&mut conn, hidden_course.id).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let chapter_id = create_chapter(&mut conn, course.id, "Dashboard chapter").await;
    create_content(&mut conn, chapter_id, "article").await;
    create_reward_policy(&mut conn, course.id).await;
    create_join_request(
        &mut conn,
        pending_learner.id(),
        course.id,
        COURSE_JOIN_STATUS_PENDING,
    )
    .await;
    create_join_request(
        &mut conn,
        outsider.id(),
        course.id,
        COURSE_JOIN_STATUS_WAITLISTED,
    )
    .await;
    create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        teacher.id(),
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    )
    .await;
    create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        teacher.id(),
        REWARD_STATUS_TEACHER_APPROVED,
    )
    .await;
    create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        teacher.id(),
        REWARD_STATUS_FAILED,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/courses/teaching?limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(1));
    let dashboard_course = &body["courses"][0];
    assert_eq!(dashboard_course["id"].as_i64(), Some(i64::from(course.id)));
    assert_eq!(
        dashboard_course["organizations"][0]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(
        dashboard_course["lifecycle_status"].as_str(),
        Some(COURSE_STATUS_PUBLISHED)
    );
    assert_eq!(
        dashboard_course["content"]["content_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["rewards"]["active_policy_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["enrolled_student_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["pending_join_request_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["waitlisted_join_request_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["pending_teacher_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["teacher_approved_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["failed_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["permissions"]["can_manage_settings"].as_bool(),
        Some(true)
    );
    assert_eq!(
        dashboard_course["permissions"]["can_approve_reward_candidates"].as_bool(),
        Some(true)
    );

    let outsider_req = test::TestRequest::get()
        .uri("/courses/teaching?limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let outsider_resp = test::call_service(&app, outsider_req).await;
    assert_eq!(outsider_resp.status(), StatusCode::OK);
    let outsider_body: Value = test::read_body_json(outsider_resp).await;
    assert_eq!(outsider_body["total"].as_i64(), Some(0));
    assert!(outsider_body["courses"]
        .as_array()
        .expect("courses array")
        .is_empty());
}
