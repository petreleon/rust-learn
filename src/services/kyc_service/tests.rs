use super::*;

fn valid_request() -> SubmitKycRequest {
    SubmitKycRequest {
        country_code: "us".to_string(),
        document_last4: Some("1234".to_string()),
        document_type: "passport".to_string(),
        evidence_reference: Some("s3://kyc/mock".to_string()),
        legal_name: "Learner User".to_string(),
        provider_reference: None,
    }
}

#[test]
fn builds_normalized_submission() {
    let submission = build_submission(7, valid_request()).expect("valid request");

    assert_eq!(submission.user_id, 7);
    assert_eq!(submission.status, KYC_STATUS_SUBMITTED);
    assert_eq!(submission.country_code, "US");
    assert_eq!(submission.document_type, "passport");
    assert_eq!(submission.document_last4.as_deref(), Some("1234"));
}

#[test]
fn rejects_invalid_country_and_document_type() {
    let mut bad_country = valid_request();
    bad_country.country_code = "usa".to_string();
    assert!(matches!(
        build_submission(7, bad_country),
        Err(KycError::InvalidInput(_))
    ));

    let mut bad_doc = valid_request();
    bad_doc.document_type = "library_card".to_string();
    assert!(matches!(
        build_submission(7, bad_doc),
        Err(KycError::InvalidInput(_))
    ));
}

#[test]
fn rejection_requires_reason() {
    let decision = KycDecisionRequest {
        rejection_reason: None,
        status: KYC_STATUS_REJECTED.to_string(),
    };

    assert!(matches!(
        normalize_decision(decision),
        Err(KycError::InvalidInput(_))
    ));
}

#[test]
fn next_action_tracks_submission_status() {
    assert_eq!(next_action(true, None), "verified");
    assert_eq!(next_action(false, None), "submit");

    let mut submission = KycSubmission {
        country_code: "US".to_string(),
        created_at: chrono::Utc::now(),
        document_last4: None,
        document_type: "passport".to_string(),
        evidence_reference: None,
        id: 1,
        legal_name: "Learner User".to_string(),
        provider_reference: None,
        rejection_reason: None,
        reviewed_at: None,
        reviewer_user_id: None,
        status: KYC_STATUS_SUBMITTED.to_string(),
        submitted_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        user_id: 7,
    };
    assert_eq!(next_action(false, Some(&submission)), "wait_for_review");
    submission.status = KYC_STATUS_REJECTED.to_string();
    assert_eq!(next_action(false, Some(&submission)), "resubmit");
}
