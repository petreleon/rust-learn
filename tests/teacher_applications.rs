use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::organizations;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::teacher_application_repository::list_audit_events;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::teacher_application_service::{
    decide_application, list_applications, nominate_application, submit_application,
    ListTeacherApplicationsRequest, OrganizationTeacherNominationRequest,
    SubmitTeacherApplicationRequest, TeacherApplicationDecisionRequest, TeacherApplicationError,
};

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

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    let new_org = NewOrganization {
        name: name.to_string(),
        website_link: None,
        profile_url: None,
    };

    diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role not found");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to force assign organization role");
}

fn platform_application_request() -> SubmitTeacherApplicationRequest {
    SubmitTeacherApplicationRequest {
        requested_scope: "platform".to_string(),
        requested_organization_id: None,
        requested_course_id: None,
        experience_summary: "Five years teaching Rust and mentoring students.".to_string(),
        organization_sponsor_id: None,
        portfolio_links: Some(vec![
            "https://example.com/portfolio".to_string(),
            "   ".to_string(),
        ]),
    }
}

#[actix_web::test]
async fn user_with_submit_permission_can_apply_and_without_permission_cannot() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_user").await;
    let no_permission_user = create_user_helper(&mut conn, "teacher_apply_denied").await;

    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");
    assert_eq!(application.applicant_user_id, applicant.id());
    assert_eq!(application.status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(
        application.portfolio_links,
        serde_json::json!(["https://example.com/portfolio"])
    );

    let denied = submit_application(
        &mut conn,
        no_permission_user.id(),
        platform_application_request(),
    )
    .await
    .expect_err("user without submit permission should be denied");
    assert!(matches!(
        denied,
        TeacherApplicationError::PermissionDenied(_)
    ));
}

#[actix_web::test]
async fn organization_admin_can_nominate_teacher_to_central_queue() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("teacher_sponsor")).await;
    let nominator = create_user_helper(&mut conn, "teacher_nominator").await;
    let applicant = create_user_helper(&mut conn, "teacher_nominee").await;
    force_assign_organization_role(&mut conn, nominator.id(), organization.id, "ADMIN").await;

    let application = nominate_application(
        &mut conn,
        nominator.id(),
        organization.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Organization-sponsored instructor candidate.".to_string(),
            portfolio_links: None,
        },
    )
    .await
    .expect("organization admin should nominate teacher");

    assert_eq!(application.applicant_user_id, applicant.id());
    assert_eq!(application.organization_sponsor_id, Some(organization.id));
    assert_eq!(application.requested_organization_id, Some(organization.id));

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event_type, "organization_nominated");
}

#[actix_web::test]
async fn platform_admin_reviews_and_approves_while_moderator_is_denied() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_review_applicant").await;
    let admin = create_user_helper(&mut conn, "teacher_review_admin").await;
    let moderator = create_user_helper(&mut conn, "teacher_review_moderator").await;

    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    assign_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let denied = decide_application(
        &mut conn,
        moderator.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("moderators do not review teacher applications".to_string()),
        },
    )
    .await
    .expect_err("moderator should not approve teacher applications by role name");
    assert!(matches!(
        denied,
        TeacherApplicationError::PermissionDenied(_)
    ));

    let pending = list_applications(
        &mut conn,
        admin.id(),
        ListTeacherApplicationsRequest {
            status: Some(TEACHER_APPLICATION_STATUS_SUBMITTED.to_string()),
            ..Default::default()
        },
    )
    .await
    .expect("admin should list teacher applications");
    assert!(pending.iter().any(|item| item.id == application.id));

    let approved = decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("candidate approved by central administration".to_string()),
        },
    )
    .await
    .expect("admin should approve teacher application");
    assert_eq!(approved.status, TEACHER_APPLICATION_STATUS_APPROVED);
    assert_eq!(approved.reviewer_id, Some(admin.id()));

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].to_status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(audit[1].to_status, TEACHER_APPLICATION_STATUS_APPROVED);
}
