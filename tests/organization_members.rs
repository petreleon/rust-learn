use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::organizations;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::repositories::delegated_permission_repository::create_delegated_permission;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::Value;
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

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}

#[actix_web::test]
async fn organization_member_list_returns_scoped_members_permissions_and_filters() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let operator = create_test_user(&mut conn, "member_operator").await;
    let delegated_student = create_test_user(&mut conn, "delegated_student").await;
    let teacher = create_test_user(&mut conn, "member_teacher").await;
    let other_member = create_test_user(&mut conn, "other_member").await;
    let org = create_organization(&mut conn, &unique_string("MemberOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherMemberOrg")).await;

    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, delegated_student.id(), org.id, "STUDENT").await;
    assign_organization_role(&mut conn, teacher.id(), org.id, "TEACHER").await;
    assign_organization_role(&mut conn, other_member.id(), other_org.id, "STUDENT").await;
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: operator.id(),
            grantee_user_id: delegated_student.id(),
            permission: "VIEW_ORG_REWARD_REPORTS".to_string(),
            scope_type: "organization".to_string(),
            organization_id: Some(org.id),
            course_id: None,
            reason: Some("Temporary report review".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create delegated permission");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/organizations/{}/members?search=delegated&permission=VIEW_ORG_REWARD_REPORTS&limit=10",
            org.id
        ))
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
    assert_eq!(body["total"].as_i64(), Some(1));
    assert_eq!(body["limit"].as_i64(), Some(10));
    assert_eq!(body["search"].as_str(), Some("delegated"));
    assert_eq!(body["permission"].as_str(), Some("VIEW_ORG_REWARD_REPORTS"));
    assert_eq!(
        body["operator_permissions"]["can_view_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_invite_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_manage_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_assign_roles"].as_bool(),
        Some(false)
    );

    let member = &body["members"][0];
    assert_eq!(
        member["name"].as_str(),
        Some(delegated_student.name.as_str())
    );
    assert_eq!(
        member["email"].as_str(),
        Some(delegated_student.email.as_str())
    );
    assert!(array_contains(&member["roles"], "STUDENT"));
    assert!(array_contains(
        &member["direct_permissions"],
        "VIEW_ORGANIZATION"
    ));
    assert!(array_contains(
        &member["delegated_permissions"],
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert!(array_contains(
        &member["effective_permissions"],
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert_eq!(member["delegated_permission_count"].as_u64(), Some(1));
}

#[actix_web::test]
async fn organization_member_list_denies_users_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "member_outsider").await;
    let member = create_test_user(&mut conn, "member_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberDeniedOrg")).await;
    assign_organization_role(&mut conn, member.id(), org.id, "STUDENT").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/members", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
