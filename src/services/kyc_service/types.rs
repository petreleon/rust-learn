#[derive(Debug, Deserialize)]
pub struct SubmitKycRequest {
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KycDecisionRequest {
    pub status: String,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct KycStatusResponse {
    pub submission: Option<KycSubmission>,
    pub user_kyc_verified: bool,
    pub next_action: String,
}

#[derive(Debug, Serialize)]
pub struct KycReviewQueueResponse {
    pub submissions: Vec<KycSubmission>,
}

#[derive(Debug)]
pub enum KycError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for KycError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => Self::NotFound,
            other => Self::Database(other.to_string()),
        }
    }
}
