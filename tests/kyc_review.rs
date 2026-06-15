use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use rust_learn::application::kyc::{
    KycAuditQuery, KycAuditUseCase, KycDecisionCommand, KycError, KycReviewUseCase,
    KycSubmissionUseCase, SubmitKycCommand,
};
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::access_control::delegation::DELEGATED_SCOPE_PLATFORM;
use rust_learn::domain::kyc::audit::KycAuditEventType;
use rust_learn::domain::kyc::submission::{KYC_STATUS_REJECTED, KYC_STATUS_SUBMITTED};
use rust_learn::infra::postgres::access_control::delegated_permissions::create_delegated_permission;
use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::kyc::kyc_use_case::PostgresKycUseCase;
use rust_learn::infra::postgres::models::delegated_permission::NewDelegatedPermission;
use rust_learn::infra::postgres::models::user::User;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_pool() -> DbPool {
    let _ = dotenvy::dotenv();
    establish_connection()
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get().await.expect("failed to get DB connection")
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

fn kyc_command(user_id: i32, name: &str) -> SubmitKycCommand {
    SubmitKycCommand {
        country_code: "US".to_string(),
        document_last4: Some("1234".to_string()),
        document_type: "passport".to_string(),
        evidence_reference: Some("s3://kyc/evidence".to_string()),
        legal_name: name.to_string(),
        provider_reference: Some("provider-ref".to_string()),
        user_id,
    }
}

#[actix_web::test]
async fn kyc_submission_and_review_write_permission_scoped_audit_events() {
    let pool = setup_pool().await;
    let mut conn = setup_conn(&pool).await;
    let learner = create_user_helper(&mut conn, "kyc_audit_learner").await;
    let admin = create_user_helper(&mut conn, "kyc_audit_admin").await;
    let delegated_reviewer = create_user_helper(&mut conn, "kyc_audit_delegate").await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: admin.id(),
            grantee_user_id: delegated_reviewer.id(),
            permission: Permissions::REVIEW_KYC_SUBMISSIONS.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("KYC review delegation".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to delegate KYC review permission");
    drop(conn);

    let use_case = PostgresKycUseCase::new(pool.clone());
    let submitted = use_case
        .submit_my_kyc(kyc_command(learner.id(), "Learner User"))
        .await
        .expect("submission should persist")
        .submission
        .expect("submission returned");

    let denied = use_case
        .list_submission_audit(KycAuditQuery {
            reviewer_user_id: learner.id(),
            submission_id: submitted.id,
        })
        .await
        .expect_err("learner cannot read review audit");
    assert!(matches!(denied, KycError::PermissionDenied(_)));

    let audit = use_case
        .list_submission_audit(KycAuditQuery {
            reviewer_user_id: admin.id(),
            submission_id: submitted.id,
        })
        .await
        .expect("admin can read audit");
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].actor_user_id, Some(learner.id()));
    assert_eq!(audit[0].event_type, KycAuditEventType::Submitted);
    assert_eq!(audit[0].from_status, None);
    assert_eq!(audit[0].to_status, KYC_STATUS_SUBMITTED);

    let delegated_audit = use_case
        .list_submission_audit(KycAuditQuery {
            reviewer_user_id: delegated_reviewer.id(),
            submission_id: submitted.id,
        })
        .await
        .expect("delegated reviewer can read audit");
    assert_eq!(delegated_audit.len(), 1);
    assert_eq!(delegated_audit[0].to_status, KYC_STATUS_SUBMITTED);

    let rejected = use_case
        .decide_submission(KycDecisionCommand {
            rejection_reason: Some("Document expired".to_string()),
            reviewer_user_id: admin.id(),
            status: KYC_STATUS_REJECTED.to_string(),
            submission_id: submitted.id,
        })
        .await
        .expect("admin can reject KYC");
    assert_eq!(rejected.status, KYC_STATUS_REJECTED);

    let audit = use_case
        .list_submission_audit(KycAuditQuery {
            reviewer_user_id: admin.id(),
            submission_id: submitted.id,
        })
        .await
        .expect("admin can read decision audit");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[1].actor_user_id, Some(admin.id()));
    assert_eq!(audit[1].event_type, KycAuditEventType::ReviewDecision);
    assert_eq!(audit[1].from_status.as_deref(), Some(KYC_STATUS_SUBMITTED));
    assert_eq!(audit[1].to_status, KYC_STATUS_REJECTED);
    assert_eq!(audit[1].reason.as_deref(), Some("Document expired"));

    let resubmitted = use_case
        .submit_my_kyc(kyc_command(learner.id(), "Learner User Again"))
        .await
        .expect("rejected learner can resubmit")
        .submission
        .expect("resubmission returned");
    let resubmitted_audit = use_case
        .list_submission_audit(KycAuditQuery {
            reviewer_user_id: admin.id(),
            submission_id: resubmitted.id,
        })
        .await
        .expect("admin can read resubmission audit");
    assert_eq!(resubmitted_audit.len(), 1);
    assert_eq!(resubmitted_audit[0].to_status, KYC_STATUS_SUBMITTED);
}
