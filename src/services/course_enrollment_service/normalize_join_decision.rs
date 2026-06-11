fn normalize_join_decision(status: &str) -> Result<String, CourseEnrollmentError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        COURSE_JOIN_STATUS_APPROVED
        | COURSE_JOIN_STATUS_REJECTED
        | COURSE_JOIN_STATUS_WAITLISTED => Ok(normalized),
        _ => Err(CourseEnrollmentError::InvalidStatus(
            "unsupported course join decision status".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
