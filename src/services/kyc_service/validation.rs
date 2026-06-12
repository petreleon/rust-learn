fn normalize_country_code(value: &str) -> Result<String, KycError> {
    let code = value.trim().to_ascii_uppercase();
    if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(code)
    } else {
        Err(KycError::InvalidInput(
            "country_code must be a two-letter ISO country code".to_string(),
        ))
    }
}

fn normalize_document_type(value: &str) -> Result<String, KycError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "passport" | "national_id" | "driver_license" => Ok(value.trim().to_ascii_lowercase()),
        _ => Err(KycError::InvalidInput(
            "document_type must be passport, national_id, or driver_license".to_string(),
        )),
    }
}

fn normalize_optional(value: Option<String>, max_len: usize) -> Result<Option<String>, KycError> {
    let Some(value) = value else { return Ok(None) };
    let value = value.trim().to_string();
    if value.is_empty() {
        return Ok(None);
    }
    if value.len() > max_len {
        return Err(KycError::InvalidInput("field value is too long".to_string()));
    }
    Ok(Some(value))
}

fn build_submission(user_id: i32, request: SubmitKycRequest) -> Result<NewKycSubmission, KycError> {
    let legal_name = request.legal_name.trim().to_string();
    if legal_name.len() < 3 || legal_name.len() > 160 {
        return Err(KycError::InvalidInput(
            "legal_name must be between 3 and 160 characters".to_string(),
        ));
    }

    Ok(NewKycSubmission {
        user_id,
        status: KYC_STATUS_SUBMITTED.to_string(),
        legal_name,
        country_code: normalize_country_code(&request.country_code)?,
        document_type: normalize_document_type(&request.document_type)?,
        document_last4: normalize_optional(request.document_last4, 16)?,
        evidence_reference: normalize_optional(request.evidence_reference, 512)?,
        provider_reference: normalize_optional(request.provider_reference, 128)?,
    })
}

fn normalize_decision(request: KycDecisionRequest) -> Result<(String, Option<String>), KycError> {
    let status = request.status.trim().to_ascii_lowercase();
    let reason = normalize_optional(request.rejection_reason, 512)?;
    match status.as_str() {
        KYC_STATUS_VERIFIED => Ok((status, None)),
        KYC_STATUS_REJECTED if reason.is_some() => Ok((status, reason)),
        KYC_STATUS_REJECTED => Err(KycError::InvalidInput(
            "rejection_reason is required when rejecting KYC".to_string(),
        )),
        _ => Err(KycError::InvalidInput(
            "status must be verified or rejected".to_string(),
        )),
    }
}

fn next_action(user_verified: bool, submission: Option<&KycSubmission>) -> String {
    if user_verified {
        return "verified".to_string();
    }
    match submission.map(|s| s.status.as_str()) {
        Some(KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW) => "wait_for_review".to_string(),
        Some(KYC_STATUS_REJECTED | KYC_STATUS_EXPIRED | KYC_STATUS_PROVIDER_ERROR) => {
            "resubmit".to_string()
        }
        _ => "submit".to_string(),
    }
}
