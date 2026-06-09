use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations, platform_roles, role_permission_platform};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::course_repository::user_permission_course_request;
use rust_learn::repositories::organization_repository::user_permission_organization_request;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::platform_repository::user_permission_platform_request;
use rust_learn::repositories::teacher_application_repository::list_audit_events;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::teacher_application_service::{
    decide_application, get_my_application, list_applications, list_platform_applications,
    nominate_application, submit_application, ListTeacherApplicationsRequest,
    OrganizationTeacherNominationRequest, PlatformTeacherApplicationsRequest,
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

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    let new_course = NewCourse {
        title: title.to_string(),
    };

    diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result(conn)
        .await
        .expect("failed to create course")
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

async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description.eq(Some(
                "test-only teacher application permission bundle".to_string(),
            )),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom platform role");

    for permission in permissions {
        diesel::insert_into(role_permission_platform::table)
            .values((
                role_permission_platform::platform_role_id.eq(Some(role_id)),
                role_permission_platform::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom platform role permission");
    }

    role_id
}

async fn assign_platform_role_id(conn: &mut AsyncPgConnection, user_id: i32, role_id: i32) {
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
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
        idempotency_key: None,
    }
}

#[actix_web::test]
async fn custom_platform_permissions_drive_teacher_application_flow() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "custom_teacher_apply_user").await;
    let reviewer = create_user_helper(&mut conn, "custom_teacher_reviewer").await;
    let review_only_user = create_user_helper(&mut conn, "custom_teacher_review_only").await;

    let submit_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_SUBMITTER"),
        &[Permissions::SUBMIT_TEACHER_APPLICATION],
    )
    .await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_APPROVER"),
        &[
            Permissions::REVIEW_TEACHER_APPLICATIONS,
            Permissions::APPROVE_TEACHER_APPLICATION,
        ],
    )
    .await;
    let review_only_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_REVIEW_ONLY"),
        &[Permissions::REVIEW_TEACHER_APPLICATIONS],
    )
    .await;
    assign_platform_role_id(&mut conn, applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;
    assign_platform_role_id(&mut conn, review_only_user.id(), review_only_role_id).await;

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("submit permission should create teacher application");

    let visible = list_applications(
        &mut conn,
        reviewer.id(),
        ListTeacherApplicationsRequest {
            applicant_user_id: Some(applicant.id()),
            ..Default::default()
        },
    )
    .await
    .expect("review permission should list teacher applications");
    assert!(visible.iter().any(|item| item.id == application.id));

    let denied_approval = decide_application(
        &mut conn,
        review_only_user.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("review-only user attempted approval".to_string()),
        },
    )
    .await
    .expect_err("review-only permission must not approve teacher applications");
    assert!(matches!(
        denied_approval,
        TeacherApplicationError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_TEACHER_APPLICATION.to_string()
    ));

    let approved = decide_application(
        &mut conn,
        reviewer.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("custom permission reviewer approved".to_string()),
        },
    )
    .await
    .expect("approve permission should approve teacher application");
    assert_eq!(approved.status, TEACHER_APPLICATION_STATUS_APPROVED);
    assert_eq!(approved.reviewer_id, Some(reviewer.id()));
}

