use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{courses, courses_organizations, organizations};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
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

async fn create_test_user(conn: &mut AsyncPgConnection) -> User {
    let email = format!("{}@example.com", unique_string("course_discovery"));
    create_user(
        conn,
        "Course Discovery",
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
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

async fn link_course_to_org(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
    order: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order,
        })
        .execute(conn)
        .await
        .expect("failed to link course and organization");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn course_list_supports_search_org_filter_and_pagination() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let viewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, viewer.id(), "STUDENT").await;

    let org = create_organization(&mut conn, &unique_string("DiscoveryOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherOrg")).await;
    let prefix = unique_string("DiscoveryCourse");
    let first = create_course(&mut conn, &format!("{} Rust Patterns", prefix)).await;
    let second = create_course(&mut conn, &format!("{} Rust Services", prefix)).await;
    let excluded = create_course(&mut conn, &format!("{} TypeScript", prefix)).await;
    let other_org_course = create_course(&mut conn, &format!("{} Rust External", prefix)).await;

    link_course_to_org(&mut conn, first.id, org.id, 0).await;
    link_course_to_org(&mut conn, second.id, org.id, 1).await;
    link_course_to_org(&mut conn, excluded.id, org.id, 2).await;
    link_course_to_org(&mut conn, other_org_course.id, other_org.id, 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=rust&organization_id={}&limit=1&offset=1",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(viewer.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(2));
    assert_eq!(body["limit"].as_i64(), Some(1));
    assert_eq!(body["offset"].as_i64(), Some(1));
    assert_eq!(body["search"].as_str(), Some("rust"));
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert_eq!(body["courses"].as_array().expect("courses array").len(), 1);
    assert_eq!(
        body["courses"][0]["title"].as_str(),
        Some(second.title.as_str())
    );
}
