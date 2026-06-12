fn audit_metadata(submission: &KycSubmission) -> serde_json::Value {
    serde_json::json!({
        "country_code": submission.country_code,
        "document_type": submission.document_type,
        "has_document_last4": submission.document_last4.is_some(),
        "has_evidence_reference": submission.evidence_reference.is_some(),
        "has_provider_reference": submission.provider_reference.is_some()
    })
}

async fn record_submission_audit(
    conn: &mut AsyncPgConnection,
    submission: &KycSubmission,
    actor_user_id: i32,
) -> QueryResult<KycAuditEvent> {
    KycAuditEvent::create(
        conn,
        NewKycAuditEvent {
            actor_user_id: Some(actor_user_id),
            event_type: KYC_AUDIT_EVENT_SUBMITTED.to_string(),
            from_status: None,
            metadata: audit_metadata(submission),
            reason: None,
            submission_id: submission.id,
            to_status: submission.status.clone(),
        },
    )
    .await
}

async fn record_decision_audit(
    conn: &mut AsyncPgConnection,
    submission: &KycSubmission,
    actor_user_id: i32,
    from_status: String,
    reason: Option<String>,
) -> QueryResult<KycAuditEvent> {
    KycAuditEvent::create(
        conn,
        NewKycAuditEvent {
            actor_user_id: Some(actor_user_id),
            event_type: KYC_AUDIT_EVENT_REVIEW_DECISION.to_string(),
            from_status: Some(from_status),
            metadata: audit_metadata(submission),
            reason,
            submission_id: submission.id,
            to_status: submission.status.clone(),
        },
    )
    .await
}