#[actix_web::test]
async fn platform_teacher_application_review_contract_returns_context_and_filters() {
    let mut conn = setup_conn().await;
    let org = create_organization(&mut conn, &unique_string("platform_teacher_review_org")).await;
    let course = create_course(&mut conn, &unique_string("Platform Async Teaching Lab")).await;
    let search_marker = unique_string("platform_review_marker");
    let submitted_applicant = create_user_helper(&mut conn, "platform_review_submitted").await;
    let approved_applicant = create_user_helper(&mut conn, "platform_review_approved").await;
    let reviewer = create_user_helper(&mut conn, "platform_review_reviewer").await;
    let outsider = create_user_helper(&mut conn, "platform_review_outsider").await;

    let submit_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEW_SUBMITTER"),
        &[Permissions::SUBMIT_TEACHER_APPLICATION],
    )
    .await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEWER"),
        &[
            Permissions::REVIEW_TEACHER_APPLICATIONS,
            Permissions::APPROVE_TEACHER_APPLICATION,
            Permissions::REJECT_TEACHER_APPLICATION,
        ],
    )
    .await;
    assign_platform_role_id(&mut conn, submitted_applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, approved_applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;

    let submitted = submit_application(
        &mut conn,
        submitted_applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "course".to_string(),
            requested_organization_id: None,
            requested_course_id: Some(course.id),
            experience_summary: format!(
                "Async Rust mentor with production examples. {search_marker}"
            ),
            organization_sponsor_id: Some(org.id),
            portfolio_links: Some(vec!["https://example.test/async-mentor".to_string()]),
            idempotency_key: None,
        },
    )
    .await
    .expect("submitted fixture should be created");

    let approved = submit_application(
        &mut conn,
        approved_applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "organization".to_string(),
            requested_organization_id: Some(org.id),
            requested_course_id: None,
            experience_summary: "Organization teaching lead.".to_string(),
            organization_sponsor_id: Some(org.id),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("approved fixture should be created");
    decide_application(
        &mut conn,
        reviewer.id(),
        approved.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("experienced organization teacher".to_string()),
        },
    )
    .await
    .expect("reviewer should approve fixture");

    let response = list_platform_applications(
        &mut conn,
        reviewer.id(),
        PlatformTeacherApplicationsRequest {
            status: Some("submitted".to_string()),
            search: Some(search_marker.clone()),
            limit: Some(10),
            offset: Some(0),
        },
    )
    .await
    .expect("reviewer should load platform review queue");

    assert_eq!(response.total, 1);
    assert_eq!(response.limit, 10);
    assert_eq!(response.offset, 0);
    assert_eq!(response.status.as_deref(), Some("submitted"));
    assert_eq!(response.search.as_deref(), Some(search_marker.as_str()));
    assert!(response.summary.total >= 2);
    assert!(response.summary.submitted >= 1);
    assert!(response.summary.approved >= 1);
    assert!(response.operator_permissions.can_view_applications);
    assert!(response.operator_permissions.can_approve_applications);
    assert!(response.operator_permissions.can_reject_applications);
    assert!(response.operator_permissions.can_request_changes);

    let application = response
        .applications
        .get(0)
        .expect("filtered application should be returned");
    assert_eq!(application.id, submitted.id);
    assert_eq!(application.applicant.id, submitted_applicant.id());
    assert_eq!(application.applicant.name, submitted_applicant.name);
    assert_eq!(application.requested_scope, "course");
    assert_eq!(
        application
            .requested_course
            .as_ref()
            .map(|course| course.title.as_str()),
        Some(course.title.as_str())
    );
    assert_eq!(
        application
            .sponsor_organization
            .as_ref()
            .map(|organization| organization.name.as_str()),
        Some(org.name.as_str())
    );
    assert_eq!(
        application.portfolio_links,
        vec!["https://example.test/async-mentor".to_string()]
    );
    assert_eq!(application.audit.event_count, 1);
    assert_eq!(
        application.audit.latest_event_type.as_deref(),
        Some("submitted")
    );

    let denied = list_platform_applications(
        &mut conn,
        outsider.id(),
        PlatformTeacherApplicationsRequest::default(),
    )
    .await
    .expect_err("users without review permission cannot load platform queue");
    assert!(matches!(
        denied,
        TeacherApplicationError::PermissionDenied(permission)
            if permission == Permissions::REVIEW_TEACHER_APPLICATIONS.to_string()
    ));
}

#[actix_web::test]
async fn teacher_application_submission_is_idempotent_by_key() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_idempotent").await;
    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let idempotency_key = unique_string("teacher_application_submit");
    let mut request = platform_application_request();
    request.idempotency_key = Some(idempotency_key.clone());

    let application = submit_application(&mut conn, applicant.id(), request.clone())
        .await
        .expect("initial teacher application should be created");
    let duplicate = submit_application(&mut conn, applicant.id(), request)
        .await
        .expect("same idempotency key and payload should return existing application");
    assert_eq!(duplicate.id, application.id);
    assert_eq!(
        duplicate.idempotency_key.as_deref(),
        Some(idempotency_key.as_str())
    );

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(
        audit.len(),
        1,
        "idempotent replay should not add another submitted audit event"
    );

    let mut conflicting_request = platform_application_request();
    conflicting_request.idempotency_key = Some(idempotency_key);
    conflicting_request.experience_summary =
        "Different application payload for same retry key.".to_string();
    let denied = submit_application(&mut conn, applicant.id(), conflicting_request)
        .await
        .expect_err("idempotency key reuse for a different application should be rejected");
    assert!(matches!(
        denied,
        TeacherApplicationError::InvalidInput(message)
            if message.contains("idempotency key is already used")
    ));
}

