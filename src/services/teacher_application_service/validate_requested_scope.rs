fn validate_requested_scope(
    requested_scope: &str,
    requested_organization_id: Option<i32>,
    requested_course_id: Option<i32>,
    organization_sponsor_id: Option<i32>,
) -> Result<(), TeacherApplicationError> {
    match requested_scope {
        TEACHER_APPLICATION_SCOPE_PLATFORM => Ok(()),
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            if requested_organization_id
                .or(organization_sponsor_id)
                .is_none()
            {
                return Err(TeacherApplicationError::InvalidInput(
                    "organization scope requires a requested organization or sponsor".to_string(),
                ));
            }
            Ok(())
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            if requested_course_id.is_none() {
                return Err(TeacherApplicationError::InvalidInput(
                    "course scope requires requested_course_id".to_string(),
                ));
            }
            Ok(())
        }
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported requested_scope".to_string(),
        )),
    }
}

fn clean_portfolio_links(portfolio_links: Option<Vec<String>>) -> Vec<String> {
    portfolio_links
        .unwrap_or_default()
        .into_iter()
        .map(|link| link.trim().to_string())
        .filter(|link| !link.is_empty())
        .collect()
}

fn portfolio_links_from_json(portfolio_links: &serde_json::Value) -> Vec<String> {
    portfolio_links
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str())
                .map(|link| link.trim().to_string())
                .filter(|link| !link.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_scope(scope: &str) -> Result<String, TeacherApplicationError> {
    let normalized = scope.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_SCOPE_PLATFORM
        | TEACHER_APPLICATION_SCOPE_ORGANIZATION
        | TEACHER_APPLICATION_SCOPE_COURSE => Ok(normalized),
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported requested_scope".to_string(),
        )),
    }
}

fn normalize_status(status: &str) -> Result<String, TeacherApplicationError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_STATUS_SUBMITTED
        | TEACHER_APPLICATION_STATUS_NEEDS_CHANGES
        | TEACHER_APPLICATION_STATUS_APPROVED
        | TEACHER_APPLICATION_STATUS_REJECTED => Ok(normalized),
        _ => Err(TeacherApplicationError::InvalidInput(
            "unsupported teacher application status".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
