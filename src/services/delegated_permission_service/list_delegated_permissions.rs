pub async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListDelegatedPermissionsRequest,
) -> Result<Vec<DelegatedPermission>, DelegatedPermissionError> {
    ensure_can_delegate_reward_permissions(conn, actor_user_id).await?;

    let permission = request
        .permission
        .as_deref()
        .map(normalize_permission)
        .transpose()?;
    let scope_type = request
        .scope_type
        .as_deref()
        .map(normalize_scope_type)
        .transpose()?;

    delegated_permission_repository::list_delegated_permissions(
        conn,
        delegated_permission_repository::DelegatedPermissionFilter {
            grantor_user_id: request.grantor_user_id,
            grantee_user_id: request.grantee_user_id,
            permission,
            scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(DelegatedPermissionError::from)
}

async fn ensure_can_delegate_reward_permissions(
    conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let permission = Permissions::DELEGATE_REWARD_APPROVAL.to_string();
    if user_permission_platform_request(conn, grantor_user_id, &permission).await? {
        Ok(())
    } else {
        Err(DelegatedPermissionError::PermissionDenied(permission))
    }
}

fn normalize_permission(permission: &str) -> Result<String, DelegatedPermissionError> {
    let permission = permission.trim();
    let parsed = Permissions::from_str(permission).map_err(|_| {
        DelegatedPermissionError::InvalidInput("unsupported delegated permission".to_string())
    })?;

    Ok(parsed.to_string())
}

fn normalize_scope_type(scope_type: &str) -> Result<String, DelegatedPermissionError> {
    let normalized = scope_type.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        DELEGATED_SCOPE_PLATFORM | DELEGATED_SCOPE_ORGANIZATION | DELEGATED_SCOPE_COURSE => {
            Ok(normalized)
        }
        _ => Err(DelegatedPermissionError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}

fn ensure_permission_can_be_delegated_for_reward_work(
    permission: &str,
) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "APPROVE_REWARD_AMOUNT"
            | "EXECUTE_REWARD_PAYOUT"
            | "VIEW_REWARD_AUDIT"
            | "MANAGE_REWARD_FRAUD_BLOCKS"
            | "BLOCK_REWARD_TEACHER"
            | "BLOCK_REWARD_ORGANIZATION"
            | "SUBMIT_ORG_COURSE_REWARD_EVENT"
            | "VIEW_ORG_REWARD_REPORTS"
            | "MANAGE_ORG_REWARD_BUDGET"
            | "SUBMIT_COURSE_REWARD_EVENT"
            | "CREATE_REWARDABLE_COURSE_EVENT"
            | "APPROVE_STUDENT_REWARD_CANDIDATE"
            | "VIEW_COURSE_REWARD_STATUS"
            | "GRADE_REWARDABLE_ASSESSMENT"
            | "MANAGE_COURSE_REWARD_RULES"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission is not delegatable for reward work".to_string(),
        ))
    }
}

async fn normalize_scope_ids(
    conn: &mut AsyncPgConnection,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(Option<i32>, Option<i32>), DelegatedPermissionError> {
    match scope_type {
        DELEGATED_SCOPE_PLATFORM => {
            ensure_platform_permission_scope(permission)?;
            if organization_id.is_some() || course_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "platform delegation cannot include organization_id or course_id".to_string(),
                ));
            }
            Ok((None, None))
        }
        DELEGATED_SCOPE_ORGANIZATION => {
            ensure_organization_permission_scope(permission)?;
            let organization_id = organization_id.ok_or_else(|| {
                DelegatedPermissionError::InvalidInput(
                    "organization delegation requires organization_id".to_string(),
                )
            })?;
            if course_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "organization delegation cannot include course_id".to_string(),
                ));
            }
            ensure_organization_exists(conn, organization_id).await?;
            Ok((Some(organization_id), None))
        }
        DELEGATED_SCOPE_COURSE => {
            ensure_course_permission_scope(permission)?;
            let course_id = course_id.ok_or_else(|| {
                DelegatedPermissionError::InvalidInput(
                    "course delegation requires course_id".to_string(),
                )
            })?;
            if organization_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "course delegation cannot include organization_id".to_string(),
                ));
            }
            ensure_course_exists(conn, course_id).await?;
            Ok((None, Some(course_id)))
        }
        _ => Err(DelegatedPermissionError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}