#[actix_web::test]
async fn applicant_can_read_latest_application_snapshot_without_review_permission() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_snapshot").await;
    let stranger = create_user_helper(&mut conn, "teacher_apply_snapshot_empty").await;
    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let snapshot = get_my_application(&mut conn, applicant.id())
        .await
        .expect("applicant should read own application snapshot");
    assert_eq!(
        snapshot.application.as_ref().map(|item| item.id),
        Some(application.id)
    );
    assert_eq!(snapshot.audit_events.len(), 1);
    assert_eq!(snapshot.audit_events[0].application_id, application.id);
    assert_eq!(snapshot.audit_events[0].to_status, application.status);

    let empty_snapshot = get_my_application(&mut conn, stranger.id())
        .await
        .expect("authenticated user without application should get an empty snapshot");
    assert!(empty_snapshot.application.is_none());
    assert!(empty_snapshot.audit_events.is_empty());
}

#[actix_web::test]
async fn duplicate_open_teacher_application_submission_conflicts_without_retry_key() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_duplicate").await;
    let admin = create_user_helper(&mut conn, "teacher_apply_duplicate_admin").await;
    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let duplicate = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect_err("open application should block duplicate submission");
    assert!(matches!(
        duplicate,
        TeacherApplicationError::InvalidTransition(message)
            if message.contains("already exists with status submitted")
    ));

    decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("candidate can reapply with stronger portfolio".to_string()),
        },
    )
    .await
    .expect("admin should reject teacher application");

    let resubmitted = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("rejected application should allow a new submission");
    assert_ne!(resubmitted.id, application.id);
    assert_eq!(resubmitted.status, TEACHER_APPLICATION_STATUS_SUBMITTED);
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
    let platform_admin = create_user_helper(&mut conn, "teacher_nomination_reviewer").await;
    force_assign_organization_role(&mut conn, nominator.id(), organization.id, "ADMIN").await;
    assign_role_to_user(&mut conn, platform_admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

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
            idempotency_key: None,
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

    decide_application(
        &mut conn,
        platform_admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("approved for sponsored organization".to_string()),
        },
    )
    .await
    .expect("platform admin should approve organization nomination");

    let applicant_has_org_teacher_permission = user_permission_organization_request(
        &mut conn,
        applicant.id(),
        organization.id,
        &Permissions::GENERATE_REPORT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_has_org_teacher_permission,
        "approved organization-scope applicant should receive the organization teaching bundle"
    );
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

    let applicant_has_teacher_bundle_permission = user_permission_platform_request(
        &mut conn,
        applicant.id(),
        &Permissions::GENERATE_REPORT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_has_teacher_bundle_permission,
        "approved platform-scope applicant should receive the platform teaching bundle"
    );

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].to_status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(audit[1].to_status, TEACHER_APPLICATION_STATUS_APPROVED);
}

#[actix_web::test]
async fn course_scope_approval_assigns_course_teacher_permission_bundle() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("teacher_scope_course")).await;
    let applicant = create_user_helper(&mut conn, "teacher_course_applicant").await;
    let admin = create_user_helper(&mut conn, "teacher_course_admin").await;

    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let application = submit_application(
        &mut conn,
        applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "course".to_string(),
            requested_organization_id: None,
            requested_course_id: Some(course.id),
            experience_summary: "Course-specific Rust instructor.".to_string(),
            organization_sponsor_id: None,
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("course-scope application should be created");

    decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("approved for this course".to_string()),
        },
    )
    .await
    .expect("admin should approve course-scope teacher application");

    let applicant_can_manage_course_settings = user_permission_course_request(
        &mut conn,
        applicant.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_can_manage_course_settings,
        "approved course-scope applicant should receive the course teaching bundle"
    );
}
