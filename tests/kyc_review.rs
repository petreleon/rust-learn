use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::models::kyc_audit_event::{
    KYC_AUDIT_EVENT_REVIEW_DECISION, KYC_AUDIT_EVENT_SUBMITTED,
};
use rust_learn::models::kyc_submission::{KYC_STATUS_REJECTED, KYC_STATUS_SUBMITTED};
use rust_learn::models::user::User;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::kyc_service::{
    decide_submission, list_submission_audit, submit_my_kyc, KycDecisionRequest, KycError,
    SubmitKycRequest,
};

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
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

fn kyc_request(name: &str) -> SubmitKycRequest {
    SubmitKycRequest {
        country_code: "US".to_string(),
        document_last4: Some("1234".to_string()),
        document_type: "passport".to_string(),
        evidence_reference: Some("s3://kyc/evidence".to_string()),
        legal_name: name.to_string(),
        provider_reference: Some("provider-ref".to_string()),
    }
}

#[actix_web::test]
async fn kyc_submission_and_review_write_permission_scoped_audit_events() {
    let mut conn = setup_conn().await;
    let learner = create_user_helper(&mut conn, "kyc_audit_learner").await;
    let admin = create_user_helper(&mut conn, "kyc_audit_admin").await;
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let submitted = submit_my_kyc(&mut conn, learner.id(), kyc_request("Learner User"))
        .await
        .expect("submission should persist")
        .submission
        .expect("submission returned");
    let denied = list_submission_audit(&mut conn, learner.id(), submitted.id)
        .await
        .expect_err("learner cannot read review audit");
    assert!(matches!(denied, KycError::PermissionDenied(_)));

    let audit = list_submission_audit(&mut conn, admin.id(), submitted.id)
        .await
        .expect("admin can read audit");
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].actor_user_id, Some(learner.id()));
    assert_eq!(audit[0].event_type, KYC_AUDIT_EVENT_SUBMITTED);
    assert_eq!(audit[0].from_status, None);
    assert_eq!(audit[0].to_status, KYC_STATUS_SUBMITTED);

    let rejected = decide_submission(
        &mut conn,
        admin.id(),
        submitted.id,
        KycDecisionRequest {
            rejection_reason: Some("Document expired".to_string()),
            status: KYC_STATUS_REJECTED.to_string(),
        },
    )
    .await
    .expect("admin can reject KYC");
    assert_eq!(rejected.status, KYC_STATUS_REJECTED);

    let audit = list_submission_audit(&mut conn, admin.id(), submitted.id)
        .await
        .expect("admin can read decision audit");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[1].actor_user_id, Some(admin.id()));
    assert_eq!(audit[1].event_type, KYC_AUDIT_EVENT_REVIEW_DECISION);
    assert_eq!(audit[1].from_status.as_deref(), Some(KYC_STATUS_SUBMITTED));
    assert_eq!(audit[1].to_status, KYC_STATUS_REJECTED);
    assert_eq!(audit[1].reason.as_deref(), Some("Document expired"));

    let resubmitted = submit_my_kyc(&mut conn, learner.id(), kyc_request("Learner User Again"))
        .await
        .expect("rejected learner can resubmit")
        .submission
        .expect("resubmission returned");
    let resubmitted_audit = list_submission_audit(&mut conn, admin.id(), resubmitted.id)
        .await
        .expect("admin can read resubmission audit");
    assert_eq!(resubmitted_audit.len(), 1);
    assert_eq!(resubmitted_audit[0].to_status, KYC_STATUS_SUBMITTED);
}
