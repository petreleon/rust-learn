#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitKycCommand {
    pub user_id: i32,
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
}
