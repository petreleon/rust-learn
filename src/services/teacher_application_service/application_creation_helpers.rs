fn build_new_application(
    applicant_user_id: i32,
    request: SubmitTeacherApplicationRequest,
    forced_sponsor_id: Option<i32>,
) -> Result<NewTeacherApplication, TeacherApplicationError> {
    let requested_scope = normalize_scope(&request.requested_scope)?;
    let experience_summary = request.experience_summary.trim().to_string();
    if experience_summary.is_empty() {
        return Err(TeacherApplicationError::InvalidInput(
            "experience_summary is required".to_string(),
        ));
    }

    validate_requested_scope(
        &requested_scope,
        request.requested_organization_id,
        request.requested_course_id,
        request.organization_sponsor_id.or(forced_sponsor_id),
    )?;

    Ok(NewTeacherApplication {
        applicant_user_id,
        requested_scope,
        requested_organization_id: request.requested_organization_id,
        requested_course_id: request.requested_course_id,
        experience_summary,
        organization_sponsor_id: forced_sponsor_id.or(request.organization_sponsor_id),
        portfolio_links: json!(clean_portfolio_links(request.portfolio_links)),
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
        idempotency_key: normalize_idempotency_key(request.idempotency_key)?,
    })
}

async fn find_idempotent_application(
    conn: &mut AsyncPgConnection,
    requested: &NewTeacherApplication,
) -> Result<Option<TeacherApplication>, TeacherApplicationError> {
    let Some(idempotency_key) = requested.idempotency_key.as_deref() else {
        return Ok(None);
    };

    let existing =
        teacher_application_repository::find_application_by_idempotency_key(conn, idempotency_key)
            .await?;
    if let Some(existing) = existing.as_ref() {
        ensure_idempotent_application_matches(existing, requested)?;
    }

    Ok(existing)
}

async fn ensure_no_blocking_application(
    conn: &mut AsyncPgConnection,
    requested: &NewTeacherApplication,
) -> Result<(), TeacherApplicationError> {
    let existing = teacher_application_repository::find_latest_application_for_applicant(
        conn,
        requested.applicant_user_id,
    )
    .await?;

    match existing {
        Some(application) if application.status != TEACHER_APPLICATION_STATUS_REJECTED => {
            Err(TeacherApplicationError::InvalidTransition(format!(
                "teacher application already exists with status {}",
                application.status
            )))
        }
        _ => Ok(()),
    }
}

fn ensure_idempotent_application_matches(
    existing: &TeacherApplication,
    requested: &NewTeacherApplication,
) -> Result<(), TeacherApplicationError> {
    if existing.applicant_user_id == requested.applicant_user_id
        && existing.requested_scope == requested.requested_scope
        && existing.requested_organization_id == requested.requested_organization_id
        && existing.requested_course_id == requested.requested_course_id
        && existing.experience_summary == requested.experience_summary
        && existing.organization_sponsor_id == requested.organization_sponsor_id
        && existing.portfolio_links == requested.portfolio_links
    {
        Ok(())
    } else {
        Err(TeacherApplicationError::InvalidInput(
            "teacher application idempotency key is already used by another application"
                .to_string(),
        ))
    }
}

fn normalize_idempotency_key(
    idempotency_key: Option<String>,
) -> Result<Option<String>, TeacherApplicationError> {
    match idempotency_key {
        Some(value) => {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                Err(TeacherApplicationError::InvalidInput(
                    "idempotency_key cannot be blank".to_string(),
                ))
            } else {
                Ok(Some(trimmed))
            }
        }
        None => Ok(None),
    }
}
