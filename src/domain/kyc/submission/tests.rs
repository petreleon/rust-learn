use super::*;

fn valid_input() -> KycSubmissionInput {
    KycSubmissionInput {
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
    let submission = build_submission(7, valid_input()).expect("valid request");

    assert_eq!(submission.user_id, 7);
    assert_eq!(submission.status, KYC_STATUS_SUBMITTED);
    assert_eq!(submission.country_code, "US");
    assert_eq!(submission.document_type, "passport");
    assert_eq!(submission.document_last4.as_deref(), Some("1234"));
}

#[test]
fn rejects_invalid_country_and_document_type() {
    let mut bad_country = valid_input();
    bad_country.country_code = "usa".to_string();
    assert!(matches!(
        build_submission(7, bad_country),
        Err(KycRuleError::InvalidInput(_))
    ));

    let mut bad_doc = valid_input();
    bad_doc.document_type = "library_card".to_string();
    assert!(matches!(
        build_submission(7, bad_doc),
        Err(KycRuleError::InvalidInput(_))
    ));
}

#[test]
fn rejection_requires_reason() {
    let decision = KycDecisionInput {
        rejection_reason: None,
        status: KYC_STATUS_REJECTED.to_string(),
    };

    assert!(matches!(
        normalize_decision(decision),
        Err(KycRuleError::InvalidInput(_))
    ));
}

#[test]
fn next_action_tracks_submission_status() {
    assert_eq!(next_action(true, None), "verified");
    assert_eq!(next_action(false, None), "submit");
    assert_eq!(
        next_action(false, Some(KYC_STATUS_SUBMITTED)),
        "wait_for_review"
    );
    assert_eq!(next_action(false, Some(KYC_STATUS_REJECTED)), "resubmit");
}

#[test]
fn transition_guards_block_duplicate_or_final_decisions() {
    assert!(matches!(
        ensure_can_submit(false, Some(KYC_STATUS_UNDER_REVIEW)),
        Err(KycRuleError::InvalidTransition(_))
    ));
    assert!(matches!(
        ensure_can_decide(KYC_STATUS_REJECTED),
        Err(KycRuleError::InvalidTransition(_))
    ));
}
