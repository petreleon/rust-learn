fn ensure_platform_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "APPROVE_REWARD_AMOUNT"
            | "EXECUTE_REWARD_PAYOUT"
            | "VIEW_REWARD_AUDIT"
            | "MANAGE_REWARD_FRAUD_BLOCKS"
            | "BLOCK_REWARD_TEACHER"
            | "BLOCK_REWARD_ORGANIZATION"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at platform scope".to_string(),
        ))
    }
}

fn ensure_organization_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "SUBMIT_ORG_COURSE_REWARD_EVENT" | "VIEW_ORG_REWARD_REPORTS" | "MANAGE_ORG_REWARD_BUDGET"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at organization scope".to_string(),
        ))
    }
}

fn ensure_course_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "SUBMIT_COURSE_REWARD_EVENT"
            | "CREATE_REWARDABLE_COURSE_EVENT"
            | "APPROVE_STUDENT_REWARD_CANDIDATE"
            | "VIEW_COURSE_REWARD_STATUS"
            | "GRADE_REWARDABLE_ASSESSMENT"
            | "MANAGE_COURSE_REWARD_RULES"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at course scope".to_string(),
        ))
    }
}

async fn ensure_organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let exists = select(exists(
        organizations::table.filter(organizations::id.eq(organization_id)),
    ))
    .get_result::<bool>(conn)
    .await?;

    if exists {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "organization scope does not exist".to_string(),
        ))
    }
}

async fn ensure_course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let exists = select(exists(courses::table.filter(courses::id.eq(course_id))))
        .get_result::<bool>(conn)
        .await?;

    if exists {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "course scope does not exist".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests;
