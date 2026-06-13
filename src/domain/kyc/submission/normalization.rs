use super::KycRuleError;

pub(super) fn normalize_country_code(value: &str) -> Result<String, KycRuleError> {
    let code = value.trim().to_ascii_uppercase();
    if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(code)
    } else {
        Err(KycRuleError::InvalidInput(
            "country_code must be a two-letter ISO country code".to_string(),
        ))
    }
}

pub(super) fn normalize_document_type(value: &str) -> Result<String, KycRuleError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "passport" | "national_id" | "driver_license" => Ok(value.trim().to_ascii_lowercase()),
        _ => Err(KycRuleError::InvalidInput(
            "document_type must be passport, national_id, or driver_license".to_string(),
        )),
    }
}

pub(super) fn normalize_optional(
    value: Option<String>,
    max_len: usize,
) -> Result<Option<String>, KycRuleError> {
    let Some(value) = value else { return Ok(None) };
    let value = value.trim().to_string();
    if value.is_empty() {
        return Ok(None);
    }
    if value.len() > max_len {
        return Err(KycRuleError::InvalidInput(
            "field value is too long".to_string(),
        ));
    }
    Ok(Some(value))
}
