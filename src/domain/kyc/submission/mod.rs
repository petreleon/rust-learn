mod normalization;

use normalization::{normalize_country_code, normalize_document_type, normalize_optional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KycRuleError {
    InvalidInput(String),
    InvalidTransition(String),
}

pub const KYC_STATUS_EXPIRED: &str = "expired";
pub const KYC_STATUS_PROVIDER_ERROR: &str = "provider_error";
pub const KYC_STATUS_REJECTED: &str = "rejected";
pub const KYC_STATUS_SUBMITTED: &str = "submitted";
pub const KYC_STATUS_UNDER_REVIEW: &str = "under_review";
pub const KYC_STATUS_VERIFIED: &str = "verified";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KycSubmissionInput {
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedKycSubmission {
    pub user_id: i32,
    pub status: String,
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KycDecisionInput {
    pub status: String,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedKycDecision {
    pub status: String,
    pub rejection_reason: Option<String>,
}

impl NormalizedKycDecision {
    pub fn verifies_user(&self) -> bool {
        self.status == KYC_STATUS_VERIFIED
    }
}

pub fn build_submission(
    user_id: i32,
    input: KycSubmissionInput,
) -> Result<NormalizedKycSubmission, KycRuleError> {
    let legal_name = input.legal_name.trim().to_string();
    if legal_name.len() < 3 || legal_name.len() > 160 {
        return Err(KycRuleError::InvalidInput(
            "legal_name must be between 3 and 160 characters".to_string(),
        ));
    }

    Ok(NormalizedKycSubmission {
        user_id,
        status: KYC_STATUS_SUBMITTED.to_string(),
        legal_name,
        country_code: normalize_country_code(&input.country_code)?,
        document_type: normalize_document_type(&input.document_type)?,
        document_last4: normalize_optional(input.document_last4, 16)?,
        evidence_reference: normalize_optional(input.evidence_reference, 512)?,
        provider_reference: normalize_optional(input.provider_reference, 128)?,
    })
}

pub fn ensure_can_submit(
    user_verified: bool,
    latest_status: Option<&str>,
) -> Result<(), KycRuleError> {
    if user_verified {
        return Err(KycRuleError::InvalidTransition(
            "KYC is already verified for this account".to_string(),
        ));
    }
    if matches!(
        latest_status,
        Some(KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW)
    ) {
        return Err(KycRuleError::InvalidTransition(
            "A KYC submission is already waiting for review".to_string(),
        ));
    }
    Ok(())
}

pub fn ensure_can_decide(current_status: &str) -> Result<(), KycRuleError> {
    if matches!(
        current_status,
        KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW
    ) {
        Ok(())
    } else {
        Err(KycRuleError::InvalidTransition(
            "Only submitted or under-review KYC records can be decided".to_string(),
        ))
    }
}

pub fn normalize_decision(input: KycDecisionInput) -> Result<NormalizedKycDecision, KycRuleError> {
    let status = input.status.trim().to_ascii_lowercase();
    let reason = normalize_optional(input.rejection_reason, 512)?;
    match status.as_str() {
        KYC_STATUS_VERIFIED => Ok(NormalizedKycDecision {
            status,
            rejection_reason: None,
        }),
        KYC_STATUS_REJECTED if reason.is_some() => Ok(NormalizedKycDecision {
            status,
            rejection_reason: reason,
        }),
        KYC_STATUS_REJECTED => Err(KycRuleError::InvalidInput(
            "rejection_reason is required when rejecting KYC".to_string(),
        )),
        _ => Err(KycRuleError::InvalidInput(
            "status must be verified or rejected".to_string(),
        )),
    }
}

pub fn next_action(user_verified: bool, latest_status: Option<&str>) -> String {
    if user_verified {
        return "verified".to_string();
    }
    match latest_status {
        Some(KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW) => "wait_for_review".to_string(),
        Some(KYC_STATUS_REJECTED | KYC_STATUS_EXPIRED | KYC_STATUS_PROVIDER_ERROR) => {
            "resubmit".to_string()
        }
        _ => "submit".to_string(),
    }
}

#[cfg(test)]
mod tests;
